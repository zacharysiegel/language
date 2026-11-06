use serde_json::Value;

use crate::grading::brazilian_portuguese_taught_in_english;
use crate::grading::{ChatMessage, ChatRoleKind};

const BRAZILIAN_PORTUGUESE: &str = "pt-BR";
const ENGLISH: &str = "en";

#[derive(Clone, Copy, Debug)]
pub struct GradedAnswer<'answer_lifetime> {
    pub level: &'answer_lifetime str,
    pub prompt_source: &'answer_lifetime str,
    pub context_source: Option<&'answer_lifetime str>,
    pub submission: &'answer_lifetime str,
}

pub struct GraderExample {
    pub graded_answer: GradedAnswer<'static>,
    pub response_json: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraderPromptKind {
    BrazilianPortugueseTaughtInEnglish,
}

impl GraderPromptKind {
    pub fn examples(self) -> &'static [GraderExample] {
        match self {
            GraderPromptKind::BrazilianPortugueseTaughtInEnglish => {
                &brazilian_portuguese_taught_in_english::GRADER_EXAMPLES
            }
        }
    }

    pub fn response_schema(self) -> Value {
        match self {
            GraderPromptKind::BrazilianPortugueseTaughtInEnglish => {
                brazilian_portuguese_taught_in_english::response_schema()
            }
        }
    }

    pub fn render_system_prompt(self, examples: &[&GraderExample]) -> String {
        match self {
            GraderPromptKind::BrazilianPortugueseTaughtInEnglish => {
                brazilian_portuguese_taught_in_english::render_system_prompt(examples)
            }
        }
    }

    pub fn render_user_message(self, graded_answer: &GradedAnswer) -> String {
        match self {
            GraderPromptKind::BrazilianPortugueseTaughtInEnglish => {
                brazilian_portuguese_taught_in_english::render_user_message(graded_answer)
            }
        }
    }

    /// Every example as a chat exchange before the answer.
    pub fn build_messages(self, graded_answer: &GradedAnswer) -> Vec<ChatMessage> {
        let chat_turn_examples: Vec<&GraderExample> = self.examples().iter().collect();

        self.build_messages_with_examples(&[], &chat_turn_examples, graded_answer)
    }

    pub fn build_messages_with_examples(
        self,
        system_prompt_examples: &[&GraderExample],
        chat_turn_examples: &[&GraderExample],
        graded_answer: &GradedAnswer,
    ) -> Vec<ChatMessage> {
        let mut messages: Vec<ChatMessage> = vec![ChatMessage {
            role: ChatRoleKind::System,
            text: self.render_system_prompt(system_prompt_examples),
        }];

        for example in chat_turn_examples {
            messages.push(ChatMessage {
                role: ChatRoleKind::User,
                text: self.render_user_message(&example.graded_answer),
            });
            messages.push(ChatMessage {
                role: ChatRoleKind::Assistant,
                text: example.response_json.to_string(),
            });
        }

        messages.push(ChatMessage {
            role: ChatRoleKind::User,
            text: self.render_user_message(graded_answer),
        });

        messages
    }
}

pub fn find_grader_prompt(target_language: &str, source_language: &str) -> Option<GraderPromptKind> {
    match (target_language, source_language) {
        (BRAZILIAN_PORTUGUESE, ENGLISH) => Some(GraderPromptKind::BrazilianPortugueseTaughtInEnglish),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRADED_ANSWER: GradedAnswer<'static> = GradedAnswer {
        level: "a1",
        prompt_source: "What is your name?",
        context_source: None,
        submission: "Eu chamo Pedro.",
    };

    #[test]
    fn find_grader_prompt_knows_only_brazilian_portuguese_taught_in_english() {
        assert_eq!(
            find_grader_prompt("pt-BR", "en"),
            Some(GraderPromptKind::BrazilianPortugueseTaughtInEnglish)
        );
        assert_eq!(find_grader_prompt("pt-BR", "es"), None);
        assert_eq!(find_grader_prompt("es", "en"), None);
    }

    #[test]
    fn build_messages_sends_every_example_as_an_exchange_before_the_answer() {
        let grader_prompt: GraderPromptKind = GraderPromptKind::BrazilianPortugueseTaughtInEnglish;

        let messages: Vec<ChatMessage> = grader_prompt.build_messages(&GRADED_ANSWER);
        let roles: Vec<ChatRoleKind> = messages.iter().map(|message| message.role).collect();

        let mut expected_roles: Vec<ChatRoleKind> = vec![ChatRoleKind::System];

        for _ in grader_prompt.examples() {
            expected_roles.extend([ChatRoleKind::User, ChatRoleKind::Assistant]);
        }

        expected_roles.push(ChatRoleKind::User);

        assert_eq!(roles, expected_roles);
        assert_eq!(messages[2].text, grader_prompt.examples()[0].response_json);
        assert_eq!(
            messages.last().expect("there is a final message").text,
            grader_prompt.render_user_message(&GRADED_ANSWER)
        );
    }
}
