/* English names, since the apps' own words are in English; keyed by a tag's primary language subtag. */
const LANGUAGE_NAMES: [(&str, &str); 12] = [
    ("de", "German"),
    ("en", "English"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("it", "Italian"),
    ("ja", "Japanese"),
    ("ko", "Korean"),
    ("nl", "Dutch"),
    ("pt", "Portuguese"),
    ("ru", "Russian"),
    ("sv", "Swedish"),
    ("zh", "Chinese"),
];

/// The language's name for the apps to show, or the tag itself for a language without one.
pub fn language_name(language_tag: &str) -> String {
    let primary_subtag: String = language_tag.split(['-', '_']).next().unwrap_or(language_tag).to_lowercase();

    LANGUAGE_NAMES
        .iter()
        .find(|(subtag, _)| *subtag == primary_subtag)
        .map_or_else(|| language_tag.to_string(), |(_, name)| name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_name_reads_the_primary_subtag_and_falls_back_to_the_tag() {
        assert_eq!(language_name("pt-BR"), "Portuguese");
        assert_eq!(language_name("en"), "English");
        assert_eq!(language_name("tlh"), "tlh");
    }
}
