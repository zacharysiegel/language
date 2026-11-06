use crate::course::Exercise;
use crate::practice::AnswerStatus;

pub const PASS_PROPORTION: f64 = 0.8;
pub const QUICK_PRACTICE_EXERCISE_COUNT: usize = 10;

pub fn pass_percent() -> u32 {
    (PASS_PROPORTION * 100.0).round() as u32
}

pub fn scored_exercise_count(exercises: &[Exercise]) -> usize {
    exercises.iter().filter(|exercise| exercise.kind().is_scored()).count()
}

/// The quiz runs every exercise of a unit and counts toward completing it; quick practice samples a few.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeModeKind {
    Full,
    Quick,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionScore {
    pub correct_count: usize,
    pub total_count: usize,
}

impl SessionScore {
    /// `statuses` are the answers to the first of `exercises`; a scored exercise without one counts as missed.
    pub fn of_answers(exercises: &[Exercise], statuses: &[AnswerStatus]) -> SessionScore {
        SessionScore {
            correct_count: statuses.iter().filter(|status| status.is_correct()).count(),
            total_count: scored_exercise_count(exercises),
        }
    }

    pub fn proportion(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }

        self.correct_count as f64 / self.total_count as f64
    }

    pub fn passed(&self) -> bool {
        self.proportion() >= PASS_PROPORTION
    }

    pub fn percent(&self) -> u32 {
        (self.proportion() * 100.0).round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::{MultipleChoiceExercise, OpenAnswerExercise};

    fn choice() -> Exercise {
        Exercise::MultipleChoice(MultipleChoiceExercise {
            prompt: "Which is a greeting?".to_string(),
            choices: vec!["Oi".to_string(), "Casa".to_string()],
            answer_index: 0,
            explanation: None,
        })
    }

    fn open_answer() -> Exercise {
        Exercise::OpenAnswer(OpenAnswerExercise {
            prompt_source: "Introduce yourself.".to_string(),
            context_source: None,
            example_answers: vec!["Oi, eu sou a Ana.".to_string()],
            explanation: None,
            clip_key: None,
        })
    }

    #[test]
    fn of_answers_counts_only_scored_exercises_and_unanswered_ones_as_missed() {
        let exercises: Vec<Exercise> = vec![choice(), open_answer(), choice(), choice()];
        let statuses: Vec<AnswerStatus> = vec![AnswerStatus::Correct, AnswerStatus::Unscored, AnswerStatus::Wrong];

        assert_eq!(SessionScore::of_answers(&exercises, &statuses), SessionScore { correct_count: 1, total_count: 3 });
    }

    #[test]
    fn passed_requires_the_pass_proportion() {
        assert!(SessionScore { correct_count: 4, total_count: 5 }.passed());
        assert!(!SessionScore { correct_count: 3, total_count: 5 }.passed());
    }

    #[test]
    fn proportion_of_an_empty_session_is_zero() {
        assert_eq!(SessionScore { correct_count: 0, total_count: 0 }.proportion(), 0.0);
    }

    #[test]
    fn percent_rounds_to_the_nearest_whole_number() {
        assert_eq!(SessionScore { correct_count: 2, total_count: 3 }.percent(), 67);
    }
}
