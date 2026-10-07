use std::fs;
use std::path::{Path, PathBuf};

use crate::AppError;

const WORKSPACE_MANIFEST_TABLE: &str = "[workspace]";

/// The nearest directory at or above `start_directory` whose `Cargo.toml` declares a workspace.
pub fn find_workspace_root(start_directory: &Path) -> Result<PathBuf, AppError> {
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

/// `path` unchanged when absolute, else joined onto the workspace root found from the current directory.
pub fn resolve_workspace_relative(path: &Path) -> Result<PathBuf, AppError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let current_directory: PathBuf = std::env::current_dir()?;
    let workspace_root: PathBuf = find_workspace_root(&current_directory)?;

    Ok(workspace_root.join(path))
}
