use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use shared::AppError;
use shared::course::{Curriculum, Unit};

use crate::workspace::WorkspacePaths;

/// The ids of every course directory that holds a curriculum, in name order.
pub fn list_course_ids(paths: &WorkspacePaths) -> Result<Vec<String>, AppError> {
    let mut course_ids: Vec<String> = Vec::new();

    for directory_entry in fs::read_dir(&paths.courses_directory)? {
        let course_id: String = directory_entry?.file_name().to_string_lossy().into_owned();

        if paths.curriculum_path(&course_id).exists() {
            course_ids.push(course_id);
        }
    }

    course_ids.sort();

    Ok(course_ids)
}

pub fn read_curriculum(paths: &WorkspacePaths, course_id: &str) -> Result<Curriculum, AppError> {
    let curriculum_path: PathBuf = paths.curriculum_path(course_id);
    let text: String = fs::read_to_string(&curriculum_path)
        .map_err(|error| AppError::from(format!("could not read a curriculum; [path={} error={error}]", curriculum_path.display())))?;

    parse_json(&text, &curriculum_path)
}

/// `None` when the unit has no file yet.
pub fn read_unit(paths: &WorkspacePaths, course_id: &str, unit_id: &str) -> Result<Option<Unit>, AppError> {
    let unit_path: PathBuf = paths.unit_path(course_id, unit_id);
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
