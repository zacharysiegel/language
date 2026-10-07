use std::collections::BTreeSet;
use std::future::Future;
use std::path::Path;

use shared::AppError;

use crate::artifact::repository::cloudflare_r2_artifact_repository::CloudflareR2ArtifactRepository;
use crate::artifact::repository::dry_artifact_repository::DryArtifactRepository;
use crate::artifact::repository::local_artifact_repository::LocalArtifactRepository;

pub const CONTENT_TYPE_SHARD: &str = "application/vnd.sqlite3";
pub const CONTENT_TYPE_MANIFEST: &str = "application/json";
pub const CONTENT_TYPE_AUDIO: &str = "audio/mp4";

pub const CACHE_CONTROL_IMMUTABLE: &str = "public, max-age=31536000, immutable";
pub const CACHE_CONTROL_POINTER: &str = "public, max-age=300";

/// How a published object is served.
#[derive(Debug, Clone, Copy)]
pub struct ObjectHeaders {
    pub content_type: &'static str,
    pub cache_control: &'static str,
}

/// An object store that published bundles are written to.
pub trait ArtifactRepository {
    fn put_file(&self, key: &str, source_path: &Path, headers: ObjectHeaders) -> impl Future<Output = Result<(), AppError>> + Send;

    /// Every key under `prefix`, for skipping objects a previous publish already wrote.
    fn existing_keys(&self, prefix: &str) -> impl Future<Output = Result<BTreeSet<String>, AppError>> + Send;

    fn url(&self, key: &str) -> String;
}

pub enum ArtifactRepositoryKind {
    Local(LocalArtifactRepository),
    CloudflareR2(CloudflareR2ArtifactRepository),
    Dry(DryArtifactRepository),
}

impl ArtifactRepositoryKind {
    pub async fn put_file(&self, key: &str, source_path: &Path, headers: ObjectHeaders) -> Result<(), AppError> {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.put_file(key, source_path, headers).await,
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.put_file(key, source_path, headers).await,
            ArtifactRepositoryKind::Dry(repository) => repository.put_file(key, source_path, headers).await,
        }
    }

    pub async fn existing_keys(&self, prefix: &str) -> Result<BTreeSet<String>, AppError> {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.existing_keys(prefix).await,
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.existing_keys(prefix).await,
            ArtifactRepositoryKind::Dry(repository) => repository.existing_keys(prefix).await,
        }
    }

    pub fn url(&self, key: &str) -> String {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.url(key),
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.url(key),
            ArtifactRepositoryKind::Dry(repository) => repository.url(key),
        }
    }
}
