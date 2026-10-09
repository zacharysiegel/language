use chrono::{DateTime, Utc};

const PUBLICATION_LABEL_FIELD: &str = "publication_label";
const ARTIFACT_CREATED_FIELD: &str = "artifact_created";

/// The publication a course pointer names, read without checking the manifest schema, so a pointer of a newer
/// manifest schema is still read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointerPublication {
    pub publication_label: String,
    /// When the publication's course content was built; `None` when the pointer carries no readable time.
    pub artifact_created: Option<DateTime<Utc>>,
}

/// `None` when the bytes are not a JSON object with a string `publication_label`.
pub fn read_pointer_publication(manifest_bytes: &[u8]) -> Option<PointerPublication> {
    let document: serde_json::Value = serde_json::from_slice(manifest_bytes).ok()?;
    let publication_label: String = document.get(PUBLICATION_LABEL_FIELD)?.as_str()?.to_string();
    let artifact_created: Option<DateTime<Utc>> = document
        .get(ARTIFACT_CREATED_FIELD)
        .and_then(serde_json::Value::as_str)
        .and_then(|text| DateTime::parse_from_rfc3339(text).ok())
        .map(|created| created.with_timezone(&Utc));

    Some(PointerPublication {
        publication_label,
        artifact_created,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_pointer_publication_reads_a_newer_manifest_schema() {
        let pointer_bytes: &[u8] =
            br#"{"manifest_schema_version": 99, "publication_label": "2026-10-09+albany", "artifact_created": "2026-10-09T12:00:00Z", "renamed": {}}"#;

        let pointer_publication: Option<PointerPublication> = read_pointer_publication(pointer_bytes);

        assert_eq!(
            pointer_publication,
            Some(PointerPublication {
                publication_label: "2026-10-09+albany".to_string(),
                artifact_created: Some(DateTime::parse_from_rfc3339("2026-10-09T12:00:00Z").unwrap().with_timezone(&Utc)),
            }),
        );
    }

    #[test]
    fn read_pointer_publication_needs_a_publication_label_but_not_a_build_time() {
        let without_time: Option<PointerPublication> = read_pointer_publication(br#"{"publication_label": "2026-10-09+albany"}"#);

        assert_eq!(without_time.map(|pointer_publication| pointer_publication.artifact_created), Some(None));
        assert_eq!(read_pointer_publication(br#"{"artifact_created": "2026-10-09T12:00:00Z"}"#), None);
        assert_eq!(read_pointer_publication(b"not json"), None);
    }
}
