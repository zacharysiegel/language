use std::sync::atomic::{AtomicU64, Ordering};

use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::progress::VoiceChoiceKind;

use crate::pending_indication::{self, PendingIcon};
use crate::platform::{self, AudioClip, PlaybackPhaseKind};
use crate::progress_state::{ProgressStore, use_progress_store};

static NEXT_SEQUENCE_ID: AtomicU64 = AtomicU64::new(0);
static NEXT_PLAYBACK_TOKEN: AtomicU64 = AtomicU64::new(0);

const LOADING_RECORDING_ANNOUNCEMENT: &str = "Loading the recording";

pub fn audio_clip(clip_key: &Option<String>, fallback_text: &str, voice_choice: VoiceChoiceKind) -> AudioClip {
    AudioClip {
        clip_key: clip_key.clone(),
        fallback_text: fallback_text.to_string(),
        voice_choice,
    }
}

fn next_playback_token() -> u64 {
    NEXT_PLAYBACK_TOKEN.fetch_add(1, Ordering::Relaxed)
}

/// The control a playback's pending indication is shown on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackTarget {
    Clip {
        clip_key: Option<String>,
        text: String,
        /// Whether it is the 0.7× control, whatever speed the playback runs at.
        slow: bool,
    },
    Sequence {
        sequence_id: u64,
    },
}

impl PlaybackTarget {
    pub fn for_clip(clip: &AudioClip, slow: bool) -> PlaybackTarget {
        PlaybackTarget::Clip {
            clip_key: clip.clip_key.clone(),
            text: clip.fallback_text.clone(),
            slow,
        }
    }

    fn new_sequence() -> PlaybackTarget {
        PlaybackTarget::Sequence {
            sequence_id: NEXT_SEQUENCE_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PendingPlayback {
    target: PlaybackTarget,
    /// The play request it belongs to.
    attempt: u64,
    /// This wait for sound, one of several in a sequence.
    wait: u64,
    indicated: bool,
}

#[derive(Clone, Copy)]
pub struct PlaybackStore {
    pending: RwSignal<Option<PendingPlayback>>,
    indicated_target: Memo<Option<PlaybackTarget>>,
}

impl PlaybackStore {
    /// Plays `clip`, showing its pending indication on `target`.
    pub fn play_clip(self, clip: AudioClip, target: PlaybackTarget, slow: bool) {
        let attempt: u64 = next_playback_token();

        spawn_local(async move {
            platform::play_clip(&clip, slow, |phase| self.observe_phase(&target, attempt, phase)).await;
            self.end_attempt(attempt);
        });
    }

    /// Plays `clip` at normal speed, indicated on its normal-speed play control.
    pub fn play_at_normal_speed(self, clip: AudioClip) {
        let target: PlaybackTarget = PlaybackTarget::for_clip(&clip, false);

        self.play_clip(clip, target, false);
    }

    pub fn play_sequence(self, clips: Vec<AudioClip>, target: PlaybackTarget, slow: bool) {
        let attempt: u64 = next_playback_token();

        spawn_local(async move {
            platform::play_sequence(clips, slow, |phase| self.observe_phase(&target, attempt, phase)).await;
            self.end_attempt(attempt);
        });
    }

    pub fn stop(self) {
        platform::stop_audio();
        self.pending.set(None);
    }

    /// Whether `target` shows a pending indication now.
    pub fn is_indicated(self, target: &PlaybackTarget) -> bool {
        self.indicated_target.with(|indicated_target| indicated_target.as_ref() == Some(target))
    }

    fn is_any_indicated(self) -> bool {
        self.indicated_target.with(Option::is_some)
    }

    fn observe_phase(self, target: &PlaybackTarget, attempt: u64, phase: PlaybackPhaseKind) {
        match phase {
            PlaybackPhaseKind::WaitingForSound => self.begin_wait(target.clone(), attempt),
            PlaybackPhaseKind::SoundStarted => self.end_attempt(attempt),
        }
    }

    fn begin_wait(self, target: PlaybackTarget, attempt: u64) {
        let wait: u64 = next_playback_token();

        self.pending.set(Some(PendingPlayback {
            target,
            attempt,
            wait,
            indicated: false,
        }));

        set_timeout(move || self.indicate_wait(wait), pending_indication::pending_indication_delay());
    }

    fn indicate_wait(self, wait: u64) {
        self.pending.update(|pending| {
            if let Some(pending) = pending.as_mut().filter(|pending| pending.wait == wait) {
                pending.indicated = true;
            }
        });
    }

    fn end_attempt(self, attempt: u64) {
        let belongs_to_attempt: bool = self.pending.with_untracked(|pending| pending.as_ref().is_some_and(|pending| pending.attempt == attempt));

        if belongs_to_attempt {
            self.pending.set(None);
        }
    }
}

pub fn provide_playback_store() {
    let pending: RwSignal<Option<PendingPlayback>> = RwSignal::new(None);
    let indicated_target: Memo<Option<PlaybackTarget>> = Memo::new(move |_| {
        pending.with(|pending| pending.as_ref().filter(|pending| pending.indicated).map(|pending| pending.target.clone()))
    });

    provide_context(PlaybackStore { pending, indicated_target });
}

pub fn use_playback_store() -> PlaybackStore {
    expect_context::<PlaybackStore>()
}

/// Whether `target` shows a pending indication, notifying only when that changes.
pub fn indicated_pending(playback_store: PlaybackStore, target: PlaybackTarget) -> Memo<bool> {
    Memo::new(move |_| playback_store.is_indicated(&target))
}

/// The live region that announces a pending playback to assistive technology.
#[component]
pub fn PlaybackAnnouncement() -> impl IntoView {
    let playback_store: PlaybackStore = use_playback_store();

    view! {
        <p class="visually-hidden" role="status" aria-live="polite">
            {move || playback_store.is_any_indicated().then_some(LOADING_RECORDING_ANNOUNCEMENT)}
        </p>
    }
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

/// Holds a control's glyph, and the pending glyph over it, so a pending control keeps its size.
#[component]
fn GlyphSlot(pending: Memo<bool>, children: Children) -> impl IntoView {
    view! {
        <span class="glyph-slot">
            <span class="glyph-resting">{children()}</span>
            {move || pending.get().then(|| view! { <PendingIcon /> })}
        </span>
    }
}

#[component]
pub fn PlayButton(
    clip_key: Option<String>,
    #[prop(into)] text: String,
    #[prop(optional)] slow: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] voice_choice: VoiceChoiceKind,
) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let playback_store: PlaybackStore = use_playback_store();
    let title: &str = if slow { "Play slowly" } else { "Play audio" };
    let class: &str = match (slow, label.is_some()) {
        (true, _) => "play play-slow",
        (false, true) => "play has-label",
        (false, false) => "play",
    };
    let clip: AudioClip = audio_clip(&clip_key, &text, voice_choice);
    let target: PlaybackTarget = PlaybackTarget::for_clip(&clip, slow);
    let pending: Memo<bool> = indicated_pending(playback_store, target.clone());

    view! {
        <button
            type="button"
            class=class
            class:pending=pending
            title=title
            aria-label=format!("{title}: {text}")
            aria-busy=move || pending.get().then_some("true")
            on:click=move |event| {
                event.stop_propagation();
                let use_slow: bool = slow || progress_store.read(|progress| progress.settings.slow_audio);
                playback_store.play_clip(clip.clone(), target.clone(), use_slow);
            }
        >
            <GlyphSlot pending>
                {if slow { view! { "0.7×" }.into_any() } else { view! { <PlayIcon /> }.into_any() }}
            </GlyphSlot>
            {label}
        </button>
    }
}

#[component]
pub fn PlayAllButton(clips: Vec<AudioClip>) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let playback_store: PlaybackStore = use_playback_store();
    let target: PlaybackTarget = PlaybackTarget::new_sequence();
    let pending: Memo<bool> = indicated_pending(playback_store, target.clone());

    view! {
        <span class="row">
            <button
                type="button"
                class="play has-label"
                class:pending=pending
                aria-busy=move || pending.get().then_some("true")
                on:click=move |_| {
                    let use_slow: bool = progress_store.read(|progress| progress.settings.slow_audio);
                    playback_store.play_sequence(clips.clone(), target.clone(), use_slow);
                }
            >
                <GlyphSlot pending>
                    <PlayIcon />
                </GlyphSlot>
                "Play all"
            </button>
            <button type="button" class="play has-label" on:click=move |_| playback_store.stop()>
                <StopIcon />
                "Stop"
            </button>
        </span>
    }
}
