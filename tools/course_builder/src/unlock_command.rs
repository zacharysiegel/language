use shared::AppError;

use crate::artifact::repository::{self, CloudflareR2ArtifactRepository, LeaseRemovalKind};
use crate::publish_command;

pub fn run_unlock_cloudflare_r2() -> Result<bool, AppError> {
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let removal: LeaseRemovalKind = runtime.block_on(async {
        let r2_repository: CloudflareR2ArtifactRepository = publish_command::create_cloudflare_r2_repository().await?;

        repository::remove_repository_lease(&r2_repository).await
    })?;

    match removal {
        LeaseRemovalKind::Removed { holder_description } => log::info!("removed the repository lease; [{holder_description}]"),
        LeaseRemovalKind::NotHeld => log::info!("no command holds the repository lease"),
    }

    Ok(true)
}
