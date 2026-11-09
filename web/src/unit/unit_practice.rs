use leptos::prelude::*;
use shared::course::{self, Exercise, Unit};
use shared::grading::{self, GraderPromptKind};
use shared::practice::{self, PracticeModeKind, SessionScore};
use shared::progress::{PausedPractice, PausedPracticeStateKind, PracticeOutcome, SavedExerciseResult};

use crate::grading::OpenResponseGradingContext;
use crate::platform;
use crate::practice::{ExerciseSession, SessionExitKind, SessionScoringKind};
use crate::progress_state::{ProgressStore, use_progress_store};

const EXERCISES_CHANGED_NOTE: &str = "The practice changed since you paused, so it starts over.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PracticeSessionKind {
    Quiz,
    QuickPractice,
    WritingPractice,
}

impl PracticeSessionKind {
    fn title(self) -> &'static str {
        match self {
            PracticeSessionKind::Quiz => "Quiz",
            PracticeSessionKind::QuickPractice => "Practice",
            PracticeSessionKind::WritingPractice => "Writing practice",
        }
    }

    fn practice_mode(self) -> Option<PracticeModeKind> {
        match self {
            PracticeSessionKind::Quiz => Some(PracticeModeKind::Full),
            PracticeSessionKind::QuickPractice => Some(PracticeModeKind::Quick),
            PracticeSessionKind::WritingPractice => None,
        }
    }

    fn exit_kind(self) -> SessionExitKind {
        match self {
            PracticeSessionKind::Quiz => SessionExitKind::Pause,
            PracticeSessionKind::QuickPractice | PracticeSessionKind::WritingPractice => SessionExitKind::Quit,
        }
    }

    fn scoring(self) -> SessionScoringKind {
        match self {
            PracticeSessionKind::Quiz | PracticeSessionKind::QuickPractice => SessionScoringKind::Scored,
            PracticeSessionKind::WritingPractice => SessionScoringKind::Unscored,
        }
    }
}

#[derive(Debug, Clone)]
struct ActiveSession {
    kind: PracticeSessionKind,
    /// In the order the session asks them.
    exercises: Vec<Exercise>,
    answered_results: Vec<SavedExerciseResult>,
}

#[derive(Debug, Clone)]
enum PracticeStartKind {
    /// Replaces any paused quiz.
    FullFromTheStart,
    ResumeFull(PausedPractice),
    Quick,
    Writing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExercisesChangedNoteKind {
    Hidden,
    Shown,
}

fn quick_session(unit_exercises: &[Exercise]) -> ActiveSession {
    ActiveSession {
        kind: PracticeSessionKind::QuickPractice,
        exercises: practice::quick_practice_exercises(unit_exercises, platform::random_unit),
        answered_results: Vec::new(),
    }
}

fn full_session(paused_practice: &PausedPractice, quiz_exercises: &[Exercise]) -> ActiveSession {
    let ordered_exercises: Vec<Exercise> = paused_practice
        .exercise_order
        .iter()
        .filter_map(|exercise_index| quiz_exercises.get(*exercise_index).cloned())
        .collect();

    ActiveSession {
        kind: PracticeSessionKind::Quiz,
        exercises: ordered_exercises,
        answered_results: paused_practice.results.clone(),
    }
}

fn writing_session(writing_exercises: &[Exercise]) -> ActiveSession {
    ActiveSession {
        kind: PracticeSessionKind::WritingPractice,
        exercises: writing_exercises.to_vec(),
        answered_results: Vec::new(),
    }
}

fn counted(count: usize, singular_noun: &str, plural_noun: &str) -> String {
    let noun: &str = if count == 1 { singular_noun } else { plural_noun };

    format!("{count} {noun}")
}

fn quiz_label(exercise_count: usize) -> String {
    format!("Quiz ({})", counted(exercise_count, "exercise", "exercises"))
}

fn writing_practice_label(task_count: usize) -> String {
    format!("Writing practice ({})", counted(task_count, "task", "tasks"))
}

fn resume_label(paused_practice: &PausedPractice) -> String {
    let question_count: usize = paused_practice.question_count();
    // A practice paused on the last answer's feedback has no unanswered question left.
    let question_number: usize = (paused_practice.next_question_index() + 1).min(question_count);

    format!("Resume quiz (question {question_number} of {question_count})")
}

#[component]
pub fn PracticeSection(course_id: String, target_language: String, source_language: String, unit: Unit) -> impl IntoView {
    provide_open_response_grading(&target_language, &source_language, &unit);

    let progress_store: ProgressStore = use_progress_store();
    let unit_id: String = unit.id.clone();
    let quiz_exercises: Vec<Exercise> = practice::quiz_exercises(&unit.exercises);
    let writing_exercises: Vec<Exercise> = practice::writing_practice_exercises(&unit.exercises);
    let active_session: RwSignal<Option<ActiveSession>> = RwSignal::new(None);
    let exercises_changed_note: RwSignal<ExercisesChangedNoteKind> = RwSignal::new(ExercisesChangedNoteKind::Hidden);
    let start_labels: PracticeStartLabels = PracticeStartLabels {
        quiz: quiz_label(quiz_exercises.len()),
        writing_practice: (!writing_exercises.is_empty()).then(|| writing_practice_label(writing_exercises.len())),
    };
    let vocabulary_count: usize = unit.vocabulary.len();

    drop_paused_practice_of_changed_exercises(progress_store, &course_id, &unit_id, &quiz_exercises, exercises_changed_note);

    let start_session: Callback<PracticeStartKind> = {
        let unit_exercises: Vec<Exercise> = unit.exercises.clone();
        let quiz_exercises: Vec<Exercise> = quiz_exercises.clone();
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        Callback::new(move |start: PracticeStartKind| {
            if !matches!(start, PracticeStartKind::Writing) {
                progress_store.start_unit(&course_id, &unit_id);
            }

            let session: Option<ActiveSession> = match start {
                PracticeStartKind::FullFromTheStart => progress_store
                    .start_full_practice(&course_id, &unit_id, &quiz_exercises)
                    .map(|paused_practice| full_session(&paused_practice, &quiz_exercises)),
                PracticeStartKind::ResumeFull(paused_practice) => Some(full_session(&paused_practice, &quiz_exercises)),
                PracticeStartKind::Quick => Some(quick_session(&unit_exercises)),
                PracticeStartKind::Writing => Some(writing_session(&writing_exercises)),
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

    let paused_practice_state = move || progress_store.paused_practice_state(&course_id, &unit_id, &quiz_exercises);

    view! {
        {move || match active_session.get() {
            Some(session) => {
                let record_completion = record_completion.clone();
                let kind: PracticeSessionKind = session.kind;
                let on_answer: Option<Callback<SavedExerciseResult>> = match kind {
                    PracticeSessionKind::Quiz => Some(record_full_practice_answer),
                    PracticeSessionKind::QuickPractice | PracticeSessionKind::WritingPractice => None,
                };
                let on_complete: Option<Callback<SessionScore>> = kind
                    .practice_mode()
                    .map(|mode| Callback::new(move |score: SessionScore| record_completion(mode, score)));

                view! {
                    <ExerciseSession
                        title=kind.title()
                        exercises=session.exercises
                        answered_results=session.answered_results
                        on_answer
                        exit_kind=kind.exit_kind()
                        scoring=kind.scoring()
                        on_complete
                        on_exit=Callback::new(move |()| active_session.set(None))
                    />
                }
                .into_any()
            },
            None => {
                let paused_practice_state = paused_practice_state.clone();
                let start_labels: PracticeStartLabels = start_labels.clone();
                let is_note_shown = move || exercises_changed_note.get() == ExercisesChangedNoteKind::Shown;

                view! {
                    <p>{best_summary.clone()}</p>
                    <p class="muted">{format!("Pass mark: {}%.", practice::pass_percent())}</p>
                    <Show when=is_note_shown>
                        <p class="muted">{EXERCISES_CHANGED_NOTE}</p>
                    </Show>
                    {move || practice_start_buttons(paused_practice_state(), start_labels.clone(), start_session)}
                }
                .into_any()
            },
        }}
    }
}

fn provide_open_response_grading(target_language: &str, source_language: &str, unit: &Unit) {
    let grader_prompt: Option<GraderPromptKind> = grading::find_grader_prompt(target_language, source_language);

    if let Some(grader_prompt) = grader_prompt {
        provide_context(OpenResponseGradingContext {
            grader_prompt,
            level: unit.level.clone(),
            target_language_name: course::language_name(target_language),
        });
    }
}

/// Clears the unit's paused quiz, and shows the note, whenever its exercises differ from the quiz's.
fn drop_paused_practice_of_changed_exercises(
    progress_store: ProgressStore,
    course_id: &str,
    unit_id: &str,
    quiz_exercises: &[Exercise],
    exercises_changed_note: RwSignal<ExercisesChangedNoteKind>,
) {
    let course_id: String = course_id.to_string();
    let unit_id: String = unit_id.to_string();
    let quiz_exercises: Vec<Exercise> = quiz_exercises.to_vec();

    Effect::new(move |_| {
        let state: PausedPracticeStateKind = progress_store.paused_practice_state(&course_id, &unit_id, &quiz_exercises);

        if state != PausedPracticeStateKind::ExercisesChanged {
            return;
        }

        progress_store.clear_paused_practice(&course_id, &unit_id);
        exercises_changed_note.set(ExercisesChangedNoteKind::Shown);
    });
}

#[derive(Debug, Clone)]
struct PracticeStartLabels {
    quiz: String,
    /// `None` for a unit without open responses.
    writing_practice: Option<String>,
}

fn practice_start_buttons(
    paused_practice_state: PausedPracticeStateKind,
    start_labels: PracticeStartLabels,
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
                {start_labels.quiz}
            </button>
        }
        .into_any(),
    };

    let writing_practice_button = start_labels.writing_practice.map(|label| {
        view! { <button type="button" on:click=move |_| start_session.run(PracticeStartKind::Writing)>{label}</button> }
    });

    view! {
        <div class="row">
            {full_practice_buttons}
            <button type="button" on:click=move |_| start_session.run(PracticeStartKind::Quick)>{quick_label}</button>
            {writing_practice_button}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiz_label_counts_the_exercises() {
        assert_eq!(quiz_label(1), "Quiz (1 exercise)");
        assert_eq!(quiz_label(26), "Quiz (26 exercises)");
    }

    #[test]
    fn writing_practice_label_counts_the_tasks() {
        assert_eq!(writing_practice_label(1), "Writing practice (1 task)");
        assert_eq!(writing_practice_label(2), "Writing practice (2 tasks)");
    }
}
