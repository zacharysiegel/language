use clap::ValueEnum;
use shared::grading;
use shared::grading::{ChatMessage, GraderExample, GraderPromptKind};

use crate::grading_case::GradingCase;

pub const GRADER_PROMPT: GraderPromptKind = GraderPromptKind::BrazilianPortugueseTaughtInEnglish;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PromptVariantKind {
    Plain,
    OneShotMistake,
    OneShotCorrect,
    FewShot,
    FewShotReversed,
    FewShotSix,
    FewShotMostlyCorrect,
    FewShotInSystem,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExamplePlacementKind {
    ChatTurns,
    SystemPrompt,
}

impl PromptVariantKind {
    pub const ALL: [PromptVariantKind; 8] = [
        PromptVariantKind::Plain,
        PromptVariantKind::OneShotMistake,
        PromptVariantKind::OneShotCorrect,
        PromptVariantKind::FewShot,
        PromptVariantKind::FewShotReversed,
        PromptVariantKind::FewShotSix,
        PromptVariantKind::FewShotMostlyCorrect,
        PromptVariantKind::FewShotInSystem,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            PromptVariantKind::Plain => "plain",
            PromptVariantKind::OneShotMistake => "one_shot_mistake",
            PromptVariantKind::OneShotCorrect => "one_shot_correct",
            PromptVariantKind::FewShot => "few_shot",
            PromptVariantKind::FewShotReversed => "few_shot_reversed",
            PromptVariantKind::FewShotSix => "few_shot_six",
            PromptVariantKind::FewShotMostlyCorrect => "few_shot_mostly_correct",
            PromptVariantKind::FewShotInSystem => "few_shot_in_system",
        }
    }

    pub fn examples(self) -> Vec<&'static GraderExample> {
        let examples: &[&'static GraderExample] = match self {
            PromptVariantKind::Plain => &[],
            PromptVariantKind::OneShotMistake => &[&grading::SPELLING_MISTAKE_EXAMPLE],
            PromptVariantKind::OneShotCorrect => &[&grading::CORRECT_SUNDAYS_EXAMPLE],
            PromptVariantKind::FewShot | PromptVariantKind::FewShotInSystem => &[
                &grading::SPELLING_MISTAKE_EXAMPLE,
                &grading::CORRECT_SUNDAYS_EXAMPLE,
                &grading::OFF_TOPIC_WITH_MISTAKE_EXAMPLE,
            ],
            PromptVariantKind::FewShotReversed => &[
                &grading::OFF_TOPIC_WITH_MISTAKE_EXAMPLE,
                &grading::CORRECT_SUNDAYS_EXAMPLE,
                &grading::SPELLING_MISTAKE_EXAMPLE,
            ],
            PromptVariantKind::FewShotSix => &[
                &grading::SPELLING_MISTAKE_EXAMPLE,
                &grading::CORRECT_COLLOQUIAL_EXAMPLE,
                &grading::VERB_MISTAKE_EXAMPLE,
                &grading::CORRECT_SUNDAYS_EXAMPLE,
                &grading::PREPOSITION_MISTAKE_EXAMPLE,
                &grading::OFF_TOPIC_WITH_MISTAKE_EXAMPLE,
            ],
            PromptVariantKind::FewShotMostlyCorrect => &[
                &grading::CORRECT_SUNDAYS_EXAMPLE,
                &grading::CORRECT_COLLOQUIAL_EXAMPLE,
                &grading::SPELLING_MISTAKE_EXAMPLE,
            ],
        };

        examples.to_vec()
    }

    pub fn example_placement(self) -> ExamplePlacementKind {
        match self {
            PromptVariantKind::FewShotInSystem => ExamplePlacementKind::SystemPrompt,
            _ => ExamplePlacementKind::ChatTurns,
        }
    }
}

pub fn build_messages(variant: PromptVariantKind, case: &GradingCase) -> Vec<ChatMessage> {
    let examples: Vec<&GraderExample> = variant.examples();

    let (system_prompt_examples, chat_turn_examples): (&[&GraderExample], &[&GraderExample]) =
        match variant.example_placement() {
            ExamplePlacementKind::SystemPrompt => (&examples, &[]),
            ExamplePlacementKind::ChatTurns => (&[], &examples),
        };

    GRADER_PROMPT.build_messages_with_examples(system_prompt_examples, chat_turn_examples, &case.graded_answer())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::grading_case;

    fn load_shipped_cases() -> Vec<GradingCase> {
        let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cases/pt_br.en.json");

        grading_case::load_cases(&path).expect("the shipped cases file is valid")
    }

    #[test]
    fn examples_are_not_graded_cases() {
        let cases: Vec<GradingCase> = load_shipped_cases();

        for example in GRADER_PROMPT.examples() {
            assert!(cases.iter().all(|case| case.submission != example.graded_answer.submission));
        }
    }

    #[test]
    fn holdout_cases_repeat_neither_tuned_cases_nor_examples() {
        let holdout_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cases/pt_br.en.holdout.json");
        let holdout_cases: Vec<GradingCase> = grading_case::load_cases(&holdout_path).expect("the holdout cases file is valid");
        let tuned_cases: Vec<GradingCase> = load_shipped_cases();

        for holdout_case in &holdout_cases {
            assert!(tuned_cases.iter().all(|case| case.id != holdout_case.id && case.submission != holdout_case.submission));
            assert!(
                GRADER_PROMPT
                    .examples()
                    .iter()
                    .all(|example| example.graded_answer.submission != holdout_case.submission)
            );
        }
    }

    #[test]
    fn build_messages_places_examples_as_chat_turns_or_in_the_system_prompt() {
        let cases: Vec<GradingCase> = load_shipped_cases();

        for variant in PromptVariantKind::ALL {
            let messages: Vec<ChatMessage> = build_messages(variant, &cases[0]);
            let examples: Vec<&GraderExample> = variant.examples();

            match variant.example_placement() {
                ExamplePlacementKind::ChatTurns => assert_eq!(messages.len(), 2 + examples.len() * 2),
                ExamplePlacementKind::SystemPrompt => {
                    assert_eq!(messages.len(), 2);
                    assert!(
                        examples
                            .iter()
                            .all(|example| messages[0].text.contains(example.graded_answer.submission))
                    );
                }
            }
        }
    }

    #[test]
    fn build_messages_few_shot_six_equals_shared_build_messages() {
        let cases: Vec<GradingCase> = load_shipped_cases();

        let evaluated_messages: Vec<ChatMessage> = build_messages(PromptVariantKind::FewShotSix, &cases[0]);
        let shared_messages: Vec<ChatMessage> = GRADER_PROMPT.build_messages(&cases[0].graded_answer());

        assert_eq!(evaluated_messages, shared_messages);
    }

    #[test]
    fn variant_names_are_unique() {
        let names: std::collections::HashSet<&str> =
            PromptVariantKind::ALL.iter().map(|variant| variant.as_str()).collect();

        assert_eq!(names.len(), PromptVariantKind::ALL.len());
    }
}
