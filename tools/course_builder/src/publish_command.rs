use std::path::PathBuf;

use shared::AppError;
use shared::artifact::{Manifest, discovery};

use crate::artifact::repository::{
    ArtifactRepositoryKind, CloudflareR2ArtifactRepository, CloudflareR2Config, DryArtifactRepository, ENV_LOCAL_REPOSITORY_ROOT,
    ENV_R2_ACCOUNT_ID, ENV_R2_ARTIFACT_BUCKET, ENV_R2_ARTIFACT_PUBLIC_BASE_URL, ENV_R2_PUBLISH_ACCESS_KEY_ID,
    LocalArtifactRepository, SECRET_R2_PUBLISH_SECRET_ACCESS_KEY,
};
use crate::artifact::{self, PublishReport};
use crate::build_command;
use crate::secrets;
use crate::workspace::WorkspacePaths;

pub enum PublishTargetKind {
    Local {
        root: Option<PathBuf>,
        public_base_url: Option<String>,
    },
    CloudflareR2,
    Dry,
}

pub fn run_publish(paths: &WorkspacePaths, target: PublishTargetKind, bundle: Option<PathBuf>) -> Result<bool, AppError> {
    let bundle_directory: PathBuf = match bundle {
        Some(bundle_directory) => bundle_directory,
        None => artifact::find_newest_bundle(paths)?,
    };
    let bundle_manifest: Manifest = artifact::read_bundle_manifest(&bundle_directory)?;

    check_test_courses_publishable(&target, &bundle_manifest)?;

    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let report: PublishReport = runtime.block_on(async {
        let repository: ArtifactRepositoryKind = create_repository(paths, target).await?;

        artifact::publish_bundle(&bundle_directory, &repository).await
    })?;

    log::info!(
        "publish complete; [version={} latest={} shards={} recordings_uploaded={} recordings_already_present={}]",
        report.version,
        report.latest_manifest_url,
        report.shards_published,
        report.audio_published,
        report.audio_already_present,
    );

    Ok(true)
}

/// Cloudflare R2 is the production repository, so a bundle with a test course is refused there.
pub fn check_test_courses_publishable(target: &PublishTargetKind, bundle_manifest: &Manifest) -> Result<(), AppError> {
    let test_course_ids: Vec<&str> = bundle_manifest.test_course_ids();

    match target {
        PublishTargetKind::CloudflareR2 if !test_course_ids.is_empty() => Err(AppError::from(format!(
            "a bundle with a test course cannot be published to Cloudflare R2; rebuild without --{}; [version={} test_courses={}]",
            build_command::INCLUDE_TEST_COURSES_FLAG,
            bundle_manifest.version,
            test_course_ids.join(","),
        ))),
        PublishTargetKind::CloudflareR2 | PublishTargetKind::Local { .. } | PublishTargetKind::Dry => Ok(()),
    }
}

/// The R2 repository named by `.env`, with the secret access key from `secrets.yaml`.
pub async fn create_cloudflare_r2_repository() -> Result<CloudflareR2ArtifactRepository, AppError> {
    let config: CloudflareR2Config = CloudflareR2Config {
        account_id: secrets::read_environment(ENV_R2_ACCOUNT_ID)?,
        bucket: secrets::read_environment(ENV_R2_ARTIFACT_BUCKET)?,
        access_key_id: secrets::read_environment(ENV_R2_PUBLISH_ACCESS_KEY_ID)?,
        secret_access_key: secrets::master_decrypt_utf8(SECRET_R2_PUBLISH_SECRET_ACCESS_KEY)?,
        public_base_url: secrets::read_environment(ENV_R2_ARTIFACT_PUBLIC_BASE_URL)?,
    };

    Ok(CloudflareR2ArtifactRepository::create(config).await)
}

async fn create_repository(paths: &WorkspacePaths, target: PublishTargetKind) -> Result<ArtifactRepositoryKind, AppError> {
    match target {
        PublishTargetKind::Local { root, public_base_url } => {
            let root: PathBuf = root
                .or_else(|| dotenvy::var(ENV_LOCAL_REPOSITORY_ROOT).ok().map(PathBuf::from))
                .unwrap_or_else(|| paths.local_repository_directory.clone());
            let public_base_url: String = public_base_url.unwrap_or_else(|| discovery::STATIC_REPOSITORY_BASE_URL.to_string());

            Ok(ArtifactRepositoryKind::Local(LocalArtifactRepository::new(root, public_base_url)))
        },
        PublishTargetKind::CloudflareR2 => Ok(ArtifactRepositoryKind::CloudflareR2(create_cloudflare_r2_repository().await?)),
        PublishTargetKind::Dry => Ok(ArtifactRepositoryKind::Dry(DryArtifactRepository::new())),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::Utc;
    use shared::artifact::{ManifestCourse, ManifestEntry, manifest};

    use super::*;

    const TEACHING_COURSE_ID: &str = "pt_br.en.teaching";
    const TEST_COURSE_ID: &str = "pt_br.en.testing";

    fn manifest_course(course_id: &str, test_course: bool) -> ManifestCourse {
        ManifestCourse {
            title: course_id.to_string(),
            target_language: "pt-BR".to_string(),
            source_language: "en".to_string(),
            outline: ManifestEntry {
                relative_path: format!("{}/{course_id}/course.sqlite.br", manifest::SUBDIR_DATA),
                size_bytes: 0,
                sha256: "0".repeat(64),
            },
            units: BTreeMap::new(),
            test_course,
        }
    }

    fn manifest_with_courses(course_test_flags: &[(&str, bool)]) -> Manifest {
        Manifest {
            manifest_schema_version: manifest::MANIFEST_SCHEMA_VERSION,
            version: "2026-10-08+Springfield".to_string(),
            artifact_created: Utc::now(),
            courses: course_test_flags
                .iter()
                .map(|(course_id, test_course)| (course_id.to_string(), manifest_course(course_id, *test_course)))
                .collect(),
            voice_pipeline_revisions: BTreeMap::new(),
        }
    }

    fn local_target() -> PublishTargetKind {
        PublishTargetKind::Local {
            root: None,
            public_base_url: None,
        }
    }

    #[test]
    fn check_test_courses_publishable_refuses_a_test_course_for_cloudflare_r2() {
        let bundle_manifest: Manifest = manifest_with_courses(&[(TEACHING_COURSE_ID, false), (TEST_COURSE_ID, true)]);

        let checked: Result<(), AppError> = check_test_courses_publishable(&PublishTargetKind::CloudflareR2, &bundle_manifest);

        assert!(checked.unwrap_err().to_string().contains(TEST_COURSE_ID));
    }

    #[test]
    fn check_test_courses_publishable_accepts_a_test_course_locally_and_in_a_dry_run() {
        let bundle_manifest: Manifest = manifest_with_courses(&[(TEACHING_COURSE_ID, false), (TEST_COURSE_ID, true)]);

        assert!(check_test_courses_publishable(&local_target(), &bundle_manifest).is_ok());
        assert!(check_test_courses_publishable(&PublishTargetKind::Dry, &bundle_manifest).is_ok());
    }

    #[test]
    fn check_test_courses_publishable_accepts_a_bundle_without_test_courses_for_cloudflare_r2() {
        let bundle_manifest: Manifest = manifest_with_courses(&[(TEACHING_COURSE_ID, false)]);

        assert!(check_test_courses_publishable(&PublishTargetKind::CloudflareR2, &bundle_manifest).is_ok());
    }
}
