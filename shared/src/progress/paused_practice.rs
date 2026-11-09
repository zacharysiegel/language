use serde::{Deserialize, Serialize};

use crate::artifact::content_address;
use crate::course::Exercise;
use crate::practice::{AnswerStatus, SessionScore};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedExerciseResult {
    pub status: AnswerStatus,
    /// The answer shown back to the learner after grading.
    pub answer_text: String,
}

impl SavedExerciseResult {
    pub fn is_missed(&self) -> bool {
        self.status.is_missed()
    }
}

/// A quiz of one unit, saved after every graded answer so it can be left and continued exactly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PausedPractice {
    /// The `exercise_list_fingerprint` of the quiz's exercises when the practice started.
    pub exercise_list_fingerprint: String,
    /// Indices into the quiz's exercises, in the order the practice asks them.
    pub exercise_order: Vec<usize>,
    /// One per answered question, in the order asked.
    pub results: Vec<SavedExerciseResult>,
}

impl PausedPractice {
    pub fn new(exercises: &[Exercise]) -> PausedPractice {
        PausedPractice {
            exercise_list_fingerprint: exercise_list_fingerprint(exercises),
            exercise_order: (0..exercises.len()).collect(),
            results: Vec::new(),
        }
    }

    /// Equal to `question_count` once every question is answered.
    pub fn next_question_index(&self) -> usize {
        self.results.len()
    }

    pub fn question_count(&self) -> usize {
        self.exercise_order.len()
    }

    pub fn is_every_question_answered(&self) -> bool {
        self.next_question_index() >= self.question_count()
    }

    /// Unanswered questions count as missed.
    pub fn score(&self) -> SessionScore {
        let statuses: Vec<AnswerStatus> = self.results.iter().map(|result| result.status).collect();

        SessionScore::of_answers(self.question_count(), &statuses)
    }

    /// Ignores a result once every question is answered.
    pub fn record_result(&mut self, result: SavedExerciseResult) {
        if self.is_every_question_answered() {
            return;
        }

        self.results.push(result);
    }

    pub fn matches_exercises(&self, exercises: &[Exercise]) -> bool {
        let fingerprint_matches: bool = self.exercise_list_fingerprint == exercise_list_fingerprint(exercises);
        let order_in_range: bool = self.exercise_order.iter().all(|exercise_index| *exercise_index < exercises.len());
        let results_in_range: bool = self.results.len() <= self.question_count();

        fingerprint_matches && order_in_range && results_in_range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PausedPracticeStateKind {
    NotPaused,
    Resumable(PausedPractice),
    /// The unit's exercises differ from the ones the paused practice was started with.
    ExercisesChanged,
}

/// Identifies a list of exercises by their questions and answers; equal for lists that differ only in clip keys.
pub fn exercise_list_fingerprint(exercises: &[Exercise]) -> String {
    let exercises_without_clip_keys: Vec<Exercise> = exercises.iter().map(exercise_without_clip_keys).collect();
    // Serializing exercises has no failing case.
    let serialized: Vec<u8> = serde_json::to_vec(&exercises_without_clip_keys).unwrap_or_default();

    content_address::sha256_hex(&serialized)
}

fn exercise_without_clip_keys(exercise: &Exercise) -> Exercise {
    let mut stripped: Exercise = exercise.clone();

    match &mut stripped {
        Exercise::FillBlank(exercise) => exercise.clip_key = None,
        Exercise::TranslateToTarget(exercise) => exercise.clip_key = None,
        Exercise::OrderWords(exercise) => exercise.clip_key = None,
        Exercise::ListeningChoice(exercise) => exercise.clip_key = None,
        Exercise::Dictation(exercise) => exercise.clip_key = None,
        Exercise::OpenResponse(exercise) => exercise.clip_key = None,
        Exercise::Match(exercise) => {
            for pair in &mut exercise.pairs {
                pair.clip_key = None;
            }
        },
        Exercise::MultipleChoice(_) => {},
    }

    stripped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::{course_test_fixtures, DictationExercise};

    fn dictation(clip_key: Option<&str>) -> Exercise {
        Exercise::Dictation(DictationExercise {
            audio_target: "Bom dia.".to_string(),
            hint_source: "Good morning.".to_string(),
            explanation: None,
            clip_key: clip_key.map(str::to_string),
        })
    }

    fn result(status: AnswerStatus) -> SavedExerciseResult {
        SavedExerciseResult {
            status,
            answer_text: "sim".to_string(),
        }
    }

    #[test]
    fn exercise_list_fingerprint_ignores_clip_keys() {
        let with_clip_key: String = exercise_list_fingerprint(&[course_test_fixtures::multiple_choice("a"), dictation(Some("bom-dia-clip"))]);
        let without_clip_key: String = exercise_list_fingerprint(&[course_test_fixtures::multiple_choice("a"), dictation(None)]);

        assert_eq!(with_clip_key, without_clip_key);
    }

    #[test]
    fn exercise_list_fingerprint_changes_with_a_question_or_the_order() {
        let original: String = exercise_list_fingerprint(&course_test_fixtures::multiple_choice_exercises(&["a", "b"]));

        assert_ne!(exercise_list_fingerprint(&course_test_fixtures::multiple_choice_exercises(&["a", "c"])), original);
        assert_ne!(exercise_list_fingerprint(&course_test_fixtures::multiple_choice_exercises(&["b", "a"])), original);
        assert_ne!(exercise_list_fingerprint(&[course_test_fixtures::multiple_choice("a")]), original);
    }

    #[test]
    fn new_asks_every_exercise_in_the_unit_order() {
        let paused_practice: PausedPractice = PausedPractice::new(&course_test_fixtures::multiple_choice_exercises(&["a", "b", "c"]));

        assert_eq!(paused_practice.exercise_order, vec![0, 1, 2]);
        assert_eq!(paused_practice.next_question_index(), 0);
        assert_eq!(paused_practice.question_count(), 3);
    }

    #[test]
    fn record_result_ignores_results_after_the_last_question() {
        let mut paused_practice: PausedPractice = PausedPractice::new(&[course_test_fixtures::multiple_choice("a")]);

        paused_practice.record_result(result(AnswerStatus::Correct));
        paused_practice.record_result(result(AnswerStatus::Wrong));

        assert_eq!(paused_practice.results, vec![result(AnswerStatus::Correct)]);
        assert!(paused_practice.is_every_question_answered());
    }

    #[test]
    fn score_counts_accent_mistakes_as_correct_and_unanswered_questions_as_missed() {
        let mut paused_practice: PausedPractice = PausedPractice::new(&course_test_fixtures::multiple_choice_exercises(&["a", "b", "c", "d"]));

        paused_practice.record_result(result(AnswerStatus::Correct));
        paused_practice.record_result(result(AnswerStatus::AccentMistake));
        paused_practice.record_result(result(AnswerStatus::Wrong));

        assert_eq!(paused_practice.score(), SessionScore { correct_count: 2, total_count: 4 });
    }

    #[test]
    fn deserialize_reads_a_paused_practice_saved_with_unscored_exercise_indices() {
        let saved_json: &str = r#"{"exercise_list_fingerprint":"f","exercise_order":[0,1,2],"unscored_exercise_indices":[1],"results":[{"status":"correct","answer_text":"sim"},{"status":"unscored","answer_text":"Oi"}]}"#;

        let paused_practice: PausedPractice = serde_json::from_str(saved_json).unwrap();

        assert_eq!(paused_practice.exercise_order, vec![0, 1, 2]);
        assert_eq!(paused_practice.results, vec![result(AnswerStatus::Correct), SavedExerciseResult {
            status: AnswerStatus::Unscored,
            answer_text: "Oi".to_string(),
        }]);
    }

    #[test]
    fn matches_exercises_only_for_the_exercises_it_started_with() {
        let exercises: Vec<Exercise> = course_test_fixtures::multiple_choice_exercises(&["a", "b"]);
        let paused_practice: PausedPractice = PausedPractice::new(&exercises);

        assert!(paused_practice.matches_exercises(&exercises));
        assert!(!paused_practice.matches_exercises(&course_test_fixtures::multiple_choice_exercises(&["a", "changed"])));
    }

    #[test]
    fn matches_exercises_rejects_an_order_outside_the_exercises() {
        let exercises: Vec<Exercise> = course_test_fixtures::multiple_choice_exercises(&["a", "b"]);
        let mut paused_practice: PausedPractice = PausedPractice::new(&exercises);

        paused_practice.exercise_order = vec![0, 2];

        assert!(!paused_practice.matches_exercises(&exercises));
    }
}
