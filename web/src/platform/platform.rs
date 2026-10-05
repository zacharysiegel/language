use chrono::NaiveDate;
use shared::AppError;

#[derive(Debug, Clone)]
pub struct AudioClip {
    pub url: String,
    pub fallback_text: String,
}

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
pub use browser::*;
#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
pub use server::*;

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
mod browser {
    use std::cell::{Cell, RefCell};

    use js_sys::{Function, Promise};
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{HtmlAudioElement, SpeechSynthesisUtterance, Storage, Window};

    use super::*;

    const PROGRESS_STORAGE_KEY: &str = "portuguese-course.progress.v1";
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

    thread_local! {
        static ACTIVE_CLIP: RefCell<Option<ActiveClip>> = const { RefCell::new(None) };
        static PLAYBACK_GENERATION: Cell<u64> = const { Cell::new(0) };
    }

    fn js_error(value: JsValue) -> AppError {
        AppError::from(value.as_string().unwrap_or_else(|| format!("{value:?}")))
    }

    fn get_window() -> Result<Window, AppError> {
        web_sys::window().ok_or_else(|| AppError::from("no window".to_string()))
    }

    fn local_storage() -> Option<Storage> {
        web_sys::window()?.local_storage().ok()?
    }

    pub async fn fetch_text(url: &str) -> Result<String, AppError> {
        let window: Window = get_window()?;
        let response_value: JsValue = JsFuture::from(window.fetch_with_str(url)).await.map_err(js_error)?;
        let response: web_sys::Response = response_value
            .dyn_into()
            .map_err(|value| AppError::from(format!("unexpected fetch result; [value={value:?}]")))?;

        if !response.ok() {
            return Err(AppError::from(format!("request failed; [url={url} status={}]", response.status())));
        }

        let text_promise: Promise = response.text().map_err(js_error)?;
        let text_value: JsValue = JsFuture::from(text_promise).await.map_err(js_error)?;

        text_value
            .as_string()
            .ok_or_else(|| AppError::from(format!("response body was not text; [url={url}]")))
    }

    pub fn read_stored_progress() -> Option<String> {
        local_storage()?.get_item(PROGRESS_STORAGE_KEY).ok()?
    }

    pub fn write_stored_progress(text: &str) {
        let Some(storage) = local_storage() else {
            return;
        };

        if let Err(error) = storage.set_item(PROGRESS_STORAGE_KEY, text) {
            log::warn!("could not save progress; [error={error:?}]");
        }
    }

    pub fn now_milliseconds() -> i64 {
        js_sys::Date::now() as i64
    }

    pub fn today() -> NaiveDate {
        let now: js_sys::Date = js_sys::Date::new_0();

        NaiveDate::from_ymd_opt(now.get_full_year() as i32, now.get_month() + 1, now.get_date())
            .unwrap_or(NaiveDate::MIN)
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

    pub async fn play_clip(url: &str, fallback_text: &str, slow: bool) {
        stop_audio();
        play_without_stopping(url, fallback_text, playback_rate(slow)).await;
    }

    pub async fn play_sequence(clips: Vec<AudioClip>, slow: bool) {
        stop_audio();

        let sequence_generation: u64 = PLAYBACK_GENERATION.with(Cell::get);

        for clip in &clips {
            let stopped: bool = PLAYBACK_GENERATION.with(Cell::get) != sequence_generation;

            if stopped {
                return;
            }

            play_without_stopping(&clip.url, &clip.fallback_text, playback_rate(slow)).await;
        }
    }

    async fn play_without_stopping(url: &str, fallback_text: &str, rate: f64) {
        let outcome: String = play_audio_file(url, rate).await;

        if outcome == CLIP_FAILED {
            speak_with_browser_voice(fallback_text, rate).await;
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
    pub async fn fetch_text(_url: &str) -> Result<String, AppError> {
        Err(AppError::from("fetching is only available in the browser".to_string()))
    }

    pub fn read_stored_progress() -> Option<String> {
        None
    }

    pub fn write_stored_progress(_text: &str) {}

    pub fn now_milliseconds() -> i64 {
        0
    }

    pub fn today() -> NaiveDate {
        NaiveDate::MIN
    }

    pub fn random_unit() -> f64 {
        0.0
    }

    pub fn stop_audio() {}

    pub async fn play_clip(_url: &str, _fallback_text: &str, _slow: bool) {}

    pub async fn play_sequence(_clips: Vec<AudioClip>, _slow: bool) {}
}
