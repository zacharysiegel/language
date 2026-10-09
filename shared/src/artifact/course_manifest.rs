use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::artifact::manifest::{self, ManifestEntry};
use crate::artifact::{repository_key, schema_version};
use crate::course::course_identity;

pub const COURSE_MANIFEST_SCHEMA_VERSION: u32 = 3;

/// One publication of one course.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseManifest {
    pub manifest_schema_version: u32,
    pub course_id: String,
    /// Unique within the course.
    pub publication_label: String,
    pub artifact_created: DateTime<Utc>,
    pub title: String,
    pub target_language: String,
    pub source_language: String,
    /// A course that exists to test the apps, not to teach.
    pub test_course: bool,
    pub outline: ManifestEntry,
    /// By unit id.
    pub units: BTreeMap<String, ManifestEntry>,
    /// By speech engine code.
    pub voice_pipeline_revisions: BTreeMap<String, u32>,
}

impl CourseManifest {
    pub fn file_entries(&self) -> impl Iterator<Item = &ManifestEntry> {
        std::iter::once(&self.outline).chain(self.units.values())
    }

    pub fn find_unit(&self, unit_id: &str) -> Result<&ManifestEntry, AppError> {
        self.units.get(unit_id).ok_or_else(|| {
            AppError::from(format!(
                "the published course has no such unit; [course_id={} unit_id={unit_id}]",
                self.course_id,
            ))
        })
    }

    /// The pointer a consumer should try when it cannot read the one at `latest_manifest_key`, or `None` when
    /// the document there is not a course manifest from a newer schema version.
    pub fn schema_fallback_key(latest_manifest_key: &str, latest_manifest_bytes: &[u8]) -> Option<String> {
        let found: u64 =
            schema_version::read_schema_version(latest_manifest_bytes, manifest::MANIFEST_SCHEMA_VERSION_FIELD).ok()?;

        if found <= u64::from(COURSE_MANIFEST_SCHEMA_VERSION) {
            return None;
        }

        Some(repository_key::course_schema_pointer_key(latest_manifest_key, COURSE_MANIFEST_SCHEMA_VERSION))
    }
}

/// `expected_course_id` is the course whose pointer the bytes came from, when there is one.
pub fn parse_course_manifest(bytes: &[u8], expected_course_id: Option<&str>) -> Result<CourseManifest, AppError> {
    schema_version::require_schema_version(bytes, manifest::MANIFEST_SCHEMA_VERSION_FIELD, COURSE_MANIFEST_SCHEMA_VERSION)?;

    let course_manifest: CourseManifest = serde_json::from_slice(bytes)?;

    validate_course_identity(&course_manifest, expected_course_id)?;
    validate_publication_label(&course_manifest)?;

    for entry in course_manifest.file_entries() {
        manifest::validate_entry(entry)?;
    }

    Ok(course_manifest)
}

fn validate_course_identity(course_manifest: &CourseManifest, expected_course_id: Option<&str>) -> Result<(), AppError> {
    let course_id: &str = &course_manifest.course_id;

    if !course_identity::is_well_formed_course_id(course_id) {
        return Err(AppError::from(format!("course manifest has a malformed course id; [course_id={course_id}]")));
    }

    match expected_course_id {
        Some(expected_course_id) if expected_course_id != course_id => Err(AppError::from(format!(
            "course manifest is for a different course than its pointer; [expected_course_id={expected_course_id} course_id={course_id}]",
        ))),
        _ => Ok(()),
    }
}

fn validate_publication_label(course_manifest: &CourseManifest) -> Result<(), AppError> {
    let publication_label: &str = &course_manifest.publication_label;
    let label_is_one_path_segment: bool =
        !publication_label.is_empty() && !publication_label.contains('/') && !publication_label.contains("..");

    if !label_is_one_path_segment {
        return Err(AppError::from(format!(
            "course manifest has an unsafe publication label; [course_id={} publication_label={publication_label}]",
            course_manifest.course_id,
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const COURSE_ID: &str = "pt_br.en.foundations";

    fn valid_sha256() -> String {
        "ab12cd34".repeat(8)
    }

    fn course_manifest_json_with(course_id: &str, publication_label: &str) -> String {
        format!(
            r#"{{
  "manifest_schema_version": {COURSE_MANIFEST_SCHEMA_VERSION},
  "course_id": "{course_id}",
  "publication_label": "{publication_label}",
  "artifact_created": "2026-10-08T14:02:11Z",
  "title": "Português do Brasil",
  "target_language": "pt-BR",
  "source_language": "en",
  "test_course": false,
  "outline": {{ "relative_path": "data/course-{sha}.sqlite.br", "size_bytes": 1000, "sha256": "{sha}" }},
  "units": {{ "a1-01": {{ "relative_path": "data/unit-a1-01-{sha}.sqlite.br", "size_bytes": 2000, "sha256": "{sha}" }} }},
  "voice_pipeline_revisions": {{ "kokoro": 3 }}
}}"#,
            sha = valid_sha256(),
        )
    }

    fn valid_course_manifest_json() -> String {
        course_manifest_json_with(COURSE_ID, "2026-10-08+duluth")
    }

    fn with_schema_version(json: &str, manifest_schema_version: u32) -> String {
        json.replace(
            &format!("\"manifest_schema_version\": {COURSE_MANIFEST_SCHEMA_VERSION}"),
            &format!("\"manifest_schema_version\": {manifest_schema_version}"),
        )
    }

    #[test]
    fn parse_course_manifest_round_trips() {
        let course_manifest: CourseManifest =
            parse_course_manifest(valid_course_manifest_json().as_bytes(), Some(COURSE_ID)).unwrap();

        assert_eq!(course_manifest.publication_label, "2026-10-08+duluth");
        assert_eq!(course_manifest.find_unit("a1-01").unwrap().size_bytes, 2000);
        assert_eq!(course_manifest.file_entries().count(), 2);
        assert_eq!(course_manifest.voice_pipeline_revisions["kokoro"], 3);

        let produced: String = serde_json::to_string_pretty(&course_manifest).unwrap();
        let reparsed: CourseManifest = parse_course_manifest(produced.as_bytes(), None).unwrap();

        assert_eq!(course_manifest, reparsed);
        assert!(produced.starts_with("{\n  \"manifest_schema_version\""));
    }

    #[test]
    fn parse_course_manifest_rejects_a_manifest_for_another_course_than_its_pointer() {
        let error: AppError =
            parse_course_manifest(valid_course_manifest_json().as_bytes(), Some("pt_br.en.test_course")).unwrap_err();

        assert!(error.to_string().contains("different course than its pointer"));
    }

    #[test]
    fn parse_course_manifest_rejects_a_malformed_course_id() {
        let json: String = course_manifest_json_with("Foundations", "2026-10-08+duluth");

        assert!(parse_course_manifest(json.as_bytes(), None).is_err());
    }

    #[test]
    fn parse_course_manifest_rejects_a_malformed_publication_label() {
        for rejected_label in ["2026-10-08+duluth/extra", "..", "2026-10-08+..", ""] {
            let json: String = course_manifest_json_with(COURSE_ID, rejected_label);

            assert!(parse_course_manifest(json.as_bytes(), Some(COURSE_ID)).is_err());
        }
    }

    #[test]
    fn parse_course_manifest_rejects_a_schema_version_this_build_does_not_read() {
        let older: String = with_schema_version(&valid_course_manifest_json(), COURSE_MANIFEST_SCHEMA_VERSION - 1);
        let newer: String = with_schema_version(&valid_course_manifest_json(), COURSE_MANIFEST_SCHEMA_VERSION + 1);

        assert!(parse_course_manifest(older.as_bytes(), Some(COURSE_ID)).is_err());
        assert!(parse_course_manifest(newer.as_bytes(), Some(COURSE_ID)).is_err());
    }

    #[test]
    fn parse_course_manifest_rejects_path_traversal_relative_path() {
        let json: String = valid_course_manifest_json().replace("data/course", "../course");

        assert!(parse_course_manifest(json.as_bytes(), Some(COURSE_ID)).is_err());
    }

    #[test]
    fn parse_course_manifest_rejects_a_malformed_sha256() {
        let json: String = valid_course_manifest_json().replace(&format!("\"sha256\": \"{}\"", valid_sha256()), "\"sha256\": \"not-hex\"");

        assert!(parse_course_manifest(json.as_bytes(), Some(COURSE_ID)).is_err());
    }

    #[test]
    fn parse_course_manifest_ignores_unknown_fields() {
        let json: String = valid_course_manifest_json().replace(
            r#""test_course": false,"#,
            r#""test_course": false,
  "field_added_in_a_later_revision": "ignored","#,
        );

        let course_manifest: CourseManifest = parse_course_manifest(json.as_bytes(), Some(COURSE_ID)).unwrap();

        assert_eq!(course_manifest.publication_label, "2026-10-08+duluth");
    }

    #[test]
    fn schema_fallback_key_offers_the_readers_own_pointer_beside_the_named_pointer() {
        let latest_manifest_key: String = repository_key::course_latest_manifest_key(COURSE_ID);
        let newer: String = with_schema_version(&valid_course_manifest_json(), COURSE_MANIFEST_SCHEMA_VERSION + 1);

        assert_eq!(
            CourseManifest::schema_fallback_key(&latest_manifest_key, newer.as_bytes()),
            Some(format!("courses/pt_br.en.foundations/latest/manifest.{COURSE_MANIFEST_SCHEMA_VERSION}.json")),
        );
    }

    #[test]
    fn schema_fallback_key_declines_every_case_but_a_newer_document() {
        let latest_manifest_key: String = repository_key::course_latest_manifest_key(COURSE_ID);

        assert_eq!(CourseManifest::schema_fallback_key(&latest_manifest_key, valid_course_manifest_json().as_bytes()), None);
        assert_eq!(CourseManifest::schema_fallback_key(&latest_manifest_key, b"<html>a proxy error page</html>"), None);
    }
}
