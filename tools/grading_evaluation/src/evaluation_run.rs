use std::path::{Path, PathBuf};
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::AppError;
use shared::grading::ChatMessage;

use crate::cross_llama_runtime::{CrossLlamaRunRecord, CrossLlamaRuntime, ThinkingSettings};
use crate::grader_prompt::{self, PromptVariantKind};
use crate::grading_case::GradingCase;
use crate::grading_runtime::{ChatReply, GradingRequest, GradingRuntime, ModelDescription};
use crate::ollama_client::{OllamaConnection, OllamaModel};

const CROSS_LLAMA_MODEL_LABEL_PREFIX: &str = "cross-llama";
const TEMPLATE_TOKEN_ALLOWANCE_PER_MESSAGE: usize = 16;
const TEMPLATE_TOKEN_ALLOWANCE_PER_PROMPT: usize = 64;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunRecord {
    pub model: String,
    pub prompt_variant: String,
    pub temperature: f64,
    pub repeats: u32,
    pub started_at: String,
    pub model_description: ModelDescription,
    #[serde(default)]
    pub runtime: RuntimeRecord,
    pub case_calls: Vec<CaseCallRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RuntimeRecord {
    #[default]
    Ollama,
    CrossLlama(CrossLlamaRunRecord),
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
    pub variants: &'options_lifetime [PromptVariantKind],
    pub temperature: f64,
    pub repeats: u32,
    pub output_directory: &'options_lifetime Path,
}

struct RunModel<'model_lifetime> {
    label: &'model_lifetime str,
    description: &'model_lifetime ModelDescription,
    runtime_record: &'model_lifetime RuntimeRecord,
}

pub fn run_ollama_evaluation(
    connection: &OllamaConnection,
    models: &[String],
    cases: &[GradingCase],
    options: &RunOptions,
) -> Result<Vec<PathBuf>, AppError> {
    ensure_models_installed(connection, models)?;

    let mut written_paths: Vec<PathBuf> = Vec::new();

    for model in models {
        let model_description: ModelDescription = connection.describe_model(model)?;
        let runtime: OllamaModel = OllamaModel {
            connection,
            model,
        };
        let run_model: RunModel = RunModel {
            label: model,
            description: &model_description,
            runtime_record: &RuntimeRecord::Ollama,
        };

        written_paths.extend(run_model_evaluation(&runtime, &run_model, cases, options)?);
    }

    Ok(written_paths)
}

pub fn run_cross_llama_evaluation(
    model_path: &Path,
    thinking_settings: &ThinkingSettings,
    cases: &[GradingCase],
    options: &RunOptions,
) -> Result<Vec<PathBuf>, AppError> {
    let context_token_count: u32 =
        bound_context_token_count(cases, options.variants, thinking_settings.maximum_token_count())?;
    let model_description: ModelDescription = describe_model_file(model_path)?;
    let runtime: CrossLlamaRuntime =
        CrossLlamaRuntime::load(model_path, context_token_count, thinking_settings.clone())?;

    let label: String = cross_llama_model_label(model_path, thinking_settings);
    let runtime_record: RuntimeRecord = RuntimeRecord::CrossLlama(CrossLlamaRunRecord {
        model_path: model_path.to_path_buf(),
        context_token_count,
        maximum_token_count: thinking_settings.maximum_token_count(),
        reasoning_token_allowance: thinking_settings.reasoning_token_allowance(),
        thinking: thinking_settings.clone(),
    });
    let run_model: RunModel = RunModel {
        label: &label,
        description: &model_description,
        runtime_record: &runtime_record,
    };

    run_model_evaluation(&runtime, &run_model, cases, options)
}

fn run_model_evaluation<Runtime: GradingRuntime>(
    runtime: &Runtime,
    run_model: &RunModel,
    cases: &[GradingCase],
    options: &RunOptions,
) -> Result<Vec<PathBuf>, AppError> {
    std::fs::create_dir_all(options.output_directory)?;

    let schema: Value = grader_prompt::GRADER_PROMPT.response_schema();
    warm_up_model(runtime, run_model.label, &schema, options.temperature, cases.first());

    let mut written_paths: Vec<PathBuf> = Vec::new();

    for variant in options.variants {
        let record: RunRecord = run_variant(runtime, run_model, cases, options, *variant, &schema);
        let path: PathBuf = options.output_directory.join(run_file_name(run_model.label, *variant));

        std::fs::write(&path, serde_json::to_string_pretty(&record)?)?;
        log::info!("wrote the run; [path={}]", path.display());
        written_paths.push(path);
    }

    Ok(written_paths)
}

/// At least the longest prompt's token count plus `maximum_token_count`.
fn bound_context_token_count(
    cases: &[GradingCase],
    variants: &[PromptVariantKind],
    maximum_token_count: usize,
) -> Result<u32, AppError> {
    let mut longest_prompt_token_bound: usize = 0;

    for variant in variants {
        for case in cases {
            let messages: Vec<ChatMessage> = grader_prompt::build_messages(*variant, case);
            // A token holds at least one byte of text.
            let prompt_token_bound: usize = messages
                .iter()
                .map(|message| message.text.len() + TEMPLATE_TOKEN_ALLOWANCE_PER_MESSAGE)
                .sum::<usize>()
                + TEMPLATE_TOKEN_ALLOWANCE_PER_PROMPT;

            longest_prompt_token_bound = longest_prompt_token_bound.max(prompt_token_bound);
        }
    }

    Ok(u32::try_from(longest_prompt_token_bound + maximum_token_count)?)
}

fn describe_model_file(model_path: &Path) -> Result<ModelDescription, AppError> {
    let metadata: std::fs::Metadata = std::fs::metadata(model_path).map_err(|error| {
        AppError::from(format!(
            "could not read the model file; [path={} error={error}]",
            model_path.display()
        ))
    })?;

    Ok(ModelDescription {
        family: String::new(),
        parameter_size: String::new(),
        quantization_level: String::new(),
        size_bytes: metadata.len(),
    })
}

fn cross_llama_model_label(model_path: &Path, thinking_settings: &ThinkingSettings) -> String {
    let model_name: String =
        model_path.file_stem().map(|stem| stem.to_string_lossy().to_string()).unwrap_or_default();

    format!("{CROSS_LLAMA_MODEL_LABEL_PREFIX}:{model_name}:{}", thinking_settings.label())
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

fn warm_up_model<Runtime: GradingRuntime>(
    runtime: &Runtime,
    model_label: &str,
    schema: &Value,
    temperature: f64,
    first_case: Option<&GradingCase>,
) {
    let Some(case) = first_case else {
        return;
    };

    let messages: Vec<ChatMessage> = grader_prompt::build_messages(PromptVariantKind::Plain, case);
    let request: GradingRequest = GradingRequest {
        messages: &messages,
        response_schema: schema,
        temperature,
        seed: 0,
    };

    let started: Instant = Instant::now();
    let warm_up: Result<ChatReply, AppError> = runtime.grade(&request);

    match warm_up {
        Ok(_) => log::info!(
            "warmed up the model; [model={model_label} seconds={:.1}]",
            started.elapsed().as_secs_f64()
        ),
        Err(error) => log::warn!("the warm-up call failed; [model={model_label} error={error}]"),
    }
}

fn run_variant<Runtime: GradingRuntime>(
    runtime: &Runtime,
    run_model: &RunModel,
    cases: &[GradingCase],
    options: &RunOptions,
    variant: PromptVariantKind,
    schema: &Value,
) -> RunRecord {
    let model: &str = run_model.label;
    let started_at: String = chrono::Utc::now().to_rfc3339();
    let mut case_calls: Vec<CaseCallRecord> = Vec::new();

    for (case_index, case) in cases.iter().enumerate() {
        let messages: Vec<ChatMessage> = grader_prompt::build_messages(variant, case);

        for repeat in 1..=options.repeats {
            let request: GradingRequest = GradingRequest {
                messages: &messages,
                response_schema: schema,
                temperature: options.temperature,
                seed: repeat,
            };

            let started: Instant = Instant::now();
            let result: Result<ChatReply, AppError> = runtime.grade(&request);
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
        model_description: run_model.description.clone(),
        runtime: run_model.runtime_record.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_record_without_runtime_or_thinking_reads_as_ollama() {
        let recorded: &str = r#"{
            "model": "gemma3:12b",
            "prompt_variant": "few_shot_six",
            "temperature": 0.0,
            "repeats": 1,
            "started_at": "2026-11-03T00:00:00Z",
            "model_description": { "family": "gemma3", "parameter_size": "12B", "quantization_level": "Q4_K_M", "size_bytes": 1 },
            "case_calls": [{
                "case_id": "a1-control-coffee",
                "case_fingerprint": "0",
                "repeat": 1,
                "reply": {
                    "content": "{}",
                    "total_duration_nanoseconds": 1,
                    "load_duration_nanoseconds": 0,
                    "prompt_token_count": 1,
                    "response_token_count": 1,
                    "response_duration_nanoseconds": 1
                },
                "error": null
            }]
        }"#;

        let record: RunRecord = serde_json::from_str(recorded).expect("an earlier record parses");

        assert!(matches!(record.runtime, RuntimeRecord::Ollama));
        assert!(record.case_calls[0].reply.as_ref().is_some_and(|reply| reply.thinking.is_none()));
    }
}
