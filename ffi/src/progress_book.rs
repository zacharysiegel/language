use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use chrono::Utc;
use shared::AppError;
use shared::course::Exercise;
use shared::filesystem;
use shared::practice::{PracticeModeKind, SessionScore};
use shared::progress::{
    AudioEngineKind, CourseProgress, PausedPractice, PausedPracticeStateKind, PracticeOutcome, Progress, QueuedCards, ReviewDirectionKind, SavedExerciseResult,
    Settings, UnitProgress, UnitStatusKind, VoiceChoiceKind,
};
use shared::review::{self, ReviewCard};

use crate::course_records::ExerciseRecord;
use crate::ffi_error::CoreError;
use crate::practice_functions::AnswerStatusRecord;

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

/// Which texts decide the recording set a clip plays from.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum VoiceChoiceRecord {
    OwnText,
    WholeDialogue {
        line_texts: Vec<String>,
    },
}

impl From<VoiceChoiceRecord> for VoiceChoiceKind {
    fn from(record: VoiceChoiceRecord) -> VoiceChoiceKind {
        match record {
            VoiceChoiceRecord::OwnText => VoiceChoiceKind::OwnText,
            VoiceChoiceRecord::WholeDialogue { line_texts } => VoiceChoiceKind::WholeDialogue { line_texts },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ReviewDirectionRecord {
    TargetToSource,
    SourceToTarget,
    Mixed,
}

impl From<ReviewDirectionRecord> for ReviewDirectionKind {
    fn from(record: ReviewDirectionRecord) -> ReviewDirectionKind {
        match record {
            ReviewDirectionRecord::TargetToSource => ReviewDirectionKind::TargetToSource,
            ReviewDirectionRecord::SourceToTarget => ReviewDirectionKind::SourceToTarget,
            ReviewDirectionRecord::Mixed => ReviewDirectionKind::Mixed,
        }
    }
}

impl From<ReviewDirectionKind> for ReviewDirectionRecord {
    fn from(kind: ReviewDirectionKind) -> ReviewDirectionRecord {
        match kind {
            ReviewDirectionKind::TargetToSource => ReviewDirectionRecord::TargetToSource,
            ReviewDirectionKind::SourceToTarget => ReviewDirectionRecord::SourceToTarget,
            ReviewDirectionKind::Mixed => ReviewDirectionRecord::Mixed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SettingsRecord {
    pub show_translations: bool,
    pub slow_audio: bool,
    pub review_direction: ReviewDirectionRecord,
    pub audio_engine: AudioEngineRecord,
    pub last_course_id: Option<String>,
}

impl From<&Settings> for SettingsRecord {
    fn from(settings: &Settings) -> SettingsRecord {
        SettingsRecord {
            show_translations: settings.show_translations,
            slow_audio: settings.slow_audio,
            review_direction: settings.review_direction.into(),
            audio_engine: settings.audio_engine.into(),
            last_course_id: settings.last_course_id.clone(),
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
            last_course_id: record.last_course_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum UnitStatusRecord {
    NotStarted,
    InProgress,
    Completed,
}

impl From<UnitStatusRecord> for UnitStatusKind {
    fn from(record: UnitStatusRecord) -> UnitStatusKind {
        match record {
            UnitStatusRecord::NotStarted => UnitStatusKind::NotStarted,
            UnitStatusRecord::InProgress => UnitStatusKind::InProgress,
            UnitStatusRecord::Completed => UnitStatusKind::Completed,
        }
    }
}

impl From<UnitStatusKind> for UnitStatusRecord {
    fn from(kind: UnitStatusKind) -> UnitStatusRecord {
        match kind {
            UnitStatusKind::NotStarted => UnitStatusRecord::NotStarted,
            UnitStatusKind::InProgress => UnitStatusRecord::InProgress,
            UnitStatusKind::Completed => UnitStatusRecord::Completed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, uniffi::Record)]
pub struct UnitProgressRecord {
    pub status: UnitStatusRecord,
    pub best_proportion: f64,
    pub attempt_count: u32,
}

impl From<UnitProgress> for UnitProgressRecord {
    fn from(unit_progress: UnitProgress) -> UnitProgressRecord {
        UnitProgressRecord {
            status: unit_progress.status.into(),
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

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SavedExerciseResultRecord {
    pub status: AnswerStatusRecord,
    /// The answer shown back to the learner after grading.
    pub answer_text: String,
}

impl From<SavedExerciseResultRecord> for SavedExerciseResult {
    fn from(record: SavedExerciseResultRecord) -> SavedExerciseResult {
        SavedExerciseResult {
            status: record.status.into(),
            answer_text: record.answer_text,
        }
    }
}

impl From<&SavedExerciseResult> for SavedExerciseResultRecord {
    fn from(result: &SavedExerciseResult) -> SavedExerciseResultRecord {
        SavedExerciseResultRecord {
            status: result.status.into(),
            answer_text: result.answer_text.clone(),
        }
    }
}

/// A quiz of one unit, saved after every graded answer so it can be left and continued exactly.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct PausedPracticeRecord {
    /// Indices into the unit's exercises, in the order the practice asks them.
    pub exercise_order: Vec<u32>,
    /// One per answered question, in the order asked.
    pub results: Vec<SavedExerciseResultRecord>,
    /// Equal to `question_count` once every question is answered.
    pub next_question_index: u32,
    pub question_count: u32,
}

impl From<&PausedPractice> for PausedPracticeRecord {
    fn from(paused_practice: &PausedPractice) -> PausedPracticeRecord {
        PausedPracticeRecord {
            exercise_order: paused_practice.exercise_order.iter().map(|exercise_index| *exercise_index as u32).collect(),
            results: paused_practice.results.iter().map(SavedExerciseResultRecord::from).collect(),
            next_question_index: paused_practice.next_question_index() as u32,
            question_count: paused_practice.question_count() as u32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum PausedPracticeStateRecord {
    NotPaused,
    Resumable {
        paused_practice: PausedPracticeRecord,
    },
    /// The unit's exercises differ from the ones the paused practice was started with.
    ExercisesChanged,
}

impl From<PausedPracticeStateKind> for PausedPracticeStateRecord {
    fn from(kind: PausedPracticeStateKind) -> PausedPracticeStateRecord {
        match kind {
            PausedPracticeStateKind::NotPaused => PausedPracticeStateRecord::NotPaused,
            PausedPracticeStateKind::Resumable(paused_practice) => PausedPracticeStateRecord::Resumable {
                paused_practice: (&paused_practice).into(),
            },
            PausedPracticeStateKind::ExercisesChanged => PausedPracticeStateRecord::ExercisesChanged,
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

    pub fn unit_progress(&self, course_id: String, unit_id: String) -> UnitProgressRecord {
        self.lock().course_progress(&course_id).unit_progress(&unit_id).into()
    }

    /// Moves a not started unit to in progress; saves only when it was not started.
    pub fn start_unit(&self, course_id: String, unit_id: String) -> Result<(), CoreError> {
        let status: UnitStatusKind = self.lock().course_progress(&course_id).unit_progress(&unit_id).status;

        if status != UnitStatusKind::NotStarted {
            return Ok(());
        }

        self.change(|progress| progress.course_progress_mut(&course_id).start_unit(&unit_id))
    }

    /// Keeps the best score and the attempt count.
    pub fn set_unit_status(&self, course_id: String, unit_id: String, status: UnitStatusRecord) -> Result<(), CoreError> {
        self.change(|progress| progress.course_progress_mut(&course_id).set_unit_status(&unit_id, status.into()))
    }

    pub fn completed_unit_count(&self, course_id: String) -> u32 {
        self.lock().course_progress(&course_id).completed_unit_count() as u32
    }

    pub fn record_practice_session(
        &self,
        course_id: String,
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
            outcome = Some(progress.record_practice_session(&course_id, &unit_id, mode.into(), score, vocabulary_count as usize, now_milliseconds()));
        })?;

        outcome
            .map(PracticeOutcomeRecord::from)
            .ok_or_else(|| CoreError::arising("the practice session was not recorded".to_string()))
    }

    /// Replaces any paused practice of the unit.
    pub fn start_full_practice(&self, course_id: String, unit_id: String, exercises: Vec<ExerciseRecord>) -> Result<PausedPracticeRecord, CoreError> {
        let exercises: Vec<Exercise> = exercises_of(&exercises);
        let mut paused_practice: Option<PausedPractice> = None;

        self.change(|progress| paused_practice = Some(progress.course_progress_mut(&course_id).start_full_practice(&unit_id, &exercises)))?;

        paused_practice
            .as_ref()
            .map(PausedPracticeRecord::from)
            .ok_or_else(|| CoreError::arising("the quiz was not started".to_string()))
    }

    /// Does nothing when the unit has no paused practice.
    pub fn record_full_practice_result(&self, course_id: String, unit_id: String, result: SavedExerciseResultRecord) -> Result<(), CoreError> {
        self.change(|progress| progress.course_progress_mut(&course_id).record_full_practice_result(&unit_id, result.into()))
    }

    pub fn paused_practice_state(&self, course_id: String, unit_id: String, exercises: Vec<ExerciseRecord>) -> PausedPracticeStateRecord {
        let exercises: Vec<Exercise> = exercises_of(&exercises);

        self.lock().course_progress(&course_id).paused_practice_state(&unit_id, &exercises).into()
    }

    pub fn clear_paused_practice(&self, course_id: String, unit_id: String) -> Result<(), CoreError> {
        self.change(|progress| progress.course_progress_mut(&course_id).clear_paused_practice(&unit_id))
    }

    /// Records the unit's paused practice as a quiz session; `None` while it has unanswered questions or when
    /// the unit has no paused practice.
    pub fn finish_full_practice(&self, course_id: String, unit_id: String, vocabulary_count: u32) -> Result<Option<PracticeOutcomeRecord>, CoreError> {
        let mut outcome: Option<PracticeOutcome> = None;

        self.change(|progress| outcome = progress.finish_full_practice(&course_id, &unit_id, vocabulary_count as usize, now_milliseconds()))?;

        Ok(outcome.map(PracticeOutcomeRecord::from))
    }

    pub fn queue_unit_vocabulary(&self, course_id: String, unit_id: String, vocabulary_count: u32) -> Result<QueuedCardsRecord, CoreError> {
        let card_ids: Vec<String> = (0..vocabulary_count as usize).map(|item_index| shared::course::vocabulary_card_id(&unit_id, item_index)).collect();
        let mut queued: Option<QueuedCards> = None;

        self.change(|progress| queued = Some(progress.course_progress_mut(&course_id).queue_cards_for_review(card_ids.into_iter(), now_milliseconds())))?;

        queued
            .map(QueuedCardsRecord::from)
            .ok_or_else(|| CoreError::arising("the words were not queued".to_string()))
    }

    pub fn due_card_count(&self, course_id: String, now_milliseconds: i64) -> u32 {
        self.lock().course_progress(&course_id).due_card_count(now_milliseconds) as u32
    }

    /// The earliest due time, in any course, of a card that is due later than `now_milliseconds`.
    pub fn next_due_milliseconds_after(&self, now_milliseconds: i64) -> Option<i64> {
        self.lock().next_due_milliseconds_after(now_milliseconds)
    }

    pub fn card_count(&self, course_id: String) -> u32 {
        self.lock().course_progress(&course_id).cards.len() as u32
    }

    /// The ids of up to `limit` of the course's cards due by `now_milliseconds`, in the order they should be reviewed.
    pub fn due_card_ids(&self, course_id: String, limit: u32, now_milliseconds: i64) -> Vec<String> {
        let course_progress: CourseProgress = self.lock().course_progress(&course_id);

        review::select_due_cards(course_progress.cards.values(), now_milliseconds, limit as usize)
            .into_iter()
            .map(|card| card.id.clone())
            .collect()
    }

    /// Reschedules the card as remembered or forgotten.
    pub fn grade_card(&self, course_id: String, card_id: String, remembered: bool) -> Result<(), CoreError> {
        let now: i64 = now_milliseconds();

        self.change(|progress| {
            let course_progress: &mut CourseProgress = progress.course_progress_mut(&course_id);
            let graded: Option<ReviewCard> = course_progress.cards.get(&card_id).map(|card| card.graded(remembered, now));

            if let Some(graded) = graded {
                course_progress.cards.insert(card_id.clone(), graded);
            }
        })
    }

    pub fn count_by_box(&self, course_id: String) -> Vec<u32> {
        review::count_by_box(self.lock().course_progress(&course_id).cards.values()).iter().map(|count| *count as u32).collect()
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

fn exercises_of(exercise_records: &[ExerciseRecord]) -> Vec<Exercise> {
    exercise_records.iter().map(Exercise::from).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const COURSE_ID: &str = "pt_br.en.foundations";

    fn book_in(directory: &tempfile::TempDir) -> Arc<ProgressBook> {
        ProgressBook::open(directory.path().join("progress.json").display().to_string())
    }

    #[test]
    fn changes_are_saved_and_read_back() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();

        let book: Arc<ProgressBook> = book_in(&directory);
        book.record_practice_session(COURSE_ID.to_string(), "u".to_string(), PracticeModeRecord::Full, 9, 10, 4).unwrap();

        let reopened: Arc<ProgressBook> = book_in(&directory);

        assert_eq!(reopened.unit_progress(COURSE_ID.to_string(), "u".to_string()).status, UnitStatusRecord::Completed);
        assert_eq!(reopened.card_count(COURSE_ID.to_string()), 4);
        assert_eq!(reopened.due_card_count(COURSE_ID.to_string(), now_milliseconds()), 4);
        assert_eq!(reopened.card_count("es.en.foundations".to_string()), 0);
    }

    #[test]
    fn start_unit_moves_only_a_not_started_unit_to_in_progress() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let book: Arc<ProgressBook> = book_in(&directory);

        book.start_unit(COURSE_ID.to_string(), "u".to_string()).unwrap();
        book.set_unit_status(COURSE_ID.to_string(), "done".to_string(), UnitStatusRecord::Completed).unwrap();
        book.start_unit(COURSE_ID.to_string(), "done".to_string()).unwrap();

        let reopened: Arc<ProgressBook> = book_in(&directory);

        assert_eq!(reopened.unit_progress(COURSE_ID.to_string(), "u".to_string()).status, UnitStatusRecord::InProgress);
        assert_eq!(reopened.unit_progress(COURSE_ID.to_string(), "done".to_string()).status, UnitStatusRecord::Completed);
        assert_eq!(reopened.unit_progress(COURSE_ID.to_string(), "other".to_string()).status, UnitStatusRecord::NotStarted);
    }

    #[test]
    fn set_unit_status_is_saved_and_keeps_the_score_and_attempts() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let book: Arc<ProgressBook> = book_in(&directory);
        book.record_practice_session(COURSE_ID.to_string(), "u".to_string(), PracticeModeRecord::Full, 9, 10, 4).unwrap();

        book.set_unit_status(COURSE_ID.to_string(), "u".to_string(), UnitStatusRecord::NotStarted).unwrap();

        let reopened: Arc<ProgressBook> = book_in(&directory);

        assert_eq!(
            reopened.unit_progress(COURSE_ID.to_string(), "u".to_string()),
            UnitProgressRecord {
                status: UnitStatusRecord::NotStarted,
                best_proportion: 0.9,
                attempt_count: 1,
            },
        );
        assert_eq!(reopened.completed_unit_count(COURSE_ID.to_string()), 0);
    }

    #[test]
    fn grading_a_remembered_card_takes_it_out_of_the_due_queue() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let book: Arc<ProgressBook> = book_in(&directory);
        book.queue_unit_vocabulary(COURSE_ID.to_string(), "u".to_string(), 2).unwrap();

        let first_due: String = book.due_card_ids(COURSE_ID.to_string(), 10, now_milliseconds())[0].clone();
        book.grade_card(COURSE_ID.to_string(), first_due, true).unwrap();

        assert_eq!(book.due_card_count(COURSE_ID.to_string(), now_milliseconds()), 1);
    }

    #[test]
    fn due_reads_follow_the_time_they_are_given() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let book: Arc<ProgressBook> = book_in(&directory);
        book.queue_unit_vocabulary(COURSE_ID.to_string(), "u".to_string(), 2).unwrap();
        let first_due: String = book.due_card_ids(COURSE_ID.to_string(), 10, now_milliseconds())[0].clone();
        book.grade_card(COURSE_ID.to_string(), first_due.clone(), false).unwrap();

        let graded_milliseconds: i64 = now_milliseconds();
        let next_due_milliseconds: i64 = book.next_due_milliseconds_after(graded_milliseconds).unwrap();

        assert!(next_due_milliseconds > graded_milliseconds);
        assert_eq!(book.due_card_count(COURSE_ID.to_string(), next_due_milliseconds - 1), 1);
        assert_eq!(book.due_card_count(COURSE_ID.to_string(), next_due_milliseconds), 2);
        assert!(book.due_card_ids(COURSE_ID.to_string(), 10, next_due_milliseconds).contains(&first_due));
        assert_eq!(book.next_due_milliseconds_after(next_due_milliseconds), None);
    }

    fn choice_records(prompts: &[&str]) -> Vec<ExerciseRecord> {
        prompts
            .iter()
            .map(|prompt| ExerciseRecord::MultipleChoice {
                prompt: prompt.to_string(),
                choices: vec!["sim".to_string(), "não".to_string()],
                answer_index: 0,
                explanation: None,
            })
            .collect()
    }

    fn result_record(status: AnswerStatusRecord) -> SavedExerciseResultRecord {
        SavedExerciseResultRecord {
            status,
            answer_text: "sim".to_string(),
        }
    }

    #[test]
    fn a_paused_full_practice_is_saved_after_every_result_and_finishes_after_reopening() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let exercises: Vec<ExerciseRecord> = choice_records(&["a", "b"]);
        let book: Arc<ProgressBook> = book_in(&directory);
        book.start_full_practice(COURSE_ID.to_string(), "u".to_string(), exercises.clone()).unwrap();
        book.record_full_practice_result(COURSE_ID.to_string(), "u".to_string(), result_record(AnswerStatusRecord::AccentMistake)).unwrap();

        let reopened: Arc<ProgressBook> = book_in(&directory);
        let state: PausedPracticeStateRecord = reopened.paused_practice_state(COURSE_ID.to_string(), "u".to_string(), exercises.clone());

        assert_eq!(
            state,
            PausedPracticeStateRecord::Resumable {
                paused_practice: PausedPracticeRecord {
                    exercise_order: vec![0, 1],
                    results: vec![result_record(AnswerStatusRecord::AccentMistake)],
                    next_question_index: 1,
                    question_count: 2,
                },
            },
        );

        reopened.record_full_practice_result(COURSE_ID.to_string(), "u".to_string(), result_record(AnswerStatusRecord::Correct)).unwrap();

        let outcome: Option<PracticeOutcomeRecord> = reopened.finish_full_practice(COURSE_ID.to_string(), "u".to_string(), 4).unwrap();

        assert_eq!(outcome, Some(PracticeOutcomeRecord { passed: true, words_added_count: 4 }));
        assert_eq!(book_in(&directory).unit_progress(COURSE_ID.to_string(), "u".to_string()).status, UnitStatusRecord::Completed);
        assert_eq!(
            book_in(&directory).paused_practice_state(COURSE_ID.to_string(), "u".to_string(), exercises),
            PausedPracticeStateRecord::NotPaused,
        );
    }

    #[test]
    fn a_paused_full_practice_of_changed_exercises_is_reported_and_can_be_cleared() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let book: Arc<ProgressBook> = book_in(&directory);
        book.start_full_practice(COURSE_ID.to_string(), "u".to_string(), choice_records(&["a", "b"])).unwrap();

        let changed_exercises: Vec<ExerciseRecord> = choice_records(&["a", "rewritten"]);
        let state: PausedPracticeStateRecord = book.paused_practice_state(COURSE_ID.to_string(), "u".to_string(), changed_exercises.clone());

        assert_eq!(state, PausedPracticeStateRecord::ExercisesChanged);

        book.clear_paused_practice(COURSE_ID.to_string(), "u".to_string()).unwrap();

        assert_eq!(
            book_in(&directory).paused_practice_state(COURSE_ID.to_string(), "u".to_string(), changed_exercises),
            PausedPracticeStateRecord::NotPaused,
        );
    }

    #[test]
    fn an_unreadable_file_is_kept_aside_and_replaced() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        fs::write(directory.path().join("progress.json"), "not json").unwrap();

        let book: Arc<ProgressBook> = book_in(&directory);

        assert_eq!(book.card_count(COURSE_ID.to_string()), 0);
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
            last_course_id: Some(COURSE_ID.to_string()),
        };

        book.update_settings(settings.clone()).unwrap();

        assert_eq!(book_in(&directory).settings(), settings);
    }
}
