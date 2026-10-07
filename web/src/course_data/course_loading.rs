use std::sync::Arc;

use shared::artifact::ManifestSource;
use shared::course::CourseOutline;
use shared::course_shard::ClipAudioEntry;

/// A course outline read from one published version.
#[derive(Debug, Clone)]
pub struct LoadedCourse {
    pub source: ManifestSource,
    pub audio_base_url: String,
    pub outline: Arc<CourseOutline>,
    pub clip_audio: Vec<ClipAudioEntry>,
}

impl LoadedCourse {
    pub fn version(&self) -> &str {
        &self.source.manifest.version
    }
}

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
pub use browser::*;
#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
pub use server::*;

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
mod browser {
    use std::sync::Arc;

    use shared::AppError;
    use shared::artifact::{self, ManifestEntry, ManifestSource, discovery};
    use shared::course_shard::{self, CourseShardContents, UnitShardContents};

    use super::LoadedCourse;
    use crate::artifact_client::{BrowserFetch, OpfsArtifactCache};
    use crate::platform;

    const MANIFEST_STORAGE_KEY: &str = "portuguese-course.artifact.manifest";
    const REPOSITORY_BASE_URL_STORAGE_KEY: &str = "portuguese-course.artifact.repository-base-url";

    /// The version this browser last loaded, read from its cache; `None` on a first visit.
    pub async fn load_cached_course() -> Option<LoadedCourse> {
        let manifest_text: String = platform::read_stored_text(MANIFEST_STORAGE_KEY)?;
        let repository_base_url: String = platform::read_stored_text(REPOSITORY_BASE_URL_STORAGE_KEY)?;
        let loaded: Result<LoadedCourse, AppError> = async {
            let source: ManifestSource = artifact::parse_cached_manifest(manifest_text.into_bytes(), &repository_base_url)?;

            load_course_from(source, repository_base_url.clone()).await
        }
        .await;

        loaded.map_err(|error| log::warn!("could not open the cached course; [error={error}]")).ok()
    }

    /// The version the repository currently publishes; remembered, and the cache trimmed to it.
    pub async fn load_latest_course() -> Result<LoadedCourse, AppError> {
        let source: ManifestSource =
            artifact::discover_latest_manifest(&BrowserFetch, discovery::DISCOVERY_PATH, discovery::STATIC_REPOSITORY_BASE_URL).await?;
        let audio_base_url: String = source.repository_base_url.clone();
        let loaded: LoadedCourse = load_course_from(source, audio_base_url).await?;

        remember_version(&loaded.source).await;

        Ok(loaded)
    }

    /// The bundle shipped with the app; its recordings are fetched from the static repository.
    pub async fn load_embedded_course() -> Result<LoadedCourse, AppError> {
        let source: ManifestSource = artifact::load_embedded_manifest(&BrowserFetch, discovery::EMBEDDED_BUNDLE_BASE_URL).await?;

        load_course_from(source, discovery::STATIC_REPOSITORY_BASE_URL.to_string()).await
    }

    pub async fn load_unit_shard(course: &LoadedCourse, unit_id: &str) -> Result<UnitShardContents, AppError> {
        let entry: &ManifestEntry = course
            .source
            .manifest
            .units
            .get(unit_id)
            .ok_or_else(|| AppError::from(format!("the published course has no such unit; [unit_id={unit_id}]")))?;
        let artifact_cache: OpfsArtifactCache = OpfsArtifactCache::open().await;
        let bytes: Vec<u8> = artifact::load_shard(&BrowserFetch, &artifact_cache, &course.source, entry).await?;

        course_shard::read_unit_shard(&bytes)
    }

    async fn load_course_from(source: ManifestSource, audio_base_url: String) -> Result<LoadedCourse, AppError> {
        let artifact_cache: OpfsArtifactCache = OpfsArtifactCache::open().await;
        let bytes: Vec<u8> = artifact::load_shard(&BrowserFetch, &artifact_cache, &source, &source.manifest.course).await?;
        let contents: CourseShardContents = course_shard::read_course_shard(&bytes)?;

        Ok(LoadedCourse {
            source,
            audio_base_url,
            outline: Arc::new(contents.outline),
            clip_audio: contents.clip_audio,
        })
    }

    async fn remember_version(source: &ManifestSource) {
        let Ok(manifest_text) = String::from_utf8(source.manifest_bytes.clone())
        else {
            return;
        };

        platform::store_text(MANIFEST_STORAGE_KEY, &manifest_text);
        platform::store_text(REPOSITORY_BASE_URL_STORAGE_KEY, &source.repository_base_url);

        let kept_paths: Vec<String> = source.manifest.file_entries().map(|entry| entry.relative_path.clone()).collect();
        let artifact_cache: OpfsArtifactCache = OpfsArtifactCache::open().await;
        let trimmed: Result<(), AppError> = shared::artifact::ArtifactCache::retain(&artifact_cache, &kept_paths).await;

        if let Err(error) = trimmed {
            log::warn!("could not trim the artifact cache; [error={error}]");
        }
    }
}

#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
mod server {
    use shared::AppError;
    use shared::course_shard::UnitShardContents;

    use super::LoadedCourse;

    // The server renders a static shell; nothing below runs before the client attaches.
    pub async fn load_cached_course() -> Option<LoadedCourse> {
        None
    }

    pub async fn load_latest_course() -> Result<LoadedCourse, AppError> {
        Err(AppError::from("the course loads only in the browser".to_string()))
    }

    pub async fn load_embedded_course() -> Result<LoadedCourse, AppError> {
        Err(AppError::from("the course loads only in the browser".to_string()))
    }

    pub async fn load_unit_shard(_course: &LoadedCourse, _unit_id: &str) -> Result<UnitShardContents, AppError> {
        Err(AppError::from("the course loads only in the browser".to_string()))
    }
}
