use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::artifact::content_address;

pub const MANIFEST_FILENAME: &str = "manifest.json";
pub const MANIFEST_SCHEMA_VERSION_FIELD: &str = "manifest_schema_version";
pub const SUBDIR_DATA: &str = "data";
pub const SUBDIR_AUDIO: &str = "audio";
pub const AUDIO_FILE_EXTENSION: &str = "m4a";
/* Brotli-compressed SQLite. */
pub const SHARD_FILE_EXTENSION: &str = "sqlite.br";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub relative_path: String,
    pub size_bytes: u64,
    pub sha256: String,
}

pub fn validate_entry(entry: &ManifestEntry) -> Result<(), AppError> {
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
