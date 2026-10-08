use std::fs;
use std::io::ErrorKind;

use crate::AppErrorStatic;
use crate::http::{HttpFetch, HttpRequest, HttpResponse};

/// Serves URLs that are filesystem paths, answering a missing file with 404, for bundles shipped as files.
pub struct DirectoryFetch;

impl HttpFetch for DirectoryFetch {
    async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppErrorStatic> {
        let read: Result<Vec<u8>, std::io::Error> = fs::read(&request.url);

        match read {
            Ok(bytes) => Ok(HttpResponse { status: 200, bytes }),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(HttpResponse {
                status: 404,
                bytes: Vec::new(),
            }),
            Err(error) => Err(AppErrorStatic::from(format!("reading a bundled file failed; [path={} error={error}]", request.url))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpCacheMode;

    #[tokio::test]
    async fn get_reads_a_file_and_answers_404_for_a_missing_one() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let path: std::path::PathBuf = directory.path().join("manifest.json");
        fs::write(&path, b"{}").unwrap();

        let present: HttpResponse = DirectoryFetch
            .get(&HttpRequest {
                url: path.display().to_string(),
                cache_mode: HttpCacheMode::Default,
            })
            .await
            .unwrap();
        let absent: HttpResponse = DirectoryFetch
            .get(&HttpRequest {
                url: directory.path().join("absent").display().to_string(),
                cache_mode: HttpCacheMode::Default,
            })
            .await
            .unwrap();

        assert_eq!(present.bytes, b"{}".to_vec());
        assert_eq!(absent.status, 404);
    }
}
