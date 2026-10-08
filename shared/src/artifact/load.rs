use futures_util::future;

use crate::AppError;
use crate::artifact::cache::ArtifactCache;
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
) -> Result<ManifestSource, AppError> {
    let (discovery_bytes, speculative_manifest_bytes): (Result<Vec<u8>, AppError>, Result<Vec<u8>, AppError>) = future::join(
        fetch::fetch_bytes(http_fetch, discovery_url, HttpCacheMode::Reload),
        fetch::fetch_latest_manifest(http_fetch, static_base_url),
    )
    .await;

    let discovery_document: Result<DiscoveryDocument, AppError> = discovery_bytes
        .and_then(|bytes| discovery::parse_discovery_document(&bytes))
        .map(|document| DiscoveryDocument {
            schema_version: document.schema_version,
            repository_base_url: fetch::resolve_reference(discovery_url, &document.repository_base_url),
            minimum_client_version: document.minimum_client_version,
            sunset: document.sunset,
        });

    if let Err(error) = &discovery_document {
        log::warn!("discovery unavailable, using the static repository; [error={error}]");
    }

    let sunset: Option<String> = read_client_support(&discovery_document);
    let authoritative_base: AuthoritativeBase = discovery::authoritative_repository_base(static_base_url, &discovery_document);

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

/// The contract's sunset date; a client below `minimum_client_version` is logged and still served.
fn read_client_support(discovery_document: &Result<DiscoveryDocument, AppError>) -> Option<String> {
    let Ok(document) = discovery_document
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
pub async fn load_embedded_manifest(http_fetch: &impl HttpFetch, embedded_base_url: &str) -> Result<ManifestSource, AppError> {
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
) -> Result<Vec<u8>, AppError> {
    let compressed_bytes: Vec<u8> = load_shard_bytes(http_fetch, artifact_cache, source, entry).await?;

    compression::decompress(&compressed_bytes)
}

/// A shard's verified bytes, as published, from the cache, else fetched from the repository and cached. A cached copy whose
/// hash no longer matches is refetched; a failure to write the cache does not fail the load.
pub async fn load_shard_bytes(
    http_fetch: &impl HttpFetch,
    artifact_cache: &impl ArtifactCache,
    source: &ManifestSource,
    entry: &ManifestEntry,
) -> Result<Vec<u8>, AppError> {
    let cached_bytes: Option<Vec<u8>> = read_verified_cache_entry(artifact_cache, entry).await;

    if let Some(bytes) = cached_bytes {
        return Ok(bytes);
    }

    let bytes: Vec<u8> =
        fetch::fetch_versioned_file(http_fetch, &source.repository_base_url, &source.manifest.version, &entry.relative_path).await?;

    content_address::verify_sha256(&entry.relative_path, &bytes, &entry.sha256)?;

    let cache_write: Result<(), AppError> = artifact_cache.put(&entry.relative_path, &bytes).await;

    if let Err(error) = cache_write {
        log::warn!("could not cache a shard; [relative_path={} error={error}]", entry.relative_path);
    }

    Ok(bytes)
}

async fn read_verified_cache_entry(artifact_cache: &impl ArtifactCache, entry: &ManifestEntry) -> Option<Vec<u8>> {
    let cache_read: Result<Option<Vec<u8>>, AppError> = artifact_cache.get(&entry.relative_path).await;
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::artifact::cache::tests::MockArtifactCache;
    use crate::artifact::content_address::sha256_hex;
    use crate::http::tests::MockHttpFetch;

    use super::*;

    fn manifest_json(sha256: &str) -> String {
        format!(
            r#"{{
  "manifest_schema_version": {schema_version},
  "version": "v1",
  "artifact_created": "2026-10-07T12:00:00Z",
  "course": {{ "relative_path": "data/course-{sha256}.sqlite", "size_bytes": 3, "sha256": "{sha256}" }},
  "units": {{}},
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

        let result: Result<Vec<u8>, AppError> = load_shard_bytes(&http_fetch, &cache, &source_at("/repository"), &entry).await;

        assert!(result.is_err());
        assert!(cache.paths().await.is_empty());
    }
}
