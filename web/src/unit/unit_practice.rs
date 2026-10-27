use leptos::prelude::*;
use shared::course::{Exercise, Unit};
use shared::practice::{self, PracticeModeKind, SessionScore};
use shared::progress::{PausedPractice, PausedPracticeStateKind, PracticeOutcome, SavedExerciseResult};

use crate::platform;
use crate::practice::{ExerciseSession, SessionExitKind};
use crate::progress_state::{ProgressStore, use_progress_store};

const EXERCISES_CHANGED_NOTE: &str = "The practice changed since you paused, so it starts over.";

#[derive(Debug, Clone)]
struct ActiveSession {
    mode: PracticeModeKind,
    /// In the order the session asks them.
    exercises: Vec<Exercise>,
    answered_results: Vec<SavedExerciseResult>,
}

fn session_title(mode: PracticeModeKind) -> &'static str {
    match mode {
        PracticeModeKind::Full => "Quiz",
        PracticeModeKind::Quick => "Practice",
    }
}

#[derive(Debug, Clone)]
enum PracticeStartKind {
    /// Replaces any paused quiz.
    FullFromTheStart,
    ResumeFull(PausedPractice),
    Quick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExercisesChangedNoteKind {
    Hidden,
    Shown,
}

fn quick_session(all_exercises: &[Exercise]) -> ActiveSession {
    let mut sampled: Vec<Exercise> = practice::shuffled(all_exercises, platform::random_unit);
    sampled.truncate(practice::QUICK_PRACTICE_EXERCISE_COUNT);

    ActiveSession {
        mode: PracticeModeKind::Quick,
        exercises: sampled,
        answered_results: Vec::new(),
    }
}

fn full_session(paused_practice: &PausedPractice, all_exercises: &[Exercise]) -> ActiveSession {
    let ordered_exercises: Vec<Exercise> = paused_practice
        .exercise_order
        .iter()
        .filter_map(|exercise_index| all_exercises.get(*exercise_index).cloned())
        .collect();

    ActiveSession {
        mode: PracticeModeKind::Full,
        exercises: ordered_exercises,
        answered_results: paused_practice.results.clone(),
    }
}

fn session_exit_kind(mode: PracticeModeKind) -> SessionExitKind {
    match mode {
        PracticeModeKind::Full => SessionExitKind::Pause,
        PracticeModeKind::Quick => SessionExitKind::Quit,
    }
}

fn resume_label(paused_practice: &PausedPractice) -> String {
    let question_count: usize = paused_practice.question_count();
    // A practice paused on the last answer's feedback has no unanswered question left.
    let question_number: usize = (paused_practice.next_question_index() + 1).min(question_count);

    format!("Resume quiz (question {question_number} of {question_count})")
}

#[component]
pub fn PracticeSection(course_id: String, unit: Unit) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let unit_id: String = unit.id.clone();
    let active_session: RwSignal<Option<ActiveSession>> = RwSignal::new(None);
    let exercises_changed_note: RwSignal<ExercisesChangedNoteKind> = RwSignal::new(ExercisesChangedNoteKind::Hidden);
    let exercise_count: usize = unit.exercises.len();
    let vocabulary_count: usize = unit.vocabulary.len();

    drop_paused_practice_of_changed_exercises(progress_store, &course_id, &unit, exercises_changed_note);

    let start_session: Callback<PracticeStartKind> = {
        let all_exercises: Vec<Exercise> = unit.exercises.clone();
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        Callback::new(move |start: PracticeStartKind| {
            progress_store.start_unit(&course_id, &unit_id);

            let session: Option<ActiveSession> = match start {
                PracticeStartKind::FullFromTheStart => progress_store
                    .start_full_practice(&course_id, &unit_id, &all_exercises)
                    .map(|paused_practice| full_session(&paused_practice, &all_exercises)),
                PracticeStartKind::ResumeFull(paused_practice) => Some(full_session(&paused_practice, &all_exercises)),
                PracticeStartKind::Quick => Some(quick_session(&all_exercises)),
            };

            exercises_changed_note.set(ExercisesChangedNoteKind::Hidden);
            active_session.set(session);
        })
    };

    let record_full_practice_answer: Callback<SavedExerciseResult> = {
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        Callback::new(move |result: SavedExerciseResult| progress_store.record_full_practice_result(&course_id, &unit_id, result))
    };

    let record_completion = {
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        move |mode: PracticeModeKind, score: SessionScore| {
            let now_milliseconds: i64 = platform::now_milliseconds();

            progress_store.update(|progress| match mode {
                PracticeModeKind::Full => {
                    let outcome: Option<PracticeOutcome> = progress.finish_full_practice(&course_id, &unit_id, vocabulary_count, now_milliseconds);

                    if outcome.is_none() {
                        log::warn!("a quiz ended without a saved session to record; [unit_id={unit_id}]");
                    }
                },
                PracticeModeKind::Quick => {
                    progress.record_practice_session(&course_id, &unit_id, mode, score, vocabulary_count, now_milliseconds);
                },
            });
        }
    };

    let best_summary = {
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        move || {
            let unit_progress = progress_store.read(|progress| progress.course_progress(&course_id).unit_progress(&unit_id));

            if unit_progress.attempt_count == 0 {
                "You have not taken the quiz yet.".to_string()
            } else {
                format!(
                    "Quiz: {}% over {} attempt(s). {}",
                    (unit_progress.best_proportion * 100.0).round(),
                    unit_progress.attempt_count,
                    if unit_progress.is_completed() { "Unit completed." } else { "" },
                )
            }
        }
    };

    let paused_practice_state = {
        let all_exercises: Vec<Exercise> = unit.exercises.clone();

        move || progress_store.paused_practice_state(&course_id, &unit_id, &all_exercises)
    };

    view! {
        {move || match active_session.get() {
            Some(session) => {
                let record_completion = record_completion.clone();
                let mode: PracticeModeKind = session.mode;
                let on_answer: Option<Callback<SavedExerciseResult>> = match mode {
                    PracticeModeKind::Full => Some(record_full_practice_answer),
                    PracticeModeKind::Quick => None,
                };

                view! {
                    <ExerciseSession
                        title=session_title(mode)
                        exercises=session.exercises
                        answered_results=session.answered_results
                        on_answer
                        exit_kind=session_exit_kind(mode)
                        on_complete=Callback::new(move |score: SessionScore| record_completion(mode, score))
                        on_exit=Callback::new(move |()| active_session.set(None))
                    />
                }
                .into_any()
            },
            None => {
                let paused_practice_state = paused_practice_state.clone();
                let is_note_shown = move || exercises_changed_note.get() == ExercisesChangedNoteKind::Shown;

                view! {
                    <p>{best_summary.clone()}</p>
                    <p class="muted">{format!("Pass mark: {}%.", practice::pass_percent())}</p>
                    <Show when=is_note_shown>
                        <p class="muted">{EXERCISES_CHANGED_NOTE}</p>
                    </Show>
                    {move || practice_start_buttons(paused_practice_state(), exercise_count, start_session)}
                }
                .into_any()
            },
        }}
    }
}

/// Clears the unit's paused quiz, and shows the note, whenever its exercises differ from the unit's.
fn drop_paused_practice_of_changed_exercises(
    progress_store: ProgressStore,
    course_id: &str,
    unit: &Unit,
    exercises_changed_note: RwSignal<ExercisesChangedNoteKind>,
) {
    let course_id: String = course_id.to_string();
    let unit_id: String = unit.id.clone();
    let all_exercises: Vec<Exercise> = unit.exercises.clone();

    Effect::new(move |_| {
        let state: PausedPracticeStateKind = progress_store.paused_practice_state(&course_id, &unit_id, &all_exercises);

        if state != PausedPracticeStateKind::ExercisesChanged {
            return;
        }

        progress_store.clear_paused_practice(&course_id, &unit_id);
        exercises_changed_note.set(ExercisesChangedNoteKind::Shown);
    });
}

fn practice_start_buttons(
    paused_practice_state: PausedPracticeStateKind,
    exercise_count: usize,
    start_session: Callback<PracticeStartKind>,
) -> impl IntoView {
    let quick_label: String = format!("Quick practice ({} random exercises)", practice::QUICK_PRACTICE_EXERCISE_COUNT);

    let full_practice_buttons: AnyView = match paused_practice_state {
        PausedPracticeStateKind::Resumable(paused_practice) => {
            let label: String = resume_label(&paused_practice);

            view! {
                <button type="button" class="primary" on:click=move |_| start_session.run(PracticeStartKind::ResumeFull(paused_practice.clone()))>
                    {label}
                </button>
                <button type="button" on:click=move |_| start_session.run(PracticeStartKind::FullFromTheStart)>"Start over"</button>
            }
            .into_any()
        },
        PausedPracticeStateKind::NotPaused | PausedPracticeStateKind::ExercisesChanged => view! {
            <button type="button" class="primary" on:click=move |_| start_session.run(PracticeStartKind::FullFromTheStart)>
                {format!("Quiz ({exercise_count} exercises)")}
            </button>
        }
        .into_any(),
    };

    view! {
        <div class="row">
            {full_practice_buttons}
            <button type="button" on:click=move |_| start_session.run(PracticeStartKind::Quick)>{quick_label}</button>
        </div>
    }
}
