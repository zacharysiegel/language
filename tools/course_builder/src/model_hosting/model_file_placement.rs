use std::path::Path;

use shared::AppError;
use shared::artifact::repository_key;

use crate::artifact::repository::{ArtifactRepository, ListedObject, LocalArtifactRepository};
use crate::model_hosting::{self, HostedModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFilePlacementKind {
    Placed,
    AlreadyPresent,
}

/// Puts `model_file`, checked against `model`, at the model's file key, unless the repository holds the model's size there.
pub async fn place_model_file(repository: &LocalArtifactRepository, model: &HostedModel, model_file: &Path) -> Result<ModelFilePlacementKind, AppError> {
    let file_key: String = model.file_key();
    let listed_objects: Vec<ListedObject> = repository.list_objects(&repository_key::model_directory_key(&model.name, &model.sha256)).await?;
    let already_present: bool = listed_objects
        .iter()
        .any(|listed_object| listed_object.key == file_key && listed_object.size_bytes == model.size_bytes);

    if already_present {
        return Ok(ModelFilePlacementKind::AlreadyPresent);
    }

    model_hosting::verify_model_file(model, model_file)?;
    repository.link_file(&file_key, model_file)?;

    Ok(ModelFilePlacementKind::Placed)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use shared::artifact::content_address;

    use super::*;
    use crate::model_hosting::test_model_files;

    #[tokio::test]
    async fn place_model_file_places_the_file_once_at_its_key() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let model_file: PathBuf = test_model_files::write_fixture_model(directory.path());
        let model: HostedModel = test_model_files::fixture_hosted_model(&model_file, 1);
        let repository: LocalArtifactRepository = LocalArtifactRepository::new(directory.path().join("repository"), "/repository".to_string());

        let first_placement: ModelFilePlacementKind = place_model_file(&repository, &model, &model_file).await.unwrap();
        let second_placement: ModelFilePlacementKind = place_model_file(&repository, &model, &model_file).await.unwrap();

        assert_eq!(first_placement, ModelFilePlacementKind::Placed);
        assert_eq!(second_placement, ModelFilePlacementKind::AlreadyPresent);
        assert_eq!(fs::read(repository.root().join(model.file_key())).unwrap(), fs::read(&model_file).unwrap());
    }

    #[tokio::test]
    async fn place_model_file_refuses_a_file_with_another_sha256() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let model_file: PathBuf = test_model_files::write_fixture_model(directory.path());
        let fixture: HostedModel = test_model_files::fixture_hosted_model(&model_file, 1);
        let model: HostedModel = HostedModel {
            name: fixture.name,
            file_name: fixture.file_name,
            sha256: content_address::sha256_hex(b"another file"),
            size_bytes: fixture.size_bytes,
            part_count: fixture.part_count,
        };
        let repository: LocalArtifactRepository = LocalArtifactRepository::new(directory.path().join("repository"), "/repository".to_string());

        let placed: Result<ModelFilePlacementKind, AppError> = place_model_file(&repository, &model, &model_file).await;

        assert!(placed.err().unwrap().to_string().contains("SHA-256"));
        assert!(!repository.root().join(model.file_key()).exists());
    }
}
