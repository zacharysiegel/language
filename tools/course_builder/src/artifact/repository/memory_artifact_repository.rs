use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use chrono::{DateTime, TimeDelta, Utc};
use shared::AppError;
use shared::artifact::content_address;

use crate::artifact::repository::artifact_repository::{
    self, ArtifactRepository, ListedObject, ObjectHeaders, StoredObject, WriteConditionKind, WriteOutcomeKind,
};

const MEMORY_BASE_URL: &str = "memory://repository";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordedOperationKind {
    Put { key: String, headers: ObjectHeaders },
    Delete { key: String },
}

#[derive(Debug, Clone)]
struct MemoryObject {
    bytes: Vec<u8>,
    uploaded: DateTime<Utc>,
}

#[derive(Debug)]
struct MemoryState {
    objects: BTreeMap<String, MemoryObject>,
    operations: Vec<RecordedOperationKind>,
    now: DateTime<Utc>,
    writes_before_failure: Option<usize>,
}

/// A repository in memory that records every write and can fail a chosen write. Each write advances its clock by
/// one second.
#[derive(Debug)]
pub struct MemoryArtifactRepository {
    state: Mutex<MemoryState>,
}

impl MemoryArtifactRepository {
    pub fn new() -> MemoryArtifactRepository {
        MemoryArtifactRepository {
            state: Mutex::new(MemoryState {
                objects: BTreeMap::new(),
                operations: Vec::new(),
                now: DateTime::parse_from_rfc3339("2026-10-08T14:00:00Z").unwrap().with_timezone(&Utc),
                writes_before_failure: None,
            }),
        }
    }

    /// The write after `write_count` more successful writes fails, and so does every write after it.
    pub fn fail_after_writes(&self, write_count: usize) {
        self.state.lock().unwrap().writes_before_failure = Some(write_count);
    }

    pub fn stop_failing(&self) {
        self.state.lock().unwrap().writes_before_failure = None;
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.state.lock().unwrap().now
    }

    pub fn operations(&self) -> Vec<RecordedOperationKind> {
        self.state.lock().unwrap().operations.clone()
    }

    pub fn put_keys(&self) -> Vec<String> {
        self.operations()
            .into_iter()
            .filter_map(|operation| match operation {
                RecordedOperationKind::Put { key, .. } => Some(key),
                RecordedOperationKind::Delete { .. } => None,
            })
            .collect()
    }

    pub fn clear_operations(&self) {
        self.state.lock().unwrap().operations.clear();
    }

    /// Every object's bytes by key.
    pub fn contents(&self) -> BTreeMap<String, Vec<u8>> {
        self.state.lock().unwrap().objects.iter().map(|(key, object)| (key.clone(), object.bytes.clone())).collect()
    }

    pub fn bytes_at(&self, key: &str) -> Option<Vec<u8>> {
        self.state.lock().unwrap().objects.get(key).map(|object| object.bytes.clone())
    }

    /// Stores `bytes` without recording a write.
    pub fn insert(&self, key: &str, bytes: &[u8]) {
        self.store(key, bytes.to_vec());
    }

    fn record_write(&self, operation: RecordedOperationKind) -> Result<(), AppError> {
        let mut state: MutexGuard<MemoryState> = self.state.lock().unwrap();

        if let Some(writes_before_failure) = state.writes_before_failure {
            if writes_before_failure == 0 {
                return Err(AppError::from(format!("injected write failure; [operation={operation:?}]")));
            }

            state.writes_before_failure = Some(writes_before_failure - 1);
        }

        state.operations.push(operation);

        Ok(())
    }

    fn store(&self, key: &str, bytes: Vec<u8>) {
        let mut state: MutexGuard<MemoryState> = self.state.lock().unwrap();
        let uploaded: DateTime<Utc> = state.now;

        state.objects.insert(key.to_string(), MemoryObject { bytes, uploaded });
        state.now += TimeDelta::seconds(1);
    }
}

fn condition_is_met(condition: &WriteConditionKind, current_bytes: Option<&Vec<u8>>) -> bool {
    match (condition, current_bytes) {
        (WriteConditionKind::Unconditional, _) => true,
        (WriteConditionKind::Absent, current_bytes) => current_bytes.is_none(),
        (WriteConditionKind::MatchesEntityTag(_), None) => false,
        (WriteConditionKind::MatchesEntityTag(expected_entity_tag), Some(current_bytes)) => *expected_entity_tag == content_address::sha256_hex(current_bytes),
    }
}

impl ArtifactRepository for MemoryArtifactRepository {
    async fn put_file(&self, key: &str, source_path: &Path, headers: ObjectHeaders) -> Result<(), AppError> {
        let bytes: Vec<u8> = fs::read(source_path)?;

        self.record_write(RecordedOperationKind::Put { key: key.to_string(), headers })?;
        self.store(key, bytes);

        Ok(())
    }

    async fn put_bytes(&self, key: &str, bytes: &[u8], headers: ObjectHeaders, condition: WriteConditionKind) -> Result<WriteOutcomeKind, AppError> {
        let current_bytes: Option<Vec<u8>> = self.bytes_at(key);

        if !condition_is_met(&condition, current_bytes.as_ref()) {
            return Ok(WriteOutcomeKind::PreconditionFailed);
        }

        self.record_write(RecordedOperationKind::Put { key: key.to_string(), headers })?;
        self.store(key, bytes.to_vec());

        Ok(WriteOutcomeKind::Written)
    }

    async fn get_object(&self, key: &str) -> Result<Option<StoredObject>, AppError> {
        let stored_object: Option<StoredObject> = self.bytes_at(key).map(|bytes| StoredObject {
            entity_tag: content_address::sha256_hex(&bytes),
            bytes,
        });

        Ok(stored_object)
    }

    async fn list_objects(&self, directory_prefix: &str) -> Result<Vec<ListedObject>, AppError> {
        artifact_repository::require_directory_prefix(directory_prefix)?;

        let state: MutexGuard<MemoryState> = self.state.lock().unwrap();
        let listed_objects: Vec<ListedObject> = state
            .objects
            .iter()
            .filter(|(key, _)| key.starts_with(directory_prefix))
            .map(|(key, object)| ListedObject {
                key: key.clone(),
                uploaded: object.uploaded,
            })
            .collect();

        Ok(listed_objects)
    }

    async fn delete_keys(&self, keys: &[String]) -> Result<(), AppError> {
        for key in keys {
            self.record_write(RecordedOperationKind::Delete { key: key.clone() })?;
            self.state.lock().unwrap().objects.remove(key);
        }

        Ok(())
    }

    fn url(&self, key: &str) -> String {
        format!("{MEMORY_BASE_URL}/{key}")
    }
}
