use reqwest::header::{self, HeaderValue};
use reqwest::{Client, ClientBuilder, RequestBuilder};

use crate::error;
use crate::{AppError, AppErrorStatic};
use crate::http::{HttpCacheMode, HttpFetch, HttpRequest, HttpResponse};

pub struct ReqwestHttpFetch {
    client: Client,
}

impl ReqwestHttpFetch {
    pub fn create() -> Result<ReqwestHttpFetch, AppError> {
        let builder: ClientBuilder = with_trust_roots(Client::builder())?;
        let client: Client = builder
            .build()
            .map_err(|error| AppError::from(format!("building an HTTP client failed; [error={error}]")))?;

        Ok(ReqwestHttpFetch { client })
    }
}

#[cfg(not(target_os = "android"))] // Android's system trust store is reachable only through JNI
fn with_trust_roots(builder: ClientBuilder) -> Result<ClientBuilder, AppError> {
    Ok(builder)
}

#[cfg(target_os = "android")] // every other target verifies against the system trust store through native-tls
fn with_trust_roots(builder: ClientBuilder) -> Result<ClientBuilder, AppError> {
    let mut root_certificates: Vec<reqwest::Certificate> = Vec::with_capacity(webpki_root_certs::TLS_SERVER_ROOT_CERTS.len());

    for root_certificate_der in webpki_root_certs::TLS_SERVER_ROOT_CERTS {
        let root_certificate: reqwest::Certificate = reqwest::Certificate::from_der(root_certificate_der.as_ref())
            .map_err(|error| AppError::from(format!("reading a root certificate failed; [error={error}]")))?;
        root_certificates.push(root_certificate);
    }

    Ok(builder.tls_certs_only(root_certificates))
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
