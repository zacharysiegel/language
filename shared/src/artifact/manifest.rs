use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::artifact::{content_address, schema_version};

pub const MANIFEST_FILENAME: &str = "manifest.json";
pub const MANIFEST_SCHEMA_VERSION: u32 = 2;
pub const MANIFEST_SCHEMA_VERSION_FIELD: &str = "manifest_schema_version";
pub const SUBDIR_DATA: &str = "data";
pub const SUBDIR_AUDIO: &str = "audio";
pub const AUDIO_FILE_EXTENSION: &str = "m4a";
/* Brotli-compressed SQLite. */
pub const SHARD_FILE_EXTENSION: &str = "sqlite.br";

/* The producer uploads a byte-for-byte copy of the just-published manifest here; a consumer starts from it. */
pub const MANIFEST_LATEST_KEY: &str = "latest/manifest.json";

/* The repository's top-level directory that holds the pointers, which is not a version. */
pub const LATEST_POINTER_DIRECTORY: &str = "latest";

/// Stable-pointer key for one manifest schema version. A publish refreshes only the key for the schema it writes.
pub fn schema_pointer_key(manifest_schema_version: u32) -> String {
    format!("{LATEST_POINTER_DIRECTORY}/manifest.{manifest_schema_version}.json")
}

pub fn versioned_manifest_key(version: &str) -> String {
    format!("{version}/{MANIFEST_FILENAME}")
}

/* Manifest paths are relative to the version's directory; audio is shared by every version, so it sits outside them. */
pub fn versioned_file_key(version: &str, relative_path: &str) -> String {
    format!("{version}/{relative_path}")
}

pub fn audio_key(audio_sha256: &str) -> String {
    format!("{SUBDIR_AUDIO}/{audio_sha256}.{AUDIO_FILE_EXTENSION}")
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub manifest_schema_version: u32,
    pub version: String,
    pub artifact_created: DateTime<Utc>,
    /// Every published course, by course id.
    pub courses: BTreeMap<String, ManifestCourse>,
    pub voice_pipeline_revisions: BTreeMap<String, u32>,
}

impl Manifest {
    pub fn file_entries(&self) -> impl Iterator<Item = &ManifestEntry> {
        self.courses.values().flat_map(ManifestCourse::file_entries)
    }

    pub fn find_course(&self, course_id: &str) -> Result<&ManifestCourse, AppError> {
        self.courses
            .get(course_id)
            .ok_or_else(|| AppError::from(format!("the published version has no such course; [course_id={course_id}]")))
    }

    pub fn test_course_ids(&self) -> Vec<&str> {
        self.courses
            .iter()
            .filter(|(_, manifest_course)| manifest_course.test_course)
            .map(|(course_id, _)| course_id.as_str())
            .collect()
    }
}

/// One course's shards, and what a course list shows before any shard is opened.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestCourse {
    pub title: String,
    pub target_language: String,
    pub source_language: String,
    pub outline: ManifestEntry,
    /// The course's unit shards, by unit id.
    pub units: BTreeMap<String, ManifestEntry>,
    /// A course that exists to test the apps, not to teach.
    #[serde(default)]
    pub test_course: bool,
}

impl ManifestCourse {
    pub fn file_entries(&self) -> impl Iterator<Item = &ManifestEntry> {
        std::iter::once(&self.outline).chain(self.units.values())
    }

    pub fn find_unit(&self, unit_id: &str) -> Result<&ManifestEntry, AppError> {
        self.units
            .get(unit_id)
            .ok_or_else(|| AppError::from(format!("the published course has no such unit; [unit_id={unit_id}]")))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub relative_path: String,
    pub size_bytes: u64,
    pub sha256: String,
}

pub fn parse_manifest(bytes: &[u8]) -> Result<Manifest, AppError> {
    schema_version::require_schema_version(bytes, MANIFEST_SCHEMA_VERSION_FIELD, MANIFEST_SCHEMA_VERSION)?;

    let manifest: Manifest = serde_json::from_slice(bytes)?;

    for entry in manifest.file_entries() {
        validate_entry(entry)?;
    }

    Ok(manifest)
}

/// The pointer key a consumer should try when it cannot read `MANIFEST_LATEST_KEY`, or `None` when the
/// document is not a manifest from a newer schema version.
pub fn schema_fallback_key(latest_manifest_bytes: &[u8]) -> Option<String> {
    let found: u64 = schema_version::read_schema_version(latest_manifest_bytes, MANIFEST_SCHEMA_VERSION_FIELD).ok()?;

    if found <= u64::from(MANIFEST_SCHEMA_VERSION) {
        return None;
    }

    Some(schema_pointer_key(MANIFEST_SCHEMA_VERSION))
}

fn validate_entry(entry: &ManifestEntry) -> Result<(), AppError> {
    if !content_address::is_sha256_hex(&entry.sha256) {
        return Err(AppError::from(format!(
            "manifest entry has a malformed sha256; [relative_path={} sha256={}]",
            entry.relative_path, entry.sha256,
        )));
    }

    if entry.relative_path.contains("..") || entry.relative_path.starts_with('/') {
        return Err(AppError::from(format!("manifest entry has an unsafe relative_path; [relative_path={}]", entry.relative_path)));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_sha256() -> String {
        "ab12cd34".repeat(8)
    }

    fn valid_manifest_json() -> String {
        format!(
            r#"{{
  "manifest_schema_version": {schema_version},
  "version": "20261007T120000Z-1a2b3c4d",
  "artifact_created": "2026-10-07T12:00:00Z",
  "courses": {{
    "pt_br.en.foundations": {{
      "title": "Português do Brasil",
      "target_language": "pt-BR",
      "source_language": "en",
      "outline": {{ "relative_path": "data/pt_br.en.foundations/course-{sha}.sqlite.br", "size_bytes": 1000, "sha256": "{sha}" }},
      "units": {{ "a1-01": {{ "relative_path": "data/pt_br.en.foundations/unit-a1-01-{sha}.sqlite.br", "size_bytes": 2000, "sha256": "{sha}" }} }}
    }}
  }},
  "voice_pipeline_revisions": {{ "kokoro": 3, "macos_say": 1 }}
}}"#,
            schema_version = MANIFEST_SCHEMA_VERSION,
            sha = valid_sha256(),
        )
    }

    #[test]
    fn keys_render_the_published_layout() {
        assert_eq!(schema_pointer_key(2), "latest/manifest.2.json");
        assert_eq!(versioned_manifest_key("v1"), "v1/manifest.json");
        assert_eq!(versioned_file_key("v1", "data/a.sqlite"), "v1/data/a.sqlite");
        assert_eq!(audio_key("abc"), "audio/abc.m4a");
    }

    #[test]
    fn parse_manifest_round_trips() {
        let manifest: Manifest = parse_manifest(valid_manifest_json().as_bytes()).unwrap();

        assert_eq!(manifest.version, "20261007T120000Z-1a2b3c4d");
        assert_eq!(manifest.courses["pt_br.en.foundations"].units["a1-01"].size_bytes, 2000);
        assert_eq!(manifest.file_entries().count(), 2);
        assert_eq!(manifest.voice_pipeline_revisions["kokoro"], 3);

        let produced: String = serde_json::to_string_pretty(&manifest).unwrap();
        let reparsed: Manifest = parse_manifest(produced.as_bytes()).unwrap();

        assert_eq!(manifest, reparsed);
    }

    #[test]
    fn test_course_ids_names_only_the_courses_marked_as_test_courses() {
        let mut manifest: Manifest = parse_manifest(valid_manifest_json().as_bytes()).unwrap();

        assert!(manifest.test_course_ids().is_empty());

        manifest.courses.get_mut("pt_br.en.foundations").unwrap().test_course = true;

        assert_eq!(manifest.test_course_ids(), vec!["pt_br.en.foundations"]);
    }

    #[test]
    fn serialized_manifest_starts_with_the_schema_version_field() {
        let manifest: Manifest = parse_manifest(valid_manifest_json().as_bytes()).unwrap();

        let produced: String = serde_json::to_string_pretty(&manifest).unwrap();

        assert!(produced.starts_with("{\n  \"manifest_schema_version\""));
    }

    #[test]
    fn schema_fallback_key_offers_the_readers_own_pointer_for_a_newer_document() {
        let json: String = valid_manifest_json().replace(
            &format!("\"manifest_schema_version\": {MANIFEST_SCHEMA_VERSION}"),
            &format!("\"manifest_schema_version\": {}", MANIFEST_SCHEMA_VERSION + 1),
        );

        assert_eq!(schema_fallback_key(json.as_bytes()), Some(schema_pointer_key(MANIFEST_SCHEMA_VERSION)));
    }

    #[test]
    fn schema_fallback_key_declines_every_case_but_a_newer_document() {
        assert_eq!(schema_fallback_key(valid_manifest_json().as_bytes()), None);
        assert_eq!(schema_fallback_key(b"<html>a proxy error page</html>"), None);
    }

    #[test]
    fn parse_manifest_rejects_a_schema_version_this_build_does_not_read() {
        let newer_version: u32 = MANIFEST_SCHEMA_VERSION + 1;
        let json: String = valid_manifest_json().replace(
            &format!("\"manifest_schema_version\": {MANIFEST_SCHEMA_VERSION}"),
            &format!("\"manifest_schema_version\": {newer_version}"),
        );

        let error: AppError = parse_manifest(json.as_bytes()).unwrap_err();

        assert!(error.to_string().contains(&format!("manifest_schema_version {newer_version} comes from a newer build")));
    }

    #[test]
    fn parse_manifest_rejects_malformed_sha256() {
        let json: String = valid_manifest_json().replace(&format!("\"sha256\": \"{}\"", valid_sha256()), "\"sha256\": \"not-hex\"");

        assert!(parse_manifest(json.as_bytes()).is_err());
    }

    #[test]
    fn parse_manifest_rejects_path_traversal_relative_path() {
        let json: String = valid_manifest_json().replace("data/pt_br.en.foundations/course", "../course");

        assert!(parse_manifest(json.as_bytes()).is_err());
    }

    #[test]
    fn parse_manifest_ignores_unknown_fields() {
        let json: String = valid_manifest_json().replace(
            r#""version": "20261007T120000Z-1a2b3c4d","#,
            r#""version": "20261007T120000Z-1a2b3c4d",
  "field_added_in_a_later_revision": "ignored","#,
        );

        let manifest: Manifest = parse_manifest(json.as_bytes()).unwrap();

        assert_eq!(manifest.version, "20261007T120000Z-1a2b3c4d");
    }
}
