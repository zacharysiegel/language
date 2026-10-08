use shared::course_shard::ClipAudioEntry;
use shared::progress::{AudioEngineKind, VoiceChoiceKind};

/// Something to play: the recording of a clip, or the text spoken by the browser's voice when it has none.
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub clip_key: Option<String>,
    pub fallback_text: String,
    pub voice_choice: VoiceChoiceKind,
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
    const BROWSER_VOICE_LANGUAGE: &str = "pt-BR";

    struct ActiveClip {
        audio: HtmlAudioElement,
        finish: Function,
    }

    /// The recordings of the shown version.
    #[derive(Default)]
    struct ClipAudioRegistry {
        audio_base_url: String,
        audio_sha256_by_clip: BTreeMap<String, BTreeMap<VoiceEngineKind, String>>,
    }

    thread_local! {
        static CLIP_AUDIO: RefCell<ClipAudioRegistry> = RefCell::new(ClipAudioRegistry::default());
        static SELECTED_ENGINE: Cell<AudioEngineKind> = const { Cell::new(AudioEngineKind::Auto) };
        static ACTIVE_CLIP: RefCell<Option<ActiveClip>> = const { RefCell::new(None) };
        static PLAYBACK_GENERATION: Cell<u64> = const { Cell::new(0) };
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
        let Some(storage) = local_storage() else {
            return;
        };

        if let Err(error) = storage.set_item(key, text) {
            log::warn!("could not write local storage; [key={key} error={error:?}]");
        }
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

    pub fn reset_clip_audio(audio_base_url: &str, entries: &[ClipAudioEntry]) {
        CLIP_AUDIO.with(|registry| {
            *registry.borrow_mut() = ClipAudioRegistry {
                audio_base_url: audio_base_url.to_string(),
                audio_sha256_by_clip: BTreeMap::new(),
            };
        });

        register_clip_audio(entries);
    }

    pub fn register_clip_audio(entries: &[ClipAudioEntry]) {
        CLIP_AUDIO.with(|registry| {
            let mut registry = registry.borrow_mut();

            for entry in entries {
                registry
                    .audio_sha256_by_clip
                    .entry(entry.clip_key.clone())
                    .or_default()
                    .insert(entry.engine, entry.audio_sha256.clone());
            }
        });
    }

    /* The chosen voice's recording, else any other voice's, so a clip one engine lacks still plays a recording. */
    fn clip_url(clip: &AudioClip) -> Option<String> {
        let clip_key: &str = clip.clip_key.as_deref()?;
        let preferred_engine: VoiceEngineKind = SELECTED_ENGINE.with(Cell::get).voice_engine_for_clip(&clip.fallback_text, &clip.voice_choice);

        CLIP_AUDIO.with(|registry| {
            let registry = registry.borrow();
            let audio_sha256_by_engine: &BTreeMap<VoiceEngineKind, String> = registry.audio_sha256_by_clip.get(clip_key)?;
            let audio_sha256: &String = audio_sha256_by_engine
                .get(&preferred_engine)
                .or_else(|| audio_sha256_by_engine.values().next())?;

            Some(fetch::join_url(&registry.audio_base_url, &artifact::audio_key(audio_sha256)))
        })
    }

    pub async fn play_clip(clip: &AudioClip, slow: bool) {
        stop_audio();
        play_without_stopping(clip, playback_rate(slow)).await;
    }

    pub async fn play_sequence(clips: Vec<AudioClip>, slow: bool) {
        stop_audio();

        let sequence_generation: u64 = PLAYBACK_GENERATION.with(Cell::get);

        for clip in &clips {
            let stopped: bool = PLAYBACK_GENERATION.with(Cell::get) != sequence_generation;

            if stopped {
                return;
            }

            play_without_stopping(clip, playback_rate(slow)).await;
        }
    }

    async fn play_without_stopping(clip: &AudioClip, rate: f64) {
        let url: Option<String> = clip_url(clip);
        let Some(url) = url
        else {
            speak_with_browser_voice(&clip.fallback_text, rate).await;
            return;
        };

        let starting_generation: u64 = PLAYBACK_GENERATION.with(Cell::get);
        let outcome: String = play_audio_file(&url, rate).await;
        let stopped_meanwhile: bool = PLAYBACK_GENERATION.with(Cell::get) != starting_generation;

        // A pause while play() is pending rejects it, which is a stop and not a load failure.
        if outcome == CLIP_FAILED && !stopped_meanwhile {
            speak_with_browser_voice(&clip.fallback_text, rate).await;
        }
    }

    async fn play_audio_file(url: &str, rate: f64) -> String {
        let Ok(audio) = HtmlAudioElement::new_with_src(url) else {
            return CLIP_FAILED.to_string();
        };

        audio.set_playback_rate(rate);

        let finished: Promise = finished_promise(&audio);
        let Ok(play_promise) = audio.play() else {
            return CLIP_FAILED.to_string();
        };

        if JsFuture::from(play_promise).await.is_err() {
            return CLIP_FAILED.to_string();
        }

        let outcome: Result<JsValue, JsValue> = JsFuture::from(finished).await;

        outcome.ok().and_then(|value| value.as_string()).unwrap_or_else(|| CLIP_FAILED.to_string())
    }

    fn finished_promise(audio: &HtmlAudioElement) -> Promise {
        Promise::new(&mut |resolve: Function, _reject: Function| {
            let on_ended: JsValue = resolve_with(&resolve, CLIP_ENDED);
            let on_error: JsValue = resolve_with(&resolve, CLIP_FAILED);

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

    fn resolve_with(resolve: &Function, outcome: &'static str) -> JsValue {
        let resolve: Function = resolve.clone();

        Closure::once_into_js(move || {
            _ = resolve.call1(&JsValue::NULL, &JsValue::from_str(outcome));
        })
    }

    async fn speak_with_browser_voice(text: &str, rate: f64) {
        let Some(synthesis) = web_sys::window().and_then(|window| window.speech_synthesis().ok()) else {
            return;
        };
        let Ok(utterance) = SpeechSynthesisUtterance::new_with_text(text) else {
            return;
        };

        utterance.set_lang(BROWSER_VOICE_LANGUAGE);
        utterance.set_rate(rate as f32);

        let finished: Promise = Promise::new(&mut |resolve: Function, _reject: Function| {
            let on_end: JsValue = resolve_with(&resolve, CLIP_ENDED);
            let on_error: JsValue = resolve_with(&resolve, CLIP_FAILED);

            utterance.set_onend(Some(on_end.unchecked_ref()));
            utterance.set_onerror(Some(on_error.unchecked_ref()));
        });

        synthesis.cancel();
        synthesis.speak(&utterance);

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

    pub fn reset_clip_audio(_audio_base_url: &str, _entries: &[ClipAudioEntry]) {}

    pub fn register_clip_audio(_entries: &[ClipAudioEntry]) {}

    pub async fn play_clip(_clip: &AudioClip, _slow: bool) {}

    pub async fn play_sequence(_clips: Vec<AudioClip>, _slow: bool) {}
}
