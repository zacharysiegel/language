use std::path::{Path, PathBuf};

use shared::AppError;
use shared::filesystem;

use crate::speech_engine::SpeechEngineKind;

const CURRICULUM_PATH: &str = "data/curriculum.json";
const UNITS_DIRECTORY: &str = "data/units";
const SPEECH_MODELS_DIRECTORY: &str = "models";
const CLIP_STORE_DIRECTORY: &str = "build/clips";
const BUNDLES_DIRECTORY: &str = "build/artifacts";
const LOCAL_REPOSITORY_DIRECTORY: &str = "build/repository";
const EMBEDDED_BUNDLE_DIRECTORY: &str = "web/static/embedded";

pub struct WorkspacePaths {
    pub curriculum_path: PathBuf,
    pub units_directory: PathBuf,
    pub speech_models_directory: PathBuf,
    pub clip_store_directory: PathBuf,
    pub bundles_directory: PathBuf,
    pub local_repository_directory: PathBuf,
    pub embedded_bundle_directory: PathBuf,
}

impl WorkspacePaths {
    pub fn locate() -> Result<WorkspacePaths, AppError> {
        let current_directory: PathBuf = std::env::current_dir()?;
        let root: PathBuf = filesystem::find_workspace_root(&current_directory)?;

        Ok(WorkspacePaths::at_root(&root))
    }

    pub fn at_root(root: &Path) -> WorkspacePaths {
        WorkspacePaths {
            curriculum_path: root.join(CURRICULUM_PATH),
            units_directory: root.join(UNITS_DIRECTORY),
            speech_models_directory: root.join(SPEECH_MODELS_DIRECTORY),
            clip_store_directory: root.join(CLIP_STORE_DIRECTORY),
            bundles_directory: root.join(BUNDLES_DIRECTORY),
            local_repository_directory: root.join(LOCAL_REPOSITORY_DIRECTORY),
            embedded_bundle_directory: root.join(EMBEDDED_BUNDLE_DIRECTORY),
        }
    }

    pub fn unit_path(&self, unit_id: &str) -> PathBuf {
        self.units_directory.join(format!("{unit_id}.json"))
    }

    pub fn engine_clip_directory(&self, engine: SpeechEngineKind) -> PathBuf {
        self.clip_store_directory.join(engine.code())
    }
}
