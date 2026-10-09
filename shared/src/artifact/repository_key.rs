use crate::artifact::manifest;

pub const COURSE_INDEX_KEY: &str = "course-index.json";
pub const COURSES_DIRECTORY: &str = "courses";
pub const REPOSITORY_LOCK_KEY: &str = ".repository-lock";
pub const RECORDINGS_FILENAME: &str = "recordings.json";

/* The directory in a course's directory that holds its pointers, which is not a publication. */
pub const COURSE_LATEST_DIRECTORY: &str = "latest";

pub const CACHE_CONTROL_POINTER: &str = "public, max-age=300";
pub const CACHE_CONTROL_IMMUTABLE: &str = "public, max-age=31536000, immutable";

pub fn course_directory_key(course_id: &str) -> String {
    format!("{COURSES_DIRECTORY}/{course_id}/")
}

pub fn course_latest_directory_key(course_id: &str) -> String {
    format!("{}{COURSE_LATEST_DIRECTORY}/", course_directory_key(course_id))
}

pub fn course_latest_manifest_key(course_id: &str) -> String {
    format!("{}{}", course_latest_directory_key(course_id), manifest::MANIFEST_FILENAME)
}

/// The pointer for one manifest schema, in the same directory as `latest_manifest_key`.
pub fn course_schema_pointer_key(latest_manifest_key: &str, manifest_schema_version: u32) -> String {
    let directory_prefix: &str = match latest_manifest_key.rfind('/') {
        Some(separator_index) => &latest_manifest_key[..=separator_index],
        None => "",
    };

    format!("{directory_prefix}manifest.{manifest_schema_version}.json")
}

pub fn publication_directory_key(course_id: &str, publication_label: &str) -> String {
    format!("{}{publication_label}/", course_directory_key(course_id))
}

pub fn publication_manifest_key(course_id: &str, publication_label: &str) -> String {
    format!("{}{}", publication_directory_key(course_id, publication_label), manifest::MANIFEST_FILENAME)
}

pub fn publication_recordings_key(course_id: &str, publication_label: &str) -> String {
    format!("{}{RECORDINGS_FILENAME}", publication_directory_key(course_id, publication_label))
}

/// `relative_path` is relative to the publication's directory.
pub fn publication_file_key(course_id: &str, publication_label: &str, relative_path: &str) -> String {
    format!("{}{relative_path}", publication_directory_key(course_id, publication_label))
}

/// A key whose object is replaced in place, so caches must revalidate it.
pub fn is_pointer_key(key: &str) -> bool {
    if key == COURSE_INDEX_KEY {
        return true;
    }

    let Some(within_courses) = key.strip_prefix(&format!("{COURSES_DIRECTORY}/"))
    else {
        return false;
    };
    let Some((_course_id, within_course)) = within_courses.split_once('/')
    else {
        return false;
    };

    within_course.starts_with(&format!("{COURSE_LATEST_DIRECTORY}/"))
}

pub fn cache_control_for_key(key: &str) -> &'static str {
    if is_pointer_key(key) { CACHE_CONTROL_POINTER } else { CACHE_CONTROL_IMMUTABLE }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COURSE_ID: &str = "pt_br.en.foundations";
    const PUBLICATION_LABEL: &str = "2026-10-08+duluth";

    #[test]
    fn keys_render_the_published_layout() {
        assert_eq!(course_directory_key(COURSE_ID), "courses/pt_br.en.foundations/");
        assert_eq!(course_latest_directory_key(COURSE_ID), "courses/pt_br.en.foundations/latest/");
        assert_eq!(course_latest_manifest_key(COURSE_ID), "courses/pt_br.en.foundations/latest/manifest.json");
        assert_eq!(publication_directory_key(COURSE_ID, PUBLICATION_LABEL), "courses/pt_br.en.foundations/2026-10-08+duluth/");
        assert_eq!(
            publication_manifest_key(COURSE_ID, PUBLICATION_LABEL),
            "courses/pt_br.en.foundations/2026-10-08+duluth/manifest.json",
        );
        assert_eq!(
            publication_recordings_key(COURSE_ID, PUBLICATION_LABEL),
            "courses/pt_br.en.foundations/2026-10-08+duluth/recordings.json",
        );
        assert_eq!(
            publication_file_key(COURSE_ID, PUBLICATION_LABEL, "data/course-ab.sqlite.br"),
            "courses/pt_br.en.foundations/2026-10-08+duluth/data/course-ab.sqlite.br",
        );
    }

    #[test]
    fn course_schema_pointer_key_is_the_sibling_of_the_named_pointer() {
        assert_eq!(
            course_schema_pointer_key(&course_latest_manifest_key(COURSE_ID), 3),
            "courses/pt_br.en.foundations/latest/manifest.3.json",
        );
        assert_eq!(course_schema_pointer_key("manifest.json", 3), "manifest.3.json");
    }

    #[test]
    fn publication_directory_keys_of_labels_sharing_a_prefix_do_not_match_each_other() {
        let shorter: String = publication_directory_key(COURSE_ID, "2026-10-08+charlotte");
        let longer: String = publication_directory_key(COURSE_ID, "2026-10-08+charlottesville");

        assert!(!longer.starts_with(&shorter));
    }

    #[test]
    fn is_pointer_key_holds_only_for_the_index_and_a_courses_latest_directory() {
        assert!(is_pointer_key(COURSE_INDEX_KEY));
        assert!(is_pointer_key(&course_latest_manifest_key(COURSE_ID)));
        assert!(is_pointer_key(&course_schema_pointer_key(&course_latest_manifest_key(COURSE_ID), 3)));

        assert!(!is_pointer_key(&publication_manifest_key(COURSE_ID, PUBLICATION_LABEL)));
        assert!(!is_pointer_key(&publication_recordings_key(COURSE_ID, PUBLICATION_LABEL)));
        assert!(!is_pointer_key(&publication_file_key(COURSE_ID, PUBLICATION_LABEL, "data/course-ab.sqlite.br")));
        assert!(!is_pointer_key(&manifest::audio_key("ab")));
        assert!(!is_pointer_key("latest/manifest.json"));
    }

    #[test]
    fn cache_control_for_key_revalidates_pointers_and_keeps_everything_else() {
        assert_eq!(cache_control_for_key(COURSE_INDEX_KEY), CACHE_CONTROL_POINTER);
        assert_eq!(cache_control_for_key(&course_latest_manifest_key(COURSE_ID)), CACHE_CONTROL_POINTER);
        assert_eq!(cache_control_for_key(&publication_manifest_key(COURSE_ID, PUBLICATION_LABEL)), CACHE_CONTROL_IMMUTABLE);
        assert_eq!(cache_control_for_key(&manifest::audio_key("ab")), CACHE_CONTROL_IMMUTABLE);
    }
}
