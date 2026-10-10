use shared::course::SpokenClip;
use shared::progress::AudioEngineKind;

/// One started prefetch, for cancelling it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordingPrefetchId(u64);

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
pub use browser::*;
#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
pub use server::*;

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
mod browser {
    use std::cell::{Cell, RefCell};
    use std::collections::{BTreeSet, VecDeque};

    use js_sys::{Function, Promise, Reflect};
    use leptos::task::spawn_local;
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{AbortController, AbortSignal, Blob, RequestInit, RequestMode, Response, Window};

    use super::*;
    use crate::artifact_client::js_interop;
    use crate::platform::{self, AudioClip};

    const RECORDING_PREFETCH_CONCURRENCY: usize = 2;
    const PRESSED_PLAY_RECHECK_MILLISECONDS: i32 = 100;
    const FETCH_PRIORITY_PROPERTY: &str = "priority";
    const LOW_FETCH_PRIORITY: &str = "low";

    struct PrefetchRun {
        prefetch_id: RecordingPrefetchId,
        queued_urls: VecDeque<String>,
        downloads_in_flight: Vec<DownloadInFlight>,
        next_download_id: u64,
        repository_has_answered: bool,
    }

    struct DownloadInFlight {
        download_id: u64,
        url: String,
        abort_controller: AbortController,
        /// Resolves each play waiting for this download with its `Blob`, or with null when it fails.
        waiting_plays: Vec<Function>,
    }

    struct StartedDownload {
        download_id: u64,
        url: String,
        signal: AbortSignal,
    }

    enum RecordingDownloadFailure {
        Aborted,
        /// No answer from the repository: no network, a DNS failure, a refused or timed-out connection.
        ConnectionFailed(String),
        /// The repository answered with this HTTP status instead of the recording.
        RepositoryRefused(u16),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum PrefetchStepKind {
        Continue,
        Stop,
    }

    /// A background download a play takes over.
    pub struct InFlightRecording(Promise);

    impl InFlightRecording {
        /// `None` when the download fails or is cancelled.
        pub async fn recording(self) -> Option<Blob> {
            let settled: Result<JsValue, JsValue> = JsFuture::from(self.0).await;

            settled.ok().and_then(|value| value.dyn_into::<Blob>().ok())
        }
    }

    thread_local! {
        static NEXT_PREFETCH_ID: Cell<u64> = const { Cell::new(0) };
        static ACTIVE_RUN: RefCell<Option<PrefetchRun>> = const { RefCell::new(None) };
        static PREFETCHED_RECORDING_URLS: RefCell<BTreeSet<String>> = const { RefCell::new(BTreeSet::new()) };
    }

    /// Downloads into the browser's cache each recording `clips` would play with `engine`, once per page, and
    /// cancels the prefetch started before it. No download starts while a play waits for its recording.
    pub fn start_prefetch(clips: Vec<SpokenClip>, engine: AudioEngineKind) -> RecordingPrefetchId {
        cancel_active_prefetch();

        let prefetch_id: RecordingPrefetchId = next_prefetch_id();
        let queued_urls: VecDeque<String> = recording_urls_to_fetch(clips, engine);

        ACTIVE_RUN.with(|active| {
            *active.borrow_mut() = Some(PrefetchRun {
                prefetch_id,
                queued_urls,
                downloads_in_flight: Vec::new(),
                next_download_id: 0,
                repository_has_answered: false,
            });
        });

        // One download until the repository first answers, so an unreachable one costs a single request.
        spawn_local(download_queued_recordings(prefetch_id));

        prefetch_id
    }

    /// Stops `prefetch_id` and aborts its downloads in flight, unless a later prefetch replaced it.
    pub fn cancel_prefetch(prefetch_id: RecordingPrefetchId) {
        if is_active(prefetch_id) {
            cancel_active_prefetch();
        }
    }

    /// Cancels the background downloads in flight other than one of `recording_url` and puts them back at the front of
    /// the queue, which no longer holds `recording_url`. Returns the download of `recording_url` when one is in flight.
    pub fn give_way_to_play(recording_url: &str) -> Option<InFlightRecording> {
        let yielded_downloads: Vec<DownloadInFlight> = ACTIVE_RUN.with(|active| {
            let mut active = active.borrow_mut();
            let Some(run) = active.as_mut()
            else {
                return Vec::new();
            };
            let (kept_downloads, yielded_downloads): (Vec<DownloadInFlight>, Vec<DownloadInFlight>) =
                run.downloads_in_flight.drain(..).partition(|download| download.url == recording_url);

            run.downloads_in_flight = kept_downloads;
            run.queued_urls.retain(|queued_url| queued_url != recording_url);

            for download in yielded_downloads.iter().rev() {
                run.queued_urls.push_front(download.url.clone());
            }

            yielded_downloads
        });

        for download in yielded_downloads {
            abort_download(download);
        }

        wait_for_download_in_flight(recording_url)
    }

    fn next_prefetch_id() -> RecordingPrefetchId {
        NEXT_PREFETCH_ID.with(|next| {
            let prefetch_id: u64 = next.get();
            next.set(prefetch_id + 1);

            RecordingPrefetchId(prefetch_id)
        })
    }

    fn cancel_active_prefetch() {
        let active_run: Option<PrefetchRun> = ACTIVE_RUN.with(|active| active.borrow_mut().take());
        let Some(active_run) = active_run
        else {
            return;
        };

        for download in active_run.downloads_in_flight {
            abort_download(download);
        }
    }

    fn abort_download(download: DownloadInFlight) {
        download.abort_controller.abort();
        settle_waiting_plays(download, None);
    }

    fn settle_waiting_plays(download: DownloadInFlight, recording: Option<&Blob>) {
        let settled_value: JsValue = recording.map_or(JsValue::NULL, JsValue::from);

        for resolve in download.waiting_plays {
            _ = resolve.call1(&JsValue::NULL, &settled_value);
        }
    }

    fn wait_for_download_in_flight(recording_url: &str) -> Option<InFlightRecording> {
        ACTIVE_RUN.with(|active| {
            let mut active = active.borrow_mut();
            let download: &mut DownloadInFlight = active
                .as_mut()?
                .downloads_in_flight
                .iter_mut()
                .find(|download| download.url == recording_url)?;
            let mut captured_resolve: Option<Function> = None;
            let settled: Promise = Promise::new(&mut |resolve: Function, _reject: Function| captured_resolve = Some(resolve));

            download.waiting_plays.extend(captured_resolve);

            Some(InFlightRecording(settled))
        })
    }

    fn is_active(prefetch_id: RecordingPrefetchId) -> bool {
        ACTIVE_RUN.with(|active| active.borrow().as_ref().is_some_and(|run| run.prefetch_id == prefetch_id))
    }

    fn is_prefetched(url: &str) -> bool {
        PREFETCHED_RECORDING_URLS.with(|prefetched| prefetched.borrow().contains(url))
    }

    fn record_prefetched(url: String) {
        PREFETCHED_RECORDING_URLS.with(|prefetched| prefetched.borrow_mut().insert(url));
    }

    fn recording_urls_to_fetch(clips: Vec<SpokenClip>, engine: AudioEngineKind) -> VecDeque<String> {
        let mut seen_urls: BTreeSet<String> = BTreeSet::new();
        let mut queued_urls: VecDeque<String> = VecDeque::new();

        for clip in clips {
            let url: Option<String> = platform::recording_url(&AudioClip::from(clip), engine);
            let Some(url) = url
            else {
                continue;
            };
            let first_sighting: bool = seen_urls.insert(url.clone());

            if first_sighting && !is_prefetched(&url) {
                queued_urls.push_back(url);
            }
        }

        queued_urls
    }

    async fn download_queued_recordings(prefetch_id: RecordingPrefetchId) {
        loop {
            wait_while_a_play_waits_for_its_recording().await;

            let started_download: Option<StartedDownload> = start_next_download(prefetch_id);
            let Some(started_download) = started_download
            else {
                return;
            };

            let downloaded: Result<Blob, RecordingDownloadFailure> =
                download_recording(&started_download.url, &started_download.signal).await;
            let step: PrefetchStepKind = finish_download(prefetch_id, started_download, downloaded);

            if step == PrefetchStepKind::Stop {
                return;
            }
        }
    }

    async fn wait_while_a_play_waits_for_its_recording() {
        while platform::is_recording_play_waiting() {
            sleep_milliseconds(PRESSED_PLAY_RECHECK_MILLISECONDS).await;
        }
    }

    async fn sleep_milliseconds(milliseconds: i32) {
        let elapsed: Promise = Promise::new(&mut |resolve: Function, _reject: Function| {
            let scheduled: Option<i32> = web_sys::window()
                .and_then(|window| window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, milliseconds).ok());

            if scheduled.is_none() {
                _ = resolve.call0(&JsValue::NULL);
            }
        });

        _ = JsFuture::from(elapsed).await;
    }

    fn start_next_download(prefetch_id: RecordingPrefetchId) -> Option<StartedDownload> {
        let created: Result<AbortController, JsValue> = AbortController::new();
        let Ok(abort_controller) = created
        else {
            log::warn!("could not create an abort controller for the recording prefetch");
            return None;
        };

        ACTIVE_RUN.with(|active| {
            let mut active = active.borrow_mut();
            let run: &mut PrefetchRun = active.as_mut().filter(|run| run.prefetch_id == prefetch_id)?;
            let url: String = run.queued_urls.pop_front()?;
            let download_id: u64 = run.next_download_id;
            let signal: AbortSignal = abort_controller.signal();

            run.next_download_id += 1;
            run.downloads_in_flight.push(DownloadInFlight {
                download_id,
                url: url.clone(),
                abort_controller,
                waiting_plays: Vec::new(),
            });

            Some(StartedDownload {
                download_id,
                url,
                signal,
            })
        })
    }

    fn finish_download(
        prefetch_id: RecordingPrefetchId,
        started_download: StartedDownload,
        downloaded: Result<Blob, RecordingDownloadFailure>,
    ) -> PrefetchStepKind {
        let finished_download: Option<DownloadInFlight> = take_download_in_flight(prefetch_id, started_download.download_id);

        if let Some(finished_download) = finished_download {
            settle_waiting_plays(finished_download, downloaded.as_ref().ok());
        }

        match downloaded {
            Ok(_) => {
                // A play may have requeued this download after its body arrived.
                forget_queued_url(prefetch_id, &started_download.url);
                record_prefetched(started_download.url);
                note_repository_answered(prefetch_id);
            },
            Err(RecordingDownloadFailure::Aborted) => {},
            Err(RecordingDownloadFailure::RepositoryRefused(status)) => {
                log::debug!("skipped a recording the repository refused; [url={} status={status}]", started_download.url);
                note_repository_answered(prefetch_id);
            },
            Err(RecordingDownloadFailure::ConnectionFailed(message)) => {
                stop_unreachable_prefetch(prefetch_id, &started_download.url, &message);
                return PrefetchStepKind::Stop;
            },
        }

        if is_active(prefetch_id) { PrefetchStepKind::Continue } else { PrefetchStepKind::Stop }
    }

    fn take_download_in_flight(prefetch_id: RecordingPrefetchId, download_id: u64) -> Option<DownloadInFlight> {
        ACTIVE_RUN.with(|active| {
            let mut active = active.borrow_mut();
            let run: &mut PrefetchRun = active.as_mut().filter(|run| run.prefetch_id == prefetch_id)?;
            let position: usize = run.downloads_in_flight.iter().position(|download| download.download_id == download_id)?;

            Some(run.downloads_in_flight.remove(position))
        })
    }

    fn forget_queued_url(prefetch_id: RecordingPrefetchId, url: &str) {
        ACTIVE_RUN.with(|active| {
            if let Some(run) = active.borrow_mut().as_mut().filter(|run| run.prefetch_id == prefetch_id) {
                run.queued_urls.retain(|queued_url| queued_url != url);
            }
        });
    }

    fn note_repository_answered(prefetch_id: RecordingPrefetchId) {
        let first_answer: bool = ACTIVE_RUN.with(|active| {
            let mut active = active.borrow_mut();
            let Some(run) = active.as_mut().filter(|run| run.prefetch_id == prefetch_id)
            else {
                return false;
            };
            let first_answer: bool = !run.repository_has_answered;

            run.repository_has_answered = true;

            first_answer
        });

        if first_answer {
            for _ in 1..RECORDING_PREFETCH_CONCURRENCY {
                spawn_local(download_queued_recordings(prefetch_id));
            }
        }
    }

    // Every other download of the run would fail the same way, so the run ends with one warning.
    fn stop_unreachable_prefetch(prefetch_id: RecordingPrefetchId, url: &str, message: &str) {
        if !is_active(prefetch_id) {
            return;
        }

        log::warn!("stopped prefetching recordings, the repository is unreachable; [url={url} error={message}]");
        cancel_active_prefetch();
    }

    /* A CORS request, as the play's audio element makes, since the repository varies recordings on Origin. Complete
       only once the whole body is read. */
    async fn download_recording(url: &str, signal: &AbortSignal) -> Result<Blob, RecordingDownloadFailure> {
        let window: Window = js_interop::get_window().map_err(|error| RecordingDownloadFailure::ConnectionFailed(error.to_string()))?;
        let init: RequestInit = RequestInit::new();

        init.set_method("GET");
        init.set_mode(RequestMode::Cors);
        init.set_signal(Some(signal));
        // web-sys has no setter for a request's fetch priority.
        _ = Reflect::set(&init, &JsValue::from_str(FETCH_PRIORITY_PROPERTY), &JsValue::from_str(LOW_FETCH_PRIORITY));

        let answered: Result<JsValue, JsValue> = JsFuture::from(window.fetch_with_str_and_init(url, &init)).await;
        let response: Response = answered
            .and_then(|value| value.dyn_into::<Response>())
            .map_err(|error| download_failure(&error, signal))?;

        if !response.ok() {
            return Err(RecordingDownloadFailure::RepositoryRefused(response.status()));
        }

        let body: Promise = response.blob().map_err(|error| download_failure(&error, signal))?;
        let read: Result<JsValue, JsValue> = JsFuture::from(body).await;

        read.and_then(|value| value.dyn_into::<Blob>()).map_err(|error| download_failure(&error, signal))
    }

    fn download_failure(error: &JsValue, signal: &AbortSignal) -> RecordingDownloadFailure {
        if signal.aborted() {
            return RecordingDownloadFailure::Aborted;
        }

        RecordingDownloadFailure::ConnectionFailed(js_interop::error_message(error))
    }
}

#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
mod server {
    use super::*;

    pub fn start_prefetch(_clips: Vec<SpokenClip>, _engine: AudioEngineKind) -> RecordingPrefetchId {
        RecordingPrefetchId(0)
    }

    pub fn cancel_prefetch(_prefetch_id: RecordingPrefetchId) {}
}
