use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::platform::{self, AudioClip};
use crate::progress_state::{ProgressStore, use_progress_store};

pub fn audio_clip(clip_key: &Option<String>, fallback_text: &str) -> AudioClip {
    AudioClip {
        clip_key: clip_key.clone(),
        fallback_text: fallback_text.to_string(),
    }
}

pub fn play_in_background(clip: AudioClip, slow: bool) {
    spawn_local(async move {
        platform::play_clip(&clip, slow).await;
    });
}

#[component]
fn PlayIcon() -> impl IntoView {
    view! {
        <svg class="icon" viewBox="0 0 13 13" aria-hidden="true">
            <path d="M3.5 2.5l7 4-7 4z" />
        </svg>
    }
}

#[component]
fn StopIcon() -> impl IntoView {
    view! {
        <svg class="icon" viewBox="0 0 13 13" aria-hidden="true">
            <rect x="3" y="3" width="7" height="7" />
        </svg>
    }
}

#[component]
pub fn PlayButton(
    clip_key: Option<String>,
    #[prop(into)] text: String,
    #[prop(optional)] slow: bool,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let title: &str = if slow { "Play slowly" } else { "Play audio" };
    let class: &str = match (slow, label.is_some()) {
        (true, _) => "play play-slow",
        (false, true) => "play has-label",
        (false, false) => "play",
    };

    view! {
        <button
            type="button"
            class=class
            title=title
            aria-label=format!("{title}: {text}")
            on:click=move |event| {
                event.stop_propagation();
                let use_slow: bool = slow || progress_store.read(|progress| progress.settings.slow_audio);
                play_in_background(audio_clip(&clip_key, &text), use_slow);
            }
        >
            {if slow { view! { "0.7×" }.into_any() } else { view! { <PlayIcon /> }.into_any() }}
            {label}
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
                class="play has-label"
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
                <PlayIcon />
                "Play all"
            </button>
            <button type="button" class="play has-label" on:click=move |_| platform::stop_audio()>
                <StopIcon />
                "Stop"
            </button>
        </span>
    }
}
