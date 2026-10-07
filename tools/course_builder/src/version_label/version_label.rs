use std::collections::BTreeSet;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use shared::AppError;

const CITIES_RAW: &str = include_str!("./united_states_cities.txt");
const MAX_GENERATION_ATTEMPTS: usize = 16;
const DATE_PREFIX_FORMAT: &str = "%Y-%m-%d";

static CITIES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    CITIES_RAW
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
});

/// A label `<date>+<city>` that none of `taken_labels` uses.
pub fn generate(created: DateTime<Utc>, taken_labels: &BTreeSet<String>) -> Result<String, AppError> {
    let date_prefix: String = created.format(DATE_PREFIX_FORMAT).to_string();
    let random_state: RandomState = RandomState::new();

    for attempt in 0..MAX_GENERATION_ATTEMPTS {
        let candidate: String = format!("{date_prefix}+{}", pick_city(&random_state, attempt));

        if !taken_labels.contains(&candidate) {
            return Ok(candidate);
        }
    }

    Err(AppError::from(format!(
        "could not generate an unused version label; [attempts={MAX_GENERATION_ATTEMPTS} date={date_prefix}]",
    )))
}

/* `RandomState` is seeded randomly per instance, so hashing the attempt number picks an unpredictable city. */
fn pick_city(random_state: &RandomState, attempt: usize) -> &'static str {
    let index: usize = random_state.hash_one(attempt) as usize % CITIES.len();

    CITIES[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cities_are_lowercase_ascii_words_joined_by_hyphens() {
        for city in CITIES.iter() {
            let words_are_lowercase_ascii: bool = city.split('-').all(|word| !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_lowercase()));

            assert!(words_are_lowercase_ascii, "{city}");
        }
    }

    #[test]
    fn cities_are_distinct() {
        let distinct_cities: BTreeSet<&str> = CITIES.iter().copied().collect();

        assert_eq!(distinct_cities.len(), CITIES.len());
    }

    #[test]
    fn generate_prefixes_the_creation_date() {
        let created: DateTime<Utc> = "2026-10-07T22:24:23Z".parse().unwrap();

        let label: String = generate(created, &BTreeSet::new()).unwrap();
        let (date_prefix, city): (&str, &str) = label.split_once('+').unwrap();

        assert_eq!(date_prefix, "2026-10-07");
        assert!(CITIES.contains(&city));
    }

    #[test]
    fn generate_fails_when_every_label_of_the_day_is_taken() {
        let created: DateTime<Utc> = "2026-10-07T22:24:23Z".parse().unwrap();
        let every_label: BTreeSet<String> = CITIES.iter().map(|city| format!("2026-10-07+{city}")).collect();

        assert!(generate(created, &every_label).is_err());
    }

    #[test]
    fn generate_never_returns_a_taken_label() {
        let created: DateTime<Utc> = "2026-10-07T22:24:23Z".parse().unwrap();
        let taken_labels: BTreeSet<String> = CITIES.iter().step_by(2).map(|city| format!("2026-10-07+{city}")).collect();

        for _ in 0..50 {
            let label: String = generate(created, &taken_labels).unwrap();

            assert!(!taken_labels.contains(&label));
        }
    }
}
