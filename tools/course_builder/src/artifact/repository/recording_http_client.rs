use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use aws_sdk_s3::Client;
use aws_sdk_s3::config::http::{HttpRequest, HttpResponse};
use aws_sdk_s3::config::retry::RetryConfig;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, HttpClient, IntoShared, Region, RuntimeComponents};
use aws_sdk_s3::primitives::SdkBody;
use aws_smithy_runtime_api::client::http::{HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpConnector};
use aws_smithy_runtime_api::http::StatusCode;

use crate::artifact::repository::cloudflare_r2_artifact_repository::CloudflareR2ArtifactRepository;

const RECORDED_ENDPOINT_URL: &str = "https://r2.recorded.invalid";
const RECORDED_BUCKET: &str = "language-recorded";
const RECORDED_PUBLIC_BASE_URL: &str = "https://artifacts.recorded.invalid";

#[derive(Debug, Clone)]
pub struct RecordedRequest {
    pub method: String,
    pub uri: String,
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CannedResponse {
    pub status: u16,
    pub entity_tag: Option<String>,
    pub body: String,
}

impl CannedResponse {
    pub fn empty(status: u16) -> CannedResponse {
        CannedResponse {
            status,
            entity_tag: None,
            body: String::new(),
        }
    }

    pub fn s3_error(status: u16, code: &str) -> CannedResponse {
        CannedResponse {
            status,
            entity_tag: None,
            body: format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Error><Code>{code}</Code><Message>{code}</Message></Error>"),
        }
    }

    pub fn object(entity_tag: &str, body: &str) -> CannedResponse {
        CannedResponse {
            status: 200,
            entity_tag: Some(entity_tag.to_string()),
            body: body.to_string(),
        }
    }
}

/// An S3 transport that records each request and answers with the next canned response, in order.
#[derive(Debug, Clone)]
pub struct RecordingHttpClient {
    recorded_requests: Arc<Mutex<Vec<RecordedRequest>>>,
    canned_responses: Arc<Mutex<VecDeque<CannedResponse>>>,
}

impl RecordingHttpClient {
    pub fn answering(canned_responses: Vec<CannedResponse>) -> RecordingHttpClient {
        RecordingHttpClient {
            recorded_requests: Arc::new(Mutex::new(Vec::new())),
            canned_responses: Arc::new(Mutex::new(VecDeque::from(canned_responses))),
        }
    }

    pub fn recorded_requests(&self) -> Vec<RecordedRequest> {
        self.recorded_requests.lock().unwrap().clone()
    }

    pub fn repository(&self) -> CloudflareR2ArtifactRepository {
        let config: aws_sdk_s3::Config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new("auto"))
            .credentials_provider(Credentials::new("recorded-access-key", "recorded-secret", None, None, "recorded"))
            .endpoint_url(RECORDED_ENDPOINT_URL)
            .force_path_style(true)
            .retry_config(RetryConfig::disabled())
            .http_client(self.clone())
            .build();

        CloudflareR2ArtifactRepository::from_client(Client::from_conf(config), RECORDED_BUCKET.to_string(), RECORDED_PUBLIC_BASE_URL.to_string())
    }
}

impl HttpConnector for RecordingHttpClient {
    fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
        let recorded_request: RecordedRequest = RecordedRequest {
            method: request.method().to_string(),
            uri: request.uri().to_string(),
            if_match: request.headers().get("If-Match").map(str::to_string),
            if_none_match: request.headers().get("If-None-Match").map(str::to_string),
        };
        let canned_response: Option<CannedResponse> = self.canned_responses.lock().unwrap().pop_front();
        let Some(canned_response) = canned_response
        else {
            panic!("a request no canned response answers; [method={} uri={}]", recorded_request.method, recorded_request.uri);
        };

        self.recorded_requests.lock().unwrap().push(recorded_request);

        let mut response: HttpResponse = HttpResponse::new(StatusCode::try_from(canned_response.status).unwrap(), SdkBody::from(canned_response.body));

        if let Some(entity_tag) = canned_response.entity_tag {
            response.headers_mut().insert("ETag", entity_tag);
        }

        HttpConnectorFuture::ready(Ok(response))
    }
}

impl HttpClient for RecordingHttpClient {
    fn http_connector(&self, _settings: &HttpConnectorSettings, _components: &RuntimeComponents) -> SharedHttpConnector {
        self.clone().into_shared()
    }
}
