use std::path::PathBuf;

use shared::AppError;

use crate::artifact::repository::{self, ArtifactRepository, ArtifactRepositoryKind, LocalArtifactRepository, LocalRepositoryLock, RepositoryLockKind};
use crate::model_hosting::{self, HostedModel, ModelFilePlacementKind, ModelPublicationReport, ModelSplit};
use crate::publish_command::{self, RepositoryLocationKind};
use crate::workspace::WorkspacePaths;

/* Off for now: production downloads the grading model from its Hugging Face repository. */
const R2_MODEL_HOSTING_ENABLED: bool = false;

pub fn run_publish_model_local(paths: &WorkspacePaths, root: Option<PathBuf>, model_file: Option<PathBuf>) -> Result<bool, AppError> {
    let model: HostedModel = HostedModel::grading_model();
    let model_file: PathBuf = chosen_model_file(paths, &model, model_file);
    let repository: LocalArtifactRepository = publish_command::create_local_repository(paths, root, None);
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let placement: ModelFilePlacementKind = runtime.block_on(async {
        let lock: LocalRepositoryLock = repository::lock_local_repository(repository.root()).await?;

        let placed: Result<ModelFilePlacementKind, AppError> = model_hosting::place_model_file(&repository, &model, &model_file).await;
        let released: Result<(), AppError> = lock.release();

        repository::combine_with_release(placed, released)
    })?;

    log::info!(
        "published the model; [model={} url={} placement={placement:?} source={}]",
        model.name,
        repository.url(&model.file_key()),
        model_file.display(),
    );

    Ok(true)
}

pub fn run_publish_model_cloudflare_r2(paths: &WorkspacePaths, model_file: Option<PathBuf>) -> Result<bool, AppError> {
    if !R2_MODEL_HOSTING_ENABLED {
        log::warn!("R2 model hosting is turned off for now, so nothing was uploaded; production downloads the grading model from Hugging Face");
        return Ok(true);
    }

    let model: HostedModel = HostedModel::grading_model();
    let model_file: PathBuf = chosen_model_file(paths, &model, model_file);

    log::info!("checking the model file; [path={} model={}]", model_file.display(), model.name);

    let model_split: ModelSplit = model_hosting::plan_model_split(&model, &model_file, model_hosting::MAXIMUM_PART_TENSOR_BYTES)?;
    let split_directory: PathBuf = paths.model_split_directory.join(&model.name);
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let report: ModelPublicationReport = runtime.block_on(async {
        let repository: ArtifactRepositoryKind = publish_command::create_repository(paths, RepositoryLocationKind::CloudflareR2).await?;
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

fn chosen_model_file(paths: &WorkspacePaths, model: &HostedModel, model_file: Option<PathBuf>) -> PathBuf {
    model_file.unwrap_or_else(|| paths.models_directory.join(&model.file_name))
}
