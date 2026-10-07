use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use shared::AppError;
use shared::artifact::{self, Manifest, ManifestEntry, compression, content_address, manifest};
use shared::course::{Course, Unit};
use shared::course_shard::{self, ClipAudioEntry};

use crate::artifact::course_shard_writer::{self, ClipAudioIndex, ClipCatalog};
use crate::audio_plan::AudioPlan;
use crate::workspace::WorkspacePaths;

const VERSION_TIMESTAMP_FORMAT: &str = "%Y%m%dT%H%M%SZ";
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
    course: &Course,
    plan: &AudioPlan,
    recordings: &ClipRecordings,
) -> Result<BuiltBundle, AppError> {
    let artifact_created: DateTime<Utc> = Utc::now();
    let version: String = artifact_created.format(VERSION_TIMESTAMP_FORMAT).to_string();
    let partial_directory: PathBuf = paths.bundles_directory.join(format!("{PARTIAL_BUNDLE_PREFIX}{version}"));
    let bundle_directory: PathBuf = paths.bundles_directory.join(&version);

    if partial_directory.exists() {
        fs::remove_dir_all(&partial_directory)?;
    }

    fs::create_dir_all(partial_directory.join(manifest::SUBDIR_DATA))?;

    let clip_catalog: ClipCatalog = ClipCatalog {
        plan,
        audio_index: &recordings.audio_index,
    };
    let course_entry: ManifestEntry = write_course_shard(&partial_directory, course, &clip_catalog)?;
    let mut unit_entries: BTreeMap<String, ManifestEntry> = BTreeMap::new();

    for unit in &course.units {
        unit_entries.insert(unit.id.clone(), write_unit_shard(&partial_directory, unit, &clip_catalog)?);
    }

    let linked_audio_count: usize = link_audio_files(&partial_directory, &recordings.file_by_audio_sha256)?;

    let built_manifest: Manifest = Manifest {
        manifest_schema_version: manifest::MANIFEST_SCHEMA_VERSION,
        version: version.clone(),
        artifact_created,
        course: course_entry,
        units: unit_entries,
        voice_pipeline_revisions: recordings.pipeline_revision_by_engine_code.clone(),
    };

    write_manifest(&partial_directory.join(manifest::MANIFEST_FILENAME), &built_manifest)?;
    fs::rename(&partial_directory, &bundle_directory)?;

    log::info!(
        "wrote bundle; [version={version} units={} audio_files={linked_audio_count} path={}]",
        built_manifest.units.len(),
        bundle_directory.display(),
    );

    Ok(BuiltBundle {
        directory: bundle_directory,
        manifest: built_manifest,
    })
}

fn write_course_shard(bundle_directory: &Path, course: &Course, clip_catalog: &ClipCatalog) -> Result<ManifestEntry, AppError> {
    let partial_path: PathBuf = bundle_directory.join(manifest::SUBDIR_DATA).join(PARTIAL_SHARD_FILE_NAME);

    course_shard_writer::write_course_shard(&partial_path, course, clip_catalog)?;

    name_shard_by_content(bundle_directory, &partial_path, "course")
}

fn write_unit_shard(bundle_directory: &Path, unit: &Unit, clip_catalog: &ClipCatalog) -> Result<ManifestEntry, AppError> {
    let partial_path: PathBuf = bundle_directory.join(manifest::SUBDIR_DATA).join(PARTIAL_SHARD_FILE_NAME);

    course_shard_writer::write_unit_shard(&partial_path, unit, clip_catalog)?;

    name_shard_by_content(bundle_directory, &partial_path, &format!("unit-{}", unit.id))
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
    let bundle_names: Vec<String> = list_bundle_names(&paths.bundles_directory)?;
    let newest_name: &String = bundle_names.last().ok_or_else(|| {
        AppError::from(format!(
            "no bundle has been built; run `course_builder build` first; [directory={}]",
            paths.bundles_directory.display(),
        ))
    })?;

    Ok(paths.bundles_directory.join(newest_name))
}

/* Version names are UTC timestamps, so name order is creation order. */
fn list_bundle_names(bundles_directory: &Path) -> Result<Vec<String>, AppError> {
    if !bundles_directory.exists() {
        return Ok(Vec::new());
    }

    let mut bundle_names: Vec<String> = Vec::new();

    for directory_entry in fs::read_dir(bundles_directory)? {
        let directory_entry: fs::DirEntry = directory_entry?;
        let name: String = directory_entry.file_name().to_string_lossy().into_owned();
        let is_finished_bundle: bool = directory_entry.file_type()?.is_dir() && !name.starts_with('.');

        if is_finished_bundle {
            bundle_names.push(name);
        }
    }

    bundle_names.sort();

    Ok(bundle_names)
}

/// Removes all but the newest bundles, and any bundle left partial by an interrupted build.
pub fn prune_bundles(paths: &WorkspacePaths) -> Result<(), AppError> {
    let bundle_names: Vec<String> = list_bundle_names(&paths.bundles_directory)?;
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

    fs::create_dir_all(embedded_directory.join(&bundle.manifest.version).join(manifest::SUBDIR_DATA))?;

    for entry in bundle.manifest.file_entries() {
        let destination: PathBuf = embedded_directory.join(manifest::versioned_file_key(&bundle.manifest.version, &entry.relative_path));

        fs::copy(bundle.directory.join(&entry.relative_path), destination)?;
    }

    write_manifest(&embedded_directory.join(manifest::MANIFEST_FILENAME), &bundle.manifest)?;

    log::info!("refreshed the embedded bundle; [version={} path={}]", bundle.manifest.version, embedded_directory.display());

    Ok(())
}

/// Every recording hash the bundle's shards name, after checking each shard against its manifest hash.
pub fn referenced_audio_sha256s(bundle_directory: &Path, bundle_manifest: &Manifest) -> Result<BTreeSet<String>, AppError> {
    let course_bytes: Vec<u8> = read_verified_shard(bundle_directory, &bundle_manifest.course)?;
    let mut clip_audio: Vec<ClipAudioEntry> = course_shard::read_course_shard(&course_bytes)?.clip_audio;

    for entry in bundle_manifest.units.values() {
        let unit_bytes: Vec<u8> = read_verified_shard(bundle_directory, entry)?;

        clip_audio.extend(course_shard::read_unit_shard(&unit_bytes)?.clip_audio);
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
