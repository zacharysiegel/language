use std::fs;
use std::path::{Path, PathBuf};

use shared::AppError;

const WORKSPACE_MANIFEST_TABLE: &str = "[workspace]";
const CURRICULUM_PATH: &str = "data/curriculum.json";
const UNITS_DIRECTORY: &str = "data/units";
const COMPILED_COURSE_DIRECTORY: &str = "web/static/course";

pub struct WorkspacePaths {
    pub curriculum_path: PathBuf,
    pub units_directory: PathBuf,
    pub compiled_course_directory: PathBuf,
}

impl WorkspacePaths {
    pub fn locate() -> Result<WorkspacePaths, AppError> {
        let current_directory: PathBuf = std::env::current_dir()?;
        let root: PathBuf = find_workspace_root(&current_directory)?;

        Ok(WorkspacePaths {
            curriculum_path: root.join(CURRICULUM_PATH),
            units_directory: root.join(UNITS_DIRECTORY),
            compiled_course_directory: root.join(COMPILED_COURSE_DIRECTORY),
        })
    }

    pub fn unit_path(&self, unit_id: &str) -> PathBuf {
        self.units_directory.join(format!("{unit_id}.json"))
    }

    pub fn compiled_course_path(&self) -> PathBuf {
        self.compiled_course_directory.join("course.json")
    }

    pub fn audio_directory(&self) -> PathBuf {
        self.compiled_course_directory.join("audio")
    }
}

fn find_workspace_root(start_directory: &Path) -> Result<PathBuf, AppError> {
    let mut directory: Option<&Path> = Some(start_directory);

    while let Some(candidate) = directory {
        // A directory with no readable manifest is the ordinary case while searching upward.
        let manifest_text: Option<String> = fs::read_to_string(candidate.join("Cargo.toml")).ok();
        let is_workspace_root: bool = manifest_text.is_some_and(|text| text.contains(WORKSPACE_MANIFEST_TABLE));

        if is_workspace_root {
            return Ok(candidate.to_path_buf());
        }

        directory = candidate.parent();
    }

    Err(AppError::from(format!(
        "no Cargo.toml declaring a workspace at or above the starting directory; [start={}]",
        start_directory.display(),
    )))
}
