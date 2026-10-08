use shared::course::{Exercise, FillBlankExercise};
use shared::practice::{self, AnswerStatus, SessionScore};
use shared::review;

use crate::course_records::ExerciseRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AnswerStatusRecord {
    Correct,
    AccentMistake,
    Wrong,
}

impl From<AnswerStatus> for AnswerStatusRecord {
    fn from(status: AnswerStatus) -> AnswerStatusRecord {
        match status {
            AnswerStatus::Correct => AnswerStatusRecord::Correct,
            AnswerStatus::AccentMistake => AnswerStatusRecord::AccentMistake,
            AnswerStatus::Wrong => AnswerStatusRecord::Wrong,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, uniffi::Record)]
pub struct SessionScoreRecord {
    pub correct_count: u32,
    pub total_count: u32,
    pub percent: u32,
    pub passed: bool,
}

#[uniffi::export]
pub fn check_typed_answer(typed_text: String, accepted_answers: Vec<String>) -> AnswerStatusRecord {
    practice::check_typed_answer(&typed_text, &accepted_answers).into()
}

#[uniffi::export]
pub fn session_score(correct_count: u32, total_count: u32) -> SessionScoreRecord {
    let score: SessionScore = SessionScore {
        correct_count: correct_count as usize,
        total_count: total_count as usize,
    };

    SessionScoreRecord {
        correct_count,
        total_count,
        percent: score.percent(),
        passed: score.passed(),
    }
}

#[uniffi::export]
pub fn pass_percent() -> u32 {
    (practice::PASS_PROPORTION * 100.0).round() as u32
}

#[uniffi::export]
pub fn quick_practice_exercise_count() -> u32 {
    practice::QUICK_PRACTICE_EXERCISE_COUNT as u32
}

#[uniffi::export]
pub fn review_interval_days() -> Vec<u32> {
    review::REVIEW_INTERVAL_DAYS.to_vec()
}

#[uniffi::export]
pub fn fill_blank_marker() -> String {
    FillBlankExercise::BLANK_MARKER.to_string()
}

/// Text folded for search: lowercase, without punctuation or accents.
#[uniffi::export]
pub fn search_key(text: String) -> String {
    practice::normalize_answer_ignoring_accents(&text)
}

#[uniffi::export]
pub fn exercise_instruction(exercise: ExerciseRecord) -> String {
    Exercise::from(&exercise).kind().instruction().to_string()
}

/// The Portuguese sentence the exercise's clip speaks.
#[uniffi::export]
pub fn exercise_spoken_text(exercise: ExerciseRecord) -> Option<String> {
    Exercise::from(&exercise).spoken_text()
}

#[uniffi::export]
pub fn exercise_prompt_text(exercise: ExerciseRecord) -> String {
    Exercise::from(&exercise).prompt_text()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_typed_answer_reports_an_accent_mistake() {
        let status: AnswerStatusRecord = check_typed_answer("voce e brasileiro".to_string(), vec!["Você é brasileiro.".to_string()]);

        assert_eq!(status, AnswerStatusRecord::AccentMistake);
    }

    #[test]
    fn exercise_spoken_text_completes_a_fill_blank_sentence() {
        let exercise: ExerciseRecord = ExerciseRecord::FillBlank {
            prompt_target: "Eu ___ brasileiro.".to_string(),
            answers: vec!["sou".to_string()],
            hint_source: "I am Brazilian.".to_string(),
            explanation: None,
            clip_key: None,
        };

        assert_eq!(exercise_spoken_text(exercise), Some("Eu sou brasileiro.".to_string()));
    }
}
