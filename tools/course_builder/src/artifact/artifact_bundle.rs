use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use shared::AppError;
use shared::artifact::{self, Manifest, ManifestCourse, ManifestEntry, compression, content_address, manifest};
use shared::course::Course;
use shared::course_shard::{self, ClipAudioEntry};

use crate::artifact::course_shard_writer::{self, ClipAudioIndex, ClipCatalog};
use crate::audio_plan::AudioPlan;
use crate::version_label;
use crate::workspace::WorkspacePaths;

const PARTIAL_BUNDLE_PREFIX: &str = ".partial-";
const PARTIAL_SHARD_FILE_NAME: &str = ".shard.part";
const BUNDLES_KEPT: usize = 2;

/// The recordings available to a bundle.
pub struct ClipRecordings {
    pub audio_index: ClipAudioIndex,
    pub file_by_audio_sha256: BTreeMap<String, PathBuf>,
    pub pipeline_revision_by_engine_code: BTreeMap<String, u32>,
}

pub struct BuiltBundle {
    pub directory: PathBuf,
    pub manifest: Manifest,
}

/// Writes a complete bundle under the bundles directory; it appears under its version name only once finished.
pub fn write_bundle(
    paths: &WorkspacePaths,
    courses: &[Course],
    plan: &AudioPlan,
    recordings: &ClipRecordings,
) -> Result<BuiltBundle, AppError> {
    let artifact_created: DateTime<Utc> = Utc::now();
    let taken_versions: BTreeSet<String> = read_taken_versions(paths)?;
    let version: String = version_label::generate(artifact_created, &taken_versions)?;
    let partial_directory: PathBuf = paths.bundles_directory.join(format!("{PARTIAL_BUNDLE_PREFIX}{version}"));
    let bundle_directory: PathBuf = paths.bundles_directory.join(&version);

    if partial_directory.exists() {
        fs::remove_dir_all(&partial_directory)?;
    }

    let clip_catalog: ClipCatalog = ClipCatalog {
        plan,
        audio_index: &recordings.audio_index,
    };
    let mut manifest_courses: BTreeMap<String, ManifestCourse> = BTreeMap::new();

    for course in courses {
        manifest_courses.insert(course.id.clone(), write_course_shards(&partial_directory, course, &clip_catalog)?);
    }

    let linked_audio_count: usize = link_audio_files(&partial_directory, &recordings.file_by_audio_sha256)?;

    let built_manifest: Manifest = Manifest {
        manifest_schema_version: manifest::MANIFEST_SCHEMA_VERSION,
        version: version.clone(),
        artifact_created,
        courses: manifest_courses,
        voice_pipeline_revisions: recordings.pipeline_revision_by_engine_code.clone(),
    };

    write_manifest(&partial_directory.join(manifest::MANIFEST_FILENAME), &built_manifest)?;
    fs::rename(&partial_directory, &bundle_directory)?;

    log::info!(
        "wrote bundle; [version={version} courses={} shards={} audio_files={linked_audio_count} path={}]",
        built_manifest.courses.len(),
        built_manifest.file_entries().count(),
        bundle_directory.display(),
    );

    Ok(BuiltBundle {
        directory: bundle_directory,
        manifest: built_manifest,
    })
}

/// Every version a local bundle, the local repository or the embedded bundle already uses.
fn read_taken_versions(paths: &WorkspacePaths) -> Result<BTreeSet<String>, AppError> {
    let mut taken_versions: BTreeSet<String> = list_version_names(&paths.bundles_directory)?.into_iter().collect();

    taken_versions.extend(list_version_names(&paths.local_repository_directory)?);
    taken_versions.extend(read_embedded_version(paths));

    Ok(taken_versions)
}

fn read_embedded_version(paths: &WorkspacePaths) -> Option<String> {
    let embedded_manifest: Result<Manifest, AppError> = read_bundle_manifest(&paths.embedded_bundle_directory);

    embedded_manifest.ok().map(|embedded_manifest| embedded_manifest.version)
}

/// Writes the course's outline shard and one shard per unit under `data/<course id>/`.
fn write_course_shards(bundle_directory: &Path, course: &Course, clip_catalog: &ClipCatalog) -> Result<ManifestCourse, AppError> {
    let course_data_directory: String = format!("{}/{}", manifest::SUBDIR_DATA, course.id);
    let partial_path: PathBuf = bundle_directory.join(&course_data_directory).join(PARTIAL_SHARD_FILE_NAME);

    fs::create_dir_all(bundle_directory.join(&course_data_directory))?;
    course_shard_writer::write_course_shard(&partial_path, course, clip_catalog)?;

    let outline: ManifestEntry = name_shard_by_content(bundle_directory, &partial_path, &course_data_directory, "course")?;
    let mut units: BTreeMap<String, ManifestEntry> = BTreeMap::new();

    for unit in &course.units {
        course_shard_writer::write_unit_shard(&partial_path, unit, clip_catalog)?;
        units.insert(
            unit.id.clone(),
            name_shard_by_content(bundle_directory, &partial_path, &course_data_directory, &format!("unit-{}", unit.id))?,
        );
    }

    Ok(ManifestCourse {
        title: course.title.clone(),
        target_language: course.target_language.clone(),
        source_language: course.source_language.clone(),
        outline,
        units,
        test_course: course.test_course,
    })
}

/* The hash covers the compressed bytes, which are what a client downloads and verifies. */
fn name_shard_by_content(bundle_directory: &Path, partial_path: &Path, data_directory: &str, stem: &str) -> Result<ManifestEntry, AppError> {
    let plain_bytes: Vec<u8> = fs::read(partial_path)?;
    let compressed_bytes: Vec<u8> = compression::compress(&plain_bytes)?;
    let sha256: String = content_address::sha256_hex(&compressed_bytes);
    let file_name: String = content_address::content_addressed_file_name(stem, &sha256, manifest::SHARD_FILE_EXTENSION);
    let relative_path: String = format!("{data_directory}/{file_name}");

    fs::write(bundle_directory.join(&relative_path), &compressed_bytes)?;
    fs::remove_file(partial_path)?;

    Ok(ManifestEntry {
        relative_path,
        size_bytes: compressed_bytes.len() as u64,
        sha256,
    })
}

/* Hard links, because the recordings already exist in the clip store and run to hundreds of megabytes. */
fn link_audio_files(bundle_directory: &Path, file_by_audio_sha256: &BTreeMap<String, PathBuf>) -> Result<usize, AppError> {
    let audio_directory: PathBuf = bundle_directory.join(manifest::SUBDIR_AUDIO);

    fs::create_dir_all(&audio_directory)?;

    for (audio_sha256, source_path) in file_by_audio_sha256 {
        let destination: PathBuf = bundle_directory.join(manifest::audio_key(audio_sha256));

        fs::hard_link(source_path, &destination)?;
    }

    Ok(file_by_audio_sha256.len())
}

pub fn write_manifest(path: &Path, written_manifest: &Manifest) -> Result<(), AppError> {
    let manifest_text: String = serde_json::to_string_pretty(written_manifest)?;

    fs::write(path, format!("{manifest_text}\n"))?;

    Ok(())
}

pub fn read_bundle_manifest(bundle_directory: &Path) -> Result<Manifest, AppError> {
    let manifest_path: PathBuf = bundle_directory.join(manifest::MANIFEST_FILENAME);
    let manifest_bytes: Vec<u8> = fs::read(&manifest_path)
        .map_err(|error| AppError::from(format!("could not read a bundle manifest; [path={} error={error}]", manifest_path.display())))?;

    manifest::parse_manifest(&manifest_bytes)
}

/// The newest finished bundle in the bundles directory.
pub fn find_newest_bundle(paths: &WorkspacePaths) -> Result<PathBuf, AppError> {
    let bundle_names: Vec<String> = list_versions_oldest_first(&paths.bundles_directory)?;
    let newest_name: &String = bundle_names.last().ok_or_else(|| {
        AppError::from(format!(
            "no bundle has been built; run `course_builder build` first; [directory={}]",
            paths.bundles_directory.display(),
        ))
    })?;

    Ok(paths.bundles_directory.join(newest_name))
}

/// The versions under `parent`, oldest first by their manifests' creation time; a version whose manifest cannot
/// be read sorts first.
pub fn list_versions_oldest_first(parent: &Path) -> Result<Vec<String>, AppError> {
    let mut versions_by_creation: Vec<(Option<DateTime<Utc>>, String)> = list_version_names(parent)?
        .into_iter()
        .map(|version_name| (read_artifact_created(&parent.join(&version_name)), version_name))
        .collect();

    versions_by_creation.sort();

    Ok(versions_by_creation.into_iter().map(|(_, version_name)| version_name).collect())
}

fn read_artifact_created(version_directory: &Path) -> Option<DateTime<Utc>> {
    let read_manifest: Result<Manifest, AppError> = read_bundle_manifest(version_directory);

    match read_manifest {
        Ok(version_manifest) => Some(version_manifest.artifact_created),
        Err(error) => {
            log::warn!("a version's manifest is unreadable; [path={} error={error}]", version_directory.display());

            None
        },
    }
}

/* A version is any directory except the pointers, the shared recordings, and a bundle still being written. */
fn list_version_names(parent: &Path) -> Result<Vec<String>, AppError> {
    if !parent.exists() {
        return Ok(Vec::new());
    }

    let mut version_names: Vec<String> = Vec::new();

    for directory_entry in fs::read_dir(parent)? {
        let directory_entry: fs::DirEntry = directory_entry?;
        let name: String = directory_entry.file_name().to_string_lossy().into_owned();
        let is_version: bool = directory_entry.file_type()?.is_dir()
            && !name.starts_with('.')
            && name != manifest::LATEST_POINTER_DIRECTORY
            && name != manifest::SUBDIR_AUDIO;

        if is_version {
            version_names.push(name);
        }
    }

    Ok(version_names)
}

/// Removes all but the newest bundles, and any bundle left partial by an interrupted build.
pub fn prune_bundles(paths: &WorkspacePaths) -> Result<(), AppError> {
    let bundle_names: Vec<String> = list_versions_oldest_first(&paths.bundles_directory)?;
    let removable_count: usize = bundle_names.len().saturating_sub(BUNDLES_KEPT);

    for bundle_name in &bundle_names[..removable_count] {
        fs::remove_dir_all(paths.bundles_directory.join(bundle_name))?;
    }

    for directory_entry in fs::read_dir(&paths.bundles_directory)? {
        let directory_entry: fs::DirEntry = directory_entry?;
        let is_partial: bool = directory_entry.file_name().to_string_lossy().starts_with(PARTIAL_BUNDLE_PREFIX);

        if is_partial {
            fs::remove_dir_all(directory_entry.path())?;
        }
    }

    Ok(())
}

/// Copies the bundle's manifest and shards, without audio or test courses, to where the web app serves its
/// embedded bundle. Leaving a test course out gives the embedded bundle a version label of its own.
pub fn refresh_embedded_bundle(paths: &WorkspacePaths, bundle: &BuiltBundle) -> Result<(), AppError> {
    let embedded_directory: &Path = &paths.embedded_bundle_directory;
    let embedded_manifest: Manifest = embedded_manifest_for(paths, &bundle.manifest)?;

    if embedded_directory.exists() {
        fs::remove_dir_all(embedded_directory)?;
    }

    for entry in embedded_manifest.file_entries() {
        let destination: PathBuf = embedded_directory.join(manifest::versioned_file_key(&embedded_manifest.version, &entry.relative_path));

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(bundle.directory.join(&entry.relative_path), destination)?;
    }

    write_manifest(&embedded_directory.join(manifest::MANIFEST_FILENAME), &embedded_manifest)?;

    log::info!(
        "refreshed the embedded bundle; [version={} bundle_version={} courses={} test_courses_left_out={} path={}]",
        embedded_manifest.version,
        bundle.manifest.version,
        embedded_manifest.courses.len(),
        bundle.manifest.courses.len() - embedded_manifest.courses.len(),
        embedded_directory.display(),
    );

    Ok(())
}

fn embedded_manifest_for(paths: &WorkspacePaths, bundle_manifest: &Manifest) -> Result<Manifest, AppError> {
    let courses: BTreeMap<String, ManifestCourse> = bundle_manifest
        .courses
        .iter()
        .filter(|(_, manifest_course)| !manifest_course.test_course)
        .map(|(course_id, manifest_course)| (course_id.clone(), manifest_course.clone()))
        .collect();

    /* Clients compare versions by label alone, so different course content must not share one. */
    let version: String = if courses.len() == bundle_manifest.courses.len() {
        bundle_manifest.version.clone()
    } else {
        version_label::generate(bundle_manifest.artifact_created, &read_taken_versions(paths)?)?
    };

    Ok(Manifest {
        manifest_schema_version: bundle_manifest.manifest_schema_version,
        version,
        artifact_created: bundle_manifest.artifact_created,
        courses,
        voice_pipeline_revisions: bundle_manifest.voice_pipeline_revisions.clone(),
    })
}

/// Every recording hash the bundle's shards name, after checking each shard against its manifest hash.
pub fn referenced_audio_sha256s(bundle_directory: &Path, bundle_manifest: &Manifest) -> Result<BTreeSet<String>, AppError> {
    let mut clip_audio: Vec<ClipAudioEntry> = Vec::new();

    for manifest_course in bundle_manifest.courses.values() {
        let course_bytes: Vec<u8> = read_verified_shard(bundle_directory, &manifest_course.outline)?;

        clip_audio.extend(course_shard::read_course_shard(&course_bytes)?.clip_audio);

        for entry in manifest_course.units.values() {
            let unit_bytes: Vec<u8> = read_verified_shard(bundle_directory, entry)?;

            clip_audio.extend(course_shard::read_unit_shard(&unit_bytes)?.clip_audio);
        }
    }

    Ok(clip_audio.into_iter().map(|clip_audio_entry| clip_audio_entry.audio_sha256).collect())
}

/// The shard's decompressed bytes, after checking the published bytes against the manifest.
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
    use crate::test_courses;

    const TEST_COURSE_ID: &str = "pt_br.en.testing";
    const UNITS_PER_COURSE: usize = 2;

    /* A teaching course and a test course made from it, each cut to a few units. */
    fn teaching_and_test_courses(plan: &mut AudioPlan) -> Vec<Course> {
        let mut teaching_course: Course = test_courses::read_keyed_courses(plan)
            .into_iter()
            .find(|course| !course.test_course)
            .unwrap();

        teaching_course.units.truncate(UNITS_PER_COURSE);

        let mut test_course: Course = teaching_course.clone();

        test_course.id = TEST_COURSE_ID.to_string();
        test_course.test_course = true;

        vec![teaching_course, test_course]
    }

    fn no_recordings() -> ClipRecordings {
        ClipRecordings {
            audio_index: BTreeMap::new(),
            file_by_audio_sha256: BTreeMap::new(),
            pipeline_revision_by_engine_code: BTreeMap::new(),
        }
    }

    fn read_embedded_manifest(paths: &WorkspacePaths) -> Manifest {
        let embedded_bytes: Vec<u8> = fs::read(paths.embedded_bundle_directory.join(manifest::MANIFEST_FILENAME)).unwrap();

        manifest::parse_manifest(&embedded_bytes).unwrap()
    }

    #[test]
    fn refresh_embedded_bundle_leaves_out_test_courses_the_bundle_holds() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let paths: WorkspacePaths = WorkspacePaths::at_root(workspace_directory.path());
        let mut plan: AudioPlan = AudioPlan::new();
        let courses: Vec<Course> = teaching_and_test_courses(&mut plan);
        let teaching_course_id: &String = &courses[0].id;

        let bundle: BuiltBundle = write_bundle(&paths, &courses, &plan, &no_recordings()).unwrap();
        refresh_embedded_bundle(&paths, &bundle).unwrap();

        let embedded_manifest: Manifest = read_embedded_manifest(&paths);
        let test_course_data_directory: String = format!("{}/{TEST_COURSE_ID}", manifest::SUBDIR_DATA);
        let embedded_test_course_directories: [PathBuf; 2] = [&bundle.manifest.version, &embedded_manifest.version]
            .map(|version| paths.embedded_bundle_directory.join(manifest::versioned_file_key(version, &test_course_data_directory)));

        assert_eq!(bundle.manifest.test_course_ids(), vec![TEST_COURSE_ID]);
        assert_eq!(embedded_manifest.courses.keys().collect::<Vec<&String>>(), vec![teaching_course_id]);
        assert_ne!(embedded_manifest.version, bundle.manifest.version);
        assert!(embedded_test_course_directories.iter().all(|directory| !directory.exists()));

        for entry in embedded_manifest.file_entries() {
            let embedded_path: PathBuf = paths.embedded_bundle_directory.join(manifest::versioned_file_key(&embedded_manifest.version, &entry.relative_path));

            assert!(embedded_path.exists());
        }
    }

    #[test]
    fn refresh_embedded_bundle_keeps_the_bundle_version_when_no_course_is_left_out() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let paths: WorkspacePaths = WorkspacePaths::at_root(workspace_directory.path());
        let mut plan: AudioPlan = AudioPlan::new();
        let teaching_courses: Vec<Course> = teaching_and_test_courses(&mut plan).into_iter().filter(|course| !course.test_course).collect();

        let bundle: BuiltBundle = write_bundle(&paths, &teaching_courses, &plan, &no_recordings()).unwrap();
        refresh_embedded_bundle(&paths, &bundle).unwrap();

        assert_eq!(read_embedded_manifest(&paths), bundle.manifest);
    }
}
