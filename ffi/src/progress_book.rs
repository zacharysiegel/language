use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use chrono::{NaiveDate, Utc};
use shared::AppError;
use shared::filesystem;
use shared::practice::{PracticeModeKind, SessionScore};
use shared::progress::{AudioEngineKind, PracticeOutcome, Progress, QueuedCards, ReviewDirectionKind, Settings, UnitProgress};
use shared::review::{self, ReviewCard};

use crate::ffi_error::CoreError;

const UNREADABLE_PROGRESS_SUFFIX: &str = "unreadable";

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AudioEngineRecord {
    Auto,
    Kokoro,
    MacosSay,
}

impl From<AudioEngineRecord> for AudioEngineKind {
    fn from(record: AudioEngineRecord) -> AudioEngineKind {
        match record {
            AudioEngineRecord::Auto => AudioEngineKind::Auto,
            AudioEngineRecord::Kokoro => AudioEngineKind::Kokoro,
            AudioEngineRecord::MacosSay => AudioEngineKind::MacosSay,
        }
    }
}

impl From<AudioEngineKind> for AudioEngineRecord {
    fn from(kind: AudioEngineKind) -> AudioEngineRecord {
        match kind {
            AudioEngineKind::Auto => AudioEngineRecord::Auto,
            AudioEngineKind::Kokoro => AudioEngineRecord::Kokoro,
            AudioEngineKind::MacosSay => AudioEngineRecord::MacosSay,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ReviewDirectionRecord {
    PortugueseToEnglish,
    EnglishToPortuguese,
    Mixed,
}

impl From<ReviewDirectionRecord> for ReviewDirectionKind {
    fn from(record: ReviewDirectionRecord) -> ReviewDirectionKind {
        match record {
            ReviewDirectionRecord::PortugueseToEnglish => ReviewDirectionKind::PortugueseToEnglish,
            ReviewDirectionRecord::EnglishToPortuguese => ReviewDirectionKind::EnglishToPortuguese,
            ReviewDirectionRecord::Mixed => ReviewDirectionKind::Mixed,
        }
    }
}

impl From<ReviewDirectionKind> for ReviewDirectionRecord {
    fn from(kind: ReviewDirectionKind) -> ReviewDirectionRecord {
        match kind {
            ReviewDirectionKind::PortugueseToEnglish => ReviewDirectionRecord::PortugueseToEnglish,
            ReviewDirectionKind::EnglishToPortuguese => ReviewDirectionRecord::EnglishToPortuguese,
            ReviewDirectionKind::Mixed => ReviewDirectionRecord::Mixed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct SettingsRecord {
    pub show_translations: bool,
    pub slow_audio: bool,
    pub review_direction: ReviewDirectionRecord,
    pub audio_engine: AudioEngineRecord,
}

impl From<&Settings> for SettingsRecord {
    fn from(settings: &Settings) -> SettingsRecord {
        SettingsRecord {
            show_translations: settings.show_translations,
            slow_audio: settings.slow_audio,
            review_direction: settings.review_direction.into(),
            audio_engine: settings.audio_engine.into(),
        }
    }
}

impl From<SettingsRecord> for Settings {
    fn from(record: SettingsRecord) -> Settings {
        Settings {
            show_translations: record.show_translations,
            slow_audio: record.slow_audio,
            review_direction: record.review_direction.into(),
            audio_engine: record.audio_engine.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, uniffi::Record)]
pub struct UnitProgressRecord {
    pub completed: bool,
    pub best_proportion: f64,
    pub attempt_count: u32,
}

impl From<UnitProgress> for UnitProgressRecord {
    fn from(unit_progress: UnitProgress) -> UnitProgressRecord {
        UnitProgressRecord {
            completed: unit_progress.completed,
            best_proportion: unit_progress.best_proportion,
            attempt_count: unit_progress.attempt_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum PracticeModeRecord {
    Full,
    Quick,
}

impl From<PracticeModeRecord> for PracticeModeKind {
    fn from(record: PracticeModeRecord) -> PracticeModeKind {
        match record {
            PracticeModeRecord::Full => PracticeModeKind::Full,
            PracticeModeRecord::Quick => PracticeModeKind::Quick,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct PracticeOutcomeRecord {
    pub passed: bool,
    pub words_added_count: u32,
}

impl From<PracticeOutcome> for PracticeOutcomeRecord {
    fn from(outcome: PracticeOutcome) -> PracticeOutcomeRecord {
        PracticeOutcomeRecord {
            passed: outcome.passed,
            words_added_count: outcome.words_added_count as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct QueuedCardsRecord {
    pub added_count: u32,
    pub made_due_count: u32,
}

impl From<QueuedCards> for QueuedCardsRecord {
    fn from(queued: QueuedCards) -> QueuedCardsRecord {
        QueuedCardsRecord {
            added_count: queued.added_count as u32,
            made_due_count: queued.made_due_count as u32,
        }
    }
}

/// The learner's progress, saved to one file after every change.
#[derive(uniffi::Object)]
pub struct ProgressBook {
    path: PathBuf,
    progress: Mutex<Progress>,
}

#[uniffi::export]
impl ProgressBook {
    /// An unreadable saved file is kept beside the original and replaced by empty progress.
    #[uniffi::constructor]
    pub fn open(path: String) -> Arc<ProgressBook> {
        let path: PathBuf = PathBuf::from(path);
        let progress: Progress = read_saved_progress(&path);

        Arc::new(ProgressBook {
            path,
            progress: Mutex::new(progress),
        })
    }

    pub fn settings(&self) -> SettingsRecord {
        (&self.lock().settings).into()
    }

    pub fn update_settings(&self, settings: SettingsRecord) -> Result<(), CoreError> {
        self.change(|progress| progress.settings = settings.into())
    }

    pub fn unit_progress(&self, unit_id: String) -> UnitProgressRecord {
        self.lock().unit_progress(&unit_id).into()
    }

    pub fn completed_unit_count(&self) -> u32 {
        self.lock().completed_unit_count() as u32
    }

    pub fn record_practice_session(
        &self,
        unit_id: String,
        mode: PracticeModeRecord,
        correct_count: u32,
        total_count: u32,
        vocabulary_count: u32,
    ) -> Result<PracticeOutcomeRecord, CoreError> {
        let score: SessionScore = SessionScore {
            correct_count: correct_count as usize,
            total_count: total_count as usize,
        };
        let mut outcome: Option<PracticeOutcome> = None;

        self.change(|progress| {
            outcome = Some(progress.record_practice_session(&unit_id, mode.into(), score, vocabulary_count as usize, today(), now_milliseconds()));
        })?;

        outcome
            .map(PracticeOutcomeRecord::from)
            .ok_or_else(|| CoreError::Failure { message: "the practice session was not recorded".to_string() })
    }

    pub fn queue_unit_vocabulary(&self, unit_id: String, vocabulary_count: u32) -> Result<QueuedCardsRecord, CoreError> {
        let card_ids: Vec<String> = (0..vocabulary_count as usize).map(|item_index| shared::course::vocabulary_card_id(&unit_id, item_index)).collect();
        let mut queued: Option<QueuedCards> = None;

        self.change(|progress| queued = Some(progress.queue_cards_for_review(card_ids.into_iter(), now_milliseconds())))?;

        queued
            .map(QueuedCardsRecord::from)
            .ok_or_else(|| CoreError::Failure { message: "the words were not queued".to_string() })
    }

    pub fn due_card_count(&self) -> u32 {
        let now: i64 = now_milliseconds();

        self.lock().cards.values().filter(|card| card.due_milliseconds <= now).count() as u32
    }

    pub fn card_count(&self) -> u32 {
        self.lock().cards.len() as u32
    }

    /// The ids of up to `limit` cards due now, in the order they should be reviewed.
    pub fn due_card_ids(&self, limit: u32) -> Vec<String> {
        let progress: MutexGuard<'_, Progress> = self.lock();

        review::select_due_cards(progress.cards.values(), now_milliseconds(), limit as usize)
            .into_iter()
            .map(|card| card.id.clone())
            .collect()
    }

    /// Reschedules the card and counts one review as today's activity.
    pub fn grade_card(&self, card_id: String, remembered: bool) -> Result<(), CoreError> {
        let now: i64 = now_milliseconds();

        self.change(|progress| {
            let graded: Option<ReviewCard> = progress.cards.get(&card_id).map(|card| card.graded(remembered, now));

            if let Some(graded) = graded {
                progress.cards.insert(card_id.clone(), graded);
            }

            progress.record_review_activity(today(), 1);
        })
    }

    pub fn count_by_box(&self) -> Vec<u32> {
        review::count_by_box(self.lock().cards.values()).iter().map(|count| *count as u32).collect()
    }

    pub fn streak_days(&self) -> u32 {
        self.lock().streak_days(today())
    }

    pub fn export_json(&self) -> Result<String, CoreError> {
        Ok(self.lock().to_json()?)
    }

    pub fn import_json(&self, text: String) -> Result<(), CoreError> {
        let imported: Progress = Progress::from_json(&text)?;

        self.change(|progress| *progress = imported)
    }

    pub fn reset(&self) -> Result<(), CoreError> {
        self.change(|progress| *progress = Progress::default())
    }
}

impl ProgressBook {
    fn lock(&self) -> MutexGuard<'_, Progress> {
        self.progress.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn change(&self, changer: impl FnOnce(&mut Progress)) -> Result<(), CoreError> {
        let mut progress: MutexGuard<'_, Progress> = self.lock();

        changer(&mut progress);

        let text: String = progress.to_json()?;

        save(&self.path, &text)?;

        Ok(())
    }
}

fn read_saved_progress(path: &Path) -> Progress {
    let read: Result<String, std::io::Error> = fs::read_to_string(path);
    let text: String = match read {
        Ok(text) => text,
        Err(error) if error.kind() == ErrorKind::NotFound => return Progress::default(),
        Err(error) => {
            log::warn!("could not read saved progress; [path={} error={error}]", path.display());
            return Progress::default();
        },
    };
    let parsed: Result<Progress, AppError> = Progress::from_json(&text);

    match parsed {
        Ok(progress) => progress,
        Err(error) => {
            log::warn!("replacing unreadable saved progress, keeping a copy; [path={} error={error}]", path.display());

            let backup_copied: Result<u64, std::io::Error> = fs::copy(path, path.with_extension(UNREADABLE_PROGRESS_SUFFIX));

            if let Err(error) = backup_copied {
                log::warn!("could not keep a copy of unreadable progress; [error={error}]");
            }

            Progress::default()
        },
    }
}

fn save(path: &Path, text: &str) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    filesystem::write_atomically(path, text.as_bytes())
}

fn now_milliseconds() -> i64 {
    Utc::now().timestamp_millis()
}

fn today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn book_in(directory: &tempfile::TempDir) -> Arc<ProgressBook> {
        ProgressBook::open(directory.path().join("progress.json").display().to_string())
    }

    #[test]
    fn changes_are_saved_and_read_back() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();

        let book: Arc<ProgressBook> = book_in(&directory);
        book.record_practice_session("u".to_string(), PracticeModeRecord::Full, 9, 10, 4).unwrap();

        let reopened: Arc<ProgressBook> = book_in(&directory);

        assert!(reopened.unit_progress("u".to_string()).completed);
        assert_eq!(reopened.card_count(), 4);
        assert_eq!(reopened.due_card_count(), 4);
    }

    #[test]
    fn grading_a_remembered_card_takes_it_out_of_the_due_queue() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let book: Arc<ProgressBook> = book_in(&directory);
        book.queue_unit_vocabulary("u".to_string(), 2).unwrap();

        let first_due: String = book.due_card_ids(10)[0].clone();
        book.grade_card(first_due, true).unwrap();

        assert_eq!(book.due_card_count(), 1);
        assert_eq!(book.streak_days(), 1);
    }

    #[test]
    fn an_unreadable_file_is_kept_aside_and_replaced() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        fs::write(directory.path().join("progress.json"), "not json").unwrap();

        let book: Arc<ProgressBook> = book_in(&directory);

        assert_eq!(book.card_count(), 0);
        assert!(directory.path().join("progress.unreadable").exists());
    }

    #[test]
    fn settings_round_trip() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let book: Arc<ProgressBook> = book_in(&directory);
        let settings: SettingsRecord = SettingsRecord {
            show_translations: false,
            slow_audio: true,
            review_direction: ReviewDirectionRecord::Mixed,
            audio_engine: AudioEngineRecord::Kokoro,
        };

        book.update_settings(settings).unwrap();

        assert_eq!(book_in(&directory).settings(), settings);
    }
}
