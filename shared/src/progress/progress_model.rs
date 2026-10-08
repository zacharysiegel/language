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
        match self {
            AudioEngineKind::Kokoro => VoiceEngineKind::Kokoro,
            AudioEngineKind::MacosSay => VoiceEngineKind::MacosSay,
            AudioEngineKind::Auto if text.split_whitespace().count() <= AUTO_SHORT_CLIP_WORD_LIMIT => VoiceEngineKind::MacosSay,
            AudioEngineKind::Auto => VoiceEngineKind::Kokoro,
        }
    }
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UnitProgress {
    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub best_proportion: f64,
    #[serde(default)]
    pub attempt_count: u32,
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

    pub fn record_unit_attempt(&mut self, unit_id: &str, proportion: f64, pass_proportion: f64) {
        let previous: UnitProgress = self.unit_progress(unit_id);

        self.units.insert(
            unit_id.to_string(),
            UnitProgress {
                completed: previous.completed || proportion >= pass_proportion,
                best_proportion: previous.best_proportion.max(proportion),
                attempt_count: previous.attempt_count + 1,
            },
        );
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
        self.units.values().filter(|unit_progress| unit_progress.completed).count()
    }

    pub fn due_card_count(&self, now_milliseconds: i64) -> usize {
        self.cards.values().filter(|card| card.due_milliseconds <= now_milliseconds).count()
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

    /// A full practice records an attempt at the unit and, when passed, adds the unit's words to the course's review
    /// deck; a quick practice records nothing.
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

    #[test]
    fn record_unit_attempt_keeps_best_proportion_and_completion() {
        let mut course_progress: CourseProgress = CourseProgress::default();

        course_progress.record_unit_attempt("u", 0.9, 0.8);
        course_progress.record_unit_attempt("u", 0.5, 0.8);

        let unit_progress: UnitProgress = course_progress.unit_progress("u");

        assert!(unit_progress.completed);
        assert_eq!(unit_progress.best_proportion, 0.9);
        assert_eq!(unit_progress.attempt_count, 2);
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
    fn auto_engine_uses_the_macos_voice_for_short_clips_only() {
        assert_eq!(AudioEngineKind::Auto.voice_engine_for("o pé"), VoiceEngineKind::MacosSay);
        assert_eq!(AudioEngineKind::Auto.voice_engine_for("Oi, tudo bem?"), VoiceEngineKind::Kokoro);
        assert_eq!(AudioEngineKind::Kokoro.voice_engine_for("o"), VoiceEngineKind::Kokoro);
        assert_eq!(AudioEngineKind::MacosSay.voice_engine_for("Eu moro aqui agora."), VoiceEngineKind::MacosSay);
    }

    #[test]
    fn from_json_fills_missing_fields_with_defaults() {
        let progress: Progress = Progress::from_json("{}").unwrap();

        assert!(progress.settings.show_translations);
        assert!(progress.courses.is_empty());
        assert_eq!(progress.settings.last_course_id, None);
    }
}
