use crate::course::Curriculum;

const COURSE_ID_SEPARATOR: char = '.';
const COURSE_ID_PART_COUNT: usize = 3;

/// A language tag as it appears in a course id: lowercase, with underscores for the tag's hyphens.
pub fn language_id_part(language_tag: &str) -> String {
    language_tag.to_lowercase().replace('-', "_")
}

/// The problem with the curriculum's id, if any. An id is `<target language>.<source language>.<curriculum name>`, each
/// part lowercase letters, digits and underscores, and the language parts agree with the declared languages.
pub fn course_id_issue(curriculum: &Curriculum) -> Option<String> {
    let parts: Vec<&str> = curriculum.id.split(COURSE_ID_SEPARATOR).collect();
    let parts_well_formed: bool = parts.len() == COURSE_ID_PART_COUNT
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'));

    if !parts_well_formed {
        return Some(format!(
            "course id must be <target language>.<source language>.<curriculum name> in lowercase letters, digits and underscores; [id={}]",
            curriculum.id,
        ));
    }

    let expected_target: String = language_id_part(&curriculum.target_language);
    let expected_source: String = language_id_part(&curriculum.source_language);

    if parts[0] != expected_target || parts[1] != expected_source {
        return Some(format!(
            "course id's languages disagree with the declared languages; [id={} expected_prefix={expected_target}.{expected_source}]",
            curriculum.id,
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn curriculum_with_id(id: &str) -> Curriculum {
        Curriculum {
            id: id.to_string(),
            title: "t".to_string(),
            target_language: "pt-BR".to_string(),
            source_language: "en".to_string(),
            levels: Vec::new(),
            units: Vec::new(),
        }
    }

    #[test]
    fn language_id_part_lowercases_and_replaces_hyphens() {
        assert_eq!(language_id_part("pt-BR"), "pt_br");
        assert_eq!(language_id_part("en"), "en");
    }

    #[test]
    fn course_id_issue_accepts_an_id_matching_the_languages() {
        assert_eq!(course_id_issue(&curriculum_with_id("pt_br.en.foundations")), None);
    }

    #[test]
    fn course_id_issue_rejects_malformed_ids_and_mismatched_languages() {
        assert!(course_id_issue(&curriculum_with_id("pt_br.en")).is_some());
        assert!(course_id_issue(&curriculum_with_id("pt-br.en.foundations")).is_some());
        assert!(course_id_issue(&curriculum_with_id("es.en.foundations")).is_some());
    }
}
