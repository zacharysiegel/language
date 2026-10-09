use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use shared::AppError;
use shared::artifact::{Manifest, content_address, manifest};
use shared::filesystem;

use crate::artifact::artifact_bundle;
use crate::artifact::repository::artifact_repository::{
    self, ArtifactRepository, ListedObject, ObjectHeaders, StoredObject, WriteConditionKind, WriteOutcomeKind,
};

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

    pub fn root(&self) -> &Path {
        &self.root
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

    fn path_for_key(&self, key: &str) -> Result<PathBuf, AppError> {
        let leaves_root: bool = key.is_empty() || key.starts_with('/') || key.split('/').any(|component| component == "..");

        if leaves_root {
            return Err(AppError::from(format!("a repository key must name a path inside the repository; [key={key}]")));
        }

        Ok(self.root.join(key))
    }

    /// Stops at the first directory that is not empty, and never removes the root.
    fn remove_empty_directories_up_to_root(&self, deepest_directory: &Path) -> Result<(), AppError> {
        let mut directory: &Path = deepest_directory;

        while directory != self.root && directory.starts_with(&self.root) {
            let emptiness: DirectoryEmptinessKind = directory_emptiness(directory)?;

            match emptiness {
                DirectoryEmptinessKind::Empty => fs::remove_dir(directory)
                    .map_err(|error| AppError::from(format!("removing an empty directory failed; [path={} error={error}]", directory.display())))?,
                DirectoryEmptinessKind::Missing => {},
                DirectoryEmptinessKind::NotEmpty => return Ok(()),
            }

            let Some(parent) = directory.parent() else {
                return Ok(());
            };

            directory = parent;
        }

        Ok(())
    }
}

enum DirectoryEmptinessKind {
    Empty,
    NotEmpty,
    Missing,
}

fn directory_emptiness(directory: &Path) -> Result<DirectoryEmptinessKind, AppError> {
    let read_directory: Result<fs::ReadDir, io::Error> = fs::read_dir(directory);
    let mut entries: fs::ReadDir = match read_directory {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(DirectoryEmptinessKind::Missing),
        Err(error) => return Err(AppError::from(format!("reading a directory failed; [path={} error={error}]", directory.display()))),
    };
    let first_entry: Option<Result<fs::DirEntry, io::Error>> = entries.next();

    match first_entry {
        None => Ok(DirectoryEmptinessKind::Empty),
        Some(_) => Ok(DirectoryEmptinessKind::NotEmpty),
    }
}

fn condition_is_met(condition: &WriteConditionKind, current_object: Option<&StoredObject>) -> bool {
    match (condition, current_object) {
        (WriteConditionKind::Unconditional, _) => true,
        (WriteConditionKind::Absent, current_object) => current_object.is_none(),
        (WriteConditionKind::MatchesEntityTag(_), None) => false,
        (WriteConditionKind::MatchesEntityTag(expected_entity_tag), Some(current_object)) => *expected_entity_tag == current_object.entity_tag,
    }
}

fn mark_uploaded_now(path: &Path) -> Result<(), AppError> {
    let file: fs::File = fs::OpenOptions::new().write(true).open(path)?;

    file.set_modified(SystemTime::now())
        .map_err(|error| AppError::from(format!("setting an object's upload time failed; [path={} error={error}]", path.display())))
}

fn collect_files_recursively(directory: &Path, key_prefix: &str, listed_objects: &mut Vec<ListedObject>) -> Result<(), AppError> {
    for directory_entry in fs::read_dir(directory)? {
        let directory_entry: fs::DirEntry = directory_entry?;
        let name: String = directory_entry.file_name().to_string_lossy().into_owned();
        let file_type: fs::FileType = directory_entry.file_type()?;

        if file_type.is_dir() {
            collect_files_recursively(&directory_entry.path(), &format!("{key_prefix}{name}/"), listed_objects)?;
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let modified: SystemTime = directory_entry.metadata()?.modified()?;

        listed_objects.push(ListedObject {
            key: format!("{key_prefix}{name}"),
            uploaded: DateTime::<Utc>::from(modified),
        });
    }

    Ok(())
}

impl ArtifactRepository for LocalArtifactRepository {
    async fn put_file(&self, key: &str, source_path: &Path, _headers: ObjectHeaders) -> Result<(), AppError> {
        let destination: PathBuf = self.path_for_key(key)?;

        if let Some(parent) = destination.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // A copy on macOS keeps the source file's modification time.
        tokio::fs::copy(source_path, &destination).await?;
        mark_uploaded_now(&destination)?;

        Ok(())
    }

    /// The comparison and the write are atomic only while the caller holds the repository lock.
    async fn put_bytes(&self, key: &str, bytes: &[u8], _headers: ObjectHeaders, condition: WriteConditionKind) -> Result<WriteOutcomeKind, AppError> {
        let destination: PathBuf = self.path_for_key(key)?;
        let current_object: Option<StoredObject> = self.get_object(key).await?;

        if !condition_is_met(&condition, current_object.as_ref()) {
            return Ok(WriteOutcomeKind::PreconditionFailed);
        }

        if let Some(parent) = destination.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        filesystem::write_atomically(&destination, bytes)?;

        Ok(WriteOutcomeKind::Written)
    }

    /// The entity tag is the SHA-256 of the bytes.
    async fn get_object(&self, key: &str) -> Result<Option<StoredObject>, AppError> {
        let path: PathBuf = self.path_for_key(key)?;
        let read: Result<Vec<u8>, io::Error> = tokio::fs::read(&path).await;

        match read {
            Ok(bytes) => {
                let entity_tag: String = content_address::sha256_hex(&bytes);

                Ok(Some(StoredObject { bytes, entity_tag }))
            },
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(AppError::from(format!("reading a repository object failed; [path={} error={error}]", path.display()))),
        }
    }

    /// The upload time is the file's modification time.
    async fn list_objects(&self, directory_prefix: &str) -> Result<Vec<ListedObject>, AppError> {
        artifact_repository::require_directory_prefix(directory_prefix)?;

        let directory: PathBuf = self.path_for_key(directory_prefix)?;
        let mut listed_objects: Vec<ListedObject> = Vec::new();

        if !directory.is_dir() {
            return Ok(listed_objects);
        }

        collect_files_recursively(&directory, directory_prefix, &mut listed_objects)?;
        listed_objects.sort_by(|left, right| left.key.cmp(&right.key));

        Ok(listed_objects)
    }

    /// Also removes the directories the deletions leave empty, up to but not including the root.
    async fn delete_keys(&self, keys: &[String]) -> Result<(), AppError> {
        for key in keys {
            let path: PathBuf = self.path_for_key(key)?;
            let removed: Result<(), io::Error> = tokio::fs::remove_file(&path).await;

            match removed {
                Ok(()) => {},
                Err(error) if error.kind() == io::ErrorKind::NotFound => {},
                Err(error) => return Err(AppError::from(format!("deleting a repository object failed; [path={} error={error}]", path.display()))),
            }

            if let Some(parent) = path.parent() {
                self.remove_empty_directories_up_to_root(parent)?;
            }
        }

        Ok(())
    }

    fn url(&self, key: &str) -> String {
        format!("{}/{key}", self.public_base_url.trim_end_matches('/'))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    const PUBLICATION_HEADERS: ObjectHeaders = ObjectHeaders {
        content_type: artifact_repository::CONTENT_TYPE_MANIFEST,
        cache_control: artifact_repository::CACHE_CONTROL_IMMUTABLE,
    };

    fn repository_in(directory: &tempfile::TempDir) -> LocalArtifactRepository {
        LocalArtifactRepository::new(directory.path().join("repository"), "/repository".to_string())
    }

    async fn put(repository: &LocalArtifactRepository, key: &str) {
        let outcome: WriteOutcomeKind = repository.put_bytes(key, key.as_bytes(), PUBLICATION_HEADERS, WriteConditionKind::Unconditional).await.unwrap();

        assert_eq!(outcome, WriteOutcomeKind::Written);
    }

    #[tokio::test]
    async fn delete_keys_leaves_no_empty_directory_and_keeps_the_root() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);
        let publication_keys: Vec<String> = vec![
            "courses/pt_br.en.foundations/2026-10-08+duluth/manifest.json".to_string(),
            "courses/pt_br.en.foundations/2026-10-08+duluth/data/course-1.sqlite.br".to_string(),
        ];

        for key in &publication_keys {
            put(&repository, key).await;
        }

        put(&repository, "courses/pt_br.en.foundations/latest/manifest.json").await;

        repository.delete_keys(&publication_keys).await.unwrap();

        assert!(!repository.root().join("courses/pt_br.en.foundations/2026-10-08+duluth").exists());
        assert!(repository.root().join("courses/pt_br.en.foundations/latest/manifest.json").exists());

        repository.delete_keys(&["courses/pt_br.en.foundations/latest/manifest.json".to_string()]).await.unwrap();

        assert!(!repository.root().join("courses").exists());
        assert!(repository.root().is_dir());
    }

    #[tokio::test]
    async fn delete_keys_ignores_a_key_already_absent() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);

        repository.delete_keys(&["courses/pt_br.en.foundations/latest/manifest.json".to_string()]).await.unwrap();
    }

    #[tokio::test]
    async fn list_objects_lists_recursively_and_never_matches_a_longer_sibling_name() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);

        put(&repository, "courses/pt_br.en.foundations/2026-10-08+charlotte/manifest.json").await;
        put(&repository, "courses/pt_br.en.foundations/2026-10-08+charlotte/data/course-1.sqlite.br").await;
        put(&repository, "courses/pt_br.en.foundations/2026-10-08+charlottesville/manifest.json").await;

        let listed_keys: BTreeSet<String> =
            artifact_repository::list_keys(&repository, "courses/pt_br.en.foundations/2026-10-08+charlotte/").await.unwrap();

        assert_eq!(
            listed_keys,
            BTreeSet::from([
                "courses/pt_br.en.foundations/2026-10-08+charlotte/data/course-1.sqlite.br".to_string(),
                "courses/pt_br.en.foundations/2026-10-08+charlotte/manifest.json".to_string(),
            ]),
        );
    }

    #[tokio::test]
    async fn list_objects_refuses_a_prefix_without_a_trailing_separator() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);

        let listed: Result<Vec<ListedObject>, AppError> = repository.list_objects("courses/pt_br.en.foundations/2026-10-08+charlotte").await;

        assert!(listed.is_err());
    }

    #[tokio::test]
    async fn list_objects_reports_the_time_put_file_wrote_not_the_source_file_time() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);
        let source_path: PathBuf = workspace_directory.path().join("manifest.json");
        let source_built: SystemTime = SystemTime::UNIX_EPOCH + Duration::from_secs(86_400);
        let key: &str = "courses/pt_br.en.foundations/2026-10-08+duluth/manifest.json";

        fs::write(&source_path, b"{}").unwrap();
        fs::File::options().write(true).open(&source_path).unwrap().set_modified(source_built).unwrap();

        let before_upload: DateTime<Utc> = DateTime::<Utc>::from(SystemTime::now()) - chrono::Duration::seconds(1);

        repository.put_file(key, &source_path, PUBLICATION_HEADERS).await.unwrap();

        let listed_objects: Vec<ListedObject> = repository.list_objects("courses/").await.unwrap();

        assert_eq!(listed_objects.len(), 1);
        assert_eq!(listed_objects[0].key, key);
        assert!(listed_objects[0].uploaded >= before_upload);
    }

    #[tokio::test]
    async fn put_bytes_writes_only_when_the_condition_holds() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);
        let key: &str = "course-index.json";

        assert_eq!(repository.get_object(key).await.unwrap(), None);

        let first_outcome: WriteOutcomeKind = repository.put_bytes(key, b"first", PUBLICATION_HEADERS, WriteConditionKind::Absent).await.unwrap();
        let first_object: StoredObject = repository.get_object(key).await.unwrap().unwrap();
        let repeated_absent_outcome: WriteOutcomeKind =
            repository.put_bytes(key, b"second", PUBLICATION_HEADERS, WriteConditionKind::Absent).await.unwrap();
        let stale_outcome: WriteOutcomeKind = repository
            .put_bytes(key, b"second", PUBLICATION_HEADERS, WriteConditionKind::MatchesEntityTag(content_address::sha256_hex(b"other")))
            .await
            .unwrap();
        let matching_outcome: WriteOutcomeKind = repository
            .put_bytes(key, b"second", PUBLICATION_HEADERS, WriteConditionKind::MatchesEntityTag(first_object.entity_tag.clone()))
            .await
            .unwrap();
        let second_object: StoredObject = repository.get_object(key).await.unwrap().unwrap();

        assert_eq!(first_outcome, WriteOutcomeKind::Written);
        assert_eq!(first_object.bytes, b"first");
        assert_eq!(repeated_absent_outcome, WriteOutcomeKind::PreconditionFailed);
        assert_eq!(stale_outcome, WriteOutcomeKind::PreconditionFailed);
        assert_eq!(matching_outcome, WriteOutcomeKind::Written);
        assert_eq!(second_object.bytes, b"second");
        assert_ne!(second_object.entity_tag, first_object.entity_tag);
    }

    #[tokio::test]
    async fn path_for_key_refuses_a_key_that_leaves_the_root() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);

        let read: Result<Option<StoredObject>, AppError> = repository.get_object("courses/../../outside.json").await;

        assert!(read.is_err());
    }
}
