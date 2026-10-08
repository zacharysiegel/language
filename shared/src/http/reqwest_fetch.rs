use reqwest::header::{self, HeaderValue};
use reqwest::{Client, RequestBuilder};

use crate::error;
use crate::{AppError, AppErrorStatic};
use crate::http::{HttpCacheMode, HttpFetch, HttpRequest, HttpResponse};

pub struct ReqwestHttpFetch {
    client: Client,
}

impl ReqwestHttpFetch {
    pub fn create() -> Result<ReqwestHttpFetch, AppError> {
        let client: Client = Client::builder()
            .build()
            .map_err(|error| AppError::from(format!("building an HTTP client failed; [error={error}]")))?;

        Ok(ReqwestHttpFetch { client })
    }
}

impl HttpFetch for ReqwestHttpFetch {
    async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppErrorStatic> {
        let builder: RequestBuilder = match request.cache_mode {
            // reqwest holds no cache of its own; the header is addressed to intermediaries.
            HttpCacheMode::Reload => self.client.get(&request.url).header(header::CACHE_CONTROL, HeaderValue::from_static("no-cache")),
            HttpCacheMode::Default => self.client.get(&request.url),
        };

        let response: reqwest::Response = builder
            .send()
            .await
            .map_err(|error| AppErrorStatic::from(format!("fetch failed; [url={} error={}]", request.url, error::render_error_chain(&error))))?;
        let status: u16 = response.status().as_u16();
        let bytes: Vec<u8> = response
            .bytes()
            .await
            .map_err(|error| AppErrorStatic::from(format!("reading a response failed; [url={} error={}]", request.url, error::render_error_chain(&error))))?
            .to_vec();

        Ok(HttpResponse { status, bytes })
    }
}
