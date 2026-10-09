use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use shared::AppError;
use shared::artifact::{self, CourseManifest, ManifestEntry, compression, content_address, course_manifest, manifest, repository_key};
use shared::course::{Course, course_identity};
use shared::course_shard::{self, ClipAudioEntry};

use crate::artifact::course_shard_writer::{self, ClipAudioIndex, ClipCatalog};
use crate::artifact::embedded_bundle;
use crate::artifact::json_document;
use crate::artifact::local_files;
use crate::artifact::recordings_list::{self, RecordingsList};
use crate::audio_plan::AudioPlan;
use crate::publication_label;
use crate::workspace::WorkspacePaths;

pub const BUNDLES_KEPT_PER_COURSE: usize = 2;

const PARTIAL_BUNDLE_PREFIX: &str = ".partial-";
const PARTIAL_SHARD_FILE_NAME: &str = ".shard.part";
const OUTLINE_SHARD_STEM: &str = "course";

/// The recordings available to a bundle.
pub struct ClipRecordings {
    pub audio_index: ClipAudioIndex,
    pub file_by_audio_sha256: BTreeMap<String, PathBuf>,
    pub pipeline_revision_by_engine_code: BTreeMap<String, u32>,
}

/// One course's publication as built, before any repository holds it.
#[derive(Debug, Clone, PartialEq)]
pub struct BuiltBundle {
    pub directory: PathBuf,
    pub manifest: CourseManifest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CourseBundleOutcomeKind {
    Written(BuiltBundle),
    /// The course's newest bundle, whose course content this build reproduced.
    Unchanged(BuiltBundle),
}

/// Writes the course's bundle under its own bundles directory; it appears under its publication label only once
/// finished.
pub fn write_course_bundle(
    paths: &WorkspacePaths,
    course: &Course,
    plan: &AudioPlan,
    recordings: &ClipRecordings,
) -> Result<CourseBundleOutcomeKind, AppError> {
    let artifact_created: DateTime<Utc> = Utc::now();
    let taken_labels: BTreeSet<String> = read_taken_publication_labels(paths, &course.id)?;
    let publication_label: String = publication_label::generate(artifact_created, &taken_labels)?;
    let course_bundles_directory: PathBuf = paths.course_bundles_directory(&course.id);
    let partial_directory: PathBuf = course_bundles_directory.join(format!("{PARTIAL_BUNDLE_PREFIX}{publication_label}"));
    let bundle_directory: PathBuf = course_bundles_directory.join(&publication_label);

    local_files::remove_directory_if_present(&partial_directory)?;

    let clip_catalog: ClipCatalog = ClipCatalog {
        plan,
        audio_index: &recordings.audio_index,
    };
    let (outline, units): (ManifestEntry, BTreeMap<String, ManifestEntry>) = write_course_shards(&partial_directory, course, &clip_catalog)?;
    let clip_audio: Vec<ClipAudioEntry> = read_verified_clip_audio(&partial_directory, &outline, units.values())?;
    let recordings_list: RecordingsList = RecordingsList {
        recording_sha256s: clip_audio.iter().map(|clip_audio_entry| clip_audio_entry.audio_sha256.clone()).collect(),
    };

    let built_manifest: CourseManifest = CourseManifest {
        manifest_schema_version: course_manifest::COURSE_MANIFEST_SCHEMA_VERSION,
        course_id: course.id.clone(),
        publication_label: publication_label.clone(),
        artifact_created,
        title: course.title.clone(),
        target_language: course.target_language.clone(),
        source_language: course.source_language.clone(),
        test_course: course.test_course,
        outline,
        units,
        voice_pipeline_revisions: revisions_of_engines_used(&recordings.pipeline_revision_by_engine_code, &clip_audio),
    };

    let newest_bundle: Option<BuiltBundle> = find_newest_course_bundle(paths, &course.id)?;

    if let Some(newest_bundle) = newest_bundle {
        if has_the_same_course_content(&newest_bundle.manifest, &built_manifest) {
            fs::remove_dir_all(&partial_directory)?;
            log::info!(
                "course content unchanged; [course_id={} publication_label={}]",
                course.id,
                newest_bundle.manifest.publication_label,
            );

            return Ok(CourseBundleOutcomeKind::Unchanged(newest_bundle));
        }
    }

    let linked_audio_count: usize = link_audio_files(&partial_directory, &recordings.file_by_audio_sha256, &recordings_list)?;

    json_document::write_json_document(&partial_directory.join(repository_key::RECORDINGS_FILENAME), &recordings_list)?;
    json_document::write_json_document(&partial_directory.join(manifest::MANIFEST_FILENAME), &built_manifest)?;
    fs::rename(&partial_directory, &bundle_directory)?;

    log::info!(
        "wrote bundle; [course_id={} publication_label={publication_label} shards={} audio_files={linked_audio_count} path={}]",
        course.id,
        built_manifest.file_entries().count(),
        bundle_directory.display(),
    );

    Ok(CourseBundleOutcomeKind::Written(BuiltBundle {
        directory: bundle_directory,
        manifest: built_manifest,
    }))
}

fn has_the_same_course_content(existing_manifest: &CourseManifest, built_manifest: &CourseManifest) -> bool {
    let mut comparable_manifest: CourseManifest = existing_manifest.clone();

    comparable_manifest.publication_label = built_manifest.publication_label.clone();
    comparable_manifest.artifact_created = built_manifest.artifact_created;

    comparable_manifest == *built_manifest
}

/// The revisions of the engines whose recordings the shards name.
fn revisions_of_engines_used(pipeline_revision_by_engine_code: &BTreeMap<String, u32>, clip_audio: &[ClipAudioEntry]) -> BTreeMap<String, u32> {
    let used_engine_codes: BTreeSet<&str> = clip_audio.iter().map(|clip_audio_entry| clip_audio_entry.engine.code()).collect();

    pipeline_revision_by_engine_code
        .iter()
        .filter(|(engine_code, _)| used_engine_codes.contains(engine_code.as_str()))
        .map(|(engine_code, revision)| (engine_code.clone(), *revision))
        .collect()
}

/// Every label the course's local bundles, its publications in the local repository and its embedded publication use.
pub fn read_taken_publication_labels(paths: &WorkspacePaths, course_id: &str) -> Result<BTreeSet<String>, AppError> {
    let mut taken_labels: BTreeSet<String> = list_publication_directory_names(&paths.course_bundles_directory(course_id))?;

    taken_labels.extend(list_publication_directory_names(&paths.local_repository_course_directory(course_id))?);
    taken_labels.extend(embedded_bundle::read_embedded_publication_label(paths, course_id));

    Ok(taken_labels)
}

/* A publication is any directory except the pointers and a bundle still being written. */
fn list_publication_directory_names(parent: &Path) -> Result<BTreeSet<String>, AppError> {
    let mut names: BTreeSet<String> = BTreeSet::new();

    if !parent.is_dir() {
        return Ok(names);
    }

    for directory_entry in fs::read_dir(parent)? {
        let directory_entry: fs::DirEntry = directory_entry?;
        let name: String = directory_entry.file_name().to_string_lossy().into_owned();
        let is_publication: bool = directory_entry.file_type()?.is_dir() && !name.starts_with('.') && name != repository_key::COURSE_LATEST_DIRECTORY;

        if is_publication {
            names.insert(name);
        }
    }

    Ok(names)
}

/// Writes the course's outline shard and one shard per unit under `data/`.
fn write_course_shards(
    bundle_directory: &Path,
    course: &Course,
    clip_catalog: &ClipCatalog,
) -> Result<(ManifestEntry, BTreeMap<String, ManifestEntry>), AppError> {
    let partial_path: PathBuf = bundle_directory.join(manifest::SUBDIR_DATA).join(PARTIAL_SHARD_FILE_NAME);

    fs::create_dir_all(bundle_directory.join(manifest::SUBDIR_DATA))?;
    course_shard_writer::write_course_shard(&partial_path, course, clip_catalog)?;

    let outline: ManifestEntry = name_shard_by_content(bundle_directory, &partial_path, OUTLINE_SHARD_STEM)?;
    let mut units: BTreeMap<String, ManifestEntry> = BTreeMap::new();

    for unit in &course.units {
        course_shard_writer::write_unit_shard(&partial_path, unit, clip_catalog)?;
        units.insert(unit.id.clone(), name_shard_by_content(bundle_directory, &partial_path, &format!("unit-{}", unit.id))?);
    }

    Ok((outline, units))
}

/* The hash covers the compressed bytes, which are what a client downloads and verifies. */
fn name_shard_by_content(bundle_directory: &Path, partial_path: &Path, stem: &str) -> Result<ManifestEntry, AppError> {
    let plain_bytes: Vec<u8> = fs::read(partial_path)?;
    let compressed_bytes: Vec<u8> = compression::compress(&plain_bytes)?;
    let sha256: String = content_address::sha256_hex(&compressed_bytes);
    let file_name: String = content_address::content_addressed_file_name(stem, &sha256, manifest::SHARD_FILE_EXTENSION);
    let relative_path: String = format!("{}/{file_name}", manifest::SUBDIR_DATA);

    fs::write(bundle_directory.join(&relative_path), &compressed_bytes)?;
    fs::remove_file(partial_path)?;

    Ok(ManifestEntry {
        relative_path,
        size_bytes: compressed_bytes.len() as u64,
        sha256,
    })
}

/* Hard links, because the recordings already exist in the clip store and run to hundreds of megabytes. */
fn link_audio_files(
    bundle_directory: &Path,
    file_by_audio_sha256: &BTreeMap<String, PathBuf>,
    recordings_list: &RecordingsList,
) -> Result<usize, AppError> {
    fs::create_dir_all(bundle_directory.join(manifest::SUBDIR_AUDIO))?;

    for audio_sha256 in &recordings_list.recording_sha256s {
        let Some(source_path) = file_by_audio_sha256.get(audio_sha256) else {
            return Err(AppError::from(format!("a recording the shards name is not in the clip store; [sha256={audio_sha256}]")));
        };

        fs::hard_link(source_path, audio_file_path(bundle_directory, audio_sha256))?;
    }

    Ok(recordings_list.recording_sha256s.len())
}

pub fn bundle_manifest_path(bundle_directory: &Path) -> PathBuf {
    bundle_directory.join(manifest::MANIFEST_FILENAME)
}

pub fn read_bundle_manifest(bundle_directory: &Path) -> Result<CourseManifest, AppError> {
    let manifest_path: PathBuf = bundle_manifest_path(bundle_directory);
    let manifest_bytes: Vec<u8> = fs::read(&manifest_path)
        .map_err(|error| AppError::from(format!("could not read a bundle manifest; [path={} error={error}]", manifest_path.display())))?;

    artifact::parse_course_manifest(&manifest_bytes, None)
}

pub fn bundle_recordings_path(bundle_directory: &Path) -> PathBuf {
    bundle_directory.join(repository_key::RECORDINGS_FILENAME)
}

pub fn read_bundle_recordings(bundle_directory: &Path) -> Result<RecordingsList, AppError> {
    let recordings_path: PathBuf = bundle_recordings_path(bundle_directory);
    let recordings_bytes: Vec<u8> = fs::read(&recordings_path)
        .map_err(|error| AppError::from(format!("could not read a bundle's recordings list; [path={} error={error}]", recordings_path.display())))?;

    recordings_list::parse_recordings_list(&recordings_bytes)
}

struct ListedBundle {
    directory: PathBuf,
    /// `None` when the bundle's manifest is unreadable.
    manifest: Option<CourseManifest>,
}

/* A bundle whose manifest is unreadable sorts first. */
fn list_course_bundles_oldest_first(paths: &WorkspacePaths, course_id: &str) -> Result<Vec<ListedBundle>, AppError> {
    let course_bundles_directory: PathBuf = paths.course_bundles_directory(course_id);
    let mut listed_bundles: Vec<ListedBundle> = Vec::new();

    for name in list_publication_directory_names(&course_bundles_directory)? {
        let directory: PathBuf = course_bundles_directory.join(&name);
        let read_manifest: Result<CourseManifest, AppError> = read_bundle_manifest(&directory);

        let bundle_manifest: Option<CourseManifest> = match read_manifest {
            Ok(bundle_manifest) => Some(bundle_manifest),
            Err(error) => {
                log::warn!("a bundle's manifest is unreadable; [path={} error={error}]", directory.display());
                None
            },
        };

        listed_bundles.push(ListedBundle {
            directory,
            manifest: bundle_manifest,
        });
    }

    listed_bundles.sort_by_key(|listed_bundle| listed_bundle.manifest.as_ref().map(|bundle_manifest| bundle_manifest.artifact_created));

    Ok(listed_bundles)
}

/// The course's newest bundle with a readable manifest.
pub fn find_newest_course_bundle(paths: &WorkspacePaths, course_id: &str) -> Result<Option<BuiltBundle>, AppError> {
    let listed_bundles: Vec<ListedBundle> = list_course_bundles_oldest_first(paths, course_id)?;
    let newest_bundle: Option<BuiltBundle> = listed_bundles.into_iter().rev().find_map(|listed_bundle| {
        listed_bundle.manifest.map(|bundle_manifest| BuiltBundle {
            directory: listed_bundle.directory,
            manifest: bundle_manifest,
        })
    });

    Ok(newest_bundle)
}

/// Removes all but the course's newest bundles, and any of its bundles left partial by an interrupted build.
pub fn prune_course_bundles(paths: &WorkspacePaths, course_id: &str) -> Result<(), AppError> {
    let listed_bundles: Vec<ListedBundle> = list_course_bundles_oldest_first(paths, course_id)?;
    let removable_count: usize = listed_bundles.len().saturating_sub(BUNDLES_KEPT_PER_COURSE);

    for listed_bundle in &listed_bundles[..removable_count] {
        fs::remove_dir_all(&listed_bundle.directory)?;
    }

    let course_bundles_directory: PathBuf = paths.course_bundles_directory(course_id);

    if !course_bundles_directory.is_dir() {
        return Ok(());
    }

    for directory_entry in fs::read_dir(&course_bundles_directory)? {
        let directory_entry: fs::DirEntry = directory_entry?;
        let is_partial: bool = directory_entry.file_name().to_string_lossy().starts_with(PARTIAL_BUNDLE_PREFIX);

        if is_partial {
            fs::remove_dir_all(directory_entry.path())?;
        }
    }

    Ok(())
}

/// Removes the bundles of every course id absent from `data_course_ids`; a directory not named like a course is left.
pub fn remove_bundles_of_absent_courses(paths: &WorkspacePaths, data_course_ids: &[String]) -> Result<(), AppError> {
    if !paths.bundles_directory.is_dir() {
        return Ok(());
    }

    for directory_entry in fs::read_dir(&paths.bundles_directory)? {
        let directory_entry: fs::DirEntry = directory_entry?;
        let name: String = directory_entry.file_name().to_string_lossy().into_owned();
        let is_absent_course: bool = directory_entry.file_type()?.is_dir()
            && course_identity::is_well_formed_course_id(&name)
            && !data_course_ids.contains(&name);

        if is_absent_course {
            fs::remove_dir_all(directory_entry.path())?;
            log::info!("removed the bundles of a course absent from the data; [course_id={name}]");
        }
    }

    Ok(())
}

/// Gives the bundle a publication label that neither `taken_labels` nor the workspace uses.
pub fn relabel_bundle(paths: &WorkspacePaths, bundle: &BuiltBundle, taken_labels: &BTreeSet<String>) -> Result<BuiltBundle, AppError> {
    let mut every_taken_label: BTreeSet<String> = read_taken_publication_labels(paths, &bundle.manifest.course_id)?;

    every_taken_label.extend(taken_labels.iter().cloned());
    every_taken_label.insert(bundle.manifest.publication_label.clone());

    let publication_label: String = publication_label::generate(bundle.manifest.artifact_created, &every_taken_label)?;
    let mut relabeled_manifest: CourseManifest = bundle.manifest.clone();

    relabeled_manifest.publication_label = publication_label.clone();

    let Some(course_bundles_directory) = bundle.directory.parent() else {
        return Err(AppError::from(format!("a bundle directory has no parent; [path={}]", bundle.directory.display())));
    };
    let relabeled_directory: PathBuf = course_bundles_directory.join(&publication_label);

    json_document::write_json_document(&bundle_manifest_path(&bundle.directory), &relabeled_manifest)?;
    fs::rename(&bundle.directory, &relabeled_directory)?;

    log::info!(
        "relabeled a bundle whose label the repository holds for other course content; [course_id={} previous_label={} publication_label={publication_label}]",
        bundle.manifest.course_id,
        bundle.manifest.publication_label,
    );

    Ok(BuiltBundle {
        directory: relabeled_directory,
        manifest: relabeled_manifest,
    })
}

/// Every recording hash the bundle's shards name, after checking each shard against its manifest hash.
pub fn referenced_audio_sha256s(bundle_directory: &Path, bundle_manifest: &CourseManifest) -> Result<BTreeSet<String>, AppError> {
    let clip_audio: Vec<ClipAudioEntry> = read_verified_clip_audio(bundle_directory, &bundle_manifest.outline, bundle_manifest.units.values())?;

    Ok(clip_audio.into_iter().map(|clip_audio_entry| clip_audio_entry.audio_sha256).collect())
}

fn read_verified_clip_audio<'a>(
    bundle_directory: &Path,
    outline: &ManifestEntry,
    units: impl Iterator<Item = &'a ManifestEntry>,
) -> Result<Vec<ClipAudioEntry>, AppError> {
    let outline_bytes: Vec<u8> = read_verified_shard(bundle_directory, outline)?;
    let mut clip_audio: Vec<ClipAudioEntry> = course_shard::read_course_shard(&outline_bytes)?.clip_audio;

    for entry in units {
        let unit_bytes: Vec<u8> = read_verified_shard(bundle_directory, entry)?;

        clip_audio.extend(course_shard::read_unit_shard(&unit_bytes)?.clip_audio);
    }

    Ok(clip_audio)
}

/// The shard's decompressed bytes, after checking the stored bytes against the manifest.
fn read_verified_shard(bundle_directory: &Path, entry: &ManifestEntry) -> Result<Vec<u8>, AppError> {
    let compressed_bytes: Vec<u8> = fs::read(bundle_directory.join(&entry.relative_path))?;

    content_address::verify_sha256(&entry.relative_path, &compressed_bytes, &entry.sha256)?;

    compression::decompress(&compressed_bytes)
}

pub fn audio_file_path(bundle_directory: &Path, audio_sha256: &str) -> PathBuf {
    bundle_directory.join(artifact::audio_key(audio_sha256))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_courses::{self, FixtureWorkspace, SECOND_COURSE_ID, TEACHING_COURSE_ID};

    fn bundle_names(paths: &WorkspacePaths, course_id: &str) -> BTreeSet<String> {
        list_publication_directory_names(&paths.course_bundles_directory(course_id)).unwrap()
    }

    #[test]
    fn write_course_bundle_writes_the_course_layout_and_its_recordings_list() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);

        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);
        let recordings_list: RecordingsList = read_bundle_recordings(&bundle.directory).unwrap();

        assert_eq!(bundle.directory, workspace.paths.course_bundles_directory(TEACHING_COURSE_ID).join(&bundle.manifest.publication_label));
        assert_eq!(read_bundle_manifest(&bundle.directory).unwrap(), bundle.manifest);
        assert_eq!(bundle.manifest.course_id, TEACHING_COURSE_ID);
        assert_eq!(bundle.manifest.units.len(), course.units.len());
        assert!(bundle.manifest.file_entries().all(|entry| entry.relative_path.starts_with(&format!("{}/", manifest::SUBDIR_DATA))));
        assert!(bundle.manifest.file_entries().all(|entry| bundle.directory.join(&entry.relative_path).is_file()));
        assert_eq!(recordings_list.recording_sha256s, referenced_audio_sha256s(&bundle.directory, &bundle.manifest).unwrap());
        assert_eq!(recordings_list.recording_sha256s.len(), test_courses::RECORDINGS_PER_FIXTURE_COURSE);
        assert!(recordings_list.recording_sha256s.iter().all(|audio_sha256| audio_file_path(&bundle.directory, audio_sha256).is_file()));
    }

    #[test]
    fn write_course_bundle_records_the_revisions_of_only_the_engines_the_course_uses() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);

        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        assert_eq!(bundle.manifest.voice_pipeline_revisions, BTreeMap::from([("kokoro".to_string(), 3)]));
    }

    #[test]
    fn write_course_bundle_returns_the_newest_bundle_when_the_course_content_is_unchanged() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let first_bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        let second_outcome: CourseBundleOutcomeKind = write_course_bundle(&workspace.paths, &course, &plan, &workspace.recordings_for(&course)).unwrap();

        assert_eq!(second_outcome, CourseBundleOutcomeKind::Unchanged(first_bundle.clone()));
        assert_eq!(bundle_names(&workspace.paths, TEACHING_COURSE_ID), BTreeSet::from([first_bundle.manifest.publication_label]));
        assert_eq!(fs::read_dir(workspace.paths.course_bundles_directory(TEACHING_COURSE_ID)).unwrap().count(), 1);
    }

    #[test]
    fn courses_take_publication_labels_only_from_their_own_bundles() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);

        let teaching_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);
        let second_bundle: BuiltBundle = workspace.build_bundle(&second_course, &plan);

        assert_eq!(
            read_taken_publication_labels(&workspace.paths, TEACHING_COURSE_ID).unwrap(),
            BTreeSet::from([teaching_bundle.manifest.publication_label.clone()]),
        );
        assert_eq!(
            read_taken_publication_labels(&workspace.paths, SECOND_COURSE_ID).unwrap(),
            BTreeSet::from([second_bundle.manifest.publication_label.clone()]),
        );
    }

    #[test]
    fn prune_course_bundles_keeps_the_newest_of_one_course_and_leaves_another_courses_bundles() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let mut teaching_labels: Vec<String> = Vec::new();

        workspace.build_bundle(&second_course, &plan);

        for title in ["First title", "Second title", "Third title"] {
            teaching_course.title = title.to_string();
            teaching_labels.push(workspace.build_bundle(&teaching_course, &plan).manifest.publication_label);
        }

        fs::create_dir_all(workspace.paths.course_bundles_directory(TEACHING_COURSE_ID).join(format!("{PARTIAL_BUNDLE_PREFIX}left-over"))).unwrap();

        prune_course_bundles(&workspace.paths, TEACHING_COURSE_ID).unwrap();

        assert_eq!(bundle_names(&workspace.paths, TEACHING_COURSE_ID), teaching_labels[1..].iter().cloned().collect::<BTreeSet<String>>());
        assert_eq!(fs::read_dir(workspace.paths.course_bundles_directory(TEACHING_COURSE_ID)).unwrap().count(), BUNDLES_KEPT_PER_COURSE);
        assert_eq!(bundle_names(&workspace.paths, SECOND_COURSE_ID).len(), 1);
    }

    #[test]
    fn remove_bundles_of_absent_courses_leaves_present_courses_and_directories_not_named_like_a_course() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let old_layout_bundle: PathBuf = workspace.paths.bundles_directory.join("2026-10-08+duluth");
        let absent_course_bundles: PathBuf = workspace.paths.course_bundles_directory(SECOND_COURSE_ID);
        let present_course_bundles: PathBuf = workspace.paths.course_bundles_directory(TEACHING_COURSE_ID);

        for directory in [&old_layout_bundle, &absent_course_bundles, &present_course_bundles] {
            fs::create_dir_all(directory).unwrap();
        }

        remove_bundles_of_absent_courses(&workspace.paths, &[TEACHING_COURSE_ID.to_string()]).unwrap();

        assert!(old_layout_bundle.exists());
        assert!(present_course_bundles.exists());
        assert!(!absent_course_bundles.exists());
    }

    #[test]
    fn relabel_bundle_renames_the_directory_and_rewrites_the_manifest() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let bundle: BuiltBundle = workspace.build_bundle(&course, &plan);

        let relabeled: BuiltBundle = relabel_bundle(&workspace.paths, &bundle, &BTreeSet::new()).unwrap();

        assert_ne!(relabeled.manifest.publication_label, bundle.manifest.publication_label);
        assert!(!bundle.directory.exists());
        assert_eq!(relabeled.directory.file_name().unwrap().to_string_lossy(), relabeled.manifest.publication_label);
        assert_eq!(read_bundle_manifest(&relabeled.directory).unwrap(), relabeled.manifest);
        assert!(has_the_same_course_content(&bundle.manifest, &relabeled.manifest));
    }
}
