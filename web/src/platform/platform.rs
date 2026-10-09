use shared::course::SpokenClip;
use shared::course_shard::ClipAudioEntry;
use shared::progress::{AudioEngineKind, VoiceChoiceKind};

/// Something to play: the recording of a clip, or the text spoken by the browser's voice when it has none.
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub clip_key: Option<String>,
    pub fallback_text: String,
    pub voice_choice: VoiceChoiceKind,
}

impl From<SpokenClip> for AudioClip {
    fn from(spoken_clip: SpokenClip) -> AudioClip {
        AudioClip {
            clip_key: Some(spoken_clip.clip_key),
            fallback_text: spoken_clip.spoken_text,
            voice_choice: spoken_clip.voice_choice,
        }
    }
}

/// How far a playback has got toward making sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackPhaseKind {
    WaitingForSound,
    SoundStarted,
}

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
pub use browser::*;
#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
pub use server::*;

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
mod browser {
    use std::cell::{Cell, RefCell};
    use std::collections::BTreeMap;

    use js_sys::{Function, Promise};
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use shared::artifact::{self, fetch};
    use shared::course_shard::VoiceEngineKind;
    use web_sys::{HtmlAudioElement, SpeechSynthesisUtterance, Storage};

    use super::*;

    const PROGRESS_STORAGE_KEY: &str = "language.progress.v2";
    const UNREADABLE_PROGRESS_STORAGE_KEY: &str = "language.progress.v2.unreadable";
    const SLOW_PLAYBACK_RATE: f64 = 0.7;
    const NORMAL_PLAYBACK_RATE: f64 = 1.0;
    const CLIP_ENDED: &str = "ended";
    const CLIP_FAILED: &str = "failed";
    const CLIP_STOPPED: &str = "stopped";
    const UTTERANCE_STARTED: &str = "started";
    const BROWSER_VOICE_LANGUAGE: &str = "pt-BR";
    const CROSS_ORIGIN_ANONYMOUS: &str = "anonymous";

    struct ActiveClip {
        audio: HtmlAudioElement,
        finish: Function,
    }

    struct ClipRecording {
        audio_sha256: String,
        /// The recordings base of the publication whose shard registered the recording.
        audio_base_url: String,
    }

    thread_local! {
        static CLIP_RECORDINGS: RefCell<BTreeMap<String, BTreeMap<VoiceEngineKind, ClipRecording>>> = const { RefCell::new(BTreeMap::new()) };
        static SELECTED_ENGINE: Cell<AudioEngineKind> = const { Cell::new(AudioEngineKind::Auto) };
        static ACTIVE_CLIP: RefCell<Option<ActiveClip>> = const { RefCell::new(None) };
        static PLAYBACK_GENERATION: Cell<u64> = const { Cell::new(0) };
        // The playback generation of a play waiting for its recording to start sounding.
        static RECORDING_WAIT_GENERATION: Cell<Option<u64>> = const { Cell::new(None) };
    }

    fn local_storage() -> Option<Storage> {
        web_sys::window()?.local_storage().ok()?
    }

    pub fn read_stored_text(key: &str) -> Option<String> {
        local_storage()?.get_item(key).ok()?
    }

    pub fn read_stored_progress() -> Option<String> {
        read_stored_text(PROGRESS_STORAGE_KEY)
    }

    pub fn write_stored_progress(text: &str) {
        store_text(PROGRESS_STORAGE_KEY, text);
    }

    pub fn back_up_unreadable_progress(text: &str) {
        store_text(UNREADABLE_PROGRESS_STORAGE_KEY, text);
    }

    pub fn store_text(key: &str, text: &str) {
        let Some(storage) = local_storage()
        else {
            return;
        };

        if let Err(error) = storage.set_item(key, text) {
            log::warn!("could not write local storage; [key={key} error={error:?}]");
        }
    }

    pub fn remove_stored_text(key: &str) {
        let Some(storage) = local_storage()
        else {
            return;
        };

        let removed: Result<(), JsValue> = storage.remove_item(key);

        if let Err(error) = removed {
            log::warn!("could not remove from local storage; [key={key} error={error:?}]");
        }
    }

    pub fn stored_keys_with_prefix(prefix: &str) -> Vec<String> {
        let Some(storage) = local_storage()
        else {
            return Vec::new();
        };
        let key_count: u32 = storage.length().unwrap_or(0);

        (0..key_count)
            .filter_map(|position| storage.key(position).ok().flatten())
            .filter(|key| key.starts_with(prefix))
            .collect()
    }

    pub fn now_milliseconds() -> i64 {
        js_sys::Date::now() as i64
    }

    pub fn random_unit() -> f64 {
        js_sys::Math::random()
    }

    fn playback_rate(slow: bool) -> f64 {
        if slow { SLOW_PLAYBACK_RATE } else { NORMAL_PLAYBACK_RATE }
    }

    pub fn stop_audio() {
        PLAYBACK_GENERATION.with(|generation| generation.set(generation.get() + 1));
        RECORDING_WAIT_GENERATION.with(|wait_generation| wait_generation.set(None));

        let active_clip: Option<ActiveClip> = ACTIVE_CLIP.with(|active| active.borrow_mut().take());

        if let Some(clip) = active_clip {
            _ = clip.audio.pause();
            _ = clip.finish.call1(&JsValue::NULL, &JsValue::from_str(CLIP_STOPPED));
        }

        if let Some(synthesis) = web_sys::window().and_then(|window| window.speech_synthesis().ok()) {
            synthesis.cancel();
        }
    }

    pub fn set_audio_engine(engine: AudioEngineKind) {
        SELECTED_ENGINE.with(|selected| selected.set(engine));
    }

    /// Adds `entries` to the recordings every earlier call registered, each served under `audio_base_url`.
    pub fn register_clip_audio(audio_base_url: &str, entries: &[ClipAudioEntry]) {
        CLIP_RECORDINGS.with(|registry| {
            let mut registry = registry.borrow_mut();

            for entry in entries {
                let clip_recording: ClipRecording = ClipRecording {
                    audio_sha256: entry.audio_sha256.clone(),
                    audio_base_url: audio_base_url.to_string(),
                };

                registry.entry(entry.clip_key.clone()).or_default().insert(entry.engine, clip_recording);
            }
        });
    }

    /// Whether a play is waiting for its recording to start sounding.
    pub fn is_recording_play_waiting() -> bool {
        RECORDING_WAIT_GENERATION.with(Cell::get).is_some()
    }

    fn clip_url(clip: &AudioClip) -> Option<String> {
        recording_url(clip, SELECTED_ENGINE.with(Cell::get))
    }

    /// The URL `clip` plays from when `engine` is selected; `None` when the browser's voice speaks it.
    pub fn recording_url(clip: &AudioClip, engine: AudioEngineKind) -> Option<String> {
        let clip_key: &str = clip.clip_key.as_deref()?;
        let preferred_engine: VoiceEngineKind = engine.voice_engine_for_clip(&clip.fallback_text, &clip.voice_choice);

        CLIP_RECORDINGS.with(|registry| {
            let registry = registry.borrow();
            let recording_by_engine: &BTreeMap<VoiceEngineKind, ClipRecording> = registry.get(clip_key)?;
            // The chosen voice's recording, else any other voice's, so a clip one engine lacks still plays a recording.
            let clip_recording: &ClipRecording = recording_by_engine
                .get(&preferred_engine)
                .or_else(|| recording_by_engine.values().next())?;

            Some(fetch::join_url(&clip_recording.audio_base_url, &artifact::audio_key(&clip_recording.audio_sha256)))
        })
    }

    /// Reports each wait for sound and each start of sound to `on_phase`.
    pub async fn play_clip(clip: &AudioClip, slow: bool, on_phase: impl Fn(PlaybackPhaseKind)) {
        stop_audio();
        play_without_stopping(clip, playback_rate(slow), &on_phase).await;
    }

    /// Reports each line's wait for sound and start of sound to `on_phase`.
    pub async fn play_sequence(clips: Vec<AudioClip>, slow: bool, on_phase: impl Fn(PlaybackPhaseKind)) {
        stop_audio();

        let sequence_generation: u64 = PLAYBACK_GENERATION.with(Cell::get);

        for clip in &clips {
            let stopped: bool = PLAYBACK_GENERATION.with(Cell::get) != sequence_generation;

            if stopped {
                return;
            }

            play_without_stopping(clip, playback_rate(slow), &on_phase).await;
        }
    }

    async fn play_without_stopping(clip: &AudioClip, rate: f64, on_phase: &impl Fn(PlaybackPhaseKind)) {
        on_phase(PlaybackPhaseKind::WaitingForSound);

        let url: Option<String> = clip_url(clip);
        let Some(url) = url
        else {
            speak_with_browser_voice(&clip.fallback_text, rate, on_phase).await;
            return;
        };

        let starting_generation: u64 = PLAYBACK_GENERATION.with(Cell::get);
        let on_recording_phase = |phase: PlaybackPhaseKind| {
            if phase == PlaybackPhaseKind::SoundStarted {
                end_recording_wait(starting_generation);
            }

            on_phase(phase);
        };

        RECORDING_WAIT_GENERATION.with(|wait_generation| wait_generation.set(Some(starting_generation)));

        let outcome: String = play_audio_file(&url, rate, &on_recording_phase).await;

        end_recording_wait(starting_generation);

        let stopped_meanwhile: bool = PLAYBACK_GENERATION.with(Cell::get) != starting_generation;

        // A pause while play() is pending rejects it, which is a stop and not a load failure.
        if outcome == CLIP_FAILED && !stopped_meanwhile {
            speak_with_browser_voice(&clip.fallback_text, rate, on_phase).await;
        }
    }

    // A play that a newer one replaced must not end the newer one's wait.
    fn end_recording_wait(generation: u64) {
        RECORDING_WAIT_GENERATION.with(|wait_generation| {
            if wait_generation.get() == Some(generation) {
                wait_generation.set(None);
            }
        });
    }

    async fn play_audio_file(url: &str, rate: f64, on_phase: &impl Fn(PlaybackPhaseKind)) -> String {
        let Ok(audio) = HtmlAudioElement::new()
        else {
            return CLIP_FAILED.to_string();
        };

        // The repository varies recordings on Origin, so only a CORS request reuses a response the prefetch cached.
        audio.set_cross_origin(Some(CROSS_ORIGIN_ANONYMOUS));
        audio.set_src(url);
        audio.set_playback_rate(rate);

        let finished: Promise = finished_promise(&audio);
        let Ok(play_promise) = audio.play()
        else {
            return CLIP_FAILED.to_string();
        };

        // play() resolves once playback has begun.
        let started: Result<JsValue, JsValue> = JsFuture::from(play_promise).await;

        if started.is_err() {
            return CLIP_FAILED.to_string();
        }

        on_phase(PlaybackPhaseKind::SoundStarted);

        let outcome: Result<JsValue, JsValue> = JsFuture::from(finished).await;

        outcome.ok().and_then(|value| value.as_string()).unwrap_or_else(|| CLIP_FAILED.to_string())
    }

    fn finished_promise(audio: &HtmlAudioElement) -> Promise {
        Promise::new(&mut |resolve: Function, _reject: Function| {
            let on_ended: JsValue = resolve_with(&[resolve.clone()], CLIP_ENDED);
            let on_error: JsValue = resolve_with(&[resolve.clone()], CLIP_FAILED);

            audio.set_onended(Some(on_ended.unchecked_ref()));
            audio.set_onerror(Some(on_error.unchecked_ref()));

            ACTIVE_CLIP.with(|active| {
                *active.borrow_mut() = Some(ActiveClip {
                    audio: audio.clone(),
                    finish: resolve,
                });
            });
        })
    }

    fn resolve_with(resolves: &[Function], outcome: &'static str) -> JsValue {
        let resolves: Vec<Function> = resolves.to_vec();

        Closure::once_into_js(move || {
            for resolve in &resolves {
                _ = resolve.call1(&JsValue::NULL, &JsValue::from_str(outcome));
            }
        })
    }

    fn resolvable_promise() -> Option<(Promise, Function)> {
        let mut captured_resolve: Option<Function> = None;
        let promise: Promise = Promise::new(&mut |resolve: Function, _reject: Function| captured_resolve = Some(resolve));

        captured_resolve.map(|resolve| (promise, resolve))
    }

    async fn speak_with_browser_voice(text: &str, rate: f64, on_phase: &impl Fn(PlaybackPhaseKind)) {
        let Some(synthesis) = web_sys::window().and_then(|window| window.speech_synthesis().ok())
        else {
            return;
        };
        let Ok(utterance) = SpeechSynthesisUtterance::new_with_text(text)
        else {
            return;
        };

        utterance.set_lang(BROWSER_VOICE_LANGUAGE);
        utterance.set_rate(rate as f32);

        let (Some((started, resolve_started)), Some((finished, resolve_finished))) = (resolvable_promise(), resolvable_promise())
        else {
            return;
        };
        let on_start: JsValue = resolve_with(&[resolve_started.clone()], UTTERANCE_STARTED);
        // An utterance that ends or fails without starting also settles the wait for its start.
        let on_end: JsValue = resolve_with(&[resolve_started.clone(), resolve_finished.clone()], CLIP_ENDED);
        let on_error: JsValue = resolve_with(&[resolve_started, resolve_finished], CLIP_FAILED);

        utterance.set_onstart(Some(on_start.unchecked_ref()));
        utterance.set_onend(Some(on_end.unchecked_ref()));
        utterance.set_onerror(Some(on_error.unchecked_ref()));

        synthesis.cancel();
        synthesis.speak(&utterance);

        let start_outcome: Result<JsValue, JsValue> = JsFuture::from(started).await;
        let began_speaking: bool = start_outcome.ok().and_then(|value| value.as_string()).as_deref() == Some(UTTERANCE_STARTED);

        if began_speaking {
            on_phase(PlaybackPhaseKind::SoundStarted);
        }

        _ = JsFuture::from(finished).await;
    }
}

#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
mod server {
    use super::*;

    // The server renders a static shell; nothing below runs before the client attaches.
    pub fn read_stored_text(_key: &str) -> Option<String> {
        None
    }

    pub fn store_text(_key: &str, _text: &str) {}

    pub fn remove_stored_text(_key: &str) {}

    pub fn stored_keys_with_prefix(_prefix: &str) -> Vec<String> {
        Vec::new()
    }

    pub fn read_stored_progress() -> Option<String> {
        None
    }

    pub fn write_stored_progress(_text: &str) {}

    pub fn back_up_unreadable_progress(_text: &str) {}

    pub fn now_milliseconds() -> i64 {
        0
    }

    pub fn random_unit() -> f64 {
        0.0
    }

    pub fn stop_audio() {}

    pub fn set_audio_engine(_engine: AudioEngineKind) {}

    pub fn register_clip_audio(_audio_base_url: &str, _entries: &[ClipAudioEntry]) {}

    pub async fn play_clip(_clip: &AudioClip, _slow: bool, _on_phase: impl Fn(PlaybackPhaseKind)) {}

    pub async fn play_sequence(_clips: Vec<AudioClip>, _slow: bool, _on_phase: impl Fn(PlaybackPhaseKind)) {}
}
