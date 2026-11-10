use serde::{Deserialize, Serialize};

use crate::artifact::content_address;
use crate::course::OpenResponseExercise;
use crate::grading::{GraderFinding, GraderPromptIdentity, GraderPromptKind, GraderResponse};

/// The learner's latest answer to one open-response task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedOpenResponse {
    /// The task as it was when answered.
    pub task: OpenResponseTask,
    pub answer_text: String,
    pub answered_milliseconds: i64,
    /// Absent until a grader grades the answer, and after a grading that failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<SavedOpenResponseGrade>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenResponseTask {
    pub level: String,
    pub prompt_source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_source: Option<String>,
}

impl OpenResponseTask {
    pub fn of(level: &str, exercise: &OpenResponseExercise) -> OpenResponseTask {
        OpenResponseTask {
            level: level.to_string(),
            prompt_source: exercise.prompt_source.clone(),
            context_source: exercise.context_source.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedOpenResponseGrade {
    pub grader_prompt: GraderPromptIdentity,
    pub answers_the_question: bool,
    pub written_in_target_language: bool,
    /// Only the findings that apply to the answer.
    pub findings: Vec<SavedGraderFinding>,
}

impl SavedOpenResponseGrade {
    /// `response` holds only the findings that apply to the answer.
    pub fn of_response(grader_prompt: GraderPromptKind, response: &GraderResponse) -> SavedOpenResponseGrade {
        SavedOpenResponseGrade {
            grader_prompt: grader_prompt.identity(),
            answers_the_question: response.answers_the_question,
            written_in_target_language: response.written_in_portuguese,
            findings: response.findings.iter().map(SavedGraderFinding::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedGraderFinding {
    /// The mistaken text, as it appears in the answer.
    pub quote: String,
    pub correction: String,
    pub explanation: String,
    pub category: String,
}

impl From<&GraderFinding> for SavedGraderFinding {
    fn from(finding: &GraderFinding) -> SavedGraderFinding {
        SavedGraderFinding {
            quote: finding.quote.clone(),
            correction: finding.correction.clone(),
            explanation: finding.explanation.clone(),
            category: finding.category.clone(),
        }
    }
}

/// Identifies an open-response task across course content changes that leave its task and scene facts as they are.
pub fn open_response_task_key(unit_id: &str, exercise: &OpenResponseExercise) -> String {
    let task_text: (&str, Option<&str>) = (&exercise.prompt_source, exercise.context_source.as_deref());
    // Serializing strings has no failing case.
    let serialized: Vec<u8> = serde_json::to_vec(&task_text).unwrap_or_default();

    format!("{unit_id}:{}", content_address::sha256_hex(&serialized))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_response(prompt_source: &str, context_source: Option<&str>) -> OpenResponseExercise {
        OpenResponseExercise {
            prompt_source: prompt_source.to_string(),
            context_source: context_source.map(str::to_string),
            example_answers: vec!["Oi, eu sou a Ana.".to_string()],
            explanation: None,
            clip_key: None,
        }
    }

    #[test]
    fn open_response_task_key_ignores_the_example_answers_explanation_and_clip() {
        let original: OpenResponseExercise = open_response("Introduce yourself.", None);
        let mut republished: OpenResponseExercise = original.clone();
        republished.example_answers.push("Olá, meu nome é Ana.".to_string());
        republished.explanation = Some("Say your name.".to_string());
        republished.clip_key = Some("open-response-clip".to_string());

        assert_eq!(open_response_task_key("u", &original), open_response_task_key("u", &republished));
    }

    #[test]
    fn open_response_task_key_changes_with_the_unit_the_task_or_the_scene_facts() {
        let original: String = open_response_task_key("u", &open_response("Describe Paulo.", Some("Paulo is tall.")));

        assert_ne!(open_response_task_key("v", &open_response("Describe Paulo.", Some("Paulo is tall."))), original);
        assert_ne!(open_response_task_key("u", &open_response("Describe Ana.", Some("Paulo is tall."))), original);
        assert_ne!(open_response_task_key("u", &open_response("Describe Paulo.", Some("Paulo is short."))), original);
        assert_ne!(open_response_task_key("u", &open_response("Describe Paulo.", None)), original);
    }
}
