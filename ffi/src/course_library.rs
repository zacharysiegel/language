use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use shared::AppErrorStatic;
use shared::artifact::{self, FilesystemArtifactCache, ManifestEntry, ManifestSource, content_address, discovery, fetch, manifest};
use shared::course;
use shared::course_shard::{self, ClipAudioEntry, CourseShardContents, UnitShardContents, VoiceEngineKind};
use shared::filesystem;
use shared::http::{DirectoryFetch, HttpCacheMode, ReqwestHttpFetch};

use crate::course_records::{CourseOutlineRecord, CourseSummaryRecord, UnitLessonRecord};
use crate::ffi_error::CoreError;
use crate::progress_book::AudioEngineRecord;

const SHARD_CACHE_DIRECTORY: &str = "shards";
const AUDIO_CACHE_DIRECTORY: &str = "audio";
const CURRENT_MANIFEST_FILE: &str = "current-manifest.json";
const CURRENT_REPOSITORY_BASE_URL_FILE: &str = "current-repository-base-url";

#[derive(Debug, Clone, uniffi::Record)]
pub struct CoreConfiguration {
    /// The origin that serves `/discovery` and the static repository at `/repository`.
    pub app_origin: String,
    pub cache_directory: String,
    pub embedded_bundle_directory: String,
}

/// Opens published versions of the courses for a native app.
#[derive(uniffi::Object)]
pub struct CourseLibrary {
    configuration: CoreConfiguration,
    http_fetch: Arc<ReqwestHttpFetch>,
}

#[uniffi::export(async_runtime = "tokio")]
impl CourseLibrary {
    #[uniffi::constructor]
    pub fn new(configuration: CoreConfiguration) -> Result<Arc<CourseLibrary>, CoreError> {
        let http_fetch: ReqwestHttpFetch = ReqwestHttpFetch::create()?;

        Ok(Arc::new(CourseLibrary {
            configuration,
            http_fetch: Arc::new(http_fetch),
        }))
    }

    /// The version this device last loaded from a repository, read from its cache; `None` when there is none.
    pub async fn load_cached_course(&self) -> Option<Arc<CourseVersion>> {
        let loaded: Result<Option<Arc<CourseVersion>>, AppErrorStatic> = self.open_cached_course().await;

        loaded.unwrap_or_else(|error| {
            log::warn!("could not open the cached course; [error={error}]");

            None
        })
    }

    /// The version the repository publishes now; remembered, and the cache trimmed to it.
    pub async fn load_latest_course(&self) -> Result<Arc<CourseVersion>, CoreError> {
        Ok(self.open_latest_course().await?)
    }

    /// The bundle shipped inside the app; its recordings come from the static repository.
    pub async fn load_embedded_course(&self) -> Result<Arc<CourseVersion>, CoreError> {
        Ok(self.open_embedded_course().await?)
    }
}

impl CourseLibrary {
    async fn open_latest_course(&self) -> Result<Arc<CourseVersion>, AppErrorStatic> {
        let discovery_url: String = fetch::join_url(&self.configuration.app_origin, discovery::DISCOVERY_PATH.trim_start_matches('/'));
        let static_base_url: String = self.static_repository_base_url();
        let source: ManifestSource = artifact::discover_latest_manifest(self.http_fetch.as_ref(), &discovery_url, &static_base_url).await?;
        let audio_base_url: String = source.repository_base_url.clone();
        let version: Arc<CourseVersion> = self.open_version(source, audio_base_url, ShardOriginKind::Repository).await?;

        self.remember_version(&version.source).await?;

        Ok(version)
    }

    async fn open_embedded_course(&self) -> Result<Arc<CourseVersion>, AppErrorStatic> {
        let source: ManifestSource = artifact::load_embedded_manifest(&DirectoryFetch, &self.configuration.embedded_bundle_directory).await?;
        let audio_base_url: String = self.static_repository_base_url();

        self.open_version(source, audio_base_url, ShardOriginKind::Embedded).await
    }

    fn static_repository_base_url(&self) -> String {
        fetch::join_url(&self.configuration.app_origin, discovery::STATIC_REPOSITORY_BASE_URL.trim_start_matches('/'))
    }

    fn cache_directory(&self) -> PathBuf {
        PathBuf::from(&self.configuration.cache_directory)
    }

    async fn open_cached_course(&self) -> Result<Option<Arc<CourseVersion>>, AppErrorStatic> {
        let manifest_path: PathBuf = self.cache_directory().join(CURRENT_MANIFEST_FILE);
        let base_url_path: PathBuf = self.cache_directory().join(CURRENT_REPOSITORY_BASE_URL_FILE);
        let (Ok(manifest_bytes), Ok(repository_base_url)) = (fs::read(&manifest_path), fs::read_to_string(&base_url_path))
        else {
            return Ok(None);
        };

        let source: ManifestSource = artifact::parse_cached_manifest(manifest_bytes, repository_base_url.trim())?;
        let audio_base_url: String = source.repository_base_url.clone();
        let version: Arc<CourseVersion> = self.open_version(source, audio_base_url, ShardOriginKind::Repository).await?;

        Ok(Some(version))
    }

    async fn open_version(&self, source: ManifestSource, audio_base_url: String, origin: ShardOriginKind) -> Result<Arc<CourseVersion>, AppErrorStatic> {
        let shard_loader: ShardLoader = ShardLoader {
            origin,
            cache_directory: self.cache_directory(),
            http_fetch: self.http_fetch.clone(),
        };

        Ok(Arc::new(CourseVersion {
            source,
            audio_base_url,
            shard_loader,
            audio_sha256_by_clip: Mutex::new(BTreeMap::new()),
        }))
    }

    async fn remember_version(&self, source: &ManifestSource) -> Result<(), AppErrorStatic> {
        let cache_directory: PathBuf = self.cache_directory();

        fs::create_dir_all(&cache_directory)?;
        filesystem::write_atomically(&cache_directory.join(CURRENT_MANIFEST_FILE), &source.manifest_bytes)?;
        filesystem::write_atomically(&cache_directory.join(CURRENT_REPOSITORY_BASE_URL_FILE), source.repository_base_url.as_bytes())?;

        let kept_paths: Vec<String> = source.manifest.file_entries().map(|entry| entry.relative_path.clone()).collect();
        let shard_cache: FilesystemArtifactCache = FilesystemArtifactCache::create(cache_directory.join(SHARD_CACHE_DIRECTORY));

        artifact::ArtifactCache::retain(&shard_cache, &kept_paths).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShardOriginKind {
    Repository,
    Embedded,
}

/// Reads a version's shards through the cache, from wherever the version came from.
struct ShardLoader {
    origin: ShardOriginKind,
    cache_directory: PathBuf,
    http_fetch: Arc<ReqwestHttpFetch>,
}

impl ShardLoader {
    async fn load_shard_bytes(&self, source: &ManifestSource, entry: &ManifestEntry) -> Result<Vec<u8>, AppErrorStatic> {
        let shard_cache: FilesystemArtifactCache = FilesystemArtifactCache::create(self.cache_directory.join(SHARD_CACHE_DIRECTORY));

        match self.origin {
            ShardOriginKind::Repository => artifact::load_shard(self.http_fetch.as_ref(), &shard_cache, source, entry).await,
            ShardOriginKind::Embedded => artifact::load_shard(&DirectoryFetch, &shard_cache, source, entry).await,
        }
    }
}

/// One published version of the courses, opened from a repository or from the bundle shipped with the app.
#[derive(uniffi::Object)]
pub struct CourseVersion {
    source: ManifestSource,
    audio_base_url: String,
    shard_loader: ShardLoader,
    audio_sha256_by_clip: Mutex<BTreeMap<String, BTreeMap<VoiceEngineKind, String>>>,
}

#[uniffi::export(async_runtime = "tokio")]
impl CourseVersion {
    pub fn version(&self) -> String {
        self.source.manifest.version.clone()
    }

    pub fn repository_base_url(&self) -> String {
        self.source.repository_base_url.clone()
    }

    pub fn is_embedded(&self) -> bool {
        self.shard_loader.origin == ShardOriginKind::Embedded
    }

    pub fn courses(&self) -> Vec<CourseSummaryRecord> {
        self.source
            .manifest
            .courses
            .iter()
            .map(|(course_id, manifest_course)| CourseSummaryRecord {
                id: course_id.clone(),
                title: manifest_course.title.clone(),
                target_language_name: course::language_name(&manifest_course.target_language),
                source_language_name: course::language_name(&manifest_course.source_language),
            })
            .collect()
    }

    pub async fn load_course(&self, course_id: String) -> Result<CourseOutlineRecord, CoreError> {
        Ok(self.open_course(&course_id).await?)
    }

    pub async fn load_unit(&self, course_id: String, unit_id: String) -> Result<UnitLessonRecord, CoreError> {
        Ok(self.open_unit(&course_id, &unit_id).await?)
    }

    /// A local file holding the clip's recording in the voice `engine` picks for `spoken_text`, else in any voice,
    /// downloaded and verified on first use; `None` when the clip has no recording.
    pub async fn recording_path(&self, clip_key: String, engine: AudioEngineRecord, spoken_text: String) -> Result<Option<String>, CoreError> {
        Ok(self.find_recording(&clip_key, engine, &spoken_text).await?)
    }
}

impl CourseVersion {
    async fn open_course(&self, course_id: &str) -> Result<CourseOutlineRecord, AppErrorStatic> {
        let entry: &ManifestEntry = &self.source.manifest.find_course(course_id)?.outline;
        let bytes: Vec<u8> = self.shard_loader.load_shard_bytes(&self.source, entry).await?;
        let contents: CourseShardContents = course_shard::read_course_shard(&bytes)?;

        self.register_clip_audio(&contents.clip_audio);

        Ok((&contents.outline).into())
    }

    async fn open_unit(&self, course_id: &str, unit_id: &str) -> Result<UnitLessonRecord, AppErrorStatic> {
        let entry: &ManifestEntry = self.source.manifest.find_course(course_id)?.find_unit(unit_id)?;
        let bytes: Vec<u8> = self.shard_loader.load_shard_bytes(&self.source, entry).await?;
        let contents: UnitShardContents = course_shard::read_unit_shard(&bytes)?;

        self.register_clip_audio(&contents.clip_audio);

        Ok((&contents.lesson).into())
    }

    async fn find_recording(&self, clip_key: &str, engine: AudioEngineRecord, spoken_text: &str) -> Result<Option<String>, AppErrorStatic> {
        let preferred_engine: VoiceEngineKind = shared::progress::AudioEngineKind::from(engine).voice_engine_for(spoken_text);
        let Some(audio_sha256) = self.audio_sha256_for(clip_key, preferred_engine)
        else {
            return Ok(None);
        };

        let path: PathBuf = self.cached_recording(&audio_sha256).await?;

        Ok(Some(path.display().to_string()))
    }

    fn register_clip_audio(&self, entries: &[ClipAudioEntry]) {
        let mut audio_sha256_by_clip = self.audio_sha256_by_clip.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        for entry in entries {
            audio_sha256_by_clip
                .entry(entry.clip_key.clone())
                .or_default()
                .insert(entry.engine, entry.audio_sha256.clone());
        }
    }

    /* The preferred voice's recording, else any other voice's, so a clip one engine lacks still plays a recording. */
    fn audio_sha256_for(&self, clip_key: &str, preferred_engine: VoiceEngineKind) -> Option<String> {
        let audio_sha256_by_clip = self.audio_sha256_by_clip.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let audio_sha256_by_engine: &BTreeMap<VoiceEngineKind, String> = audio_sha256_by_clip.get(clip_key)?;

        audio_sha256_by_engine
            .get(&preferred_engine)
            .or_else(|| audio_sha256_by_engine.values().next())
            .cloned()
    }

    async fn cached_recording(&self, audio_sha256: &str) -> Result<PathBuf, AppErrorStatic> {
        let audio_directory: PathBuf = self.shard_loader.cache_directory.join(AUDIO_CACHE_DIRECTORY);
        let path: PathBuf = audio_directory.join(format!("{audio_sha256}.{}", manifest::AUDIO_FILE_EXTENSION));

        if path.exists() {
            return Ok(path);
        }

        let url: String = fetch::join_url(&self.audio_base_url, &manifest::audio_key(audio_sha256));
        let bytes: Vec<u8> = fetch::fetch_bytes(self.shard_loader.http_fetch.as_ref(), &url, HttpCacheMode::Default).await?;

        content_address::verify_sha256(&url, &bytes, audio_sha256)?;
        fs::create_dir_all(&audio_directory)?;
        filesystem::write_atomically(&path, &bytes)?;

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn embedded_bundle_directory() -> Option<PathBuf> {
        let directory: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/static/embedded");

        directory.join(manifest::MANIFEST_FILENAME).exists().then_some(directory)
    }

    /* The embedded bundle is build output; without it there is nothing to open. */
    #[tokio::test]
    async fn load_embedded_course_opens_the_outline_and_a_unit_and_caches_their_shards() {
        let Some(embedded_directory) = embedded_bundle_directory()
        else {
            eprintln!("skipped: run `course_builder build` to produce the embedded bundle");
            return;
        };

        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let library: Arc<CourseLibrary> = CourseLibrary::new(CoreConfiguration {
            app_origin: "http://127.0.0.1:9".to_string(),
            cache_directory: cache_directory.path().display().to_string(),
            embedded_bundle_directory: embedded_directory.display().to_string(),
        })
        .unwrap();

        let version: Arc<CourseVersion> = library.clone().load_embedded_course().await.unwrap();
        let course_id: String = version.courses()[0].id.clone();
        let outline: CourseOutlineRecord = version.clone().load_course(course_id.clone()).await.unwrap();
        let first_unit_id: String = outline.units[0].id.clone();
        let lesson: UnitLessonRecord = version.clone().load_unit(course_id, first_unit_id).await.unwrap();
        let cached_shard_count: usize = fs::read_dir(cache_directory.path().join(SHARD_CACHE_DIRECTORY)).unwrap().count();

        assert!(version.is_embedded());
        assert!(!outline.units.is_empty());
        assert!(!lesson.exercises.is_empty());
        assert_eq!(cached_shard_count, 2);
        assert!(library.load_cached_course().await.is_none());
    }

    #[tokio::test]
    async fn load_latest_course_reports_an_unreachable_repository() {
        let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let library: Arc<CourseLibrary> = CourseLibrary::new(CoreConfiguration {
            app_origin: "http://127.0.0.1:9".to_string(),
            cache_directory: cache_directory.path().display().to_string(),
            embedded_bundle_directory: cache_directory.path().display().to_string(),
        })
        .unwrap();

        assert!(library.load_latest_course().await.is_err());
    }
}
