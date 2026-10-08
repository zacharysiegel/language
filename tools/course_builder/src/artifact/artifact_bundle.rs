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

/// Every version a local bundle or the local repository already uses.
fn read_taken_versions(paths: &WorkspacePaths) -> Result<BTreeSet<String>, AppError> {
    let mut taken_versions: BTreeSet<String> = list_version_names(&paths.bundles_directory)?.into_iter().collect();

    taken_versions.extend(list_version_names(&paths.local_repository_directory)?);

    Ok(taken_versions)
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

/// Copies the bundle's manifest and shards, without audio, to where the web app serves its embedded bundle.
pub fn refresh_embedded_bundle(paths: &WorkspacePaths, bundle: &BuiltBundle) -> Result<(), AppError> {
    let embedded_directory: &Path = &paths.embedded_bundle_directory;

    if embedded_directory.exists() {
        fs::remove_dir_all(embedded_directory)?;
    }

    for entry in bundle.manifest.file_entries() {
        let destination: PathBuf = embedded_directory.join(manifest::versioned_file_key(&bundle.manifest.version, &entry.relative_path));

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(bundle.directory.join(&entry.relative_path), destination)?;
    }

    write_manifest(&embedded_directory.join(manifest::MANIFEST_FILENAME), &bundle.manifest)?;

    log::info!("refreshed the embedded bundle; [version={} path={}]", bundle.manifest.version, embedded_directory.display());

    Ok(())
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
