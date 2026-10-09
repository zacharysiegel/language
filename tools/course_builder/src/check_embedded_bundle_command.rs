use shared::AppError;

use crate::artifact::repository::CloudflareR2ArtifactRepository;
use crate::artifact::{self, EmbeddedBundleMismatchKind};
use crate::publish_command;
use crate::workspace::WorkspacePaths;

/// Reads only, so it takes no repository lock.
pub fn run_check_embedded_bundle_cloudflare_r2(paths: &WorkspacePaths) -> Result<bool, AppError> {
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let mismatches: Vec<EmbeddedBundleMismatchKind> = runtime.block_on(async {
        let r2_repository: CloudflareR2ArtifactRepository = publish_command::create_cloudflare_r2_repository().await?;

        artifact::check_embedded_bundle(paths, &r2_repository).await
    })?;

    for mismatch in &mismatches {
        eprintln!("{}", mismatch.describe());
    }

    if !mismatches.is_empty() {
        eprintln!("the embedded bundle does not match Cloudflare R2; `publish cloudflare-r2` refreshes it; [mismatches={}]", mismatches.len());
        return Ok(false);
    }

    log::info!("the embedded bundle matches Cloudflare R2");

    Ok(true)
}
