use std::path::{Path, PathBuf};
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::AppError;

use crate::grader_prompt::{self, PromptVariantKind};
use crate::grading_case::GradingCase;
use crate::ollama_client::{ChatMessage, ChatReply, ChatRequest, ModelDescription, OllamaConnection};

#[derive(Debug, Serialize, Deserialize)]
pub struct RunRecord {
    pub model: String,
    pub prompt_variant: String,
    pub temperature: f64,
    pub repeats: u32,
    pub started_at: String,
    pub model_description: ModelDescription,
    pub case_calls: Vec<CaseCallRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseCallRecord {
    pub case_id: String,
    pub case_fingerprint: String,
    pub repeat: u32,
    pub reply: Option<ChatReply>,
    pub error: Option<String>,
}

pub struct RunOptions<'options_lifetime> {
    pub models: &'options_lifetime [String],
    pub variants: &'options_lifetime [PromptVariantKind],
    pub temperature: f64,
    pub repeats: u32,
    pub output_directory: &'options_lifetime Path,
}

pub fn run_evaluation(
    connection: &OllamaConnection,
    cases: &[GradingCase],
    options: &RunOptions,
) -> Result<Vec<PathBuf>, AppError> {
    ensure_models_installed(connection, options.models)?;
    std::fs::create_dir_all(options.output_directory)?;

    let schema: Value = grader_prompt::response_schema();
    let mut written_paths: Vec<PathBuf> = Vec::new();

    for model in options.models {
        let model_description: ModelDescription = connection.describe_model(model)?;
        warm_up_model(connection, model, &schema, options.temperature, cases.first());

        for variant in options.variants {
            let record: RunRecord =
                run_variant(connection, cases, options, model, *variant, &schema, &model_description);
            let path: PathBuf = options.output_directory.join(run_file_name(model, *variant));

            std::fs::write(&path, serde_json::to_string_pretty(&record)?)?;
            log::info!("wrote the run; [path={}]", path.display());
            written_paths.push(path);
        }
    }

    Ok(written_paths)
}

pub fn run_file_name(model: &str, variant: PromptVariantKind) -> String {
    let safe_model: String = model
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || character == '.' || character == '-' {
                character
            } else {
                '_'
            }
        })
        .collect();

    format!("{safe_model}__{}.json", variant.as_str())
}

fn ensure_models_installed(connection: &OllamaConnection, models: &[String]) -> Result<(), AppError> {
    let installed_names: Vec<String> = connection.installed_model_names()?;

    let missing_names: Vec<&String> = models
        .iter()
        .filter(|model| {
            !installed_names
                .iter()
                .any(|installed| installed == *model || installed == &format!("{model}:latest"))
        })
        .collect();

    if missing_names.is_empty() {
        return Ok(());
    }

    let commands: Vec<String> = missing_names.iter().map(|model| format!("ollama pull {model}")).collect();

    Err(AppError::from(format!(
        "models not installed; run: {}",
        commands.join(" && ")
    )))
}

fn warm_up_model(
    connection: &OllamaConnection,
    model: &str,
    schema: &Value,
    temperature: f64,
    first_case: Option<&GradingCase>,
) {
    let Some(case) = first_case else {
        return;
    };

    let messages: Vec<ChatMessage> = grader_prompt::build_messages(PromptVariantKind::Plain, case);
    let request: ChatRequest = ChatRequest {
        model,
        messages: &messages,
        response_schema: schema,
        temperature,
        seed: 0,
    };

    let started: Instant = Instant::now();
    let warm_up: Result<ChatReply, AppError> = connection.chat(&request);

    match warm_up {
        Ok(_) => log::info!(
            "loaded the model; [model={model} seconds={:.1}]",
            started.elapsed().as_secs_f64()
        ),
        Err(error) => log::warn!("the warm-up call failed; [model={model} error={error}]"),
    }
}

fn run_variant(
    connection: &OllamaConnection,
    cases: &[GradingCase],
    options: &RunOptions,
    model: &str,
    variant: PromptVariantKind,
    schema: &Value,
    model_description: &ModelDescription,
) -> RunRecord {
    let started_at: String = chrono::Utc::now().to_rfc3339();
    let mut case_calls: Vec<CaseCallRecord> = Vec::new();

    for (case_index, case) in cases.iter().enumerate() {
        let messages: Vec<ChatMessage> = grader_prompt::build_messages(variant, case);

        for repeat in 1..=options.repeats {
            let request: ChatRequest = ChatRequest {
                model,
                messages: &messages,
                response_schema: schema,
                temperature: options.temperature,
                seed: i64::from(repeat),
            };

            let started: Instant = Instant::now();
            let result: Result<ChatReply, AppError> = connection.chat(&request);
            let seconds: f64 = started.elapsed().as_secs_f64();

            log::info!(
                "graded a case; [model={model} variant={} case={} number={}/{} repeat={repeat} seconds={seconds:.1} ok={}]",
                variant.as_str(),
                case.id,
                case_index + 1,
                cases.len(),
                result.is_ok(),
            );

            case_calls.push(into_call_record(case, repeat, result));
        }
    }

    RunRecord {
        model: model.to_string(),
        prompt_variant: variant.as_str().to_string(),
        temperature: options.temperature,
        repeats: options.repeats,
        started_at,
        model_description: model_description.clone(),
        case_calls,
    }
}

fn into_call_record(case: &GradingCase, repeat: u32, result: Result<ChatReply, AppError>) -> CaseCallRecord {
    match result {
        Ok(reply) => CaseCallRecord {
            case_id: case.id.clone(),
            case_fingerprint: case.fingerprint(),
            repeat,
            reply: Some(reply),
            error: None,
        },
        Err(error) => CaseCallRecord {
            case_id: case.id.clone(),
            case_fingerprint: case.fingerprint(),
            repeat,
            reply: None,
            error: Some(error.to_string()),
        },
    }
}

pub fn read_run_record(path: &Path) -> Result<RunRecord, AppError> {
    let content: String = std::fs::read_to_string(path)
        .map_err(|error| AppError::from(format!("could not read a run; [path={} error={error}]", path.display())))?;
    let record: RunRecord = serde_json::from_str(&content).map_err(|error| {
        AppError::from(format!(
            "could not parse a run; [path={} error={error}]",
            path.display()
        ))
    })?;

    Ok(record)
}
