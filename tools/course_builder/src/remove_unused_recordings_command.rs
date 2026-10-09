use shared::AppError;

use crate::artifact;
use crate::artifact::repository::{self, ArtifactRepositoryKind, RepositoryLockKind};
use crate::publish_command::{self, RepositoryLocationKind};
use crate::workspace::WorkspacePaths;

pub fn run_remove_unused_recordings(paths: &WorkspacePaths, location: RepositoryLocationKind) -> Result<bool, AppError> {
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let removed_count: usize = runtime.block_on(async {
        let repository: ArtifactRepositoryKind = publish_command::create_repository(paths, location).await?;
        let lock: RepositoryLockKind = repository::acquire_repository_lock(&repository).await?;

        let removed: Result<usize, AppError> = artifact::remove_unused_recordings(&repository).await;
        let released: Result<(), AppError> = lock.release(&repository).await;

        repository::combine_with_release(removed, released)
    })?;

    log::info!("removed the recordings no publication names; [count={removed_count}]");

    Ok(true)
}
