use crate::finding_scoring::FindingOutcomeKind;
use crate::finding_verification::{VerificationRecord, VerifiedCase, VerifiedFinding, VerifiedRun};

const THRESHOLDS: [f64; 5] = [0.3, 0.5, 0.7, 0.8, 0.9];
const CALIBRATION_BIN_COUNT: usize = 5;

struct LabelledProbability {
    probability: f64,
    real: bool,
}

pub fn render_verification_report(records: &[VerificationRecord]) -> String {
    let mut report: String = String::from("# Finding verification\n\n");

    report.push_str(&render_separation_table(records));

    for record in records {
        report.push_str(&format!(
            "\n## {} ({} input)\n\n### Calibration\n\n",
            record.decider_label, record.input_kind
        ));
        report.push_str(&render_calibration_table(record));
        report.push_str("\n### Grading with findings below the threshold removed\n\n");
        report.push_str(&render_threshold_table(record));
    }

    report
}

fn render_separation_table(records: &[VerificationRecord]) -> String {
    let headers: Vec<&str> = vec![
        "Decider",
        "Input",
        "Model",
        "Real findings",
        "False findings",
        "Mean probability, real",
        "Mean probability, false",
        "AUROC",
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();

    for record in records {
        let labelled: Vec<LabelledProbability> = labelled_probabilities(record);
        let real_count: usize = labelled.iter().filter(|item| item.real).count();
        let false_count: usize = labelled.len() - real_count;

        rows.push(vec![
            record.decider_label.clone(),
            record.input_kind.clone(),
            record.decider_model.clone(),
            real_count.to_string(),
            false_count.to_string(),
            format_mean(&labelled, true),
            format_mean(&labelled, false),
            format_number(area_under_roc_curve(&labelled)),
        ]);
    }

    render_table(&headers, &rows)
}

fn render_calibration_table(record: &VerificationRecord) -> String {
    let labelled: Vec<LabelledProbability> = labelled_probabilities(record);
    let headers: Vec<&str> = vec![
        "Stated probability",
        "Findings",
        "Mean stated probability",
        "Proportion real",
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();

    for bin in 0..CALIBRATION_BIN_COUNT {
        let lower: f64 = bin as f64 / CALIBRATION_BIN_COUNT as f64;
        let upper: f64 = (bin + 1) as f64 / CALIBRATION_BIN_COUNT as f64;
        let in_bin: Vec<&LabelledProbability> = labelled
            .iter()
            .filter(|item| {
                item.probability >= lower
                    && (item.probability < upper || (bin == CALIBRATION_BIN_COUNT - 1 && item.probability <= upper))
            })
            .collect();

        let mean_probability: f64 = in_bin.iter().map(|item| item.probability).sum::<f64>() / in_bin.len() as f64;
        let proportion_real: f64 = in_bin.iter().filter(|item| item.real).count() as f64 / in_bin.len() as f64;

        rows.push(vec![
            format!("{lower:.1} to {upper:.1}"),
            in_bin.len().to_string(),
            format_number(mean_probability),
            format_number(proportion_real),
        ]);
    }

    render_table(&headers, &rows)
}

fn render_threshold_table(record: &VerificationRecord) -> String {
    let headers: Vec<&str> = vec![
        "Model",
        "Prompt",
        "Threshold",
        "Findings kept",
        "Precision",
        "Recall",
        "False positive rate (correct answers)",
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();

    for run in &record.runs {
        rows.push(threshold_row(run, None));

        for threshold in THRESHOLDS {
            rows.push(threshold_row(run, Some(threshold)));
        }
    }

    render_table(&headers, &rows)
}

fn threshold_row(run: &VerifiedRun, threshold: Option<f64>) -> Vec<String> {
    let mut real_kept: usize = 0;
    let mut false_kept: usize = 0;
    let mut reference_total: usize = 0;
    let mut control_count: usize = 0;
    let mut flagged_controls: usize = 0;

    for case in &run.cases {
        reference_total += case.reference_mistake_count;

        let (case_real, case_false) = count_kept(case, threshold);

        real_kept += case_real;
        false_kept += case_false;

        if case.is_control {
            control_count += 1;

            if case_real + case_false > 0 {
                flagged_controls += 1;
            }
        }
    }

    vec![
        run.model.clone(),
        run.prompt_variant.clone(),
        threshold.map(|value| format!("{value:.1}")).unwrap_or_else(|| "none".to_string()),
        (real_kept + false_kept).to_string(),
        format_proportion(real_kept, real_kept + false_kept),
        format_proportion(real_kept, reference_total),
        format_proportion(flagged_controls, control_count),
    ]
}

fn count_kept(case: &VerifiedCase, threshold: Option<f64>) -> (usize, usize) {
    let mut real: usize = 0;
    let mut not_real: usize = 0;

    for finding in &case.findings {
        let counted: Option<bool> = counted_as_real(finding, threshold);

        match counted {
            Some(true) => real += 1,
            Some(false) => not_real += 1,
            None => {}
        }
    }

    (real, not_real)
}

fn counted_as_real(finding: &VerifiedFinding, threshold: Option<f64>) -> Option<bool> {
    match finding.outcome {
        FindingOutcomeKind::Duplicate => None,
        FindingOutcomeKind::ChangesNothing => match threshold {
            Some(_) => None,
            None => Some(false),
        },
        FindingOutcomeKind::Matched | FindingOutcomeKind::FalsePositive | FindingOutcomeKind::HallucinatedQuote => {
            let kept: bool = match (threshold, finding.probability) {
                (Some(threshold), Some(probability)) => probability >= threshold,
                _ => true,
            };

            if !kept {
                return None;
            }

            Some(finding.outcome == FindingOutcomeKind::Matched)
        }
    }
}

fn labelled_probabilities(record: &VerificationRecord) -> Vec<LabelledProbability> {
    let mut labelled: Vec<LabelledProbability> = Vec::new();

    for run in &record.runs {
        for case in &run.cases {
            for finding in &case.findings {
                let real: Option<bool> = match finding.outcome {
                    FindingOutcomeKind::Matched => Some(true),
                    FindingOutcomeKind::FalsePositive | FindingOutcomeKind::HallucinatedQuote => Some(false),
                    FindingOutcomeKind::ChangesNothing | FindingOutcomeKind::Duplicate => None,
                };

                if let (Some(real), Some(probability)) = (real, finding.probability) {
                    labelled.push(LabelledProbability { probability, real });
                }
            }
        }
    }

    labelled
}

/// The probability that a randomly chosen real finding receives a higher stated probability than a randomly chosen
/// false one, counting ties as half.
fn area_under_roc_curve(labelled: &[LabelledProbability]) -> f64 {
    let real_findings: Vec<&LabelledProbability> = labelled.iter().filter(|item| item.real).collect();
    let false_findings: Vec<&LabelledProbability> = labelled.iter().filter(|item| !item.real).collect();

    if real_findings.is_empty() || false_findings.is_empty() {
        return f64::NAN;
    }

    let mut winning_pairs: f64 = 0.0;

    for real_finding in &real_findings {
        for false_finding in &false_findings {
            if real_finding.probability > false_finding.probability {
                winning_pairs += 1.0;
            } else if real_finding.probability == false_finding.probability {
                winning_pairs += 0.5;
            }
        }
    }

    winning_pairs / (real_findings.len() * false_findings.len()) as f64
}

fn format_mean(labelled: &[LabelledProbability], real: bool) -> String {
    let selected: Vec<f64> = labelled.iter().filter(|item| item.real == real).map(|item| item.probability).collect();

    format_number(selected.iter().sum::<f64>() / selected.len() as f64)
}

fn format_number(value: f64) -> String {
    if value.is_nan() {
        return "n/a".to_string();
    }

    format!("{value:.2}")
}

fn format_proportion(numerator: usize, denominator: usize) -> String {
    if denominator == 0 {
        return "n/a".to_string();
    }

    format!(
        "{:.0}% ({numerator}/{denominator})",
        numerator as f64 / denominator as f64 * 100.0
    )
}

fn render_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let column_widths: Vec<usize> = (0..headers.len())
        .map(|column| {
            let row_width: usize = rows.iter().map(|row| row[column].chars().count()).max().unwrap_or(0);

            row_width.max(headers[column].chars().count())
        })
        .collect();

    let header_cells: Vec<String> = headers.iter().map(|header| header.to_string()).collect();
    let separator_cells: Vec<String> = column_widths.iter().map(|width| "-".repeat(*width)).collect();
    let mut table: String = render_table_line(&header_cells, &column_widths);

    table.push_str(&render_table_line(&separator_cells, &column_widths));

    for row in rows {
        table.push_str(&render_table_line(row, &column_widths));
    }

    table
}

fn render_table_line(cells: &[String], column_widths: &[usize]) -> String {
    let padded_cells: Vec<String> = cells
        .iter()
        .zip(column_widths)
        .map(|(cell, width)| format!("{cell:<width$}", width = *width))
        .collect();

    format!("| {} |\n", padded_cells.join(" | "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labelled(probability: f64, real: bool) -> LabelledProbability {
        LabelledProbability { probability, real }
    }

    #[test]
    fn area_under_roc_curve_is_one_for_perfect_separation() {
        let items: Vec<LabelledProbability> = vec![
            labelled(0.9, true),
            labelled(0.8, true),
            labelled(0.2, false),
            labelled(0.1, false),
        ];

        assert_eq!(area_under_roc_curve(&items), 1.0);
    }

    #[test]
    fn area_under_roc_curve_counts_ties_as_half() {
        let items: Vec<LabelledProbability> = vec![labelled(0.5, true), labelled(0.5, false)];

        assert_eq!(area_under_roc_curve(&items), 0.5);
    }

    #[test]
    fn area_under_roc_curve_is_not_a_number_without_both_classes() {
        let items: Vec<LabelledProbability> = vec![labelled(0.5, true)];

        assert!(area_under_roc_curve(&items).is_nan());
    }

    #[test]
    fn counted_as_real_drops_changes_nothing_once_a_verifier_runs() {
        let finding: VerifiedFinding = VerifiedFinding {
            outcome: FindingOutcomeKind::ChangesNothing,
            probability: None,
        };

        assert_eq!(counted_as_real(&finding, None), Some(false));
        assert_eq!(counted_as_real(&finding, Some(0.5)), None);
    }

    #[test]
    fn counted_as_real_removes_a_finding_below_the_threshold() {
        let finding: VerifiedFinding = VerifiedFinding {
            outcome: FindingOutcomeKind::Matched,
            probability: Some(0.4),
        };

        assert_eq!(counted_as_real(&finding, Some(0.5)), None);
        assert_eq!(counted_as_real(&finding, Some(0.3)), Some(true));
        assert_eq!(counted_as_real(&finding, None), Some(true));
    }
}
