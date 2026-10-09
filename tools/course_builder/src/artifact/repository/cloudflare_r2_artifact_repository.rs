use std::path::Path;

use aws_sdk_s3::Client;
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_bucket_cors::GetBucketCorsOutput;
use aws_sdk_s3::operation::get_object::{GetObjectError, GetObjectOutput};
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
use aws_sdk_s3::operation::put_object::builders::PutObjectFluentBuilder;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::{AggregatedBytes, ByteStream};
use aws_sdk_s3::types::{CorsConfiguration, CorsRule, Object};
use chrono::{DateTime, Utc};
use shared::AppError;
use shared::error::render_error_chain;

use crate::artifact::repository::artifact_repository::{
    self, ArtifactRepository, ListedObject, ObjectHeaders, StoredObject, WriteConditionKind, WriteOutcomeKind,
};
use crate::artifact::repository::cors_policy::CorsPolicy;

/* R2 ignores the region but the S3 client requires one; "auto" is the value Cloudflare documents. */
const R2_REGION_PLACEHOLDER: &str = "auto";
const R2_CREDENTIALS_PROVIDER_NAME: &str = "language-r2";

const HTTP_STATUS_NOT_FOUND: u16 = 404;
const HTTP_STATUS_PRECONDITION_FAILED: u16 = 412;
const ENTITY_TAG_MATCHES_ANY: &str = "*";

pub const ENV_R2_ACCOUNT_ID: &str = "R2_ACCOUNT_ID";
pub const ENV_R2_ARTIFACT_BUCKET: &str = "R2_ARTIFACT_BUCKET";
pub const ENV_R2_ARTIFACT_PUBLIC_BASE_URL: &str = "R2_ARTIFACT_PUBLIC_BASE_URL";
pub const ENV_R2_PUBLISH_ACCESS_KEY_ID: &str = "R2_PUBLISH_ACCESS_KEY_ID";
pub const ENV_R2_JURISDICTION: &str = "R2_JURISDICTION";
pub const SECRET_R2_PUBLISH_SECRET_ACCESS_KEY: &str = "cloudflare.r2.publish.secret_access_key";

pub struct CloudflareR2Config {
    pub account_id: String,
    /// The bucket's jurisdiction (`eu`, `fedramp`, `us`); `None` for a bucket created without one.
    pub jurisdiction: Option<String>,
    pub bucket: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub public_base_url: String,
}

pub struct CloudflareR2ArtifactRepository {
    client: Client,
    bucket: String,
    public_base_url: String,
}

impl CloudflareR2ArtifactRepository {
    pub async fn create(config: CloudflareR2Config) -> CloudflareR2ArtifactRepository {
        let credentials: Credentials =
            Credentials::new(config.access_key_id, config.secret_access_key, None, None, R2_CREDENTIALS_PROVIDER_NAME);
        let endpoint_url: String = r2_endpoint_url(&config.account_id, config.jurisdiction.as_deref());

        let sdk_config: aws_config::SdkConfig = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(endpoint_url)
            .credentials_provider(credentials)
            .region(Region::new(R2_REGION_PLACEHOLDER))
            .load()
            .await;

        CloudflareR2ArtifactRepository::from_client(Client::new(&sdk_config), config.bucket, config.public_base_url)
    }

    pub fn from_client(client: Client, bucket: String, public_base_url: String) -> CloudflareR2ArtifactRepository {
        CloudflareR2ArtifactRepository {
            client,
            bucket,
            public_base_url,
        }
    }
}

/* A bucket with a jurisdiction answers only on that jurisdiction's endpoint, and is absent from the default one. */
pub fn r2_endpoint_url(account_id: &str, jurisdiction: Option<&str>) -> String {
    match jurisdiction {
        Some(jurisdiction) => format!("https://{account_id}.{jurisdiction}.r2.cloudflarestorage.com"),
        None => format!("https://{account_id}.r2.cloudflarestorage.com"),
    }
}

impl CloudflareR2ArtifactRepository {
    /// Replaces the bucket's CORS configuration with `policy`; needs a token with bucket admin permission.
    pub async fn apply_cors_policy(&self, policy: &CorsPolicy) -> Result<(), AppError> {
        let configuration: CorsConfiguration = cors_configuration(policy)?;

        self.client
            .put_bucket_cors()
            .bucket(&self.bucket)
            .cors_configuration(configuration)
            .send()
            .await
            .map_err(|error| AppError::from(format!("put_bucket_cors failed; [bucket={} error={}]", self.bucket, render_error_chain(&error))))?;

        Ok(())
    }

    pub async fn read_cors_policies(&self) -> Result<Vec<CorsPolicy>, AppError> {
        let output: GetBucketCorsOutput = self
            .client
            .get_bucket_cors()
            .bucket(&self.bucket)
            .send()
            .await
            .map_err(|error| AppError::from(format!("get_bucket_cors failed; [bucket={} error={}]", self.bucket, render_error_chain(&error))))?;

        let policies: Vec<CorsPolicy> = output
            .cors_rules()
            .iter()
            .map(|rule| CorsPolicy {
                allowed_origins: rule.allowed_origins().to_vec(),
                allowed_methods: rule.allowed_methods().to_vec(),
                max_age_seconds: rule.max_age_seconds().unwrap_or_default(),
            })
            .collect();

        Ok(policies)
    }
}

fn cors_configuration(policy: &CorsPolicy) -> Result<CorsConfiguration, AppError> {
    let rule: CorsRule = CorsRule::builder()
        .set_allowed_origins(Some(policy.allowed_origins.clone()))
        .set_allowed_methods(Some(policy.allowed_methods.clone()))
        .max_age_seconds(policy.max_age_seconds)
        .build()
        .map_err(|error| AppError::from(format!("invalid CORS rule; [error={error}]")))?;

    CorsConfiguration::builder()
        .cors_rules(rule)
        .build()
        .map_err(|error| AppError::from(format!("invalid CORS configuration; [error={error}]")))
}

fn response_status<E>(error: &SdkError<E, HttpResponse>) -> Option<u16> {
    error.raw_response().map(|response| response.status().as_u16())
}

fn with_write_condition(request: PutObjectFluentBuilder, condition: WriteConditionKind) -> PutObjectFluentBuilder {
    match condition {
        WriteConditionKind::Unconditional => request,
        WriteConditionKind::Absent => request.if_none_match(ENTITY_TAG_MATCHES_ANY),
        WriteConditionKind::MatchesEntityTag(entity_tag) => request.if_match(entity_tag),
    }
}

fn listed_object(object: &Object) -> Result<ListedObject, AppError> {
    let Some(key) = object.key()
    else {
        return Err(AppError::from("list_objects_v2 returned an object without a key"));
    };
    let Some(last_modified) = object.last_modified()
    else {
        return Err(AppError::from(format!("list_objects_v2 returned an object without LastModified; [key={key}]")));
    };
    let Some(uploaded) = DateTime::<Utc>::from_timestamp(last_modified.secs(), last_modified.subsec_nanos())
    else {
        return Err(AppError::from(format!("list_objects_v2 returned an unrepresentable LastModified; [key={key}]")));
    };

    Ok(ListedObject {
        key: key.to_string(),
        uploaded,
    })
}

impl ArtifactRepository for CloudflareR2ArtifactRepository {
    async fn put_file(&self, key: &str, source_path: &Path, headers: ObjectHeaders) -> Result<(), AppError> {
        let body: ByteStream = ByteStream::from_path(source_path)
            .await
            .map_err(|error| AppError::from(format!("could not read a file to upload; [path={} error={error}]", source_path.display())))?;

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .content_type(headers.content_type)
            .cache_control(headers.cache_control)
            .send()
            .await
            .map_err(|error| AppError::from(format!("put_object failed; [bucket={} key={key} error={}]", self.bucket, render_error_chain(&error))))?;

        Ok(())
    }

    async fn put_bytes(&self, key: &str, bytes: &[u8], headers: ObjectHeaders, condition: WriteConditionKind) -> Result<WriteOutcomeKind, AppError> {
        let request: PutObjectFluentBuilder = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(bytes.to_vec()))
            .content_type(headers.content_type)
            .cache_control(headers.cache_control);
        let sent: Result<PutObjectOutput, SdkError<PutObjectError, HttpResponse>> = with_write_condition(request, condition).send().await;

        match sent {
            Ok(_) => Ok(WriteOutcomeKind::Written),
            Err(error) if response_status(&error) == Some(HTTP_STATUS_PRECONDITION_FAILED) => Ok(WriteOutcomeKind::PreconditionFailed),
            Err(error) => Err(AppError::from(format!("put_object failed; [bucket={} key={key} error={}]", self.bucket, render_error_chain(&error)))),
        }
    }

    async fn get_object(&self, key: &str) -> Result<Option<StoredObject>, AppError> {
        let sent: Result<GetObjectOutput, SdkError<GetObjectError, HttpResponse>> =
            self.client.get_object().bucket(&self.bucket).key(key).send().await;
        let output: GetObjectOutput = match sent {
            Ok(output) => output,
            Err(error) if response_status(&error) == Some(HTTP_STATUS_NOT_FOUND) => return Ok(None),
            Err(error) => {
                return Err(AppError::from(format!("get_object failed; [bucket={} key={key} error={}]", self.bucket, render_error_chain(&error))));
            },
        };
        let entity_tag: Option<String> = output.e_tag().map(str::to_string);
        let body: AggregatedBytes = output
            .body
            .collect()
            .await
            .map_err(|error| AppError::from(format!("reading an object's body failed; [bucket={} key={key} error={error}]", self.bucket)))?;

        let Some(entity_tag) = entity_tag
        else {
            return Err(AppError::from(format!("get_object returned no ETag; [bucket={} key={key}]", self.bucket)));
        };

        Ok(Some(StoredObject {
            bytes: body.to_vec(),
            entity_tag,
        }))
    }

    /// The upload time is the object's `LastModified`.
    async fn list_objects(&self, directory_prefix: &str) -> Result<Vec<ListedObject>, AppError> {
        artifact_repository::require_directory_prefix(directory_prefix)?;

        let mut listed_objects: Vec<ListedObject> = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let page: ListObjectsV2Output = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(directory_prefix)
                .set_continuation_token(continuation_token.clone())
                .send()
                .await
                .map_err(|error| {
                    AppError::from(format!(
                        "list_objects_v2 failed; [bucket={} prefix={directory_prefix} error={}]",
                        self.bucket,
                        render_error_chain(&error),
                    ))
                })?;

            for object in page.contents() {
                listed_objects.push(listed_object(object)?);
            }

            continuation_token = page.next_continuation_token().map(str::to_string);

            if continuation_token.is_none() {
                return Ok(listed_objects);
            }
        }
    }

    async fn delete_keys(&self, keys: &[String]) -> Result<(), AppError> {
        for key in keys {
            self.client
                .delete_object()
                .bucket(&self.bucket)
                .key(key)
                .send()
                .await
                .map_err(|error| AppError::from(format!("delete_object failed; [bucket={} key={key} error={}]", self.bucket, render_error_chain(&error))))?;
        }

        Ok(())
    }

    fn url(&self, key: &str) -> String {
        format!("{}/{key}", self.public_base_url.trim_end_matches('/'))
    }
}

#[cfg(test)]
mod tests {
    use shared::artifact::repository_key;

    use super::*;
    use crate::artifact::repository::recording_http_client::{CannedResponse, RecordedRequest, RecordingHttpClient};

    const INDEX_KEY: &str = "course-index.json";
    const INDEX_HEADERS: ObjectHeaders = ObjectHeaders {
        content_type: artifact_repository::CONTENT_TYPE_MANIFEST,
        cache_control: repository_key::CACHE_CONTROL_POINTER,
    };

    #[test]
    fn r2_endpoint_url_names_the_jurisdiction_only_when_the_bucket_has_one() {
        assert_eq!(r2_endpoint_url("abc", None), "https://abc.r2.cloudflarestorage.com");
        assert_eq!(r2_endpoint_url("abc", Some("us")), "https://abc.us.r2.cloudflarestorage.com");
    }

    #[tokio::test]
    async fn put_bytes_sends_if_none_match_for_an_absent_condition_and_reads_412_as_a_failed_precondition() {
        let http_client: RecordingHttpClient = RecordingHttpClient::answering(vec![CannedResponse::s3_error(412, "PreconditionFailed")]);
        let repository: CloudflareR2ArtifactRepository = http_client.repository();

        let outcome: WriteOutcomeKind = repository.put_bytes(INDEX_KEY, b"{}", INDEX_HEADERS, WriteConditionKind::Absent).await.unwrap();
        let recorded_requests: Vec<RecordedRequest> = http_client.recorded_requests();

        assert_eq!(outcome, WriteOutcomeKind::PreconditionFailed);
        assert_eq!(recorded_requests.len(), 1);
        assert_eq!(recorded_requests[0].method, "PUT");
        assert_eq!(recorded_requests[0].if_none_match.as_deref(), Some("*"));
        assert_eq!(recorded_requests[0].if_match, None);
    }

    #[tokio::test]
    async fn put_bytes_sends_if_match_with_the_entity_tag_read_earlier() {
        let http_client: RecordingHttpClient = RecordingHttpClient::answering(vec![CannedResponse::empty(200)]);
        let repository: CloudflareR2ArtifactRepository = http_client.repository();

        let outcome: WriteOutcomeKind = repository
            .put_bytes(INDEX_KEY, b"{}", INDEX_HEADERS, WriteConditionKind::MatchesEntityTag("\"index-tag\"".to_string()))
            .await
            .unwrap();
        let recorded_requests: Vec<RecordedRequest> = http_client.recorded_requests();

        assert_eq!(outcome, WriteOutcomeKind::Written);
        assert_eq!(recorded_requests[0].if_match.as_deref(), Some("\"index-tag\""));
        assert_eq!(recorded_requests[0].if_none_match, None);
    }

    #[tokio::test]
    async fn put_bytes_sends_no_condition_when_unconditional() {
        let http_client: RecordingHttpClient = RecordingHttpClient::answering(vec![CannedResponse::empty(200)]);
        let repository: CloudflareR2ArtifactRepository = http_client.repository();

        repository.put_bytes(INDEX_KEY, b"{}", INDEX_HEADERS, WriteConditionKind::Unconditional).await.unwrap();
        let recorded_requests: Vec<RecordedRequest> = http_client.recorded_requests();

        assert_eq!(recorded_requests[0].if_match, None);
        assert_eq!(recorded_requests[0].if_none_match, None);
    }

    #[tokio::test]
    async fn put_bytes_reports_any_other_failure_as_an_error() {
        let http_client: RecordingHttpClient = RecordingHttpClient::answering(vec![CannedResponse::s3_error(403, "AccessDenied")]);
        let repository: CloudflareR2ArtifactRepository = http_client.repository();

        let written: Result<WriteOutcomeKind, AppError> = repository.put_bytes(INDEX_KEY, b"{}", INDEX_HEADERS, WriteConditionKind::Absent).await;

        assert!(written.is_err());
    }

    #[tokio::test]
    async fn get_object_reads_404_as_absent_and_returns_the_entity_tag_otherwise() {
        let http_client: RecordingHttpClient =
            RecordingHttpClient::answering(vec![CannedResponse::s3_error(404, "NoSuchKey"), CannedResponse::object("\"index-tag\"", "{}")]);
        let repository: CloudflareR2ArtifactRepository = http_client.repository();

        let absent_object: Option<StoredObject> = repository.get_object(INDEX_KEY).await.unwrap();
        let present_object: Option<StoredObject> = repository.get_object(INDEX_KEY).await.unwrap();

        assert_eq!(absent_object, None);
        assert_eq!(
            present_object,
            Some(StoredObject {
                bytes: b"{}".to_vec(),
                entity_tag: "\"index-tag\"".to_string(),
            }),
        );
    }

    #[tokio::test]
    async fn list_objects_sends_the_directory_prefix_and_reads_last_modified() {
        let listing: String = concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
            "<ListBucketResult xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">",
            "<Name>language-recorded</Name><Prefix>courses/pt_br.en.foundations/2026-10-08+charlotte/</Prefix>",
            "<KeyCount>1</KeyCount><MaxKeys>1000</MaxKeys><IsTruncated>false</IsTruncated>",
            "<Contents><Key>courses/pt_br.en.foundations/2026-10-08+charlotte/manifest.json</Key>",
            "<LastModified>2026-10-08T14:02:11.000Z</LastModified><ETag>\"manifest-tag\"</ETag><Size>2</Size>",
            "<StorageClass>STANDARD</StorageClass></Contents>",
            "</ListBucketResult>",
        )
        .to_string();
        let http_client: RecordingHttpClient = RecordingHttpClient::answering(vec![CannedResponse {
            status: 200,
            entity_tag: None,
            body: listing,
        }]);
        let repository: CloudflareR2ArtifactRepository = http_client.repository();

        let listed_objects: Vec<ListedObject> = repository.list_objects("courses/pt_br.en.foundations/2026-10-08+charlotte/").await.unwrap();
        let recorded_requests: Vec<RecordedRequest> = http_client.recorded_requests();

        assert!(recorded_requests[0].uri.contains("prefix=courses%2Fpt_br.en.foundations%2F2026-10-08%2Bcharlotte%2F"));
        assert_eq!(
            listed_objects,
            vec![ListedObject {
                key: "courses/pt_br.en.foundations/2026-10-08+charlotte/manifest.json".to_string(),
                uploaded: DateTime::parse_from_rfc3339("2026-10-08T14:02:11Z").unwrap().with_timezone(&Utc),
            }],
        );
    }

    #[tokio::test]
    async fn list_objects_refuses_a_prefix_without_a_trailing_separator_before_any_request() {
        let http_client: RecordingHttpClient = RecordingHttpClient::answering(Vec::new());
        let repository: CloudflareR2ArtifactRepository = http_client.repository();

        let listed: Result<Vec<ListedObject>, AppError> = repository.list_objects("courses/pt_br.en.foundations/2026-10-08+charlotte").await;

        assert!(listed.is_err());
        assert!(http_client.recorded_requests().is_empty());
    }
}
