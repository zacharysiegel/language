use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use shared::AppError;
use shared::artifact::{Manifest, manifest};

use crate::artifact::artifact_bundle;
use crate::artifact::repository::artifact_repository::{ArtifactRepository, ObjectHeaders};

pub const ENV_LOCAL_REPOSITORY_ROOT: &str = "LANGUAGE_LOCAL_REPOSITORY_ROOT";

const LOCAL_VERSIONS_KEPT: usize = 2;

pub struct LocalArtifactRepository {
    root: PathBuf,
    public_base_url: String,
}

impl LocalArtifactRepository {
    pub fn new(root: PathBuf, public_base_url: String) -> LocalArtifactRepository {
        LocalArtifactRepository { root, public_base_url }
    }

    /// Removes all but the newest versions, then the recordings none of the kept versions name; when a kept version
    /// cannot be read, no recording is removed.
    pub fn retain_newest_versions(&self) -> Result<(), AppError> {
        let version_names: Vec<String> = artifact_bundle::list_versions_oldest_first(&self.root)?;
        let removable_count: usize = version_names.len().saturating_sub(LOCAL_VERSIONS_KEPT);

        for version_name in &version_names[..removable_count] {
            fs::remove_dir_all(self.root.join(version_name))?;
            log::info!("removed an old version; [version={version_name}]");
        }

        let kept_audio_read: Result<BTreeSet<String>, AppError> = self.read_referenced_audio_sha256s(&version_names[removable_count..]);
        let kept_audio_sha256s: BTreeSet<String> = match kept_audio_read {
            Ok(kept_audio_sha256s) => kept_audio_sha256s,
            Err(error) => {
                log::warn!("a kept version is unreadable by this build, so every recording is kept; [error={error}]");
                return Ok(());
            },
        };
        let removed_audio_count: usize = self.remove_audio_except(&kept_audio_sha256s)?;

        if removed_audio_count > 0 {
            log::info!("removed recordings no kept version names; [count={removed_audio_count}]");
        }

        Ok(())
    }

    fn read_referenced_audio_sha256s(&self, version_names: &[String]) -> Result<BTreeSet<String>, AppError> {
        let mut audio_sha256s: BTreeSet<String> = BTreeSet::new();

        for version_name in version_names {
            let version_directory: PathBuf = self.root.join(version_name);
            let version_manifest: Manifest = artifact_bundle::read_bundle_manifest(&version_directory)?;

            audio_sha256s.extend(artifact_bundle::referenced_audio_sha256s(&version_directory, &version_manifest)?);
        }

        Ok(audio_sha256s)
    }

    fn remove_audio_except(&self, kept_audio_sha256s: &BTreeSet<String>) -> Result<usize, AppError> {
        let audio_directory: PathBuf = self.root.join(manifest::SUBDIR_AUDIO);

        if !audio_directory.exists() {
            return Ok(0);
        }

        let mut removed_count: usize = 0;

        for directory_entry in fs::read_dir(&audio_directory)? {
            let path: PathBuf = directory_entry?.path();
            let audio_sha256: String = path.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default();

            if !kept_audio_sha256s.contains(&audio_sha256) {
                fs::remove_file(&path)?;
                removed_count += 1;
            }
        }

        Ok(removed_count)
    }
}

impl ArtifactRepository for LocalArtifactRepository {
    async fn put_file(&self, key: &str, source_path: &Path, _headers: ObjectHeaders) -> Result<(), AppError> {
        let destination: PathBuf = self.root.join(key);

        if let Some(parent) = destination.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::copy(source_path, &destination).await?;

        Ok(())
    }

    async fn existing_keys(&self, prefix: &str) -> Result<BTreeSet<String>, AppError> {
        let prefix_directory: PathBuf = self.root.join(prefix.trim_end_matches('/'));
        let mut keys: BTreeSet<String> = BTreeSet::new();

        if !prefix_directory.exists() {
            return Ok(keys);
        }

        let mut read_directory: tokio::fs::ReadDir = tokio::fs::read_dir(&prefix_directory).await?;

        while let Some(directory_entry) = read_directory.next_entry().await? {
            let name: String = directory_entry.file_name().to_string_lossy().into_owned();

            keys.insert(format!("{}/{name}", prefix.trim_end_matches('/')));
        }

        Ok(keys)
    }

    fn url(&self, key: &str) -> String {
        format!("{}/{key}", self.public_base_url.trim_end_matches('/'))
    }
}
