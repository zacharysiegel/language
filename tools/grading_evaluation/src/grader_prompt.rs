use clap::ValueEnum;
use serde_json::{Value, json};

use crate::grading_case::{ErrorCategoryKind, GradingCase};
use crate::ollama_client::ChatMessage;

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

    pub fn examples(self) -> Vec<&'static FewShotExample> {
        let indexes: &[usize] = match self {
            PromptVariantKind::Plain => &[],
            PromptVariantKind::OneShotMistake => &[SPELLING_MISTAKE],
            PromptVariantKind::OneShotCorrect => &[CORRECT_SUNDAYS],
            PromptVariantKind::FewShot | PromptVariantKind::FewShotInSystem => {
                &[SPELLING_MISTAKE, CORRECT_SUNDAYS, OFF_TOPIC_WITH_MISTAKE]
            }
            PromptVariantKind::FewShotReversed => &[OFF_TOPIC_WITH_MISTAKE, CORRECT_SUNDAYS, SPELLING_MISTAKE],
            PromptVariantKind::FewShotSix => &[
                SPELLING_MISTAKE,
                CORRECT_COLLOQUIAL,
                VERB_MISTAKE,
                CORRECT_SUNDAYS,
                PREPOSITION_MISTAKE,
                OFF_TOPIC_WITH_MISTAKE,
            ],
            PromptVariantKind::FewShotMostlyCorrect => &[CORRECT_SUNDAYS, CORRECT_COLLOQUIAL, SPELLING_MISTAKE],
        };

        indexes.iter().map(|index| &EXAMPLE_POOL[*index]).collect()
    }

    pub fn example_placement(self) -> ExamplePlacementKind {
        match self {
            PromptVariantKind::FewShotInSystem => ExamplePlacementKind::SystemPrompt,
            _ => ExamplePlacementKind::ChatTurns,
        }
    }
}

pub struct FewShotExample {
    pub level: &'static str,
    pub prompt_source: &'static str,
    pub context_source: Option<&'static str>,
    pub submission: &'static str,
    pub response_json: &'static str,
}

const SPELLING_MISTAKE: usize = 0;
const CORRECT_SUNDAYS: usize = 1;
const OFF_TOPIC_WITH_MISTAKE: usize = 2;
const VERB_MISTAKE: usize = 3;
const CORRECT_COLLOQUIAL: usize = 4;
const PREPOSITION_MISTAKE: usize = 5;

pub const EXAMPLE_POOL: [FewShotExample; 6] = [
    FewShotExample {
        level: "a2",
        prompt_source: "Describe what Paulo is eating.",
        context_source: Some("Paulo is at a café. He eats a cheese sandwich and drinks orange juice."),
        submission: "O Paulo come um sanduiche de queijo e bebe suco de laranja no café.",
        response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[{"error_text":"sanduiche","explanation":"The word needs an accent on the i: sanduíche.","kind":"spelling","replacement":"sanduíche"}]}"#,
    },
    FewShotExample {
        level: "a1",
        prompt_source: "What do you do on Sundays?",
        context_source: None,
        submission: "Aos domingos eu almoço com a minha família e descanso.",
        response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[]}"#,
    },
    FewShotExample {
        level: "a1",
        prompt_source: "Describe the weather today.",
        context_source: Some("It is cold and rainy."),
        submission: "Eu gosto de pizza. Minhas irmãs são muito alta.",
        response_json: r#"{"answers_the_question":false,"language_is_portuguese":true,"mistakes":[{"error_text":"alta","explanation":"The adjective must be plural to agree with 'irmãs'.","kind":"agreement","replacement":"altas"}]}"#,
    },
    FewShotExample {
        level: "a2",
        prompt_source: "What did you do yesterday?",
        context_source: None,
        submission: "Ontem eu comeu uma pizza e depois saí com os meus amigos.",
        response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[{"error_text":"comeu","explanation":"The verb must agree with 'eu': comi.","kind":"verb_form","replacement":"comi"}]}"#,
    },
    FewShotExample {
        level: "a1",
        prompt_source: "How was your weekend?",
        context_source: None,
        submission: "A gente foi à praia no sábado e eu tô com um pouco de sono hoje.",
        response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[]}"#,
    },
    FewShotExample {
        level: "a1",
        prompt_source: "Where do you study?",
        context_source: None,
        submission: "Eu estudo em a universidade perto de casa.",
        response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[{"error_text":"em a","explanation":"'em' and 'a' contract to 'na'.","kind":"article_preposition","replacement":"na"}]}"#,
    },
];

pub fn build_messages(variant: PromptVariantKind, case: &GradingCase) -> Vec<ChatMessage> {
    let examples: Vec<&FewShotExample> = variant.examples();
    let placement: ExamplePlacementKind = variant.example_placement();

    let system_examples: &[&FewShotExample] = match placement {
        ExamplePlacementKind::SystemPrompt => &examples,
        ExamplePlacementKind::ChatTurns => &[],
    };
    let mut messages: Vec<ChatMessage> = vec![ChatMessage {
        role: "system",
        content: render_system_prompt(system_examples),
    }];

    if placement == ExamplePlacementKind::ChatTurns {
        for example in &examples {
            messages.push(ChatMessage {
                role: "user",
                content: render_example_input(example),
            });
            messages.push(ChatMessage {
                role: "assistant",
                content: example.response_json.to_string(),
            });
        }
    }

    messages.push(ChatMessage {
        role: "user",
        content: render_user_message(
            &case.level,
            &case.task.prompt_source,
            case.task.context_source.as_deref(),
            &case.submission,
        ),
    });

    messages
}

fn render_example_input(example: &FewShotExample) -> String {
    render_user_message(
        example.level,
        example.prompt_source,
        example.context_source,
        example.submission,
    )
}

pub fn response_schema() -> Value {
    let category_names: Vec<&str> = ErrorCategoryKind::ALL.iter().map(|category| category.as_str()).collect();

    json!({
        "type": "object",
        "properties": {
            "answers_the_question": { "type": "boolean" },
            "language_is_portuguese": { "type": "boolean" },
            "mistakes": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "error_text": { "type": "string" },
                        "explanation": { "type": "string" },
                        "kind": { "type": "string", "enum": category_names },
                        "replacement": { "type": "string" },
                    },
                    "required": ["error_text", "explanation", "kind", "replacement"],
                },
            },
        },
        "required": ["answers_the_question", "language_is_portuguese", "mistakes"],
    })
}

fn render_user_message(level: &str, prompt_source: &str, context_source: Option<&str>, submission: &str) -> String {
    let mut message: String = format!("Task: {prompt_source}\n");

    if let Some(context) = context_source {
        message.push_str(&format!("Scene facts: {context}\n"));
    }

    message.push_str(&format!("Level: {}\nAnswer: {submission}", level.to_uppercase()));

    message
}

fn render_system_prompt(examples: &[&FewShotExample]) -> String {
    let category_lines: Vec<String> = ErrorCategoryKind::ALL
        .iter()
        .map(|category| format!("- {}: {}", category.as_str(), category.description()))
        .collect();

    let mut prompt: String = format!(
        "You check short written answers from an English-speaking learner of Brazilian Portuguese (pt-BR). \
You are given a task, sometimes facts about the scene, the learner's level and the learner's answer. \
Report only real mistakes in the answer.\n\
\n\
For every mistake give:\n\
- error_text: the wrong word or the smallest wrong phrase, copied exactly from the answer. Never copy a whole sentence for one mistake.\n\
- explanation: one short sentence in English saying why it is wrong.\n\
- kind: one of the kinds below.\n\
- replacement: the text that replaces error_text.\n\
\n\
Kinds:\n\
{categories}\n\
\n\
Rules:\n\
- Spoken Brazilian forms are correct; do not report them: tô, a gente, pra, né, você for 'you'.\n\
- Do not report style, correct but unusual wording, or punctuation.\n\
- Do not report a mistake you are not sure about. If the answer has no mistakes, return an empty mistakes list.\n\
- language_is_portuguese is false when the answer is written in another language.\n\
- answers_the_question is true when the answer is about what the task asked, even if it contains mistakes.",
        categories = category_lines.join("\n"),
    );

    for (index, example) in examples.iter().enumerate() {
        prompt.push_str(&format!(
            "\n\nExample {}:\n{}\nOutput: {}",
            index + 1,
            render_example_input(example),
            example.response_json
        ));
    }

    prompt
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::grader_response;
    use crate::grading_case;

    fn load_shipped_cases() -> Vec<GradingCase> {
        let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cases/pt_br.en.json");

        grading_case::load_cases(&path).expect("the shipped cases file is valid")
    }

    #[test]
    fn example_responses_parse_as_grader_responses() {
        for example in &EXAMPLE_POOL {
            assert!(grader_response::parse_grader_response(example.response_json).is_ok());
        }
    }

    #[test]
    fn examples_are_not_graded_cases() {
        let cases: Vec<GradingCase> = load_shipped_cases();

        for example in &EXAMPLE_POOL {
            assert!(cases.iter().all(|case| case.submission != example.submission));
        }
    }

    #[test]
    fn build_messages_ends_with_the_case_for_every_variant() {
        let cases: Vec<GradingCase> = load_shipped_cases();

        for variant in PromptVariantKind::ALL {
            let messages: Vec<ChatMessage> = build_messages(variant, &cases[0]);
            let last_message: &ChatMessage = messages.last().expect("there is a final message");

            assert_eq!(last_message.role, "user");
            assert!(last_message.content.ends_with(&cases[0].submission));
        }
    }

    #[test]
    fn build_messages_adds_one_exchange_per_example_as_chat_turns() {
        let cases: Vec<GradingCase> = load_shipped_cases();

        for variant in PromptVariantKind::ALL {
            let message_count: usize = build_messages(variant, &cases[0]).len();

            let expected_count: usize = match variant.example_placement() {
                ExamplePlacementKind::ChatTurns => 2 + variant.examples().len() * 2,
                ExamplePlacementKind::SystemPrompt => 2,
            };

            assert_eq!(message_count, expected_count);
        }
    }

    #[test]
    fn build_messages_puts_system_prompt_examples_in_the_system_message() {
        let cases: Vec<GradingCase> = load_shipped_cases();

        let messages: Vec<ChatMessage> = build_messages(PromptVariantKind::FewShotInSystem, &cases[0]);

        assert!(messages[0].content.contains("Example 3:"));
        assert!(messages[0].content.contains(EXAMPLE_POOL[SPELLING_MISTAKE].submission));
    }

    #[test]
    fn variant_names_are_unique() {
        let names: std::collections::HashSet<&str> =
            PromptVariantKind::ALL.iter().map(|variant| variant.as_str()).collect();

        assert_eq!(names.len(), PromptVariantKind::ALL.len());
    }

    #[test]
    fn response_schema_lists_every_error_category() {
        let schema: Value = response_schema();
        let category_names: &Value = &schema["properties"]["mistakes"]["items"]["properties"]["kind"]["enum"];

        assert_eq!(
            category_names.as_array().expect("the categories are an array").len(),
            ErrorCategoryKind::ALL.len()
        );
    }
}
