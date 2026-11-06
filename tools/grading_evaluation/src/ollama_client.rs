use std::collections::HashSet;
use std::sync::Mutex;
use std::time::Duration;

use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use shared::AppError;
use shared::grading::ChatMessage;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(300);
const MAXIMUM_RESPONSE_TOKENS: u32 = 1024;
const KEEP_MODEL_LOADED: &str = "30m";

pub struct ChatRequest<'request_lifetime> {
    pub model: &'request_lifetime str,
    pub messages: &'request_lifetime [ChatMessage],
    pub response_schema: &'request_lifetime Value,
    pub temperature: f64,
    pub seed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReply {
    pub content: String,
    pub total_duration_nanoseconds: u64,
    pub load_duration_nanoseconds: u64,
    pub prompt_token_count: u64,
    pub response_token_count: u64,
    pub response_duration_nanoseconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescription {
    pub family: String,
    pub parameter_size: String,
    pub quantization_level: String,
    pub size_bytes: u64,
}

pub struct OllamaConnection {
    client: Client,
    base_url: String,
    models_without_thinking: Mutex<HashSet<String>>,
}

#[derive(Deserialize)]
struct OllamaChatSerialOut {
    message: OllamaChatMessageSerialOut,
    #[serde(default)]
    total_duration: u64,
    #[serde(default)]
    load_duration: u64,
    #[serde(default)]
    prompt_eval_count: u64,
    #[serde(default)]
    eval_count: u64,
    #[serde(default)]
    eval_duration: u64,
}

#[derive(Deserialize)]
struct OllamaChatMessageSerialOut {
    content: String,
}

#[derive(Deserialize)]
struct OllamaTagsSerialOut {
    models: Vec<OllamaTagSerialOut>,
}

#[derive(Deserialize)]
struct OllamaTagSerialOut {
    name: String,
    size: u64,
}

#[derive(Deserialize)]
struct OllamaShowSerialOut {
    details: OllamaShowDetailsSerialOut,
}

#[derive(Deserialize)]
struct OllamaShowDetailsSerialOut {
    #[serde(default)]
    family: String,
    #[serde(default)]
    parameter_size: String,
    #[serde(default)]
    quantization_level: String,
}

impl OllamaConnection {
    pub fn connect(base_url: &str) -> Result<OllamaConnection, AppError> {
        let client: Client = Client::builder().timeout(REQUEST_TIMEOUT).build().map_err(request_failure)?;

        Ok(OllamaConnection {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            models_without_thinking: Mutex::new(HashSet::new()),
        })
    }

    pub fn installed_model_names(&self) -> Result<Vec<String>, AppError> {
        let tags: OllamaTagsSerialOut = self.get_tags()?;

        Ok(tags.models.into_iter().map(|tag| tag.name).collect())
    }

    pub fn describe_model(&self, model: &str) -> Result<ModelDescription, AppError> {
        let tags: OllamaTagsSerialOut = self.get_tags()?;
        let latest_name: String = format!("{model}:latest");
        let size_bytes: u64 = tags
            .models
            .iter()
            .find(|tag| tag.name == model || tag.name == latest_name)
            .map(|tag| tag.size)
            .unwrap_or(0);

        let response: Response = self
            .client
            .post(format!("{}/api/show", self.base_url))
            .json(&json!({ "model": model }))
            .send()
            .map_err(request_failure)?;
        let show: OllamaShowSerialOut = parse_success(response)?;

        Ok(ModelDescription {
            family: show.details.family,
            parameter_size: show.details.parameter_size,
            quantization_level: show.details.quantization_level,
            size_bytes,
        })
    }

    pub fn chat(&self, request: &ChatRequest) -> Result<ChatReply, AppError> {
        let thinking_supported: bool = !self
            .models_without_thinking
            .lock()
            .expect("the thinking set lock is not poisoned")
            .contains(request.model);

        if thinking_supported {
            let response: Response = self.send_chat(request, true)?;
            let status_is_bad_request: bool = response.status() == reqwest::StatusCode::BAD_REQUEST;

            if !status_is_bad_request {
                return into_reply(parse_success(response)?);
            }

            let body: String = response.text().map_err(request_failure)?;

            if !body.contains("think") {
                return Err(AppError::from(format!(
                    "the chat request was rejected; [model={} body={body}]",
                    request.model
                )));
            }

            self.models_without_thinking
                .lock()
                .expect("the thinking set lock is not poisoned")
                .insert(request.model.to_string());
        }

        let response: Response = self.send_chat(request, false)?;

        into_reply(parse_success(response)?)
    }

    fn get_tags(&self) -> Result<OllamaTagsSerialOut, AppError> {
        let response: Response =
            self.client.get(format!("{}/api/tags", self.base_url)).send().map_err(request_failure)?;

        parse_success(response)
    }

    fn send_chat(&self, request: &ChatRequest, with_thinking_disabled: bool) -> Result<Response, AppError> {
        let messages: Vec<Value> = request
            .messages
            .iter()
            .map(|message| json!({ "role": message.role.as_str(), "content": message.text }))
            .collect();

        let mut body: Value = json!({
            "model": request.model,
            "stream": false,
            "messages": messages,
            "format": request.response_schema,
            "keep_alive": KEEP_MODEL_LOADED,
            "options": {
                "temperature": request.temperature,
                "seed": request.seed,
                "num_predict": MAXIMUM_RESPONSE_TOKENS,
            },
        });

        if with_thinking_disabled {
            body["think"] = Value::Bool(false);
        }

        let response: Response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&body)
            .send()
            .map_err(request_failure)?;

        Ok(response)
    }
}

fn parse_success<Parsed: serde::de::DeserializeOwned>(response: Response) -> Result<Parsed, AppError> {
    let status: reqwest::StatusCode = response.status();

    if !status.is_success() {
        let body: String = response.text().unwrap_or_default();

        return Err(AppError::from(format!(
            "Ollama answered with an error; [status={status} body={body}]"
        )));
    }

    let parsed: Parsed = response.json().map_err(request_failure)?;

    Ok(parsed)
}

fn request_failure(error: reqwest::Error) -> AppError {
    AppError::from_error("the request to Ollama failed", Box::new(error))
}

fn into_reply(chat: OllamaChatSerialOut) -> Result<ChatReply, AppError> {
    Ok(ChatReply {
        content: chat.message.content,
        total_duration_nanoseconds: chat.total_duration,
        load_duration_nanoseconds: chat.load_duration,
        prompt_token_count: chat.prompt_eval_count,
        response_token_count: chat.eval_count,
        response_duration_nanoseconds: chat.eval_duration,
    })
}
