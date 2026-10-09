use crate::AppErrorStatic;
use crate::artifact::course_manifest::CourseManifest;
use crate::artifact::repository_key;
use crate::http::{HttpCacheMode, HttpFetch, HttpRequest, HttpResponse};

pub fn join_url(base_url: &str, key: &str) -> String {
    format!("{}/{key}", base_url.trim_end_matches('/'))
}

/// `reference` made absolute against `base_url`'s origin when it is a root-relative path and `base_url` has one.
pub fn resolve_reference(base_url: &str, reference: &str) -> String {
    let is_root_relative: bool = reference.starts_with('/') && !reference.starts_with("//");
    let origin: Option<&str> = base_url
        .find("://")
        .map(|scheme_end| {
            let authority_start: usize = scheme_end + "://".len();
            let authority_end: usize = base_url[authority_start..]
                .find('/')
                .map_or(base_url.len(), |offset| authority_start + offset);

            &base_url[..authority_end]
        });

    match (is_root_relative, origin) {
        (true, Some(origin)) => format!("{origin}{reference}"),
        _ => reference.to_string(),
    }
}

pub async fn fetch_bytes(http_fetch: &impl HttpFetch, url: &str, cache_mode: HttpCacheMode) -> Result<Vec<u8>, AppErrorStatic> {
    let response: HttpResponse = http_fetch
        .get(&HttpRequest {
            url: url.to_string(),
            cache_mode,
        })
        .await?;

    if !response.is_success() {
        return Err(AppErrorStatic::from(format!("fetch failed; [url={url} status={}]", response.status)));
    }

    Ok(response.bytes)
}

/* Pointers change on every publish, so they bypass the HTTP cache; every other file is content-addressed. */
pub async fn fetch_pointer(http_fetch: &impl HttpFetch, base_url: &str, key: &str) -> Result<Vec<u8>, AppErrorStatic> {
    fetch_bytes(http_fetch, &join_url(base_url, key), HttpCacheMode::Reload).await
}

pub async fn fetch_course_index(http_fetch: &impl HttpFetch, base_url: &str) -> Result<Vec<u8>, AppErrorStatic> {
    fetch_pointer(http_fetch, base_url, repository_key::COURSE_INDEX_KEY).await
}

/// The course manifest bytes at `latest_manifest_key`, falling back to this reader's own schema pointer beside it
/// when that pointer has been published by a newer schema version.
pub async fn fetch_course_manifest_pointer(
    http_fetch: &impl HttpFetch,
    base_url: &str,
    latest_manifest_key: &str,
) -> Result<Vec<u8>, AppErrorStatic> {
    let latest_bytes: Vec<u8> = fetch_pointer(http_fetch, base_url, latest_manifest_key).await?;
    let fallback_key: Option<String> = CourseManifest::schema_fallback_key(latest_manifest_key, &latest_bytes);

    match fallback_key {
        Some(key) => fetch_pointer(http_fetch, base_url, &key).await,
        None => Ok(latest_bytes),
    }
}

/// `relative_path` is relative to the publication's directory.
pub async fn fetch_publication_file(
    http_fetch: &impl HttpFetch,
    base_url: &str,
    course_id: &str,
    publication_label: &str,
    relative_path: &str,
) -> Result<Vec<u8>, AppErrorStatic> {
    let key: String = repository_key::publication_file_key(course_id, publication_label, relative_path);

    fetch_bytes(http_fetch, &join_url(base_url, &key), HttpCacheMode::Default).await
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::artifact::course_manifest;
    use crate::http::tests::MockHttpFetch;

    use super::*;

    #[test]
    fn join_url_collapses_a_trailing_slash() {
        assert_eq!(join_url("https://example.org/", "data/x"), "https://example.org/data/x");
        assert_eq!(join_url("/repository", "data/x"), "/repository/data/x");
    }

    #[test]
    fn resolve_reference_joins_a_root_relative_path_to_the_base_origin() {
        assert_eq!(resolve_reference("http://127.0.0.1:3000/discovery", "/repository"), "http://127.0.0.1:3000/repository");
        assert_eq!(resolve_reference("https://example.org", "/repository"), "https://example.org/repository");
        assert_eq!(resolve_reference("/discovery", "/repository"), "/repository");
        assert_eq!(resolve_reference("http://127.0.0.1:3000/discovery", "https://cdn.example/r"), "https://cdn.example/r");
    }

    #[tokio::test]
    async fn fetch_bytes_reports_a_non_success_status() {
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::new());

        let error: AppErrorStatic = fetch_pointer(&http_fetch, "/repository", repository_key::COURSE_INDEX_KEY).await.unwrap_err();

        assert!(error.to_string().contains("status=404"));
    }

    #[tokio::test]
    async fn fetch_course_manifest_pointer_returns_the_named_pointer_when_readable() {
        let latest_manifest_key: String = repository_key::course_latest_manifest_key("pt_br.en.foundations");
        let current_document: Vec<u8> =
            format!(r#"{{"manifest_schema_version": {}}}"#, course_manifest::COURSE_MANIFEST_SCHEMA_VERSION).into_bytes();
        let http_fetch: MockHttpFetch =
            MockHttpFetch::new(BTreeMap::from([(format!("/repository/{latest_manifest_key}"), current_document.clone())]));

        let bytes: Vec<u8> = fetch_course_manifest_pointer(&http_fetch, "/repository", &latest_manifest_key).await.unwrap();

        assert_eq!(bytes, current_document);
        assert_eq!(http_fetch.requested_urls().await.len(), 1);
    }

    #[tokio::test]
    async fn fetch_course_manifest_pointer_falls_back_to_the_schema_pointer_beside_the_named_pointer() {
        let latest_manifest_key: String = repository_key::course_latest_manifest_key("pt_br.en.foundations");
        let schema_pointer_key: String =
            repository_key::course_schema_pointer_key(&latest_manifest_key, course_manifest::COURSE_MANIFEST_SCHEMA_VERSION);
        let newer_document: Vec<u8> = br#"{"manifest_schema_version": 99}"#.to_vec();
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::from([
            (format!("/repository/{latest_manifest_key}"), newer_document),
            (format!("/repository/{schema_pointer_key}"), b"older reader's manifest".to_vec()),
        ]));

        let bytes: Vec<u8> = fetch_course_manifest_pointer(&http_fetch, "/repository", &latest_manifest_key).await.unwrap();

        assert_eq!(bytes, b"older reader's manifest".to_vec());
        assert_eq!(
            http_fetch.requested_urls().await,
            vec![
                "/repository/courses/pt_br.en.foundations/latest/manifest.json".to_string(),
                format!("/repository/courses/pt_br.en.foundations/latest/manifest.{}.json", course_manifest::COURSE_MANIFEST_SCHEMA_VERSION),
            ],
        );
    }
}
