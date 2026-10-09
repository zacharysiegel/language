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
    use std::rc::Rc;

    use js_sys::{Function, Promise, Reflect};
    use leptos::task::spawn_local;
    use shared::{AppError, AppErrorStatic};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{AbortController, AbortSignal, RequestInit, RequestMode, Response, Window};

    use super::*;
    use crate::artifact_client::js_interop;
    use crate::platform::{self, AudioClip};

    const RECORDING_PREFETCH_CONCURRENCY: usize = 2;
    const PRESSED_PLAY_RECHECK_MILLISECONDS: i32 = 100;
    const FETCH_PRIORITY_PROPERTY: &str = "priority";
    const LOW_FETCH_PRIORITY: &str = "low";

    struct ActivePrefetch {
        prefetch_id: RecordingPrefetchId,
        abort_controller: AbortController,
    }

    thread_local! {
        static NEXT_PREFETCH_ID: Cell<u64> = const { Cell::new(0) };
        static ACTIVE_PREFETCH: RefCell<Option<ActivePrefetch>> = const { RefCell::new(None) };
        static PREFETCHED_RECORDING_URLS: RefCell<BTreeSet<String>> = const { RefCell::new(BTreeSet::new()) };
    }

    /// Downloads into the browser's cache each recording `clips` would play with `engine`, once per page, and
    /// cancels the prefetch started before it. No download starts while a play waits for its recording.
    pub fn start_prefetch(clips: Vec<SpokenClip>, engine: AudioEngineKind) -> RecordingPrefetchId {
        cancel_active_prefetch();

        let prefetch_id: RecordingPrefetchId = next_prefetch_id();
        let created: Result<AbortController, JsValue> = AbortController::new();
        let Ok(abort_controller) = created
        else {
            log::warn!("could not create an abort controller for the recording prefetch");
            return prefetch_id;
        };

        let signal: AbortSignal = abort_controller.signal();
        let queued_urls: Rc<RefCell<VecDeque<String>>> = Rc::new(RefCell::new(recording_urls_to_fetch(clips, engine)));

        ACTIVE_PREFETCH.with(|active| {
            *active.borrow_mut() = Some(ActivePrefetch {
                prefetch_id,
                abort_controller,
            });
        });

        for _ in 0..RECORDING_PREFETCH_CONCURRENCY {
            spawn_local(fetch_queued_recordings(prefetch_id, queued_urls.clone(), signal.clone()));
        }

        prefetch_id
    }

    /// Stops `prefetch_id` and aborts its downloads in flight, unless a later prefetch replaced it.
    pub fn cancel_prefetch(prefetch_id: RecordingPrefetchId) {
        if is_active(prefetch_id) {
            cancel_active_prefetch();
        }
    }

    fn next_prefetch_id() -> RecordingPrefetchId {
        NEXT_PREFETCH_ID.with(|next| {
            let prefetch_id: u64 = next.get();
            next.set(prefetch_id + 1);

            RecordingPrefetchId(prefetch_id)
        })
    }

    fn cancel_active_prefetch() {
        let active_prefetch: Option<ActivePrefetch> = ACTIVE_PREFETCH.with(|active| active.borrow_mut().take());

        if let Some(active_prefetch) = active_prefetch {
            active_prefetch.abort_controller.abort();
        }
    }

    fn is_active(prefetch_id: RecordingPrefetchId) -> bool {
        ACTIVE_PREFETCH.with(|active| active.borrow().as_ref().is_some_and(|active| active.prefetch_id == prefetch_id))
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

    async fn fetch_queued_recordings(
        prefetch_id: RecordingPrefetchId,
        queued_urls: Rc<RefCell<VecDeque<String>>>,
        signal: AbortSignal,
    ) {
        loop {
            wait_while_a_play_waits_for_its_recording().await;

            if !is_active(prefetch_id) {
                return;
            }

            let next_url: Option<String> = queued_urls.borrow_mut().pop_front();
            let Some(url) = next_url
            else {
                return;
            };

            let fetched: Result<(), AppErrorStatic> = fetch_recording(&url, &signal).await;

            match fetched {
                Ok(()) => record_prefetched(url),
                Err(_) if !is_active(prefetch_id) => return,
                Err(error) => log::warn!("could not prefetch a recording; [url={url} error={error}]"),
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

    /* A CORS request, as the play's audio element makes, since the repository varies recordings on Origin. Complete
       only once the whole body is read. */
    async fn fetch_recording(url: &str, signal: &AbortSignal) -> Result<(), AppErrorStatic> {
        let window: Window = js_interop::get_window()?;
        let init: RequestInit = RequestInit::new();

        init.set_method("GET");
        init.set_mode(RequestMode::Cors);
        init.set_signal(Some(signal));
        // web-sys has no setter for a request's fetch priority.
        Reflect::set(&init, &JsValue::from_str(FETCH_PRIORITY_PROPERTY), &JsValue::from_str(LOW_FETCH_PRIORITY)).map_err(js_interop::error)?;

        let response: Response = js_interop::await_and_cast(window.fetch_with_str_and_init(url, &init)).await?;

        if !response.ok() {
            return Err(AppError::from(format!("the repository answered {}", response.status())).into());
        }

        let body: Promise = response.array_buffer().map_err(js_interop::error)?;

        JsFuture::from(body).await.map_err(js_interop::error)?;

        Ok(())
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
