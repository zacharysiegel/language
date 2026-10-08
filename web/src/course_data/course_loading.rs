use shared::artifact::ManifestSource;

/// A published version of the courses, and where its recordings are served.
#[derive(Debug, Clone)]
pub struct LoadedVersion {
    pub source: ManifestSource,
    pub audio_base_url: String,
}

impl LoadedVersion {
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
    use shared::artifact::{self, ManifestEntry, ManifestSource, discovery};
    use shared::{AppError, AppErrorStatic};
    use shared::course_shard::{self, CourseShardContents, UnitShardContents};

    use super::LoadedVersion;
    use crate::artifact_client::{BrowserFetch, OpfsArtifactCache};
    use crate::platform;

    const MANIFEST_STORAGE_KEY: &str = "language.artifact.manifest";
    const REPOSITORY_BASE_URL_STORAGE_KEY: &str = "language.artifact.repository-base-url";

    /// The version this browser last loaded; `None` on a first visit.
    pub async fn load_cached_version() -> Option<LoadedVersion> {
        let manifest_text: String = platform::read_stored_text(MANIFEST_STORAGE_KEY)?;
        let repository_base_url: String = platform::read_stored_text(REPOSITORY_BASE_URL_STORAGE_KEY)?;
        let parsed: Result<ManifestSource, AppError> = artifact::parse_cached_manifest(manifest_text.into_bytes(), &repository_base_url);

        match parsed {
            Ok(source) => Some(LoadedVersion {
                source,
                audio_base_url: repository_base_url,
            }),
            Err(error) => {
                log::warn!("could not read the cached manifest; [error={error}]");
                None
            },
        }
    }

    /// The version the repository publishes now; remembered, and the cache trimmed to it.
    pub async fn load_latest_version() -> Result<LoadedVersion, AppError> {
        let source: ManifestSource =
            artifact::discover_latest_manifest(&BrowserFetch, discovery::DISCOVERY_PATH, discovery::STATIC_REPOSITORY_BASE_URL).await?;

        remember_version(&source).await;

        Ok(LoadedVersion {
            audio_base_url: source.repository_base_url.clone(),
            source,
        })
    }

    /// The bundle shipped with the app; its recordings are fetched from the static repository.
    pub async fn load_embedded_version() -> Result<LoadedVersion, AppError> {
        let source: ManifestSource = artifact::load_embedded_manifest(&BrowserFetch, discovery::EMBEDDED_BUNDLE_BASE_URL).await?;

        Ok(LoadedVersion {
            source,
            audio_base_url: discovery::STATIC_REPOSITORY_BASE_URL.to_string(),
        })
    }

    pub async fn load_course_shard(version: &LoadedVersion, course_id: &str) -> Result<CourseShardContents, AppError> {
        let entry: &ManifestEntry = &version.source.manifest.find_course(course_id)?.outline;
        let bytes: Vec<u8> = load_shard(version, entry).await?;

        course_shard::read_course_shard(&bytes)
    }

    pub async fn load_unit_shard(version: &LoadedVersion, course_id: &str, unit_id: &str) -> Result<UnitShardContents, AppError> {
        let entry: &ManifestEntry = version.source.manifest.find_course(course_id)?.find_unit(unit_id)?;
        let bytes: Vec<u8> = load_shard(version, entry).await?;

        course_shard::read_unit_shard(&bytes)
    }

    async fn load_shard(version: &LoadedVersion, entry: &ManifestEntry) -> Result<Vec<u8>, AppErrorStatic> {
        let artifact_cache: OpfsArtifactCache = OpfsArtifactCache::open().await;

        artifact::load_shard(&BrowserFetch, &artifact_cache, &version.source, entry).await
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
        let trimmed: Result<(), AppErrorStatic> = shared::artifact::ArtifactCache::retain(&artifact_cache, &kept_paths).await;

        if let Err(error) = trimmed {
            log::warn!("could not trim the artifact cache; [error={error}]");
        }
    }
}

#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
mod server {
    use shared::AppError;
    use shared::course_shard::{CourseShardContents, UnitShardContents};

    use super::LoadedVersion;

    // The server renders a static shell; nothing below runs before the client attaches.
    pub async fn load_cached_version() -> Option<LoadedVersion> {
        None
    }

    pub async fn load_latest_version() -> Result<LoadedVersion, AppError> {
        Err(AppError::from("courses load only in the browser".to_string()))
    }

    pub async fn load_embedded_version() -> Result<LoadedVersion, AppError> {
        Err(AppError::from("courses load only in the browser".to_string()))
    }

    pub async fn load_course_shard(_version: &LoadedVersion, _course_id: &str) -> Result<CourseShardContents, AppError> {
        Err(AppError::from("courses load only in the browser".to_string()))
    }

    pub async fn load_unit_shard(_version: &LoadedVersion, _course_id: &str, _unit_id: &str) -> Result<UnitShardContents, AppError> {
        Err(AppError::from("courses load only in the browser".to_string()))
    }
}
