use shared::artifact::CourseManifestSource;

/// One course's publication, and where its recordings are served.
#[derive(Debug, Clone)]
pub struct LoadedCoursePublication {
    pub source: CourseManifestSource,
    pub audio_base_url: String,
}

impl LoadedCoursePublication {
    pub fn course_id(&self) -> &str {
        &self.source.manifest.course_id
    }

    pub fn publication_label(&self) -> &str {
        &self.source.manifest.publication_label
    }

    /// Whether shards and recordings loaded through `other` are the ones this publication serves.
    pub fn serves_the_same_files_as(&self, other: &LoadedCoursePublication) -> bool {
        self.source.manifest_bytes == other.source.manifest_bytes && self.audio_base_url == other.audio_base_url
    }
}

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
pub use browser::*;
#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
pub use server::*;

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
mod browser {
    use std::collections::BTreeSet;

    use shared::artifact::{self, ArtifactCache, CourseIndexEntry, CourseIndexSource, CourseManifest, CourseManifestSource, ManifestEntry, discovery};
    use shared::course_shard::{self, CourseShardContents, UnitShardContents};
    use shared::{AppError, AppErrorStatic};

    use super::LoadedCoursePublication;
    use crate::artifact_client::{BrowserFetch, OpfsArtifactCache};
    use crate::platform;

    const COURSE_INDEX_STORAGE_KEY: &str = "language.artifact.course-index";
    const COURSE_INDEX_BASE_URL_STORAGE_KEY: &str = "language.artifact.course-index-base-url";
    const COURSE_MANIFEST_STORAGE_KEY_PREFIX: &str = "language.artifact.course-manifest.";
    const COURSE_MANIFEST_BASE_URL_STORAGE_KEY_PREFIX: &str = "language.artifact.course-manifest-base-url.";
    const LEGACY_MANIFEST_STORAGE_KEY: &str = "language.artifact.manifest";
    const LEGACY_REPOSITORY_BASE_URL_STORAGE_KEY: &str = "language.artifact.repository-base-url";

    fn course_manifest_storage_key(course_id: &str) -> String {
        format!("{COURSE_MANIFEST_STORAGE_KEY_PREFIX}{course_id}")
    }

    fn course_manifest_base_url_storage_key(course_id: &str) -> String {
        format!("{COURSE_MANIFEST_BASE_URL_STORAGE_KEY_PREFIX}{course_id}")
    }

    /// Deletes what the browser stored for the single manifest that listed every course.
    pub fn forget_legacy_manifest() {
        platform::remove_stored_text(LEGACY_MANIFEST_STORAGE_KEY);
        platform::remove_stored_text(LEGACY_REPOSITORY_BASE_URL_STORAGE_KEY);
    }

    /// The course index this browser last read from a repository; `None` when it has read none.
    pub async fn load_cached_course_index() -> Option<CourseIndexSource> {
        let course_index_text: String = platform::read_stored_text(COURSE_INDEX_STORAGE_KEY)?;
        let repository_base_url: String = platform::read_stored_text(COURSE_INDEX_BASE_URL_STORAGE_KEY)?;
        let parsed: Result<CourseIndexSource, AppError> = artifact::parse_cached_course_index(course_index_text.into_bytes(), &repository_base_url);

        match parsed {
            Ok(source) => Some(source),
            Err(error) => {
                log::warn!("could not read the cached course index; [error={error}]");
                None
            },
        }
    }

    /// The course index the repository serves now; remembered, and the courses it omits forgotten.
    pub async fn load_latest_course_index() -> Result<CourseIndexSource, AppError> {
        let source: CourseIndexSource =
            artifact::discover_course_index(&BrowserFetch, discovery::DISCOVERY_PATH, discovery::STATIC_REPOSITORY_BASE_URL).await?;

        remember_course_index(&source).await;

        Ok(source)
    }

    pub async fn load_embedded_course_index() -> Result<CourseIndexSource, AppError> {
        let source: CourseIndexSource = artifact::load_embedded_course_index(&BrowserFetch, discovery::EMBEDDED_BUNDLE_BASE_URL).await?;

        Ok(source)
    }

    /// The course's publication this browser last read from a repository; `None` when it has read none.
    pub async fn load_cached_course_publication(course_id: &str) -> Option<LoadedCoursePublication> {
        let manifest_text: String = platform::read_stored_text(&course_manifest_storage_key(course_id))?;
        let repository_base_url: String = platform::read_stored_text(&course_manifest_base_url_storage_key(course_id))?;
        let parsed: Result<CourseManifestSource, AppError> =
            artifact::parse_cached_course_manifest(manifest_text.into_bytes(), &repository_base_url, course_id);

        match parsed {
            Ok(source) => Some(LoadedCoursePublication {
                source,
                audio_base_url: repository_base_url,
            }),
            Err(error) => {
                log::warn!("could not read a cached course manifest; [course_id={course_id} error={error}]");
                None
            },
        }
    }

    /// The publication the course's latest pointer names in the repository `course_index` came from; remembered.
    pub async fn load_latest_course_publication(course_index: &CourseIndexSource, course_id: &str) -> Result<LoadedCoursePublication, AppError> {
        let source: CourseManifestSource = artifact::fetch_latest_course_manifest(&BrowserFetch, course_index, course_id).await?;

        remember_course_publication(&source).await;

        Ok(LoadedCoursePublication {
            audio_base_url: source.repository_base_url.clone(),
            source,
        })
    }

    /// The course's publication in the bundle included in the app; its recordings are fetched from the static repository.
    pub async fn load_embedded_course_publication(course_id: &str) -> Result<LoadedCoursePublication, AppError> {
        let embedded_index: CourseIndexSource = load_embedded_course_index().await?;
        let entry: &CourseIndexEntry = embedded_index
            .course_index
            .courses
            .get(course_id)
            .ok_or_else(|| AppError::from(format!("the embedded bundle does not include the course; [course_id={course_id}]")))?;

        let source: CourseManifestSource =
            artifact::load_embedded_course_manifest(&BrowserFetch, discovery::EMBEDDED_BUNDLE_BASE_URL, course_id, entry).await?;

        Ok(LoadedCoursePublication {
            source,
            audio_base_url: discovery::STATIC_REPOSITORY_BASE_URL.to_string(),
        })
    }

    pub async fn load_course_shard(publication: &LoadedCoursePublication) -> Result<CourseShardContents, AppError> {
        let entry: &ManifestEntry = &publication.source.manifest.outline;
        let bytes: Vec<u8> = load_publication_shard(publication, entry).await?;

        course_shard::read_course_shard(&bytes)
    }

    pub async fn load_unit_shard(publication: &LoadedCoursePublication, unit_id: &str) -> Result<UnitShardContents, AppError> {
        let entry: &ManifestEntry = publication.source.manifest.find_unit(unit_id)?;
        let bytes: Vec<u8> = load_publication_shard(publication, entry).await?;

        course_shard::read_unit_shard(&bytes)
    }

    async fn load_publication_shard(publication: &LoadedCoursePublication, entry: &ManifestEntry) -> Result<Vec<u8>, AppErrorStatic> {
        let artifact_cache: OpfsArtifactCache = OpfsArtifactCache::open().await;

        artifact::load_publication_shard(&BrowserFetch, &artifact_cache, &publication.source, entry).await
    }

    async fn remember_course_index(source: &CourseIndexSource) {
        let Ok(course_index_text) = String::from_utf8(source.course_index_bytes.clone())
        else {
            return;
        };

        platform::store_text(COURSE_INDEX_STORAGE_KEY, &course_index_text);
        platform::store_text(COURSE_INDEX_BASE_URL_STORAGE_KEY, &source.repository_base_url);

        let forgotten_course_ids: Vec<String> = stored_course_manifest_ids()
            .into_iter()
            .filter(|course_id| !source.course_index.courses.contains_key(course_id))
            .collect();

        if forgotten_course_ids.is_empty() {
            return;
        }

        for course_id in &forgotten_course_ids {
            log::info!("forgetting a course the course index no longer lists; [course_id={course_id}]");
            forget_course_publication(course_id);
        }

        retain_files_of_every_stored_publication().await;
    }

    async fn remember_course_publication(source: &CourseManifestSource) {
        let Ok(manifest_text) = String::from_utf8(source.manifest_bytes.clone())
        else {
            return;
        };
        let course_id: &str = &source.manifest.course_id;

        platform::store_text(&course_manifest_storage_key(course_id), &manifest_text);
        platform::store_text(&course_manifest_base_url_storage_key(course_id), &source.repository_base_url);

        retain_files_of_every_stored_publication().await;
    }

    fn forget_course_publication(course_id: &str) {
        platform::remove_stored_text(&course_manifest_storage_key(course_id));
        platform::remove_stored_text(&course_manifest_base_url_storage_key(course_id));
    }

    fn stored_course_manifest_ids() -> BTreeSet<String> {
        platform::stored_keys_with_prefix(COURSE_MANIFEST_STORAGE_KEY_PREFIX)
            .into_iter()
            .filter_map(|key| key.strip_prefix(COURSE_MANIFEST_STORAGE_KEY_PREFIX).map(str::to_string))
            .collect()
    }

    fn stored_course_manifests() -> Vec<CourseManifest> {
        let mut course_manifests: Vec<CourseManifest> = Vec::new();

        for course_id in stored_course_manifest_ids() {
            let Some(manifest_text) = platform::read_stored_text(&course_manifest_storage_key(&course_id))
            else {
                continue;
            };
            let parsed: Result<CourseManifest, AppError> = artifact::parse_course_manifest(manifest_text.as_bytes(), Some(&course_id));

            match parsed {
                Ok(course_manifest) => course_manifests.push(course_manifest),
                Err(error) => log::warn!("could not read a cached course manifest; [course_id={course_id} error={error}]"),
            }
        }

        course_manifests
    }

    async fn retain_files_of_every_stored_publication() {
        let course_manifests: Vec<CourseManifest> = stored_course_manifests();
        let kept_paths: Vec<String> = artifact::relative_paths_to_keep(&course_manifests);
        let artifact_cache: OpfsArtifactCache = OpfsArtifactCache::open().await;
        let trimmed: Result<(), AppErrorStatic> = artifact_cache.retain(&kept_paths).await;

        if let Err(error) = trimmed {
            log::warn!("could not trim the artifact cache; [error={error}]");
        }
    }
}

#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
mod server {
    use shared::AppError;
    use shared::artifact::CourseIndexSource;
    use shared::course_shard::{CourseShardContents, UnitShardContents};

    use super::LoadedCoursePublication;

    // The server renders a static shell; nothing below runs before the client attaches.
    pub fn forget_legacy_manifest() {}

    pub async fn load_cached_course_index() -> Option<CourseIndexSource> {
        None
    }

    pub async fn load_latest_course_index() -> Result<CourseIndexSource, AppError> {
        Err(courses_load_only_in_the_browser())
    }

    pub async fn load_embedded_course_index() -> Result<CourseIndexSource, AppError> {
        Err(courses_load_only_in_the_browser())
    }

    pub async fn load_cached_course_publication(_course_id: &str) -> Option<LoadedCoursePublication> {
        None
    }

    pub async fn load_latest_course_publication(_course_index: &CourseIndexSource, _course_id: &str) -> Result<LoadedCoursePublication, AppError> {
        Err(courses_load_only_in_the_browser())
    }

    pub async fn load_embedded_course_publication(_course_id: &str) -> Result<LoadedCoursePublication, AppError> {
        Err(courses_load_only_in_the_browser())
    }

    pub async fn load_course_shard(_publication: &LoadedCoursePublication) -> Result<CourseShardContents, AppError> {
        Err(courses_load_only_in_the_browser())
    }

    pub async fn load_unit_shard(_publication: &LoadedCoursePublication, _unit_id: &str) -> Result<UnitShardContents, AppError> {
        Err(courses_load_only_in_the_browser())
    }

    fn courses_load_only_in_the_browser() -> AppError {
        AppError::from("courses load only in the browser".to_string())
    }
}
