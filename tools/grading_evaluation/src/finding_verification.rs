use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use shared::AppError;

use crate::decisor_client::{DecisorClient, YesProbability};
use crate::evaluation_report::{ScoredCall, ScoredRun};
use crate::finding_scoring::FindingOutcomeKind;
use crate::grader_response::GraderFinding;
use crate::grading_case::{GradingCase, GradingCaseKind};

const VERIFICATION_QUESTION: &str = "Is the quoted text a real mistake in the answer, in Brazilian Portuguese?";

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum VerificationInputKind {
    Bare,
    WithExplanation,
}

impl VerificationInputKind {
    pub fn as_str(self) -> &'static str {
        match self {
            VerificationInputKind::Bare => "bare",
            VerificationInputKind::WithExplanation => "with_explanation",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRecord {
    pub decider_label: String,
    pub decider_backend: String,
    pub decider_model: String,
    pub input_kind: String,
    pub runs: Vec<VerifiedRun>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedRun {
    pub model: String,
    pub prompt_variant: String,
    pub cases: Vec<VerifiedCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedCase {
    pub case_id: String,
    pub is_control: bool,
    pub reference_mistake_count: usize,
    pub findings: Vec<VerifiedFinding>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedFinding {
    pub outcome: FindingOutcomeKind,
    pub probability: Option<f64>,
}

pub fn verify_runs(
    client: &DecisorClient,
    scored_runs: &[ScoredRun],
    cases: &[GradingCase],
    input_kind: VerificationInputKind,
    decider_label: &str,
) -> Result<VerificationRecord, AppError> {
    let mut runs: Vec<VerifiedRun> = Vec::new();
    let mut decider_backend: String = String::new();
    let mut decider_model: String = String::new();

    for scored_run in scored_runs {
        let mut verified_cases: Vec<VerifiedCase> = Vec::new();

        for call in &scored_run.scored_calls {
            let case: &GradingCase = cases
                .iter()
                .find(|case| case.id == call.case_id)
                .ok_or_else(|| AppError::from(format!("a run names an unknown case; [case_id={}]", call.case_id)))?;

            let verified_case: VerifiedCase =
                verify_call(client, case, call, input_kind, &mut decider_backend, &mut decider_model)?;

            verified_cases.push(verified_case);
        }

        log::info!(
            "verified a run; [decider={decider_label} model={} variant={}]",
            scored_run.model,
            scored_run.prompt_variant
        );

        runs.push(VerifiedRun {
            model: scored_run.model.clone(),
            prompt_variant: scored_run.prompt_variant.clone(),
            cases: verified_cases,
        });
    }

    Ok(VerificationRecord {
        decider_label: decider_label.to_string(),
        decider_backend,
        decider_model,
        input_kind: input_kind.as_str().to_string(),
        runs,
    })
}

fn verify_call(
    client: &DecisorClient,
    case: &GradingCase,
    call: &ScoredCall,
    input_kind: VerificationInputKind,
    decider_backend: &mut String,
    decider_model: &mut String,
) -> Result<VerifiedCase, AppError> {
    let mut verified_findings: Vec<VerifiedFinding> = Vec::new();

    if let Some(response) = &call.response {
        for (finding, outcome) in response.findings.iter().zip(&call.score.finding_outcomes) {
            let asks_decider: bool = matches!(
                outcome,
                FindingOutcomeKind::Matched | FindingOutcomeKind::FalsePositive | FindingOutcomeKind::HallucinatedQuote
            );

            let probability: Option<f64> = if asks_decider {
                let input: String = render_decider_input(case, finding, input_kind);
                let started: Instant = Instant::now();
                let answered: YesProbability = client.ask_yes_probability(&input, VERIFICATION_QUESTION)?;

                log::debug!(
                    "decided; [case={} seconds={:.2} probability={:.3}]",
                    case.id,
                    started.elapsed().as_secs_f64(),
                    answered.probability
                );
                *decider_backend = answered.backend;
                *decider_model = answered.model;

                Some(answered.probability)
            } else {
                None
            };

            verified_findings.push(VerifiedFinding {
                outcome: *outcome,
                probability,
            });
        }
    }

    Ok(VerifiedCase {
        case_id: case.id.clone(),
        is_control: case.kind() == GradingCaseKind::Control,
        reference_mistake_count: case.reference.errors.len(),
        findings: verified_findings,
    })
}

pub fn render_decider_input(case: &GradingCase, finding: &GraderFinding, input_kind: VerificationInputKind) -> String {
    let mut input: String = format!("Task: {}\n", case.task.prompt_source);

    if let Some(context) = &case.task.context_source {
        input.push_str(&format!("Scene facts: {context}\n"));
    }

    input.push_str(&format!(
        "Answer: {}\nQuoted text: \"{}\"\nProposed replacement: \"{}\"",
        case.submission, finding.quote, finding.correction
    ));

    if input_kind == VerificationInputKind::WithExplanation {
        input.push_str(&format!("\nReason given: {}", finding.explanation));
    }

    input
}

pub fn write_record(directory: &Path, record: &VerificationRecord) -> Result<PathBuf, AppError> {
    std::fs::create_dir_all(directory)?;

    let path: PathBuf = directory.join(format!("{}__{}.json", record.decider_label, record.input_kind));

    std::fs::write(&path, serde_json::to_string_pretty(record)?)?;

    Ok(path)
}

pub fn read_records(directory: &Path) -> Result<Vec<VerificationRecord>, AppError> {
    let mut paths: Vec<PathBuf> = Vec::new();

    for entry in std::fs::read_dir(directory)? {
        let path: PathBuf = entry?.path();
        let is_record: bool = path.extension().is_some_and(|extension| extension == "json");

        if is_record {
            paths.push(path);
        }
    }

    paths.sort();

    let mut records: Vec<VerificationRecord> = Vec::new();

    for path in &paths {
        let content: String = std::fs::read_to_string(path)?;
        let record: VerificationRecord = serde_json::from_str(&content).map_err(|error| {
            AppError::from(format!(
                "could not parse a verification; [path={} error={error}]",
                path.display()
            ))
        })?;

        records.push(record);
    }

    Ok(records)
}
