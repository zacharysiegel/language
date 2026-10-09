use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use shared::AppError;
use shared::artifact::content_address;

/// The recordings a publication's shards name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordingsList {
    pub recording_sha256s: BTreeSet<String>,
}

pub fn parse_recordings_list(bytes: &[u8]) -> Result<RecordingsList, AppError> {
    let recordings_list: RecordingsList = serde_json::from_slice(bytes)?;
    let malformed_sha256: Option<&String> = recordings_list.recording_sha256s.iter().find(|sha256| !content_address::is_sha256_hex(sha256));

    if let Some(malformed_sha256) = malformed_sha256 {
        return Err(AppError::from(format!("a recordings list names a malformed sha256; [sha256={malformed_sha256}]")));
    }

    Ok(recordings_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact::json_document;

    #[test]
    fn parse_recordings_list_round_trips_in_sorted_order() {
        let recordings_list: RecordingsList = RecordingsList {
            recording_sha256s: BTreeSet::from(["b".repeat(64), "a".repeat(64)]),
        };

        let bytes: Vec<u8> = json_document::json_document_bytes(&recordings_list).unwrap();
        let text: String = String::from_utf8(bytes.clone()).unwrap();

        assert_eq!(parse_recordings_list(&bytes).unwrap(), recordings_list);
        assert!(text.find(&"a".repeat(64)).unwrap() < text.find(&"b".repeat(64)).unwrap());
    }

    #[test]
    fn parse_recordings_list_rejects_a_malformed_sha256() {
        assert!(parse_recordings_list(br#"{ "recording_sha256s": ["not-a-hash"] }"#).is_err());
    }
}
