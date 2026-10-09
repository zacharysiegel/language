use std::path::PathBuf;

use shared::AppError;
use shared::artifact::{CourseManifest, discovery};

use crate::artifact::repository::{
    self, ArtifactRepositoryKind, CloudflareR2ArtifactRepository, CloudflareR2Config, DryArtifactRepository, ENV_LOCAL_REPOSITORY_ROOT,
    ENV_R2_ACCOUNT_ID, ENV_R2_ARTIFACT_BUCKET, ENV_R2_ARTIFACT_PUBLIC_BASE_URL, ENV_R2_PUBLISH_ACCESS_KEY_ID, LocalArtifactRepository,
    RepositoryLockKind, SECRET_R2_PUBLISH_SECRET_ACCESS_KEY,
};
use crate::artifact::{self, CourseIndexChangeKind, CoursePublishOutcome, CourseSelectionKind, PointerChangeKind, PublicationUploadKind, RepositoryTargetKind, UnlistedCourseInclusionKind};
use crate::secrets;
use crate::workspace::WorkspacePaths;

pub const COURSE_FLAG: &str = "course";
pub const INCLUDE_UNLISTED_COURSES_FLAG: &str = "include-unlisted-courses";

/// A repository a command reads or writes, with what it takes to reach it.
pub enum RepositoryLocationKind {
    Local {
        root: Option<PathBuf>,
        public_base_url: Option<String>,
    },
    CloudflareR2,
    Dry,
}

impl RepositoryLocationKind {
    pub fn target(&self) -> RepositoryTargetKind {
        match self {
            RepositoryLocationKind::Local { .. } => RepositoryTargetKind::Local,
            RepositoryLocationKind::CloudflareR2 => RepositoryTargetKind::CloudflareR2,
            RepositoryLocationKind::Dry => RepositoryTargetKind::Dry,
        }
    }
}

pub struct PublishOptions {
    pub course_ids: Vec<String>,
    pub bundle: Option<PathBuf>,
    pub unlisted_course_inclusion: UnlistedCourseInclusionKind,
}

pub fn run_publish(paths: &WorkspacePaths, location: RepositoryLocationKind, options: PublishOptions) -> Result<bool, AppError> {
    let selection: CourseSelectionKind = select_courses(options)?;
    let target: RepositoryTargetKind = location.target();
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let outcomes: Vec<CoursePublishOutcome> = runtime.block_on(async {
        let repository: ArtifactRepositoryKind = create_repository(paths, location).await?;
        let lock: RepositoryLockKind = repository::acquire_repository_lock(&repository).await?;

        let published: Result<Vec<CoursePublishOutcome>, AppError> = artifact::publish_selected_courses(paths, &repository, target, &selection).await;
        let released: Result<(), AppError> = lock.release(&repository).await;

        repository::combine_with_release(published, released)
    })?;

    if target != RepositoryTargetKind::Dry {
        artifact::drop_embedded_courses_not_taught_in_the_data(paths)?;
    }

    Ok(report_outcomes(&outcomes))
}

/// `--bundle` names one course's bundle; `--course` names courses; neither publishes every course.
pub fn select_courses(options: PublishOptions) -> Result<CourseSelectionKind, AppError> {
    let Some(bundle_directory) = options.bundle else {
        if options.course_ids.is_empty() {
            return Ok(CourseSelectionKind::EveryCourse(options.unlisted_course_inclusion));
        }

        let mut course_ids: Vec<String> = options.course_ids;

        course_ids.sort();
        course_ids.dedup();

        return Ok(CourseSelectionKind::Named(course_ids));
    };
    let bundle_manifest: CourseManifest = artifact::read_bundle_manifest(&bundle_directory)?;
    let named_another_course: bool = options.course_ids.iter().any(|course_id| *course_id != bundle_manifest.course_id);

    if named_another_course {
        return Err(AppError::from(format!(
            "--bundle names a bundle of another course than --{COURSE_FLAG}; [bundle_course_id={} course_ids={}]",
            bundle_manifest.course_id,
            options.course_ids.join(","),
        )));
    }

    Ok(CourseSelectionKind::Bundle(bundle_directory))
}

/// `true` when every course published.
fn report_outcomes(outcomes: &[CoursePublishOutcome]) -> bool {
    let mut failed_course_ids: Vec<&str> = Vec::new();

    for outcome in outcomes {
        let Ok(report) = &outcome.result else {
            failed_course_ids.push(&outcome.course_id);
            continue;
        };

        log::info!(
            "published a course; [course_id={} publication_label={} latest={} upload={} pointer={} index={} shards_uploaded={} recordings_uploaded={} recordings_already_present={}]",
            report.course_id,
            report.publication_label,
            report.latest_manifest_url,
            describe_upload(report.upload),
            describe_pointer_change(report.pointer),
            describe_index_change(report.index_change),
            report.shards_uploaded,
            report.recordings_uploaded,
            report.recordings_already_present,
        );
    }

    if !failed_course_ids.is_empty() {
        eprintln!("publishing failed for some courses; [courses={} failed={}]", outcomes.len(), failed_course_ids.join(","));
        return false;
    }

    log::info!("publish finished; [courses={}]", outcomes.len());

    true
}

fn describe_upload(upload: PublicationUploadKind) -> &'static str {
    match upload {
        PublicationUploadKind::Uploaded => "uploaded",
        PublicationUploadKind::AlreadyPresent => "already_present",
    }
}

fn describe_pointer_change(pointer: PointerChangeKind) -> &'static str {
    match pointer {
        PointerChangeKind::Moved => "moved",
        PointerChangeKind::AlreadyCurrent => "already_current",
        PointerChangeKind::LeftOnAnotherPublication => "left_on_another_publication",
    }
}

fn describe_index_change(index_change: CourseIndexChangeKind) -> &'static str {
    match index_change {
        CourseIndexChangeKind::Unchanged => "unchanged",
        CourseIndexChangeKind::Added => "added",
        CourseIndexChangeKind::Updated => "updated",
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

pub async fn create_repository(paths: &WorkspacePaths, location: RepositoryLocationKind) -> Result<ArtifactRepositoryKind, AppError> {
    match location {
        RepositoryLocationKind::Local { root, public_base_url } => {
            let root: PathBuf = root
                .or_else(|| dotenvy::var(ENV_LOCAL_REPOSITORY_ROOT).ok().map(PathBuf::from))
                .unwrap_or_else(|| paths.local_repository_directory.clone());
            let public_base_url: String = public_base_url.unwrap_or_else(|| discovery::STATIC_REPOSITORY_BASE_URL.to_string());

            Ok(ArtifactRepositoryKind::Local(LocalArtifactRepository::new(root, public_base_url)))
        },
        RepositoryLocationKind::CloudflareR2 => Ok(ArtifactRepositoryKind::CloudflareR2(create_cloudflare_r2_repository().await?)),
        RepositoryLocationKind::Dry => Ok(ArtifactRepositoryKind::Dry(DryArtifactRepository::new())),
    }
}

#[cfg(test)]
mod tests {
    use shared::course::Course;

    use super::*;
    use crate::artifact::BuiltBundle;
    use crate::audio_plan::AudioPlan;
    use crate::test_courses::{self, FixtureWorkspace, SECOND_COURSE_ID, TEACHING_COURSE_ID};

    fn options(course_ids: &[&str], bundle: Option<PathBuf>, unlisted_course_inclusion: UnlistedCourseInclusionKind) -> PublishOptions {
        PublishOptions {
            course_ids: course_ids.iter().map(|course_id| course_id.to_string()).collect(),
            bundle,
            unlisted_course_inclusion,
        }
    }

    #[test]
    fn select_courses_publishes_every_course_without_a_course_or_a_bundle() {
        let selection: CourseSelectionKind = select_courses(options(&[], None, UnlistedCourseInclusionKind::Include)).unwrap();

        assert_eq!(selection, CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Include));
    }

    #[test]
    fn select_courses_names_each_course_once() {
        let selection: CourseSelectionKind =
            select_courses(options(&[SECOND_COURSE_ID, TEACHING_COURSE_ID, SECOND_COURSE_ID], None, UnlistedCourseInclusionKind::Exclude)).unwrap();

        assert_eq!(selection, CourseSelectionKind::Named(vec![SECOND_COURSE_ID.to_string(), TEACHING_COURSE_ID.to_string()]));
    }

    #[test]
    fn select_courses_refuses_a_bundle_of_another_course_than_the_one_named() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        let matching: CourseSelectionKind =
            select_courses(options(&[TEACHING_COURSE_ID], Some(bundle.directory.clone()), UnlistedCourseInclusionKind::Exclude)).unwrap();
        let mismatched: Result<CourseSelectionKind, AppError> =
            select_courses(options(&[SECOND_COURSE_ID], Some(bundle.directory.clone()), UnlistedCourseInclusionKind::Exclude));

        assert_eq!(matching, CourseSelectionKind::Bundle(bundle.directory.clone()));
        assert!(mismatched.is_err());
    }
}
