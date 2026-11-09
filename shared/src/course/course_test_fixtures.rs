use crate::course::{Exercise, MultipleChoiceExercise, OpenResponseExercise};

pub fn multiple_choice(prompt: &str) -> Exercise {
    Exercise::MultipleChoice(MultipleChoiceExercise {
        prompt: prompt.to_string(),
        choices: vec!["sim".to_string(), "não".to_string()],
        answer_index: 0,
        explanation: None,
    })
}

pub fn open_response(prompt_source: &str) -> Exercise {
    Exercise::OpenResponse(OpenResponseExercise {
        prompt_source: prompt_source.to_string(),
        context_source: None,
        example_answers: vec!["Oi, eu sou a Ana.".to_string()],
        explanation: None,
        clip_key: None,
    })
}

pub fn multiple_choice_exercises(prompts: &[&str]) -> Vec<Exercise> {
    prompts.iter().copied().map(multiple_choice).collect()
}
