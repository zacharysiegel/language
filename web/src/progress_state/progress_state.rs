use leptos::prelude::*;
use shared::course::vocabulary_card_id;
use shared::progress::Progress;

use crate::platform;

#[derive(Clone, Copy)]
pub struct ProgressStore {
    progress: RwSignal<Progress>,
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

    pub fn add_unit_vocabulary(&self, unit_id: &str, vocabulary_count: usize) -> usize {
        let card_ids: Vec<String> = (0..vocabulary_count).map(|item_index| vocabulary_card_id(unit_id, item_index)).collect();
        let now_milliseconds: i64 = platform::now_milliseconds();
        let mut added_count: usize = 0;

        self.progress.update(|progress| {
            added_count = progress.add_cards_if_missing(card_ids.into_iter(), now_milliseconds);
        });

        added_count
    }
}

pub fn provide_progress_store() {
    let progress: RwSignal<Progress> = RwSignal::new(Progress::default());
    let loaded: RwSignal<bool> = RwSignal::new(false);

    // Saving before the stored progress has been read would overwrite it with the empty default.
    Effect::new(move |_| {
        let stored_text: Option<String> = platform::read_stored_progress();

        if let Some(stored_text) = stored_text {
            match Progress::from_json(&stored_text) {
                Ok(stored_progress) => progress.set(stored_progress),
                Err(error) => log::warn!("ignoring unreadable saved progress; [error={error}]"),
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

    provide_context(ProgressStore { progress });
}

pub fn use_progress_store() -> ProgressStore {
    expect_context::<ProgressStore>()
}
