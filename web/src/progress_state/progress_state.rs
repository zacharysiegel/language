use std::time::Duration;

use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use shared::course::vocabulary_card_id;
use shared::progress::{Progress, QueuedCards, UnitStatusKind};

use crate::platform;

// The device's clock can change while the store waits for the next card to come due.
const REVIEW_CLOCK_MAXIMUM_WAIT_MILLISECONDS: i64 = 60 * 60 * 1000;
const VISIBILITY_CHANGE_EVENT: &str = "visibilitychange";

#[derive(Clone, Copy)]
pub struct ProgressStore {
    progress: RwSignal<Progress>,
    /// The time every due read is taken at.
    review_clock_milliseconds: RwSignal<i64>,
    review_clock_advance: StoredValue<Option<TimeoutHandle>>,
}

impl ProgressStore {
    pub fn read<R>(&self, reader: impl FnOnce(&Progress) -> R) -> R {
        self.progress.with(reader)
    }

    pub fn update(&self, updater: impl FnOnce(&mut Progress)) {
        self.progress.update(updater);
    }

    pub fn replace(&self, replacement: Progress) {
        self.progress.set(replacement);
    }

    /// Records the course as the one the learner opened last, when it is not already.
    pub fn remember_course(&self, course_id: &str) {
        let is_remembered: bool = self.progress.with_untracked(|progress| progress.settings.last_course_id.as_deref() == Some(course_id));

        if !is_remembered {
            self.progress.update(|progress| progress.settings.last_course_id = Some(course_id.to_string()));
        }
    }

    /// Records the unit as started, when it is not already.
    pub fn start_unit(&self, course_id: &str, unit_id: &str) {
        let status: UnitStatusKind = self.progress.with_untracked(|progress| progress.course_progress(course_id).unit_progress(unit_id).status);

        if status == UnitStatusKind::NotStarted {
            self.progress.update(|progress| progress.course_progress_mut(course_id).start_unit(unit_id));
        }
    }

    pub fn unit_status(&self, course_id: &str, unit_id: &str) -> UnitStatusKind {
        self.progress.with(|progress| progress.course_progress(course_id).unit_progress(unit_id).status)
    }

    pub fn set_unit_status(&self, course_id: &str, unit_id: &str, status: UnitStatusKind) {
        self.progress.update(|progress| progress.course_progress_mut(course_id).set_unit_status(unit_id, status));
    }

    pub fn due_card_count(&self, course_id: &str) -> usize {
        let now_milliseconds: i64 = self.review_clock_milliseconds.get();

        self.progress.with(|progress| progress.course_progress(course_id).due_card_count(now_milliseconds))
    }

    pub fn review_clock_milliseconds(&self) -> i64 {
        self.review_clock_milliseconds.get()
    }

    /// Moves the time due reads are taken at to the present, and again when the next card comes due.
    pub fn advance_review_clock(self) {
        let now_milliseconds: i64 = platform::now_milliseconds();

        self.review_clock_milliseconds.set(now_milliseconds);
        self.schedule_review_clock_advance(now_milliseconds);
    }

    fn schedule_review_clock_advance(self, now_milliseconds: i64) {
        let wait_milliseconds: i64 = self.review_clock_wait_milliseconds(now_milliseconds);
        let delay: Duration = Duration::from_millis(u64::try_from(wait_milliseconds).unwrap_or(0));
        let previous_advance: Option<TimeoutHandle> = self.review_clock_advance.get_value();

        if let Some(previous_advance) = previous_advance {
            previous_advance.clear();
        }

        let scheduled: Result<TimeoutHandle, JsValue> = set_timeout_with_handle(move || self.advance_review_clock(), delay);

        let next_advance: Option<TimeoutHandle> = match scheduled {
            Ok(handle) => Some(handle),
            Err(error) => {
                log::warn!("could not schedule the review clock's next advance; [error={:?}]", error);
                None
            },
        };

        self.review_clock_advance.set_value(next_advance);
    }

    fn review_clock_wait_milliseconds(self, now_milliseconds: i64) -> i64 {
        let next_due_milliseconds: Option<i64> = self.progress.with_untracked(|progress| progress.next_due_milliseconds_after(now_milliseconds));

        next_due_milliseconds
            .map(|next_due_milliseconds| next_due_milliseconds - now_milliseconds)
            .unwrap_or(REVIEW_CLOCK_MAXIMUM_WAIT_MILLISECONDS)
            .min(REVIEW_CLOCK_MAXIMUM_WAIT_MILLISECONDS)
    }

    fn advance_review_clock_if_visible(self) {
        let page_hidden: bool = document().hidden();

        if page_hidden {
            return;
        }

        self.advance_review_clock();
    }

    /// Adds missing words and brings words that are scheduled for later back to the course's review queue.
    pub fn queue_unit_vocabulary_for_review(&self, course_id: &str, unit_id: &str, vocabulary_count: usize) -> QueuedCards {
        let card_ids: Vec<String> = (0..vocabulary_count).map(|item_index| vocabulary_card_id(unit_id, item_index)).collect();
        let now_milliseconds: i64 = platform::now_milliseconds();
        let mut queued: QueuedCards = QueuedCards {
            added_count: 0,
            made_due_count: 0,
        };

        self.progress.update(|progress| {
            queued = progress.course_progress_mut(course_id).queue_cards_for_review(card_ids.into_iter(), now_milliseconds);
        });

        queued
    }
}

pub fn provide_progress_store() {
    let progress: RwSignal<Progress> = RwSignal::new(Progress::default());
    // Saving waits for the stored progress to be read, or it would overwrite it with the default.
    let loaded: RwSignal<bool> = RwSignal::new(false);

    Effect::new(move |_| {
        let stored_text: Option<String> = platform::read_stored_progress();

        if let Some(stored_text) = stored_text {
            match Progress::from_json(&stored_text) {
                Ok(stored_progress) => progress.set(stored_progress),
                Err(error) => {
                    log::warn!("replacing unreadable saved progress, keeping a copy; [error={error}]");
                    platform::back_up_unreadable_progress(&stored_text);
                },
            }
        }

        loaded.set(true);
    });

    Effect::new(move |_| {
        let current: Progress = progress.get();

        if !loaded.get() {
            return;
        }

        match current.to_json() {
            Ok(text) => platform::write_stored_progress(&text),
            Err(error) => log::warn!("could not serialize progress; [error={error}]"),
        }
    });

    Effect::new(move |_| platform::set_audio_engine(progress.with(|current| current.settings.audio_engine)));

    let progress_store: ProgressStore = ProgressStore {
        progress,
        review_clock_milliseconds: RwSignal::new(platform::now_milliseconds()),
        review_clock_advance: StoredValue::new(None),
    };

    Effect::new(move |_| {
        progress.track();
        progress_store.advance_review_clock();
    });

    Effect::new(move |_| {
        window_event_listener_untyped(VISIBILITY_CHANGE_EVENT, move |_| progress_store.advance_review_clock_if_visible());
    });

    provide_context(progress_store);
}

/// Keeps the course of the current page recorded as the one the learner opened last.
pub fn remember_route_course(progress_store: ProgressStore, course_id: Memo<String>) {
    Effect::new(move |_| progress_store.remember_course(&course_id.get()));
}

pub fn use_progress_store() -> ProgressStore {
    expect_context::<ProgressStore>()
}
