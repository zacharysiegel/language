use icu_normalizer::ComposingNormalizerBorrowed;

/// The combining acute accent, with which target text marks a stressed vowel its spelling leaves unmarked.
pub const STRESS_MARK: char = '\u{0301}';

pub fn compose(text: &str) -> String {
    ComposingNormalizerBorrowed::new_nfc().normalize(text).into_owned()
}

/* After composition an acute accent remains a separate character only where no precomposed letter exists, as on a Cyrillic vowel. */
/// `text` in NFC with its stress marks removed and the accents of its other letters kept.
pub fn compose_without_stress_marks(text: &str) -> String {
    compose(text).chars().filter(|character| *character != STRESS_MARK).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_without_stress_marks_removes_a_mark_on_a_cyrillic_vowel() {
        assert_eq!(compose_without_stress_marks("молоко\u{0301}"), "молоко");
    }

    #[test]
    fn compose_without_stress_marks_keeps_a_decomposed_latin_accent() {
        assert_eq!(compose_without_stress_marks("esta\u{0301}"), "está");
    }
}
