use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::platform::{self, AudioClip};
use crate::progress_state::{ProgressStore, use_progress_store};

pub fn audio_clip(audio: &Option<String>, fallback_text: &str) -> AudioClip {
    AudioClip {
        file: audio.clone().unwrap_or_default(),
        fallback_text: fallback_text.to_string(),
    }
}

pub fn play_in_background(clip: AudioClip, slow: bool) {
    spawn_local(async move {
        platform::play_clip(&clip.file, &clip.fallback_text, slow).await;
    });
}

#[component]
pub fn PlayButton(
    audio: Option<String>,
    #[prop(into)] text: String,
    #[prop(optional)] slow: bool,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let button_label: String = label.unwrap_or_else(|| if slow { "Slow".to_string() } else { "▶".to_string() });
    let title: &str = if slow { "Play slowly" } else { "Play audio" };

    view! {
        <button
            type="button"
            class="play"
            title=title
            aria-label=format!("{title}: {text}")
            on:click=move |event| {
                event.stop_propagation();
                let use_slow: bool = slow || progress_store.read(|progress| progress.settings.slow_audio);
                play_in_background(audio_clip(&audio, &text), use_slow);
            }
        >
            {button_label}
        </button>
    }
}

#[component]
pub fn PlayAllButton(clips: Vec<AudioClip>) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();

    view! {
        <span class="row">
            <button
                type="button"
                on:click={
                    let clips: Vec<AudioClip> = clips.clone();
                    move |_| {
                        let clips: Vec<AudioClip> = clips.clone();
                        let use_slow: bool = progress_store.read(|progress| progress.settings.slow_audio);
                        spawn_local(async move {
                            platform::play_sequence(clips, use_slow).await;
                        });
                    }
                }
            >
                "▶ Play all"
            </button>
            <button type="button" on:click=move |_| platform::stop_audio()>"■ Stop"</button>
        </span>
    }
}
