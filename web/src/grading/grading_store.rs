use leptos::prelude::*;
use shared::grading::GraderPromptKind;

use crate::grading::GraderStateKind;
use crate::progress_state::{ProgressStore, use_progress_store};

/// How the grader reads a practice session's open answers.
#[derive(Debug, Clone)]
pub struct OpenAnswerGradingContext {
    pub grader_prompt: GraderPromptKind,
    pub level: String,
    pub target_language_name: String,
}

#[derive(Debug, Clone)]
pub struct OpenAnswerSubmission {
    pub grader_prompt: GraderPromptKind,
    pub level: String,
    pub prompt_source: String,
    pub context_source: Option<String>,
    pub answer: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradingFailureKind {
    ModelNotLoaded,
    Cancelled,
    Failed,
}

/// The grading model's state in this page, which outlives every route.
#[derive(Clone, Copy)]
pub struct GradingStore {
    state: RwSignal<GraderStateKind>,
}

impl GradingStore {
    pub fn state(self) -> GraderStateKind {
        self.state.get()
    }

    pub fn state_untracked(self) -> GraderStateKind {
        self.state.get_untracked()
    }
}

pub fn provide_grading_store() {
    let grading_store: GradingStore = GradingStore {
        state: RwSignal::new(GraderStateKind::Unchecked),
    };
    let progress_store: ProgressStore = use_progress_store();

    Effect::new(move |_| {
        let download_enabled: bool = progress_store.read(|progress| progress.settings.download_grading_model);

        if !download_enabled {
            grading_store.stop_download();
        }
    });

    provide_context(grading_store);
}

pub fn use_grading_store() -> GradingStore {
    expect_context::<GradingStore>()
}

#[cfg(feature = "hydrate")] // runs the model in the browser, which the server build has none of
mod browser {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use cross_llama::{
        BrowserCapability,
        ChatMessage,
        DownloadProgress,
        Generation,
        GenerationError,
        GenerationRequest,
        LoadOptions,
        LoadProgress,
        Model,
        ModelSource,
        Role,
        SlidingWindowCache,
    };
    use futures_util::future::{AbortHandle, AbortRegistration, Abortable, Aborted};
    use leptos::task::spawn_local;
    use shared::AppErrorStatic;
    use shared::artifact::fetch;
    use shared::grading::{self, ChatRoleKind, GradedAnswer, GraderResponse};
    use wasm_bindgen_futures::JsFuture;

    use super::*;
    use crate::grading::web_lock::{self, HeldWebLock};
    use crate::grading::{self as web_grading, BrowserCapabilityFacts, ModelDownloadProgress};
    use crate::platform;

    const WLLAMA_DIRECTORY_URL: &str = "/wllama/";
    const CONTEXT_TOKEN_COUNT: u32 = 4096;
    const GRADING_TEMPERATURE: f32 = 0.0;
    const GRADING_SEED: u32 = 0;
    const MAXIMUM_REPLY_TOKEN_COUNT: usize = 1024;
    // The warm-up reads the prompt for its cache; its reply is thrown away.
    const WARM_UP_REPLY_TOKEN_COUNT: usize = 1;
    // Holds the SHA-256 of the model that failed, so a different model is tried again.
    const FAILED_LOAD_STORAGE_KEY: &str = "language.grading.failed-load-model-sha256";
    // Held across this site's pages while one loads or holds a grading model, so only one copy is in memory.
    const MODEL_IN_MEMORY_LOCK_NAME: &str = "language grading model in memory";

    thread_local! {
        // Set by the first unit page that prepares grading.
        static MODEL_URL: RefCell<Option<String>> = const { RefCell::new(None) };
        static CAPABILITY_CHECK_STARTED: Cell<bool> = const { Cell::new(false) };
        // Keeps a repository without the model from being asked again on every unit page.
        static DOWNLOAD_FAILED: Cell<bool> = const { Cell::new(false) };
        static LOADED_MODEL: RefCell<Option<Rc<Model>>> = const { RefCell::new(None) };
        static RUNNING_DOWNLOAD: RefCell<Option<AbortHandle>> = const { RefCell::new(None) };
        static RUNNING_LOAD: RefCell<Option<AbortHandle>> = const { RefCell::new(None) };
        static MODEL_IN_MEMORY_LOCK: RefCell<Option<HeldWebLock>> = const { RefCell::new(None) };
    }

    impl GradingStore {
        /// Checks this browser and starts the background download when `download_enabled`; reads the grader state tracked.
        pub fn prepare(self, repository_base_url: &str, download_enabled: bool) {
            remember_model_url(repository_base_url);

            let check_started: bool = CAPABILITY_CHECK_STARTED.with(|started| started.replace(true));
            let download_failed: bool = DOWNLOAD_FAILED.with(Cell::get);

            match self.state() {
                GraderStateKind::Unchecked if !check_started => spawn_local(self.check_capability()),
                GraderStateKind::NotDownloaded if download_enabled && !download_failed => self.start_download(),
                _ => {},
            }
        }

        pub fn stop_download(self) {
            let running_download: Option<AbortHandle> = RUNNING_DOWNLOAD.with(|running| running.borrow_mut().take());

            let Some(running_download) = running_download
            else {
                return;
            };

            running_download.abort();
            self.set_state(GraderStateKind::NotDownloaded);
            log::info!("stopped the grading model's download");
        }

        /// Loads a downloaded model into memory, unless another page of this site holds one.
        pub fn load_model(self) {
            let loadable: bool = matches!(self.state_untracked(), GraderStateKind::Downloaded | GraderStateKind::InUseInAnotherPage);

            if !loadable {
                return;
            }

            let Some(model_source) = model_source()
            else {
                return;
            };

            let (abort_handle, abort_registration): (AbortHandle, AbortRegistration) = AbortHandle::new_pair();

            RUNNING_LOAD.with(|running| running.replace(Some(abort_handle.clone())));
            self.set_state(GraderStateKind::Loading);
            spawn_local(self.load(model_source, abort_handle, abort_registration));
        }

        /// Reads the grader's prompt for `submission` into the model's cache.
        pub fn warm_up(self, submission: OpenAnswerSubmission) {
            let Some(model) = loaded_model()
            else {
                return;
            };

            spawn_local(async move {
                let request: GenerationRequest = create_generation_request(&submission, None, WARM_UP_REPLY_TOKEN_COUNT);
                let warmed: Result<Generation, GenerationError> = model.generate(&request, |_| {}).await;

                match warmed {
                    Ok(generation) => log::debug!(
                        "warmed the grader's prompt; [prompt_tokens={} reused_tokens={} seconds={:.1}]",
                        generation.prompt_token_count,
                        generation.reused_token_count,
                        generation.prompt_reading_duration.as_secs_f64(),
                    ),
                    Err(error) => log::debug!("did not warm the grader's prompt; [error={error}]"),
                }
            });
        }

        pub async fn grade(self, submission: OpenAnswerSubmission) -> Result<GraderResponse, GradingFailureKind> {
            let model: Rc<Model> = loaded_model().ok_or(GradingFailureKind::ModelNotLoaded)?;
            let response_schema: serde_json::Value = submission.grader_prompt.response_schema();
            let request: GenerationRequest = create_generation_request(&submission, Some(response_schema), MAXIMUM_REPLY_TOKEN_COUNT);

            let generated: Result<Generation, GenerationError> = model.generate(&request, |_| {}).await;

            let generation: Generation = match generated {
                Ok(generation) => generation,
                Err(GenerationError::Cancelled) => return Err(GradingFailureKind::Cancelled),
                Err(GenerationError::Failed(error)) => {
                    log::warn!("could not grade an open answer; [error={error}]");
                    return Err(GradingFailureKind::Failed);
                },
            };

            log::info!(
                "graded an open answer; [prompt_tokens={} reused_tokens={} generated_tokens={} reading_seconds={:.1} generating_seconds={:.1}]",
                generation.prompt_token_count,
                generation.reused_token_count,
                generation.generated_token_count,
                generation.prompt_reading_duration.as_secs_f64(),
                generation.generating_duration.as_secs_f64(),
            );

            let parsed: Result<GraderResponse, serde_json::Error> = grading::parse_grader_response(&generation.text);

            match parsed {
                Ok(response) => Ok(web_grading::applicable_response(&submission.answer, response)),
                Err(error) => {
                    log::warn!("the grader's reply is not a grading; [error={error}]");
                    Err(GradingFailureKind::Failed)
                },
            }
        }

        /// Frees the model, cancelling a load or a generation still under way.
        pub fn unload(self) {
            let running_load: Option<AbortHandle> = RUNNING_LOAD.with(|running| running.borrow_mut().take());
            let loaded: Option<Rc<Model>> = LOADED_MODEL.with(|model| model.borrow_mut().take());
            let model_in_memory_lock: Option<HeldWebLock> = MODEL_IN_MEMORY_LOCK.with(|lock| lock.borrow_mut().take());
            let held_in_memory: bool = matches!(self.state_untracked(), GraderStateKind::Loading | GraderStateKind::Ready);

            if let Some(running_load) = running_load {
                running_load.abort();
            }

            match loaded {
                Some(model) => {
                    model.cancel();
                    spawn_local(unload_once_released(model, model_in_memory_lock));
                },
                None => {
                    if let Some(model_in_memory_lock) = model_in_memory_lock {
                        model_in_memory_lock.release();
                    }
                },
            }

            if held_in_memory {
                self.set_state(GraderStateKind::Downloaded);
                log::info!("unloaded the grading model");
            }
        }

        fn set_state(self, state: GraderStateKind) {
            self.state.set(state);
        }

        async fn check_capability(self) {
            let Some(model_source) = model_source()
            else {
                return;
            };

            let assessed_state: GraderStateKind = assess_this_browser(&model_source).await;

            log::info!("checked whether this browser can grade; [state={assessed_state:?}]");
            self.set_state(assessed_state);
        }

        fn start_download(self) {
            let Some(model_source) = model_source()
            else {
                return;
            };

            let (abort_handle, abort_registration): (AbortHandle, AbortRegistration) = AbortHandle::new_pair();

            RUNNING_DOWNLOAD.with(|running| running.replace(Some(abort_handle)));
            self.set_state(GraderStateKind::Downloading(ModelDownloadProgress::NOT_STARTED));
            spawn_local(self.download(model_source, abort_registration));
        }

        async fn download(self, model_source: ModelSource, abort_registration: AbortRegistration) {
            request_persistent_storage().await;

            let downloading = self.download_once(&model_source);
            let downloaded: Result<Result<(), cross_llama::Error>, Aborted> = Abortable::new(downloading, abort_registration).await;

            // An aborted download was stopped by `stop_download`, which has already moved the state on.
            let Ok(downloaded) = downloaded
            else {
                return;
            };

            RUNNING_DOWNLOAD.with(|running| running.replace(None));

            match downloaded {
                Ok(()) => {
                    log::info!("downloaded the grading model");
                    self.set_state(GraderStateKind::Downloaded);
                },
                Err(error) => {
                    log::warn!("could not download the grading model; [error={error}]");
                    DOWNLOAD_FAILED.with(|failed| failed.set(true));
                    self.set_state(GraderStateKind::NotDownloaded);
                },
            }
        }

        async fn download_once(self, model_source: &ModelSource) -> Result<(), cross_llama::Error> {
            log::info!("downloading the grading model; [url={}]", model_source.model_url);

            cross_llama::download_model(model_source, move |progress: DownloadProgress| self.show_download_progress(progress)).await
        }

        fn show_download_progress(self, progress: DownloadProgress) {
            let reported_progress: ModelDownloadProgress = ModelDownloadProgress {
                downloaded_byte_count: progress.downloaded_byte_count,
                total_byte_count: progress.total_byte_count,
            };
            let GraderStateKind::Downloading(shown_progress) = self.state_untracked()
            else {
                return;
            };

            let progress_changed: bool = shown_progress.downloaded_permille() != reported_progress.downloaded_permille()
                || shown_progress.total_byte_count != reported_progress.total_byte_count;

            if progress_changed {
                self.set_state(GraderStateKind::Downloading(reported_progress));
            }
        }

        async fn load(self, model_source: ModelSource, abort_handle: AbortHandle, abort_registration: AbortRegistration) {
            let Some(model_in_memory_lock) = self.take_model_in_memory_lock().await
            else {
                return;
            };

            // The lock request cannot be abandoned once made, so a load stopped meanwhile releases what it was granted.
            if abort_handle.is_aborted() {
                model_in_memory_lock.release();
                return;
            }

            MODEL_IN_MEMORY_LOCK.with(|lock| lock.replace(Some(model_in_memory_lock)));

            let load_options: LoadOptions = LoadOptions {
                context_token_count: CONTEXT_TOKEN_COUNT,
                sliding_window_cache: SlidingWindowCache::Full,
            };
            let reached_wllama_load_step: Cell<bool> = Cell::new(false);
            let loading = Model::load(model_source, &load_options, |progress: LoadProgress| {
                if matches!(progress, LoadProgress::Loading { .. }) {
                    reached_wllama_load_step.set(true);
                }
            });
            let loaded: Result<Result<Model, cross_llama::Error>, Aborted> = Abortable::new(loading, abort_registration).await;

            // An aborted load was stopped by `unload`, which has already moved the state on and released the lock.
            let Ok(loaded) = loaded
            else {
                return;
            };

            RUNNING_LOAD.with(|running| running.replace(None));

            match loaded {
                Ok(model) => {
                    LOADED_MODEL.with(|loaded_model| loaded_model.replace(Some(Rc::new(model))));
                    self.set_state(GraderStateKind::Ready);
                    log::info!("loaded the grading model");
                },
                Err(error) => {
                    log::error!("could not load the grading model; [error={error} reached_wllama_load_step={}]", reached_wllama_load_step.get());
                    release_model_in_memory_lock();

                    // Failures before wllama's load step, in the download or the cache, say nothing about this browser.
                    if reached_wllama_load_step.get() {
                        platform::store_text(FAILED_LOAD_STORAGE_KEY, grading::GRADING_MODEL_SHA256);
                    }

                    self.set_state(GraderStateKind::Failed);
                },
            }
        }

        /// The lock on holding a model in memory, or `None` with the state moved on when this page cannot have it.
        async fn take_model_in_memory_lock(self) -> Option<HeldWebLock> {
            let requested: Result<Option<HeldWebLock>, AppErrorStatic> = web_lock::request_if_available(MODEL_IN_MEMORY_LOCK_NAME).await;

            let unavailable_state: GraderStateKind = match requested {
                Ok(Some(model_in_memory_lock)) => return Some(model_in_memory_lock),
                Ok(None) => {
                    log::info!("another page of this site holds the grading model");
                    GraderStateKind::InUseInAnotherPage
                },
                Err(error) => {
                    log::error!("could not take the lock on holding the grading model; [error={error}]");
                    GraderStateKind::Failed
                },
            };

            RUNNING_LOAD.with(|running| running.replace(None));
            self.set_state(unavailable_state);
            None
        }
    }

    fn remember_model_url(repository_base_url: &str) {
        MODEL_URL.with(|model_url| {
            if model_url.borrow().is_none() {
                model_url.replace(Some(grading_model_url(repository_base_url)));
            }
        });
    }

    fn grading_model_url(repository_base_url: &str) -> String {
        if cfg!(feature = "grading-model-from-hugging-face") {
            grading::GRADING_MODEL_HUGGING_FACE_URL.to_string()
        } else {
            fetch::join_url(repository_base_url, &grading::grading_model_file_key())
        }
    }

    fn model_source() -> Option<ModelSource> {
        let model_url: String = MODEL_URL.with(|model_url| model_url.borrow().clone())?;

        Some(ModelSource {
            model_url,
            wllama_directory_url: WLLAMA_DIRECTORY_URL.to_string(),
        })
    }

    fn release_model_in_memory_lock() {
        let model_in_memory_lock: Option<HeldWebLock> = MODEL_IN_MEMORY_LOCK.with(|lock| lock.borrow_mut().take());

        if let Some(model_in_memory_lock) = model_in_memory_lock {
            model_in_memory_lock.release();
        }
    }

    fn loaded_model() -> Option<Rc<Model>> {
        LOADED_MODEL.with(|model| model.borrow().clone())
    }

    async fn assess_this_browser(model_source: &ModelSource) -> GraderStateKind {
        let probed: Result<BrowserCapability, cross_llama::Error> = cross_llama::probe_browser_capability().await;

        let capability: BrowserCapability = match probed {
            Ok(capability) => capability,
            Err(error) => {
                log::warn!("could not probe the browser's capability; [error={error}]");
                return GraderStateKind::Unsupported;
            },
        };

        let downloaded: Result<bool, cross_llama::Error> = cross_llama::is_model_downloaded(model_source).await;
        let model_downloaded: bool = downloaded.unwrap_or_else(|error| {
            log::warn!("could not read whether the grading model is downloaded; [error={error}]");
            false
        });
        let available_storage_byte_count: Option<u64> = capability
            .storage_quota_byte_count
            .zip(capability.storage_usage_byte_count)
            .map(|(quota_byte_count, usage_byte_count)| quota_byte_count.saturating_sub(usage_byte_count));
        let earlier_load_failed: bool = platform::read_stored_text(FAILED_LOAD_STORAGE_KEY).as_deref() == Some(grading::GRADING_MODEL_SHA256);

        web_grading::assess_browser_capability(&BrowserCapabilityFacts {
            has_webgpu_adapter: capability.webgpu_adapter.is_some(),
            available_storage_byte_count,
            model_downloaded,
            earlier_load_failed,
        })
    }

    /// Asks the browser not to evict the model under storage pressure; a refusal leaves it evictable.
    async fn request_persistent_storage() {
        let Some(window) = web_sys::window()
        else {
            return;
        };

        let persist_request: Result<js_sys::Promise, wasm_bindgen::JsValue> = window.navigator().storage().persist();
        let persisted: Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> = match persist_request {
            Ok(promise) => JsFuture::from(promise).await,
            Err(error) => Err(error),
        };

        log::info!("asked to persist this site's storage; [granted={:?}]", persisted.map(|granted| granted.as_bool()));
    }

    /// A generation cancelled just before still holds the model; wllama exits once that generation drops it.
    async fn unload_once_released(model: Rc<Model>, model_in_memory_lock: Option<HeldWebLock>) {
        if let Ok(model) = Rc::try_unwrap(model) {
            let unloaded: Result<(), cross_llama::Error> = model.unload().await;

            if let Err(error) = unloaded {
                log::warn!("could not unload the grading model; [error={error}]");
            }
        }

        if let Some(model_in_memory_lock) = model_in_memory_lock {
            model_in_memory_lock.release();
        }
    }

    fn create_generation_request(
        submission: &OpenAnswerSubmission,
        json_schema: Option<serde_json::Value>,
        maximum_token_count: usize,
    ) -> GenerationRequest {
        let graded_answer: GradedAnswer = GradedAnswer {
            level: &submission.level,
            prompt_source: &submission.prompt_source,
            context_source: submission.context_source.as_deref(),
            submission: &submission.answer,
        };
        let messages: Vec<ChatMessage> = submission
            .grader_prompt
            .build_messages(&graded_answer)
            .into_iter()
            .map(|message| ChatMessage {
                role: chat_role(message.role),
                text: message.text,
            })
            .collect();

        GenerationRequest {
            messages,
            json_schema,
            temperature: GRADING_TEMPERATURE,
            seed: GRADING_SEED,
            maximum_token_count,
        }
    }

    fn chat_role(role: ChatRoleKind) -> Role {
        match role {
            ChatRoleKind::System => Role::System,
            ChatRoleKind::User => Role::User,
            ChatRoleKind::Assistant => Role::Assistant,
        }
    }
}

#[cfg(not(feature = "hydrate"))] // has no browser to run the model in
mod server {
    use shared::grading::GraderResponse;

    use super::*;

    impl GradingStore {
        pub fn prepare(self, _repository_base_url: &str, _download_enabled: bool) {}

        pub fn stop_download(self) {}

        pub fn load_model(self) {}

        pub fn warm_up(self, _submission: OpenAnswerSubmission) {}

        pub async fn grade(self, _submission: OpenAnswerSubmission) -> Result<GraderResponse, GradingFailureKind> {
            Err(GradingFailureKind::ModelNotLoaded)
        }

        pub fn unload(self) {}
    }
}
