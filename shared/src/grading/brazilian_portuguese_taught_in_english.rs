use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::grading::{GradedAnswer, GraderExample};

pub const SPELLING_MISTAKE_EXAMPLE: GraderExample = GraderExample {
    graded_answer: GradedAnswer {
        level: "a2",
        prompt_source: "Describe what Paulo is eating.",
        context_source: Some("Paulo is at a café. He eats a cheese sandwich and drinks orange juice."),
        submission: "O Paulo come um sanduiche de queijo e bebe suco de laranja no café.",
    },
    response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[{"error_text":"sanduiche","explanation":"The word needs an accent on the i: sanduíche.","kind":"spelling","replacement":"sanduíche"}]}"#,
};

pub const CORRECT_COLLOQUIAL_EXAMPLE: GraderExample = GraderExample {
    graded_answer: GradedAnswer {
        level: "a1",
        prompt_source: "How was your weekend?",
        context_source: None,
        submission: "A gente foi à praia no sábado e eu tô com um pouco de sono hoje.",
    },
    response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[]}"#,
};

pub const VERB_MISTAKE_EXAMPLE: GraderExample = GraderExample {
    graded_answer: GradedAnswer {
        level: "a2",
        prompt_source: "What did you do yesterday?",
        context_source: None,
        submission: "Ontem eu comeu uma pizza e depois saí com os meus amigos.",
    },
    response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[{"error_text":"comeu","explanation":"The verb must agree with 'eu': comi.","kind":"verb_form","replacement":"comi"}]}"#,
};

pub const CORRECT_SUNDAYS_EXAMPLE: GraderExample = GraderExample {
    graded_answer: GradedAnswer {
        level: "a1",
        prompt_source: "What do you do on Sundays?",
        context_source: None,
        submission: "Aos domingos eu almoço com a minha família e descanso.",
    },
    response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[]}"#,
};

pub const PREPOSITION_MISTAKE_EXAMPLE: GraderExample = GraderExample {
    graded_answer: GradedAnswer {
        level: "a1",
        prompt_source: "Where do you study?",
        context_source: None,
        submission: "Eu estudo em a universidade perto de casa.",
    },
    response_json: r#"{"answers_the_question":true,"language_is_portuguese":true,"mistakes":[{"error_text":"em a","explanation":"'em' and 'a' contract to 'na'.","kind":"article_preposition","replacement":"na"}]}"#,
};

pub const OFF_TOPIC_WITH_MISTAKE_EXAMPLE: GraderExample = GraderExample {
    graded_answer: GradedAnswer {
        level: "a1",
        prompt_source: "Describe the weather today.",
        context_source: Some("It is cold and rainy."),
        submission: "Eu gosto de pizza. Minhas irmãs são muito alta.",
    },
    response_json: r#"{"answers_the_question":false,"language_is_portuguese":true,"mistakes":[{"error_text":"alta","explanation":"The adjective must be plural to agree with 'irmãs'.","kind":"agreement","replacement":"altas"}]}"#,
};

pub(crate) const GRADER_EXAMPLES: [GraderExample; 6] = [
    SPELLING_MISTAKE_EXAMPLE,
    CORRECT_COLLOQUIAL_EXAMPLE,
    VERB_MISTAKE_EXAMPLE,
    CORRECT_SUNDAYS_EXAMPLE,
    PREPOSITION_MISTAKE_EXAMPLE,
    OFF_TOPIC_WITH_MISTAKE_EXAMPLE,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategoryKind {
    Spelling,
    VerbForm,
    SerEstar,
    Agreement,
    ArticlePreposition,
    WordChoice,
    WordOrder,
    Pronoun,
    NotBrazilian,
    ContentMismatch,
}

impl ErrorCategoryKind {
    pub const ALL: [ErrorCategoryKind; 10] = [
        ErrorCategoryKind::Spelling,
        ErrorCategoryKind::VerbForm,
        ErrorCategoryKind::SerEstar,
        ErrorCategoryKind::Agreement,
        ErrorCategoryKind::ArticlePreposition,
        ErrorCategoryKind::WordChoice,
        ErrorCategoryKind::WordOrder,
        ErrorCategoryKind::Pronoun,
        ErrorCategoryKind::NotBrazilian,
        ErrorCategoryKind::ContentMismatch,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCategoryKind::Spelling => "spelling",
            ErrorCategoryKind::VerbForm => "verb_form",
            ErrorCategoryKind::SerEstar => "ser_estar",
            ErrorCategoryKind::Agreement => "agreement",
            ErrorCategoryKind::ArticlePreposition => "article_preposition",
            ErrorCategoryKind::WordChoice => "word_choice",
            ErrorCategoryKind::WordOrder => "word_order",
            ErrorCategoryKind::Pronoun => "pronoun",
            ErrorCategoryKind::NotBrazilian => "not_brazilian",
            ErrorCategoryKind::ContentMismatch => "content_mismatch",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            ErrorCategoryKind::Spelling => "a misspelled word, including a missing or wrong accent mark or cedilla",
            ErrorCategoryKind::VerbForm => {
                "a wrong conjugation, tense or mood of a verb, or a verb that does not agree with its subject"
            }
            ErrorCategoryKind::SerEstar => "ser used where estar is needed, or the reverse",
            ErrorCategoryKind::Agreement => "a gender or number mismatch between a noun and its article or adjective",
            ErrorCategoryKind::ArticlePreposition => {
                "a missing, extra or wrong article or preposition, including a missing contraction such as no for em o"
            }
            ErrorCategoryKind::WordChoice => {
                "a wrong word for the meaning, a false friend, or a word from another language"
            }
            ErrorCategoryKind::WordOrder => "words in an order a Portuguese speaker would not use",
            ErrorCategoryKind::Pronoun => "a missing or wrong subject, object or reflexive pronoun",
            ErrorCategoryKind::NotBrazilian => "a form or construction used only in European Portuguese",
            ErrorCategoryKind::ContentMismatch => "a statement that contradicts the facts given about the scene",
        }
    }
}

pub(crate) fn response_schema() -> Value {
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

pub(crate) fn render_user_message(graded_answer: &GradedAnswer) -> String {
    let mut message: String = format!("Task: {}\n", graded_answer.prompt_source);

    if let Some(context) = graded_answer.context_source {
        message.push_str(&format!("Scene facts: {context}\n"));
    }

    message.push_str(&format!(
        "Level: {}\nAnswer: {}",
        graded_answer.level.to_uppercase(),
        graded_answer.submission
    ));

    message
}

pub(crate) fn render_system_prompt(examples: &[&GraderExample]) -> String {
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
            render_user_message(&example.graded_answer),
            example.response_json
        ));
    }

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grading::grader_response;

    #[test]
    fn example_responses_parse_as_grader_responses() {
        for example in &GRADER_EXAMPLES {
            assert!(grader_response::parse_grader_response(example.response_json).is_ok());
        }
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

    #[test]
    fn category_names_round_trip_through_serde() {
        for category in ErrorCategoryKind::ALL {
            let serialized: String = serde_json::to_string(&category).expect("a category serializes");

            assert_eq!(serialized, format!("\"{}\"", category.as_str()));
        }
    }
}
