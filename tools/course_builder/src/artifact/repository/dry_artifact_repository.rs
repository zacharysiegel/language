use std::collections::BTreeSet;
use std::path::Path;

use shared::AppError;

use crate::artifact::repository::artifact_repository::{ArtifactRepository, ObjectHeaders};

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

    async fn existing_keys(&self, _prefix: &str) -> Result<BTreeSet<String>, AppError> {
        Ok(BTreeSet::new())
    }

    fn url(&self, key: &str) -> String {
        format!("dry:///{key}")
    }
}
