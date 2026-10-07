use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use shared::AppError;
use shared::artifact::{Manifest, manifest};

use crate::artifact::artifact_bundle;
use crate::artifact::repository::artifact_repository::{ArtifactRepository, ObjectHeaders};

pub const ENV_LOCAL_REPOSITORY_ROOT: &str = "PORTUGUESE_COURSE_LOCAL_REPOSITORY_ROOT";

const LOCAL_VERSIONS_KEPT: usize = 2;

pub struct LocalArtifactRepository {
    root: PathBuf,
    public_base_url: String,
}

impl LocalArtifactRepository {
    pub fn new(root: PathBuf, public_base_url: String) -> LocalArtifactRepository {
        LocalArtifactRepository { root, public_base_url }
    }

    /// Removes all but the newest versions, then the recordings none of the kept versions name.
    pub fn retain_newest_versions(&self) -> Result<(), AppError> {
        let version_names: Vec<String> = self.read_version_directory_names()?;
        let removable_count: usize = version_names.len().saturating_sub(LOCAL_VERSIONS_KEPT);

        for version_name in &version_names[..removable_count] {
            fs::remove_dir_all(self.root.join(version_name))?;
            log::info!("removed an old version; [version={version_name}]");
        }

        let kept_audio_sha256s: BTreeSet<String> = self.read_referenced_audio_sha256s(&version_names[removable_count..])?;
        let removed_audio_count: usize = self.remove_audio_except(&kept_audio_sha256s)?;

        if removed_audio_count > 0 {
            log::info!("removed recordings no kept version names; [count={removed_audio_count}]");
        }

        Ok(())
    }

    /* Version names are UTC timestamps, so name order is creation order; `latest/` and `audio/` are not versions. */
    fn read_version_directory_names(&self) -> Result<Vec<String>, AppError> {
        let mut version_names: Vec<String> = Vec::new();

        for directory_entry in fs::read_dir(&self.root)? {
            let directory_entry: fs::DirEntry = directory_entry?;
            let name: String = directory_entry.file_name().to_string_lossy().into_owned();
            let is_version: bool = directory_entry.file_type()?.is_dir()
                && name != manifest::LATEST_POINTER_DIRECTORY
                && name != manifest::SUBDIR_AUDIO;

            if is_version {
                version_names.push(name);
            }
        }

        version_names.sort();

        Ok(version_names)
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
