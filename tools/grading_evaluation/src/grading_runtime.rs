use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::AppError;
use shared::grading::ChatMessage;

pub const MAXIMUM_ANSWER_TOKEN_COUNT: usize = 1024;

pub struct GradingRequest<'request_lifetime> {
    pub messages: &'request_lifetime [ChatMessage],
    pub response_schema: &'request_lifetime Value,
    pub temperature: f64,
    pub seed: u32,
}

/// Grades with one loaded model.
pub trait GradingRuntime {
    fn grade(&self, request: &GradingRequest) -> Result<ChatReply, AppError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReply {
    /// The answer, without any reasoning.
    pub content: String,
    pub total_duration_nanoseconds: u64,
    pub load_duration_nanoseconds: u64,
    pub prompt_token_count: u64,
    /// Reasoning and answer together.
    pub response_token_count: u64,
    /// Reasoning and answer together.
    pub response_duration_nanoseconds: u64,
    /// `None` when thinking was off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingReply>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingReply {
    pub reasoning_text: String,
    pub reasoning_token_count: u64,
    pub reasoning_duration_nanoseconds: u64,
    pub answer_token_count: u64,
    pub answer_duration_nanoseconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescription {
    pub family: String,
    pub parameter_size: String,
    pub quantization_level: String,
    pub size_bytes: u64,
}
