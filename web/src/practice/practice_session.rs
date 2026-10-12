use std::sync::Arc;

use leptos::ev;
use leptos::prelude::*;
use shared::course::{Exercise, ExerciseKind};
use shared::practice::SessionScore;
use shared::progress::{SavedExerciseResult, VoiceChoiceKind};

use crate::audio_player::{PlayButton, PlaybackStore, audio_clip, use_playback_store};
use crate::practice;

const ENTER_KEY: &str = "Enter";

/* Kinds whose answer is a target-language sentence the learner did not just hear, so the sentence is played
   once the answer is checked. */
const SPEAK_AFTER_ANSWER_KINDS: [ExerciseKind; 3] = [
    ExerciseKind::FillBlank,
    ExerciseKind::TranslateToTarget,
    ExerciseKind::OrderWords,
];

/// What leaving a session before its summary does to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionExitKind {
    #[default]
    Quit,
    /// The session is kept and can be continued later.
    Pause,
}

impl SessionExitKind {
    fn label(self) -> &'static str {
        match self {
            SessionExitKind::Quit => "Quit",
            SessionExitKind::Pause => "Pause",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionRoundKind {
    First,
    RetryOfMissed,
}

struct SessionProgress {
    title: String,
    index: usize,
    total_count: usize,
    exit_kind: SessionExitKind,
}

fn score_of(results: &[SavedExerciseResult], total_count: usize) -> SessionScore {
    SessionScore {
        correct_count: results.iter().filter(|result| result.is_correct()).count(),
        total_count,
    }
}

#[component]
pub fn ExerciseSession(
    #[prop(into)] title: String,
    exercises: Vec<Exercise>,
    /// The results of the first questions, answered before the session opened.
    #[prop(optional)]
    answered_results: Vec<SavedExerciseResult>,
    /// Runs for each answer of the first round, never for the retry of missed questions.
    #[prop(optional_no_strip)]
    on_answer: Option<Callback<SavedExerciseResult>>,
    #[prop(optional)] exit_kind: SessionExitKind,
    on_complete: Callback<SessionScore>,
    on_exit: Callback<()>,
) -> impl IntoView {
    let active_exercises: RwSignal<Arc<Vec<Exercise>>> = RwSignal::new(Arc::new(exercises));
    let current_index: RwSignal<usize> = RwSignal::new(answered_results.len());
    let results: RwSignal<Vec<SavedExerciseResult>> = RwSignal::new(answered_results);
    let current_result: RwSignal<Option<SavedExerciseResult>> = RwSignal::new(None);
    let round: RwSignal<SessionRoundKind> = RwSignal::new(SessionRoundKind::First);
    let completion_reported: RwSignal<bool> = RwSignal::new(false);

    let advance = move || {
        if current_result.get_untracked().is_none() {
            return;
        }

        current_result.set(None);
        current_index.update(|index| *index += 1);
    };

    let keydown_listener = window_event_listener(ev::keydown, move |event| {
        let has_feedback: bool = current_result.get_untracked().is_some();

        if has_feedback && event.key() == ENTER_KEY {
            event.prevent_default();
            advance();
        }
    });
    on_cleanup(move || keydown_listener.remove());

    Effect::new(move |_| {
        let finished: bool = current_index.get() >= active_exercises.get().len();
        let is_first_round: bool = round.get_untracked() == SessionRoundKind::First;

        if finished && is_first_round && !completion_reported.get_untracked() {
            completion_reported.set(true);
            on_complete.run(score_of(&results.get_untracked(), active_exercises.get_untracked().len()));
        }
    });

    let on_submit: Callback<SavedExerciseResult> = Callback::new(move |result: SavedExerciseResult| {
        let is_first_round: bool = round.get_untracked() == SessionRoundKind::First;

        results.update(|all_results| all_results.push(result.clone()));

        if is_first_round && let Some(on_answer) = on_answer {
            on_answer.run(result.clone());
        }

        current_result.set(Some(result));
    });

    let retry_missed = move || {
        let exercises_in_session: Arc<Vec<Exercise>> = active_exercises.get_untracked();
        let missed_exercises: Vec<Exercise> = results
            .get_untracked()
            .iter()
            .zip(exercises_in_session.iter())
            .filter(|(result, _)| !result.is_correct())
            .map(|(_, exercise)| exercise.clone())
            .collect();

        active_exercises.set(Arc::new(missed_exercises));
        results.set(Vec::new());
        current_result.set(None);
        current_index.set(0);
        round.set(SessionRoundKind::RetryOfMissed);
    };

    let session_title: String = title.clone();

    view! {
        <section class="session panel">
            {move || {
                let exercises_in_session: Arc<Vec<Exercise>> = active_exercises.get();
                let index: usize = current_index.get();

                match exercises_in_session.get(index) {
                    Some(exercise) => question_view(
                        session_progress(&session_title, round.get(), index, exercises_in_session.len(), exit_kind),
                        exercise.clone(),
                        on_submit,
                        current_result,
                        advance,
                        on_exit,
                    )
                    .into_any(),
                    None => summary_view(
                        &session_title,
                        exercises_in_session,
                        results.get_untracked(),
                        retry_missed,
                        on_exit,
                    )
                    .into_any(),
                }
            }}
        </section>
    }
}

/// A retry of missed questions is never kept, so it is always quit.
fn session_progress(
    session_title: &str,
    round: SessionRoundKind,
    index: usize,
    total_count: usize,
    exit_kind: SessionExitKind,
) -> SessionProgress {
    match round {
        SessionRoundKind::First => SessionProgress {
            title: session_title.to_string(),
            index,
            total_count,
            exit_kind,
        },
        SessionRoundKind::RetryOfMissed => SessionProgress {
            title: format!("{session_title}, retry of missed"),
            index,
            total_count,
            exit_kind: SessionExitKind::Quit,
        },
    }
}

fn question_view(
    session_progress: SessionProgress,
    exercise: Exercise,
    on_submit: Callback<SavedExerciseResult>,
    current_result: RwSignal<Option<SavedExerciseResult>>,
    advance: impl Fn() + Copy + Send + Sync + 'static,
    on_exit: Callback<()>,
) -> impl IntoView {
    let SessionProgress {
        title,
        index,
        total_count,
        exit_kind,
    } = session_progress;
    let instruction: &'static str = exercise.kind().instruction();
    let feedback_exercise: Exercise = exercise.clone();

    view! {
        <div class="session-header">
            <span>{format!("{title}: question {} of {total_count}", index + 1)}</span>
            <button type="button" class="link" on:click=move |_| on_exit.run(())>{exit_kind.label()}</button>
        </div>
        <progress max=total_count value=index></progress>
        <div class="exercise-type">{instruction}</div>
        {practice::exercise_body(exercise, on_submit)}
        {move || {
            current_result
                .get()
                .map(|result| feedback_view(feedback_exercise.clone(), result, advance))
        }}
    }
}

fn feedback_view(
    exercise: Exercise,
    result: SavedExerciseResult,
    advance: impl Fn() + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let heading: &str = match result.status {
        shared::practice::AnswerStatus::Correct => "Correct",
        shared::practice::AnswerStatus::AccentMistake => "Correct, but check the accents",
        shared::practice::AnswerStatus::Wrong => "Not quite",
    };
    let feedback_class: &str = if result.is_correct() { "feedback good" } else { "feedback bad" };
    let shows_answer: bool = result.status != shared::practice::AnswerStatus::Correct;
    let spoken_text: Option<String> = exercise.spoken_text();
    let audio_path: Option<String> = exercise.clip_key().map(str::to_string);

    if SPEAK_AFTER_ANSWER_KINDS.contains(&exercise.kind()) {
        if let Some(text) = &spoken_text {
            let playback_store: PlaybackStore = use_playback_store();

            playback_store.play_at_normal_speed(audio_clip(&audio_path, text, VoiceChoiceKind::OwnText));
        }
    }

    view! {
        <div class=feedback_class aria-live="polite">
            <strong>{heading}</strong>
            {shows_answer.then(|| view! { <p>"Answer: "{result.answer_text.clone()}</p> })}
            {exercise.explanation().map(|explanation| view! { <p class="explanation">{explanation.to_string()}</p> })}
            <div class="row">
                {spoken_text.map(|text| view! { <PlayButton clip_key=audio_path.clone() text label="Hear it" /> })}
                <button type="button" class="primary" on:click=move |_| advance()>"Continue"</button>
            </div>
        </div>
    }
}

fn summary_view(
    title: &str,
    exercises: Arc<Vec<Exercise>>,
    results: Vec<SavedExerciseResult>,
    retry_missed: impl Fn() + Copy + Send + Sync + 'static,
    on_exit: Callback<()>,
) -> impl IntoView {
    let score: SessionScore = score_of(&results, exercises.len());
    let missed_items: Vec<_> = results
        .iter()
        .zip(exercises.iter())
        .filter(|(result, _)| !result.is_correct())
        .map(|(result, exercise)| {
            view! {
                <li>
                    <div>{exercise.prompt_text()}</div>
                    <div class="answer">{result.answer_text.clone()}</div>
                </li>
            }
        })
        .collect();
    let has_missed: bool = !missed_items.is_empty();
    let verdict: String = if score.passed() {
        "Passed.".to_string()
    } else {
        format!("Not passed yet. {}% is needed.", shared::practice::pass_percent())
    };
    let verdict_class: &str = if score.passed() { "good-text" } else { "bad-text" };

    view! {
        <h3>{format!("{title}: {} of {} correct ({}%)", score.correct_count, score.total_count, score.percent())}</h3>
        <p class=verdict_class>{verdict}</p>
        {has_missed.then(|| view! { <h4>"To review"</h4> })}
        <ul class="missed">{missed_items}</ul>
        <div class="row">
            {has_missed.then(|| view! { <button type="button" class="primary" on:click=move |_| retry_missed()>"Retry missed"</button> })}
            <button type="button" on:click=move |_| on_exit.run(())>"Done"</button>
        </div>
    }
}
