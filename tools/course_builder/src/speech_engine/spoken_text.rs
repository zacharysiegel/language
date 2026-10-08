use crate::speech_engine::SpeechLanguage;

const SENTENCE_PUNCTUATION: &str = ";:,.!?¡¿—…\"«»“”";

/* A lone letter or accented letter is read by its name or by the name of its accent, and speech models are unreliable for one or two sounds on their own, so a function word that cannot stand alone is spoken in a short natural phrase. */
/// The text a speech engine should speak for `text` in `language`.
pub fn spoken_text_for(language: &SpeechLanguage, text: &str) -> String {
    let bare_word: String = text
        .trim()
        .trim_end_matches(|character: char| SENTENCE_PUNCTUATION.contains(character))
        .to_lowercase();

    match language.isolated_word_phrases.iter().find(|(word, _)| *word == bare_word) {
        Some((_, phrase)) => (*phrase).to_string(),
        None => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::speech_engine::BRAZILIAN_PORTUGUESE;

    #[test]
    fn spoken_text_for_puts_a_lone_function_word_in_a_phrase() {
        assert_eq!(spoken_text_for(&BRAZILIAN_PORTUGUESE, "o"), "o livro");
        assert_eq!(spoken_text_for(&BRAZILIAN_PORTUGUESE, "É."), "é bom");
    }

    #[test]
    fn spoken_text_for_leaves_other_text_alone() {
        assert_eq!(spoken_text_for(&BRAZILIAN_PORTUGUESE, "o pé"), "o pé");
        assert_eq!(spoken_text_for(&BRAZILIAN_PORTUGUESE, "Oi, Ana!"), "Oi, Ana!");
    }
}
