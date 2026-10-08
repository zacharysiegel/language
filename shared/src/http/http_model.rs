use crate::AppErrorStatic;

pub enum HttpCacheMode {
    Default,
    Reload,
}

pub struct HttpRequest {
    pub url: String,
    pub cache_mode: HttpCacheMode,
}

pub struct HttpResponse {
    pub status: u16,
    pub bytes: Vec<u8>,
}

impl HttpResponse {
    pub fn is_success(&self) -> bool {
        (200..=299).contains(&self.status)
    }
}

/// Reports the body's length rather than its contents, which run to megabytes.
impl std::fmt::Debug for HttpResponse {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HttpResponse")
            .field("status", &self.status)
            .field("byte_count", &self.bytes.len())
            .finish()
    }
}

/// The transport the artifact loader reaches the repository through. The returned future is deliberately
/// not `Send`: some platforms' response handles are not, and one trait serves every platform.
#[allow(async_fn_in_trait)]
pub trait HttpFetch {
    /// Errors only on a transport failure; a non-2xx status is returned for the caller to judge.
    async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppErrorStatic>;
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::BTreeMap;

    use super::*;

    /// Serves the bodies it was seeded with and 404s everything else, recording each URL asked for.
    pub(crate) struct MockHttpFetch {
        bodies_by_url: BTreeMap<String, Vec<u8>>,
        requested_urls: tokio::sync::Mutex<Vec<String>>,
    }

    impl MockHttpFetch {
        pub(crate) fn new(bodies_by_url: BTreeMap<String, Vec<u8>>) -> MockHttpFetch {
            MockHttpFetch {
                bodies_by_url,
                requested_urls: tokio::sync::Mutex::new(Vec::new()),
            }
        }

        pub(crate) async fn requested_urls(&self) -> Vec<String> {
            self.requested_urls.lock().await.clone()
        }
    }

    impl HttpFetch for MockHttpFetch {
        async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppErrorStatic> {
            self.requested_urls.lock().await.push(request.url.clone());

            match self.bodies_by_url.get(&request.url) {
                Some(bytes) => Ok(HttpResponse {
                    status: 200,
                    bytes: bytes.clone(),
                }),
                None => Ok(HttpResponse {
                    status: 404,
                    bytes: Vec::new(),
                }),
            }
        }
    }

    #[tokio::test]
    async fn mock_fetch_returns_404_for_an_unseeded_url() {
        let http_fetch: MockHttpFetch = MockHttpFetch::new(BTreeMap::new());

        let response: HttpResponse = http_fetch
            .get(&HttpRequest {
                url: "https://repository.example/absent".to_string(),
                cache_mode: HttpCacheMode::Default,
            })
            .await
            .unwrap();

        assert!(!response.is_success());
    }
}
