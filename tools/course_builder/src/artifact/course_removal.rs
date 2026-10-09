use std::collections::BTreeSet;

use shared::AppError;
use shared::artifact::repository_key;

use crate::artifact::course_index_update::{self, CourseIndexRemovalKind};
use crate::artifact::embedded_bundle::{self, EmbeddedCourseRemovalKind};
use crate::artifact::publish::RepositoryTargetKind;
use crate::artifact::repository::{self, ArtifactRepository};
use crate::artifact::retention;
use crate::workspace::WorkspacePaths;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseRemovalReport {
    pub index_removal: CourseIndexRemovalKind,
    pub deleted_object_count: usize,
    pub embedded_removal: EmbeddedCourseRemovalKind,
    pub recordings_removed: usize,
}

/// Takes the course out of the index first, so no client is directed to the files deleted after it.
pub async fn remove_course(
    paths: &WorkspacePaths,
    repository: &impl ArtifactRepository,
    target: RepositoryTargetKind,
    course_id: &str,
) -> Result<CourseRemovalReport, AppError> {
    let index_removal: CourseIndexRemovalKind = course_index_update::remove_course_index_entry(repository, course_id).await?;
    let latest_keys: BTreeSet<String> = repository::list_keys(repository, &repository_key::course_latest_directory_key(course_id)).await?;
    let latest_key_list: Vec<String> = latest_keys.into_iter().collect();

    repository.delete_keys(&latest_key_list).await?;

    let course_keys: BTreeSet<String> = repository::list_keys(repository, &repository_key::course_directory_key(course_id)).await?;
    let course_key_list: Vec<String> = course_keys.into_iter().collect();

    repository.delete_keys(&course_key_list).await?;

    let embedded_removal: EmbeddedCourseRemovalKind = match target {
        RepositoryTargetKind::Local | RepositoryTargetKind::CloudflareR2 => embedded_bundle::drop_embedded_course(paths, course_id)?,
        RepositoryTargetKind::Dry => EmbeddedCourseRemovalKind::NotEmbedded,
    };
    let recordings_removed: usize = match target {
        RepositoryTargetKind::Local => retention::remove_unused_recordings(repository).await?,
        RepositoryTargetKind::CloudflareR2 | RepositoryTargetKind::Dry => 0,
    };

    Ok(CourseRemovalReport {
        index_removal,
        deleted_object_count: latest_key_list.len() + course_key_list.len(),
        embedded_removal,
        recordings_removed,
    })
}

#[cfg(test)]
mod tests {
    use shared::artifact::CourseIndex;
    use shared::course::Course;

    use super::*;
    use crate::artifact::artifact_bundle::{self, BuiltBundle};
    use crate::artifact::course_index_update;
    use crate::artifact::publish::{self, CoursePublishOutcome, CourseSelectionKind, UnlistedCourseInclusionKind};
    use crate::artifact::repository::memory_artifact_repository::{MemoryArtifactRepository, RecordedOperationKind};
    use crate::audio_plan::AudioPlan;
    use crate::test_courses::{self, FixtureWorkspace, SECOND_COURSE_ID, TEACHING_COURSE_ID};

    #[tokio::test]
    async fn remove_course_writes_the_index_before_any_delete_and_an_every_course_publish_leaves_the_course_out() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let teaching_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);

        workspace.build_bundle(&second_course, &plan);
        publish::publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::Local,
            &CourseSelectionKind::Named(vec![TEACHING_COURSE_ID.to_string(), SECOND_COURSE_ID.to_string()]),
        )
        .await
        .unwrap();
        repository.clear_operations();

        let report: CourseRemovalReport = remove_course(&workspace.paths, &repository, RepositoryTargetKind::Local, TEACHING_COURSE_ID).await.unwrap();
        let removal_operations: Vec<RecordedOperationKind> = repository.operations();
        let teaching_directory_key: String = repository_key::course_directory_key(TEACHING_COURSE_ID);
        let teaching_recordings_left: bool = artifact_bundle::read_bundle_recordings(&teaching_bundle.directory)
            .unwrap()
            .recording_sha256s
            .iter()
            .any(|audio_sha256| repository.bytes_at(&repository_key::audio_key(audio_sha256)).is_some());

        let every_course_outcomes: Vec<CoursePublishOutcome> = publish::publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::Local,
            &CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Exclude),
        )
        .await
        .unwrap();
        let index_after: CourseIndex = course_index_update::read_course_index(&repository).await.unwrap().course_index;

        assert!(matches!(&removal_operations[0], RecordedOperationKind::Put { key, .. } if key == repository_key::COURSE_INDEX_KEY));
        assert!(removal_operations[1..].iter().any(|operation| matches!(operation, RecordedOperationKind::Delete { .. })));
        assert_eq!(report.index_removal, CourseIndexRemovalKind::Removed);
        assert_eq!(report.embedded_removal, EmbeddedCourseRemovalKind::Removed);
        assert!(repository.contents().keys().all(|key| !key.starts_with(&teaching_directory_key)));
        assert!(!teaching_recordings_left);
        assert!(!workspace.paths.embedded_course_directory(TEACHING_COURSE_ID).exists());
        assert_eq!(every_course_outcomes.iter().map(|outcome| outcome.course_id.as_str()).collect::<Vec<&str>>(), vec![SECOND_COURSE_ID]);
        assert_eq!(index_after.courses.keys().collect::<Vec<&String>>(), vec![SECOND_COURSE_ID]);
    }

    #[tokio::test]
    async fn remove_course_deletes_the_files_of_a_course_the_index_does_not_list() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let left_key: String = repository_key::publication_manifest_key(TEACHING_COURSE_ID, "2026-10-08+duluth");

        repository.insert(&left_key, b"{}");

        let report: CourseRemovalReport = remove_course(&workspace.paths, &repository, RepositoryTargetKind::CloudflareR2, TEACHING_COURSE_ID).await.unwrap();

        assert_eq!(report.index_removal, CourseIndexRemovalKind::NotListed);
        assert_eq!(report.deleted_object_count, 1);
        assert_eq!(repository.bytes_at(&left_key), None);
        assert_eq!(repository.bytes_at(repository_key::COURSE_INDEX_KEY), None);
    }
}
