use std::path::{Path, PathBuf};

use shared::AppError;
use shared::artifact::repository_key;
use shared::filesystem;

use crate::speech_engine::SpeechEngineKind;

const COURSES_DIRECTORY: &str = "data/courses";
const CURRICULUM_FILE_NAME: &str = "curriculum.json";
const UNITS_DIRECTORY_NAME: &str = "units";
const SPEECH_MODELS_DIRECTORY: &str = "models";
const CLIP_STORE_DIRECTORY: &str = "build/clips";
const BUNDLES_DIRECTORY: &str = "build/artifacts";
const LOCAL_REPOSITORY_DIRECTORY: &str = "build/repository";
const EMBEDDED_BUNDLE_DIRECTORY: &str = "web/static/embedded";

pub struct WorkspacePaths {
    pub courses_directory: PathBuf,
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
            courses_directory: root.join(COURSES_DIRECTORY),
            speech_models_directory: root.join(SPEECH_MODELS_DIRECTORY),
            clip_store_directory: root.join(CLIP_STORE_DIRECTORY),
            bundles_directory: root.join(BUNDLES_DIRECTORY),
            local_repository_directory: root.join(LOCAL_REPOSITORY_DIRECTORY),
            embedded_bundle_directory: root.join(EMBEDDED_BUNDLE_DIRECTORY),
        }
    }

    pub fn course_directory(&self, course_id: &str) -> PathBuf {
        self.courses_directory.join(course_id)
    }

    pub fn curriculum_path(&self, course_id: &str) -> PathBuf {
        self.course_directory(course_id).join(CURRICULUM_FILE_NAME)
    }

    pub fn unit_path(&self, course_id: &str, unit_id: &str) -> PathBuf {
        self.course_directory(course_id).join(UNITS_DIRECTORY_NAME).join(format!("{unit_id}.json"))
    }

    pub fn engine_clip_directory(&self, engine: SpeechEngineKind) -> PathBuf {
        self.clip_store_directory.join(engine.code())
    }

    pub fn course_bundles_directory(&self, course_id: &str) -> PathBuf {
        self.bundles_directory.join(course_id)
    }

    pub fn local_repository_course_directory(&self, course_id: &str) -> PathBuf {
        self.local_repository_directory.join(repository_key::course_directory_key(course_id))
    }

    pub fn embedded_course_directory(&self, course_id: &str) -> PathBuf {
        self.embedded_bundle_directory.join(repository_key::course_directory_key(course_id))
    }
}
