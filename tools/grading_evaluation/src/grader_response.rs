use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GraderResponse {
    pub written_in_portuguese: bool,
    pub answers_the_question: bool,
    pub findings: Vec<GraderFinding>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraderFinding {
    pub quote: String,
    pub category: String,
    pub correction: String,
    pub explanation: String,
}

pub fn parse_grader_response(content: &str) -> Result<GraderResponse, serde_json::Error> {
    serde_json::from_str(content)
}
