use std::fs::{self, File, OpenOptions, TryLockError};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::AppError;
use shared::artifact::repository_key;

use crate::artifact::repository::artifact_repository::{
    self, ArtifactRepository, ArtifactRepositoryKind, ObjectHeaders, StoredObject, WriteConditionKind, WriteOutcomeKind,
};

pub const LOCAL_REPOSITORY_LOCK_FILENAME: &str = ".repository.lock";

const CACHE_CONTROL_NO_STORE: &str = "no-store";

const LEASE_HEADERS: ObjectHeaders = ObjectHeaders {
    content_type: artifact_repository::CONTENT_TYPE_MANIFEST,
    cache_control: CACHE_CONTROL_NO_STORE,
};
const UNKNOWN_HOST: &str = "unknown";

/// An exclusive lock on a local repository; the operating system also releases it when the process exits.
pub struct LocalRepositoryLock {
    file: File,
    path: PathBuf,
}

impl LocalRepositoryLock {
    pub fn release(self) -> Result<(), AppError> {
        self.file
            .unlock()
            .map_err(|error| AppError::from(format!("releasing the repository lock failed; [path={} error={error}]", self.path.display())))
    }
}

/// Waits while another process holds the lock.
pub async fn lock_local_repository(root: &Path) -> Result<LocalRepositoryLock, AppError> {
    fs::create_dir_all(root)?;

    let path: PathBuf = root.join(LOCAL_REPOSITORY_LOCK_FILENAME);
    let file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
        .map_err(|error| AppError::from(format!("opening the repository lock failed; [path={} error={error}]", path.display())))?;
    let attempt: Result<(), TryLockError> = file.try_lock();

    match attempt {
        Ok(()) => return Ok(LocalRepositoryLock { file, path }),
        Err(TryLockError::WouldBlock) => log::info!("waiting for another command to release the repository lock; [path={}]", path.display()),
        Err(TryLockError::Error(error)) => {
            return Err(AppError::from(format!("taking the repository lock failed; [path={} error={error}]", path.display())));
        },
    }

    let locked: Result<Result<File, io::Error>, tokio::task::JoinError> = tokio::task::spawn_blocking(move || file.lock().map(|()| file)).await;

    match locked {
        Ok(Ok(file)) => Ok(LocalRepositoryLock { file, path }),
        Ok(Err(error)) => Err(AppError::from(format!("taking the repository lock failed; [path={} error={error}]", path.display()))),
        Err(error) => Err(AppError::from(format!("waiting for the repository lock failed; [path={} error={error}]", path.display()))),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryLeaseHolder {
    pub host: String,
    pub process_id: u32,
    pub acquired: DateTime<Utc>,
}

impl RepositoryLeaseHolder {
    pub fn for_this_process() -> RepositoryLeaseHolder {
        RepositoryLeaseHolder {
            host: read_host_name(),
            process_id: std::process::id(),
            acquired: Utc::now(),
        }
    }

    fn describe(&self) -> String {
        format!("host={} process_id={} acquired={}", self.host, self.process_id, self.acquired.to_rfc3339())
    }
}

fn read_host_name() -> String {
    let output: Result<Output, io::Error> = Command::new("hostname").output();

    match output {
        Ok(output) if output.status.success() => {
            let host_name: String = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if host_name.is_empty() { UNKNOWN_HOST.to_string() } else { host_name }
        },
        Ok(_) | Err(_) => UNKNOWN_HOST.to_string(),
    }
}

/// The lease object a writing command holds on a remote repository.
pub struct RepositoryLease {
    lease_bytes: Vec<u8>,
}

impl RepositoryLease {
    /// Fails, leaving the object, when the lease object no longer holds this lease.
    pub async fn release(self, repository: &impl ArtifactRepository) -> Result<(), AppError> {
        let current_lease: Option<StoredObject> = repository.get_object(repository_key::REPOSITORY_LOCK_KEY).await?;

        match current_lease {
            Some(current_lease) if current_lease.bytes == self.lease_bytes => {
                repository.delete_keys(&[repository_key::REPOSITORY_LOCK_KEY.to_string()]).await
            },
            Some(current_lease) => Err(AppError::from(format!(
                "the repository lease was replaced while this command held it; [key={} {}]",
                repository_key::REPOSITORY_LOCK_KEY,
                describe_lease_bytes(&current_lease.bytes),
            ))),
            None => Err(AppError::from(format!(
                "the repository lease was removed while this command held it; [key={}]",
                repository_key::REPOSITORY_LOCK_KEY,
            ))),
        }
    }
}

/// Fails, naming the holder, when another command holds the lease.
pub async fn acquire_repository_lease(repository: &impl ArtifactRepository, holder: &RepositoryLeaseHolder) -> Result<RepositoryLease, AppError> {
    let lease_bytes: Vec<u8> = serde_json::to_vec_pretty(holder)?;
    let outcome: WriteOutcomeKind = repository
        .put_bytes(repository_key::REPOSITORY_LOCK_KEY, &lease_bytes, LEASE_HEADERS, WriteConditionKind::Absent)
        .await?;

    match outcome {
        WriteOutcomeKind::Written => Ok(RepositoryLease { lease_bytes }),
        WriteOutcomeKind::PreconditionFailed => {
            let current_lease_description: String = describe_current_lease(repository).await?;

            Err(AppError::from(format!(
                "another command holds the repository lease; once it is no longer running, `unlock cloudflare-r2` removes the lease; [key={} {current_lease_description}]",
                repository_key::REPOSITORY_LOCK_KEY,
            )))
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeaseRemovalKind {
    Removed { holder_description: String },
    NotHeld,
}

pub async fn remove_repository_lease(repository: &impl ArtifactRepository) -> Result<LeaseRemovalKind, AppError> {
    let current_lease: Option<StoredObject> = repository.get_object(repository_key::REPOSITORY_LOCK_KEY).await?;

    let Some(current_lease) = current_lease
    else {
        return Ok(LeaseRemovalKind::NotHeld);
    };

    repository.delete_keys(&[repository_key::REPOSITORY_LOCK_KEY.to_string()]).await?;

    Ok(LeaseRemovalKind::Removed {
        holder_description: describe_lease_bytes(&current_lease.bytes),
    })
}

async fn describe_current_lease(repository: &impl ArtifactRepository) -> Result<String, AppError> {
    let current_lease: Option<StoredObject> = repository.get_object(repository_key::REPOSITORY_LOCK_KEY).await?;

    match current_lease {
        Some(current_lease) => Ok(describe_lease_bytes(&current_lease.bytes)),
        None => Ok("lease=released after this command's write was refused".to_string()),
    }
}

fn describe_lease_bytes(lease_bytes: &[u8]) -> String {
    let holder: Result<RepositoryLeaseHolder, serde_json::Error> = serde_json::from_slice(lease_bytes);

    match holder {
        Ok(holder) => holder.describe(),
        Err(_) => format!("lease={}", String::from_utf8_lossy(lease_bytes)),
    }
}

/// The lock a writing command holds on a repository for its whole run.
pub enum RepositoryLockKind {
    LocalFile(LocalRepositoryLock),
    Lease(RepositoryLease),
    NotTakenForDryRun,
}

impl RepositoryLockKind {
    pub async fn release(self, repository: &ArtifactRepositoryKind) -> Result<(), AppError> {
        match self {
            RepositoryLockKind::LocalFile(local_lock) => local_lock.release(),
            RepositoryLockKind::Lease(lease) => lease.release(repository).await,
            RepositoryLockKind::NotTakenForDryRun => Ok(()),
        }
    }
}

pub async fn acquire_repository_lock(repository: &ArtifactRepositoryKind) -> Result<RepositoryLockKind, AppError> {
    match repository {
        ArtifactRepositoryKind::Local(local_repository) => Ok(RepositoryLockKind::LocalFile(lock_local_repository(local_repository.root()).await?)),
        ArtifactRepositoryKind::CloudflareR2(r2_repository) => {
            let holder: RepositoryLeaseHolder = RepositoryLeaseHolder::for_this_process();

            Ok(RepositoryLockKind::Lease(acquire_repository_lease(r2_repository, &holder).await?))
        },
        ArtifactRepositoryKind::Dry(_) => Ok(RepositoryLockKind::NotTakenForDryRun),
    }
}

/// The work's error wins over the release's, which is then only logged.
pub fn combine_with_release<T>(work_result: Result<T, AppError>, release_result: Result<(), AppError>) -> Result<T, AppError> {
    match (work_result, release_result) {
        (Ok(value), Ok(())) => Ok(value),
        (Ok(_), Err(release_error)) => Err(release_error),
        (Err(work_error), Ok(())) => Err(work_error),
        (Err(work_error), Err(release_error)) => {
            log::error!("releasing the repository lock failed; [error={release_error}]");

            Err(work_error)
        },
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, Instant};

    use super::*;
    use crate::artifact::repository::LocalArtifactRepository;
    use crate::artifact::repository::recording_http_client::{CannedResponse, RecordedRequest, RecordingHttpClient};

    const CONTENDED_WAIT: Duration = Duration::from_millis(300);
    const RELEASE_DEADLINE: Duration = Duration::from_secs(10);

    fn holder_on(host: &str) -> RepositoryLeaseHolder {
        RepositoryLeaseHolder {
            host: host.to_string(),
            process_id: 4242,
            acquired: DateTime::parse_from_rfc3339("2026-10-08T14:02:11Z").unwrap().with_timezone(&Utc),
        }
    }

    fn repository_in(directory: &tempfile::TempDir) -> LocalArtifactRepository {
        LocalArtifactRepository::new(directory.path().join("repository"), "/repository".to_string())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn lock_local_repository_makes_a_second_writer_wait_until_the_first_releases() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let root: PathBuf = workspace_directory.path().join("repository");
        let first_lock: LocalRepositoryLock = lock_local_repository(&root).await.unwrap();
        let second_root: PathBuf = root.clone();

        let second_writer: tokio::task::JoinHandle<Result<(), String>> = tokio::spawn(async move {
            let second_lock: LocalRepositoryLock = lock_local_repository(&second_root).await.map_err(|error| error.to_string())?;

            second_lock.release().map_err(|error| error.to_string())
        });

        thread::sleep(CONTENDED_WAIT);
        assert!(!second_writer.is_finished());

        first_lock.release().unwrap();

        let deadline: Instant = Instant::now() + RELEASE_DEADLINE;

        while !second_writer.is_finished() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(20));
        }

        assert!(second_writer.is_finished());
        second_writer.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn acquire_repository_lease_fails_a_second_command_and_names_the_holder() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);

        let first_lease: RepositoryLease = acquire_repository_lease(&repository, &holder_on("first-host")).await.unwrap();
        let second_attempt: Result<RepositoryLease, AppError> = acquire_repository_lease(&repository, &holder_on("second-host")).await;

        let Err(second_error) = second_attempt
        else {
            panic!("a second lease was granted while the first was held");
        };

        assert!(second_error.to_string().contains("host=first-host"));

        first_lease.release(&repository).await.unwrap();

        let third_lease: RepositoryLease = acquire_repository_lease(&repository, &holder_on("second-host")).await.unwrap();

        third_lease.release(&repository).await.unwrap();
        assert_eq!(repository.get_object(repository_key::REPOSITORY_LOCK_KEY).await.unwrap(), None);
    }

    #[tokio::test]
    async fn release_leaves_a_lease_another_command_took_since() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);
        let first_lease: RepositoryLease = acquire_repository_lease(&repository, &holder_on("first-host")).await.unwrap();

        remove_repository_lease(&repository).await.unwrap();
        let second_lease: RepositoryLease = acquire_repository_lease(&repository, &holder_on("second-host")).await.unwrap();
        let first_release: Result<(), AppError> = first_lease.release(&repository).await;

        assert!(first_release.unwrap_err().to_string().contains("host=second-host"));
        second_lease.release(&repository).await.unwrap();
    }

    #[tokio::test]
    async fn remove_repository_lease_names_the_holder_it_removed() {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let repository: LocalArtifactRepository = repository_in(&workspace_directory);

        acquire_repository_lease(&repository, &holder_on("crashed-host")).await.unwrap();
        let first_removal: LeaseRemovalKind = remove_repository_lease(&repository).await.unwrap();
        let second_removal: LeaseRemovalKind = remove_repository_lease(&repository).await.unwrap();

        assert_eq!(
            first_removal,
            LeaseRemovalKind::Removed {
                holder_description: holder_on("crashed-host").describe(),
            },
        );
        assert_eq!(second_removal, LeaseRemovalKind::NotHeld);
    }

    #[tokio::test]
    async fn acquire_repository_lease_on_cloudflare_r2_puts_only_if_absent_and_names_a_held_lease() {
        let held_lease_body: String = serde_json::to_string(&holder_on("other-host")).unwrap();
        let http_client: RecordingHttpClient = RecordingHttpClient::answering(vec![
            CannedResponse::s3_error(412, "PreconditionFailed"),
            CannedResponse::object("\"lease-tag\"", &held_lease_body),
        ]);

        let attempt: Result<RepositoryLease, AppError> = acquire_repository_lease(&http_client.repository(), &holder_on("this-host")).await;
        let recorded_requests: Vec<RecordedRequest> = http_client.recorded_requests();

        let Err(error) = attempt
        else {
            panic!("a lease was granted while another was held");
        };

        assert!(error.to_string().contains("host=other-host"));
        assert_eq!(recorded_requests[0].method, "PUT");
        assert!(recorded_requests[0].uri.contains(repository_key::REPOSITORY_LOCK_KEY));
        assert_eq!(recorded_requests[0].if_none_match.as_deref(), Some("*"));
        assert_eq!(recorded_requests[1].method, "GET");
    }
}
