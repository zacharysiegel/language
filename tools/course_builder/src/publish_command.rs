use std::path::PathBuf;

use shared::AppError;
use shared::artifact::discovery;

use crate::artifact::repository::{
    ArtifactRepositoryKind, CloudflareR2ArtifactRepository, CloudflareR2Config, DryArtifactRepository, ENV_LOCAL_REPOSITORY_ROOT,
    ENV_R2_ACCOUNT_ID, ENV_R2_ARTIFACT_BUCKET, ENV_R2_ARTIFACT_PUBLIC_BASE_URL, ENV_R2_PUBLISH_ACCESS_KEY_ID,
    LocalArtifactRepository, SECRET_R2_PUBLISH_SECRET_ACCESS_KEY,
};
use crate::artifact::{self, PublishReport};
use crate::secrets;
use crate::workspace::WorkspacePaths;

pub enum PublishTargetKind {
    Local {
        root: Option<PathBuf>,
        public_base_url: Option<String>,
    },
    CloudflareR2,
    Dry,
}

pub fn run_publish(paths: &WorkspacePaths, target: PublishTargetKind, bundle: Option<PathBuf>) -> Result<bool, AppError> {
    let bundle_directory: PathBuf = match bundle {
        Some(bundle_directory) => bundle_directory,
        None => artifact::find_newest_bundle(paths)?,
    };
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let report: PublishReport = runtime.block_on(async {
        let repository: ArtifactRepositoryKind = create_repository(paths, target).await?;

        artifact::publish_bundle(&bundle_directory, &repository).await
    })?;

    log::info!(
        "publish complete; [version={} latest={} shards={} recordings_uploaded={} recordings_already_present={}]",
        report.version,
        report.latest_manifest_url,
        report.shards_published,
        report.audio_published,
        report.audio_already_present,
    );

    Ok(true)
}

async fn create_repository(paths: &WorkspacePaths, target: PublishTargetKind) -> Result<ArtifactRepositoryKind, AppError> {
    match target {
        PublishTargetKind::Local { root, public_base_url } => {
            let root: PathBuf = root
                .or_else(|| dotenvy::var(ENV_LOCAL_REPOSITORY_ROOT).ok().map(PathBuf::from))
                .unwrap_or_else(|| paths.local_repository_directory.clone());
            let public_base_url: String = public_base_url.unwrap_or_else(|| discovery::STATIC_REPOSITORY_BASE_URL.to_string());

            Ok(ArtifactRepositoryKind::Local(LocalArtifactRepository::new(root, public_base_url)))
        },
        PublishTargetKind::CloudflareR2 => {
            let config: CloudflareR2Config = CloudflareR2Config {
                account_id: secrets::read_environment(ENV_R2_ACCOUNT_ID)?,
                bucket: secrets::read_environment(ENV_R2_ARTIFACT_BUCKET)?,
                access_key_id: secrets::read_environment(ENV_R2_PUBLISH_ACCESS_KEY_ID)?,
                secret_access_key: secrets::master_decrypt_utf8(SECRET_R2_PUBLISH_SECRET_ACCESS_KEY)?,
                public_base_url: secrets::read_environment(ENV_R2_ARTIFACT_PUBLIC_BASE_URL)?,
            };

            Ok(ArtifactRepositoryKind::CloudflareR2(CloudflareR2ArtifactRepository::create(config).await))
        },
        PublishTargetKind::Dry => Ok(ArtifactRepositoryKind::Dry(DryArtifactRepository::new())),
    }
}
