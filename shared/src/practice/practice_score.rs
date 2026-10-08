pub const PASS_PROPORTION: f64 = 0.8;
pub const QUICK_PRACTICE_EXERCISE_COUNT: usize = 10;

/// Full practice runs every exercise of a unit and counts toward completing it; quick practice samples a few.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeModeKind {
    Full,
    Quick,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionScore {
    pub correct_count: usize,
    pub total_count: usize,
}

impl SessionScore {
    pub fn proportion(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }

        self.correct_count as f64 / self.total_count as f64
    }

    pub fn passed(&self) -> bool {
        self.proportion() >= PASS_PROPORTION
    }

    pub fn percent(&self) -> u32 {
        (self.proportion() * 100.0).round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passed_requires_the_pass_proportion() {
        assert!(SessionScore { correct_count: 4, total_count: 5 }.passed());
        assert!(!SessionScore { correct_count: 3, total_count: 5 }.passed());
    }

    #[test]
    fn proportion_of_an_empty_session_is_zero() {
        assert_eq!(SessionScore { correct_count: 0, total_count: 0 }.proportion(), 0.0);
    }

    #[test]
    fn percent_rounds_to_the_nearest_whole_number() {
        assert_eq!(SessionScore { correct_count: 2, total_count: 3 }.percent(), 67);
    }
}
