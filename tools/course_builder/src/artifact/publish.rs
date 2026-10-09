//! Within a publication, the recordings, shards and `recordings.json` go up before `manifest.json`; the course's
//! pointers follow, then the course index, so a client that reads any of them finds every file it names.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use shared::AppError;
use shared::artifact::{self, CourseIndex, CourseIndexEntry, CourseManifest, content_address, course_manifest, manifest, repository_key};
use shared::course::Curriculum;

use crate::artifact::artifact_bundle::{self, BuiltBundle};
use crate::artifact::course_index_update::{self, CourseIndexChangeKind, StoredCourseIndex};
use crate::artifact::embedded_bundle;
use crate::artifact::recordings_list::RecordingsList;
use crate::artifact::repository::{self, ArtifactRepository, ObjectHeaders, StoredObject, WriteConditionKind, WriteOutcomeKind};
use crate::artifact::retention::{self, ListedPublication, PublicationGraceKind};
use crate::build_command;
use crate::course_source;
use crate::workspace::WorkspacePaths;

const MAX_RELABEL_ATTEMPTS: usize = 8;

/// The kind of repository a command writes, which decides the rules it follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryTargetKind {
    Local,
    CloudflareR2,
    Dry,
}

impl RepositoryTargetKind {
    /// The target's name on the command line.
    pub fn argument_name(self) -> &'static str {
        match self {
            RepositoryTargetKind::Local => "local",
            RepositoryTargetKind::CloudflareR2 => "cloudflare-r2",
            RepositoryTargetKind::Dry => "dry",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerMoveKind {
    ToThisBundle,
    /// Leaves a pointer that names a publication uploaded after the bundle's.
    OnlyForward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationUploadKind {
    Uploaded,
    AlreadyPresent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerChangeKind {
    Moved,
    AlreadyCurrent,
    LeftOnAnotherPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnlistedCourseInclusionKind {
    Exclude,
    Include,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseSelectionKind {
    /// The newest bundle of each named course.
    Named(Vec<String>),
    Bundle(PathBuf),
    /// The newest bundle of each course the target's index lists and the data holds.
    EveryCourse(UnlistedCourseInclusionKind),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CoursePublishReport {
    pub course_id: String,
    pub publication_label: String,
    pub latest_manifest_url: String,
    pub upload: PublicationUploadKind,
    pub pointer: PointerChangeKind,
    pub index_change: CourseIndexChangeKind,
    pub shards_uploaded: usize,
    pub recordings_uploaded: usize,
    pub recordings_already_present: usize,
}

pub struct CoursePublishOutcome {
    pub course_id: String,
    pub result: Result<CoursePublishReport, AppError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct UploadCounts {
    shards_uploaded: usize,
    recordings_uploaded: usize,
    recordings_already_present: usize,
}

struct PlannedCoursePublish {
    course_id: String,
    bundle: Result<BuiltBundle, AppError>,
    pointer_move: PointerMoveKind,
}

/// Publishes the selected courses one after another; one course's failure does not stop the rest.
pub async fn publish_selected_courses(
    paths: &WorkspacePaths,
    repository: &impl ArtifactRepository,
    target: RepositoryTargetKind,
    selection: &CourseSelectionKind,
) -> Result<Vec<CoursePublishOutcome>, AppError> {
    let planned_publishes: Vec<PlannedCoursePublish> = plan_course_publishes(paths, repository, target, selection).await?;
    let mut outcomes: Vec<CoursePublishOutcome> = Vec::new();

    for planned_publish in planned_publishes {
        let result: Result<CoursePublishReport, AppError> = match planned_publish.bundle {
            Ok(bundle) => publish_course_bundle(paths, repository, target, bundle, planned_publish.pointer_move).await,
            Err(error) => Err(error),
        };

        if let Err(error) = &result {
            log::error!("publishing a course failed; [course_id={} error={error}]", planned_publish.course_id);
        }

        outcomes.push(CoursePublishOutcome {
            course_id: planned_publish.course_id,
            result,
        });
    }

    Ok(outcomes)
}

async fn plan_course_publishes(
    paths: &WorkspacePaths,
    repository: &impl ArtifactRepository,
    target: RepositoryTargetKind,
    selection: &CourseSelectionKind,
) -> Result<Vec<PlannedCoursePublish>, AppError> {
    let planned_publishes: Vec<PlannedCoursePublish> = match selection {
        CourseSelectionKind::Bundle(bundle_directory) => {
            let bundle_manifest: CourseManifest = artifact_bundle::read_bundle_manifest(bundle_directory)?;

            vec![PlannedCoursePublish {
                course_id: bundle_manifest.course_id.clone(),
                bundle: Ok(BuiltBundle {
                    directory: bundle_directory.clone(),
                    manifest: bundle_manifest,
                }),
                pointer_move: PointerMoveKind::ToThisBundle,
            }]
        },
        CourseSelectionKind::Named(course_ids) => plan_named_courses(paths, course_ids)?,
        CourseSelectionKind::EveryCourse(inclusion) => plan_every_course(paths, repository, target, *inclusion).await?,
    };

    for planned_publish in &planned_publishes {
        if let Ok(bundle) = &planned_publish.bundle {
            check_bundle_publishable(target, &bundle.manifest)?;
        }
    }

    if target == RepositoryTargetKind::CloudflareR2 {
        let stored_index: StoredCourseIndex = course_index_update::read_course_index(repository).await?;

        check_index_lists_no_test_course(paths, target, &stored_index.course_index)?;
    }

    Ok(planned_publishes)
}

fn plan_named_courses(paths: &WorkspacePaths, course_ids: &[String]) -> Result<Vec<PlannedCoursePublish>, AppError> {
    let mut planned_publishes: Vec<PlannedCoursePublish> = Vec::new();

    for course_id in course_ids {
        let newest_bundle: Option<BuiltBundle> = artifact_bundle::find_newest_course_bundle(paths, course_id)?;
        let bundle: Result<BuiltBundle, AppError> = newest_bundle.ok_or_else(|| {
            AppError::from(format!("no bundle has been built for the course; run `course_builder build` first; [course_id={course_id}]"))
        });

        planned_publishes.push(PlannedCoursePublish {
            course_id: course_id.clone(),
            bundle,
            pointer_move: PointerMoveKind::ToThisBundle,
        });
    }

    Ok(planned_publishes)
}

async fn plan_every_course(
    paths: &WorkspacePaths,
    repository: &impl ArtifactRepository,
    target: RepositoryTargetKind,
    inclusion: UnlistedCourseInclusionKind,
) -> Result<Vec<PlannedCoursePublish>, AppError> {
    let stored_index: StoredCourseIndex = course_index_update::read_course_index(repository).await?;
    let data_course_ids: Vec<String> = course_source::list_course_ids(paths)?;

    for indexed_course_id in stored_index.course_index.courses.keys() {
        if !data_course_ids.contains(indexed_course_id) {
            log::warn!(
                "the course index lists a course the data does not hold; `remove-course {} --course {indexed_course_id}` removes it; [course_id={indexed_course_id}]",
                target.argument_name(),
            );
        }
    }

    let mut planned_publishes: Vec<PlannedCoursePublish> = Vec::new();

    for course_id in &data_course_ids {
        let listed: bool = stored_index.course_index.courses.contains_key(course_id);

        if !listed && inclusion == UnlistedCourseInclusionKind::Exclude {
            continue;
        }

        let newest_bundle: Option<BuiltBundle> = artifact_bundle::find_newest_course_bundle(paths, course_id)?;

        let Some(newest_bundle) = newest_bundle else {
            log::info!("no bundle has been built for a course, so it is left as published; [course_id={course_id}]");
            continue;
        };

        if target == RepositoryTargetKind::CloudflareR2 && is_marked_a_test_course(paths, &newest_bundle.manifest)? {
            log::info!("left out a test course, which Cloudflare R2 never holds; [course_id={course_id}]");
            continue;
        }

        planned_publishes.push(PlannedCoursePublish {
            course_id: course_id.clone(),
            bundle: Ok(newest_bundle),
            pointer_move: PointerMoveKind::OnlyForward,
        });
    }

    Ok(planned_publishes)
}

fn is_marked_a_test_course(paths: &WorkspacePaths, bundle_manifest: &CourseManifest) -> Result<bool, AppError> {
    if bundle_manifest.test_course {
        return Ok(true);
    }

    is_test_course_in_the_data(paths, &bundle_manifest.course_id)
}

/// `false` for a course the data does not hold.
fn is_test_course_in_the_data(paths: &WorkspacePaths, course_id: &str) -> Result<bool, AppError> {
    if !paths.curriculum_path(course_id).exists() {
        return Ok(false);
    }

    let curriculum: Curriculum = course_source::read_curriculum(paths, course_id)?;

    Ok(curriculum.test_course)
}

/// Cloudflare R2 is the production repository, so a test course's bundle is refused there.
pub fn check_bundle_publishable(target: RepositoryTargetKind, bundle_manifest: &CourseManifest) -> Result<(), AppError> {
    match target {
        RepositoryTargetKind::CloudflareR2 if bundle_manifest.test_course => Err(AppError::from(format!(
            "a test course cannot be published to Cloudflare R2; its bundle is built only with --{}; [course_id={} publication_label={}]",
            build_command::INCLUDE_TEST_COURSES_FLAG,
            bundle_manifest.course_id,
            bundle_manifest.publication_label,
        ))),
        RepositoryTargetKind::CloudflareR2 | RepositoryTargetKind::Local | RepositoryTargetKind::Dry => Ok(()),
    }
}

/// On Cloudflare R2, refuses an index that lists a course its entry or the data marks a test course.
pub fn check_index_lists_no_test_course(paths: &WorkspacePaths, target: RepositoryTargetKind, course_index: &CourseIndex) -> Result<(), AppError> {
    if target != RepositoryTargetKind::CloudflareR2 {
        return Ok(());
    }

    for (course_id, entry) in &course_index.courses {
        let marked_a_test_course: bool = entry.test_course || is_test_course_in_the_data(paths, course_id)?;

        if marked_a_test_course {
            return Err(AppError::from(format!(
                "the Cloudflare R2 course index lists a test course; `remove-course {} --course {course_id}` removes it; [course_id={course_id}]",
                target.argument_name(),
            )));
        }
    }

    Ok(())
}

/// Publishes one course's bundle, completing whatever an earlier, interrupted publish of it left undone.
pub async fn publish_course_bundle(
    paths: &WorkspacePaths,
    repository: &impl ArtifactRepository,
    target: RepositoryTargetKind,
    bundle: BuiltBundle,
    pointer_move: PointerMoveKind,
) -> Result<CoursePublishReport, AppError> {
    let course_id: String = bundle.manifest.course_id.clone();

    check_bundle_publishable(target, &bundle.manifest)?;

    let stored_index: StoredCourseIndex = course_index_update::read_course_index(repository).await?;

    check_index_lists_no_test_course(paths, target, &stored_index.course_index)?;

    let recordings: BTreeSet<String> = verify_bundle_recordings(&bundle)?;

    log::info!(
        "publishing; [course_id={course_id} publication_label={} shards={} recordings={}]",
        bundle.manifest.publication_label,
        bundle.manifest.file_entries().count(),
        recordings.len(),
    );

    let (bundle, upload, upload_counts): (BuiltBundle, PublicationUploadKind, UploadCounts) =
        place_publication(paths, repository, bundle, &recordings).await?;
    let manifest_bytes: Vec<u8> = read_bundle_manifest_bytes(&bundle.directory)?;
    let pointer: PointerChangeKind = update_latest_pointer(repository, &bundle.manifest, &manifest_bytes, upload, pointer_move).await?;
    let entry: CourseIndexEntry = index_entry_for_the_pointer(repository, &bundle.manifest, pointer).await?;
    let index_change: CourseIndexChangeKind = course_index_update::set_course_index_entry(repository, &stored_index, &course_id, entry).await?;

    apply_retention(repository, target, &course_id).await?;
    refresh_embedded_course_after_publish(paths, target, &bundle, pointer)?;

    Ok(CoursePublishReport {
        course_id: course_id.clone(),
        publication_label: bundle.manifest.publication_label.clone(),
        latest_manifest_url: repository.url(&repository_key::course_latest_manifest_key(&course_id)),
        upload,
        pointer,
        index_change,
        shards_uploaded: upload_counts.shards_uploaded,
        recordings_uploaded: upload_counts.recordings_uploaded,
        recordings_already_present: upload_counts.recordings_already_present,
    })
}

/// The recordings the bundle's verified shards name, which must be the ones its `recordings.json` lists.
fn verify_bundle_recordings(bundle: &BuiltBundle) -> Result<BTreeSet<String>, AppError> {
    let derived_recordings: BTreeSet<String> = artifact_bundle::referenced_audio_sha256s(&bundle.directory, &bundle.manifest)?;
    let listed_recordings: RecordingsList = artifact_bundle::read_bundle_recordings(&bundle.directory)?;

    if listed_recordings.recording_sha256s != derived_recordings {
        return Err(AppError::from(format!(
            "the bundle's recordings list differs from the recordings its shards name; [course_id={} publication_label={} listed={} named={}]",
            bundle.manifest.course_id,
            bundle.manifest.publication_label,
            listed_recordings.recording_sha256s.len(),
            derived_recordings.len(),
        )));
    }

    Ok(derived_recordings)
}

fn read_bundle_manifest_bytes(bundle_directory: &Path) -> Result<Vec<u8>, AppError> {
    let manifest_path: PathBuf = artifact_bundle::bundle_manifest_path(bundle_directory);

    fs::read(&manifest_path).map_err(|error| AppError::from(format!("could not read a bundle manifest; [path={} error={error}]", manifest_path.display())))
}

/// Uploads the bundle's publication unless the repository already holds it; relabels the bundle when the repository
/// holds other course content under its label.
async fn place_publication(
    paths: &WorkspacePaths,
    repository: &impl ArtifactRepository,
    initial_bundle: BuiltBundle,
    recordings: &BTreeSet<String>,
) -> Result<(BuiltBundle, PublicationUploadKind, UploadCounts), AppError> {
    let mut bundle: BuiltBundle = initial_bundle;

    for _ in 0..MAX_RELABEL_ATTEMPTS {
        let manifest_bytes: Vec<u8> = read_bundle_manifest_bytes(&bundle.directory)?;
        let manifest_key: String = repository_key::publication_manifest_key(&bundle.manifest.course_id, &bundle.manifest.publication_label);
        let stored_manifest: Option<StoredObject> = repository.get_object(&manifest_key).await?;

        match stored_manifest {
            Some(stored_manifest) if stored_manifest.bytes == manifest_bytes => {
                log::info!("the repository already holds the publication; [key={manifest_key}]");
                return Ok((bundle, PublicationUploadKind::AlreadyPresent, UploadCounts::default()));
            },
            Some(_) => {
                let target_labels: BTreeSet<String> = list_publication_labels(repository, &bundle.manifest.course_id).await?;

                bundle = artifact_bundle::relabel_bundle(paths, &bundle, &target_labels)?;
            },
            None => {
                let upload_counts: UploadCounts = upload_publication(repository, &bundle, &manifest_bytes, recordings).await?;

                return Ok((bundle, PublicationUploadKind::Uploaded, upload_counts));
            },
        }
    }

    Err(AppError::from(format!(
        "could not find a publication label the repository does not already use; [course_id={} attempts={MAX_RELABEL_ATTEMPTS}]",
        bundle.manifest.course_id,
    )))
}

async fn list_publication_labels(repository: &impl ArtifactRepository, course_id: &str) -> Result<BTreeSet<String>, AppError> {
    let publications: Vec<ListedPublication> = retention::list_course_publications(repository, course_id).await?;

    Ok(publications.into_iter().map(|publication| publication.publication_label).collect())
}

/* The recordings and shards a failed run already uploaded are skipped; `manifest.json` goes last. */
async fn upload_publication(
    repository: &impl ArtifactRepository,
    bundle: &BuiltBundle,
    manifest_bytes: &[u8],
    recordings: &BTreeSet<String>,
) -> Result<UploadCounts, AppError> {
    let course_id: &str = &bundle.manifest.course_id;
    let publication_label: &str = &bundle.manifest.publication_label;
    let (recordings_uploaded, recordings_already_present): (usize, usize) = upload_missing_recordings(repository, &bundle.directory, recordings).await?;
    let shards_uploaded: usize = upload_missing_shards(repository, bundle).await?;
    let recordings_key: String = repository_key::publication_recordings_key(course_id, publication_label);

    repository
        .put_file(&recordings_key, &artifact_bundle::bundle_recordings_path(&bundle.directory), ObjectHeaders::for_key(&recordings_key))
        .await?;

    let manifest_key: String = repository_key::publication_manifest_key(course_id, publication_label);
    let outcome: WriteOutcomeKind = repository
        .put_bytes(&manifest_key, manifest_bytes, ObjectHeaders::for_key(&manifest_key), WriteConditionKind::Absent)
        .await?;

    if outcome == WriteOutcomeKind::PreconditionFailed {
        return Err(precondition_failure(repository, &manifest_key));
    }

    Ok(UploadCounts {
        shards_uploaded,
        recordings_uploaded,
        recordings_already_present,
    })
}

fn precondition_failure(repository: &impl ArtifactRepository, key: &str) -> AppError {
    AppError::from(format!(
        "an object changed while this command held the repository lock, so another writer ignored the lock; [url={}]",
        repository.url(key),
    ))
}

/// Returns how many recordings were uploaded and how many were already there.
async fn upload_missing_recordings(
    repository: &impl ArtifactRepository,
    bundle_directory: &Path,
    recordings: &BTreeSet<String>,
) -> Result<(usize, usize), AppError> {
    let stored_audio_keys: BTreeSet<String> = repository::list_keys(repository, &format!("{}/", manifest::SUBDIR_AUDIO)).await?;
    let mut uploaded_count: usize = 0;

    for audio_sha256 in recordings {
        let key: String = manifest::audio_key(audio_sha256);

        if stored_audio_keys.contains(&key) {
            continue;
        }

        let source_path: PathBuf = artifact_bundle::audio_file_path(bundle_directory, audio_sha256);

        verify_audio_file(&source_path, audio_sha256)?;
        repository.put_file(&key, &source_path, ObjectHeaders::for_key(&key)).await?;
        uploaded_count += 1;
    }

    Ok((uploaded_count, recordings.len() - uploaded_count))
}

async fn upload_missing_shards(repository: &impl ArtifactRepository, bundle: &BuiltBundle) -> Result<usize, AppError> {
    let course_id: &str = &bundle.manifest.course_id;
    let publication_label: &str = &bundle.manifest.publication_label;
    let stored_keys: BTreeSet<String> = repository::list_keys(repository, &repository_key::publication_directory_key(course_id, publication_label)).await?;
    let mut uploaded_count: usize = 0;

    for entry in bundle.manifest.file_entries() {
        let key: String = repository_key::publication_file_key(course_id, publication_label, &entry.relative_path);

        if stored_keys.contains(&key) {
            continue;
        }

        repository.put_file(&key, &bundle.directory.join(&entry.relative_path), ObjectHeaders::for_key(&key)).await?;
        uploaded_count += 1;
        log::debug!("uploaded shard; [key={key}]");
    }

    Ok(uploaded_count)
}

fn verify_audio_file(path: &Path, audio_sha256: &str) -> Result<(), AppError> {
    let bytes: Vec<u8> = fs::read(path)
        .map_err(|error| AppError::from(format!("a recording the shards name is missing; [path={} error={error}]", path.display())))?;

    content_address::verify_sha256(&path.display().to_string(), &bytes, audio_sha256)
}

/// Writes the schema pointer, then `latest/manifest.json` conditional on the pointer read before.
async fn update_latest_pointer(
    repository: &impl ArtifactRepository,
    bundle_manifest: &CourseManifest,
    manifest_bytes: &[u8],
    upload: PublicationUploadKind,
    pointer_move: PointerMoveKind,
) -> Result<PointerChangeKind, AppError> {
    let course_id: &str = &bundle_manifest.course_id;
    let latest_manifest_key: String = repository_key::course_latest_manifest_key(course_id);
    let current_pointer: Option<StoredObject> = repository.get_object(&latest_manifest_key).await?;

    let write_condition: WriteConditionKind = match &current_pointer {
        Some(current_pointer) if current_pointer.bytes == manifest_bytes => return Ok(PointerChangeKind::AlreadyCurrent),
        Some(current_pointer) => WriteConditionKind::MatchesEntityTag(current_pointer.entity_tag.clone()),
        None => WriteConditionKind::Absent,
    };
    let moves: bool = match (&current_pointer, pointer_move, upload) {
        (None, _, _) | (_, PointerMoveKind::ToThisBundle, _) | (_, PointerMoveKind::OnlyForward, PublicationUploadKind::Uploaded) => true,
        (Some(current_pointer), PointerMoveKind::OnlyForward, PublicationUploadKind::AlreadyPresent) => {
            is_uploaded_after_the_pointers_publication(repository, bundle_manifest, &current_pointer.bytes).await?
        },
    };

    if !moves {
        log::info!(
            "left the latest pointer on a publication uploaded after this bundle's; a publish naming the course with --course moves it; [course_id={course_id} publication_label={}]",
            bundle_manifest.publication_label,
        );

        return Ok(PointerChangeKind::LeftOnAnotherPublication);
    }

    let schema_pointer_key: String = repository_key::course_schema_pointer_key(&latest_manifest_key, course_manifest::COURSE_MANIFEST_SCHEMA_VERSION);

    repository
        .put_bytes(&schema_pointer_key, manifest_bytes, ObjectHeaders::for_key(&schema_pointer_key), WriteConditionKind::Unconditional)
        .await?;

    let outcome: WriteOutcomeKind = repository
        .put_bytes(&latest_manifest_key, manifest_bytes, ObjectHeaders::for_key(&latest_manifest_key), write_condition)
        .await?;

    if outcome == WriteOutcomeKind::PreconditionFailed {
        return Err(precondition_failure(repository, &latest_manifest_key));
    }

    Ok(PointerChangeKind::Moved)
}

/// `true` also when the pointer names a publication the repository no longer lists.
async fn is_uploaded_after_the_pointers_publication(
    repository: &impl ArtifactRepository,
    bundle_manifest: &CourseManifest,
    pointer_bytes: &[u8],
) -> Result<bool, AppError> {
    let publications: Vec<ListedPublication> = retention::list_course_publications(repository, &bundle_manifest.course_id).await?;
    let pointer_label: Option<String> = artifact::parse_course_manifest(pointer_bytes, Some(&bundle_manifest.course_id))
        .ok()
        .map(|pointer_manifest| pointer_manifest.publication_label);
    let upload_time_of = |publication_label: &str| -> Option<DateTime<Utc>> {
        publications
            .iter()
            .find(|publication| publication.publication_label == publication_label)
            .map(|publication| publication.uploaded)
    };

    let Some(bundle_uploaded) = upload_time_of(&bundle_manifest.publication_label) else {
        return Ok(true);
    };
    let Some(pointer_uploaded) = pointer_label.as_deref().and_then(upload_time_of) else {
        return Ok(true);
    };

    Ok(bundle_uploaded > pointer_uploaded)
}

/// The index entry for the publication the course's pointer names after this publish.
async fn index_entry_for_the_pointer(
    repository: &impl ArtifactRepository,
    bundle_manifest: &CourseManifest,
    pointer: PointerChangeKind,
) -> Result<CourseIndexEntry, AppError> {
    if pointer != PointerChangeKind::LeftOnAnotherPublication {
        return Ok(course_index_update::index_entry_for(bundle_manifest));
    }

    let latest_manifest_key: String = repository_key::course_latest_manifest_key(&bundle_manifest.course_id);
    let current_pointer: Option<StoredObject> = repository.get_object(&latest_manifest_key).await?;
    let pointer_manifest: Option<CourseManifest> =
        current_pointer.and_then(|current_pointer| artifact::parse_course_manifest(&current_pointer.bytes, Some(&bundle_manifest.course_id)).ok());

    Ok(course_index_update::index_entry_for(pointer_manifest.as_ref().unwrap_or(bundle_manifest)))
}

async fn apply_retention(repository: &impl ArtifactRepository, target: RepositoryTargetKind, course_id: &str) -> Result<(), AppError> {
    match target {
        RepositoryTargetKind::Local => {
            retention::apply_course_retention(repository, course_id, PublicationGraceKind::None, Utc::now()).await?;
            retention::remove_unused_recordings(repository).await?;
        },
        RepositoryTargetKind::CloudflareR2 => {
            retention::apply_course_retention(repository, course_id, PublicationGraceKind::KeepSupersededFor(retention::PUBLICATION_GRACE_PERIOD), Utc::now())
                .await?;
        },
        RepositoryTargetKind::Dry => {},
    }

    Ok(())
}

fn refresh_embedded_course_after_publish(
    paths: &WorkspacePaths,
    target: RepositoryTargetKind,
    bundle: &BuiltBundle,
    pointer: PointerChangeKind,
) -> Result<(), AppError> {
    if target == RepositoryTargetKind::Dry {
        return Ok(());
    }

    if bundle.manifest.test_course {
        log::info!("left the embedded bundle as it is, since it never holds a test course; [course_id={}]", bundle.manifest.course_id);
        return Ok(());
    }

    if pointer == PointerChangeKind::LeftOnAnotherPublication {
        log::info!(
            "left the embedded course as it is, since the repository's pointer names another publication; [course_id={}]",
            bundle.manifest.course_id,
        );
        return Ok(());
    }

    embedded_bundle::refresh_embedded_course(paths, bundle)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use shared::artifact::CourseIndexEntry;
    use shared::course::Course;

    use super::*;
    use crate::artifact::json_document;
    use crate::artifact::recordings_list::RecordingsList;
    use crate::artifact::repository::LocalArtifactRepository;
    use crate::artifact::repository::memory_artifact_repository::{MemoryArtifactRepository, RecordedOperationKind};
    use crate::audio_plan::AudioPlan;
    use crate::test_courses::{self, FixtureWorkspace, SECOND_COURSE_ID, TEACHING_COURSE_ID, TEST_COURSE_ID};

    const NEWER_PUBLICATION_LABEL: &str = "2026-10-09+elsewhere";

    async fn publish_named(
        workspace: &FixtureWorkspace,
        repository: &impl ArtifactRepository,
        target: RepositoryTargetKind,
        bundle: &BuiltBundle,
    ) -> Result<CoursePublishReport, AppError> {
        publish_course_bundle(&workspace.paths, repository, target, bundle.clone(), PointerMoveKind::ToThisBundle).await
    }

    async fn read_index(repository: &impl ArtifactRepository) -> CourseIndex {
        course_index_update::read_course_index(repository).await.unwrap().course_index
    }

    fn latest_pointer_bytes(repository: &MemoryArtifactRepository, course_id: &str) -> Option<Vec<u8>> {
        repository.bytes_at(&repository_key::course_latest_manifest_key(course_id))
    }

    fn bundle_manifest_bytes(bundle: &BuiltBundle) -> Vec<u8> {
        fs::read(artifact_bundle::bundle_manifest_path(&bundle.directory)).unwrap()
    }

    fn outcome_ids_and_successes(outcomes: &[CoursePublishOutcome]) -> Vec<(String, bool)> {
        outcomes.iter().map(|outcome| (outcome.course_id.clone(), outcome.result.is_ok())).collect()
    }

    /* Another publisher's newer publication of the course, which the course's pointer names. */
    fn insert_newer_publication(repository: &MemoryArtifactRepository, bundle: &BuiltBundle) -> Vec<u8> {
        let mut newer_manifest: CourseManifest = bundle.manifest.clone();

        newer_manifest.publication_label = NEWER_PUBLICATION_LABEL.to_string();

        let newer_bytes: Vec<u8> = json_document::json_document_bytes(&newer_manifest).unwrap();

        repository.insert(&repository_key::publication_manifest_key(&bundle.manifest.course_id, NEWER_PUBLICATION_LABEL), &newer_bytes);
        repository.insert(&repository_key::course_latest_manifest_key(&bundle.manifest.course_id), &newer_bytes);

        newer_bytes
    }

    #[tokio::test]
    async fn publish_course_bundle_writes_every_file_points_latest_at_the_publication_and_reuses_recordings() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let repository_root: PathBuf = workspace.paths.local_repository_directory.clone();
        let repository: LocalArtifactRepository = LocalArtifactRepository::new(repository_root.clone(), "/repository".to_string());
        let first_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        let first_report: CoursePublishReport = publish_named(&workspace, &repository, RepositoryTargetKind::Local, &first_bundle).await.unwrap();
        let rebuilt: artifact_bundle::CourseBundleOutcomeKind =
            artifact_bundle::write_course_bundle(&workspace.paths, &course, &plan, &workspace.recordings_for(&course)).unwrap();

        course.title = "Renamed".to_string();
        let second_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let second_report: CoursePublishReport = publish_named(&workspace, &repository, RepositoryTargetKind::Local, &second_bundle).await.unwrap();

        let latest_manifest_key: String = repository_key::course_latest_manifest_key(TEACHING_COURSE_ID);
        let latest_bytes: Vec<u8> = fs::read(repository_root.join(&latest_manifest_key)).unwrap();
        let label: &str = &second_bundle.manifest.publication_label;

        assert!(matches!(rebuilt, artifact_bundle::CourseBundleOutcomeKind::Unchanged(_)));
        assert_eq!(latest_bytes, bundle_manifest_bytes(&second_bundle));
        assert_eq!(
            fs::read(repository_root.join(repository_key::course_schema_pointer_key(&latest_manifest_key, course_manifest::COURSE_MANIFEST_SCHEMA_VERSION))).unwrap(),
            latest_bytes,
        );
        assert!(repository_root.join(repository_key::publication_recordings_key(TEACHING_COURSE_ID, label)).is_file());

        for entry in second_bundle.manifest.file_entries() {
            assert!(repository_root.join(repository_key::publication_file_key(TEACHING_COURSE_ID, label, &entry.relative_path)).is_file());
        }

        assert_eq!(first_report.shards_uploaded, course.units.len() + 1);
        assert_eq!(first_report.recordings_uploaded, test_courses::RECORDINGS_PER_FIXTURE_COURSE);
        assert_eq!(first_report.index_change, CourseIndexChangeKind::Added);
        assert_eq!(second_report.recordings_uploaded, 0);
        assert_eq!(second_report.recordings_already_present, test_courses::RECORDINGS_PER_FIXTURE_COURSE);
        assert_eq!(second_report.index_change, CourseIndexChangeKind::Updated);
        assert_eq!(read_index(&repository).await.courses[TEACHING_COURSE_ID].title, "Renamed");
    }

    #[tokio::test]
    async fn publish_course_bundle_writes_the_publication_then_the_pointers_then_the_index() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let label: &str = &bundle.manifest.publication_label;

        publish_named(&workspace, &repository, RepositoryTargetKind::CloudflareR2, &bundle).await.unwrap();

        let put_keys: Vec<String> = repository.put_keys();
        let audio_count: usize = test_courses::RECORDINGS_PER_FIXTURE_COURSE;
        let shard_count: usize = bundle.manifest.file_entries().count();
        let latest_manifest_key: String = repository_key::course_latest_manifest_key(TEACHING_COURSE_ID);
        let expected_tail: Vec<String> = vec![
            repository_key::publication_recordings_key(TEACHING_COURSE_ID, label),
            repository_key::publication_manifest_key(TEACHING_COURSE_ID, label),
            repository_key::course_schema_pointer_key(&latest_manifest_key, course_manifest::COURSE_MANIFEST_SCHEMA_VERSION),
            latest_manifest_key.clone(),
            repository_key::COURSE_INDEX_KEY.to_string(),
        ];

        assert_eq!(put_keys.len(), audio_count + shard_count + expected_tail.len());
        assert!(put_keys[..audio_count].iter().all(|key| key.starts_with(&format!("{}/", manifest::SUBDIR_AUDIO))));
        assert!(put_keys[audio_count..audio_count + shard_count].iter().all(|key| key.ends_with(manifest::SHARD_FILE_EXTENSION)));
        assert_eq!(put_keys[audio_count + shard_count..], expected_tail);

        for operation in repository.operations() {
            let RecordedOperationKind::Put { key, headers } = operation else {
                panic!("a first publish deletes nothing");
            };

            assert_eq!(headers, ObjectHeaders::for_key(&key));
        }
    }

    #[tokio::test]
    async fn publish_course_bundle_writes_no_index_for_an_unchanged_entry_and_rewrites_it_for_a_renamed_course() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let first_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        publish_named(&workspace, &repository, RepositoryTargetKind::Local, &first_bundle).await.unwrap();

        let mut unit_change_course: Course = course.clone();

        unit_change_course.units.truncate(1);
        let unit_change_bundle: BuiltBundle = workspace.build_bundle(&unit_change_course, &plan);

        repository.clear_operations();
        let unit_change_report: CoursePublishReport =
            publish_named(&workspace, &repository, RepositoryTargetKind::Local, &unit_change_bundle).await.unwrap();
        let wrote_the_index_for_the_unit_change: bool = repository.put_keys().iter().any(|key| key == repository_key::COURSE_INDEX_KEY);

        course.title = "Renamed".to_string();
        let renamed_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        repository.clear_operations();
        let renamed_report: CoursePublishReport = publish_named(&workspace, &repository, RepositoryTargetKind::Local, &renamed_bundle).await.unwrap();

        assert_eq!(unit_change_report.index_change, CourseIndexChangeKind::Unchanged);
        assert!(!wrote_the_index_for_the_unit_change);
        assert_eq!(renamed_report.index_change, CourseIndexChangeKind::Updated);
        assert_eq!(repository.put_keys().last().map(String::as_str), Some(repository_key::COURSE_INDEX_KEY));
        assert_eq!(read_index(&repository).await.courses[TEACHING_COURSE_ID].title, "Renamed");
    }

    #[tokio::test]
    async fn publish_course_bundle_completes_on_a_rerun_after_a_failure_at_any_write() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let uninterrupted_repository: MemoryArtifactRepository = MemoryArtifactRepository::new();

        publish_named(&workspace, &uninterrupted_repository, RepositoryTargetKind::Local, &bundle).await.unwrap();

        let uninterrupted_writes: Vec<RecordedOperationKind> = uninterrupted_repository.operations();
        let index_write_position: usize = uninterrupted_repository.put_keys().iter().position(|key| key == repository_key::COURSE_INDEX_KEY).unwrap();

        for failing_write in 0..uninterrupted_writes.len() {
            let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();

            repository.fail_after_writes(failing_write);
            let interrupted: Result<CoursePublishReport, AppError> = publish_named(&workspace, &repository, RepositoryTargetKind::Local, &bundle).await;

            repository.stop_failing();
            let rerun_report: CoursePublishReport = publish_named(&workspace, &repository, RepositoryTargetKind::Local, &bundle).await.unwrap();

            assert!(interrupted.is_err(), "write {failing_write}");
            assert_eq!(repository.contents(), uninterrupted_repository.contents(), "write {failing_write}");

            if failing_write == index_write_position {
                assert_eq!(rerun_report.index_change, CourseIndexChangeKind::Added);
                assert_eq!(rerun_report.upload, PublicationUploadKind::AlreadyPresent);
            }
        }
    }

    #[tokio::test]
    async fn publish_course_bundle_relabels_a_bundle_whose_label_the_repository_holds_for_other_course_content() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let held_manifest_key: String = repository_key::publication_manifest_key(TEACHING_COURSE_ID, &bundle.manifest.publication_label);

        repository.insert(&held_manifest_key, b"another publisher's manifest");

        let report: CoursePublishReport = publish_named(&workspace, &repository, RepositoryTargetKind::Local, &bundle).await.unwrap();
        let relabeled_directory: PathBuf = workspace.paths.course_bundles_directory(TEACHING_COURSE_ID).join(&report.publication_label);

        assert_ne!(report.publication_label, bundle.manifest.publication_label);
        assert_eq!(report.upload, PublicationUploadKind::Uploaded);
        assert_eq!(repository.bytes_at(&held_manifest_key).unwrap(), b"another publisher's manifest");
        assert_eq!(
            repository.bytes_at(&repository_key::publication_manifest_key(TEACHING_COURSE_ID, &report.publication_label)),
            Some(fs::read(artifact_bundle::bundle_manifest_path(&relabeled_directory)).unwrap()),
        );
        assert!(!bundle.directory.exists());
    }

    #[tokio::test]
    async fn publish_course_bundle_refuses_a_recordings_list_that_differs_from_the_shards() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let empty_list: RecordingsList = RecordingsList {
            recording_sha256s: BTreeSet::new(),
        };

        json_document::write_json_document(&artifact_bundle::bundle_recordings_path(&bundle.directory), &empty_list).unwrap();

        let published: Result<CoursePublishReport, AppError> = publish_named(&workspace, &repository, RepositoryTargetKind::Local, &bundle).await;

        assert!(published.unwrap_err().to_string().contains("recordings list differs"));
        assert!(repository.operations().is_empty());
    }

    #[tokio::test]
    async fn publish_course_bundle_beside_a_publication_whose_label_begins_with_its_own_uploads_every_shard() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let built_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let mut charlotte_manifest: CourseManifest = built_bundle.manifest.clone();

        charlotte_manifest.publication_label = "2026-10-08+charlotte".to_string();
        json_document::write_json_document(&artifact_bundle::bundle_manifest_path(&built_bundle.directory), &charlotte_manifest).unwrap();

        let bundle: BuiltBundle = BuiltBundle {
            directory: built_bundle.directory.clone(),
            manifest: charlotte_manifest,
        };

        for entry in bundle.manifest.file_entries() {
            repository.insert(&repository_key::publication_file_key(TEACHING_COURSE_ID, "2026-10-08+charlottesville", &entry.relative_path), b"shard");
        }

        repository.insert(&repository_key::publication_manifest_key(TEACHING_COURSE_ID, "2026-10-08+charlottesville"), b"{}");

        let report: CoursePublishReport = publish_named(&workspace, &repository, RepositoryTargetKind::CloudflareR2, &bundle).await.unwrap();

        assert_eq!(report.publication_label, "2026-10-08+charlotte");
        assert_eq!(report.shards_uploaded, bundle.manifest.file_entries().count());
        assert_eq!(repository.bytes_at(&repository_key::publication_manifest_key(TEACHING_COURSE_ID, "2026-10-08+charlottesville")).unwrap(), b"{}");
    }

    #[tokio::test]
    async fn publish_selected_courses_continues_after_a_course_fails() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let mut second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let both_courses: CourseSelectionKind = CourseSelectionKind::Named(vec![TEACHING_COURSE_ID.to_string(), SECOND_COURSE_ID.to_string()]);

        workspace.build_bundle(&teaching_course, &plan);
        workspace.build_bundle(&second_course, &plan);
        publish_selected_courses(&workspace.paths, &repository, RepositoryTargetKind::Local, &both_courses).await.unwrap();

        teaching_course.title = "Renamed".to_string();
        second_course.title = "Renamed".to_string();
        let broken_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);
        let second_bundle: BuiltBundle = workspace.build_bundle(&second_course, &plan);

        fs::write(artifact_bundle::bundle_recordings_path(&broken_bundle.directory), b"not json").unwrap();

        let every_course: CourseSelectionKind = CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Exclude);
        let outcomes: Vec<CoursePublishOutcome> = publish_selected_courses(&workspace.paths, &repository, RepositoryTargetKind::Local, &every_course).await.unwrap();

        assert_eq!(
            outcome_ids_and_successes(&outcomes),
            vec![(SECOND_COURSE_ID.to_string(), true), (TEACHING_COURSE_ID.to_string(), false)],
        );
        assert_eq!(latest_pointer_bytes(&repository, SECOND_COURSE_ID), Some(bundle_manifest_bytes(&second_bundle)));
    }

    #[tokio::test]
    async fn publish_selected_courses_adds_an_unlisted_course_only_when_asked() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();

        workspace.build_bundle(&teaching_course, &plan);
        workspace.build_bundle(&second_course, &plan);
        publish_selected_courses(&workspace.paths, &repository, RepositoryTargetKind::Local, &CourseSelectionKind::Named(vec![TEACHING_COURSE_ID.to_string()]))
            .await
            .unwrap();

        let listed_only: Vec<CoursePublishOutcome> = publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::Local,
            &CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Exclude),
        )
        .await
        .unwrap();
        let index_after_listed_only: CourseIndex = read_index(&repository).await;
        let with_unlisted: Vec<CoursePublishOutcome> = publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::Local,
            &CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Include),
        )
        .await
        .unwrap();

        assert_eq!(outcome_ids_and_successes(&listed_only), vec![(TEACHING_COURSE_ID.to_string(), true)]);
        assert!(!index_after_listed_only.courses.contains_key(SECOND_COURSE_ID));
        assert_eq!(
            outcome_ids_and_successes(&with_unlisted),
            vec![(SECOND_COURSE_ID.to_string(), true), (TEACHING_COURSE_ID.to_string(), true)],
        );
        assert!(read_index(&repository).await.courses.contains_key(SECOND_COURSE_ID));
    }

    #[tokio::test]
    async fn publish_selected_courses_leaves_a_pointer_on_a_publication_uploaded_after_the_bundles() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();

        publish_named(&workspace, &repository, RepositoryTargetKind::Local, &bundle).await.unwrap();

        let newer_bytes: Vec<u8> = insert_newer_publication(&repository, &bundle);
        let embedded_before: Option<Vec<u8>> = embedded_bundle::read_embedded_course_manifest_bytes(&workspace.paths, TEACHING_COURSE_ID).unwrap();

        let outcomes: Vec<CoursePublishOutcome> = publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::Local,
            &CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Exclude),
        )
        .await
        .unwrap();
        let report: &CoursePublishReport = outcomes[0].result.as_ref().unwrap();

        assert_eq!(report.pointer, PointerChangeKind::LeftOnAnotherPublication);
        assert_eq!(latest_pointer_bytes(&repository, TEACHING_COURSE_ID), Some(newer_bytes));
        assert_eq!(embedded_bundle::read_embedded_course_manifest_bytes(&workspace.paths, TEACHING_COURSE_ID).unwrap(), embedded_before);
    }

    #[tokio::test]
    async fn publish_selected_courses_moves_the_pointer_to_a_publication_an_interrupted_publish_uploaded() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let every_course: CourseSelectionKind = CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Exclude);
        let first_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        publish_named(&workspace, &repository, RepositoryTargetKind::Local, &first_bundle).await.unwrap();

        course.title = "Renamed".to_string();
        let second_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let writes_through_the_manifest: usize = second_bundle.manifest.file_entries().count() + 2;

        repository.fail_after_writes(writes_through_the_manifest);
        let interrupted: Vec<CoursePublishOutcome> = publish_selected_courses(&workspace.paths, &repository, RepositoryTargetKind::Local, &every_course).await.unwrap();

        repository.stop_failing();
        let rerun: Vec<CoursePublishOutcome> = publish_selected_courses(&workspace.paths, &repository, RepositoryTargetKind::Local, &every_course).await.unwrap();
        let rerun_report: &CoursePublishReport = rerun[0].result.as_ref().unwrap();

        assert!(interrupted[0].result.is_err());
        assert_eq!(rerun_report.upload, PublicationUploadKind::AlreadyPresent);
        assert_eq!(rerun_report.pointer, PointerChangeKind::Moved);
        assert_eq!(latest_pointer_bytes(&repository, TEACHING_COURSE_ID), Some(bundle_manifest_bytes(&second_bundle)));
    }

    #[tokio::test]
    async fn publish_selected_courses_on_cloudflare_r2_leaves_out_test_course_bundles() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let test_course: Course = test_courses::fixture_course(&mut plan, TEST_COURSE_ID, true);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();

        workspace.build_bundle(&teaching_course, &plan);
        workspace.build_bundle(&test_course, &plan);

        let outcomes: Vec<CoursePublishOutcome> = publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::CloudflareR2,
            &CourseSelectionKind::EveryCourse(UnlistedCourseInclusionKind::Include),
        )
        .await
        .unwrap();

        assert_eq!(outcome_ids_and_successes(&outcomes), vec![(TEACHING_COURSE_ID.to_string(), true)]);
        assert!(repository.put_keys().iter().all(|key| !key.contains(TEST_COURSE_ID)));
        assert_eq!(read_index(&repository).await.courses.keys().collect::<Vec<&String>>(), vec![TEACHING_COURSE_ID]);
    }

    #[tokio::test]
    async fn publish_selected_courses_on_cloudflare_r2_refuses_a_named_test_course_before_any_upload() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let test_course: Course = test_courses::fixture_course(&mut plan, TEST_COURSE_ID, true);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let test_course_bundle: BuiltBundle = workspace.build_bundle(&test_course, &plan);

        let named: Result<Vec<CoursePublishOutcome>, AppError> = publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::CloudflareR2,
            &CourseSelectionKind::Named(vec![TEST_COURSE_ID.to_string()]),
        )
        .await;
        let by_bundle: Result<Vec<CoursePublishOutcome>, AppError> = publish_selected_courses(
            &workspace.paths,
            &repository,
            RepositoryTargetKind::CloudflareR2,
            &CourseSelectionKind::Bundle(test_course_bundle.directory.clone()),
        )
        .await;

        assert!(named.is_err_and(|error| error.to_string().contains(TEST_COURSE_ID)));
        assert!(by_bundle.is_err());
        assert!(repository.operations().is_empty());
    }

    #[tokio::test]
    async fn publish_selected_courses_on_cloudflare_r2_refuses_an_index_that_lists_a_test_course_before_any_upload() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let test_course: Course = test_courses::fixture_course(&mut plan, TEST_COURSE_ID, true);

        workspace.build_bundle(&teaching_course, &plan);
        workspace.write_curriculum(&test_course);

        for stored_flag in [false, true] {
            let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
            let listing_index: CourseIndex = CourseIndex {
                course_index_schema_version: shared::artifact::COURSE_INDEX_SCHEMA_VERSION,
                courses: BTreeMap::from([(
                    TEST_COURSE_ID.to_string(),
                    CourseIndexEntry {
                        title: test_course.title.clone(),
                        target_language: test_course.target_language.clone(),
                        source_language: test_course.source_language.clone(),
                        latest_manifest_key: repository_key::course_latest_manifest_key(TEST_COURSE_ID),
                        test_course: stored_flag,
                    },
                )]),
            };

            repository.insert(repository_key::COURSE_INDEX_KEY, &json_document::json_document_bytes(&listing_index).unwrap());

            let published: Result<Vec<CoursePublishOutcome>, AppError> = publish_selected_courses(
                &workspace.paths,
                &repository,
                RepositoryTargetKind::CloudflareR2,
                &CourseSelectionKind::Named(vec![TEACHING_COURSE_ID.to_string()]),
            )
            .await;

            assert!(published.is_err_and(|error| error.to_string().contains(&format!("remove-course cloudflare-r2 --course {TEST_COURSE_ID}"))));
            assert!(repository.operations().is_empty());
        }
    }

    #[tokio::test]
    async fn publish_refreshes_only_the_published_course_in_the_embedded_bundle_and_never_a_test_course() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let test_course: Course = test_courses::fixture_course(&mut plan, TEST_COURSE_ID, true);
        let repository: MemoryArtifactRepository = MemoryArtifactRepository::new();
        let teaching_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);
        let second_bundle: BuiltBundle = workspace.build_bundle(&second_course, &plan);
        let test_course_bundle: BuiltBundle = workspace.build_bundle(&test_course, &plan);

        for bundle in [&teaching_bundle, &second_bundle, &test_course_bundle] {
            publish_named(&workspace, &repository, RepositoryTargetKind::Local, bundle).await.unwrap();
        }

        teaching_course.title = "Renamed".to_string();
        let renamed_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);

        publish_named(&workspace, &repository, RepositoryTargetKind::Local, &renamed_bundle).await.unwrap();

        let embedded_index: CourseIndex = embedded_bundle::read_embedded_course_index(&workspace.paths).unwrap().unwrap();

        assert_eq!(embedded_index.courses.keys().collect::<Vec<&String>>(), vec![SECOND_COURSE_ID, TEACHING_COURSE_ID]);
        assert_eq!(embedded_index.courses[TEACHING_COURSE_ID].title, "Renamed");
        assert_eq!(
            embedded_bundle::read_embedded_publication_label(&workspace.paths, TEACHING_COURSE_ID),
            Some(renamed_bundle.manifest.publication_label.clone()),
        );
        assert_eq!(
            embedded_bundle::read_embedded_publication_label(&workspace.paths, SECOND_COURSE_ID),
            Some(second_bundle.manifest.publication_label.clone()),
        );
        assert!(!workspace.paths.embedded_course_directory(TEST_COURSE_ID).exists());
    }

    #[test]
    fn check_bundle_publishable_refuses_a_test_course_only_for_cloudflare_r2() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let test_course: Course = test_courses::fixture_course(&mut plan, TEST_COURSE_ID, true);
        let teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let test_course_manifest: CourseManifest = workspace.build_bundle(&test_course, &plan).manifest;
        let teaching_manifest: CourseManifest = workspace.build_bundle(&teaching_course, &plan).manifest;

        let refused: Result<(), AppError> = check_bundle_publishable(RepositoryTargetKind::CloudflareR2, &test_course_manifest);

        assert!(refused.unwrap_err().to_string().contains(TEST_COURSE_ID));
        assert!(check_bundle_publishable(RepositoryTargetKind::Local, &test_course_manifest).is_ok());
        assert!(check_bundle_publishable(RepositoryTargetKind::Dry, &test_course_manifest).is_ok());
        assert!(check_bundle_publishable(RepositoryTargetKind::CloudflareR2, &teaching_manifest).is_ok());
    }
}
