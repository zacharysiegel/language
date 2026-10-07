use std::path::{Path, PathBuf};

use shared::AppError;

use crate::artifact::repository::{self, CloudflareR2ArtifactRepository, CorsPolicy};
use crate::publish_command;

const CORS_POLICY_FILE_NAME: &str = "r2-cors.json";

pub enum CorsActionKind {
    Apply,
    List,
}

/// Applies the committed CORS policy to the R2 bucket, then reports the bucket's policy.
pub fn run_set_artifact_cors(action: CorsActionKind) -> Result<bool, AppError> {
    let policy_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(CORS_POLICY_FILE_NAME);
    let policy: CorsPolicy = repository::read_cors_policy(&policy_path)?;
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let applied_policies: Vec<CorsPolicy> = runtime.block_on(async {
        let r2_repository: CloudflareR2ArtifactRepository = publish_command::create_cloudflare_r2_repository().await?;

        if let CorsActionKind::Apply = action {
            r2_repository.apply_cors_policy(&policy).await?;
            log::info!("applied the CORS policy; [path={}]", policy_path.display());
        }

        r2_repository.read_cors_policies().await
    })?;

    for applied_policy in &applied_policies {
        log::info!(
            "bucket CORS rule; [origins={} methods={} max_age_seconds={}]",
            applied_policy.allowed_origins.join(","),
            applied_policy.allowed_methods.join(","),
            applied_policy.max_age_seconds,
        );
    }

    // Cloudflare documents up to 30 seconds of propagation, and a purge for a custom domain already serving traffic.
    log::info!("a CORS change can take 30 seconds to take effect, and a custom domain's cache may need a purge");

    Ok(true)
}
