use serde::{Deserialize, Serialize};

// Ollama writes a schema's properties in alphabetical order, so the wire names sort in the order the model should write them.
#[derive(Debug, Deserialize, Serialize)]
pub struct GraderResponse {
    pub answers_the_question: bool,
    #[serde(rename = "language_is_portuguese")]
    pub written_in_portuguese: bool,
    #[serde(rename = "mistakes")]
    pub findings: Vec<GraderFinding>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraderFinding {
    #[serde(rename = "error_text")]
    pub quote: String,
    pub explanation: String,
    #[serde(rename = "kind")]
    pub category: String,
    #[serde(rename = "replacement")]
    pub correction: String,
}

pub fn parse_grader_response(content: &str) -> Result<GraderResponse, serde_json::Error> {
    serde_json::from_str(content)
}
