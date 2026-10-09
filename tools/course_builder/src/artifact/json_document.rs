use std::path::Path;

use serde::Serialize;
use shared::AppError;
use shared::filesystem;

/// Pretty-printed with a trailing newline, so equal values give equal bytes.
pub fn json_document_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, AppError> {
    let text: String = serde_json::to_string_pretty(value)?;

    Ok(format!("{text}\n").into_bytes())
}

pub fn write_json_document<T: Serialize>(path: &Path, value: &T) -> Result<(), AppError> {
    let bytes: Vec<u8> = json_document_bytes(value)?;

    filesystem::write_atomically(path, &bytes)
}
