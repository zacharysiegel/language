use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use shared::AppError;
use shared::course::{Curriculum, Unit};

use crate::workspace::WorkspacePaths;

pub fn read_curriculum(paths: &WorkspacePaths) -> Result<Curriculum, AppError> {
    let text: String = fs::read_to_string(&paths.curriculum_path)?;

    parse_json(&text, &paths.curriculum_path)
}

/// `None` when the unit has no file yet.
pub fn read_unit(paths: &WorkspacePaths, unit_id: &str) -> Result<Option<Unit>, AppError> {
    let unit_path: PathBuf = paths.unit_path(unit_id);
    let read_result: Result<String, std::io::Error> = fs::read_to_string(&unit_path);

    match read_result {
        Ok(text) => parse_json(&text, &unit_path).map(Some),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(AppError::from(error)),
    }
}

fn parse_json<T: DeserializeOwned>(text: &str, path: &Path) -> Result<T, AppError> {
    let mut deserializer: serde_json::Deserializer<serde_json::de::StrRead> = serde_json::Deserializer::from_str(text);
    let parsed: Result<T, serde_path_to_error::Error<serde_json::Error>> = serde_path_to_error::deserialize(&mut deserializer);

    parsed.map_err(|error| {
        AppError::from(format!(
            "could not parse; [path={} field={} error={}]",
            path.display(),
            error.path(),
            error.inner(),
        ))
    })
}
