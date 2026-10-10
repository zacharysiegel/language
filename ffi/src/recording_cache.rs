use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use dashmap::DashMap;
use futures_util::StreamExt;
use futures_util::future::{self, Either};
use futures_util::stream::FuturesUnordered;
use shared::AppErrorStatic;
use shared::artifact::{content_address, fetch, manifest, repository_key};
use shared::filesystem;
use shared::http::{HttpCacheMode, HttpFetch, HttpRequest, HttpResponse};
use tokio::sync::{OnceCell, watch};

use crate::course_library;

const PREFETCH_CONCURRENCY: usize = 4;
const UNCONFIRMED_CONNECTION_PREFETCH_CONCURRENCY: usize = 1;
/// The in-flight map's own reference, with no claim holding the download.
const UNCLAIMED_IN_FLIGHT_DOWNLOAD_REFERENCE_COUNT: usize = 1;

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
    waiting_plays: watch::Sender<WaitingPlays>,
}

impl RecordingCache {
    pub(crate) fn create(audio_directory: PathBuf) -> RecordingCache {
        RecordingCache {
            audio_directory,
            downloads_in_flight: DashMap::new(),
            waiting_plays: watch::Sender::new(WaitingPlays::default()),
        }
    }

    /// The recording's local file, downloaded first when absent, ahead of any prefetch.
    pub(crate) async fn foreground_recording(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording) -> Result<PathBuf, AppErrorStatic> {
        let path: PathBuf = self.recording_file_path(&recording.audio_sha256);

        if path.exists() {
            return Ok(path);
        }

        let _waiting_play: WaitingPlay<'_> = WaitingPlay::begin(&self.waiting_plays, &recording.audio_sha256);
        let downloaded: Result<PathBuf, RecordingDownloadError> = self.download_once(http_fetch, recording, &path).await;

        downloaded.map_err(|failure| failure.error)
    }

    /// Downloads each absent recording at most once, `PREFETCH_CONCURRENCY` at a time once one download has reached the
    /// server. Starts none while a play waits for its recording or once `cancellation` is cancelled, and stops at the
    /// first connection failure. A play waiting for another recording interrupts the downloads running, which resume
    /// first once no play waits.
    pub(crate) async fn prefetch(
        &self,
        http_fetch: &impl HttpFetch,
        recordings: Vec<ClipRecording>,
        cancellation: &PrefetchCancellation,
    ) -> RecordingPrefetchSummary {
        let (already_cached_count, absent_recordings): (usize, Vec<ClipRecording>) = self.distinct_absent_recordings(recordings);
        let mut queue: PrefetchQueue = PrefetchQueue::create(absent_recordings);
        let mut running = FuturesUnordered::new();
        let mut connection: ConnectionConfirmationKind = ConnectionConfirmationKind::Unconfirmed;
        let mut summary: RecordingPrefetchSummary = RecordingPrefetchSummary::create(already_cached_count);

        loop {
            while running.len() < connection.download_slot_count() && self.may_start_download(cancellation) {
                let Some(queued) = queue.pop_front()
                else {
                    break;
                };

                running.push(self.prefetch_one(http_fetch, queued));
            }

            if running.is_empty() {
                if queue.is_empty() || cancellation.is_cancelled() {
                    break;
                }

                self.wait_for_no_waiting_play(cancellation).await;
                continue;
            }

            let next_step: Option<PrefetchStep> = running.next().await;
            let Some(step) = next_step
            else {
                continue;
            };

            match step.kind {
                PrefetchStepKind::Downloaded => {
                    summary.downloaded_count += 1;
                    connection = ConnectionConfirmationKind::Confirmed;
                },
                PrefetchStepKind::Interrupted => queue.requeue(step.queued),
                PrefetchStepKind::Failed(failure) => {
                    summary.failed_count += 1;
                    log_prefetch_failure(&step.queued.recording, &failure);

                    match failure.kind {
                        RecordingFailureKind::Recording => connection = ConnectionConfirmationKind::Confirmed,
                        RecordingFailureKind::Connection => {
                            summary.end = PrefetchEndKind::ConnectionFailed;
                            break;
                        },
                    }
                },
            }
        }

        summary.remaining_count = queue.len() + running.len();

        if summary.end == PrefetchEndKind::Completed && summary.remaining_count > 0 {
            summary.end = PrefetchEndKind::Cancelled;
        }

        summary
    }

    #[cfg(test)]
    pub(crate) fn subscribe_to_waiting_plays(&self) -> watch::Receiver<WaitingPlays> {
        self.waiting_plays.subscribe()
    }

    fn recording_file_path(&self, audio_sha256: &str) -> PathBuf {
        self.audio_directory.join(format!("{audio_sha256}.{}", manifest::AUDIO_FILE_EXTENSION))
    }

    /// The count of distinct recordings already cached, and the distinct others in their order.
    fn distinct_absent_recordings(&self, recordings: Vec<ClipRecording>) -> (usize, Vec<ClipRecording>) {
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

        (already_cached_count, absent_recordings)
    }

    fn may_start_download(&self, cancellation: &PrefetchCancellation) -> bool {
        !cancellation.is_cancelled() && self.waiting_plays.borrow().is_empty()
    }

    async fn prefetch_one(&self, http_fetch: &impl HttpFetch, queued: QueuedRecording) -> PrefetchStep {
        let kind: PrefetchStepKind = self.download_unless_interrupted(http_fetch, &queued.recording).await;

        PrefetchStep { queued, kind }
    }

    async fn download_unless_interrupted(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording) -> PrefetchStepKind {
        let path: PathBuf = self.recording_file_path(&recording.audio_sha256);
        let download = Box::pin(self.download_once(http_fetch, recording, &path));
        let interruption = Box::pin(self.wait_for_a_play_of_another_recording(&recording.audio_sha256));
        let first_finished: Either<(Result<PathBuf, RecordingDownloadError>, _), ((), _)> = future::select(download, interruption).await;

        match first_finished {
            Either::Left((Ok(_), _)) => PrefetchStepKind::Downloaded,
            Either::Left((Err(failure), _)) => PrefetchStepKind::Failed(failure),
            Either::Right(((), _)) => PrefetchStepKind::Interrupted,
        }
    }

    /// Returns once a play waits and none of the waiting plays is for `audio_sha256`.
    async fn wait_for_a_play_of_another_recording(&self, audio_sha256: &str) {
        let mut waiting_plays: watch::Receiver<WaitingPlays> = self.waiting_plays.subscribe();

        /* The sender lives as long as `self`, so the wait cannot fail. */
        let _ = waiting_plays.wait_for(|plays| plays.wait_only_for_recordings_other_than(audio_sha256)).await;
    }

    async fn wait_for_no_waiting_play(&self, cancellation: &PrefetchCancellation) {
        let mut waiting_plays: watch::Receiver<WaitingPlays> = self.waiting_plays.subscribe();
        let no_waiting_play = Box::pin(async move {
            /* The sender lives as long as `self`, so the wait cannot fail. */
            let _ = waiting_plays.wait_for(|plays| plays.is_empty()).await;
        });
        let cancelled = Box::pin(cancellation.wait_for_cancel());

        future::select(no_waiting_play, cancelled).await;
    }

    async fn download_once(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording, path: &Path) -> Result<PathBuf, RecordingDownloadError> {
        let claim: InFlightDownloadClaim<'_> = InFlightDownloadClaim::join(&self.downloads_in_flight, &recording.audio_sha256);

        claim
            .download()
            .get_or_try_init(|| self.download(http_fetch, recording, path))
            .await
            .cloned()
    }

    async fn download(&self, http_fetch: &impl HttpFetch, recording: &ClipRecording, path: &Path) -> Result<PathBuf, RecordingDownloadError> {
        /* Another download of the same recording may have finished since the caller looked. */
        if path.exists() {
            return Ok(path.to_path_buf());
        }

        let url: String = fetch::join_url(&recording.audio_base_url, &repository_key::audio_key(&recording.audio_sha256));
        let response: HttpResponse = http_fetch
            .get(&HttpRequest {
                url: url.clone(),
                cache_mode: HttpCacheMode::Default,
            })
            .await
            .map_err(RecordingDownloadError::connection)?;

        if !response.is_success() {
            let error: AppErrorStatic = AppErrorStatic::from(format!("fetching a recording failed; [url={url} status={}]", response.status));

            return Err(RecordingDownloadError::recording(error));
        }

        content_address::verify_sha256(&url, &response.bytes, &recording.audio_sha256).map_err(RecordingDownloadError::recording)?;
        course_library::create_directory(&self.audio_directory).map_err(RecordingDownloadError::recording)?;
        filesystem::write_atomically(path, &response.bytes).map_err(RecordingDownloadError::recording)?;

        Ok(path.to_path_buf())
    }
}

fn log_prefetch_failure(recording: &ClipRecording, failure: &RecordingDownloadError) {
    match failure.kind {
        RecordingFailureKind::Recording => {
            log::debug!("could not prefetch a recording; [audio_sha256={} error={}]", recording.audio_sha256, failure.error);
        },
        RecordingFailureKind::Connection => {
            log::warn!("stopped prefetching recordings at a connection failure; [audio_sha256={} error={}]", recording.audio_sha256, failure.error);
        },
    }
}

/// A share of one recording's in-flight download, whose entry leaves the in-flight map with its last claim.
struct InFlightDownloadClaim<'cache> {
    downloads_in_flight: &'cache DashMap<String, Arc<OnceCell<PathBuf>>>,
    audio_sha256: String,
    /// `None` only while the claim is being dropped.
    download: Option<Arc<OnceCell<PathBuf>>>,
}

impl<'cache> InFlightDownloadClaim<'cache> {
    fn join(downloads_in_flight: &'cache DashMap<String, Arc<OnceCell<PathBuf>>>, audio_sha256: &str) -> InFlightDownloadClaim<'cache> {
        let download: Arc<OnceCell<PathBuf>> = downloads_in_flight.entry(audio_sha256.to_string()).or_default().clone();

        InFlightDownloadClaim {
            downloads_in_flight,
            audio_sha256: audio_sha256.to_string(),
            download: Some(download),
        }
    }

    fn download(&self) -> &OnceCell<PathBuf> {
        self.download.as_deref().expect("a claim holds its download until it is dropped")
    }
}

impl Drop for InFlightDownloadClaim<'_> {
    fn drop(&mut self) {
        let Some(download) = self.download.take()
        else {
            return;
        };

        let download_pointer: *const OnceCell<PathBuf> = Arc::as_ptr(&download);

        /* This claim's reference goes first, so the last claim to finish sees only the map's. */
        drop(download);

        /* The shard lock `remove_if` holds keeps a new claim from cloning the entry between the count and the removal. */
        self.downloads_in_flight.remove_if(&self.audio_sha256, |_, in_flight| {
            Arc::as_ptr(in_flight) == download_pointer && Arc::strong_count(in_flight) == UNCLAIMED_IN_FLIGHT_DOWNLOAD_REFERENCE_COUNT
        });
    }
}

/// The plays waiting for their recording, by the recording's SHA-256.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct WaitingPlays {
    play_count_by_audio_sha256: BTreeMap<String, usize>,
}

impl WaitingPlays {
    pub(crate) fn is_empty(&self) -> bool {
        self.play_count_by_audio_sha256.is_empty()
    }

    fn wait_only_for_recordings_other_than(&self, audio_sha256: &str) -> bool {
        !self.is_empty() && !self.play_count_by_audio_sha256.contains_key(audio_sha256)
    }

    fn add(&mut self, audio_sha256: &str) {
        *self.play_count_by_audio_sha256.entry(audio_sha256.to_string()).or_default() += 1;
    }

    fn remove(&mut self, audio_sha256: &str) {
        let Some(play_count) = self.play_count_by_audio_sha256.get_mut(audio_sha256)
        else {
            return;
        };

        *play_count -= 1;

        if *play_count == 0 {
            self.play_count_by_audio_sha256.remove(audio_sha256);
        }
    }
}

/// Counted while it lives, so a play whose future is dropped stops holding prefetches back.
struct WaitingPlay<'cache> {
    waiting_plays: &'cache watch::Sender<WaitingPlays>,
    audio_sha256: String,
}

impl<'cache> WaitingPlay<'cache> {
    fn begin(waiting_plays: &'cache watch::Sender<WaitingPlays>, audio_sha256: &str) -> WaitingPlay<'cache> {
        waiting_plays.send_modify(|plays| plays.add(audio_sha256));

        WaitingPlay {
            waiting_plays,
            audio_sha256: audio_sha256.to_string(),
        }
    }
}

impl Drop for WaitingPlay<'_> {
    fn drop(&mut self) {
        self.waiting_plays.send_modify(|plays| plays.remove(&self.audio_sha256));
    }
}

/// The recordings a prefetch has yet to download, front first in the order they were asked for.
struct PrefetchQueue {
    recordings_by_position: BTreeMap<usize, ClipRecording>,
}

impl PrefetchQueue {
    fn create(recordings: Vec<ClipRecording>) -> PrefetchQueue {
        PrefetchQueue {
            recordings_by_position: recordings.into_iter().enumerate().collect(),
        }
    }

    fn pop_front(&mut self) -> Option<QueuedRecording> {
        let (position, recording): (usize, ClipRecording) = self.recordings_by_position.pop_first()?;

        Some(QueuedRecording { position, recording })
    }

    /// Ahead of every recording not yet started, since each of those was queued after it.
    fn requeue(&mut self, queued: QueuedRecording) {
        self.recordings_by_position.insert(queued.position, queued.recording);
    }

    fn len(&self) -> usize {
        self.recordings_by_position.len()
    }

    fn is_empty(&self) -> bool {
        self.recordings_by_position.is_empty()
    }
}

struct QueuedRecording {
    position: usize,
    recording: ClipRecording,
}

struct PrefetchStep {
    queued: QueuedRecording,
    kind: PrefetchStepKind,
}

#[derive(Debug)]
enum PrefetchStepKind {
    Downloaded,
    /// Stopped for a play waiting for another recording.
    Interrupted,
    Failed(RecordingDownloadError),
}

/// Whether a prefetch has seen a download reach the server, which lets it run more than one at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionConfirmationKind {
    Unconfirmed,
    Confirmed,
}

impl ConnectionConfirmationKind {
    fn download_slot_count(self) -> usize {
        match self {
            ConnectionConfirmationKind::Unconfirmed => UNCONFIRMED_CONNECTION_PREFETCH_CONCURRENCY,
            ConnectionConfirmationKind::Confirmed => PREFETCH_CONCURRENCY,
        }
    }
}

#[derive(Debug)]
struct RecordingDownloadError {
    kind: RecordingFailureKind,
    error: AppErrorStatic,
}

impl RecordingDownloadError {
    fn connection(error: impl Into<AppErrorStatic>) -> RecordingDownloadError {
        RecordingDownloadError {
            kind: RecordingFailureKind::Connection,
            error: error.into(),
        }
    }

    fn recording(error: impl Into<AppErrorStatic>) -> RecordingDownloadError {
        RecordingDownloadError {
            kind: RecordingFailureKind::Recording,
            error: error.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecordingFailureKind {
    /// The repository could not be reached, so no other recording can be fetched either.
    Connection,
    /// This recording alone could not be fetched or stored.
    Recording,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PrefetchEndKind {
    Completed,
    Cancelled,
    ConnectionFailed,
}

/// How a prefetch's distinct recordings ended up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecordingPrefetchSummary {
    pub(crate) already_cached_count: usize,
    pub(crate) downloaded_count: usize,
    pub(crate) failed_count: usize,
    /// Neither downloaded nor failed when the prefetch ended.
    pub(crate) remaining_count: usize,
    pub(crate) end: PrefetchEndKind,
}

impl RecordingPrefetchSummary {
    fn create(already_cached_count: usize) -> RecordingPrefetchSummary {
        RecordingPrefetchSummary {
            already_cached_count,
            downloaded_count: 0,
            failed_count: 0,
            remaining_count: 0,
            end: PrefetchEndKind::Completed,
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
    use std::sync::Barrier;

    use shared::http::DirectoryFetch;

    use super::*;

    /// Races the run of each concurrency test repeats to surface.
    const REPEAT_COUNT: usize = 20;

    #[test]
    fn in_flight_entry_leaves_the_map_when_two_claims_finish_at_once() {
        const ATTEMPT_COUNT: usize = 2_000;

        for _ in 0..ATTEMPT_COUNT {
            let downloads_in_flight: DashMap<String, Arc<OnceCell<PathBuf>>> = DashMap::new();
            let first_claim: InFlightDownloadClaim<'_> = InFlightDownloadClaim::join(&downloads_in_flight, "recording");
            let second_claim: InFlightDownloadClaim<'_> = InFlightDownloadClaim::join(&downloads_in_flight, "recording");
            let release: Barrier = Barrier::new(2);

            std::thread::scope(|scope| {
                scope.spawn(|| {
                    release.wait();
                    drop(first_claim);
                });
                scope.spawn(|| {
                    release.wait();
                    drop(second_claim);
                });
            });

            assert!(downloads_in_flight.is_empty());
        }
    }

    #[derive(Debug, Default)]
    struct FetchGate {
        released_all: bool,
        released_urls: BTreeSet<String>,
    }

    impl FetchGate {
        fn admits(&self, url: &str) -> bool {
            self.released_all || self.released_urls.contains(url)
        }
    }

    /// Serves files like `DirectoryFetch`, holding each response until its URL is released, and recording each URL
    /// asked for, held and served.
    struct GatedDirectoryFetch {
        requested_urls: watch::Sender<Vec<String>>,
        held_urls: watch::Sender<Vec<String>>,
        served_urls: watch::Sender<Vec<String>>,
        gate: watch::Sender<FetchGate>,
    }

    impl GatedDirectoryFetch {
        fn create(released_all: bool) -> GatedDirectoryFetch {
            GatedDirectoryFetch {
                requested_urls: watch::Sender::new(Vec::new()),
                held_urls: watch::Sender::new(Vec::new()),
                served_urls: watch::Sender::new(Vec::new()),
                gate: watch::Sender::new(FetchGate {
                    released_all,
                    released_urls: BTreeSet::new(),
                }),
            }
        }

        fn release(&self) {
            self.gate.send_modify(|gate| gate.released_all = true);
        }

        fn release_url(&self, url: &str) {
            self.gate.send_modify(|gate| {
                gate.released_urls.insert(url.to_string());
            });
        }

        fn requested_urls(&self) -> Vec<String> {
            self.requested_urls.borrow().clone()
        }

        fn served_urls(&self) -> Vec<String> {
            self.served_urls.borrow().clone()
        }

        fn request_count_of(&self, url: &str) -> usize {
            self.requested_urls.borrow().iter().filter(|requested_url| *requested_url == url).count()
        }

        fn serve_count_of(&self, url: &str) -> usize {
            self.served_urls.borrow().iter().filter(|served_url| *served_url == url).count()
        }

        async fn wait_for_request_count(&self, request_count: usize) {
            let mut requested_urls: watch::Receiver<Vec<String>> = self.requested_urls.subscribe();

            requested_urls.wait_for(|urls| urls.len() >= request_count).await.unwrap();
        }

        async fn wait_for_held_urls(&self, expected_urls: &[String]) {
            let mut held_urls: watch::Receiver<Vec<String>> = self.held_urls.subscribe();
            let expected_sorted_urls: Vec<String> = sorted(expected_urls.to_vec());

            held_urls.wait_for(|urls| sorted(urls.clone()) == expected_sorted_urls).await.unwrap();
        }
    }

    impl HttpFetch for GatedDirectoryFetch {
        async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppErrorStatic> {
            let mut gate: watch::Receiver<FetchGate> = self.gate.subscribe();

            self.requested_urls.send_modify(|urls| urls.push(request.url.clone()));

            let held_request: HeldRequest<'_> = HeldRequest::begin(&self.held_urls, &request.url);

            gate.wait_for(|gate| gate.admits(&request.url)).await.unwrap();
            drop(held_request);
            self.served_urls.send_modify(|urls| urls.push(request.url.clone()));

            DirectoryFetch.get(request).await
        }
    }

    /// Listed while it lives, so a request whose future is dropped stops counting as held.
    struct HeldRequest<'fetch> {
        held_urls: &'fetch watch::Sender<Vec<String>>,
        url: String,
    }

    impl<'fetch> HeldRequest<'fetch> {
        fn begin(held_urls: &'fetch watch::Sender<Vec<String>>, url: &str) -> HeldRequest<'fetch> {
            held_urls.send_modify(|urls| urls.push(url.to_string()));

            HeldRequest {
                held_urls,
                url: url.to_string(),
            }
        }
    }

    impl Drop for HeldRequest<'_> {
        fn drop(&mut self) {
            self.held_urls.send_modify(|urls| {
                let position: Option<usize> = urls.iter().position(|url| *url == self.url);

                if let Some(position) = position {
                    urls.remove(position);
                }
            });
        }
    }

    /// Fails every request as a transport failure after yielding once, recording each URL asked for.
    struct UnreachableFetch {
        requested_urls: watch::Sender<Vec<String>>,
    }

    impl HttpFetch for UnreachableFetch {
        async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppErrorStatic> {
            self.requested_urls.send_modify(|urls| urls.push(request.url.clone()));
            tokio::task::yield_now().await;

            Err(AppErrorStatic::from(format!("fetch failed; [url={} error=no network]", request.url)))
        }
    }

    fn sorted(mut urls: Vec<String>) -> Vec<String> {
        urls.sort();

        urls
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
            let recording: ClipRecording = self.unpublished_recording(contents);
            let path: PathBuf = self.repository.path().join(repository_key::audio_key(&recording.audio_sha256));

            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, contents).unwrap();

            recording
        }

        fn publish_recordings(&self, label: &str, count: usize) -> Vec<ClipRecording> {
            (0..count).map(|position| self.publish_recording(&format!("{label} {position}"))).collect()
        }

        fn unpublished_recording(&self, contents: &str) -> ClipRecording {
            ClipRecording {
                audio_sha256: content_address::sha256_hex(contents.as_bytes()),
                audio_base_url: self.repository.path().display().to_string(),
            }
        }

        fn recording_url(&self, recording: &ClipRecording) -> String {
            fetch::join_url(&recording.audio_base_url, &repository_key::audio_key(&recording.audio_sha256))
        }

        fn recording_urls(&self, recordings: &[ClipRecording]) -> Vec<String> {
            recordings.iter().map(|recording| self.recording_url(recording)).collect()
        }

        fn cache_recording(&self, recording: &ClipRecording, contents: &str) {
            let path: PathBuf = self.cache.recording_file_path(&recording.audio_sha256);

            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, contents).unwrap();
        }

        fn is_cached(&self, recording: &ClipRecording) -> bool {
            self.cached_file_names().contains(&format!("{}.{}", recording.audio_sha256, manifest::AUDIO_FILE_EXTENSION))
        }

        fn cached_file_names(&self) -> BTreeSet<String> {
            let audio_directory: PathBuf = self.cache_directory.path().join("audio");
            let Ok(entries) = std::fs::read_dir(audio_directory)
            else {
                return BTreeSet::new();
            };

            entries.map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect()
        }

        fn audio_sha256s_in_flight(&self) -> Vec<String> {
            self.cache.downloads_in_flight.iter().map(|entry| entry.key().clone()).collect()
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
                remaining_count: 0,
                end: PrefetchEndKind::Completed,
            },
        );
    }

    #[tokio::test]
    async fn a_play_of_a_recording_being_prefetched_shares_its_download() {
        for _ in 0..REPEAT_COUNT {
            let fixture: CacheFixture = CacheFixture::create();
            let recording: ClipRecording = fixture.publish_recording("shared");
            let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(false);
            let mut waiting_plays: watch::Receiver<WaitingPlays> = fixture.cache.subscribe_to_waiting_plays();
            let cancellation: Arc<PrefetchCancellation> = PrefetchCancellation::new();

            let prefetch = fixture.cache.prefetch(&http_fetch, vec![recording.clone()], &cancellation);
            let play = async {
                http_fetch.wait_for_request_count(1).await;

                fixture.cache.foreground_recording(&http_fetch, &recording).await
            };
            let release = async {
                waiting_plays.wait_for(|plays| !plays.is_empty()).await.unwrap();
                http_fetch.release();
            };
            let (summary, played, ()): (RecordingPrefetchSummary, Result<PathBuf, AppErrorStatic>, ()) = future::join3(prefetch, play, release).await;

            assert_eq!(http_fetch.requested_urls(), vec![fixture.recording_url(&recording)]);
            assert_eq!(std::fs::read(played.unwrap()).unwrap(), b"shared".to_vec());
            assert_eq!(summary.downloaded_count, 1);
            assert!(fixture.audio_sha256s_in_flight().is_empty());
        }
    }

    #[tokio::test]
    async fn prefetch_starts_no_download_while_a_play_waits_for_its_recording() {
        for _ in 0..REPEAT_COUNT {
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
    }

    #[tokio::test]
    async fn a_cancelled_prefetch_starts_no_further_downloads_and_finishes_those_running() {
        for _ in 0..REPEAT_COUNT {
            let fixture: CacheFixture = CacheFixture::create();
            let first: ClipRecording = fixture.publish_recording("first");
            let extra_recording_count: usize = 2;
            let running: Vec<ClipRecording> = fixture.publish_recordings("running", PREFETCH_CONCURRENCY);
            let extra: Vec<ClipRecording> = fixture.publish_recordings("extra", extra_recording_count);
            let recordings: Vec<ClipRecording> = [vec![first.clone()], running.clone(), extra.clone()].concat();
            let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(false);
            let cancellation: Arc<PrefetchCancellation> = PrefetchCancellation::new();

            http_fetch.release_url(&fixture.recording_url(&first));

            let prefetch = fixture.cache.prefetch(&http_fetch, recordings.clone(), &cancellation);
            let cancel = async {
                http_fetch.wait_for_held_urls(&fixture.recording_urls(&running)).await;
                cancellation.cancel();
                http_fetch.release();
            };
            let (summary, ()): (RecordingPrefetchSummary, ()) = future::join(prefetch, cancel).await;
            let cached_count: usize = recordings.iter().filter(|recording| fixture.is_cached(recording)).count();

            assert_eq!(http_fetch.requested_urls().len(), 1 + PREFETCH_CONCURRENCY);
            assert_eq!(cached_count, 1 + PREFETCH_CONCURRENCY);
            assert_eq!(
                summary,
                RecordingPrefetchSummary {
                    already_cached_count: 0,
                    downloaded_count: 1 + PREFETCH_CONCURRENCY,
                    failed_count: 0,
                    remaining_count: extra_recording_count,
                    end: PrefetchEndKind::Cancelled,
                },
            );
        }
    }

    #[tokio::test]
    async fn a_play_interrupts_the_prefetches_running_of_other_recordings_which_finish_once_after_it() {
        for _ in 0..REPEAT_COUNT {
            let fixture: CacheFixture = CacheFixture::create();
            let first: ClipRecording = fixture.publish_recording("first");
            let interrupted: Vec<ClipRecording> = fixture.publish_recordings("interrupted", PREFETCH_CONCURRENCY);
            let played: ClipRecording = fixture.publish_recording("played");
            let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(false);
            let cancellation: Arc<PrefetchCancellation> = PrefetchCancellation::new();
            let interrupted_urls: Vec<String> = fixture.recording_urls(&interrupted);
            let played_url: String = fixture.recording_url(&played);

            http_fetch.release_url(&fixture.recording_url(&first));

            let prefetch = fixture.cache.prefetch(&http_fetch, [vec![first.clone()], interrupted.clone()].concat(), &cancellation);
            let play_then_release = async {
                http_fetch.wait_for_held_urls(&interrupted_urls).await;

                let play = fixture.cache.foreground_recording(&http_fetch, &played);
                let observe_then_release_the_play = async {
                    http_fetch.wait_for_held_urls(std::slice::from_ref(&played_url)).await;

                    let in_flight_while_the_play_waited: Vec<String> = fixture.audio_sha256s_in_flight();
                    let request_count_while_the_play_waited: usize = http_fetch.requested_urls().len();

                    http_fetch.release_url(&played_url);

                    (in_flight_while_the_play_waited, request_count_while_the_play_waited)
                };
                let (played_path, observed): (Result<PathBuf, AppErrorStatic>, (Vec<String>, usize)) =
                    future::join(play, observe_then_release_the_play).await;

                http_fetch.release();

                (played_path, observed)
            };
            let (summary, (played_path, (in_flight_while_the_play_waited, request_count_while_the_play_waited))): (
                RecordingPrefetchSummary,
                (Result<PathBuf, AppErrorStatic>, (Vec<String>, usize)),
            ) = future::join(prefetch, play_then_release).await;
            let served_urls: Vec<String> = http_fetch.served_urls();
            let played_serve_position: Option<usize> = served_urls.iter().position(|url| *url == played_url);

            assert_eq!(std::fs::read(played_path.unwrap()).unwrap(), b"played".to_vec());
            assert_eq!(in_flight_while_the_play_waited, vec![played.audio_sha256.clone()]);
            assert_eq!(request_count_while_the_play_waited, 1 + PREFETCH_CONCURRENCY + 1);

            for interrupted_url in &interrupted_urls {
                assert_eq!(http_fetch.request_count_of(interrupted_url), 2);
                assert_eq!(http_fetch.serve_count_of(interrupted_url), 1);
                assert!(served_urls.iter().position(|url| url == interrupted_url) > played_serve_position);
            }

            assert_eq!(
                summary,
                RecordingPrefetchSummary {
                    already_cached_count: 0,
                    downloaded_count: 1 + PREFETCH_CONCURRENCY,
                    failed_count: 0,
                    remaining_count: 0,
                    end: PrefetchEndKind::Completed,
                },
            );
            assert_eq!(fixture.cached_file_names().len(), 1 + PREFETCH_CONCURRENCY + 1);
            assert!(fixture.audio_sha256s_in_flight().is_empty());
        }
    }

    #[tokio::test]
    async fn a_prefetch_of_the_played_recording_keeps_running_and_the_play_shares_it() {
        for _ in 0..REPEAT_COUNT {
            let fixture: CacheFixture = CacheFixture::create();
            let first: ClipRecording = fixture.publish_recording("first");
            let shared: ClipRecording = fixture.publish_recording("shared");
            let interrupted: Vec<ClipRecording> = fixture.publish_recordings("interrupted", PREFETCH_CONCURRENCY - 1);
            let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(false);
            let cancellation: Arc<PrefetchCancellation> = PrefetchCancellation::new();
            let interrupted_urls: Vec<String> = fixture.recording_urls(&interrupted);
            let shared_url: String = fixture.recording_url(&shared);

            http_fetch.release_url(&fixture.recording_url(&first));

            let prefetch = fixture.cache.prefetch(&http_fetch, [vec![first.clone(), shared.clone()], interrupted.clone()].concat(), &cancellation);
            let play_then_release = async {
                http_fetch.wait_for_held_urls(&[vec![shared_url.clone()], interrupted_urls.clone()].concat()).await;

                let play = fixture.cache.foreground_recording(&http_fetch, &shared);
                let observe_then_release_the_play = async {
                    http_fetch.wait_for_held_urls(std::slice::from_ref(&shared_url)).await;

                    let request_count_while_the_play_waited: usize = http_fetch.requested_urls().len();

                    http_fetch.release_url(&shared_url);

                    request_count_while_the_play_waited
                };
                let (played_path, request_count_while_the_play_waited): (Result<PathBuf, AppErrorStatic>, usize) =
                    future::join(play, observe_then_release_the_play).await;

                http_fetch.release();

                (played_path, request_count_while_the_play_waited)
            };
            let (summary, (played_path, request_count_while_the_play_waited)): (RecordingPrefetchSummary, (Result<PathBuf, AppErrorStatic>, usize)) =
                future::join(prefetch, play_then_release).await;

            assert_eq!(std::fs::read(played_path.unwrap()).unwrap(), b"shared".to_vec());
            assert_eq!(request_count_while_the_play_waited, 1 + PREFETCH_CONCURRENCY);
            assert_eq!(http_fetch.request_count_of(&shared_url), 1);
            assert_eq!(http_fetch.serve_count_of(&shared_url), 1);

            for interrupted_url in &interrupted_urls {
                assert_eq!(http_fetch.request_count_of(interrupted_url), 2);
                assert_eq!(http_fetch.serve_count_of(interrupted_url), 1);
            }

            assert_eq!(summary.downloaded_count, 1 + PREFETCH_CONCURRENCY);
            assert_eq!(summary.end, PrefetchEndKind::Completed);
            assert!(fixture.audio_sha256s_in_flight().is_empty());
        }
    }

    #[tokio::test]
    async fn prefetch_stops_at_the_first_connection_failure() {
        let fixture: CacheFixture = CacheFixture::create();
        let recordings: Vec<ClipRecording> = fixture.publish_recordings("unreachable", PREFETCH_CONCURRENCY + 1);
        let http_fetch: UnreachableFetch = UnreachableFetch {
            requested_urls: watch::Sender::new(Vec::new()),
        };

        let summary: RecordingPrefetchSummary = fixture.cache.prefetch(&http_fetch, recordings.clone(), &PrefetchCancellation::new()).await;

        assert_eq!(*http_fetch.requested_urls.borrow(), vec![fixture.recording_url(&recordings[0])]);
        assert_eq!(
            summary,
            RecordingPrefetchSummary {
                already_cached_count: 0,
                downloaded_count: 0,
                failed_count: 1,
                remaining_count: PREFETCH_CONCURRENCY,
                end: PrefetchEndKind::ConnectionFailed,
            },
        );
        assert!(fixture.cached_file_names().is_empty());
    }

    #[tokio::test]
    async fn prefetch_skips_a_missing_recording_and_downloads_the_rest() {
        let fixture: CacheFixture = CacheFixture::create();
        let missing: ClipRecording = fixture.unpublished_recording("missing");
        let present: Vec<ClipRecording> = fixture.publish_recordings("present", PREFETCH_CONCURRENCY + 1);
        let http_fetch: GatedDirectoryFetch = GatedDirectoryFetch::create(true);

        let summary: RecordingPrefetchSummary =
            fixture.cache.prefetch(&http_fetch, [vec![missing.clone()], present.clone()].concat(), &PrefetchCancellation::new()).await;

        assert_eq!(sorted(http_fetch.requested_urls()), sorted(fixture.recording_urls(&[vec![missing.clone()], present.clone()].concat())));
        assert!(present.iter().all(|recording| fixture.is_cached(recording)));
        assert!(!fixture.is_cached(&missing));
        assert_eq!(
            summary,
            RecordingPrefetchSummary {
                already_cached_count: 0,
                downloaded_count: PREFETCH_CONCURRENCY + 1,
                failed_count: 1,
                remaining_count: 0,
                end: PrefetchEndKind::Completed,
            },
        );
    }
}
