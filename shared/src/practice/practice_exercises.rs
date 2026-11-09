use crate::course::{Exercise, ExerciseKind};
use crate::practice;

pub const QUICK_PRACTICE_EXERCISE_COUNT: usize = 10;

/// The exercises of a unit other than its open responses, in the unit's order.
pub fn quiz_exercises(unit_exercises: &[Exercise]) -> Vec<Exercise> {
    unit_exercises
        .iter()
        .filter(|exercise| exercise.kind() != ExerciseKind::OpenResponse)
        .cloned()
        .collect()
}

/// `random` yields values in `[0, 1)`.
pub fn quick_practice_exercises(unit_exercises: &[Exercise], random: impl FnMut() -> f64) -> Vec<Exercise> {
    let mut sampled: Vec<Exercise> = practice::shuffled(&quiz_exercises(unit_exercises), random);

    sampled.truncate(QUICK_PRACTICE_EXERCISE_COUNT);

    sampled
}

/// The open responses of a unit, in the unit's order.
pub fn open_response_exercises(unit_exercises: &[Exercise]) -> Vec<Exercise> {
    unit_exercises
        .iter()
        .filter(|exercise| exercise.kind() == ExerciseKind::OpenResponse)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::course_test_fixtures;

    fn unit_exercises() -> Vec<Exercise> {
        vec![
            course_test_fixtures::multiple_choice("a"),
            course_test_fixtures::open_response("first"),
            course_test_fixtures::multiple_choice("b"),
            course_test_fixtures::open_response("second"),
            course_test_fixtures::multiple_choice("c"),
        ]
    }

    #[test]
    fn quiz_exercises_leaves_out_open_responses_and_keeps_the_unit_order() {
        assert_eq!(quiz_exercises(&unit_exercises()), course_test_fixtures::multiple_choice_exercises(&["a", "b", "c"]));
    }

    #[test]
    fn quick_practice_exercises_leaves_out_open_responses_and_takes_at_most_the_count() {
        let many_exercises: Vec<Exercise> = (0..QUICK_PRACTICE_EXERCISE_COUNT * 2)
            .flat_map(|exercise_index| {
                [
                    course_test_fixtures::multiple_choice(&exercise_index.to_string()),
                    course_test_fixtures::open_response("task"),
                ]
            })
            .collect();

        let sampled: Vec<Exercise> = quick_practice_exercises(&many_exercises, || 0.0);

        assert_eq!(sampled.len(), QUICK_PRACTICE_EXERCISE_COUNT);
        assert!(sampled.iter().all(|exercise| exercise.kind() != ExerciseKind::OpenResponse));
    }

    #[test]
    fn open_response_exercises_holds_only_open_responses_in_the_unit_order() {
        let expected_exercises: Vec<Exercise> = vec![
            course_test_fixtures::open_response("first"),
            course_test_fixtures::open_response("second"),
        ];

        assert_eq!(open_response_exercises(&unit_exercises()), expected_exercises);
    }
}
