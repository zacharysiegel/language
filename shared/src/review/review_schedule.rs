use serde::{Deserialize, Serialize};

pub const REVIEW_INTERVAL_DAYS: [u32; 7] = [0, 1, 3, 7, 14, 30, 90];

const DAY_MILLISECONDS: i64 = 24 * 60 * 60 * 1000;
const FORGOTTEN_RETRY_DELAY_MILLISECONDS: i64 = 60 * 1000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewCard {
    pub id: String,
    pub box_index: usize,
    pub due_milliseconds: i64,
    pub lapse_count: u32,
    pub review_count: u32,
}

impl ReviewCard {
    pub fn new(id: String, now_milliseconds: i64) -> ReviewCard {
        ReviewCard {
            id,
            box_index: 0,
            due_milliseconds: now_milliseconds,
            lapse_count: 0,
            review_count: 0,
        }
    }

    pub fn graded(&self, remembered: bool, now_milliseconds: i64) -> ReviewCard {
        let last_box_index: usize = REVIEW_INTERVAL_DAYS.len() - 1;
        let next_box_index: usize = if remembered { (self.box_index + 1).min(last_box_index) } else { 0 };
        let delay_milliseconds: i64 = if remembered {
            i64::from(REVIEW_INTERVAL_DAYS[next_box_index]) * DAY_MILLISECONDS
        } else {
            FORGOTTEN_RETRY_DELAY_MILLISECONDS
        };

        ReviewCard {
            id: self.id.clone(),
            box_index: next_box_index,
            due_milliseconds: now_milliseconds + delay_milliseconds,
            lapse_count: self.lapse_count + u32::from(!remembered),
            review_count: self.review_count + 1,
        }
    }
}

pub fn select_due_cards<'a>(
    cards: impl Iterator<Item = &'a ReviewCard>,
    now_milliseconds: i64,
    limit: usize,
) -> Vec<&'a ReviewCard> {
    let mut due_cards: Vec<&ReviewCard> = cards.filter(|card| card.due_milliseconds <= now_milliseconds).collect();

    due_cards.sort_by_key(|card| (card.box_index, card.due_milliseconds));
    due_cards.truncate(limit);

    due_cards
}

pub fn count_by_box<'a>(cards: impl Iterator<Item = &'a ReviewCard>) -> [usize; REVIEW_INTERVAL_DAYS.len()] {
    let mut counts: [usize; REVIEW_INTERVAL_DAYS.len()] = [0; REVIEW_INTERVAL_DAYS.len()];

    for card in cards {
        counts[card.box_index] += 1;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_000_000;

    #[test]
    fn graded_remembered_advances_box_and_schedules_by_interval() {
        let card: ReviewCard = ReviewCard::new("a:0".to_string(), NOW);
        let next: ReviewCard = card.graded(true, NOW);

        assert_eq!(next.box_index, 1);
        assert_eq!(next.due_milliseconds, NOW + DAY_MILLISECONDS);
        assert_eq!(next.review_count, 1);
        assert_eq!(next.lapse_count, 0);
    }

    #[test]
    fn graded_remembered_stops_at_last_box() {
        let mut card: ReviewCard = ReviewCard::new("a:0".to_string(), NOW);
        card.box_index = REVIEW_INTERVAL_DAYS.len() - 1;

        assert_eq!(card.graded(true, NOW).box_index, REVIEW_INTERVAL_DAYS.len() - 1);
    }

    #[test]
    fn graded_forgotten_resets_box_and_counts_lapse() {
        let mut card: ReviewCard = ReviewCard::new("a:0".to_string(), NOW);
        card.box_index = 4;
        let next: ReviewCard = card.graded(false, NOW);

        assert_eq!(next.box_index, 0);
        assert_eq!(next.due_milliseconds, NOW + FORGOTTEN_RETRY_DELAY_MILLISECONDS);
        assert_eq!(next.lapse_count, 1);
    }

    #[test]
    fn select_due_cards_orders_by_box_then_due_and_respects_limit() {
        let mut low_box_late: ReviewCard = ReviewCard::new("low-late".to_string(), NOW - 10);
        let mut low_box_early: ReviewCard = ReviewCard::new("low-early".to_string(), NOW - 20);
        let mut high_box: ReviewCard = ReviewCard::new("high".to_string(), NOW - 30);
        let not_due: ReviewCard = ReviewCard::new("future".to_string(), NOW + 1);
        low_box_late.box_index = 1;
        low_box_early.box_index = 1;
        high_box.box_index = 3;
        let cards: Vec<ReviewCard> = vec![high_box, low_box_late, not_due, low_box_early];

        let selected: Vec<&ReviewCard> = select_due_cards(cards.iter(), NOW, 2);
        let selected_ids: Vec<&str> = selected.iter().map(|card| card.id.as_str()).collect();

        assert_eq!(selected_ids, vec!["low-early", "low-late"]);
    }

    #[test]
    fn count_by_box_tallies_each_box() {
        let mut second: ReviewCard = ReviewCard::new("b".to_string(), NOW);
        second.box_index = 2;
        let cards: Vec<ReviewCard> = vec![ReviewCard::new("a".to_string(), NOW), second];

        assert_eq!(count_by_box(cards.iter()), [1, 0, 1, 0, 0, 0, 0]);
    }
}
