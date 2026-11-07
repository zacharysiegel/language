use std::path::{Path, PathBuf};

use shared::AppError;

use crate::artifact::repository::{self, ArtifactRepositoryKind, RepositoryLockKind};
use crate::model_hosting::{self, HostedModel, ModelPublicationReport, ModelSplit};
use crate::publish_command::{self, RepositoryLocationKind};
use crate::workspace::WorkspacePaths;

pub fn run_publish_model(paths: &WorkspacePaths, location: RepositoryLocationKind, model_file: &Path) -> Result<bool, AppError> {
    let model: HostedModel = HostedModel::grading_model();

    log::info!("checking the model file; [path={} model={}]", model_file.display(), model.name);

    let model_split: ModelSplit = model_hosting::plan_model_split(&model, model_file, model_hosting::MAXIMUM_PART_TENSOR_BYTES)?;
    let split_directory: PathBuf = paths.model_split_directory.join(&model.name);
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let report: ModelPublicationReport = runtime.block_on(async {
        let repository: ArtifactRepositoryKind = publish_command::create_repository(paths, location).await?;
        let lock: RepositoryLockKind = repository::acquire_repository_lock(&repository).await?;

        let uploaded: Result<ModelPublicationReport, AppError> = model_hosting::upload_model_parts(&repository, &model, &model_split, &split_directory).await;
        let released: Result<(), AppError> = lock.release(&repository).await;

        repository::combine_with_release(uploaded, released)
    })?;

    log::info!(
        "published the model; [model={} first_part={} parts_uploaded={} parts_already_present={}]",
        model.name,
        report.first_part_url,
        report.parts_uploaded,
        report.parts_already_present,
    );

    Ok(true)
}
