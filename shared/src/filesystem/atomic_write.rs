use std::fs;
use std::path::{Path, PathBuf};

use crate::AppError;

/// Writes `bytes` to `path` through a sibling temporary file, so a reader never sees a partial file.
pub fn write_atomically(path: &Path, bytes: &[u8]) -> Result<(), AppError> {
    let partial_path: PathBuf = path.with_extension("partial");

    fs::write(&partial_path, bytes)
        .map_err(|error| AppError::from(format!("writing a file failed; [path={} error={error}]", partial_path.display())))?;
    fs::rename(&partial_path, path)
        .map_err(|error| AppError::from(format!("renaming a written file failed; [path={} error={error}]", path.display())))?;

    Ok(())
}
