use serde::{Deserialize, Serialize};

use crate::course::stress_mark;

const PUNCTUATION_CHARACTERS: &str = ".,!?;:\"“”¿¡…";
const APOSTROPHE_VARIANTS: &str = "’‘`´";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerStatus {
    Correct,
    AccentMistake,
    Wrong,
    Unscored,
}

impl AnswerStatus {
    pub fn is_correct(self) -> bool {
        match self {
            AnswerStatus::Correct | AnswerStatus::AccentMistake => true,
            AnswerStatus::Wrong | AnswerStatus::Unscored => false,
        }
    }

    pub fn is_missed(self) -> bool {
        self == AnswerStatus::Wrong
    }
}

/* Learners type without stress marks. */
pub fn normalize_answer(text: &str) -> String {
    let folded: String = stress_mark::compose_without_stress_marks(text)
        .chars()
        .flat_map(char::to_lowercase)
        .map(|character| {
            if APOSTROPHE_VARIANTS.contains(character) {
                '\''
            } else if PUNCTUATION_CHARACTERS.contains(character) {
                ' '
            } else {
                character
            }
        })
        .collect();

    folded.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn normalize_answer_ignoring_accents(text: &str) -> String {
    normalize_answer(text)
        .chars()
        .filter(|character| !is_combining_mark(*character))
        .flat_map(strip_accent)
        .collect()
}

fn is_combining_mark(character: char) -> bool {
    ('\u{0300}'..='\u{036f}').contains(&character)
}

/* A letter a learner's keyboard may lack is spelled as it is typed without it, which can take two letters. */
fn strip_accent(character: char) -> Vec<char> {
    match character {
        'á' | 'à' | 'â' | 'ã' | 'ä' => vec!['a'],
        'é' | 'è' | 'ê' | 'ë' => vec!['e'],
        'í' | 'ì' | 'î' | 'ï' => vec!['i'],
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' => vec!['o'],
        'ú' | 'ù' | 'û' | 'ü' => vec!['u'],
        'ÿ' => vec!['y'],
        'ç' => vec!['c'],
        'ñ' => vec!['n'],
        'ё' => vec!['е'],
        'ß' => vec!['s', 's'],
        'œ' => vec!['o', 'e'],
        'æ' => vec!['a', 'e'],
        unaccented => vec![unaccented],
    }
}

pub fn check_typed_answer(typed_text: &str, accepted_answers: &[String]) -> AnswerStatus {
    let typed_normalized: String = normalize_answer(typed_text);

    if typed_normalized.is_empty() {
        return AnswerStatus::Wrong;
    }

    let exact_match: bool = accepted_answers
        .iter()
        .any(|accepted| normalize_answer(accepted) == typed_normalized);

    if exact_match {
        return AnswerStatus::Correct;
    }

    let typed_loose: String = normalize_answer_ignoring_accents(typed_text);
    let loose_match: bool = accepted_answers
        .iter()
        .any(|accepted| normalize_answer_ignoring_accents(accepted) == typed_loose);

    if loose_match { AnswerStatus::AccentMistake } else { AnswerStatus::Wrong }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepted(answers: &[&str]) -> Vec<String> {
        answers.iter().map(|answer| answer.to_string()).collect()
    }

    #[test]
    fn check_typed_answer_ignores_case_punctuation_and_spacing() {
        let status: AnswerStatus = check_typed_answer("  eu  MORO em são paulo! ", &accepted(&["Eu moro em São Paulo."]));

        assert_eq!(status, AnswerStatus::Correct);
    }

    #[test]
    fn check_typed_answer_flags_missing_accents() {
        let status: AnswerStatus = check_typed_answer("voce e brasileiro", &accepted(&["Você é brasileiro."]));

        assert_eq!(status, AnswerStatus::AccentMistake);
    }

    #[test]
    fn check_typed_answer_rejects_different_words() {
        let status: AnswerStatus = check_typed_answer("eu moro no Rio", &accepted(&["Eu moro em São Paulo."]));

        assert_eq!(status, AnswerStatus::Wrong);
    }

    #[test]
    fn check_typed_answer_rejects_empty_input() {
        assert_eq!(check_typed_answer("   ", &accepted(&["oi"])), AnswerStatus::Wrong);
    }

    #[test]
    fn check_typed_answer_accepts_any_listed_alternative() {
        let status: AnswerStatus = check_typed_answer("Moro aqui", &accepted(&["Eu moro aqui.", "Moro aqui."]));

        assert_eq!(status, AnswerStatus::Correct);
    }

    #[test]
    fn normalize_answer_ignoring_accents_folds_cedilla_and_tilde() {
        assert_eq!(normalize_answer_ignoring_accents("Coração"), "coracao");
    }

    #[test]
    fn normalize_answer_ignoring_accents_spells_out_letters_a_keyboard_may_lack() {
        assert_eq!(normalize_answer_ignoring_accents("Niño, ёлка, Straße, cœur"), "nino елка strasse coeur");
    }

    #[test]
    fn check_typed_answer_ignores_stress_marks() {
        let status: AnswerStatus = check_typed_answer("Я читаю книгу", &accepted(&["Я чита\u{0301}ю кни\u{0301}гу."]));

        assert_eq!(status, AnswerStatus::Correct);
    }

    #[test]
    fn check_typed_answer_composes_a_decomposed_accent_before_comparing() {
        let status: AnswerStatus = check_typed_answer("esta\u{0301} bien", &accepted(&["Está bien."]));

        assert_eq!(status, AnswerStatus::Correct);
    }
}
