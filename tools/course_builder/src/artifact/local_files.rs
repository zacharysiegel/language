use std::fs;
use std::io;
use std::path::Path;

use shared::AppError;

pub fn remove_directory_if_present(directory: &Path) -> Result<(), AppError> {
    let removed: Result<(), io::Error> = fs::remove_dir_all(directory);

    match removed {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(AppError::from(format!("removing a directory failed; [path={} error={error}]", directory.display()))),
    }
}

pub fn copy_creating_parent(source: &Path, destination: &Path) -> Result<(), AppError> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source, destination).map_err(|error| {
        AppError::from(format!(
            "copying a file failed; [source={} destination={} error={error}]",
            source.display(),
            destination.display(),
        ))
    })?;

    Ok(())
}
