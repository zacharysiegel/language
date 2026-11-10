use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use clap::ValueEnum;
use cross_llama::{
    Generation,
    GenerationError,
    GenerationProgress,
    GenerationRequest,
    LoadOptions,
    Model,
    ModelSource,
    Reasoning,
    Role,
    SlidingWindowCache,
    ThinkingCapability,
    ThinkingEffortKind,
    ThinkingKind,
};
use serde::{Deserialize, Serialize};
use shared::AppError;
use shared::grading::{ChatMessage, ChatRoleKind};

use crate::grading_runtime::{self, ChatReply, GradingRequest, GradingRuntime, ThinkingReply};

const UNBUDGETED_REASONING_TOKEN_ALLOWANCE: usize = 8192;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingSwitchKind {
    Off,
    On,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingEffortLevelKind {
    Low,
    Medium,
    High,
}

impl ThinkingEffortLevelKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ThinkingEffortLevelKind::Low => "low",
            ThinkingEffortLevelKind::Medium => "medium",
            ThinkingEffortLevelKind::High => "high",
        }
    }

    fn cross_llama_effort(self) -> ThinkingEffortKind {
        match self {
            ThinkingEffortLevelKind::Low => ThinkingEffortKind::Low,
            ThinkingEffortLevelKind::Medium => ThinkingEffortKind::Medium,
            ThinkingEffortLevelKind::High => ThinkingEffortKind::High,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThinkingSettings {
    pub thinking: ThinkingSwitchKind,
    pub budget_token_count: Option<usize>,
    pub effort: Option<ThinkingEffortLevelKind>,
}

impl ThinkingSettings {
    /// The most reasoning tokens; the answer's maximum comes on top.
    pub fn reasoning_token_allowance(&self) -> usize {
        match self.thinking {
            ThinkingSwitchKind::Off => 0,
            ThinkingSwitchKind::On => self.budget_token_count.unwrap_or(UNBUDGETED_REASONING_TOKEN_ALLOWANCE),
        }
    }

    pub fn maximum_token_count(&self) -> usize {
        grading_runtime::MAXIMUM_ANSWER_TOKEN_COUNT + self.reasoning_token_allowance()
    }

    pub fn label(&self) -> String {
        let mut label: String = match self.thinking {
            ThinkingSwitchKind::Off => "thinking-off".to_string(),
            ThinkingSwitchKind::On => "thinking-on".to_string(),
        };

        if let Some(budget_token_count) = self.budget_token_count {
            label.push_str(&format!("-budget-{budget_token_count}"));
        }

        if let Some(effort) = self.effort {
            label.push_str(&format!("-effort-{}", effort.as_str()));
        }

        label
    }

    fn thinking_kind(&self) -> ThinkingKind {
        match self.thinking {
            ThinkingSwitchKind::Off => ThinkingKind::Disabled,
            ThinkingSwitchKind::On => ThinkingKind::Enabled,
        }
    }

    fn cross_llama_budget_token_count(&self) -> Option<usize> {
        match self.thinking {
            ThinkingSwitchKind::Off => None,
            ThinkingSwitchKind::On => Some(self.reasoning_token_allowance()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLlamaRunRecord {
    pub model_path: PathBuf,
    pub context_token_count: u32,
    pub maximum_token_count: usize,
    pub reasoning_token_allowance: usize,
    pub thinking: ThinkingSettings,
}

pub struct CrossLlamaRuntime {
    model: Model,
    thinking_settings: ThinkingSettings,
}

impl CrossLlamaRuntime {
    pub fn load(
        model_path: &Path,
        context_token_count: u32,
        thinking_settings: ThinkingSettings,
    ) -> Result<CrossLlamaRuntime, AppError> {
        let load_options: LoadOptions = LoadOptions {
            context_token_count,
            sliding_window_cache: SlidingWindowCache::Full,
        };
        let source: ModelSource = ModelSource {
            path: model_path.to_path_buf(),
        };

        let started: Instant = Instant::now();
        let loaded: Result<Model, cross_llama::Error> =
            futures::executor::block_on(Model::load(source, &load_options, |_progress| {}));
        let model: Model = loaded.map_err(|error| {
            AppError::from(format!(
                "cross-llama could not load the model; [path={} error={}]",
                model_path.display(),
                error.message
            ))
        })?;

        log::info!(
            "loaded the model; [path={} context_token_count={context_token_count} seconds={:.1}]",
            model_path.display(),
            started.elapsed().as_secs_f64()
        );
        check_thinking_support(model.thinking_capability(), &thinking_settings)?;

        Ok(CrossLlamaRuntime {
            model,
            thinking_settings,
        })
    }

    fn create_generation_request(&self, request: &GradingRequest) -> GenerationRequest {
        GenerationRequest {
            messages: request.messages.iter().map(into_cross_llama_message).collect(),
            json_schema: Some(request.response_schema.clone()),
            temperature: request.temperature as f32,
            seed: request.seed,
            maximum_token_count: self.thinking_settings.maximum_token_count(),
            thinking: self.thinking_settings.thinking_kind(),
            thinking_budget_token_count: self.thinking_settings.cross_llama_budget_token_count(),
            thinking_effort: self.thinking_settings.effort.map(ThinkingEffortLevelKind::cross_llama_effort),
        }
    }
}

impl GradingRuntime for CrossLlamaRuntime {
    fn grade(&self, request: &GradingRequest) -> Result<ChatReply, AppError> {
        let generation_request: GenerationRequest = self.create_generation_request(request);
        let mut last_reasoning_progress_at: Option<Instant> = None;

        let started_at: Instant = Instant::now();
        let generated: Result<Generation, GenerationError> =
            futures::executor::block_on(self.model.generate(&generation_request, |progress: GenerationProgress| {
                if let GenerationProgress::GeneratingReasoning { .. } = progress {
                    last_reasoning_progress_at = Some(Instant::now());
                }
            }));
        let finished_at: Instant = Instant::now();

        let generation: Generation = generated.map_err(into_grading_failure)?;
        let reasoning_duration: Duration =
            measure_reasoning_duration(finished_at, generation.generating_duration, last_reasoning_progress_at);
        let thinking: Option<ThinkingReply> = generation
            .reasoning
            .as_ref()
            .map(|reasoning| create_thinking_reply(reasoning, &generation, reasoning_duration));

        Ok(ChatReply {
            content: generation.text,
            total_duration_nanoseconds: nanoseconds(finished_at - started_at),
            load_duration_nanoseconds: 0,
            prompt_token_count: generation.prompt_token_count as u64,
            response_token_count: generation.generated_token_count as u64,
            response_duration_nanoseconds: nanoseconds(generation.generating_duration),
            thinking,
        })
    }
}

fn check_thinking_support(capability: ThinkingCapability, thinking_settings: &ThinkingSettings) -> Result<(), AppError> {
    if thinking_settings.thinking == ThinkingSwitchKind::On && !capability.separates_reasoning {
        return Err(AppError::from(
            "thinking cannot be on: cross-llama does not know this model's reasoning delimiters",
        ));
    }

    if thinking_settings.effort.is_some() && !capability.reads_thinking_effort {
        return Err(AppError::from(
            "a thinking effort needs a chat template that reads one, which this model's does not",
        ));
    }

    if !capability.switches_thinking {
        log::warn!("the model's chat template does not switch thinking on and off; the thinking setting may not apply");
    }

    Ok(())
}

fn into_cross_llama_message(message: &ChatMessage) -> cross_llama::ChatMessage {
    let role: Role = match message.role {
        ChatRoleKind::System => Role::System,
        ChatRoleKind::User => Role::User,
        ChatRoleKind::Assistant => Role::Assistant,
    };

    cross_llama::ChatMessage {
        role,
        text: message.text.clone(),
    }
}

fn into_grading_failure(error: GenerationError) -> AppError {
    match error {
        GenerationError::Cancelled => AppError::from("cross-llama cancelled the generation"),
        GenerationError::Failed(failure) => AppError::from(format!(
            "cross-llama could not grade; [error={}]",
            failure.message
        )),
    }
}

/// From the end of prompt reading to the last reasoning token; zero when no reasoning token was generated.
fn measure_reasoning_duration(
    finished_at: Instant,
    generating_duration: Duration,
    last_reasoning_progress_at: Option<Instant>,
) -> Duration {
    let Some(last_reasoning_progress_at) = last_reasoning_progress_at
    else {
        return Duration::ZERO;
    };

    let generating_started_at: Instant = finished_at.checked_sub(generating_duration).unwrap_or(finished_at);

    last_reasoning_progress_at.saturating_duration_since(generating_started_at).min(generating_duration)
}

fn create_thinking_reply(reasoning: &Reasoning, generation: &Generation, reasoning_duration: Duration) -> ThinkingReply {
    ThinkingReply {
        reasoning_text: reasoning.text.clone(),
        reasoning_token_count: reasoning.token_count as u64,
        reasoning_duration_nanoseconds: nanoseconds(reasoning_duration),
        answer_token_count: generation.generated_token_count.saturating_sub(reasoning.token_count) as u64,
        answer_duration_nanoseconds: nanoseconds(generation.generating_duration.saturating_sub(reasoning_duration)),
    }
}

fn nanoseconds(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn thinking_settings(
        thinking: ThinkingSwitchKind,
        budget_token_count: Option<usize>,
        effort: Option<ThinkingEffortLevelKind>,
    ) -> ThinkingSettings {
        ThinkingSettings {
            thinking,
            budget_token_count,
            effort,
        }
    }

    #[test]
    fn reasoning_token_allowance_is_the_budget_or_the_unbudgeted_allowance() {
        assert_eq!(thinking_settings(ThinkingSwitchKind::Off, None, None).reasoning_token_allowance(), 0);
        assert_eq!(thinking_settings(ThinkingSwitchKind::On, Some(128), None).reasoning_token_allowance(), 128);
        assert_eq!(
            thinking_settings(ThinkingSwitchKind::On, None, None).reasoning_token_allowance(),
            UNBUDGETED_REASONING_TOKEN_ALLOWANCE
        );
    }

    #[test]
    fn label_names_every_thinking_setting() {
        let label: String =
            thinking_settings(ThinkingSwitchKind::On, Some(128), Some(ThinkingEffortLevelKind::Low)).label();

        assert_eq!(label, "thinking-on-budget-128-effort-low");
    }

    #[test]
    fn measure_reasoning_duration_counts_from_the_start_of_generating_to_the_last_reasoning_token() {
        let finished_at: Instant = Instant::now() + Duration::from_secs(10);
        let last_reasoning_progress_at: Instant = finished_at - Duration::from_secs(3);

        let reasoning_duration: Duration =
            measure_reasoning_duration(finished_at, Duration::from_secs(5), Some(last_reasoning_progress_at));

        assert_eq!(reasoning_duration, Duration::from_secs(2));
        assert_eq!(measure_reasoning_duration(finished_at, Duration::from_secs(5), None), Duration::ZERO);
    }
}
