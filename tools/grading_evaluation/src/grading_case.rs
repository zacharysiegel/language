use std::collections::HashSet;
use std::path::Path;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use shared::AppError;
use shared::grading::{CharacterSpan, ErrorCategoryKind, GradedAnswer, text_span};

const LEVELS: [&str; 4] = ["a1", "a2", "b1", "b2"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GradingCaseKind {
    Control,
    Erroneous,
    OffTopic,
    NotPortuguese,
}

impl GradingCaseKind {
    pub const ALL: [GradingCaseKind; 4] = [
        GradingCaseKind::Control,
        GradingCaseKind::Erroneous,
        GradingCaseKind::OffTopic,
        GradingCaseKind::NotPortuguese,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            GradingCaseKind::Control => "control",
            GradingCaseKind::Erroneous => "erroneous",
            GradingCaseKind::OffTopic => "off_topic",
            GradingCaseKind::NotPortuguese => "not_portuguese",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GradingCase {
    pub id: String,
    pub level: String,
    pub task: GradingTask,
    pub submission: String,
    pub reference: ReferenceAnnotation,
}

impl GradingCase {
    pub fn kind(&self) -> GradingCaseKind {
        if !self.reference.written_in_portuguese {
            return GradingCaseKind::NotPortuguese;
        }

        if !self.reference.answers_the_question {
            return GradingCaseKind::OffTopic;
        }

        if self.reference.errors.is_empty() {
            return GradingCaseKind::Control;
        }

        GradingCaseKind::Erroneous
    }

    pub fn fingerprint(&self) -> String {
        let mut hasher: Sha256 = Sha256::new();

        for part in [
            self.level.as_str(),
            self.task.prompt_source.as_str(),
            self.task.context_source.as_deref().unwrap_or(""),
            self.submission.as_str(),
        ] {
            hasher.update(part.as_bytes());
            hasher.update([0u8]);
        }

        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    pub fn graded_answer(&self) -> GradedAnswer<'_> {
        GradedAnswer {
            level: &self.level,
            prompt_source: &self.task.prompt_source,
            context_source: self.task.context_source.as_deref(),
            submission: &self.submission,
        }
    }

    pub fn reference_spans(&self) -> Vec<Option<CharacterSpan>> {
        self.reference
            .errors
            .iter()
            .map(|reference_error| text_span::locate_text(&self.submission, &reference_error.wrong_text, &[]))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GradingTask {
    pub prompt_source: String,
    #[serde(default)]
    pub context_source: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceAnnotation {
    #[serde(default = "default_true")]
    pub written_in_portuguese: bool,
    #[serde(default = "default_true")]
    pub answers_the_question: bool,
    #[serde(default)]
    pub errors: Vec<ReferenceError>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceError {
    pub wrong_text: String,
    pub category: ErrorCategoryKind,
    pub accepted_corrections: Vec<String>,
}

fn default_true() -> bool {
    true
}

pub fn load_cases(path: &Path) -> Result<Vec<GradingCase>, AppError> {
    let content: String = std::fs::read_to_string(path).map_err(|error| {
        AppError::from(format!(
            "could not read the grading cases; [path={} error={error}]",
            path.display()
        ))
    })?;

    let mut deserializer: serde_json::Deserializer<serde_json::de::StrRead<'_>> =
        serde_json::Deserializer::from_str(&content);
    let cases: Vec<GradingCase> = serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
        AppError::from(format!(
            "could not parse the grading cases; [path={} error={error}]",
            path.display()
        ))
    })?;

    validate_cases(&cases)?;

    Ok(cases)
}

pub fn validate_cases(cases: &[GradingCase]) -> Result<(), AppError> {
    let mut seen_ids: HashSet<&str> = HashSet::new();

    for case in cases {
        let inserted: bool = seen_ids.insert(&case.id);

        if !inserted {
            return Err(AppError::from(format!("duplicate grading case id; [id={}]", case.id)));
        }

        validate_case(case)?;
    }

    Ok(())
}

fn validate_case(case: &GradingCase) -> Result<(), AppError> {
    if !LEVELS.contains(&case.level.as_str()) {
        return Err(AppError::from(format!(
            "unknown level; [id={} level={}]",
            case.id, case.level
        )));
    }

    if case.submission.trim().is_empty() {
        return Err(AppError::from(format!("empty submission; [id={}]", case.id)));
    }

    if !case.reference.written_in_portuguese && !case.reference.errors.is_empty() {
        return Err(AppError::from(format!(
            "a submission not in Portuguese cannot list errors; [id={}]",
            case.id
        )));
    }

    for reference_error in &case.reference.errors {
        let located: Option<CharacterSpan> = text_span::locate_text(&case.submission, &reference_error.wrong_text, &[]);

        if located.is_none() {
            return Err(AppError::from(format!(
                "the wrong text is not in the submission; [id={} wrong_text={}]",
                case.id, reference_error.wrong_text
            )));
        }

        if reference_error.accepted_corrections.is_empty() {
            return Err(AppError::from(format!(
                "no accepted correction; [id={} wrong_text={}]",
                case.id, reference_error.wrong_text
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case_with_reference(reference: ReferenceAnnotation) -> GradingCase {
        GradingCase {
            id: "case".to_string(),
            level: "a1".to_string(),
            task: GradingTask {
                prompt_source: "What is your name?".to_string(),
                context_source: None,
            },
            submission: "Eu chamo Pedro.".to_string(),
            reference,
        }
    }

    #[test]
    fn kind_is_not_portuguese_before_anything_else() {
        let case: GradingCase = case_with_reference(ReferenceAnnotation {
            written_in_portuguese: false,
            answers_the_question: false,
            errors: Vec::new(),
        });

        assert_eq!(case.kind(), GradingCaseKind::NotPortuguese);
    }

    #[test]
    fn kind_is_off_topic_when_the_answer_ignores_the_task() {
        let case: GradingCase = case_with_reference(ReferenceAnnotation {
            written_in_portuguese: true,
            answers_the_question: false,
            errors: Vec::new(),
        });

        assert_eq!(case.kind(), GradingCaseKind::OffTopic);
    }

    #[test]
    fn kind_is_control_without_errors() {
        let case: GradingCase = case_with_reference(ReferenceAnnotation {
            written_in_portuguese: true,
            answers_the_question: true,
            errors: Vec::new(),
        });

        assert_eq!(case.kind(), GradingCaseKind::Control);
    }

    #[test]
    fn validate_cases_rejects_wrong_text_absent_from_the_submission() {
        let case: GradingCase = case_with_reference(ReferenceAnnotation {
            written_in_portuguese: true,
            answers_the_question: true,
            errors: vec![ReferenceError {
                wrong_text: "Ana".to_string(),
                category: ErrorCategoryKind::Pronoun,
                accepted_corrections: vec!["me chamo".to_string()],
            }],
        });

        assert!(validate_cases(&[case]).is_err());
    }
}
