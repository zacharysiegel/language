/* A lone letter or accented letter is read by its name or by the name of its accent, and speech models are unreliable for one or two sounds on their own, so a function word that cannot stand alone is spoken in a short natural phrase. */
const ISOLATED_WORD_PHRASES: [(&str, &str); 17] = [
    ("o", "o livro"),
    ("a", "a casa"),
    ("os", "os livros"),
    ("as", "as casas"),
    ("e", "e eu"),
    ("é", "é bom"),
    ("à", "à casa"),
    ("às", "às casas"),
    ("de", "de casa"),
    ("do", "do livro"),
    ("da", "da casa"),
    ("no", "no livro"),
    ("na", "na casa"),
    ("em", "em casa"),
    ("ao", "ao livro"),
    ("um", "um livro"),
    ("uns", "uns livros"),
];

const SENTENCE_PUNCTUATION: &str = ";:,.!?¡¿—…\"«»“”";

/// The text a speech engine should speak for `text`.
pub fn spoken_text_for(text: &str) -> String {
    let bare_word: String = text
        .trim()
        .trim_end_matches(|character: char| SENTENCE_PUNCTUATION.contains(character))
        .to_lowercase();

    match ISOLATED_WORD_PHRASES.iter().find(|(word, _)| *word == bare_word) {
        Some((_, phrase)) => (*phrase).to_string(),
        None => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spoken_text_for_puts_a_lone_function_word_in_a_phrase() {
        assert_eq!(spoken_text_for("o"), "o livro");
        assert_eq!(spoken_text_for("É."), "é bom");
    }

    #[test]
    fn spoken_text_for_leaves_other_text_alone() {
        assert_eq!(spoken_text_for("o pé"), "o pé");
        assert_eq!(spoken_text_for("Oi, Ana!"), "Oi, Ana!");
    }
}
