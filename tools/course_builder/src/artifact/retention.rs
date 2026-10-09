use std::collections::BTreeSet;

use chrono::{DateTime, TimeDelta, Utc};
use shared::AppError;
use shared::artifact::{manifest, repository_key};

use crate::artifact::pointer_publication::{self, PointerPublication};
use crate::artifact::recordings_list::{self, RecordingsList};
use crate::artifact::repository::{self, ArtifactRepository, ListedObject, StoredObject};

pub const PUBLICATIONS_KEPT_PER_COURSE: usize = 2;
pub const PUBLICATION_GRACE_PERIOD: TimeDelta = TimeDelta::days(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationGraceKind {
    None,
    /// A publication is kept while its next newer publication is younger than this.
    KeepSupersededFor(TimeDelta),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedPublication {
    pub publication_label: String,
    /// When its `manifest.json` was uploaded.
    pub uploaded: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordingsNamedKind {
    Named(BTreeSet<String>),
    /// A publication's recordings list could not be read, so any recording may be in use.
    Unknown { unreadable_key: String },
}

/// The publication a key names when it is a publication's `manifest.json`, as `(course id, publication label)`.
pub fn parse_publication_manifest_key(key: &str) -> Option<(&str, &str)> {
    let within_courses: &str = key.strip_prefix(&format!("{}/", repository_key::COURSES_DIRECTORY))?;
    let parts: Vec<&str> = within_courses.split('/').collect();

    match parts.as_slice() {
        [course_id, publication_label, file_name]
            if *file_name == manifest::MANIFEST_FILENAME && *publication_label != repository_key::COURSE_LATEST_DIRECTORY =>
        {
            Some((course_id, publication_label))
        },
        _ => None,
    }
}

/// The course's publications, oldest upload first.
pub async fn list_course_publications(repository: &impl ArtifactRepository, course_id: &str) -> Result<Vec<ListedPublication>, AppError> {
    let listed_objects: Vec<ListedObject> = repository.list_objects(&repository_key::course_directory_key(course_id)).await?;
    let mut publications: Vec<ListedPublication> = listed_objects
        .iter()
        .filter_map(|listed_object| {
            let (_, publication_label): (&str, &str) = parse_publication_manifest_key(&listed_object.key)?;

            Some(ListedPublication {
                publication_label: publication_label.to_string(),
                uploaded: listed_object.uploaded,
            })
        })
        .collect();

    publications.sort_by(|left, right| (left.uploaded, &left.publication_label).cmp(&(right.uploaded, &right.publication_label)));

    Ok(publications)
}

enum PointerLabelsKind {
    Named(BTreeSet<String>),
    Unreadable { pointer_key: String },
}

async fn read_pointer_labels(repository: &impl ArtifactRepository, course_id: &str) -> Result<PointerLabelsKind, AppError> {
    let pointer_keys: BTreeSet<String> = repository::list_keys(repository, &repository_key::course_latest_directory_key(course_id)).await?;
    let mut pointer_labels: BTreeSet<String> = BTreeSet::new();

    for pointer_key in pointer_keys {
        let pointer: Option<StoredObject> = repository.get_object(&pointer_key).await?;
        let pointer_publication: Option<PointerPublication> = pointer.and_then(|pointer| pointer_publication::read_pointer_publication(&pointer.bytes));

        let Some(pointer_publication) = pointer_publication
        else {
            return Ok(PointerLabelsKind::Unreadable { pointer_key });
        };

        pointer_labels.insert(pointer_publication.publication_label);
    }

    Ok(PointerLabelsKind::Named(pointer_labels))
}

/// The labels of the publications to delete, given the course's publications oldest first.
pub fn decide_publications_to_delete(
    publications: &[ListedPublication],
    pointer_labels: &BTreeSet<String>,
    grace: PublicationGraceKind,
    now: DateTime<Utc>,
) -> Vec<String> {
    let newest_kept_from: usize = publications.len().saturating_sub(PUBLICATIONS_KEPT_PER_COURSE);
    let mut deleted_labels: Vec<String> = Vec::new();

    for (position, publication) in publications.iter().enumerate().take(newest_kept_from) {
        let named_by_a_pointer: bool = pointer_labels.contains(&publication.publication_label);
        let within_grace: bool = match grace {
            PublicationGraceKind::None => false,
            PublicationGraceKind::KeepSupersededFor(grace_period) => now - publications[position + 1].uploaded < grace_period,
        };

        if !named_by_a_pointer && !within_grace {
            deleted_labels.push(publication.publication_label.clone());
        }
    }

    deleted_labels
}

/// Deletes the course's publications that retention does not keep; returns their labels.
pub async fn apply_course_retention(
    repository: &impl ArtifactRepository,
    course_id: &str,
    grace: PublicationGraceKind,
    now: DateTime<Utc>,
) -> Result<Vec<String>, AppError> {
    let publications: Vec<ListedPublication> = list_course_publications(repository, course_id).await?;
    let pointer_labels_read: PointerLabelsKind = read_pointer_labels(repository, course_id).await?;

    let pointer_labels: BTreeSet<String> = match pointer_labels_read {
        PointerLabelsKind::Named(pointer_labels) => pointer_labels,
        PointerLabelsKind::Unreadable { pointer_key } => {
            log::warn!("kept every publication of the course, since a pointer names no publication label; [course_id={course_id} key={pointer_key}]");
            return Ok(Vec::new());
        },
    };
    let deleted_labels: Vec<String> = decide_publications_to_delete(&publications, &pointer_labels, grace, now);

    for publication_label in &deleted_labels {
        delete_publication(repository, course_id, publication_label).await?;
        log::info!("removed an old publication; [course_id={course_id} publication_label={publication_label}]");
    }

    Ok(deleted_labels)
}

/* The manifest goes last, so an interrupted deletion leaves a publication that the next retention finds again. */
async fn delete_publication(repository: &impl ArtifactRepository, course_id: &str, publication_label: &str) -> Result<(), AppError> {
    let manifest_key: String = repository_key::publication_manifest_key(course_id, publication_label);
    let publication_keys: BTreeSet<String> = repository::list_keys(repository, &repository_key::publication_directory_key(course_id, publication_label)).await?;
    let keys_before_the_manifest: Vec<String> = publication_keys.into_iter().filter(|key| *key != manifest_key).collect();

    repository.delete_keys(&keys_before_the_manifest).await?;
    repository.delete_keys(&[manifest_key]).await
}

/// The union of the recordings lists of every publication under `courses/`, whether or not the index lists its
/// course.
pub async fn recordings_named_by_publications(repository: &impl ArtifactRepository) -> Result<RecordingsNamedKind, AppError> {
    let listed_keys: BTreeSet<String> = repository::list_keys(repository, &format!("{}/", repository_key::COURSES_DIRECTORY)).await?;
    let mut named_sha256s: BTreeSet<String> = BTreeSet::new();

    for listed_key in &listed_keys {
        let Some((course_id, publication_label)) = parse_publication_manifest_key(listed_key)
        else {
            continue;
        };
        let recordings_key: String = repository_key::publication_recordings_key(course_id, publication_label);
        let recordings_object: Option<StoredObject> = repository.get_object(&recordings_key).await?;
        let recordings_read: Option<RecordingsList> =
            recordings_object.and_then(|recordings_object| recordings_list::parse_recordings_list(&recordings_object.bytes).ok());

        let Some(recordings_read) = recordings_read
        else {
            return Ok(RecordingsNamedKind::Unknown { unreadable_key: recordings_key });
        };

        named_sha256s.extend(recordings_read.recording_sha256s);
    }

    Ok(RecordingsNamedKind::Named(named_sha256s))
}

/// Deletes the recordings no publication names; returns how many it deleted.
pub async fn remove_unused_recordings(repository: &impl ArtifactRepository) -> Result<usize, AppError> {
    let named: RecordingsNamedKind = recordings_named_by_publications(repository).await?;

    let named_sha256s: BTreeSet<String> = match named {
        RecordingsNamedKind::Named(named_sha256s) => named_sha256s,
        RecordingsNamedKind::Unknown { unreadable_key } => {
            log::warn!("kept every recording, since a publication's recordings list is unreadable; [key={unreadable_key}]");
            return Ok(0);
        },
    };
    let named_audio_keys: BTreeSet<String> = named_sha256s.iter().map(|audio_sha256| repository_key::audio_key(audio_sha256)).collect();
    let stored_audio_keys: BTreeSet<String> = repository::list_keys(repository, &format!("{}/", manifest::SUBDIR_AUDIO)).await?;
    let unused_audio_keys: Vec<String> = stored_audio_keys.difference(&named_audio_keys).cloned().collect();

    repository.delete_keys(&unused_audio_keys).await?;

    if !unused_audio_keys.is_empty() {
        log::info!("removed recordings no publication names; [count={}]", unused_audio_keys.len());
    }

    Ok(unused_audio_keys.len())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use shared::course::Course;

    use super::*;
    use crate::artifact::artifact_bundle::{self, BuiltBundle};
    use crate::artifact::publish::{self, PointerMoveKind, RepositoryTargetKind};
    use crate::artifact::repository::memory_artifact_repository::MemoryArtifactRepository;
    use crate::audio_plan::AudioPlan;
    use crate::test_courses::{self, FixtureWorkspace, SECOND_COURSE_ID, TEACHING_COURSE_ID};

    fn publication(publication_label: &str, uploaded: DateTime<Utc>) -> ListedPublication {
        ListedPublication {
            publication_label: publication_label.to_string(),
            uploaded,
        }
    }

    fn at(rfc3339: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(rfc3339).unwrap().with_timezone(&Utc)
    }

    async fn publish_bundle(workspace: &FixtureWorkspace, repository: &MemoryArtifactRepository, target: RepositoryTargetKind, bundle: &BuiltBundle) {
        publish::publish_course_bundle(&workspace.paths, repository, target, bundle.clone(), PointerMoveKind::ToThisBundle).await.unwrap();
    }

    fn publication_labels(repository: &MemoryArtifactRepository, course_id: &str) -> BTreeSet<String> {
        repository
            .contents()
            .keys()
            .filter_map(|key| parse_publication_manifest_key(key))
            .filter(|(listed_course_id, _)| *listed_course_id == course_id)
            .map(|(_, publication_label)| publication_label.to_string())
            .collect()
    }

    #[test]
    fn decide_publications_to_delete_keeps_the_newest_and_the_ones_a_pointer_names() {
        let publications: Vec<ListedPublication> = vec![
            publication("2026-10-01+albany", at("2026-10-01T00:00:00Z")),
            publication("2026-10-02+boise", at("2026-10-02T00:00:00Z")),
            publication("2026-10-03+camden", at("2026-10-03T00:00:00Z")),
            publication("2026-10-04+dover", at("2026-10-04T00:00:00Z")),
        ];
        let pointer_labels: BTreeSet<String> = BTreeSet::from(["2026-10-01+albany".to_string()]);

        let deleted_labels: Vec<String> = decide_publications_to_delete(&publications, &pointer_labels, PublicationGraceKind::None, at("2026-10-30T00:00:00Z"));

        assert_eq!(deleted_labels, vec!["2026-10-02+boise".to_string()]);
    }

    #[test]
    fn decide_publications_to_delete_keeps_a_publication_whose_next_newer_one_is_within_the_grace_period() {
        let publications: Vec<ListedPublication> = vec![
            publication("2026-10-01+albany", at("2026-10-01T00:00:00Z")),
            publication("2026-10-02+boise", at("2026-10-02T00:00:00Z")),
            publication("2026-10-20+camden", at("2026-10-20T00:00:00Z")),
            publication("2026-10-21+dover", at("2026-10-21T00:00:00Z")),
        ];
        let grace: PublicationGraceKind = PublicationGraceKind::KeepSupersededFor(PUBLICATION_GRACE_PERIOD);

        let within_grace: Vec<String> = decide_publications_to_delete(&publications, &BTreeSet::new(), grace, at("2026-10-24T00:00:00Z"));
        let after_grace: Vec<String> = decide_publications_to_delete(&publications, &BTreeSet::new(), grace, at("2026-10-28T00:00:00Z"));

        assert_eq!(within_grace, vec!["2026-10-01+albany".to_string()]);
        assert_eq!(after_grace, vec!["2026-10-01+albany".to_string(), "2026-10-02+boise".to_string()]);
    }

    #[tokio::test]
    async fn apply_course_retention_keeps_the_pointers_publication_and_leaves_another_courses_publications() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let mut second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let mut teaching_bundles: Vec<BuiltBundle> = Vec::new();

        for title in ["First", "Second", "Third"] {
            second_course.title = title.to_string();
            let second_bundle: BuiltBundle = workspace.build_bundle(&second_course, &plan);

            publish_bundle(&workspace, &repository, RepositoryTargetKind::Dry, &second_bundle).await;
        }

        for title in ["First", "Second", "Third", "Fourth"] {
            teaching_course.title = title.to_string();
            teaching_bundles.push(workspace.build_bundle(&teaching_course, &plan));
        }

        for bundle in &teaching_bundles {
            publish_bundle(&workspace, &repository, RepositoryTargetKind::Dry, bundle).await;
        }

        let first_manifest_bytes: Vec<u8> = fs::read(artifact_bundle::bundle_manifest_path(&teaching_bundles[0].directory)).unwrap();

        repository.insert(&repository_key::course_latest_manifest_key(TEACHING_COURSE_ID), &first_manifest_bytes);

        let second_labels_before: BTreeSet<String> = publication_labels(&repository, SECOND_COURSE_ID);
        let deleted_labels: Vec<String> = apply_course_retention(&repository, TEACHING_COURSE_ID, PublicationGraceKind::None, repository.now()).await.unwrap();
        let kept_labels: BTreeSet<String> = publication_labels(&repository, TEACHING_COURSE_ID);
        let expected_kept_labels: BTreeSet<String> = [&teaching_bundles[0], &teaching_bundles[2], &teaching_bundles[3]]
            .iter()
            .map(|bundle| bundle.manifest.publication_label.clone())
            .collect();

        assert_eq!(deleted_labels, vec![teaching_bundles[1].manifest.publication_label.clone()]);
        assert_eq!(kept_labels, expected_kept_labels);
        assert!(repository.contents().keys().all(|key| !key.contains(&teaching_bundles[1].manifest.publication_label)));
        assert_eq!(publication_labels(&repository, SECOND_COURSE_ID), second_labels_before);
        assert_eq!(second_labels_before.len(), 3);
    }

    /* Four publications of the teaching course, oldest first, and none deleted yet. */
    async fn publish_four_teaching_publications(workspace: &FixtureWorkspace, repository: &MemoryArtifactRepository) -> Vec<BuiltBundle> {
        let mut plan: AudioPlan = AudioPlan::new();
        let mut course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let mut bundles: Vec<BuiltBundle> = Vec::new();

        for title in ["First", "Second", "Third", "Fourth"] {
            course.title = title.to_string();
            let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

            publish_bundle(workspace, repository, RepositoryTargetKind::Dry, &bundle).await;
            bundles.push(bundle);
        }

        bundles
    }

    #[tokio::test]
    async fn apply_course_retention_keeps_every_publication_when_a_pointer_names_no_readable_publication_label() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();

        publish_four_teaching_publications(&workspace, &repository).await;
        repository.insert(&repository_key::course_latest_manifest_key(TEACHING_COURSE_ID), b"not json");

        let labels_before: BTreeSet<String> = publication_labels(&repository, TEACHING_COURSE_ID);
        let deleted_labels: Vec<String> = apply_course_retention(&repository, TEACHING_COURSE_ID, PublicationGraceKind::None, repository.now()).await.unwrap();

        assert!(deleted_labels.is_empty());
        assert_eq!(publication_labels(&repository, TEACHING_COURSE_ID), labels_before);
        assert_eq!(labels_before.len(), 4);
    }

    #[tokio::test]
    async fn apply_course_retention_keeps_the_publication_a_pointer_of_a_newer_manifest_schema_names() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let bundles: Vec<BuiltBundle> = publish_four_teaching_publications(&workspace, &repository).await;
        let mut newer_schema_pointer: serde_json::Value = serde_json::to_value(&bundles[0].manifest).unwrap();

        newer_schema_pointer[manifest::MANIFEST_SCHEMA_VERSION_FIELD] = serde_json::Value::from(99);
        repository.insert(&repository_key::course_latest_manifest_key(TEACHING_COURSE_ID), &serde_json::to_vec(&newer_schema_pointer).unwrap());

        let deleted_labels: Vec<String> = apply_course_retention(&repository, TEACHING_COURSE_ID, PublicationGraceKind::None, repository.now()).await.unwrap();

        assert_eq!(deleted_labels, vec![bundles[1].manifest.publication_label.clone()]);
    }

    #[tokio::test]
    async fn local_retention_of_one_course_keeps_the_recordings_only_another_course_names() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let second_bundle: BuiltBundle = workspace.build_bundle(&second_course, &plan);

        publish_bundle(&workspace, &repository, RepositoryTargetKind::Local, &second_bundle).await;

        for title in ["First", "Second", "Third"] {
            teaching_course.title = title.to_string();
            let teaching_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);

            publish_bundle(&workspace, &repository, RepositoryTargetKind::Local, &teaching_bundle).await;
        }

        let second_recordings: BTreeSet<String> = artifact_bundle::read_bundle_recordings(&second_bundle.directory).unwrap().recording_sha256s;

        repository.insert(&repository_key::audio_key(&"0".repeat(64)), b"a recording no publication names");
        apply_course_retention(&repository, TEACHING_COURSE_ID, PublicationGraceKind::None, repository.now()).await.unwrap();
        remove_unused_recordings(&repository).await.unwrap();

        let stored_audio_keys: BTreeSet<String> = repository.contents().into_keys().filter(|key| key.starts_with("audio/")).collect();

        assert_eq!(publication_labels(&repository, TEACHING_COURSE_ID).len(), PUBLICATIONS_KEPT_PER_COURSE);
        assert!(second_recordings.iter().all(|audio_sha256| stored_audio_keys.contains(&repository_key::audio_key(audio_sha256))));
        assert!(!stored_audio_keys.contains(&repository_key::audio_key(&"0".repeat(64))));
        assert_eq!(stored_audio_keys.len(), 2 * test_courses::RECORDINGS_PER_FIXTURE_COURSE);
    }

    #[tokio::test]
    async fn remove_unused_recordings_keeps_every_recording_when_a_recordings_list_is_unreadable() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        publish_bundle(&workspace, &repository, RepositoryTargetKind::Dry, &bundle).await;
        repository.insert(&repository_key::publication_recordings_key(TEACHING_COURSE_ID, &bundle.manifest.publication_label), b"not json");
        repository.insert(&repository_key::audio_key(&"0".repeat(64)), b"a recording no publication names");

        let named: RecordingsNamedKind = recordings_named_by_publications(&repository).await.unwrap();
        let removed_count: usize = remove_unused_recordings(&repository).await.unwrap();

        assert!(matches!(named, RecordingsNamedKind::Unknown { .. }));
        assert_eq!(removed_count, 0);
        assert!(repository.bytes_at(&repository_key::audio_key(&"0".repeat(64))).is_some());
    }
}
