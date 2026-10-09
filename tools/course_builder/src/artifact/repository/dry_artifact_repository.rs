use std::path::Path;

use shared::AppError;

use crate::artifact::repository::artifact_repository::{
    self, ArtifactRepository, ListedObject, ObjectHeaders, StoredObject, WriteConditionKind, WriteOutcomeKind,
};

/// A repository that logs every write and holds nothing.
pub struct DryArtifactRepository {}

impl DryArtifactRepository {
    pub fn new() -> DryArtifactRepository {
        DryArtifactRepository {}
    }
}

impl ArtifactRepository for DryArtifactRepository {
    async fn put_file(&self, key: &str, source_path: &Path, headers: ObjectHeaders) -> Result<(), AppError> {
        log::info!(
            "[dry] would PUT; [key={key} content_type={} cache_control={} source={}]",
            headers.content_type,
            headers.cache_control,
            source_path.display(),
        );

        Ok(())
    }

    async fn put_bytes(&self, key: &str, bytes: &[u8], headers: ObjectHeaders, condition: WriteConditionKind) -> Result<WriteOutcomeKind, AppError> {
        log::info!(
            "[dry] would PUT; [key={key} content_type={} cache_control={} size_bytes={} condition={condition:?}]",
            headers.content_type,
            headers.cache_control,
            bytes.len(),
        );

        Ok(WriteOutcomeKind::Written)
    }

    async fn get_object(&self, _key: &str) -> Result<Option<StoredObject>, AppError> {
        Ok(None)
    }

    async fn list_objects(&self, directory_prefix: &str) -> Result<Vec<ListedObject>, AppError> {
        artifact_repository::require_directory_prefix(directory_prefix)?;

        Ok(Vec::new())
    }

    async fn delete_keys(&self, keys: &[String]) -> Result<(), AppError> {
        for key in keys {
            log::info!("[dry] would DELETE; [key={key}]");
        }

        Ok(())
    }

    fn url(&self, key: &str) -> String {
        format!("dry:///{key}")
    }
}
