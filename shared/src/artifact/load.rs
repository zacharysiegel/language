use std::collections::BTreeSet;

use futures_util::future;

use crate::{AppError, AppErrorStatic};
use crate::artifact::cache::ArtifactCache;
use crate::artifact::course_index::{self, CourseIndex, CourseIndexEntry};
use crate::artifact::course_manifest::{self, CourseManifest};
use crate::artifact::discovery::{self, AuthoritativeBase, DiscoveryDocument};
use crate::artifact::manifest::{self, Manifest, ManifestEntry};
use crate::artifact::{compression, content_address, fetch};
use crate::http::{HttpCacheMode, HttpFetch};

/// A parsed manifest and where it came from, so its shards are fetched from the same repository.
#[derive(Debug, Clone)]
pub struct ManifestSource {
    pub repository_base_url: String,
    pub manifest: Manifest,
    pub manifest_bytes: Vec<u8>,
    pub sunset: Option<String>,
}

/// Fetches the latest manifest from the repository discovery names. The manifest is requested from
/// `static_base_url` in parallel with discovery, and that request is reused when discovery agrees.
pub async fn discover_latest_manifest(
    http_fetch: &impl HttpFetch,
    discovery_url: &str,
    static_base_url: &str,
) -> Result<ManifestSource, AppErrorStatic> {
    let (discovery_bytes, speculative_manifest_bytes): (Result<Vec<u8>, AppErrorStatic>, Result<Vec<u8>, AppErrorStatic>) = future::join(
        fetch::fetch_bytes(http_fetch, discovery_url, HttpCacheMode::Reload),
        fetch::fetch_latest_manifest(http_fetch, static_base_url),
    )
    .await;

    let discovery_document: Option<DiscoveryDocument> = read_discovery_document(discovery_url, discovery_bytes);
    let sunset: Option<String> = read_client_support(discovery_document.as_ref());
    let authoritative_base: AuthoritativeBase = discovery::authoritative_repository_base(static_base_url, discovery_document.as_ref());

    let (repository_base_url, manifest_bytes): (String, Vec<u8>) = match authoritative_base {
        AuthoritativeBase::Static => (static_base_url.to_string(), speculative_manifest_bytes?),
        AuthoritativeBase::Discovered(discovered_base_url) => {
            let bytes: Vec<u8> = fetch::fetch_latest_manifest(http_fetch, &discovered_base_url).await?;

            (discovered_base_url, bytes)
        },
    };

    let manifest: Manifest = manifest::parse_manifest(&manifest_bytes)?;

    Ok(ManifestSource {
        repository_base_url,
        manifest,
        manifest_bytes,
        sunset,
    })
}

/// The discovery document with its repository address made absolute; `None`, logged, when it cannot be read.
fn read_discovery_document(discovery_url: &str, discovery_bytes: Result<Vec<u8>, AppErrorStatic>) -> Option<DiscoveryDocument> {
    let parsed: Result<DiscoveryDocument, AppError> = discovery_bytes
        .map_err(AppError::from)
        .and_then(|bytes| discovery::parse_discovery_document(&bytes));

    match parsed {
        Ok(document) => Some(DiscoveryDocument {
            schema_version: document.schema_version,
            repository_base_url: fetch::resolve_reference(discovery_url, &document.repository_base_url),
            minimum_client_version: document.minimum_client_version,
            sunset: document.sunset,
        }),
        Err(error) => {
            log::warn!("discovery unavailable, using the static repository; [error={error}]");
            None
        },
    }
}

/// The contract's sunset date; a client below `minimum_client_version` is logged and still served.
fn read_client_support(discovery_document: Option<&DiscoveryDocument>) -> Option<String> {
    let Some(document) = discovery_document
    else {
        return None;
    };

    if !discovery::client_version_is_supported(discovery::CLIENT_VERSION, &document.minimum_client_version) {
        log::warn!(
            "this client is older than the repository supports; [client_version={} minimum_client_version={}]",
            discovery::CLIENT_VERSION,
            document.minimum_client_version,
        );
    }

    document.sunset.clone()
}

/// The manifest of the bundle shipped with the app, read when no repository can be reached.
pub async fn load_embedded_manifest(http_fetch: &impl HttpFetch, embedded_base_url: &str) -> Result<ManifestSource, AppErrorStatic> {
    let manifest_bytes: Vec<u8> = fetch::fetch_embedded_manifest(http_fetch, embedded_base_url).await?;
    let manifest: Manifest = manifest::parse_manifest(&manifest_bytes)?;

    Ok(ManifestSource {
        repository_base_url: embedded_base_url.to_string(),
        manifest,
        manifest_bytes,
        sunset: None,
    })
}

pub fn parse_cached_manifest(manifest_bytes: Vec<u8>, repository_base_url: &str) -> Result<ManifestSource, AppError> {
    let manifest: Manifest = manifest::parse_manifest(&manifest_bytes)?;

    Ok(ManifestSource {
        repository_base_url: repository_base_url.to_string(),
        manifest,
        manifest_bytes,
        sunset: None,
    })
}

/// A shard's decompressed bytes, ready to open as a database.
pub async fn load_shard(
    http_fetch: &impl HttpFetch,
    artifact_cache: &impl ArtifactCache,
    source: &ManifestSource,
    entry: &ManifestEntry,
) -> Result<Vec<u8>, AppErrorStatic> {
    let compressed_bytes: Vec<u8> = load_shard_bytes(http_fetch, artifact_cache, source, entry).await?;

    Ok(compression::decompress(&compressed_bytes)?)
}

/// A shard's verified bytes, as published, from the cache, else fetched from the repository and cached. A cached copy whose
/// hash no longer matches is refetched; a failure to write the cache does not fail the load.
pub async fn load_shard_bytes(
    http_fetch: &impl HttpFetch,
    artifact_cache: &impl ArtifactCache,
    source: &ManifestSource,
    entry: &ManifestEntry,
) -> Result<Vec<u8>, AppErrorStatic> {
    let cached_bytes: Option<Vec<u8>> = read_verified_cache_entry(artifact_cache, entry).await;

    if let Some(bytes) = cached_bytes {
        return Ok(bytes);
    }

    let bytes: Vec<u8> =
        fetch::fetch_versioned_file(http_fetch, &source.repository_base_url, &source.manifest.version, &entry.relative_path).await?;

    content_address::verify_sha256(&entry.relative_path, &bytes, &entry.sha256)?;

    let cache_write: Result<(), AppErrorStatic> = artifact_cache.put(&entry.relative_path, &bytes).await;

    if let Err(error) = cache_write {
        log::warn!("could not cache a shard; [relative_path={} error={error}]", entry.relative_path);
    }

    Ok(bytes)
}

async fn read_verified_cache_entry(artifact_cache: &impl ArtifactCache, entry: &ManifestEntry) -> Option<Vec<u8>> {
    let cache_read: Result<Option<Vec<u8>>, AppErrorStatic> = artifact_cache.get(&entry.relative_path).await;
    let cached_bytes: Option<Vec<u8>> = match cache_read {
        Ok(bytes) => bytes,
        Err(error) => {
            log::warn!("could not read the cache; [relative_path={} error={error}]", entry.relative_path);

            None
        },
    };
    let bytes: Vec<u8> = cached_bytes?;
    let verified: Result<(), AppError> = content_address::verify_sha256(&entry.relative_path, &bytes, &entry.sha256);

    if let Err(error) = verified {
        log::warn!("discarding a corrupt cached shard; [error={error}]");

        return None;
    }

    Some(bytes)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactOriginKind {
    Repository,
    /// The bundle included in the app.
    Embedded,
}

/// A parsed course index and the base its course pointers are read under.
#[derive(Debug, Clone)]
pub struct CourseIndexSource {
    pub origin: ArtifactOriginKind,
    pub repository_base_url: String,
    pub course_index: CourseIndex,
    pub course_index_bytes: Vec<u8>,
    pub sunset: Option<String>,
}

/// A parsed course manifest and the base its publication's files are fetched from.
#[derive(Debug, Clone)]
pub struct CourseManifestSource {
    pub origin: ArtifactOriginKind,
    pub repository_base_url: String,
    pub manifest: CourseManifest,
    pub manifest_bytes: Vec<u8>,
}

/// Fetches the course index of the repository discovery names. The index is requested from `static_base_url` in
/// parallel with discovery, and that request is reused when discovery agrees.
pub async fn discover_course_index(
    http_fetch: &impl HttpFetch,
    discovery_url: &str,
    static_base_url: &str,
) -> Result<CourseIndexSource, AppErrorStatic> {
    let (discovery_bytes, speculative_index_bytes): (Result<Vec<u8>, AppErrorStatic>, Result<Vec<u8>, AppErrorStatic>) = future::join(
        fetch::fetch_bytes(http_fetch, discovery_url, HttpCacheMode::Reload),
        fetch::fetch_course_index(http_fetch, static_base_url),
    )
    .await;

    let discovery_document: Option<DiscoveryDocument> = read_discovery_document(discovery_url, discovery_bytes);
    let sunset: Option<String> = read_client_support(discovery_document.as_ref());
    let authoritative_base: AuthoritativeBase = discovery::authoritative_repository_base(static_base_url, discovery_document.as_ref());

    let (repository_base_url, course_index_bytes): (String, Vec<u8>) = match authoritative_base {
        AuthoritativeBase::Static => (static_base_url.to_string(), speculative_index_bytes?),
        AuthoritativeBase::Discovered(discovered_base_url) => {
            let bytes: Vec<u8> = fetch::fetch_course_index(http_fetch, &discovered_base_url).await?;

            (discovered_base_url, bytes)
        },
    };

    let course_index: CourseIndex = course_index::parse_course_index(&course_index_bytes)?;

    Ok(CourseIndexSource {
        origin: ArtifactOriginKind::Repository,
        repository_base_url,
        course_index,
        course_index_bytes,
        sunset,
    })
}

pub async fn load_embedded_course_index(http_fetch: &impl HttpFetch, embedded_base_url: &str) -> Result<CourseIndexSource, AppErrorStatic> {
    let course_index_bytes: Vec<u8> = fetch::fetch_course_index(http_fetch, embedded_base_url).await?;
    let course_index: CourseIndex = course_index::parse_course_index(&course_index_bytes)?;

    Ok(CourseIndexSource {
        origin: ArtifactOriginKind::Embedded,
        repository_base_url: embedded_base_url.to_string(),
        course_index,
        course_index_bytes,
        sunset: None,
    })
}

/// `course_index_bytes` were read from the repository at `repository_base_url`.
pub fn parse_cached_course_index(course_index_bytes: Vec<u8>, repository_base_url: &str) -> Result<CourseIndexSource, AppError> {
    let course_index: CourseIndex = course_index::parse_course_index(&course_index_bytes)?;

    Ok(CourseIndexSource {
        origin: ArtifactOriginKind::Repository,
        repository_base_url: repository_base_url.to_string(),
        course_index,
        course_index_bytes,
        sunset: None,
    })
}

/// The publication `course_id`'s latest pointer names, read under the repository `course_index` came from. Fails
/// for an index from the bundle included in the app, and for a course `course_index` does not list.
pub async fn fetch_latest_course_manifest(
    http_fetch: &impl HttpFetch,
    course_index: &CourseIndexSource,
    course_id: &str,
) -> Result<CourseManifestSource, AppErrorStatic> {
    if course_index.origin == ArtifactOriginKind::Embedded {
        return Err(AppErrorStatic::from(format!(
            "a course's latest pointer is read only through a repository's course index; [course_id={course_id}]",
        )));
    }

    let entry: &CourseIndexEntry = find_index_entry(course_index, course_id)?;
    let manifest_bytes: Vec<u8> =
        fetch::fetch_course_manifest_pointer(http_fetch, &course_index.repository_base_url, &entry.latest_manifest_key).await?;
    let manifest: CourseManifest = course_manifest::parse_course_manifest(&manifest_bytes, Some(course_id))?;

    Ok(CourseManifestSource {
        origin: ArtifactOriginKind::Repository,
        repository_base_url: course_index.repository_base_url.clone(),
        manifest,
        manifest_bytes,
    })
}

fn find_index_entry<'a>(course_index: &'a CourseIndexSource, course_id: &str) -> Result<&'a CourseIndexEntry, AppErrorStatic> {
    course_index.course_index.courses.get(course_id).ok_or_else(|| {
        AppErrorStatic::from(format!(
            "the course index does not list the course; [course_id={course_id} repository_base_url={}]",
            course_index.repository_base_url,
        ))
    })
}

/// `entry` is `course_id`'s entry in the index of the bundle at `embedded_base_url`.
pub async fn load_embedded_course_manifest(
    http_fetch: &impl HttpFetch,
    embedded_base_url: &str,
    course_id: &str,
    entry: &CourseIndexEntry,
) -> Result<CourseManifestSource, AppErrorStatic> {
    let manifest_bytes: Vec<u8> = fetch::fetch_pointer(http_fetch, embedded_base_url, &entry.latest_manifest_key).await?;
    let manifest: CourseManifest = course_manifest::parse_course_manifest(&manifest_bytes, Some(course_id))?;

    Ok(CourseManifestSource {
        origin: ArtifactOriginKind::Embedded,
        repository_base_url: embedded_base_url.to_string(),
        manifest,
        manifest_bytes,
    })
}

/// `manifest_bytes` were read from `course_id`'s latest pointer in the repository at `repository_base_url`.
pub fn parse_cached_course_manifest(
    manifest_bytes: Vec<u8>,
    repository_base_url: &str,
    course_id: &str,
) -> Result<CourseManifestSource, AppError> {
    let manifest: CourseManifest = course_manifest::parse_course_manifest(&manifest_bytes, Some(course_id))?;

    Ok(CourseManifestSource {
        origin: ArtifactOriginKind::Repository,
        repository_base_url: repository_base_url.to_string(),
        manifest,
        manifest_bytes,
    })
}

/// A shard's decompressed bytes, ready to open as a database.
pub async fn load_publication_shard(
    http_fetch: &impl HttpFetch,
    artifact_cache: &impl ArtifactCache,
    source: &CourseManifestSource,
    entry: &ManifestEntry,
) -> Result<Vec<u8>, AppErrorStatic> {
    let compressed_bytes: Vec<u8> = load_publication_shard_bytes(http_fetch, artifact_cache, source, entry).await?;

    Ok(compression::decompress(&compressed_bytes)?)
}

/// A shard's verified bytes, as published, from the cache, else fetched from the publication and cached. A cached
/// copy whose hash no longer matches is refetched; a failure to write the cache does not fail the load.
pub async fn load_publication_shard_bytes(
    http_fetch: &impl HttpFetch,
    artifact_cache: &impl ArtifactCache,
    source: &CourseManifestSource,
    entry: &ManifestEntry,
) -> Result<Vec<u8>, AppErrorStatic> {
    let cached_bytes: Option<Vec<u8>> = read_verified_cache_entry(artifact_cache, entry).await;

    if let Some(bytes) = cached_bytes {
        return Ok(bytes);
    }

    let bytes: Vec<u8> = fetch::fetch_publication_file(
        http_fetch,
        &source.repository_base_url,
        &source.manifest.course_id,
        &source.manifest.publication_label,
        &entry.relative_path,
    )
    .await?;

    content_address::verify_sha256(&entry.relative_path, &bytes, &entry.sha256)?;

    write_cache_entry(artifact_cache, entry, &bytes).await;

    Ok(bytes)
}

async fn write_cache_entry(artifact_cache: &impl ArtifactCache, entry: &ManifestEntry, bytes: &[u8]) {
    let cache_write: Result<(), AppErrorStatic> = artifact_cache.put(&entry.relative_path, bytes).await;

    if let Err(error) = cache_write {
        log::warn!("could not cache a shard; [relative_path={} error={error}]", entry.relative_path);
    }
}

/// The cache paths of every file `course_manifests` name, each once, in order.
pub fn relative_paths_to_keep<'a>(course_manifests: impl IntoIterator<Item = &'a CourseManifest>) -> Vec<String> {
    let relative_paths: BTreeSet<String> = course_manifests
        .into_iter()
        .flat_map(CourseManifest::file_entries)
        .map(|entry| entry.relative_path.clone())
        .collect();

    relative_paths.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::artifact::cache::tests::MockArtifactCache;
    use crate::artifact::content_address::sha256_hex;
    use crate::artifact::repository_key;
    use crate::http::tests::MockHttpFetch;

    use super::*;

    fn manifest_json(sha256: &str) -> String {
        format!(
            r#"{{
  "manifest_schema_version": {schema_version},
  "version": "v1",
  "artifact_created": "2026-10-07T12:00:00Z",
  "courses": {{
    "c": {{
      "title": "t",
      "target_language": "pt-BR",
      "source_language": "en",
      "outline": {{ "relative_path": "data/c/course-{sha256}.sqlite", "size_bytes": 3, "sha256": "{sha256}" }},
      "units": {{}}
    }}
  }},
  "voice_pipeline_revisions": {{}}
}}"#,
            schema_version = manifest::MANIFEST_SCHEMA_VERSION,
        )
    }

    fn discovery_json(repository_base_url: &str, minimum_client_version: &str) -> Vec<u8> {
        format!(
            r#"{{"schema_version": 1, "repository_base_url": "{repository_base_url}", "minimum_client_version": "{minimum_client_version}", "sunset": null}}"#,
        )
        .into_bytes()
    }

    fn source_at(repository_base_url: &str) -> ManifestSource {
        let sha256: String = sha256_hex(b"abc");

        parse_cached_manifest(manifest_json(&sha256).into_bytes(), repository_base_url).unwrap()
    }

    fn entry_for(bytes: &[u8]) -> ManifestEntry {
        let sha256: String = sha256_hex(bytes);

        ManifestEntry {
            relative_path: format!("data/course-{sha256}.sqlite"),
            size_bytes: bytes.len() as u64,
            sha256,
        }
    }

    #[tokio::test]
    async fn discover_latest_manifest_reuses_the_speculative_request_when_discovery_names_the_static_base() {
        let sha256: String = sha256_hex(b"abc");
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            ("/discovery".to_string(), discovery_json("/repository", "0.1.0")),
            ("/repository/latest/manifest.json".to_string(), manifest_json(&sha256).into_bytes()),
        ]));

        let source: ManifestSource = discover_latest_manifest(&http_fetch, "/discovery", "/repository").await.unwrap();

        assert_eq!(source.repository_base_url, "/repository");
        assert_eq!(http_fetch.requested_urls().await.len(), 2);
    }

    #[tokio::test]
    async fn discover_latest_manifest_refetches_from_a_discovered_base() {
        let sha256: String = sha256_hex(b"abc");
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            ("/discovery".to_string(), discovery_json("https://repository.example", "0.1.0")),
            (
                "https://repository.example/latest/manifest.json".to_string(),
                manifest_json(&sha256).into_bytes(),
            ),
        ]));

        let source: ManifestSource = discover_latest_manifest(&http_fetch, "/discovery", "/repository").await.unwrap();

        assert_eq!(source.repository_base_url, "https://repository.example");
    }

    #[tokio::test]
    async fn discover_latest_manifest_resolves_a_root_relative_base_against_the_discovery_origin() {
        let sha256: String = sha256_hex(b"abc");
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            ("http://127.0.0.1:3000/discovery".to_string(), discovery_json("/repository", "0.1.0")),
            ("http://127.0.0.1:3000/repository/latest/manifest.json".to_string(), manifest_json(&sha256).into_bytes()),
        ]));

        let source: ManifestSource =
            discover_latest_manifest(&http_fetch, "http://127.0.0.1:3000/discovery", "http://127.0.0.1:3000/repository").await.unwrap();

        assert_eq!(source.repository_base_url, "http://127.0.0.1:3000/repository");
        assert_eq!(http_fetch.requested_urls().await.len(), 2);
    }

    #[tokio::test]
    async fn discover_latest_manifest_uses_the_static_base_when_discovery_fails() {
        let sha256: String = sha256_hex(b"abc");
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(
            "/repository/latest/manifest.json".to_string(),
            manifest_json(&sha256).into_bytes(),
        )]));

        let source: ManifestSource = discover_latest_manifest(&http_fetch, "/discovery", "/repository").await.unwrap();

        assert_eq!(source.repository_base_url, "/repository");
    }

    #[tokio::test]
    async fn discover_latest_manifest_still_serves_a_client_below_the_minimum_version() {
        let sha256: String = sha256_hex(b"abc");
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            ("/discovery".to_string(), discovery_json("/repository", "99.0.0")),
            ("/repository/latest/manifest.json".to_string(), manifest_json(&sha256).into_bytes()),
        ]));

        let source: ManifestSource = discover_latest_manifest(&http_fetch, "/discovery", "/repository").await.unwrap();

        assert_eq!(source.repository_base_url, "/repository");
    }

    #[tokio::test]
    async fn load_shard_bytes_fetches_verifies_and_caches() {
        let entry: ManifestEntry = entry_for(b"abc");
        let url: String = format!("/repository/v1/{}", entry.relative_path);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(url, b"abc".to_vec())]));
        let cache: MockArtifactCache = MockArtifactCache::new();

        let bytes: Vec<u8> = load_shard_bytes(&http_fetch, &cache, &source_at("/repository"), &entry).await.unwrap();

        assert_eq!(bytes, b"abc".to_vec());
        assert_eq!(cache.paths().await, vec![entry.relative_path.clone()]);
    }

    #[tokio::test]
    async fn load_shard_bytes_serves_a_cached_shard_without_the_network() {
        let entry: ManifestEntry = entry_for(b"abc");
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::new());
        let cache: MockArtifactCache = MockArtifactCache::new();
        cache.put(&entry.relative_path, b"abc").await.unwrap();

        let bytes: Vec<u8> = load_shard_bytes(&http_fetch, &cache, &source_at("/repository"), &entry).await.unwrap();

        assert_eq!(bytes, b"abc".to_vec());
        assert!(http_fetch.requested_urls().await.is_empty());
    }

    #[tokio::test]
    async fn load_shard_bytes_refetches_when_the_cached_copy_is_corrupt() {
        let entry: ManifestEntry = entry_for(b"abc");
        let url: String = format!("/repository/v1/{}", entry.relative_path);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(url, b"abc".to_vec())]));
        let cache: MockArtifactCache = MockArtifactCache::new();
        cache.put(&entry.relative_path, b"corrupted").await.unwrap();

        let bytes: Vec<u8> = load_shard_bytes(&http_fetch, &cache, &source_at("/repository"), &entry).await.unwrap();

        assert_eq!(bytes, b"abc".to_vec());
        assert_eq!(cache.get(&entry.relative_path).await.unwrap().as_deref(), Some(b"abc".as_slice()));
    }

    #[tokio::test]
    async fn load_shard_bytes_rejects_a_fetched_file_whose_hash_differs() {
        let entry: ManifestEntry = entry_for(b"abc");
        let url: String = format!("/repository/v1/{}", entry.relative_path);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(url, b"tampered".to_vec())]));
        let cache: MockArtifactCache = MockArtifactCache::new();

        let result: Result<Vec<u8>, AppErrorStatic> = load_shard_bytes(&http_fetch, &cache, &source_at("/repository"), &entry).await;

        assert!(result.is_err());
        assert!(cache.paths().await.is_empty());
    }

    const COURSE_ID: &str = "pt_br.en.foundations";
    const OTHER_COURSE_ID: &str = "pt_br.en.test_course";
    const PUBLICATION_LABEL: &str = "2026-10-08+duluth";

    fn course_manifest_json(course_id: &str, outline_bytes: &[u8]) -> Vec<u8> {
        let sha256: String = sha256_hex(outline_bytes);

        format!(
            r#"{{
  "manifest_schema_version": {schema_version},
  "course_id": "{course_id}",
  "publication_label": "{PUBLICATION_LABEL}",
  "artifact_created": "2026-10-08T14:02:11Z",
  "title": "t",
  "target_language": "pt-BR",
  "source_language": "en",
  "test_course": false,
  "outline": {{ "relative_path": "data/course-{sha256}.sqlite.br", "size_bytes": {size_bytes}, "sha256": "{sha256}" }},
  "units": {{}},
  "voice_pipeline_revisions": {{}}
}}"#,
            schema_version = course_manifest::COURSE_MANIFEST_SCHEMA_VERSION,
            size_bytes = outline_bytes.len(),
        )
        .into_bytes()
    }

    fn course_index_json(course_ids: &[&str]) -> Vec<u8> {
        let courses: BTreeMap<String, CourseIndexEntry> = course_ids
            .iter()
            .map(|course_id| {
                let entry: CourseIndexEntry = CourseIndexEntry {
                    title: "t".to_string(),
                    target_language: "pt-BR".to_string(),
                    source_language: "en".to_string(),
                    latest_manifest_key: repository_key::course_latest_manifest_key(course_id),
                    test_course: false,
                };

                (course_id.to_string(), entry)
            })
            .collect();
        let course_index: CourseIndex = CourseIndex {
            course_index_schema_version: course_index::COURSE_INDEX_SCHEMA_VERSION,
            courses,
        };

        serde_json::to_vec_pretty(&course_index).unwrap()
    }

    fn course_pointer_url(repository_base_url: &str, course_id: &str) -> String {
        fetch::join_url(repository_base_url, &repository_key::course_latest_manifest_key(course_id))
    }

    fn repository_index_at(repository_base_url: &str, course_ids: &[&str]) -> CourseIndexSource {
        parse_cached_course_index(course_index_json(course_ids), repository_base_url).unwrap()
    }

    fn course_manifest_source_at(repository_base_url: &str, outline_bytes: &[u8]) -> CourseManifestSource {
        parse_cached_course_manifest(course_manifest_json(COURSE_ID, outline_bytes), repository_base_url, COURSE_ID).unwrap()
    }

    #[tokio::test]
    async fn discover_course_index_reuses_the_speculative_request_when_discovery_names_the_static_base() {
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            ("/discovery".to_string(), discovery_json("/repository", "0.1.0")),
            ("/repository/course-index.json".to_string(), course_index_json(&[COURSE_ID])),
        ]));

        let source: CourseIndexSource = discover_course_index(&http_fetch, "/discovery", "/repository").await.unwrap();

        assert_eq!(source.origin, ArtifactOriginKind::Repository);
        assert_eq!(source.repository_base_url, "/repository");
        assert!(source.course_index.courses.contains_key(COURSE_ID));
        assert_eq!(source.course_index_bytes, course_index_json(&[COURSE_ID]));
        assert_eq!(http_fetch.requested_urls().await.len(), 2);
    }

    #[tokio::test]
    async fn discover_course_index_refetches_from_a_discovered_base() {
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            ("/discovery".to_string(), discovery_json("https://repository.example", "0.1.0")),
            ("https://repository.example/course-index.json".to_string(), course_index_json(&[COURSE_ID])),
        ]));

        let source: CourseIndexSource = discover_course_index(&http_fetch, "/discovery", "/repository").await.unwrap();

        assert_eq!(source.repository_base_url, "https://repository.example");
        assert!(http_fetch.requested_urls().await.contains(&"https://repository.example/course-index.json".to_string()));
    }

    #[tokio::test]
    async fn discover_course_index_uses_the_static_base_when_discovery_fails() {
        let http_fetch: MockHttpFetch =
            MockHttpFetch::new(BTreeMap::from([("/repository/course-index.json".to_string(), course_index_json(&[COURSE_ID]))]));

        let source: CourseIndexSource = discover_course_index(&http_fetch, "/discovery", "/repository").await.unwrap();

        assert_eq!(source.repository_base_url, "/repository");
    }

    #[tokio::test]
    async fn fetch_latest_course_manifest_reads_the_pointer_the_index_names_under_the_index_base() {
        let course_index: CourseIndexSource = repository_index_at("https://repository.example", &[COURSE_ID]);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(
            course_pointer_url("https://repository.example", COURSE_ID),
            course_manifest_json(COURSE_ID, b"abc"),
        )]));

        let source: CourseManifestSource = fetch_latest_course_manifest(&http_fetch, &course_index, COURSE_ID).await.unwrap();

        assert_eq!(source.origin, ArtifactOriginKind::Repository);
        assert_eq!(source.repository_base_url, "https://repository.example");
        assert_eq!(source.manifest.publication_label, PUBLICATION_LABEL);
        assert_eq!(source.manifest_bytes, course_manifest_json(COURSE_ID, b"abc"));
    }

    #[tokio::test]
    async fn fetch_latest_course_manifest_refuses_an_embedded_index() {
        let http_fetch: MockHttpFetch =
            MockHttpFetch::new(BTreeMap::from([("/embedded/course-index.json".to_string(), course_index_json(&[COURSE_ID]))]));
        let course_index: CourseIndexSource = load_embedded_course_index(&http_fetch, "/embedded").await.unwrap();

        let result: Result<CourseManifestSource, AppErrorStatic> = fetch_latest_course_manifest(&http_fetch, &course_index, COURSE_ID).await;

        assert_eq!(course_index.origin, ArtifactOriginKind::Embedded);
        assert!(result.is_err());
        assert_eq!(http_fetch.requested_urls().await, vec!["/embedded/course-index.json".to_string()]);
    }

    #[tokio::test]
    async fn fetch_latest_course_manifest_refuses_a_course_the_index_does_not_list() {
        let course_index: CourseIndexSource = repository_index_at("/repository", &[COURSE_ID]);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::new());

        let error: AppErrorStatic = fetch_latest_course_manifest(&http_fetch, &course_index, OTHER_COURSE_ID).await.unwrap_err();

        assert!(error.to_string().contains("does not list the course"));
        assert!(http_fetch.requested_urls().await.is_empty());
    }

    #[tokio::test]
    async fn fetch_latest_course_manifest_failing_for_one_course_leaves_another_course_loading() {
        let course_index: CourseIndexSource = repository_index_at("/repository", &[COURSE_ID, OTHER_COURSE_ID]);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(
            course_pointer_url("/repository", COURSE_ID),
            course_manifest_json(COURSE_ID, b"abc"),
        )]));

        let failed: Result<CourseManifestSource, AppErrorStatic> =
            fetch_latest_course_manifest(&http_fetch, &course_index, OTHER_COURSE_ID).await;
        let loaded: Result<CourseManifestSource, AppErrorStatic> = fetch_latest_course_manifest(&http_fetch, &course_index, COURSE_ID).await;

        assert!(failed.unwrap_err().to_string().contains("status=404"));
        assert_eq!(loaded.unwrap().manifest.course_id, COURSE_ID);
    }

    #[tokio::test]
    async fn fetch_latest_course_manifest_rejects_a_pointer_holding_another_courses_manifest() {
        let course_index: CourseIndexSource = repository_index_at("/repository", &[COURSE_ID, OTHER_COURSE_ID]);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(
            course_pointer_url("/repository", OTHER_COURSE_ID),
            course_manifest_json(COURSE_ID, b"abc"),
        )]));

        let result: Result<CourseManifestSource, AppErrorStatic> =
            fetch_latest_course_manifest(&http_fetch, &course_index, OTHER_COURSE_ID).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn load_embedded_course_manifest_reads_the_pointer_the_embedded_index_names() {
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            ("/embedded/course-index.json".to_string(), course_index_json(&[COURSE_ID])),
            (course_pointer_url("/embedded", COURSE_ID), course_manifest_json(COURSE_ID, b"abc")),
        ]));
        let course_index: CourseIndexSource = load_embedded_course_index(&http_fetch, "/embedded").await.unwrap();
        let entry: &CourseIndexEntry = &course_index.course_index.courses[COURSE_ID];

        let source: CourseManifestSource = load_embedded_course_manifest(&http_fetch, "/embedded", COURSE_ID, entry).await.unwrap();

        assert_eq!(source.origin, ArtifactOriginKind::Embedded);
        assert_eq!(source.repository_base_url, "/embedded");
        assert_eq!(source.manifest.course_id, COURSE_ID);
    }

    #[test]
    fn parse_cached_course_manifest_rejects_another_courses_manifest() {
        let result: Result<CourseManifestSource, AppError> =
            parse_cached_course_manifest(course_manifest_json(COURSE_ID, b"abc"), "/repository", OTHER_COURSE_ID);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn load_publication_shard_bytes_fetches_from_the_publication_and_caches_by_relative_path() {
        let source: CourseManifestSource = course_manifest_source_at("/repository", b"abc");
        let entry: ManifestEntry = source.manifest.outline.clone();
        let url: String = format!("/repository/courses/pt_br.en.foundations/2026-10-08+duluth/{}", entry.relative_path);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(url.clone(), b"abc".to_vec())]));
        let cache: MockArtifactCache = MockArtifactCache::new();

        let bytes: Vec<u8> = load_publication_shard_bytes(&http_fetch, &cache, &source, &entry).await.unwrap();

        assert_eq!(bytes, b"abc".to_vec());
        assert_eq!(http_fetch.requested_urls().await, vec![url]);
        assert_eq!(cache.paths().await, vec![entry.relative_path.clone()]);
    }

    #[tokio::test]
    async fn load_publication_shard_bytes_serves_a_cached_shard_without_the_network() {
        let source: CourseManifestSource = course_manifest_source_at("/repository", b"abc");
        let entry: ManifestEntry = source.manifest.outline.clone();
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::new());
        let cache: MockArtifactCache = MockArtifactCache::new();
        cache.put(&entry.relative_path, b"abc").await.unwrap();

        let bytes: Vec<u8> = load_publication_shard_bytes(&http_fetch, &cache, &source, &entry).await.unwrap();

        assert_eq!(bytes, b"abc".to_vec());
        assert!(http_fetch.requested_urls().await.is_empty());
    }

    #[tokio::test]
    async fn load_publication_shard_bytes_rejects_a_fetched_file_whose_hash_differs() {
        let source: CourseManifestSource = course_manifest_source_at("/repository", b"abc");
        let entry: ManifestEntry = source.manifest.outline.clone();
        let url: String = format!("/repository/courses/pt_br.en.foundations/2026-10-08+duluth/{}", entry.relative_path);
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([(url, b"tampered".to_vec())]));
        let cache: MockArtifactCache = MockArtifactCache::new();

        let result: Result<Vec<u8>, AppErrorStatic> = load_publication_shard_bytes(&http_fetch, &cache, &source, &entry).await;

        assert!(result.is_err());
        assert!(cache.paths().await.is_empty());
    }

    #[test]
    fn relative_paths_to_keep_is_the_union_of_every_manifests_files() {
        let mut first_manifest: CourseManifest = course_manifest_source_at("/repository", b"abc").manifest;
        let mut second_manifest: CourseManifest =
            parse_cached_course_manifest(course_manifest_json(OTHER_COURSE_ID, b"abc"), "/repository", OTHER_COURSE_ID)
                .unwrap()
                .manifest;
        let shared_outline_path: String = first_manifest.outline.relative_path.clone();

        first_manifest.units.insert("a1-01".to_string(), entry_for(b"first unit"));
        second_manifest.units.insert("a1-01".to_string(), entry_for(b"second unit"));

        let relative_paths: Vec<String> = relative_paths_to_keep([&first_manifest, &second_manifest]);

        let mut expected_paths: Vec<String> =
            vec![shared_outline_path, entry_for(b"first unit").relative_path, entry_for(b"second unit").relative_path];
        expected_paths.sort();

        assert_eq!(relative_paths, expected_paths);
    }
}

/* The iOS FFI exports these loads through `#[uniffi::export(async_runtime = "tokio")]`, which requires a `Send`
   future, so this pins the bound against the concrete implementations a native client uses. */
#[cfg(all(test, not(target_arch = "wasm32")))]
mod send_bound {
    use super::*;
    use crate::artifact::{FilesystemArtifactCache, fetch};
    use crate::http::{DirectoryFetch, HttpCacheMode, ReqwestHttpFetch};

    fn assert_send<T: Send>(_value: T) {}

    fn manifest_source() -> ManifestSource {
        let sha256: String = crate::artifact::sha256_hex(b"abc");
        let manifest_json: String = format!(
            r#"{{"manifest_schema_version": {}, "version": "v1", "artifact_created": "2026-10-07T12:00:00Z",
                "courses": {{"c": {{"title": "t", "target_language": "pt-BR", "source_language": "en",
                "outline": {{"relative_path": "data/c/course-{sha256}.sqlite.br", "size_bytes": 3, "sha256": "{sha256}"}}, "units": {{}}}}}},
                "voice_pipeline_revisions": {{}}}}"#,
            manifest::MANIFEST_SCHEMA_VERSION,
        );

        parse_cached_manifest(manifest_json.into_bytes(), "/repository").unwrap()
    }

    #[test]
    fn the_loads_the_native_client_awaits_are_send() {
        let http_fetch: ReqwestHttpFetch = ReqwestHttpFetch::create().unwrap();
        let cache: FilesystemArtifactCache = FilesystemArtifactCache::create(std::path::PathBuf::from("/tmp/send-bound"));
        let source: ManifestSource = manifest_source();
        let entry: ManifestEntry = source.manifest.courses["c"].outline.clone();

        assert_send(discover_latest_manifest(&http_fetch, "http://127.0.0.1/discovery", "http://127.0.0.1/repository"));
        assert_send(load_embedded_manifest(&DirectoryFetch, "/embedded"));
        assert_send(load_shard(&http_fetch, &cache, &source, &entry));
        assert_send(load_shard(&DirectoryFetch, &cache, &source, &entry));
        assert_send(fetch::fetch_bytes(&http_fetch, "http://127.0.0.1/audio/a.m4a", HttpCacheMode::Default));
        assert_send(crate::artifact::ArtifactCache::retain(&cache, &[]));
    }

    fn course_index_source() -> CourseIndexSource {
        let course_index_json: String = format!(
            r#"{{"course_index_schema_version": {}, "courses": {{"pt_br.en.foundations": {{"title": "t", "target_language": "pt-BR",
                "source_language": "en", "latest_manifest_key": "courses/pt_br.en.foundations/latest/manifest.json", "test_course": false}}}}}}"#,
            course_index::COURSE_INDEX_SCHEMA_VERSION,
        );

        parse_cached_course_index(course_index_json.into_bytes(), "http://127.0.0.1/repository").unwrap()
    }

    fn course_manifest_source() -> CourseManifestSource {
        let sha256: String = crate::artifact::sha256_hex(b"abc");
        let course_manifest_json: String = format!(
            r#"{{"manifest_schema_version": {}, "course_id": "pt_br.en.foundations", "publication_label": "2026-10-08+duluth",
                "artifact_created": "2026-10-08T14:02:11Z", "title": "t", "target_language": "pt-BR", "source_language": "en",
                "test_course": false, "outline": {{"relative_path": "data/course-{sha256}.sqlite.br", "size_bytes": 3, "sha256": "{sha256}"}},
                "units": {{}}, "voice_pipeline_revisions": {{}}}}"#,
            course_manifest::COURSE_MANIFEST_SCHEMA_VERSION,
        );

        parse_cached_course_manifest(course_manifest_json.into_bytes(), "http://127.0.0.1/repository", "pt_br.en.foundations").unwrap()
    }

    #[test]
    fn the_course_loads_the_native_client_awaits_are_send() {
        let http_fetch: ReqwestHttpFetch = ReqwestHttpFetch::create().unwrap();
        let cache: FilesystemArtifactCache = FilesystemArtifactCache::create(std::path::PathBuf::from("/tmp/send-bound"));
        let course_index: CourseIndexSource = course_index_source();
        let index_entry: CourseIndexEntry = course_index.course_index.courses["pt_br.en.foundations"].clone();
        let source: CourseManifestSource = course_manifest_source();
        let entry: ManifestEntry = source.manifest.outline.clone();

        assert_send(discover_course_index(&http_fetch, "http://127.0.0.1/discovery", "http://127.0.0.1/repository"));
        assert_send(load_embedded_course_index(&DirectoryFetch, "/embedded"));
        assert_send(fetch_latest_course_manifest(&http_fetch, &course_index, "pt_br.en.foundations"));
        assert_send(load_embedded_course_manifest(&DirectoryFetch, "/embedded", "pt_br.en.foundations", &index_entry));
        assert_send(load_publication_shard(&http_fetch, &cache, &source, &entry));
        assert_send(load_publication_shard(&DirectoryFetch, &cache, &source, &entry));
        assert_send(load_publication_shard_bytes(&http_fetch, &cache, &source, &entry));
        assert_send(load_publication_shard_bytes(&DirectoryFetch, &cache, &source, &entry));
    }
}
