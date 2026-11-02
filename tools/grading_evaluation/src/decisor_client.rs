use std::time::Duration;

use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_json::{Value, json};
use shared::AppError;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(180);
const QUESTION_IDENTIFIER: &str = "answer";

pub struct DecisorClient {
    client: Client,
    base_url: String,
}

#[derive(Deserialize)]
struct DecideSerialOut {
    answers: serde_json::Map<String, Value>,
    backend: String,
    model: String,
}

impl DecisorClient {
    pub fn connect(base_url: &str) -> Result<DecisorClient, AppError> {
        let client: Client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|error| AppError::from_error("could not build the decisor client", Box::new(error)))?;

        Ok(DecisorClient {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    pub fn ask_yes_probability(&self, input: &str, question: &str) -> Result<YesProbability, AppError> {
        let body: Value = json!({
            "context": { "input": input, "history": [] },
            "questions": { "questions": [{ "id": QUESTION_IDENTIFIER, "prompt": question, "kind": "Noul" }] },
        });

        let response: Response = self
            .client
            .post(format!("{}/v1/decide", self.base_url))
            .json(&body)
            .send()
            .map_err(|error| AppError::from_error("the request to decisor failed", Box::new(error)))?;

        let status: reqwest::StatusCode = response.status();

        if !status.is_success() {
            let response_body: String = response.text().unwrap_or_default();

            return Err(AppError::from(format!(
                "decisor answered with an error; [status={status} body={response_body}]"
            )));
        }

        let decided: DecideSerialOut = response
            .json()
            .map_err(|error| AppError::from_error("could not read decisor's answer", Box::new(error)))?;

        let probability: Option<f64> = decided
            .answers
            .get(QUESTION_IDENTIFIER)
            .and_then(|answer| answer.get("Noul"))
            .and_then(|noul| noul.as_f64());

        match probability {
            Some(probability) => Ok(YesProbability {
                probability,
                backend: decided.backend,
                model: decided.model,
            }),
            None => Err(AppError::from(format!(
                "decisor's answer has no yes probability; [answers={:?}]",
                decided.answers
            ))),
        }
    }
}

pub struct YesProbability {
    pub probability: f64,
    pub backend: String,
    pub model: String,
}
