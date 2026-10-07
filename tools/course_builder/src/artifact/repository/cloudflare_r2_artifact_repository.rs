use std::collections::BTreeSet;
use std::path::Path;

use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
use aws_sdk_s3::primitives::ByteStream;
use shared::AppError;
use shared::error::render_error_chain;

use crate::artifact::repository::artifact_repository::{ArtifactRepository, ObjectHeaders};

/* R2 ignores the region but the S3 client requires one; "auto" is the value Cloudflare documents. */
const R2_REGION_PLACEHOLDER: &str = "auto";
const R2_CREDENTIALS_PROVIDER_NAME: &str = "portuguese-course-r2";

pub const ENV_R2_ACCOUNT_ID: &str = "R2_ACCOUNT_ID";
pub const ENV_R2_ARTIFACT_BUCKET: &str = "R2_ARTIFACT_BUCKET";
pub const ENV_R2_ARTIFACT_PUBLIC_BASE_URL: &str = "R2_ARTIFACT_PUBLIC_BASE_URL";
pub const ENV_R2_PUBLISH_ACCESS_KEY_ID: &str = "R2_PUBLISH_ACCESS_KEY_ID";
pub const SECRET_R2_PUBLISH_SECRET_ACCESS_KEY: &str = "cloudflare.r2.publish.secret_access_key";

pub struct CloudflareR2Config {
    pub account_id: String,
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
        let endpoint_url: String = format!("https://{}.r2.cloudflarestorage.com", config.account_id);

        let sdk_config: aws_config::SdkConfig = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(endpoint_url)
            .credentials_provider(credentials)
            .region(Region::new(R2_REGION_PLACEHOLDER))
            .load()
            .await;

        CloudflareR2ArtifactRepository {
            client: Client::new(&sdk_config),
            bucket: config.bucket,
            public_base_url: config.public_base_url,
        }
    }
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

    async fn existing_keys(&self, prefix: &str) -> Result<BTreeSet<String>, AppError> {
        let mut keys: BTreeSet<String> = BTreeSet::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let page: ListObjectsV2Output = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix)
                .set_continuation_token(continuation_token.clone())
                .send()
                .await
                .map_err(|error| {
                    AppError::from(format!("list_objects_v2 failed; [bucket={} prefix={prefix} error={}]", self.bucket, render_error_chain(&error)))
                })?;

            keys.extend(page.contents().iter().filter_map(|object| object.key().map(str::to_string)));
            continuation_token = page.next_continuation_token().map(str::to_string);

            if continuation_token.is_none() {
                return Ok(keys);
            }
        }
    }

    fn url(&self, key: &str) -> String {
        format!("{}/{key}", self.public_base_url.trim_end_matches('/'))
    }
}
