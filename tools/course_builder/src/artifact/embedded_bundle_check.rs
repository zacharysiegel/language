use std::collections::BTreeSet;

use shared::AppError;
use shared::artifact::{self, CourseIndex, CourseManifest, repository_key};

use crate::artifact::course_index_update::{self, StoredCourseIndex};
use crate::artifact::embedded_bundle;
use crate::artifact::repository::{ArtifactRepository, StoredObject};
use crate::workspace::WorkspacePaths;

const UNREADABLE_LABEL: &str = "unreadable";
const ABSENT_LABEL: &str = "absent";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddedBundleMismatchKind {
    ListedOnlyInTheEmbeddedIndex { course_id: String },
    ListedOnlyInTheRepositoryIndex { course_id: String },
    TestCourseEmbedded { course_id: String },
    PublicationDiffers {
        course_id: String,
        embedded_publication_label: String,
        repository_publication_label: String,
    },
}

impl EmbeddedBundleMismatchKind {
    pub fn describe(&self) -> String {
        match self {
            EmbeddedBundleMismatchKind::ListedOnlyInTheEmbeddedIndex { course_id } => {
                format!("the embedded index lists a course the repository's index does not; [course_id={course_id}]")
            },
            EmbeddedBundleMismatchKind::ListedOnlyInTheRepositoryIndex { course_id } => {
                format!("the repository's index lists a course the embedded index does not; [course_id={course_id}]")
            },
            EmbeddedBundleMismatchKind::TestCourseEmbedded { course_id } => format!("the embedded bundle holds a test course; [course_id={course_id}]"),
            EmbeddedBundleMismatchKind::PublicationDiffers {
                course_id,
                embedded_publication_label,
                repository_publication_label,
            } => format!(
                "the embedded publication differs from the one the repository's pointer names; [course_id={course_id} embedded_publication_label={embedded_publication_label} repository_publication_label={repository_publication_label}]",
            ),
        }
    }
}

/// Every way the embedded bundle differs from the repository's index and the publications its pointers name.
pub async fn check_embedded_bundle(paths: &WorkspacePaths, repository: &impl ArtifactRepository) -> Result<Vec<EmbeddedBundleMismatchKind>, AppError> {
    let embedded_index: Option<CourseIndex> = embedded_bundle::read_embedded_course_index(paths)?;

    let Some(embedded_index) = embedded_index
    else {
        return Err(AppError::from(format!(
            "there is no embedded bundle; publishing to a repository writes it; [path={}]",
            paths.embedded_bundle_directory.display(),
        )));
    };
    let stored_index: StoredCourseIndex = course_index_update::read_course_index(repository).await?;
    let embedded_course_ids: BTreeSet<&String> = embedded_index.courses.keys().collect();
    let repository_course_ids: BTreeSet<&String> = stored_index.course_index.courses.keys().collect();
    let mut mismatches: Vec<EmbeddedBundleMismatchKind> = Vec::new();

    for course_id in embedded_course_ids.difference(&repository_course_ids) {
        mismatches.push(EmbeddedBundleMismatchKind::ListedOnlyInTheEmbeddedIndex {
            course_id: course_id.to_string(),
        });
    }

    for course_id in repository_course_ids.difference(&embedded_course_ids) {
        mismatches.push(EmbeddedBundleMismatchKind::ListedOnlyInTheRepositoryIndex {
            course_id: course_id.to_string(),
        });
    }

    for (course_id, embedded_entry) in &embedded_index.courses {
        if embedded_entry.test_course {
            mismatches.push(EmbeddedBundleMismatchKind::TestCourseEmbedded { course_id: course_id.clone() });
        }

        let publication_mismatch: Option<EmbeddedBundleMismatchKind> = compare_course_publication(paths, repository, course_id).await?;

        mismatches.extend(publication_mismatch);
    }

    Ok(mismatches)
}

async fn compare_course_publication(
    paths: &WorkspacePaths,
    repository: &impl ArtifactRepository,
    course_id: &str,
) -> Result<Option<EmbeddedBundleMismatchKind>, AppError> {
    let embedded_bytes: Option<Vec<u8>> = embedded_bundle::read_embedded_course_manifest_bytes(paths, course_id)?;
    let repository_pointer: Option<StoredObject> = repository.get_object(&repository_key::course_latest_manifest_key(course_id)).await?;
    let repository_bytes: Option<Vec<u8>> = repository_pointer.map(|repository_pointer| repository_pointer.bytes);

    let publications_match: bool = embedded_bytes.is_some() && embedded_bytes == repository_bytes;

    if publications_match {
        return Ok(None);
    }

    Ok(Some(EmbeddedBundleMismatchKind::PublicationDiffers {
        course_id: course_id.to_string(),
        embedded_publication_label: describe_publication_label(course_id, embedded_bytes.as_deref()),
        repository_publication_label: describe_publication_label(course_id, repository_bytes.as_deref()),
    }))
}

fn describe_publication_label(course_id: &str, manifest_bytes: Option<&[u8]>) -> String {
    let Some(manifest_bytes) = manifest_bytes
    else {
        return ABSENT_LABEL.to_string();
    };

    let parsed: Result<CourseManifest, AppError> = artifact::parse_course_manifest(manifest_bytes, Some(course_id));

    match parsed {
        Ok(course_manifest) => course_manifest.publication_label,
        Err(_) => UNREADABLE_LABEL.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use shared::course::Course;

    use super::*;
    use crate::artifact::artifact_bundle::BuiltBundle;
    use crate::artifact::json_document;
    use crate::artifact::publish::{self, PointerMoveKind, RepositoryTargetKind};
    use crate::artifact::repository::memory_artifact_repository::MemoryArtifactRepository;
    use crate::audio_plan::AudioPlan;
    use crate::test_courses::{self, FixtureWorkspace, SECOND_COURSE_ID, TEACHING_COURSE_ID};

    const OTHER_COURSE_ID: &str = "pt_br.en.other";

    /* Two courses published to the repository, which refreshes the embedded bundle from each. */
    async fn published_workspace(plan: &mut AudioPlan) -> (FixtureWorkspace, MemoryArtifactRepository, Vec<BuiltBundle>) {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let mut bundles: Vec<BuiltBundle> = Vec::new();

        for course_id in [TEACHING_COURSE_ID, SECOND_COURSE_ID] {
            let course: Course = test_courses::fixture_course(plan, course_id, false);
            let bundle: BuiltBundle = workspace.build_bundle(&course, plan);

            publish::publish_course_bundle(&workspace.paths, &repository, RepositoryTargetKind::CloudflareR2, bundle.clone(), PointerMoveKind::ToThisBundle)
                .await
                .unwrap();
            bundles.push(bundle);
        }

        (workspace, repository, bundles)
    }

    #[tokio::test]
    async fn check_embedded_bundle_passes_when_every_embedded_publication_is_the_repositorys() {
        let mut plan: AudioPlan = AudioPlan::new();
        let (workspace, repository, _bundles): (FixtureWorkspace, MemoryArtifactRepository, Vec<BuiltBundle>) = published_workspace(&mut plan).await;

        assert_eq!(check_embedded_bundle(&workspace.paths, &repository).await.unwrap(), Vec::new());
    }

    #[tokio::test]
    async fn check_embedded_bundle_names_both_labels_of_a_differing_publication() {
        let mut plan: AudioPlan = AudioPlan::new();
        let (workspace, repository, bundles): (FixtureWorkspace, MemoryArtifactRepository, Vec<BuiltBundle>) = published_workspace(&mut plan).await;
        let mut newer_manifest: CourseManifest = bundles[0].manifest.clone();

        newer_manifest.publication_label = "2026-10-09+elsewhere".to_string();
        repository.insert(&repository_key::course_latest_manifest_key(TEACHING_COURSE_ID), &json_document::json_document_bytes(&newer_manifest).unwrap());

        let mismatches: Vec<EmbeddedBundleMismatchKind> = check_embedded_bundle(&workspace.paths, &repository).await.unwrap();

        assert_eq!(
            mismatches,
            vec![EmbeddedBundleMismatchKind::PublicationDiffers {
                course_id: TEACHING_COURSE_ID.to_string(),
                embedded_publication_label: bundles[0].manifest.publication_label.clone(),
                repository_publication_label: "2026-10-09+elsewhere".to_string(),
            }],
        );
        assert!(mismatches[0].describe().contains("2026-10-09+elsewhere"));
    }

    #[tokio::test]
    async fn check_embedded_bundle_fails_for_a_course_either_index_lacks() {
        let mut plan: AudioPlan = AudioPlan::new();
        let (workspace, repository, _bundles): (FixtureWorkspace, MemoryArtifactRepository, Vec<BuiltBundle>) = published_workspace(&mut plan).await;
        let other_course: Course = test_courses::fixture_course(&mut plan, OTHER_COURSE_ID, false);
        let other_bundle: BuiltBundle = workspace.build_bundle(&other_course, &plan);

        embedded_bundle::drop_embedded_course(&workspace.paths, SECOND_COURSE_ID).unwrap();
        embedded_bundle::refresh_embedded_course(&workspace.paths, &other_bundle).unwrap();

        let mismatches: Vec<EmbeddedBundleMismatchKind> = check_embedded_bundle(&workspace.paths, &repository).await.unwrap();

        assert!(mismatches.contains(&EmbeddedBundleMismatchKind::ListedOnlyInTheEmbeddedIndex {
            course_id: OTHER_COURSE_ID.to_string(),
        }));
        assert!(mismatches.contains(&EmbeddedBundleMismatchKind::ListedOnlyInTheRepositoryIndex {
            course_id: SECOND_COURSE_ID.to_string(),
        }));
        assert!(mismatches.contains(&EmbeddedBundleMismatchKind::PublicationDiffers {
            course_id: OTHER_COURSE_ID.to_string(),
            embedded_publication_label: other_bundle.manifest.publication_label.clone(),
            repository_publication_label: ABSENT_LABEL.to_string(),
        }));
    }

    #[tokio::test]
    async fn check_embedded_bundle_fails_for_an_embedded_test_course() {
        let mut plan: AudioPlan = AudioPlan::new();
        let (workspace, repository, _bundles): (FixtureWorkspace, MemoryArtifactRepository, Vec<BuiltBundle>) = published_workspace(&mut plan).await;
        let mut embedded_index: CourseIndex = embedded_bundle::read_embedded_course_index(&workspace.paths).unwrap().unwrap();

        embedded_index.courses.get_mut(TEACHING_COURSE_ID).unwrap().test_course = true;
        json_document::write_json_document(&workspace.paths.embedded_bundle_directory.join(repository_key::COURSE_INDEX_KEY), &embedded_index).unwrap();

        let mismatches: Vec<EmbeddedBundleMismatchKind> = check_embedded_bundle(&workspace.paths, &repository).await.unwrap();

        assert_eq!(
            mismatches,
            vec![EmbeddedBundleMismatchKind::TestCourseEmbedded {
                course_id: TEACHING_COURSE_ID.to_string(),
            }],
        );
    }
}
