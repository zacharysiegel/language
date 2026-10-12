use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::course_shard::VoiceEngineKind;
use crate::practice::{PASS_PROPORTION, PracticeModeKind, SessionScore};
use crate::review::ReviewCard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDirectionKind {
    TargetToSource,
    SourceToTarget,
    Mixed,
}

/// Which recorded voice set plays. `Auto` uses the macOS voice for one- and two-word clips and the neural voice for longer ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineKind {
    Auto,
    Kokoro,
    MacosSay,
}

const AUTO_SHORT_CLIP_WORD_LIMIT: usize = 2;

impl AudioEngineKind {
    pub fn voice_engine_for(self, text: &str) -> VoiceEngineKind {
        self.voice_engine_for_longest_text(word_count(text))
    }

    /// One recording set for every line of a dialogue; `Auto` picks the neural set when any line is longer than a short clip.
    pub fn voice_engine_for_dialogue<'a>(self, line_texts: impl IntoIterator<Item = &'a str>) -> VoiceEngineKind {
        let longest_line_word_count: usize = line_texts.into_iter().map(word_count).max().unwrap_or(0);

        self.voice_engine_for_longest_text(longest_line_word_count)
    }

    pub fn voice_engine_for_clip(self, spoken_text: &str, voice_choice: &VoiceChoiceKind) -> VoiceEngineKind {
        match voice_choice {
            VoiceChoiceKind::OwnText => self.voice_engine_for(spoken_text),
            VoiceChoiceKind::WholeDialogue { line_texts } => self.voice_engine_for_dialogue(line_texts.iter().map(String::as_str)),
        }
    }

    fn voice_engine_for_longest_text(self, longest_word_count: usize) -> VoiceEngineKind {
        match self {
            AudioEngineKind::Kokoro => VoiceEngineKind::Kokoro,
            AudioEngineKind::MacosSay => VoiceEngineKind::MacosSay,
            AudioEngineKind::Auto if longest_word_count <= AUTO_SHORT_CLIP_WORD_LIMIT => VoiceEngineKind::MacosSay,
            AudioEngineKind::Auto => VoiceEngineKind::Kokoro,
        }
    }
}

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Which texts decide the recording set a clip plays from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum VoiceChoiceKind {
    #[default]
    OwnText,
    WholeDialogue {
        line_texts: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_show_translations")]
    pub show_translations: bool,
    #[serde(default)]
    pub slow_audio: bool,
    #[serde(default = "default_review_direction")]
    pub review_direction: ReviewDirectionKind,
    #[serde(default = "default_audio_engine")]
    pub audio_engine: AudioEngineKind,
    /// The course the learner opened last.
    #[serde(default)]
    pub last_course_id: Option<String>,
}

fn default_audio_engine() -> AudioEngineKind {
    AudioEngineKind::Auto
}

fn default_show_translations() -> bool {
    true
}

fn default_review_direction() -> ReviewDirectionKind {
    ReviewDirectionKind::TargetToSource
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            show_translations: default_show_translations(),
            slow_audio: false,
            review_direction: default_review_direction(),
            audio_engine: default_audio_engine(),
            last_course_id: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitStatusKind {
    #[default]
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(from = "StoredUnitProgress")]
pub struct UnitProgress {
    pub status: UnitStatusKind,
    pub best_proportion: f64,
    pub attempt_count: u32,
}

impl UnitProgress {
    pub fn is_completed(&self) -> bool {
        self.status == UnitStatusKind::Completed
    }
}

/// A unit record as saved; records saved before `status` existed carry `started` and `completed` instead.
#[derive(Deserialize)]
struct StoredUnitProgress {
    #[serde(default)]
    status: Option<UnitStatusKind>,
    #[serde(default)]
    started: bool,
    #[serde(default)]
    completed: bool,
    #[serde(default)]
    best_proportion: f64,
    #[serde(default)]
    attempt_count: u32,
}

impl From<StoredUnitProgress> for UnitProgress {
    fn from(stored: StoredUnitProgress) -> UnitProgress {
        let status: UnitStatusKind = match stored.status {
            Some(status) => status,
            None if stored.completed => UnitStatusKind::Completed,
            None if stored.started || stored.attempt_count > 0 => UnitStatusKind::InProgress,
            None => UnitStatusKind::NotStarted,
        };

        UnitProgress {
            status,
            best_proportion: stored.best_proportion,
            attempt_count: stored.attempt_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PracticeOutcome {
    pub passed: bool,
    pub words_added_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuedCards {
    pub added_count: usize,
    pub made_due_count: usize,
}

/// One course's review deck and unit results.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CourseProgress {
    #[serde(default)]
    pub cards: BTreeMap<String, ReviewCard>,
    #[serde(default)]
    pub units: BTreeMap<String, UnitProgress>,
}

impl CourseProgress {
    pub fn unit_progress(&self, unit_id: &str) -> UnitProgress {
        self.units.get(unit_id).cloned().unwrap_or_default()
    }

    /// A pass completes the unit; any other attempt leaves a completed unit completed and an unfinished one in progress.
    pub fn record_unit_attempt(&mut self, unit_id: &str, proportion: f64, pass_proportion: f64) {
        let previous: UnitProgress = self.unit_progress(unit_id);
        let status: UnitStatusKind = if previous.is_completed() || proportion >= pass_proportion {
            UnitStatusKind::Completed
        } else {
            UnitStatusKind::InProgress
        };

        self.units.insert(
            unit_id.to_string(),
            UnitProgress {
                status,
                best_proportion: previous.best_proportion.max(proportion),
                attempt_count: previous.attempt_count + 1,
            },
        );
    }

    /// Moves a not started unit to in progress; leaves any other status as it is.
    pub fn start_unit(&mut self, unit_id: &str) {
        let unit_progress: &mut UnitProgress = self.units.entry(unit_id.to_string()).or_default();

        if unit_progress.status == UnitStatusKind::NotStarted {
            unit_progress.status = UnitStatusKind::InProgress;
        }
    }

    /// Keeps the best score and the attempt count.
    pub fn set_unit_status(&mut self, unit_id: &str, status: UnitStatusKind) {
        self.units.entry(unit_id.to_string()).or_default().status = status;
    }

    pub fn add_cards_if_missing(&mut self, card_ids: impl Iterator<Item = String>, now_milliseconds: i64) -> usize {
        let mut added_count: usize = 0;

        for card_id in card_ids {
            if !self.cards.contains_key(&card_id) {
                self.cards.insert(card_id.clone(), ReviewCard::new(card_id, now_milliseconds));
                added_count += 1;
            }
        }

        added_count
    }

    /// Adds missing cards and makes cards that are scheduled for later due now, keeping their box.
    pub fn queue_cards_for_review(&mut self, card_ids: impl Iterator<Item = String>, now_milliseconds: i64) -> QueuedCards {
        let mut queued: QueuedCards = QueuedCards {
            added_count: 0,
            made_due_count: 0,
        };

        for card_id in card_ids {
            match self.cards.get_mut(&card_id) {
                Some(card) if card.due_milliseconds > now_milliseconds => {
                    card.due_milliseconds = now_milliseconds;
                    queued.made_due_count += 1;
                },
                Some(_) => {},
                None => {
                    self.cards.insert(card_id.clone(), ReviewCard::new(card_id, now_milliseconds));
                    queued.added_count += 1;
                },
            }
        }

        queued
    }

    pub fn completed_unit_count(&self) -> usize {
        self.units.values().filter(|unit_progress| unit_progress.is_completed()).count()
    }

    pub fn due_card_count(&self, now_milliseconds: i64) -> usize {
        self.cards.values().filter(|card| card.due_milliseconds <= now_milliseconds).count()
    }

    /// The earliest due time of the course's cards that is later than `now_milliseconds`; `None` when no card is due later.
    pub fn next_due_milliseconds_after(&self, now_milliseconds: i64) -> Option<i64> {
        self.cards
            .values()
            .map(|card| card.due_milliseconds)
            .filter(|due_milliseconds| *due_milliseconds > now_milliseconds)
            .min()
    }
}

/// Everything the learner has done, in every course.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    /// Each course's progress, by course id.
    #[serde(default)]
    pub courses: BTreeMap<String, CourseProgress>,
    #[serde(default)]
    pub settings: Settings,
}

impl Progress {
    pub fn from_json(text: &str) -> Result<Progress, AppError> {
        let progress: Progress = serde_json::from_str(text)?;

        Ok(progress)
    }

    pub fn to_json(&self) -> Result<String, AppError> {
        let text: String = serde_json::to_string_pretty(self)?;

        Ok(text)
    }

    /// The course's progress; empty for a course the learner has not started.
    pub fn course_progress(&self, course_id: &str) -> CourseProgress {
        self.courses.get(course_id).cloned().unwrap_or_default()
    }

    pub fn course_progress_mut(&mut self, course_id: &str) -> &mut CourseProgress {
        self.courses.entry(course_id.to_string()).or_default()
    }

    /// The earliest due time, in any course, of a card that is due later than `now_milliseconds`.
    pub fn next_due_milliseconds_after(&self, now_milliseconds: i64) -> Option<i64> {
        self.courses
            .values()
            .filter_map(|course_progress| course_progress.next_due_milliseconds_after(now_milliseconds))
            .min()
    }

    /// A full practice records an attempt at the unit and, when passed, adds the unit's words to the course's review
    /// deck; a quick practice only starts the unit.
    pub fn record_practice_session(
        &mut self,
        course_id: &str,
        unit_id: &str,
        mode: PracticeModeKind,
        score: SessionScore,
        vocabulary_count: usize,
        now_milliseconds: i64,
    ) -> PracticeOutcome {
        if mode == PracticeModeKind::Quick {
            self.course_progress_mut(course_id).start_unit(unit_id);

            return PracticeOutcome {
                passed: score.passed(),
                words_added_count: 0,
            };
        }

        let course_progress: &mut CourseProgress = self.course_progress_mut(course_id);

        course_progress.record_unit_attempt(unit_id, score.proportion(), PASS_PROPORTION);

        let words_added_count: usize = if score.passed() {
            let card_ids = (0..vocabulary_count).map(|item_index| crate::course::vocabulary_card_id(unit_id, item_index));

            course_progress.add_cards_if_missing(card_ids, now_milliseconds)
        } else {
            0
        };

        PracticeOutcome {
            passed: score.passed(),
            words_added_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COURSE_ID: &str = "pt_br.en.foundations";

    fn unit_status_after(changes: impl FnOnce(&mut CourseProgress)) -> UnitStatusKind {
        let mut course_progress: CourseProgress = CourseProgress::default();

        changes(&mut course_progress);

        course_progress.unit_progress("u").status
    }

    fn unit_progress_from_json(text: &str) -> UnitProgress {
        let progress: Progress = Progress::from_json(&format!(r#"{{"courses": {{"{COURSE_ID}": {{"units": {{"u": {text}}}}}}}}}"#)).unwrap();

        progress.course_progress(COURSE_ID).unit_progress("u")
    }

    #[test]
    fn record_unit_attempt_keeps_best_proportion_and_completion() {
        let mut course_progress: CourseProgress = CourseProgress::default();

        course_progress.record_unit_attempt("u", 0.9, 0.8);
        course_progress.record_unit_attempt("u", 0.5, 0.8);

        let unit_progress: UnitProgress = course_progress.unit_progress("u");

        assert_eq!(unit_progress.status, UnitStatusKind::Completed);
        assert_eq!(unit_progress.best_proportion, 0.9);
        assert_eq!(unit_progress.attempt_count, 2);
    }

    #[test]
    fn record_unit_attempt_below_the_pass_mark_leaves_the_unit_in_progress() {
        let from_not_started: UnitStatusKind = unit_status_after(|course_progress| course_progress.record_unit_attempt("u", 0.5, 0.8));
        let from_in_progress: UnitStatusKind = unit_status_after(|course_progress| {
            course_progress.start_unit("u");
            course_progress.record_unit_attempt("u", 0.5, 0.8);
        });

        assert_eq!(from_not_started, UnitStatusKind::InProgress);
        assert_eq!(from_in_progress, UnitStatusKind::InProgress);
    }

    #[test]
    fn record_unit_attempt_at_the_pass_mark_completes_the_unit() {
        let status: UnitStatusKind = unit_status_after(|course_progress| course_progress.record_unit_attempt("u", 0.8, 0.8));

        assert_eq!(status, UnitStatusKind::Completed);
    }

    #[test]
    fn a_unit_is_not_started_until_it_is_started() {
        let course_progress: CourseProgress = CourseProgress::default();

        assert_eq!(course_progress.unit_progress("u").status, UnitStatusKind::NotStarted);
    }

    #[test]
    fn start_unit_moves_only_a_not_started_unit_to_in_progress() {
        let from_not_started: UnitStatusKind = unit_status_after(|course_progress| course_progress.start_unit("u"));
        let from_completed: UnitStatusKind = unit_status_after(|course_progress| {
            course_progress.set_unit_status("u", UnitStatusKind::Completed);
            course_progress.start_unit("u");
        });

        assert_eq!(from_not_started, UnitStatusKind::InProgress);
        assert_eq!(from_completed, UnitStatusKind::Completed);
    }

    #[test]
    fn set_unit_status_sets_any_status_and_keeps_the_score_and_attempts() {
        let mut course_progress: CourseProgress = CourseProgress::default();
        course_progress.record_unit_attempt("u", 0.9, 0.8);
        course_progress.record_unit_attempt("u", 0.5, 0.8);

        for status in [UnitStatusKind::NotStarted, UnitStatusKind::InProgress, UnitStatusKind::Completed, UnitStatusKind::NotStarted] {
            course_progress.set_unit_status("u", status);

            let unit_progress: UnitProgress = course_progress.unit_progress("u");

            assert_eq!(unit_progress.status, status);
            assert_eq!(unit_progress.best_proportion, 0.9);
            assert_eq!(unit_progress.attempt_count, 2);
        }
    }

    #[test]
    fn completed_unit_count_counts_completed_units_only() {
        let mut course_progress: CourseProgress = CourseProgress::default();
        course_progress.record_unit_attempt("passed", 0.9, 0.8);
        course_progress.record_unit_attempt("failed", 0.5, 0.8);
        course_progress.start_unit("started");
        course_progress.set_unit_status("marked", UnitStatusKind::Completed);
        course_progress.record_unit_attempt("reset", 0.9, 0.8);
        course_progress.set_unit_status("reset", UnitStatusKind::NotStarted);

        assert_eq!(course_progress.completed_unit_count(), 2);
    }

    #[test]
    fn unit_status_reads_from_legacy_flags() {
        assert_eq!(unit_progress_from_json(r#"{"completed": true, "started": true}"#).status, UnitStatusKind::Completed);
        assert_eq!(unit_progress_from_json(r#"{"completed": true}"#).status, UnitStatusKind::Completed);
        assert_eq!(unit_progress_from_json(r#"{"started": true, "completed": false}"#).status, UnitStatusKind::InProgress);
        assert_eq!(unit_progress_from_json(r#"{"best_proportion": 0.5, "attempt_count": 1}"#).status, UnitStatusKind::InProgress);
        assert_eq!(unit_progress_from_json(r#"{"started": false, "completed": false}"#).status, UnitStatusKind::NotStarted);
        assert_eq!(unit_progress_from_json("{}").status, UnitStatusKind::NotStarted);
    }

    #[test]
    fn legacy_unit_records_keep_their_score_and_attempts() {
        let unit_progress: UnitProgress = unit_progress_from_json(r#"{"completed": true, "best_proportion": 0.9, "attempt_count": 3}"#);

        assert_eq!(
            unit_progress,
            UnitProgress {
                status: UnitStatusKind::Completed,
                best_proportion: 0.9,
                attempt_count: 3,
            },
        );
    }

    #[test]
    fn a_stored_status_takes_precedence_over_legacy_flags() {
        assert_eq!(unit_progress_from_json(r#"{"status": "not_started", "completed": true}"#).status, UnitStatusKind::NotStarted);
    }

    #[test]
    fn unit_progress_round_trips_through_json() {
        let mut progress: Progress = Progress::default();
        progress.course_progress_mut(COURSE_ID).record_unit_attempt("u", 0.5, 0.8);
        progress.course_progress_mut(COURSE_ID).set_unit_status("v", UnitStatusKind::Completed);

        let text: String = progress.to_json().unwrap();
        let read_back: Progress = Progress::from_json(&text).unwrap();

        assert_eq!(read_back, progress);
        assert!(!text.contains("\"started\":"));
        assert!(!text.contains("\"completed\":"));
    }

    #[test]
    fn add_cards_if_missing_does_not_reset_existing_cards() {
        let mut course_progress: CourseProgress = CourseProgress::default();
        course_progress.add_cards_if_missing(["a:0".to_string()].into_iter(), 0);
        course_progress.cards.get_mut("a:0").unwrap().box_index = 3;

        let added_count: usize = course_progress.add_cards_if_missing(["a:0".to_string(), "a:1".to_string()].into_iter(), 10);

        assert_eq!(added_count, 1);
        assert_eq!(course_progress.cards["a:0"].box_index, 3);
    }

    #[test]
    fn queue_cards_for_review_makes_scheduled_cards_due_and_keeps_their_box() {
        let mut course_progress: CourseProgress = CourseProgress::default();
        course_progress.add_cards_if_missing(["a:0".to_string(), "a:1".to_string()].into_iter(), 0);
        let remembered: ReviewCard = course_progress.cards["a:0"].graded(true, 1_000);
        course_progress.cards.insert("a:0".to_string(), remembered);

        let queued: QueuedCards =
            course_progress.queue_cards_for_review(["a:0".to_string(), "a:1".to_string(), "a:2".to_string()].into_iter(), 2_000);

        assert_eq!(queued, QueuedCards { added_count: 1, made_due_count: 1 });
        assert_eq!(course_progress.cards["a:0"].due_milliseconds, 2_000);
        assert_eq!(course_progress.cards["a:0"].box_index, 1);
        assert_eq!(course_progress.due_card_count(2_000), 3);
    }

    fn course_progress_with_cards_due_at(due_milliseconds: &[i64]) -> CourseProgress {
        let mut course_progress: CourseProgress = CourseProgress::default();

        for (card_index, due) in due_milliseconds.iter().enumerate() {
            let card_id: String = format!("a:{card_index}");

            course_progress.cards.insert(card_id.clone(), ReviewCard::new(card_id, *due));
        }

        course_progress
    }

    #[test]
    fn next_due_milliseconds_after_is_the_earliest_card_due_later_than_now() {
        let course_progress: CourseProgress = course_progress_with_cards_due_at(&[500, 1_000, 3_000, 2_000]);

        assert_eq!(course_progress.next_due_milliseconds_after(1_000), Some(2_000));
        assert_eq!(course_progress.next_due_milliseconds_after(999), Some(1_000));
        assert_eq!(course_progress.next_due_milliseconds_after(0), Some(500));
    }

    #[test]
    fn next_due_milliseconds_after_is_none_when_every_card_is_due_by_now() {
        let course_progress: CourseProgress = course_progress_with_cards_due_at(&[500, 1_000]);

        assert_eq!(course_progress.next_due_milliseconds_after(1_000), None);
        assert_eq!(CourseProgress::default().next_due_milliseconds_after(0), None);
    }

    #[test]
    fn progress_next_due_milliseconds_after_is_the_earliest_across_courses() {
        let mut progress: Progress = Progress::default();
        progress.courses.insert(COURSE_ID.to_string(), course_progress_with_cards_due_at(&[5_000, 100]));
        progress.courses.insert("es.en.foundations".to_string(), course_progress_with_cards_due_at(&[3_000]));
        progress.courses.insert("fr.en.foundations".to_string(), CourseProgress::default());

        assert_eq!(progress.next_due_milliseconds_after(1_000), Some(3_000));
        assert_eq!(progress.next_due_milliseconds_after(5_000), None);
    }

    #[test]
    fn record_practice_session_adds_the_words_of_a_passed_full_practice_to_that_course_only() {
        let mut progress: Progress = Progress::default();
        let passing: SessionScore = SessionScore { correct_count: 9, total_count: 10 };
        let failing: SessionScore = SessionScore { correct_count: 2, total_count: 10 };

        let quick: PracticeOutcome = progress.record_practice_session(COURSE_ID, "u", PracticeModeKind::Quick, passing, 3, 0);
        let failed: PracticeOutcome = progress.record_practice_session(COURSE_ID, "u", PracticeModeKind::Full, failing, 3, 0);
        let passed: PracticeOutcome = progress.record_practice_session(COURSE_ID, "u", PracticeModeKind::Full, passing, 3, 0);

        assert_eq!(quick.words_added_count, 0);
        assert_eq!(failed, PracticeOutcome { passed: false, words_added_count: 0 });
        assert_eq!(passed, PracticeOutcome { passed: true, words_added_count: 3 });
        assert_eq!(progress.course_progress(COURSE_ID).unit_progress("u").attempt_count, 2);
        assert!(progress.course_progress("es.en.foundations").cards.is_empty());
    }

    #[test]
    fn record_practice_session_starts_the_unit_and_never_downgrades_a_completed_one() {
        let mut progress: Progress = Progress::default();
        let passing: SessionScore = SessionScore { correct_count: 9, total_count: 10 };
        let failing: SessionScore = SessionScore { correct_count: 2, total_count: 10 };

        progress.record_practice_session(COURSE_ID, "quick", PracticeModeKind::Quick, failing, 3, 0);
        progress.record_practice_session(COURSE_ID, "failed", PracticeModeKind::Full, failing, 3, 0);
        progress.record_practice_session(COURSE_ID, "passed", PracticeModeKind::Full, passing, 3, 0);
        progress.record_practice_session(COURSE_ID, "passed", PracticeModeKind::Full, failing, 3, 0);
        progress.record_practice_session(COURSE_ID, "passed", PracticeModeKind::Quick, failing, 3, 0);

        let course_progress: CourseProgress = progress.course_progress(COURSE_ID);

        assert_eq!(course_progress.unit_progress("quick").status, UnitStatusKind::InProgress);
        assert_eq!(course_progress.unit_progress("quick").attempt_count, 0);
        assert_eq!(course_progress.unit_progress("failed").status, UnitStatusKind::InProgress);
        assert_eq!(course_progress.unit_progress("passed").status, UnitStatusKind::Completed);
    }

    #[test]
    fn auto_engine_uses_the_macos_voice_for_short_clips_only() {
        assert_eq!(AudioEngineKind::Auto.voice_engine_for("o pé"), VoiceEngineKind::MacosSay);
        assert_eq!(AudioEngineKind::Auto.voice_engine_for("Oi, tudo bem?"), VoiceEngineKind::Kokoro);
        assert_eq!(AudioEngineKind::Kokoro.voice_engine_for("o"), VoiceEngineKind::Kokoro);
        assert_eq!(AudioEngineKind::MacosSay.voice_engine_for("Eu moro aqui agora."), VoiceEngineKind::MacosSay);
    }

    #[test]
    fn auto_engine_uses_the_neural_voice_for_a_whole_dialogue_with_any_long_line() {
        let line_texts: [&str; 3] = ["Oi!", "Oi, tudo bem?", "Tudo."];

        assert_eq!(AudioEngineKind::Auto.voice_engine_for_dialogue(line_texts), VoiceEngineKind::Kokoro);
    }

    #[test]
    fn auto_engine_uses_the_macos_voice_for_a_whole_dialogue_of_short_lines() {
        let line_texts: [&str; 3] = ["Oi!", "Tudo bem?", "Tudo."];

        assert_eq!(AudioEngineKind::Auto.voice_engine_for_dialogue(line_texts), VoiceEngineKind::MacosSay);
    }

    #[test]
    fn explicit_engine_is_kept_for_a_whole_dialogue() {
        let line_texts: [&str; 2] = ["Oi!", "Eu moro aqui agora."];

        assert_eq!(AudioEngineKind::Kokoro.voice_engine_for_dialogue(line_texts), VoiceEngineKind::Kokoro);
        assert_eq!(AudioEngineKind::MacosSay.voice_engine_for_dialogue(line_texts), VoiceEngineKind::MacosSay);
    }

    #[test]
    fn voice_engine_for_clip_follows_its_dialogue_or_its_own_text() {
        let whole_dialogue: VoiceChoiceKind = VoiceChoiceKind::WholeDialogue {
            line_texts: vec!["Oi!".to_string(), "Eu moro aqui agora.".to_string()],
        };

        assert_eq!(AudioEngineKind::Auto.voice_engine_for_clip("Oi!", &whole_dialogue), VoiceEngineKind::Kokoro);
        assert_eq!(AudioEngineKind::Auto.voice_engine_for_clip("Oi!", &VoiceChoiceKind::OwnText), VoiceEngineKind::MacosSay);
    }

    #[test]
    fn from_json_fills_missing_fields_with_defaults() {
        let progress: Progress = Progress::from_json("{}").unwrap();

        assert!(progress.settings.show_translations);
        assert!(progress.courses.is_empty());
        assert_eq!(progress.settings.last_course_id, None);
    }
}
