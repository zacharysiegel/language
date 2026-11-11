/// The count followed by the singular noun when the count is 1, and by the plural noun otherwise.
pub fn counted_noun_phrase(count: usize, singular_noun: &str, plural_noun: &str) -> String {
    let noun: &str = if count == 1 { singular_noun } else { plural_noun };

    format!("{count} {noun}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counted_noun_phrase_takes_the_singular_noun_for_one_only() {
        assert_eq!(counted_noun_phrase(1, "attempt", "attempts"), "1 attempt");
        assert_eq!(counted_noun_phrase(0, "attempt", "attempts"), "0 attempts");
        assert_eq!(counted_noun_phrase(2, "attempt", "attempts"), "2 attempts");
        assert_eq!(counted_noun_phrase(11, "new word", "new words"), "11 new words");
    }
}
