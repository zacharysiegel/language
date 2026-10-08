use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::AppError;
use crate::artifact::ArtifactCache;
use crate::filesystem;

/// An [`ArtifactCache`] over one flat directory of content-addressed file names. A directory removed underneath
/// a running process reads as an empty cache, which is how an operating system reclaiming a cache looks from inside.
pub struct FilesystemArtifactCache {
    root: PathBuf,
}

impl FilesystemArtifactCache {
    pub fn create(root: PathBuf) -> FilesystemArtifactCache {
        FilesystemArtifactCache { root }
    }

    /* Every cached path ends in its content hash, so its last component alone is unique. */
    fn entry_path(&self, relative_path: &str) -> PathBuf {
        let entry_name: &str = relative_path.rsplit('/').next().unwrap_or(relative_path);

        self.root.join(entry_name)
    }
}

impl ArtifactCache for FilesystemArtifactCache {
    async fn put(&self, relative_path: &str, bytes: &[u8]) -> Result<(), AppError> {
        fs::create_dir_all(&self.root)
            .map_err(|error| AppError::from(format!("creating the cache directory failed; [path={} error={error}]", self.root.display())))?;

        filesystem::write_atomically(&self.entry_path(relative_path), bytes)
    }

    async fn get(&self, relative_path: &str) -> Result<Option<Vec<u8>>, AppError> {
        let path: PathBuf = self.entry_path(relative_path);
        let read: Result<Vec<u8>, std::io::Error> = fs::read(&path);

        match read {
            Ok(bytes) => Ok(Some(bytes)),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
            Err(error) => Err(AppError::from(format!("reading a cached file failed; [path={} error={error}]", path.display()))),
        }
    }

    async fn retain(&self, relative_paths_to_keep: &[String]) -> Result<(), AppError> {
        let kept_paths: Vec<PathBuf> = relative_paths_to_keep.iter().map(|relative_path| self.entry_path(relative_path)).collect();
        let read_directory: Result<fs::ReadDir, std::io::Error> = fs::read_dir(&self.root);
        let entries: fs::ReadDir = match read_directory {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(AppError::from(format!("listing the cache failed; [path={} error={error}]", self.root.display()))),
        };

        for entry in entries {
            let path: PathBuf = entry?.path();

            if !kept_paths.contains(&path) {
                fs::remove_file(&path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn put_get_and_retain_keep_only_the_named_entries() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let cache: FilesystemArtifactCache = FilesystemArtifactCache::create(directory.path().join("cache"));

        cache.put("data/a-1.sqlite.br", b"a").await.unwrap();
        cache.put("data/b-2.sqlite.br", b"b").await.unwrap();
        cache.retain(&["data/b-2.sqlite.br".to_string()]).await.unwrap();

        assert_eq!(cache.get("data/a-1.sqlite.br").await.unwrap(), None);
        assert_eq!(cache.get("data/b-2.sqlite.br").await.unwrap().as_deref(), Some(b"b".as_slice()));
    }

    #[tokio::test]
    async fn a_missing_cache_directory_reads_as_empty() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let cache: FilesystemArtifactCache = FilesystemArtifactCache::create(directory.path().join("absent"));

        assert_eq!(cache.get("data/a-1.sqlite.br").await.unwrap(), None);
        cache.retain(&[]).await.unwrap();
    }
}
