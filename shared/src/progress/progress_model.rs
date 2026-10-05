use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::review::ReviewCard;

const DATE_KEY_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDirectionKind {
    PortugueseToEnglish,
    EnglishToPortuguese,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_show_translations")]
    pub show_translations: bool,
    #[serde(default)]
    pub slow_audio: bool,
    #[serde(default = "default_review_direction")]
    pub review_direction: ReviewDirectionKind,
}

fn default_show_translations() -> bool {
    true
}

fn default_review_direction() -> ReviewDirectionKind {
    ReviewDirectionKind::PortugueseToEnglish
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            show_translations: default_show_translations(),
            slow_audio: false,
            review_direction: default_review_direction(),
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DailyActivity {
    #[serde(default)]
    pub exercise_count: u32,
    #[serde(default)]
    pub review_count: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    #[serde(default)]
    pub cards: BTreeMap<String, ReviewCard>,
    #[serde(default)]
    pub units: BTreeMap<String, UnitProgress>,
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub activity: BTreeMap<String, DailyActivity>,
}

pub fn date_key(date: NaiveDate) -> String {
    date.format(DATE_KEY_FORMAT).to_string()
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

    pub fn record_exercise_activity(&mut self, today: NaiveDate, exercise_count: u32) {
        self.activity.entry(date_key(today)).or_default().exercise_count += exercise_count;
    }

    pub fn record_review_activity(&mut self, today: NaiveDate, review_count: u32) {
        self.activity.entry(date_key(today)).or_default().review_count += review_count;
    }

    pub fn completed_unit_count(&self) -> usize {
        self.units.values().filter(|unit_progress| unit_progress.completed).count()
    }

    pub fn streak_days(&self, today: NaiveDate) -> u32 {
        let today_active: bool = self.activity.contains_key(&date_key(today));
        let mut cursor: Option<NaiveDate> = if today_active { Some(today) } else { today.pred_opt() };
        let mut streak: u32 = 0;

        while let Some(date) = cursor {
            if !self.activity.contains_key(&date_key(date)) {
                break;
            }

            streak += 1;
            cursor = date.pred_opt();
        }

        streak
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn record_unit_attempt_keeps_best_proportion_and_completion() {
        let mut progress: Progress = Progress::default();

        progress.record_unit_attempt("u", 0.9, 0.8);
        progress.record_unit_attempt("u", 0.5, 0.8);

        let unit_progress: UnitProgress = progress.unit_progress("u");

        assert!(unit_progress.completed);
        assert_eq!(unit_progress.best_proportion, 0.9);
        assert_eq!(unit_progress.attempt_count, 2);
    }

    #[test]
    fn add_cards_if_missing_does_not_reset_existing_cards() {
        let mut progress: Progress = Progress::default();
        progress.add_cards_if_missing(["a:0".to_string()].into_iter(), 0);
        progress.cards.get_mut("a:0").unwrap().box_index = 3;

        let added_count: usize = progress.add_cards_if_missing(["a:0".to_string(), "a:1".to_string()].into_iter(), 10);

        assert_eq!(added_count, 1);
        assert_eq!(progress.cards["a:0"].box_index, 3);
    }

    #[test]
    fn streak_days_counts_consecutive_days_ending_today_or_yesterday() {
        let mut progress: Progress = Progress::default();
        progress.record_exercise_activity(date(2026, 10, 3), 5);
        progress.record_review_activity(date(2026, 10, 4), 5);

        assert_eq!(progress.streak_days(date(2026, 10, 4)), 2);
        assert_eq!(progress.streak_days(date(2026, 10, 5)), 2);
        assert_eq!(progress.streak_days(date(2026, 10, 6)), 0);
    }

    #[test]
    fn from_json_fills_missing_fields_with_defaults() {
        let progress: Progress = Progress::from_json("{}").unwrap();

        assert!(progress.settings.show_translations);
        assert!(progress.cards.is_empty());
    }
}
