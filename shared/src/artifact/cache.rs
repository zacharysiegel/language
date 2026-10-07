use crate::AppError;

/// A local store of repository files by their relative path. Content-addressed names make every entry
/// immutable, so no entry is ever stale.
// The returned futures carry no `Send` bound; an implementation may hold `!Send` handles.
#[allow(async_fn_in_trait)]
pub trait ArtifactCache {
    async fn put(&self, relative_path: &str, bytes: &[u8]) -> Result<(), AppError>;

    async fn get(&self, relative_path: &str) -> Result<Option<Vec<u8>>, AppError>;

    /// Removes every entry whose path is not in `relative_paths_to_keep`.
    async fn retain(&self, relative_paths_to_keep: &[String]) -> Result<(), AppError>;
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::BTreeMap;

    use super::*;

    pub(crate) struct MockArtifactCache {
        entries: tokio::sync::Mutex<BTreeMap<String, Vec<u8>>>,
    }

    impl MockArtifactCache {
        pub(crate) fn new() -> MockArtifactCache {
            MockArtifactCache {
                entries: tokio::sync::Mutex::new(BTreeMap::new()),
            }
        }

        pub(crate) async fn paths(&self) -> Vec<String> {
            self.entries.lock().await.keys().cloned().collect()
        }
    }

    impl ArtifactCache for MockArtifactCache {
        async fn put(&self, relative_path: &str, bytes: &[u8]) -> Result<(), AppError> {
            self.entries.lock().await.insert(relative_path.to_string(), bytes.to_vec());

            Ok(())
        }

        async fn get(&self, relative_path: &str) -> Result<Option<Vec<u8>>, AppError> {
            Ok(self.entries.lock().await.get(relative_path).cloned())
        }

        async fn retain(&self, relative_paths_to_keep: &[String]) -> Result<(), AppError> {
            self.entries.lock().await.retain(|path, _| relative_paths_to_keep.contains(path));

            Ok(())
        }
    }

    #[tokio::test]
    async fn mock_cache_round_trips_and_retains() {
        let cache: MockArtifactCache = MockArtifactCache::new();
        cache.put("a", b"1").await.unwrap();
        cache.put("b", b"2").await.unwrap();

        cache.retain(&["b".to_string()]).await.unwrap();

        assert_eq!(cache.get("a").await.unwrap(), None);
        assert_eq!(cache.get("b").await.unwrap().as_deref(), Some(b"2".as_slice()));
        assert_eq!(cache.paths().await, vec!["b".to_string()]);
    }
}
