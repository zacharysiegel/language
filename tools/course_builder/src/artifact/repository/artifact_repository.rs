use std::collections::BTreeSet;
use std::future::Future;
use std::path::Path;

use chrono::{DateTime, Utc};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredObject {
    pub bytes: Vec<u8>,
    /// Opaque; meaningful only when passed back in `WriteConditionKind::MatchesEntityTag`.
    pub entity_tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteConditionKind {
    Unconditional,
    Absent,
    MatchesEntityTag(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteOutcomeKind {
    Written,
    PreconditionFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedObject {
    pub key: String,
    pub uploaded: DateTime<Utc>,
}

/// An object store that published bundles are written to.
pub trait ArtifactRepository {
    fn put_file(&self, key: &str, source_path: &Path, headers: ObjectHeaders) -> impl Future<Output = Result<(), AppError>> + Send;

    fn put_bytes(
        &self,
        key: &str,
        bytes: &[u8],
        headers: ObjectHeaders,
        condition: WriteConditionKind,
    ) -> impl Future<Output = Result<WriteOutcomeKind, AppError>> + Send;

    fn get_object(&self, key: &str) -> impl Future<Output = Result<Option<StoredObject>, AppError>> + Send;

    /// Every object whose key starts with `directory_prefix`, which must end in `/`.
    fn list_objects(&self, directory_prefix: &str) -> impl Future<Output = Result<Vec<ListedObject>, AppError>> + Send;

    /// A key already absent is not an error.
    fn delete_keys(&self, keys: &[String]) -> impl Future<Output = Result<(), AppError>> + Send;

    fn url(&self, key: &str) -> String;
}

/// A prefix without its trailing `/` also matches every sibling name it begins.
pub fn require_directory_prefix(directory_prefix: &str) -> Result<(), AppError> {
    if !directory_prefix.ends_with('/') {
        return Err(AppError::from(format!("a listing prefix must end in '/'; [prefix={directory_prefix}]")));
    }

    Ok(())
}

pub async fn list_keys(repository: &impl ArtifactRepository, directory_prefix: &str) -> Result<BTreeSet<String>, AppError> {
    let listed_objects: Vec<ListedObject> = repository.list_objects(directory_prefix).await?;

    Ok(listed_objects.into_iter().map(|listed_object| listed_object.key).collect())
}

pub enum ArtifactRepositoryKind {
    Local(LocalArtifactRepository),
    CloudflareR2(CloudflareR2ArtifactRepository),
    Dry(DryArtifactRepository),
}

impl ArtifactRepository for ArtifactRepositoryKind {
    async fn put_file(&self, key: &str, source_path: &Path, headers: ObjectHeaders) -> Result<(), AppError> {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.put_file(key, source_path, headers).await,
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.put_file(key, source_path, headers).await,
            ArtifactRepositoryKind::Dry(repository) => repository.put_file(key, source_path, headers).await,
        }
    }

    async fn put_bytes(&self, key: &str, bytes: &[u8], headers: ObjectHeaders, condition: WriteConditionKind) -> Result<WriteOutcomeKind, AppError> {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.put_bytes(key, bytes, headers, condition).await,
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.put_bytes(key, bytes, headers, condition).await,
            ArtifactRepositoryKind::Dry(repository) => repository.put_bytes(key, bytes, headers, condition).await,
        }
    }

    async fn get_object(&self, key: &str) -> Result<Option<StoredObject>, AppError> {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.get_object(key).await,
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.get_object(key).await,
            ArtifactRepositoryKind::Dry(repository) => repository.get_object(key).await,
        }
    }

    async fn list_objects(&self, directory_prefix: &str) -> Result<Vec<ListedObject>, AppError> {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.list_objects(directory_prefix).await,
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.list_objects(directory_prefix).await,
            ArtifactRepositoryKind::Dry(repository) => repository.list_objects(directory_prefix).await,
        }
    }

    async fn delete_keys(&self, keys: &[String]) -> Result<(), AppError> {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.delete_keys(keys).await,
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.delete_keys(keys).await,
            ArtifactRepositoryKind::Dry(repository) => repository.delete_keys(keys).await,
        }
    }

    fn url(&self, key: &str) -> String {
        match self {
            ArtifactRepositoryKind::Local(repository) => repository.url(key),
            ArtifactRepositoryKind::CloudflareR2(repository) => repository.url(key),
            ArtifactRepositoryKind::Dry(repository) => repository.url(key),
        }
    }
}
