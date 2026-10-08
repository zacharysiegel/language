use chrono::NaiveDate;
use leptos::prelude::*;
use shared::course::{Exercise, Unit};
use shared::practice::{PASS_PROPORTION, PracticeModeKind, QUICK_PRACTICE_EXERCISE_COUNT, SessionScore, shuffled};

use crate::platform;
use crate::practice::ExerciseSession;
use crate::progress_state::{ProgressStore, use_progress_store};

#[component]
pub fn PracticeSection(unit: Unit) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let unit_id: String = unit.id.clone();
    let active_mode: RwSignal<Option<PracticeModeKind>> = RwSignal::new(None);
    let session_exercises: RwSignal<Vec<Exercise>> = RwSignal::new(Vec::new());
    let exercise_count: usize = unit.exercises.len();
    let vocabulary_count: usize = unit.vocabulary.len();

    let start_mode = {
        let all_exercises: Vec<Exercise> = unit.exercises.clone();

        move |mode: PracticeModeKind| {
            let chosen: Vec<Exercise> = match mode {
                PracticeModeKind::Full => all_exercises.clone(),
                PracticeModeKind::Quick => {
                    let mut sampled: Vec<Exercise> = shuffled(&all_exercises, platform::random_unit);
                    sampled.truncate(QUICK_PRACTICE_EXERCISE_COUNT);
                    sampled
                },
            };

            session_exercises.set(chosen);
            active_mode.set(Some(mode));
        }
    };

    let record_completion = {
        let unit_id: String = unit_id.clone();

        move |mode: PracticeModeKind, score: SessionScore| {
            let today: NaiveDate = platform::today();
            let now_milliseconds: i64 = platform::now_milliseconds();

            progress_store.update(|progress| {
                progress.record_practice_session(&unit_id, mode, score, vocabulary_count, today, now_milliseconds);
            });
        }
    };

    let best_summary = {
        let unit_id: String = unit_id.clone();

        move || {
            let unit_progress = progress_store.read(|progress| progress.unit_progress(&unit_id));

            if unit_progress.attempt_count == 0 {
                "You have not attempted this unit yet.".to_string()
            } else {
                format!(
                    "Best score {}% over {} attempt(s). {}",
                    (unit_progress.best_proportion * 100.0).round(),
                    unit_progress.attempt_count,
                    if unit_progress.completed { "Unit completed." } else { "" },
                )
            }
        }
    };

    view! {
        {move || match active_mode.get() {
            Some(mode) => {
                let record_completion = record_completion.clone();

                view! {
                    <ExerciseSession
                        title="Practice"
                        exercises=session_exercises.get_untracked()
                        on_complete=Callback::new(move |score: SessionScore| record_completion(mode, score))
                        on_exit=Callback::new(move |()| active_mode.set(None))
                    />
                }
                .into_any()
            },
            None => {
                let start_full = start_mode.clone();
                let start_quick = start_mode.clone();

                view! {
                    <p>{best_summary.clone()}</p>
                    <p class="muted">{format!("Pass mark: {}%.", (PASS_PROPORTION * 100.0).round())}</p>
                    <div class="row">
                        <button type="button" class="primary" on:click=move |_| start_full(PracticeModeKind::Full)>
                            {format!("Full practice ({exercise_count} exercises)")}
                        </button>
                        <button type="button" on:click=move |_| start_quick(PracticeModeKind::Quick)>
                            {format!("Quick practice ({QUICK_PRACTICE_EXERCISE_COUNT} random exercises)")}
                        </button>
                    </div>
                }
                .into_any()
            },
        }}
    }
}
