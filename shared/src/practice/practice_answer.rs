const PUNCTUATION_CHARACTERS: &str = ".,!?;:\"“”¿¡…";
const APOSTROPHE_VARIANTS: &str = "’‘`´";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerStatus {
    Correct,
    AccentMistake,
    Wrong,
}

impl AnswerStatus {
    pub fn is_correct(self) -> bool {
        self != AnswerStatus::Wrong
    }
}

pub fn normalize_answer(text: &str) -> String {
    let folded: String = text
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
        .map(strip_accent)
        .collect()
}

fn is_combining_mark(character: char) -> bool {
    ('\u{0300}'..='\u{036f}').contains(&character)
}

fn strip_accent(character: char) -> char {
    match character {
        'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        unaccented => unaccented,
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
}
