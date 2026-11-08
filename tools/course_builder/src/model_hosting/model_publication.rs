use std::collections::BTreeMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use shared::AppError;
use shared::artifact::{content_address, repository_key};
use shared::grading;

use crate::artifact::local_files;
use crate::artifact::repository::{ArtifactRepository, ListedObject, ObjectHeaders};
use crate::model_hosting::gguf_file::{self, GgufHeader};
use crate::model_hosting::gguf_split::{self, GgufSplitPart};

/* About 512 MiB per part, measured as llama-gguf-split measures it. */
pub const MAXIMUM_PART_TENSOR_BYTES: u64 = 512 * 1024 * 1024;

/// A model file a repository serves, unsplit or in llama.cpp's split format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostedModel {
    pub name: String,
    /// Of the unsplit file.
    pub file_name: String,
    /// Of the unsplit file.
    pub sha256: String,
    pub size_bytes: u64,
    pub part_count: u16,
}

impl HostedModel {
    pub fn grading_model() -> HostedModel {
        HostedModel {
            name: grading::GRADING_MODEL_NAME.to_string(),
            file_name: grading::GRADING_MODEL_FILE_NAME.to_string(),
            sha256: grading::GRADING_MODEL_SHA256.to_string(),
            size_bytes: grading::GRADING_MODEL_SIZE_BYTES,
            part_count: grading::GRADING_MODEL_PART_COUNT,
        }
    }

    pub fn file_key(&self) -> String {
        repository_key::model_file_key(&self.name, &self.sha256, &self.file_name)
    }

    fn part_key(&self, part_number: u16) -> String {
        repository_key::model_part_key(&self.name, &self.sha256, part_number, self.part_count)
    }
}

/// A model file whose size and hash match its `HostedModel`, and the parts it splits into.
pub struct ModelSplit {
    pub model_file: PathBuf,
    pub source: GgufHeader,
    pub parts: Vec<GgufSplitPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelPublicationReport {
    pub first_part_url: String,
    pub parts_uploaded: usize,
    pub parts_already_present: usize,
}

pub fn plan_model_split(model: &HostedModel, model_file: &Path, maximum_part_tensor_bytes: u64) -> Result<ModelSplit, AppError> {
    verify_model_file(model, model_file)?;

    let source: GgufHeader = gguf_file::read_gguf_header_of_file(model_file)?;
    let parts: Vec<GgufSplitPart> = gguf_split::plan_split(&source, maximum_part_tensor_bytes)?;

    if parts.len() != usize::from(model.part_count) {
        return Err(AppError::from(format!(
            "the model splits into another number of parts than its part count; [model={} parts={} part_count={}]",
            model.name,
            parts.len(),
            model.part_count,
        )));
    }

    Ok(ModelSplit {
        model_file: model_file.to_path_buf(),
        source,
        parts,
    })
}

pub fn verify_model_file(model: &HostedModel, model_file: &Path) -> Result<(), AppError> {
    let size_bytes: u64 = fs::metadata(model_file)
        .map_err(|error| AppError::from(format!("reading the model file failed; [path={} error={error}]", model_file.display())))?
        .len();

    if size_bytes != model.size_bytes {
        return Err(AppError::from(format!(
            "the model file has another size than the model's; [path={} size_bytes={size_bytes} expected={}]",
            model_file.display(),
            model.size_bytes,
        )));
    }

    let mut file: File = File::open(model_file)?;
    let sha256: String = content_address::sha256_hex_of_reader(&mut file)?;

    if sha256 != model.sha256 {
        return Err(AppError::from(format!(
            "the model file has another SHA-256 than the model's; [path={} sha256={sha256} expected={}]",
            model_file.display(),
            model.sha256,
        )));
    }

    Ok(())
}

/// Uploads every part the repository lacks at its planned size, writing each in `split_directory` first.
pub async fn upload_model_parts(
    repository: &impl ArtifactRepository,
    model: &HostedModel,
    model_split: &ModelSplit,
    split_directory: &Path,
) -> Result<ModelPublicationReport, AppError> {
    let listed_objects: Vec<ListedObject> = repository.list_objects(&repository_key::model_directory_key(&model.name, &model.sha256)).await?;
    let stored_size_by_key: BTreeMap<String, u64> = listed_objects.into_iter().map(|listed_object| (listed_object.key, listed_object.size_bytes)).collect();
    let mut source_file: File = File::open(&model_split.model_file)?;
    let mut parts_uploaded: usize = 0;
    let mut parts_already_present: usize = 0;

    local_files::remove_directory_if_present(split_directory)?;
    fs::create_dir_all(split_directory)?;

    // The first part goes last, so a repository holding it holds every part.
    for part in model_split.parts.iter().rev() {
        let stored_size_bytes: Option<&u64> = stored_size_by_key.get(&model.part_key(part.part_number));

        if stored_size_bytes == Some(&part.size_bytes) {
            parts_already_present += 1;
            continue;
        }

        upload_part(repository, &mut source_file, model, model_split, part, split_directory).await?;
        parts_uploaded += 1;
    }

    local_files::remove_directory_if_present(split_directory)?;

    Ok(ModelPublicationReport {
        first_part_url: repository.url(&model.part_key(1)),
        parts_uploaded,
        parts_already_present,
    })
}

async fn upload_part(
    repository: &impl ArtifactRepository,
    source_file: &mut File,
    model: &HostedModel,
    model_split: &ModelSplit,
    part: &GgufSplitPart,
    split_directory: &Path,
) -> Result<(), AppError> {
    let key: String = model.part_key(part.part_number);
    let part_path: PathBuf = split_directory.join(repository_key::model_part_file_name(&model.name, part.part_number, model.part_count));

    gguf_split::write_part(source_file, &model_split.source, part, &part_path)?;
    repository.put_file(&key, &part_path, ObjectHeaders::for_key(&key)).await?;
    fs::remove_file(&part_path)?;

    log::info!("uploaded a model part; [key={key} size_bytes={}]", part.size_bytes);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact::repository::memory_artifact_repository::MemoryArtifactRepository;
    use crate::model_hosting::test_model_files;

    const FIXTURE_PART_TENSOR_BYTES: u64 = 64;
    const FIXTURE_PART_COUNT: u16 = 3;

    fn fixture_model(model_file: &Path) -> HostedModel {
        test_model_files::fixture_hosted_model(model_file, FIXTURE_PART_COUNT)
    }

    #[tokio::test]
    async fn upload_model_parts_uploads_the_first_part_last_and_only_parts_missing_at_their_size() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let model_file: PathBuf = test_model_files::write_fixture_model(directory.path());
        let model: HostedModel = fixture_model(&model_file);
        let model_split: ModelSplit = plan_model_split(&model, &model_file, FIXTURE_PART_TENSOR_BYTES).unwrap();
        let split_directory: PathBuf = directory.path().join("split");
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();

        let first_report: ModelPublicationReport = upload_model_parts(&repository, &model, &model_split, &split_directory).await.unwrap();
        let first_put_keys: Vec<String> = repository.put_keys();

        repository.insert(&model.part_key(2), b"truncated");
        repository.clear_operations();

        let second_report: ModelPublicationReport = upload_model_parts(&repository, &model, &model_split, &split_directory).await.unwrap();

        assert_eq!(first_put_keys, vec![model.part_key(3), model.part_key(2), model.part_key(1)]);
        assert_eq!(first_report.parts_uploaded, 3);
        assert_eq!(repository.put_keys(), vec![model.part_key(2)]);
        assert_eq!(second_report.parts_already_present, 2);
        assert!(second_report.first_part_url.ends_with("/fixture-00001-of-00003.gguf"));
        assert!(!split_directory.exists());
    }

    #[test]
    fn plan_model_split_refuses_a_file_with_another_sha256() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let model_file: PathBuf = test_model_files::write_fixture_model(directory.path());
        let fixture: HostedModel = fixture_model(&model_file);
        let model: HostedModel = HostedModel {
            name: fixture.name,
            file_name: fixture.file_name,
            sha256: content_address::sha256_hex(b"another file"),
            size_bytes: fixture.size_bytes,
            part_count: fixture.part_count,
        };

        let planned: Result<ModelSplit, AppError> = plan_model_split(&model, &model_file, FIXTURE_PART_TENSOR_BYTES);

        assert!(planned.err().unwrap().to_string().contains("SHA-256"));
    }

    #[test]
    fn plan_model_split_refuses_a_split_into_another_number_of_parts() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let model_file: PathBuf = test_model_files::write_fixture_model(directory.path());
        let model: HostedModel = fixture_model(&model_file);

        let planned: Result<ModelSplit, AppError> = plan_model_split(&model, &model_file, FIXTURE_PART_TENSOR_BYTES * 4);

        assert!(planned.err().unwrap().to_string().contains("part count"));
    }

    #[test]
    fn plan_model_split_refuses_a_file_with_another_size() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let model_file: PathBuf = test_model_files::write_fixture_model(directory.path());
        let fixture: HostedModel = fixture_model(&model_file);
        let model: HostedModel = HostedModel {
            name: fixture.name,
            file_name: fixture.file_name,
            sha256: fixture.sha256,
            size_bytes: fixture.size_bytes + 1,
            part_count: fixture.part_count,
        };

        let planned: Result<ModelSplit, AppError> = plan_model_split(&model, &model_file, FIXTURE_PART_TENSOR_BYTES);

        assert!(planned.err().unwrap().to_string().contains("another size"));
    }
}
