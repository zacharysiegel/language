use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use shared::artifact::{
    self, ArtifactOriginKind, CourseIndexSource, CourseManifest, CourseManifestSource, FilesystemArtifactCache, ManifestEntry,
    content_address, course_manifest, discovery, fetch, manifest,
};
use shared::course_shard::{self, ClipAudioEntry, CourseShardContents, UnitShardContents, VoiceEngineKind};
use shared::filesystem;
use shared::http::{DirectoryFetch, HttpCacheMode, HttpFetch, ReqwestHttpFetch};
use shared::progress::{AudioEngineKind, VoiceChoiceKind};
use shared::{AppError, AppErrorStatic};

use crate::course_records::{CourseOutlineRecord, CourseSummaryRecord, UnitLessonRecord};
use crate::ffi_error::CoreError;
use crate::progress_book::{AudioEngineRecord, VoiceChoiceRecord};

const SHARD_CACHE_DIRECTORY: &str = "shards";
const AUDIO_CACHE_DIRECTORY: &str = "audio";
const COURSE_CACHE_DIRECTORY: &str = "courses";
const CACHED_COURSE_INDEX_FILE: &str = "course-index.json";
const CACHED_COURSE_INDEX_BASE_URL_FILE: &str = "course-index-base-url";
const CACHED_COURSE_MANIFEST_FILE: &str = "manifest.json";
const CACHED_COURSE_MANIFEST_BASE_URL_FILE: &str = "repository-base-url";
/// Written by the single-manifest cache layout.
const LEGACY_CACHE_FILES: [&str; 2] = ["current-manifest.json", "current-repository-base-url"];

#[derive(Debug, Clone, uniffi::Record)]
pub struct CoreConfiguration {
    /// The origin that serves `/discovery` and the static repository at `/repository`.
    pub app_origin: String,
    pub cache_directory: String,
    pub embedded_bundle_directory: String,
}

/// Where one engine's recording of a clip is fetched from.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ClipRecording {
    audio_sha256: String,
    /// The recordings base of the publication whose shard named this recording.
    audio_base_url: String,
}

/// Opens the course index and each course's publications for a native app.
#[derive(uniffi::Object)]
pub struct CourseLibrary {
    configuration: CoreConfiguration,
    http_fetch: ReqwestHttpFetch,
    /// By clip key, then by speech engine.
    clip_recordings: DashMap<String, BTreeMap<VoiceEngineKind, ClipRecording>>,
    /// Held by every write of the remembered index or a remembered manifest, through the shard retain that follows it.
    cache_update_lock: tokio::sync::Mutex<()>,
}

#[uniffi::export(async_runtime = "tokio")]
impl CourseLibrary {
    #[uniffi::constructor]
    pub fn new(configuration: CoreConfiguration) -> Result<Arc<CourseLibrary>, CoreError> {
        let http_fetch: ReqwestHttpFetch = ReqwestHttpFetch::create()?;
        let library: CourseLibrary = CourseLibrary::create(configuration, http_fetch);

        library.remove_legacy_cache_files();

        Ok(Arc::new(library))
    }

    /// The course index this device last read from a repository; `None` when there is none or it cannot be read.
    pub async fn load_cached_course_index(&self) -> Option<Arc<PublishedCourseIndex>> {
        let loaded: Result<Option<Arc<PublishedCourseIndex>>, AppErrorStatic> = self.open_cached_course_index();

        loaded.unwrap_or_else(|error| {
            log::warn!("could not open the cached course index; [error={error}]");

            None
        })
    }

    /// The repository's course index; remembered, and every course it omits forgotten.
    pub async fn load_latest_course_index(&self) -> Result<Arc<PublishedCourseIndex>, CoreError> {
        Ok(self.open_latest_course_index(&self.http_fetch).await?)
    }

    /// The course index of the bundle included in the app.
    pub async fn load_embedded_course_index(&self) -> Result<Arc<PublishedCourseIndex>, CoreError> {
        Ok(self.open_embedded_course_index().await?)
    }

    /// The publication of `course_id` this device last read from a repository; `None` when there is none or it
    /// cannot be read.
    pub async fn load_cached_course_publication(self: Arc<Self>, course_id: String) -> Option<Arc<CoursePublication>> {
        let loaded: Result<Option<Arc<CoursePublication>>, AppErrorStatic> = self.open_cached_course_publication(&course_id);

        loaded.unwrap_or_else(|error| {
            log::warn!("could not open the cached course publication; [course_id={course_id} error={error}]");

            None
        })
    }

    /// The publication `course_id`'s latest pointer names, read under the repository `course_index` came from; remembered,
    /// and the shard cache trimmed to every remembered publication. Fails for an index from the bundle included in the app.
    pub async fn load_latest_course_publication(
        self: Arc<Self>,
        course_index: Arc<PublishedCourseIndex>,
        course_id: String,
    ) -> Result<Arc<CoursePublication>, CoreError> {
        Ok(self.open_latest_course_publication(&self.http_fetch, &course_index, &course_id).await?)
    }

    /// The publication of `course_id` in the bundle included in the app; its recordings come from the static repository.
    pub async fn load_embedded_course_publication(self: Arc<Self>, course_id: String) -> Result<Arc<CoursePublication>, CoreError> {
        Ok(self.open_embedded_course_publication(&course_id).await?)
    }

    /// A local file holding the clip's recording from the set `engine` picks for `spoken_text` and `voice_choice`, else from
    /// any set, downloaded and verified on first use; `None` when no loaded shard names a recording of the clip.
    pub async fn recording_path(
        &self,
        clip_key: String,
        engine: AudioEngineRecord,
        spoken_text: String,
        voice_choice: VoiceChoiceRecord,
    ) -> Result<Option<String>, CoreError> {
        let voice_choice: VoiceChoiceKind = voice_choice.into();
        let preferred_engine: VoiceEngineKind = AudioEngineKind::from(engine).voice_engine_for_clip(&spoken_text, &voice_choice);

        Ok(self.find_recording(&self.http_fetch, &clip_key, preferred_engine).await?)
    }
}

impl CourseLibrary {
    fn create(configuration: CoreConfiguration, http_fetch: ReqwestHttpFetch) -> CourseLibrary {
        CourseLibrary {
            configuration,
            http_fetch,
            clip_recordings: DashMap::new(),
            cache_update_lock: tokio::sync::Mutex::new(()),
        }
    }

    fn remove_legacy_cache_files(&self) {
        for file_name in LEGACY_CACHE_FILES {
            let path: PathBuf = self.cache_directory().join(file_name);
            let removed: Result<(), std::io::Error> = fs::remove_file(&path);

            match removed {
                Ok(()) => log::info!("removed a file of the single-manifest cache; [path={}]", path.display()),
                Err(error) if error.kind() == ErrorKind::NotFound => {},
                Err(error) => log::warn!("could not remove a file of the single-manifest cache; [path={} error={error}]", path.display()),
            }
        }
    }

    fn cache_directory(&self) -> PathBuf {
        PathBuf::from(&self.configuration.cache_directory)
    }

    fn cached_courses_directory(&self) -> PathBuf {
        self.cache_directory().join(COURSE_CACHE_DIRECTORY)
    }

    /* Course ids are validated as lowercase letters, digits, underscores and dots, so each is a safe directory name. */
    fn cached_course_directory(&self, course_id: &str) -> PathBuf {
        self.cached_courses_directory().join(course_id)
    }

    fn shard_cache(&self) -> FilesystemArtifactCache {
        FilesystemArtifactCache::create(self.cache_directory().join(SHARD_CACHE_DIRECTORY))
    }

    fn static_repository_base_url(&self) -> String {
        fetch::join_url(&self.configuration.app_origin, discovery::STATIC_REPOSITORY_BASE_URL.trim_start_matches('/'))
    }

    fn open_cached_course_index(&self) -> Result<Option<Arc<PublishedCourseIndex>>, AppErrorStatic> {
        let cache_directory: PathBuf = self.cache_directory();
        let cached_files: Option<(Vec<u8>, String)> =
            read_cached_pair(&cache_directory.join(CACHED_COURSE_INDEX_FILE), &cache_directory.join(CACHED_COURSE_INDEX_BASE_URL_FILE))?;
        let Some((course_index_bytes, repository_base_url)) = cached_files
        else {
            return Ok(None);
        };

        let source: CourseIndexSource = artifact::parse_cached_course_index(course_index_bytes, repository_base_url.trim())?;

        Ok(Some(Arc::new(PublishedCourseIndex { source })))
    }

    pub(crate) async fn open_latest_course_index(&self, http_fetch: &impl HttpFetch) -> Result<Arc<PublishedCourseIndex>, AppErrorStatic> {
        let discovery_url: String = fetch::join_url(&self.configuration.app_origin, discovery::DISCOVERY_PATH.trim_start_matches('/'));
        let static_base_url: String = self.static_repository_base_url();
        let source: CourseIndexSource = artifact::discover_course_index(http_fetch, &discovery_url, &static_base_url).await?;

        self.remember_course_index(&source).await?;

        Ok(Arc::new(PublishedCourseIndex { source }))
    }

    async fn open_embedded_course_index(&self) -> Result<Arc<PublishedCourseIndex>, AppErrorStatic> {
        let source: CourseIndexSource = artifact::load_embedded_course_index(&DirectoryFetch, &self.configuration.embedded_bundle_directory).await?;

        Ok(Arc::new(PublishedCourseIndex { source }))
    }

    fn open_cached_course_publication(self: &Arc<Self>, course_id: &str) -> Result<Option<Arc<CoursePublication>>, AppErrorStatic> {
        let course_directory: PathBuf = self.cached_course_directory(course_id);
        let cached_files: Option<(Vec<u8>, String)> = read_cached_pair(
            &course_directory.join(CACHED_COURSE_MANIFEST_FILE),
            &course_directory.join(CACHED_COURSE_MANIFEST_BASE_URL_FILE),
        )?;
        let Some((manifest_bytes, repository_base_url)) = cached_files
        else {
            return Ok(None);
        };

        let source: CourseManifestSource = artifact::parse_cached_course_manifest(manifest_bytes, repository_base_url.trim(), course_id)?;

        Ok(Some(CoursePublication::open(self.clone(), source)))
    }

    pub(crate) async fn open_latest_course_publication(
        self: &Arc<Self>,
        http_fetch: &impl HttpFetch,
        course_index: &PublishedCourseIndex,
        course_id: &str,
    ) -> Result<Arc<CoursePublication>, AppErrorStatic> {
        let source: CourseManifestSource = artifact::fetch_latest_course_manifest(http_fetch, &course_index.source, course_id).await?;

        self.remember_course_publication(&source).await?;

        Ok(CoursePublication::open(self.clone(), source))
    }

    async fn open_embedded_course_publication(self: &Arc<Self>, course_id: &str) -> Result<Arc<CoursePublication>, AppErrorStatic> {
        let source: CourseManifestSource =
            artifact::load_embedded_course_manifest(&DirectoryFetch, &self.configuration.embedded_bundle_directory, course_id).await?;

        Ok(CoursePublication::open(self.clone(), source))
    }

    async fn remember_course_index(&self, source: &CourseIndexSource) -> Result<(), AppErrorStatic> {
        let _cache_update_guard: tokio::sync::MutexGuard<'_, ()> = self.cache_update_lock.lock().await;
        let cache_directory: PathBuf = self.cache_directory();

        create_directory(&cache_directory)?;
        filesystem::write_atomically(&cache_directory.join(CACHED_COURSE_INDEX_FILE), &source.course_index_bytes)?;
        filesystem::write_atomically(&cache_directory.join(CACHED_COURSE_INDEX_BASE_URL_FILE), source.repository_base_url.as_bytes())?;

        let forgotten_course_ids: Vec<String> = source.course_index.course_ids_not_listed(self.remembered_course_ids()?);

        if forgotten_course_ids.is_empty() {
            return Ok(());
        }

        self.forget_courses(&forgotten_course_ids)?;

        self.retain_shards_of_remembered_publications().await
    }

    fn forget_courses(&self, course_ids: &[String]) -> Result<(), AppErrorStatic> {
        for course_id in course_ids {
            let course_directory: PathBuf = self.cached_course_directory(course_id);

            fs::remove_dir_all(&course_directory).map_err(|error| {
                AppErrorStatic::from(format!("forgetting a course failed; [path={} error={error}]", course_directory.display()))
            })?;
            log::info!("forgot a course the course index no longer lists; [course_id={course_id}]");
        }

        Ok(())
    }

    /* A course the remembered index omits stays forgotten, even when a pointer read before that index arrives later. */
    async fn remember_course_publication(&self, source: &CourseManifestSource) -> Result<(), AppErrorStatic> {
        let _cache_update_guard: tokio::sync::MutexGuard<'_, ()> = self.cache_update_lock.lock().await;
        let course_id: &str = &source.manifest.course_id;
        let remembered_index: Option<Arc<PublishedCourseIndex>> = self.open_cached_course_index()?;
        let remembered_index_omits_the_course: bool =
            remembered_index.is_some_and(|course_index| !course_index.source.course_index.courses.contains_key(course_id));

        if remembered_index_omits_the_course {
            log::info!("not remembering a course the remembered course index omits; [course_id={course_id}]");

            return Ok(());
        }

        let course_directory: PathBuf = self.cached_course_directory(course_id);

        create_directory(&course_directory)?;
        filesystem::write_atomically(&course_directory.join(CACHED_COURSE_MANIFEST_FILE), &source.manifest_bytes)?;
        filesystem::write_atomically(&course_directory.join(CACHED_COURSE_MANIFEST_BASE_URL_FILE), source.repository_base_url.as_bytes())?;

        self.retain_shards_of_remembered_publications().await
    }

    /* Callers hold `cache_update_lock`. */
    async fn retain_shards_of_remembered_publications(&self) -> Result<(), AppErrorStatic> {
        let remembered_manifests: Vec<CourseManifest> = self.read_remembered_manifests()?;
        let kept_paths: Vec<String> = artifact::relative_paths_to_keep(&remembered_manifests);

        artifact::ArtifactCache::retain(&self.shard_cache(), &kept_paths).await
    }

    fn read_remembered_manifests(&self) -> Result<Vec<CourseManifest>, AppErrorStatic> {
        let mut remembered_manifests: Vec<CourseManifest> = Vec::new();

        for course_id in self.remembered_course_ids()? {
            let manifest_path: PathBuf = self.cached_course_directory(&course_id).join(CACHED_COURSE_MANIFEST_FILE);
            let parsed: Result<CourseManifest, AppError> = fs::read(&manifest_path)
                .map_err(|error| AppError::from(format!("reading a cached file failed; [path={} error={error}]", manifest_path.display())))
                .and_then(|bytes| course_manifest::parse_course_manifest(&bytes, Some(&course_id)));

            match parsed {
                Ok(course_manifest) => remembered_manifests.push(course_manifest),
                Err(error) => log::warn!("a remembered course manifest is unreadable, so its shards are not kept; [error={error}]"),
            }
        }

        Ok(remembered_manifests)
    }

    fn remembered_course_ids(&self) -> Result<Vec<String>, AppErrorStatic> {
        let courses_directory: PathBuf = self.cached_courses_directory();
        let read_directory: Result<fs::ReadDir, std::io::Error> = fs::read_dir(&courses_directory);
        let entries: fs::ReadDir = match read_directory {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => {
                return Err(AppErrorStatic::from(format!(
                    "listing the remembered courses failed; [path={} error={error}]",
                    courses_directory.display(),
                )));
            },
        };
        let mut course_ids: Vec<String> = Vec::new();

        for entry in entries {
            let entry: fs::DirEntry = entry?;
            let is_directory: bool = entry.file_type()?.is_dir();

            if !is_directory {
                continue;
            }

            course_ids.push(entry.file_name().to_string_lossy().into_owned());
        }

        course_ids.sort();

        Ok(course_ids)
    }

    fn register_clip_audio(&self, entries: &[ClipAudioEntry], audio_base_url: &str) {
        for entry in entries {
            let recording: ClipRecording = ClipRecording {
                audio_sha256: entry.audio_sha256.clone(),
                audio_base_url: audio_base_url.to_string(),
            };

            self.clip_recordings.entry(entry.clip_key.clone()).or_default().insert(entry.engine, recording);
        }
    }

    /* The preferred voice's recording, else any other voice's, so a clip one engine lacks still plays a recording. */
    fn clip_recording_for(&self, clip_key: &str, preferred_engine: VoiceEngineKind) -> Option<ClipRecording> {
        let recordings_by_engine: Ref<'_, String, BTreeMap<VoiceEngineKind, ClipRecording>> = self.clip_recordings.get(clip_key)?;

        recordings_by_engine
            .get(&preferred_engine)
            .or_else(|| recordings_by_engine.values().next())
            .cloned()
    }

    pub(crate) async fn find_recording(
        &self,
        http_fetch: &impl HttpFetch,
        clip_key: &str,
        preferred_engine: VoiceEngineKind,
    ) -> Result<Option<String>, AppErrorStatic> {
        let Some(recording) = self.clip_recording_for(clip_key, preferred_engine)
        else {
            return Ok(None);
        };

        let path: PathBuf = self.cached_recording(http_fetch, &recording).await?;

        Ok(Some(path.display().to_string()))
    }

    async fn cached_recording(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording) -> Result<PathBuf, AppErrorStatic> {
        let audio_directory: PathBuf = self.cache_directory().join(AUDIO_CACHE_DIRECTORY);
        let path: PathBuf = audio_directory.join(format!("{}.{}", recording.audio_sha256, manifest::AUDIO_FILE_EXTENSION));

        if path.exists() {
            return Ok(path);
        }

        let url: String = fetch::join_url(&recording.audio_base_url, &manifest::audio_key(&recording.audio_sha256));
        let bytes: Vec<u8> = fetch::fetch_bytes(http_fetch, &url, HttpCacheMode::Default).await?;

        content_address::verify_sha256(&url, &bytes, &recording.audio_sha256)?;
        create_directory(&audio_directory)?;
        filesystem::write_atomically(&path, &bytes)?;

        Ok(path)
    }
}

/// Both files' contents, or `None` when either is absent.
fn read_cached_pair(bytes_path: &Path, base_url_path: &Path) -> Result<Option<(Vec<u8>, String)>, AppErrorStatic> {
    let bytes: Option<Vec<u8>> = read_cached_file(bytes_path)?;
    let base_url_bytes: Option<Vec<u8>> = read_cached_file(base_url_path)?;
    let (Some(bytes), Some(base_url_bytes)) = (bytes, base_url_bytes)
    else {
        return Ok(None);
    };

    let base_url: String = String::from_utf8(base_url_bytes)
        .map_err(|error| AppErrorStatic::from(format!("a cached base URL is not UTF-8; [path={} error={error}]", base_url_path.display())))?;

    Ok(Some((bytes, base_url)))
}

fn read_cached_file(path: &Path) -> Result<Option<Vec<u8>>, AppErrorStatic> {
    let read: Result<Vec<u8>, std::io::Error> = fs::read(path);

    match read {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(AppErrorStatic::from(format!("reading a cached file failed; [path={} error={error}]", path.display()))),
    }
}

fn create_directory(path: &Path) -> Result<(), AppErrorStatic> {
    fs::create_dir_all(path).map_err(|error| AppErrorStatic::from(format!("creating a cache directory failed; [path={} error={error}]", path.display())))
}

/// A course index, read from a repository, from this device's cache of one, or from the bundle included in the app.
#[derive(uniffi::Object)]
pub struct PublishedCourseIndex {
    source: CourseIndexSource,
}

#[uniffi::export]
impl PublishedCourseIndex {
    pub fn courses(&self) -> Vec<CourseSummaryRecord> {
        self.source
            .course_index
            .courses
            .iter()
            .map(|(course_id, entry)| CourseSummaryRecord::from_index_entry(course_id, entry))
            .collect()
    }

    pub fn is_embedded(&self) -> bool {
        self.source.origin == ArtifactOriginKind::Embedded
    }
}

/// One publication of one course, opened from a repository, from this device's cache of one, or from the bundle
/// included in the app.
#[derive(uniffi::Object)]
pub struct CoursePublication {
    library: Arc<CourseLibrary>,
    source: CourseManifestSource,
    manifest_sha256: String,
    audio_base_url: String,
}

#[uniffi::export(async_runtime = "tokio")]
impl CoursePublication {
    pub fn course_id(&self) -> String {
        self.source.manifest.course_id.clone()
    }

    pub fn publication_label(&self) -> String {
        self.source.manifest.publication_label.clone()
    }

    /// The SHA-256 of the manifest's bytes; equal for two publications only when their course content is the same.
    pub fn manifest_sha256(&self) -> String {
        self.manifest_sha256.clone()
    }

    pub fn repository_base_url(&self) -> String {
        self.source.repository_base_url.clone()
    }

    /// The base the recordings its shards name are fetched under.
    pub fn recordings_base_url(&self) -> String {
        self.audio_base_url.clone()
    }

    pub fn is_embedded(&self) -> bool {
        self.source.origin == ArtifactOriginKind::Embedded
    }

    pub async fn load_course(&self) -> Result<CourseOutlineRecord, CoreError> {
        let outline: CourseOutlineRecord = match self.source.origin {
            ArtifactOriginKind::Repository => self.open_course(&self.library.http_fetch).await?,
            ArtifactOriginKind::Embedded => self.open_course(&DirectoryFetch).await?,
        };

        Ok(outline)
    }

    pub async fn load_unit(&self, unit_id: String) -> Result<UnitLessonRecord, CoreError> {
        let lesson: UnitLessonRecord = match self.source.origin {
            ArtifactOriginKind::Repository => self.open_unit(&self.library.http_fetch, &unit_id).await?,
            ArtifactOriginKind::Embedded => self.open_unit(&DirectoryFetch, &unit_id).await?,
        };

        Ok(lesson)
    }
}

impl CoursePublication {
    fn open(library: Arc<CourseLibrary>, source: CourseManifestSource) -> Arc<CoursePublication> {
        let manifest_sha256: String = content_address::sha256_hex(&source.manifest_bytes);
        let audio_base_url: String = artifact::recordings_base_url(&source, &library.static_repository_base_url());

        Arc::new(CoursePublication {
            library,
            source,
            manifest_sha256,
            audio_base_url,
        })
    }

    pub(crate) async fn open_course(&self, http_fetch: &impl HttpFetch) -> Result<CourseOutlineRecord, AppErrorStatic> {
        let bytes: Vec<u8> = self.load_shard(http_fetch, &self.source.manifest.outline).await?;
        let contents: CourseShardContents = course_shard::read_course_shard(&bytes)?;

        self.library.register_clip_audio(&contents.clip_audio, &self.audio_base_url);

        Ok((&contents.outline).into())
    }

    pub(crate) async fn open_unit(&self, http_fetch: &impl HttpFetch, unit_id: &str) -> Result<UnitLessonRecord, AppErrorStatic> {
        let entry: &ManifestEntry = self.source.manifest.find_unit(unit_id)?;
        let bytes: Vec<u8> = self.load_shard(http_fetch, entry).await?;
        let contents: UnitShardContents = course_shard::read_unit_shard(&bytes)?;

        self.library.register_clip_audio(&contents.clip_audio, &self.audio_base_url);

        Ok((&contents.lesson).into())
    }

    async fn load_shard(&self, http_fetch: &impl HttpFetch, entry: &ManifestEntry) -> Result<Vec<u8>, AppErrorStatic> {
        artifact::load_publication_shard(http_fetch, &self.library.shard_cache(), &self.source, entry).await
    }
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use shared::artifact::{CourseIndex, CourseIndexEntry, course_index, repository_key};

    use super::*;

    const COURSE_A: &str = "pt_br.en.course_a";
    const COURSE_B: &str = "pt_br.en.course_b";

    fn create_library(app_origin: &Path, cache_directory: &Path, embedded_bundle_directory: &Path) -> Arc<CourseLibrary> {
        CourseLibrary::new(CoreConfiguration {
            app_origin: app_origin.display().to_string(),
            cache_directory: cache_directory.display().to_string(),
            embedded_bundle_directory: embedded_bundle_directory.display().to_string(),
        })
        .unwrap()
    }

    fn repository_directory(app_origin: &Path) -> PathBuf {
        app_origin.join(discovery::STATIC_REPOSITORY_BASE_URL.trim_start_matches('/'))
    }

    fn fixture_manifest(course_id: &str, publication_label: &str, outline_sha256: &str) -> CourseManifest {
        CourseManifest {
            manifest_schema_version: course_manifest::COURSE_MANIFEST_SCHEMA_VERSION,
            course_id: course_id.to_string(),
            publication_label: publication_label.to_string(),
            artifact_created: "2026-10-08T14:02:11Z".parse().unwrap(),
            title: format!("Title of {course_id}"),
            target_language: "pt-BR".to_string(),
            source_language: "en".to_string(),
            test_course: false,
            outline: ManifestEntry {
                relative_path: format!("data/course-{outline_sha256}.{}", manifest::SHARD_FILE_EXTENSION),
                size_bytes: 0,
                sha256: outline_sha256.to_string(),
            },
            units: BTreeMap::new(),
            voice_pipeline_revisions: BTreeMap::new(),
        }
    }

    fn write_file(path: &Path, bytes: &[u8]) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    /* Publishes the manifest and moves its course's latest pointer to it. */
    fn publish(repository: &Path, course_manifest: &CourseManifest) {
        let manifest_bytes: Vec<u8> = serde_json::to_vec_pretty(course_manifest).unwrap();
        let course_id: &str = &course_manifest.course_id;

        write_file(
            &repository.join(repository_key::publication_manifest_key(course_id, &course_manifest.publication_label)),
            &manifest_bytes,
        );
        write_file(&repository.join(repository_key::course_latest_manifest_key(course_id)), &manifest_bytes);
    }

    fn write_course_index(repository: &Path, course_ids: &[&str]) {
        let courses: BTreeMap<String, CourseIndexEntry> = course_ids
            .iter()
            .map(|course_id| {
                let entry: CourseIndexEntry = CourseIndexEntry {
                    title: format!("Title of {course_id}"),
                    target_language: "pt-BR".to_string(),
                    source_language: "en".to_string(),
                    latest_manifest_key: repository_key::course_latest_manifest_key(course_id),
                    test_course: false,
                };

                (course_id.to_string(), entry)
            })
            .collect();
        let index: CourseIndex = CourseIndex {
            course_index_schema_version: course_index::COURSE_INDEX_SCHEMA_VERSION,
            courses,
        };

        write_file(&repository.join(repository_key::COURSE_INDEX_KEY), &serde_json::to_vec_pretty(&index).unwrap());
    }

    /* Stands in for a shard a load cached; retention goes by file name only. */
    fn cache_outline_shard(cache_directory: &Path, course_manifest: &CourseManifest) {
        let file_name: &str = course_manifest.outline.relative_path.rsplit('/').next().unwrap();

        write_file(&cache_directory.join(SHARD_CACHE_DIRECTORY).join(file_name), b"shard");
    }

    fn cached_shard_names(cache_directory: &Path) -> BTreeSet<String> {
        fs::read_dir(cache_directory.join(SHARD_CACHE_DIRECTORY))
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect()
    }

    fn outline_shard_name(course_manifest: &CourseManifest) -> String {
        course_manifest.outline.relative_path.rsplit('/').next().unwrap().to_string()
    }

    fn copy_directory(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).unwrap();

        for entry in fs::read_dir(source).unwrap() {
            let entry: fs::DirEntry = entry.unwrap();
            let destination_path: PathBuf = destination.join(entry.file_name());

            if entry.file_type().unwrap().is_dir() {
                copy_directory(&entry.path(), &destination_path);
            } else {
                fs::copy(entry.path(), &destination_path).unwrap();
            }
        }
    }

    fn embedded_bundle_directory() -> Option<PathBuf> {
        let directory: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/static/embedded");

        directory.join(repository_key::COURSE_INDEX_KEY).exists().then_some(directory)
    }

    /* The embedded bundle is build output; without it there is nothing to open. */
    #[tokio::test]
    async fn embedded_course_index_and_publication_open_the_outline_and_a_unit_and_cache_their_shards() {
        let Some(embedded_directory) = embedded_bundle_directory()
        else {
            eprintln!("skipped: run ./scripts/dev/build-local-repository.sh to produce the embedded bundle");
            return;
        };

        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let library: Arc<CourseLibrary> = create_library(Path::new("http://127.0.0.1:9"), cache_directory.path(), &embedded_directory);

        let course_index: Arc<PublishedCourseIndex> = library.load_embedded_course_index().await.unwrap();
        let course_id: String = course_index.courses()[0].id.clone();
        let publication: Arc<CoursePublication> = library.clone().load_embedded_course_publication(course_id.clone()).await.unwrap();
        let outline: CourseOutlineRecord = publication.load_course().await.unwrap();
        let lesson: UnitLessonRecord = publication.load_unit(outline.units[0].id.clone()).await.unwrap();
        let cached_shard_count: usize = cached_shard_names(cache_directory.path()).len();

        assert!(course_index.is_embedded());
        assert!(publication.is_embedded());
        assert_eq!(publication.course_id(), course_id);
        assert!(!outline.units.is_empty());
        assert!(!lesson.exercises.is_empty());
        assert_eq!(cached_shard_count, 2);
        assert!(library.load_cached_course_index().await.is_none());
        assert!(library.clone().load_cached_course_publication(course_id).await.is_none());
    }

    #[tokio::test]
    async fn latest_publication_loads_its_shards_from_the_repository_and_is_remembered() {
        let Some(embedded_directory) = embedded_bundle_directory()
        else {
            eprintln!("skipped: run ./scripts/dev/build-local-repository.sh to produce the embedded bundle");
            return;
        };

        let app_origin: tempfile::TempDir = tempfile::tempdir().unwrap();
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        copy_directory(&embedded_directory, &repository_directory(app_origin.path()));
        let library: Arc<CourseLibrary> = create_library(app_origin.path(), cache_directory.path(), cache_directory.path());

        let course_index: Arc<PublishedCourseIndex> = library.open_latest_course_index(&DirectoryFetch).await.unwrap();
        let course_id: String = course_index.courses()[0].id.clone();
        let publication: Arc<CoursePublication> =
            library.open_latest_course_publication(&DirectoryFetch, &course_index, &course_id).await.unwrap();
        let outline: CourseOutlineRecord = publication.open_course(&DirectoryFetch).await.unwrap();
        let cached_index: Arc<PublishedCourseIndex> = library.load_cached_course_index().await.unwrap();
        let cached_publication: Arc<CoursePublication> = library.clone().load_cached_course_publication(course_id).await.unwrap();

        assert!(!course_index.is_embedded());
        assert!(!publication.is_embedded());
        assert!(!outline.units.is_empty());
        assert_eq!(cached_index.courses().len(), course_index.courses().len());
        assert_eq!(cached_publication.manifest_sha256(), publication.manifest_sha256());
        assert_eq!(cached_publication.repository_base_url(), repository_directory(app_origin.path()).display().to_string());
        assert_eq!(cached_shard_names(cache_directory.path()).len(), 1);
    }

    #[tokio::test]
    async fn refreshing_one_course_keeps_another_courses_remembered_manifest_and_shards() {
        let app_origin: tempfile::TempDir = tempfile::tempdir().unwrap();
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: PathBuf = repository_directory(app_origin.path());
        let course_a_first: CourseManifest = fixture_manifest(COURSE_A, "2026-10-08+duluth", &"a1".repeat(32));
        let course_a_second: CourseManifest = fixture_manifest(COURSE_A, "2026-10-09+fargo", &"a2".repeat(32));
        let course_b: CourseManifest = fixture_manifest(COURSE_B, "2026-10-08+duluth", &"b1".repeat(32));
        write_course_index(&repository, &[COURSE_A, COURSE_B]);
        publish(&repository, &course_a_first);
        publish(&repository, &course_b);
        let library: Arc<CourseLibrary> = create_library(app_origin.path(), cache_directory.path(), cache_directory.path());

        let course_index: Arc<PublishedCourseIndex> = library.open_latest_course_index(&DirectoryFetch).await.unwrap();
        library.open_latest_course_publication(&DirectoryFetch, &course_index, COURSE_A).await.unwrap();
        let course_b_publication: Arc<CoursePublication> =
            library.open_latest_course_publication(&DirectoryFetch, &course_index, COURSE_B).await.unwrap();
        cache_outline_shard(cache_directory.path(), &course_a_first);
        cache_outline_shard(cache_directory.path(), &course_b);
        publish(&repository, &course_a_second);
        cache_outline_shard(cache_directory.path(), &course_a_second);

        let course_a_publication: Arc<CoursePublication> =
            library.open_latest_course_publication(&DirectoryFetch, &course_index, COURSE_A).await.unwrap();
        let cached_course_b: Arc<CoursePublication> = library.clone().load_cached_course_publication(COURSE_B.to_string()).await.unwrap();

        assert_eq!(course_a_publication.publication_label(), "2026-10-09+fargo");
        assert_eq!(cached_course_b.manifest_sha256(), course_b_publication.manifest_sha256());
        assert_eq!(
            cached_shard_names(cache_directory.path()),
            BTreeSet::from([outline_shard_name(&course_a_second), outline_shard_name(&course_b)]),
        );
    }

    #[tokio::test]
    async fn latest_course_index_forgets_a_course_it_omits_and_its_shards() {
        let app_origin: tempfile::TempDir = tempfile::tempdir().unwrap();
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: PathBuf = repository_directory(app_origin.path());
        let course_a: CourseManifest = fixture_manifest(COURSE_A, "2026-10-08+duluth", &"a1".repeat(32));
        let course_b: CourseManifest = fixture_manifest(COURSE_B, "2026-10-08+duluth", &"b1".repeat(32));
        write_course_index(&repository, &[COURSE_A, COURSE_B]);
        publish(&repository, &course_a);
        publish(&repository, &course_b);
        let library: Arc<CourseLibrary> = create_library(app_origin.path(), cache_directory.path(), cache_directory.path());
        let first_index: Arc<PublishedCourseIndex> = library.open_latest_course_index(&DirectoryFetch).await.unwrap();
        library.open_latest_course_publication(&DirectoryFetch, &first_index, COURSE_A).await.unwrap();
        library.open_latest_course_publication(&DirectoryFetch, &first_index, COURSE_B).await.unwrap();
        cache_outline_shard(cache_directory.path(), &course_a);
        cache_outline_shard(cache_directory.path(), &course_b);
        write_course_index(&repository, &[COURSE_A]);

        let second_index: Arc<PublishedCourseIndex> = library.open_latest_course_index(&DirectoryFetch).await.unwrap();
        let late_course_b: Result<Arc<CoursePublication>, AppErrorStatic> =
            library.open_latest_course_publication(&DirectoryFetch, &first_index, COURSE_B).await;

        assert_eq!(second_index.courses().len(), 1);
        assert!(!library.cached_course_directory(COURSE_B).exists());
        assert!(library.clone().load_cached_course_publication(COURSE_B.to_string()).await.is_none());
        assert!(library.clone().load_cached_course_publication(COURSE_A.to_string()).await.is_some());
        assert!(late_course_b.is_ok());
        assert!(!library.cached_course_directory(COURSE_B).exists());
        assert_eq!(cached_shard_names(cache_directory.path()), BTreeSet::from([outline_shard_name(&course_a)]));
    }

    #[tokio::test]
    async fn latest_course_publication_refuses_an_embedded_index() {
        let embedded_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        write_course_index(embedded_directory.path(), &[COURSE_A]);
        let library: Arc<CourseLibrary> = create_library(embedded_directory.path(), cache_directory.path(), embedded_directory.path());
        let embedded_index: Arc<PublishedCourseIndex> = library.load_embedded_course_index().await.unwrap();

        let loaded: Result<Arc<CoursePublication>, AppErrorStatic> =
            library.open_latest_course_publication(&DirectoryFetch, &embedded_index, COURSE_A).await;

        assert!(loaded.is_err());
    }

    #[tokio::test]
    async fn new_deletes_the_single_manifest_cache_files() {
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        for file_name in LEGACY_CACHE_FILES {
            write_file(&cache_directory.path().join(file_name), b"legacy");
        }

        create_library(Path::new("http://127.0.0.1:9"), cache_directory.path(), cache_directory.path());

        for file_name in LEGACY_CACHE_FILES {
            assert!(!cache_directory.path().join(file_name).exists());
        }
    }

    #[tokio::test]
    async fn find_recording_fetches_a_registered_clip_from_its_publications_base_and_falls_back_to_another_engine() {
        let app_origin: tempfile::TempDir = tempfile::tempdir().unwrap();
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: PathBuf = repository_directory(app_origin.path());
        let recording_bytes: &[u8] = b"recording";
        let audio_sha256: String = content_address::sha256_hex(recording_bytes);
        write_file(&repository.join(manifest::audio_key(&audio_sha256)), recording_bytes);
        let library: Arc<CourseLibrary> = create_library(app_origin.path(), cache_directory.path(), cache_directory.path());
        library.register_clip_audio(
            &[ClipAudioEntry {
                clip_key: "clip".to_string(),
                engine: VoiceEngineKind::Kokoro,
                audio_sha256: audio_sha256.clone(),
                audio_size_bytes: 9,
            }],
            &repository.display().to_string(),
        );

        let found: Option<String> = library.find_recording(&DirectoryFetch, "clip", VoiceEngineKind::MacosSay).await.unwrap();
        let unregistered: Option<String> = library.find_recording(&DirectoryFetch, "other", VoiceEngineKind::Kokoro).await.unwrap();

        assert_eq!(fs::read(found.unwrap()).unwrap(), recording_bytes.to_vec());
        assert_eq!(unregistered, None);
    }

    #[tokio::test]
    async fn load_latest_course_index_names_the_unreachable_repository() {
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let library: Arc<CourseLibrary> = create_library(Path::new("http://127.0.0.1:9"), cache_directory.path(), cache_directory.path());

        let loaded: Result<Arc<PublishedCourseIndex>, CoreError> = library.load_latest_course_index().await;
        let Err(CoreError::Failure { message, .. }) = loaded
        else {
            panic!("an unreachable repository loaded");
        };

        assert!(message.contains("http://127.0.0.1:9/repository/course-index.json"), "{message}");
    }
}
