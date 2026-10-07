//! Audio and shards go up before `{version}/manifest.json`, and `latest/manifest.json` is the last PUT, so a
//! client that reads the latest pointer finds every file it names already present.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use shared::AppError;
use shared::artifact::{Manifest, content_address, manifest};

use crate::artifact::artifact_bundle;
use crate::artifact::repository::{
    ArtifactRepositoryKind, CACHE_CONTROL_IMMUTABLE, CACHE_CONTROL_POINTER, CONTENT_TYPE_AUDIO, CONTENT_TYPE_MANIFEST,
    CONTENT_TYPE_SHARD, ObjectHeaders,
};

const SHARD_HEADERS: ObjectHeaders = ObjectHeaders {
    content_type: CONTENT_TYPE_SHARD,
    cache_control: CACHE_CONTROL_IMMUTABLE,
};
const AUDIO_HEADERS: ObjectHeaders = ObjectHeaders {
    content_type: CONTENT_TYPE_AUDIO,
    cache_control: CACHE_CONTROL_IMMUTABLE,
};
const VERSIONED_MANIFEST_HEADERS: ObjectHeaders = ObjectHeaders {
    content_type: CONTENT_TYPE_MANIFEST,
    cache_control: CACHE_CONTROL_IMMUTABLE,
};
const POINTER_HEADERS: ObjectHeaders = ObjectHeaders {
    content_type: CONTENT_TYPE_MANIFEST,
    cache_control: CACHE_CONTROL_POINTER,
};

#[derive(Debug, Clone)]
pub struct PublishReport {
    pub version: String,
    pub latest_manifest_url: String,
    pub shards_published: usize,
    pub audio_published: usize,
    pub audio_already_present: usize,
}

pub async fn publish_bundle(bundle_directory: &Path, repository: &ArtifactRepositoryKind) -> Result<PublishReport, AppError> {
    let bundle_manifest: Manifest = artifact_bundle::read_bundle_manifest(bundle_directory)?;
    let referenced_audio_sha256s: BTreeSet<String> = artifact_bundle::referenced_audio_sha256s(bundle_directory, &bundle_manifest)?;
    let version: &str = &bundle_manifest.version;

    log::info!("publishing; [version={version} shards={} recordings={}]", bundle_manifest.units.len() + 1, referenced_audio_sha256s.len());

    let (audio_published, audio_already_present): (usize, usize) =
        publish_audio(bundle_directory, &referenced_audio_sha256s, repository).await?;

    for entry in bundle_manifest.file_entries() {
        let key: String = manifest::versioned_file_key(version, &entry.relative_path);

        repository.put_file(&key, &bundle_directory.join(&entry.relative_path), SHARD_HEADERS).await?;
        log::debug!("uploaded shard; [key={key}]");
    }

    let shards_published: usize = bundle_manifest.file_entries().count();
    let manifest_path: PathBuf = bundle_directory.join(manifest::MANIFEST_FILENAME);

    repository
        .put_file(&manifest::versioned_manifest_key(version), &manifest_path, VERSIONED_MANIFEST_HEADERS)
        .await?;
    repository
        .put_file(&manifest::schema_pointer_key(manifest::MANIFEST_SCHEMA_VERSION), &manifest_path, POINTER_HEADERS)
        .await?;
    repository.put_file(manifest::MANIFEST_LATEST_KEY, &manifest_path, POINTER_HEADERS).await?;

    if let ArtifactRepositoryKind::Local(local_repository) = repository {
        local_repository.retain_newest_versions()?;
    }

    Ok(PublishReport {
        version: version.to_string(),
        latest_manifest_url: repository.url(manifest::MANIFEST_LATEST_KEY),
        shards_published,
        audio_published,
        audio_already_present,
    })
}

/// Uploads the recordings the repository lacks; returns how many were uploaded and how many were already there.
async fn publish_audio(
    bundle_directory: &Path,
    referenced_audio_sha256s: &BTreeSet<String>,
    repository: &ArtifactRepositoryKind,
) -> Result<(usize, usize), AppError> {
    let existing_keys: BTreeSet<String> = repository.existing_keys(manifest::SUBDIR_AUDIO).await?;
    let mut published_count: usize = 0;

    for audio_sha256 in referenced_audio_sha256s {
        let key: String = manifest::audio_key(audio_sha256);

        if existing_keys.contains(&key) {
            continue;
        }

        let source_path: PathBuf = artifact_bundle::audio_file_path(bundle_directory, audio_sha256);

        verify_audio_file(&source_path, audio_sha256)?;
        repository.put_file(&key, &source_path, AUDIO_HEADERS).await?;
        published_count += 1;
    }

    Ok((published_count, referenced_audio_sha256s.len() - published_count))
}

fn verify_audio_file(path: &Path, audio_sha256: &str) -> Result<(), AppError> {
    let bytes: Vec<u8> = fs::read(path)
        .map_err(|error| AppError::from(format!("a recording the shards name is missing; [path={} error={error}]", path.display())))?;

    content_address::verify_sha256(&path.display().to_string(), &bytes, audio_sha256)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use shared::artifact::sha256_hex;
    use shared::course::{Course, Curriculum, Unit};
    use shared::course_shard::{ClipAudioEntry, VoiceEngineKind};

    use super::*;
    use crate::artifact::repository::LocalArtifactRepository;
    use crate::artifact::{BuiltBundle, ClipRecordings, write_bundle};
    use crate::audio_plan::{self, AudioPlan};
    use crate::course_source;
    use crate::workspace::WorkspacePaths;

    const RECORDED_CLIP_COUNT: usize = 3;

    fn read_keyed_course(plan: &mut AudioPlan) -> Course {
        let paths: WorkspacePaths = WorkspacePaths::locate().unwrap();
        let curriculum: Curriculum = course_source::read_curriculum(&paths).unwrap();
        let units: Vec<Unit> = curriculum
            .units
            .iter()
            .filter_map(|curriculum_unit| course_source::read_unit(&paths, &curriculum_unit.id).unwrap())
            .map(|unit| audio_plan::assign_clip_keys(&unit, plan))
            .collect();

        Course {
            title: curriculum.title,
            levels: curriculum.levels,
            units,
        }
    }

    /* Recordings for the first few vocabulary clips only; the rest of the course has none. */
    fn recordings_in(directory: &Path, course: &Course) -> ClipRecordings {
        let mut recordings: ClipRecordings = ClipRecordings {
            audio_index: BTreeMap::new(),
            file_by_audio_sha256: BTreeMap::new(),
            pipeline_revision_by_engine_code: BTreeMap::from([(VoiceEngineKind::Kokoro.code().to_string(), 1)]),
        };
        let clip_keys: Vec<String> = course.units[0].vocabulary.iter().filter_map(|item| item.clip_key.clone()).take(RECORDED_CLIP_COUNT).collect();

        for clip_key in clip_keys {
            let bytes: Vec<u8> = format!("recording of {clip_key}").into_bytes();
            let audio_sha256: String = sha256_hex(&bytes);
            let path: PathBuf = directory.join(format!("{clip_key}.m4a"));

            fs::write(&path, &bytes).unwrap();
            recordings.audio_index.entry(clip_key.clone()).or_default().push(ClipAudioEntry {
                clip_key,
                engine: VoiceEngineKind::Kokoro,
                audio_sha256: audio_sha256.clone(),
                audio_size_bytes: bytes.len() as u64,
            });
            recordings.file_by_audio_sha256.insert(audio_sha256, path);
        }

        recordings
    }

    #[tokio::test]
    async fn publish_bundle_writes_every_file_and_points_latest_at_the_version() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let paths: WorkspacePaths = WorkspacePaths::at_root(workspace_directory.path());
        let clip_directory: PathBuf = workspace_directory.path().join("clips");
        fs::create_dir_all(&clip_directory).unwrap();

        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = read_keyed_course(&mut plan);
        let recordings: ClipRecordings = recordings_in(&clip_directory, &course);
        let bundle: BuiltBundle = write_bundle(&paths, &course, &plan, &recordings).unwrap();
        let repository_root: PathBuf = workspace_directory.path().join("repository");
        let repository: ArtifactRepositoryKind =
            ArtifactRepositoryKind::Local(LocalArtifactRepository::new(repository_root.clone(), "/repository".to_string()));

        let first_report: PublishReport = publish_bundle(&bundle.directory, &repository).await.unwrap();
        let second_report: PublishReport = publish_bundle(&bundle.directory, &repository).await.unwrap();

        let latest_bytes: Vec<u8> = fs::read(repository_root.join(manifest::MANIFEST_LATEST_KEY)).unwrap();
        let latest_manifest: Manifest = manifest::parse_manifest(&latest_bytes).unwrap();

        assert_eq!(latest_manifest.version, bundle.manifest.version);
        assert!(repository_root.join(manifest::schema_pointer_key(manifest::MANIFEST_SCHEMA_VERSION)).exists());

        for entry in latest_manifest.file_entries() {
            assert!(repository_root.join(manifest::versioned_file_key(&latest_manifest.version, &entry.relative_path)).exists());
        }

        assert_eq!(first_report.shards_published, course.units.len() + 1);
        assert_eq!(first_report.audio_published, RECORDED_CLIP_COUNT);
        assert_eq!(second_report.audio_published, 0);
        assert_eq!(second_report.audio_already_present, RECORDED_CLIP_COUNT);
    }
}
