use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use dashmap::DashMap;
use futures_util::StreamExt;
use futures_util::future::{self, Either};
use futures_util::stream;
use shared::AppErrorStatic;
use shared::artifact::{content_address, fetch, manifest, repository_key};
use shared::filesystem;
use shared::http::{HttpCacheMode, HttpFetch};
use tokio::sync::{OnceCell, watch};

use crate::course_library;

const PREFETCH_CONCURRENCY: usize = 4;

/// Where one engine's recording of a clip is fetched from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClipRecording {
    pub(crate) audio_sha256: String,
    /// The recordings base of the publication whose shard named this recording.
    pub(crate) audio_base_url: String,
}

/// Recordings downloaded and verified into one directory; callers asking for the same recording at the same time share
/// one download.
pub(crate) struct RecordingCache {
    audio_directory: PathBuf,
    /// By SHA-256, while the download runs.
    downloads_in_flight: DashMap<String, Arc<OnceCell<PathBuf>>>,
    /// Plays waiting for their recording; a prefetch starts no download while any is.
    foreground_fetch_count: watch::Sender<usize>,
}

impl RecordingCache {
    pub(crate) fn create(audio_directory: PathBuf) -> RecordingCache {
        RecordingCache {
            audio_directory,
            downloads_in_flight: DashMap::new(),
            foreground_fetch_count: watch::Sender::new(0),
        }
    }

    /// The recording's local file, downloaded first when absent, ahead of any prefetch.
    pub(crate) async fn foreground_recording(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording) -> Result<PathBuf, AppErrorStatic> {
        let path: PathBuf = self.recording_file_path(&recording.audio_sha256);

        if path.exists() {
            return Ok(path);
        }

        let _foreground_fetch: ForegroundFetch<'_> = ForegroundFetch::begin(&self.foreground_fetch_count);

        self.download_once(http_fetch, recording, &path).await
    }

    /// Downloads each absent recording at most once, `PREFETCH_CONCURRENCY` at a time, starting none while a play waits
    /// for its recording or once `cancellation` is cancelled.
    pub(crate) async fn prefetch(
        &self,
        http_fetch: &impl HttpFetch,
        recordings: Vec<ClipRecording>,
        cancellation: &PrefetchCancellation,
    ) -> RecordingPrefetchSummary {
        let mut seen_sha256s: BTreeSet<String> = BTreeSet::new();
        let mut absent_recordings: Vec<ClipRecording> = Vec::new();
        let mut already_cached_count: usize = 0;

        for recording in recordings {
            let first_sighting: bool = seen_sha256s.insert(recording.audio_sha256.clone());

            if !first_sighting {
                continue;
            }

            if self.recording_file_path(&recording.audio_sha256).exists() {
                already_cached_count += 1;
                continue;
            }

            absent_recordings.push(recording);
        }

        let step_outcomes: Vec<PrefetchStepKind> = stream::iter(absent_recordings)
            .map(|recording| self.prefetch_one(http_fetch, recording, cancellation))
            .buffer_unordered(PREFETCH_CONCURRENCY)
            .collect()
            .await;

        RecordingPrefetchSummary::count(already_cached_count, &step_outcomes)
    }

    #[cfg(test)]
    pub(crate) fn subscribe_to_foreground_fetch_count(&self) -> watch::Receiver<usize> {
        self.foreground_fetch_count.subscribe()
    }

    fn recording_file_path(&self, audio_sha256: &str) -> PathBuf {
        self.audio_directory.join(format!("{audio_sha256}.{}", manifest::AUDIO_FILE_EXTENSION))
    }

    async fn prefetch_one(&self, http_fetch: &impl HttpFetch, recording: ClipRecording, cancellation: &PrefetchCancellation) -> PrefetchStepKind {
        let wait_outcome: PrefetchWaitKind = self.wait_for_no_foreground_fetch(cancellation).await;

        if wait_outcome == PrefetchWaitKind::Cancelled {
            return PrefetchStepKind::Cancelled;
        }

        let path: PathBuf = self.recording_file_path(&recording.audio_sha256);
        let downloaded: Result<PathBuf, AppErrorStatic> = self.download_once(http_fetch, &recording, &path).await;

        match downloaded {
            Ok(_) => PrefetchStepKind::Downloaded,
            Err(error) => {
                log::warn!("could not prefetch a recording; [audio_sha256={} error={error}]", recording.audio_sha256);

                PrefetchStepKind::Failed
            },
        }
    }

    async fn wait_for_no_foreground_fetch(&self, cancellation: &PrefetchCancellation) -> PrefetchWaitKind {
        if cancellation.is_cancelled() {
            return PrefetchWaitKind::Cancelled;
        }

        let mut foreground_fetch_count: watch::Receiver<usize> = self.foreground_fetch_count.subscribe();
        let no_foreground_fetch = Box::pin(async move {
            /* The sender lives as long as `self`, so the wait cannot fail. */
            let _ = foreground_fetch_count.wait_for(|count| *count == 0).await;
        });
        let cancelled = Box::pin(cancellation.wait_for_cancel());
        let first_finished: Either<((), _), ((), _)> = future::select(no_foreground_fetch, cancelled).await;

        match first_finished {
            Either::Left(_) if !cancellation.is_cancelled() => PrefetchWaitKind::Clear,
            Either::Left(_) | Either::Right(_) => PrefetchWaitKind::Cancelled,
        }
    }

    async fn download_once(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording, path: &Path) -> Result<PathBuf, AppErrorStatic> {
        let audio_sha256: &str = &recording.audio_sha256;
        let download: Arc<OnceCell<PathBuf>> = self.downloads_in_flight.entry(audio_sha256.to_string()).or_default().clone();
        let downloaded: Result<PathBuf, AppErrorStatic> = download
            .get_or_try_init(|| self.download(http_fetch, recording, path))
            .await
            .cloned();

        self.downloads_in_flight.remove_if(audio_sha256, |_, in_flight| Arc::ptr_eq(in_flight, &download));

        downloaded
    }

    async fn download(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording, path: &Path) -> Result<PathBuf, AppErrorStatic> {
        /* Another download of the same recording may have finished since the caller looked. */
        if path.exists() {
            return Ok(path.to_path_buf());
        }

        let url: String = fetch::join_url(&recording.audio_base_url, &repository_key::audio_key(&recording.audio_sha256));
        let bytes: Vec<u8> = fetch::fetch_bytes(http_fetch, &url, HttpCacheMode::Default).await?;

        content_address::verify_sha256(&url, &bytes, &recording.audio_sha256)?;
        course_library::create_directory(&self.audio_directory)?;
        filesystem::write_atomically(path, &bytes)?;

        Ok(path.to_path_buf())
    }
}

/// Counted while it lives, so a play whose future is dropped stops holding prefetches back.
struct ForegroundFetch<'cache> {
    foreground_fetch_count: &'cache watch::Sender<usize>,
}

impl<'cache> ForegroundFetch<'cache> {
    fn begin(foreground_fetch_count: &'cache watch::Sender<usize>) -> ForegroundFetch<'cache> {
        foreground_fetch_count.send_modify(|count| *count += 1);

        ForegroundFetch { foreground_fetch_count }
    }
}

impl Drop for ForegroundFetch<'_> {
    fn drop(&mut self) {
        self.foreground_fetch_count.send_modify(|count| *count -= 1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PrefetchWaitKind {
    Clear,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PrefetchStepKind {
    Downloaded,
    Failed,
    Cancelled,
}

/// How a prefetch's distinct recordings ended up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecordingPrefetchSummary {
    pub(crate) already_cached_count: usize,
    pub(crate) downloaded_count: usize,
    pub(crate) failed_count: usize,
    pub(crate) cancelled_count: usize,
}

impl RecordingPrefetchSummary {
    fn count(already_cached_count: usize, step_outcomes: &[PrefetchStepKind]) -> RecordingPrefetchSummary {
        let count_of = |kind: PrefetchStepKind| step_outcomes.iter().filter(|outcome| **outcome == kind).count();

        RecordingPrefetchSummary {
            already_cached_count,
            downloaded_count: count_of(PrefetchStepKind::Downloaded),
            failed_count: count_of(PrefetchStepKind::Failed),
            cancelled_count: count_of(PrefetchStepKind::Cancelled),
        }
    }
}

/// Stops a prefetch from starting further downloads once cancelled; downloads already running finish.
#[derive(uniffi::Object)]
pub struct PrefetchCancellation {
    cancelled: watch::Sender<bool>,
}

#[uniffi::export]
impl PrefetchCancellation {
    #[uniffi::constructor]
    pub fn new() -> Arc<PrefetchCancellation> {
        Arc::new(PrefetchCancellation {
            cancelled: watch::Sender::new(false),
        })
    }

    pub fn cancel(&self) {
        self.cancelled.send_replace(true);
    }
}

impl PrefetchCancellation {
    pub(crate) fn is_cancelled(&self) -> bool {
        *self.cancelled.borrow()
    }

    async fn wait_for_cancel(&self) {
        let mut cancelled: watch::Receiver<bool> = self.cancelled.subscribe();

        /* The sender lives as long as `self`, so the wait cannot fail. */
        let _ = cancelled.wait_for(|cancelled| *cancelled).await;
    }
}

#[cfg(test)]
mod tests {
    use shared::http::{DirectoryFetch, HttpRequest, HttpResponse};

    use super::*;

    /// Serves files like `DirectoryFetch`, holding every response until released and recording each URL asked for.
    struct GatedDirectoryFetch {
        requested_urls: watch::Sender<Vec<String>>,
        released: watch::Sender<bool>,
    }

    impl GatedDirectoryFetch {
        fn create(released: bool) -> GatedDirectoryFetch {
            GatedDirectoryFetch {
                requested_urls: watch::Sender::new(Vec::new()),
                released: watch::Sender::new(released),
            }
        }

        fn release(&self) {
            self.released.send_replace(true);
        }

        fn requested_urls(&self) -> Vec<String> {
            self.requested_urls.borrow().clone()
        }

        async fn wait_for_request_count(&self, request_count: usize) {
            let mut requested_urls: watch::Receiver<Vec<String>> = self.requested_urls.subscribe();

            requested_urls.wait_for(|urls| urls.len() >= request_count).await.unwrap();
        }
    }

    impl HttpFetch for GatedDirectoryFetch {
        async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppErrorStatic> {
            let mut released: watch::Receiver<bool> = self.released.subscribe();

            self.requested_urls.send_modify(|urls| urls.push(request.url.clone()));
            released.wait_for(|released| *released).await.unwrap();

            DirectoryFetch.get(request).await
        }
    }

    struct CacheFixture {
        repository: tempfile::TempDir,
        cache_directory: tempfile::TempDir,
        cache: RecordingCache,
    }

    impl CacheFixture {
        fn create() -> CacheFixture {
            let repository: tempfile::TempDir = tempfile::tempdir().unwrap();
            let cache_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
            let cache: RecordingCache = RecordingCache::create(cache_directory.path().join("audio"));

            CacheFixture {
                repository,
                cache_directory,
                cache,
            }
        }

        fn publish_recording(&self, contents: &str) -> ClipRecording {
            let audio_sha256: String = content_address::sha256_hex(contents.as_bytes());
            let path: PathBuf = self.repository.path().join(repository_key::audio_key(&audio_sha256));

            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, contents).unwrap();

            ClipRecording {
                audio_sha256,
                audio_base_url: self.repository.path().display().to_string(),
            }
        }

        fn recording_url(&self, recording: &ClipRecording) -> String {
            fetch::join_url(&recording.audio_base_url, &repository_key::audio_key(&recording.audio_sha256))
        }

        fn cache_recording(&self, recording: &ClipRecording, contents: &str) {
            let path: PathBuf = self.cache.recording_file_path(&recording.audio_sha256);

            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, contents).unwrap();
        }

        fn is_cached(&self, recording: &ClipRecording) -> bool {
            self.cache_directory.path().join("audio").join(format!("{}.{}", recording.audio_sha256, manifest::AUDIO_FILE_EXTENSION)).exists()
        }
    }

    #[tokio::test]
    async fn prefetch_downloads_each_absent_recording_once_and_skips_one_already_cached() {
        let fixture: CacheFixture = CacheFixture::create();
        let cached: ClipRecording = fixture.publish_recording("cached");
        let absent: ClipRecording = fixture.publish_recording("absent");
        let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(true);
        fixture.cache_recording(&cached, "cached");

        let summary: RecordingPrefetchSummary = fixture
            .cache
            .prefetch(&http_fetch, vec![cached.clone(), absent.clone(), absent.clone()], &PrefetchCancellation::new())
            .await;

        assert_eq!(http_fetch.requested_urls(), vec![fixture.recording_url(&absent)]);
        assert!(fixture.is_cached(&absent));
        assert_eq!(
            summary,
            RecordingPrefetchSummary {
                already_cached_count: 1,
                downloaded_count: 1,
                failed_count: 0,
                cancelled_count: 0,
            },
        );
    }

    #[tokio::test]
    async fn a_play_of_a_recording_being_prefetched_shares_its_download() {
        let fixture: CacheFixture = CacheFixture::create();
        let recording: ClipRecording = fixture.publish_recording("shared");
        let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(false);
        let mut foreground_fetch_count: watch::Receiver<usize> = fixture.cache.subscribe_to_foreground_fetch_count();
        let cancellation: Arc<PrefetchCancellation> = PrefetchCancellation::new();

        let prefetch = fixture.cache.prefetch(&http_fetch, vec![recording.clone()], &cancellation);
        let play = async {
            http_fetch.wait_for_request_count(1).await;

            fixture.cache.foreground_recording(&http_fetch, &recording).await
        };
        let release = async {
            foreground_fetch_count.wait_for(|count| *count == 1).await.unwrap();
            http_fetch.release();
        };
        let (summary, played, ()): (RecordingPrefetchSummary, Result<PathBuf, AppErrorStatic>, ()) = future::join3(prefetch, play, release).await;

        assert_eq!(http_fetch.requested_urls(), vec![fixture.recording_url(&recording)]);
        assert_eq!(std::fs::read(played.unwrap()).unwrap(), b"shared".to_vec());
        assert_eq!(summary.downloaded_count, 1);
    }

    #[tokio::test]
    async fn prefetch_starts_no_download_while_a_play_waits_for_its_recording() {
        let fixture: CacheFixture = CacheFixture::create();
        let played: ClipRecording = fixture.publish_recording("played");
        let prefetched: ClipRecording = fixture.publish_recording("prefetched");
        let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(false);
        let cancellation: Arc<PrefetchCancellation> = PrefetchCancellation::new();

        let play = fixture.cache.foreground_recording(&http_fetch, &played);
        let prefetch = async {
            http_fetch.wait_for_request_count(1).await;

            fixture.cache.prefetch(&http_fetch, vec![prefetched.clone()], &cancellation).await
        };
        let release = async {
            http_fetch.wait_for_request_count(1).await;

            for _ in 0..PREFETCH_CONCURRENCY * 4 {
                tokio::task::yield_now().await;
            }

            let requested_while_the_play_waited: Vec<String> = http_fetch.requested_urls();

            http_fetch.release();

            requested_while_the_play_waited
        };
        let (play_outcome, summary, requested_while_the_play_waited): (Result<PathBuf, AppErrorStatic>, RecordingPrefetchSummary, Vec<String>) =
            future::join3(play, prefetch, release).await;

        assert!(play_outcome.is_ok());
        assert_eq!(requested_while_the_play_waited, vec![fixture.recording_url(&played)]);
        assert_eq!(http_fetch.requested_urls(), vec![fixture.recording_url(&played), fixture.recording_url(&prefetched)]);
        assert_eq!(summary.downloaded_count, 1);
    }

    #[tokio::test]
    async fn a_cancelled_prefetch_starts_no_further_downloads_and_finishes_those_running() {
        let fixture: CacheFixture = CacheFixture::create();
        let extra_recording_count: usize = 2;
        let recordings: Vec<ClipRecording> = (0..PREFETCH_CONCURRENCY + extra_recording_count)
            .map(|position| fixture.publish_recording(&format!("recording {position}")))
            .collect();
        let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(false);
        let cancellation: Arc<PrefetchCancellation> = PrefetchCancellation::new();

        let prefetch = fixture.cache.prefetch(&http_fetch, recordings.clone(), &cancellation);
        let cancel = async {
            http_fetch.wait_for_request_count(PREFETCH_CONCURRENCY).await;
            cancellation.cancel();
            http_fetch.release();
        };
        let (summary, ()): (RecordingPrefetchSummary, ()) = future::join(prefetch, cancel).await;
        let cached_count: usize = recordings.iter().filter(|recording| fixture.is_cached(recording)).count();

        assert_eq!(http_fetch.requested_urls().len(), PREFETCH_CONCURRENCY);
        assert_eq!(cached_count, PREFETCH_CONCURRENCY);
        assert_eq!(
            summary,
            RecordingPrefetchSummary {
                already_cached_count: 0,
                downloaded_count: PREFETCH_CONCURRENCY,
                failed_count: 0,
                cancelled_count: extra_recording_count,
            },
        );
    }
}
