use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::artifact::{manifest, repository_key, schema_version};
use crate::course::course_identity;

pub const COURSE_INDEX_SCHEMA_VERSION: u32 = 1;
pub const COURSE_INDEX_SCHEMA_VERSION_FIELD: &str = "course_index_schema_version";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseIndex {
    pub course_index_schema_version: u32,
    /// By course id.
    pub courses: BTreeMap<String, CourseIndexEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseIndexEntry {
    pub title: String,
    pub target_language: String,
    pub source_language: String,
    /// Relative to the repository base.
    pub latest_manifest_key: String,
    /// A course that exists to test the apps, not to teach.
    pub test_course: bool,
}

impl CourseIndex {
    /// The ids among `course_ids` that the index does not list, in their order.
    pub fn course_ids_not_listed(&self, course_ids: impl IntoIterator<Item = String>) -> Vec<String> {
        course_ids.into_iter().filter(|course_id| !self.courses.contains_key(course_id)).collect()
    }
}

pub fn parse_course_index(bytes: &[u8]) -> Result<CourseIndex, AppError> {
    schema_version::require_schema_version(bytes, COURSE_INDEX_SCHEMA_VERSION_FIELD, COURSE_INDEX_SCHEMA_VERSION)?;

    let course_index: CourseIndex = serde_json::from_slice(bytes)?;

    for (course_id, entry) in &course_index.courses {
        validate_index_entry(course_id, entry)?;
    }

    Ok(course_index)
}

fn validate_index_entry(course_id: &str, entry: &CourseIndexEntry) -> Result<(), AppError> {
    if !course_identity::is_well_formed_course_id(course_id) {
        return Err(AppError::from(format!("course index lists a malformed course id; [course_id={course_id}]")));
    }

    let key: &str = &entry.latest_manifest_key;
    let manifest_suffix: String = format!("/{}", manifest::MANIFEST_FILENAME);
    let key_is_the_courses_own: bool = !key.contains("..")
        && key.starts_with(&repository_key::course_directory_key(course_id))
        && key.ends_with(&manifest_suffix);

    if !key_is_the_courses_own {
        return Err(AppError::from(format!(
            "course index names a latest manifest outside the course's directory; [course_id={course_id} latest_manifest_key={key}]",
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const COURSE_ID: &str = "pt_br.en.foundations";

    fn index_json_with(course_id: &str, latest_manifest_key: &str) -> String {
        format!(
            r#"{{
  "course_index_schema_version": {COURSE_INDEX_SCHEMA_VERSION},
  "courses": {{
    "{course_id}": {{
      "title": "Português do Brasil",
      "target_language": "pt-BR",
      "source_language": "en",
      "latest_manifest_key": "{latest_manifest_key}",
      "test_course": false
    }}
  }}
}}"#,
        )
    }

    fn valid_index_json() -> String {
        index_json_with(COURSE_ID, &repository_key::course_latest_manifest_key(COURSE_ID))
    }

    fn with_schema_version(json: &str, course_index_schema_version: u32) -> String {
        json.replace(
            &format!("\"{COURSE_INDEX_SCHEMA_VERSION_FIELD}\": {COURSE_INDEX_SCHEMA_VERSION}"),
            &format!("\"{COURSE_INDEX_SCHEMA_VERSION_FIELD}\": {course_index_schema_version}"),
        )
    }

    #[test]
    fn parse_course_index_round_trips() {
        let course_index: CourseIndex = parse_course_index(valid_index_json().as_bytes()).unwrap();

        assert_eq!(course_index.courses[COURSE_ID].title, "Português do Brasil");
        assert_eq!(course_index.courses[COURSE_ID].latest_manifest_key, "courses/pt_br.en.foundations/latest/manifest.json");

        let produced: String = serde_json::to_string_pretty(&course_index).unwrap();
        let reparsed: CourseIndex = parse_course_index(produced.as_bytes()).unwrap();

        assert_eq!(course_index, reparsed);
        assert!(produced.starts_with("{\n  \"course_index_schema_version\""));
    }

    #[test]
    fn course_ids_not_listed_keeps_the_ids_the_index_omits() {
        let course_index: CourseIndex = parse_course_index(valid_index_json().as_bytes()).unwrap();
        let remembered_course_ids: Vec<String> = vec!["pt_br.en.travel".to_string(), COURSE_ID.to_string(), "es.en.foundations".to_string()];

        let omitted_course_ids: Vec<String> = course_index.course_ids_not_listed(remembered_course_ids);

        assert_eq!(omitted_course_ids, vec!["pt_br.en.travel".to_string(), "es.en.foundations".to_string()]);
    }

    #[test]
    fn parse_course_index_rejects_an_older_schema_version() {
        let json: String = with_schema_version(&valid_index_json(), 0);

        let error: AppError = parse_course_index(json.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("course_index_schema_version 0 predates this build"));
    }

    #[test]
    fn parse_course_index_rejects_a_newer_schema_version() {
        let newer_version: u32 = COURSE_INDEX_SCHEMA_VERSION + 1;
        let json: String = with_schema_version(&valid_index_json(), newer_version);

        let error: AppError = parse_course_index(json.as_bytes()).unwrap_err();

        assert!(error.to_string().contains(&format!("course_index_schema_version {newer_version} comes from a newer build")));
    }

    #[test]
    fn parse_course_index_rejects_a_malformed_course_id() {
        let json: String = index_json_with("Foundations", "courses/Foundations/latest/manifest.json");

        let error: AppError = parse_course_index(json.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("malformed course id"));
    }

    #[test]
    fn parse_course_index_rejects_a_latest_manifest_key_outside_the_courses_directory() {
        let other_course_key: String = repository_key::course_latest_manifest_key("pt_br.en.test_course");
        let rejected_keys: [&str; 4] = [
            &other_course_key,
            "latest/manifest.json",
            "courses/pt_br.en.foundations/latest/recordings.json",
            "courses/pt_br.en.foundations/../pt_br.en.test_course/latest/manifest.json",
        ];

        for rejected_key in rejected_keys {
            let json: String = index_json_with(COURSE_ID, rejected_key);

            assert!(parse_course_index(json.as_bytes()).is_err());
        }
    }

    #[test]
    fn serialized_course_index_lists_courses_in_course_id_order() {
        let mut course_index: CourseIndex = parse_course_index(valid_index_json().as_bytes()).unwrap();
        let mut earlier_entry: CourseIndexEntry = course_index.courses[COURSE_ID].clone();

        earlier_entry.latest_manifest_key = repository_key::course_latest_manifest_key("es.en.foundations");
        course_index.courses.insert("es.en.foundations".to_string(), earlier_entry);

        let produced: String = serde_json::to_string(&course_index).unwrap();

        assert!(produced.find("es.en.foundations").unwrap() < produced.find(COURSE_ID).unwrap());
    }
}
