use std::collections::{BTreeMap, HashMap};

use shared::grading::{ErrorCategoryKind, GraderResponse, grader_response};

use crate::evaluation_run::RunRecord;
use crate::finding_scoring::{self, CaseScore};
use crate::grading_case::{GradingCase, GradingCaseKind};

const MAXIMUM_CONTROL_FALSE_ALARM_RATE: f64 = 0.10;
const MAXIMUM_HALLUCINATED_QUOTE_RATE: f64 = 0.02;
const MINIMUM_PRECISION: f64 = 0.70;
const MINIMUM_RECALL: f64 = 0.60;
const MINIMUM_CORRECTION_AGREEMENT: f64 = 0.60;
const MAXIMUM_P95_LATENCY_SECONDS: f64 = 15.0;

pub struct ScoredCall {
    pub case_id: String,
    pub repeat: u32,
    pub response: Option<GraderResponse>,
    pub score: CaseScore,
}

pub struct ScoredRun {
    pub model: String,
    pub prompt_variant: String,
    pub parameter_size: String,
    pub size_bytes: u64,
    pub repeats: u32,
    pub scored_calls: Vec<ScoredCall>,
    pub latencies_seconds: Vec<f64>,
    pub response_token_count: u64,
    pub response_duration_seconds: f64,
}

pub struct SummaryCounts {
    pub reference_mistakes: usize,
    pub found_correctly: usize,
    pub false_alarms: usize,
    pub quotes_not_in_answer: usize,
    pub correct_answers: usize,
    pub correct_answers_flagged: usize,
    pub answers_graded: usize,
}

pub struct ModelSummary {
    pub model: String,
    pub prompt_variant: String,
    pub parameter_size: String,
    pub size_bytes: u64,
    pub counts: SummaryCounts,
    pub response_valid_rate: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub control_false_alarm_rate: f64,
    pub hallucinated_quote_rate: f64,
    pub category_accuracy: f64,
    pub correction_agreement: f64,
    pub imprecise_quote_rate: f64,
    pub duplicate_finding_rate: f64,
    pub answers_the_question_accuracy: f64,
    pub written_in_portuguese_accuracy: f64,
    pub unstable_case_rate: Option<f64>,
    pub latency_p50_seconds: f64,
    pub latency_p95_seconds: f64,
    pub tokens_per_second: f64,
    pub category_recall: BTreeMap<ErrorCategoryKind, (usize, usize)>,
}

pub fn score_run(record: &RunRecord, cases: &[GradingCase]) -> Result<ScoredRun, shared::AppError> {
    let case_by_id: HashMap<&str, &GradingCase> = cases.iter().map(|case| (case.id.as_str(), case)).collect();
    let mut scored_calls: Vec<ScoredCall> = Vec::new();
    let mut latencies_seconds: Vec<f64> = Vec::new();
    let mut response_token_count: u64 = 0;
    let mut response_duration_nanoseconds: u64 = 0;

    for call in &record.case_calls {
        let case: &&GradingCase = case_by_id.get(call.case_id.as_str()).ok_or_else(|| {
            shared::AppError::from(format!(
                "a run names a case the cases file lacks; [case_id={} model={}]",
                call.case_id, record.model
            ))
        })?;

        if call.case_fingerprint != case.fingerprint() {
            return Err(shared::AppError::from(format!(
                "a case changed after the run graded it; grade again; [case_id={} model={}]",
                call.case_id, record.model
            )));
        }

        let response: Option<GraderResponse> =
            call.reply.as_ref().and_then(|reply| grader_response::parse_grader_response(&reply.content).ok());
        let score: CaseScore = finding_scoring::score_case(case, response.as_ref());

        if let Some(reply) = &call.reply {
            let work_nanoseconds: u64 =
                reply.total_duration_nanoseconds.saturating_sub(reply.load_duration_nanoseconds);
            latencies_seconds.push(work_nanoseconds as f64 / 1e9);
            response_token_count += reply.response_token_count;
            response_duration_nanoseconds += reply.response_duration_nanoseconds;
        }

        scored_calls.push(ScoredCall {
            case_id: call.case_id.clone(),
            repeat: call.repeat,
            response,
            score,
        });
    }

    Ok(ScoredRun {
        model: record.model.clone(),
        prompt_variant: record.prompt_variant.clone(),
        parameter_size: record.model_description.parameter_size.clone(),
        size_bytes: record.model_description.size_bytes,
        repeats: record.repeats,
        scored_calls,
        latencies_seconds,
        response_token_count,
        response_duration_seconds: response_duration_nanoseconds as f64 / 1e9,
    })
}

pub fn summarize(run: &ScoredRun, cases: &[GradingCase]) -> ModelSummary {
    let case_by_id: HashMap<&str, &GradingCase> = cases.iter().map(|case| (case.id.as_str(), case)).collect();
    let scores: Vec<&CaseScore> = run.scored_calls.iter().map(|call| &call.score).collect();

    let true_positives: usize = scores.iter().map(|score| score.true_positives()).sum();
    let false_positives: usize = scores.iter().map(|score| score.false_positives).sum();
    let hallucinated_quotes: usize = scores.iter().map(|score| score.hallucinated_quotes).sum();
    let duplicates: usize = scores.iter().map(|score| score.duplicates).sum();
    let reference_errors: usize = scores.iter().map(|score| score.reference_error_count()).sum();
    let finding_count: usize =
        scores.iter().filter(|score| score.findings_scored).map(|score| score.finding_count).sum();

    let matches: Vec<&finding_scoring::FindingMatch> = scores.iter().flat_map(|score| score.matches.iter()).collect();
    let category_correct: usize = matches.iter().filter(|finding_match| finding_match.category_correct).count();
    let correction_agrees: usize = matches.iter().filter(|finding_match| finding_match.correction_agrees).count();
    let imprecise: usize = matches.iter().filter(|finding_match| finding_match.imprecise_quote).count();

    let control_count: usize = scores.iter().filter(|score| score.kind == GradingCaseKind::Control).count();
    let flagged_controls: usize = scores.iter().filter(|score| score.flagged_a_control()).count();

    let precision: f64 = ratio(true_positives, true_positives + false_positives + hallucinated_quotes);
    let recall: f64 = ratio(true_positives, reference_errors);

    ModelSummary {
        model: run.model.clone(),
        prompt_variant: run.prompt_variant.clone(),
        parameter_size: run.parameter_size.clone(),
        size_bytes: run.size_bytes,
        counts: SummaryCounts {
            reference_mistakes: reference_errors,
            found_correctly: true_positives,
            false_alarms: false_positives,
            quotes_not_in_answer: hallucinated_quotes,
            correct_answers: control_count,
            correct_answers_flagged: flagged_controls,
            answers_graded: scores.len(),
        },
        response_valid_rate: ratio(scores.iter().filter(|score| score.response_valid).count(), scores.len()),
        precision,
        recall,
        f1: harmonic_mean(precision, recall),
        control_false_alarm_rate: ratio(flagged_controls, control_count),
        hallucinated_quote_rate: ratio(hallucinated_quotes, finding_count),
        category_accuracy: ratio(category_correct, matches.len()),
        correction_agreement: ratio(correction_agrees, matches.len()),
        imprecise_quote_rate: ratio(imprecise, matches.len()),
        duplicate_finding_rate: ratio(duplicates, finding_count),
        answers_the_question_accuracy: ratio(
            scores.iter().filter(|score| score.answers_the_question_correct).count(),
            scores.len(),
        ),
        written_in_portuguese_accuracy: ratio(
            scores.iter().filter(|score| score.written_in_portuguese_correct).count(),
            scores.len(),
        ),
        unstable_case_rate: unstable_case_rate(run),
        latency_p50_seconds: percentile(&run.latencies_seconds, 0.50),
        latency_p95_seconds: percentile(&run.latencies_seconds, 0.95),
        tokens_per_second: run.response_token_count as f64 / run.response_duration_seconds,
        category_recall: category_recall(run, &case_by_id),
    }
}

fn category_recall(
    run: &ScoredRun,
    case_by_id: &HashMap<&str, &GradingCase>,
) -> BTreeMap<ErrorCategoryKind, (usize, usize)> {
    let mut category_recall: BTreeMap<ErrorCategoryKind, (usize, usize)> = BTreeMap::new();

    for call in &run.scored_calls {
        let Some(case) = case_by_id.get(call.case_id.as_str()) else {
            continue;
        };

        for (reference_index, reference_error) in case.reference.errors.iter().enumerate() {
            let hit: bool =
                call.score.matches.iter().any(|finding_match| finding_match.reference_index == reference_index);
            let entry: &mut (usize, usize) = category_recall.entry(reference_error.category).or_insert((0, 0));

            entry.1 += 1;

            if hit {
                entry.0 += 1;
            }
        }
    }

    category_recall
}

fn unstable_case_rate(run: &ScoredRun) -> Option<f64> {
    if run.repeats < 2 {
        return None;
    }

    let mut signatures_by_case: BTreeMap<&str, Vec<String>> = BTreeMap::new();

    for call in &run.scored_calls {
        let mut matched_indexes: Vec<usize> =
            call.score.matches.iter().map(|finding_match| finding_match.reference_index).collect();
        matched_indexes.sort_unstable();

        let signature: String = format!(
            "{matched_indexes:?}/{}/{}/{}",
            call.score.false_positives, call.score.hallucinated_quotes, call.score.response_valid
        );

        signatures_by_case.entry(&call.case_id).or_default().push(signature);
    }

    let unstable: usize = signatures_by_case
        .values()
        .filter(|signatures| signatures.iter().any(|signature| signature != &signatures[0]))
        .count();

    Some(ratio(unstable, signatures_by_case.len()))
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        return f64::NAN;
    }

    numerator as f64 / denominator as f64
}

fn harmonic_mean(left: f64, right: f64) -> f64 {
    if left.is_nan() || right.is_nan() || left + right == 0.0 {
        return f64::NAN;
    }

    2.0 * left * right / (left + right)
}

fn percentile(values: &[f64], quantile: f64) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    let mut sorted_values: Vec<f64> = values.to_vec();
    sorted_values.sort_by(|left, right| left.total_cmp(right));

    let rank: usize = (quantile * sorted_values.len() as f64).ceil() as usize;

    sorted_values[rank.clamp(1, sorted_values.len()) - 1]
}

pub fn render_report(summaries: &[ModelSummary]) -> String {
    let mut report: String = String::new();

    report.push_str("# Grading evaluation\n\n");
    report.push_str("## Error detection\n\n");
    report.push_str(&render_table(
        &accuracy_headers(),
        &summaries.iter().map(accuracy_row).collect::<Vec<Vec<String>>>(),
    ));
    report.push_str(
        "\n- Precision: findings that match a real mistake ÷ (those findings + findings that match none); a repeat finding of an already matched mistake counts in neither\n- Recall: real mistakes matched by a finding ÷ all real mistakes\n- False positive rate: correct answers with at least one finding ÷ all correct answers\n",
    );
    report.push_str("\n## Counts behind the rates\n\n");
    report.push_str(&render_table(
        &count_headers(),
        &summaries.iter().map(count_row).collect::<Vec<Vec<String>>>(),
    ));
    report.push_str("\n## Behavior and cost\n\n");
    report.push_str(&render_table(
        &behavior_headers(),
        &summaries.iter().map(behavior_row).collect::<Vec<Vec<String>>>(),
    ));
    report.push_str("\n## Recall by error category\n\n");
    report.push_str(&render_table(
        &category_headers(),
        &summaries.iter().map(category_row).collect::<Vec<Vec<String>>>(),
    ));
    report.push_str("\n## Bar\n\n");
    report.push_str(&render_bar_description());
    report.push('\n');
    report.push_str(&render_table(
        &bar_headers(),
        &summaries.iter().map(bar_row).collect::<Vec<Vec<String>>>(),
    ));

    report
}

pub fn render_failures(runs: &[ScoredRun], cases: &[GradingCase]) -> String {
    let case_by_id: HashMap<&str, &GradingCase> = cases.iter().map(|case| (case.id.as_str(), case)).collect();
    let mut failures: String = String::from(
        "# Grading failures\n\nEvery call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.\n",
    );

    for run in runs {
        failures.push_str(&format!("\n## {} / {}\n", run.model, run.prompt_variant));

        for call in run.scored_calls.iter().filter(|call| call.score.has_failure()) {
            if let Some(case) = case_by_id.get(call.case_id.as_str()) {
                failures.push_str(&render_call_detail(case, call));
            }
        }
    }

    failures
}

pub fn render_sample(runs: &[ScoredRun], cases: &[GradingCase], sample_size: usize) -> String {
    let case_by_id: HashMap<&str, &GradingCase> = cases.iter().map(|case| (case.id.as_str(), case)).collect();
    let stride: usize = cases.len().div_ceil(sample_size.max(1));
    let sampled_ids: Vec<&str> = cases.iter().step_by(stride.max(1)).map(|case| case.id.as_str()).collect();
    let mut sample: String =
        String::from("# Grading review sample\n\nThe same cases for every run, for rating the explanations by hand.\n");

    for run in runs {
        sample.push_str(&format!("\n## {} / {}\n", run.model, run.prompt_variant));

        for call in run
            .scored_calls
            .iter()
            .filter(|call| call.repeat == 1 && sampled_ids.contains(&call.case_id.as_str()))
        {
            if let Some(case) = case_by_id.get(call.case_id.as_str()) {
                sample.push_str(&render_call_detail(case, call));
            }
        }
    }

    sample
}

fn render_call_detail(case: &GradingCase, call: &ScoredCall) -> String {
    let mut detail: String = format!(
        "\n### {} ({}, {})\n\n- Task: {}\n- Answer: {}\n",
        case.id,
        case.kind().as_str(),
        case.level,
        case.task.prompt_source,
        case.submission,
    );

    for reference_error in &case.reference.errors {
        detail.push_str(&format!(
            "- Reference: `{}` -> {} [{}]\n",
            reference_error.wrong_text,
            reference_error.accepted_corrections.join(" | "),
            reference_error.category.as_str(),
        ));
    }

    let Some(response) = &call.response else {
        detail.push_str("- Response: invalid or missing\n");

        return detail;
    };

    detail.push_str(&format!(
        "- Flags: written_in_portuguese={} answers_the_question={}\n",
        response.written_in_portuguese, response.answers_the_question
    ));

    for (finding, outcome) in response.findings.iter().zip(&call.score.finding_outcomes) {
        detail.push_str(&format!(
            "- Found ({}): `{}` -> `{}` [{}] {}\n",
            outcome.as_str(),
            finding.quote,
            finding.correction,
            finding.category,
            finding.explanation,
        ));
    }

    for reference_index in &call.score.missed_reference_indexes {
        detail.push_str(&format!(
            "- Missed: `{}`\n",
            case.reference.errors[*reference_index].wrong_text
        ));
    }

    detail
}

fn accuracy_headers() -> Vec<&'static str> {
    vec![
        "Model",
        "Prompt",
        "Precision",
        "Recall",
        "F1",
        "False positive rate",
        "Quotes not in answer",
        "Category",
        "Correction",
        "Imprecise quote",
        "Duplicates",
    ]
}

fn accuracy_row(summary: &ModelSummary) -> Vec<String> {
    vec![
        summary.model.clone(),
        summary.prompt_variant.clone(),
        percent(summary.precision),
        percent(summary.recall),
        percent(summary.f1),
        percent(summary.control_false_alarm_rate),
        percent(summary.hallucinated_quote_rate),
        percent(summary.category_accuracy),
        percent(summary.correction_agreement),
        percent(summary.imprecise_quote_rate),
        percent(summary.duplicate_finding_rate),
    ]
}

fn count_headers() -> Vec<&'static str> {
    vec![
        "Model",
        "Prompt",
        "Answers graded",
        "Reference mistakes",
        "Found correctly",
        "False alarms",
        "Quotes not in answer",
        "Correct answers",
        "Correct answers flagged",
    ]
}

fn count_row(summary: &ModelSummary) -> Vec<String> {
    vec![
        summary.model.clone(),
        summary.prompt_variant.clone(),
        summary.counts.answers_graded.to_string(),
        summary.counts.reference_mistakes.to_string(),
        summary.counts.found_correctly.to_string(),
        summary.counts.false_alarms.to_string(),
        summary.counts.quotes_not_in_answer.to_string(),
        summary.counts.correct_answers.to_string(),
        summary.counts.correct_answers_flagged.to_string(),
    ]
}

fn behavior_headers() -> Vec<&'static str> {
    vec![
        "Model",
        "Prompt",
        "Parameters",
        "Download",
        "Valid response",
        "On-topic flag",
        "Language flag",
        "Unstable cases",
        "p50 s",
        "p95 s",
        "Tokens/s",
    ]
}

fn behavior_row(summary: &ModelSummary) -> Vec<String> {
    vec![
        summary.model.clone(),
        summary.prompt_variant.clone(),
        summary.parameter_size.clone(),
        format!("{:.1} GB", summary.size_bytes as f64 / 1e9),
        percent(summary.response_valid_rate),
        percent(summary.answers_the_question_accuracy),
        percent(summary.written_in_portuguese_accuracy),
        summary.unstable_case_rate.map(percent).unwrap_or_else(|| "n/a".to_string()),
        format!("{:.1}", summary.latency_p50_seconds),
        format!("{:.1}", summary.latency_p95_seconds),
        format!("{:.0}", summary.tokens_per_second),
    ]
}

fn category_headers() -> Vec<&'static str> {
    let mut headers: Vec<&'static str> = vec!["Model", "Prompt"];
    headers.extend(ErrorCategoryKind::ALL.iter().map(|category| category.as_str()));

    headers
}

fn category_row(summary: &ModelSummary) -> Vec<String> {
    let mut row: Vec<String> = vec![summary.model.clone(), summary.prompt_variant.clone()];

    for category in ErrorCategoryKind::ALL {
        let cell: String = match summary.category_recall.get(&category) {
            Some((hits, total)) => format!("{} ({hits}/{total})", percent(ratio(*hits, *total))),
            None => "n/a".to_string(),
        };

        row.push(cell);
    }

    row
}

fn render_bar_description() -> String {
    format!(
        "- False positive rate at most {}\n- Quotes not in the answer at most {}\n- Precision at least {}\n- Recall at least {}\n- Correction agreement at least {}\n- p95 latency at most {MAXIMUM_P95_LATENCY_SECONDS:.0} s\n",
        percent(MAXIMUM_CONTROL_FALSE_ALARM_RATE),
        percent(MAXIMUM_HALLUCINATED_QUOTE_RATE),
        percent(MINIMUM_PRECISION),
        percent(MINIMUM_RECALL),
        percent(MINIMUM_CORRECTION_AGREEMENT),
    )
}

fn bar_headers() -> Vec<&'static str> {
    vec![
        "Model",
        "Prompt",
        "Control",
        "Quotes",
        "Precision",
        "Recall",
        "Correction",
        "Latency",
        "Meets all",
    ]
}

fn bar_row(summary: &ModelSummary) -> Vec<String> {
    let checks: [bool; 6] = [
        summary.control_false_alarm_rate <= MAXIMUM_CONTROL_FALSE_ALARM_RATE,
        summary.hallucinated_quote_rate <= MAXIMUM_HALLUCINATED_QUOTE_RATE || summary.hallucinated_quote_rate.is_nan(),
        summary.precision >= MINIMUM_PRECISION,
        summary.recall >= MINIMUM_RECALL,
        summary.correction_agreement >= MINIMUM_CORRECTION_AGREEMENT,
        summary.latency_p95_seconds <= MAXIMUM_P95_LATENCY_SECONDS,
    ];

    let mut row: Vec<String> = vec![summary.model.clone(), summary.prompt_variant.clone()];
    row.extend(checks.iter().map(|passed| mark(*passed)));
    row.push(mark(checks.iter().all(|passed| *passed)));

    row
}

fn mark(passed: bool) -> String {
    if passed { "yes".to_string() } else { "no".to_string() }
}

fn percent(proportion: f64) -> String {
    if proportion.is_nan() {
        return "n/a".to_string();
    }

    format!("{:.0}%", proportion * 100.0)
}

fn render_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let column_widths: Vec<usize> = (0..headers.len())
        .map(|column| {
            let row_width: usize = rows.iter().map(|row| row[column].chars().count()).max().unwrap_or(0);

            row_width.max(headers[column].chars().count())
        })
        .collect();

    let mut table: String = render_table_line(
        &headers.iter().map(|header| header.to_string()).collect::<Vec<String>>(),
        &column_widths,
    );
    let separator_cells: Vec<String> = column_widths.iter().map(|width| "-".repeat(*width)).collect();

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

    #[test]
    fn percentile_uses_the_nearest_rank() {
        let values: Vec<f64> = vec![5.0, 1.0, 3.0, 2.0, 4.0];

        assert_eq!(percentile(&values, 0.50), 3.0);
        assert_eq!(percentile(&values, 0.95), 5.0);
    }

    #[test]
    fn harmonic_mean_is_not_a_number_when_a_part_is_undefined() {
        assert!(harmonic_mean(f64::NAN, 0.5).is_nan());
        assert!((harmonic_mean(0.5, 1.0) - 2.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn render_table_pads_every_column_to_its_widest_cell() {
        let table: String = render_table(&["A", "Long header"], &[vec!["wide cell".to_string(), "x".to_string()]]);

        assert_eq!(
            table,
            "| A         | Long header |\n| --------- | ----------- |\n| wide cell | x           |\n"
        );
    }

    #[test]
    fn percent_shows_not_available_for_an_undefined_proportion() {
        assert_eq!(percent(f64::NAN), "n/a");
        assert_eq!(percent(0.256), "26%");
    }
}
