use std::sync::Arc;
use std::time::Duration;

use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use shared::course::{
    DictationExercise, Exercise, FillBlankExercise, ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise,
    OpenAnswerExercise, OrderWordsExercise, TranslateToTargetExercise,
};
use leptos::task::spawn_local;
use shared::grading::GraderResponse;
use shared::practice::{AnswerStatus, check_typed_answer, shuffled, shuffled_avoiding_identity};
use shared::progress::{SavedExerciseResult, VoiceChoiceKind};

use crate::audio_player::{PlayButton, PlaybackStore, PlaybackTarget, audio_clip, indicated_pending, use_playback_store};
use crate::grading::{
    self, GradedFeedbackHeading, GraderFindings, GraderStateKind, GradingFailureKind, GradingStore, ModelDownloadStatus,
    OpenAnswerGradingContext, OpenAnswerSubmission, SelfCheckNotice, use_grading_store,
};
use crate::pending_indication::{self, PendingIcon};
use crate::platform::{self, AudioClip};

const WRONG_FLASH_MILLISECONDS: u64 = 500;
const OPEN_ANSWER_ROWS: u32 = 4;
pub const SELF_CHECK_HEADING: &str = "Compare with the examples";

fn choice_result(chosen_correctly: bool, answer_text: &str) -> SavedExerciseResult {
    let status: AnswerStatus = if chosen_correctly { AnswerStatus::Correct } else { AnswerStatus::Wrong };

    SavedExerciseResult {
        status,
        answer_text: answer_text.to_string(),
    }
}

pub fn exercise_body(
    exercise: Exercise,
    on_submit: Callback<SavedExerciseResult>,
    open_answer_outcome: RwSignal<Option<OpenAnswerOutcomeKind>>,
) -> AnyView {
    match exercise {
        Exercise::MultipleChoice(exercise) => view! { <MultipleChoiceView exercise on_submit /> }.into_any(),
        Exercise::FillBlank(exercise) => view! { <FillBlankView exercise on_submit /> }.into_any(),
        Exercise::TranslateToTarget(exercise) => view! { <TranslateView exercise on_submit /> }.into_any(),
        Exercise::OrderWords(exercise) => view! { <OrderWordsView exercise on_submit /> }.into_any(),
        Exercise::ListeningChoice(exercise) => view! { <ListeningChoiceView exercise on_submit /> }.into_any(),
        Exercise::Dictation(exercise) => view! { <DictationView exercise on_submit /> }.into_any(),
        Exercise::Match(exercise) => view! { <MatchView exercise on_submit /> }.into_any(),
        Exercise::OpenAnswer(exercise) => view! { <OpenAnswerView exercise on_submit outcome=open_answer_outcome /> }.into_any(),
    }
}

fn choice_buttons(
    choices: &[String],
    answer_index: usize,
    answer_text: String,
    on_submit: Callback<SavedExerciseResult>,
) -> impl IntoView + use<> {
    let selected: RwSignal<Option<usize>> = RwSignal::new(None);
    let display_order: Vec<usize> = shuffled(&(0..choices.len()).collect::<Vec<usize>>(), platform::random_unit);

    let buttons: Vec<_> = display_order
        .into_iter()
        .map(|choice_index| {
            let choice_text: String = choices[choice_index].clone();
            let answer_text: String = answer_text.clone();

            view! {
                <button
                    type="button"
                    class="choice"
                    class:right=move || selected.get().is_some() && choice_index == answer_index
                    class:wrong=move || selected.get() == Some(choice_index) && choice_index != answer_index
                    disabled=move || selected.get().is_some()
                    on:click=move |_| {
                        if selected.get_untracked().is_some() {
                            return;
                        }

                        selected.set(Some(choice_index));
                        on_submit.run(choice_result(choice_index == answer_index, &answer_text));
                    }
                >
                    {choice_text}
                </button>
            }
        })
        .collect();

    view! { <div class="choices">{buttons}</div> }
}

#[component]
fn MultipleChoiceView(exercise: MultipleChoiceExercise, on_submit: Callback<SavedExerciseResult>) -> impl IntoView {
    let answer_text: String = exercise.choices[exercise.answer_index].clone();

    view! {
        <p class="prompt">{exercise.prompt.clone()}</p>
        {choice_buttons(&exercise.choices, exercise.answer_index, answer_text, on_submit)}
    }
}

#[component]
fn ListeningChoiceView(exercise: ListeningChoiceExercise, on_submit: Callback<SavedExerciseResult>) -> impl IntoView {
    let answer_text: String = format!("{} = {}", exercise.audio_target, exercise.choices[exercise.answer_index]);
    let autoplay_clip: AudioClip = audio_clip(&exercise.clip_key, &exercise.audio_target, VoiceChoiceKind::OwnText);
    let playback_store: PlaybackStore = use_playback_store();

    Effect::new(move |_| playback_store.play_at_normal_speed(autoplay_clip.clone()));

    view! {
        <div class="row">
            <PlayButton clip_key=exercise.clip_key.clone() text=exercise.audio_target.clone() label="Play" />
            <PlayButton clip_key=exercise.clip_key.clone() text=exercise.audio_target.clone() slow=true />
        </div>
        {choice_buttons(&exercise.choices, exercise.answer_index, answer_text, on_submit)}
    }
}

fn submit_typed_answer(
    typed: RwSignal<String>,
    answered: RwSignal<bool>,
    accepted_answers: &[String],
    answer_text: &str,
    on_submit: Callback<SavedExerciseResult>,
) {
    let typed_text: String = typed.get_untracked();

    if answered.get_untracked() || typed_text.trim().is_empty() {
        return;
    }

    answered.set(true);

    let status: AnswerStatus = check_typed_answer(&typed_text, accepted_answers);

    on_submit.run(SavedExerciseResult {
        status,
        answer_text: answer_text.to_string(),
    });
}

#[component]
fn TypedInput(
    typed: RwSignal<String>,
    answered: RwSignal<bool>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            _ = input.focus();
        }
    });

    view! {
        <input
            node_ref=input_ref
            type="text"
            class=format!("typed {class}")
            autocomplete="off"
            autocapitalize="off"
            spellcheck="false"
            prop:value=move || typed.get()
            on:input:target=move |event| typed.set(event.target().value())
            disabled=move || answered.get()
        />
    }
}

#[component]
fn CheckButton(answered: RwSignal<bool>) -> impl IntoView {
    view! {
        <button type="submit" class="primary" disabled=move || answered.get()>"Check"</button>
    }
}

#[component]
fn FillBlankView(exercise: FillBlankExercise, on_submit: Callback<SavedExerciseResult>) -> impl IntoView {
    let typed: RwSignal<String> = RwSignal::new(String::new());
    let answered: RwSignal<bool> = RwSignal::new(false);
    let (before, after): (&str, &str) = exercise
        .prompt_target
        .split_once(FillBlankExercise::BLANK_MARKER)
        .unwrap_or((exercise.prompt_target.as_str(), ""));
    let (before, after): (String, String) = (before.to_string(), after.to_string());
    let answer_text: String = exercise.completed_sentence();
    let accepted_answers: Vec<String> = exercise.answers.clone();

    view! {
        <form on:submit=move |event: SubmitEvent| {
            event.prevent_default();
            submit_typed_answer(typed, answered, &accepted_answers, &answer_text, on_submit);
        }>
            <p class="hint">{exercise.hint_source.clone()}</p>
            <p class="prompt sentence">
                {before}<TypedInput typed answered class="inline" />{after}
            </p>
            <CheckButton answered />
        </form>
    }
}

#[component]
fn TranslateView(exercise: TranslateToTargetExercise, on_submit: Callback<SavedExerciseResult>) -> impl IntoView {
    let typed: RwSignal<String> = RwSignal::new(String::new());
    let answered: RwSignal<bool> = RwSignal::new(false);
    let answer_text: String = exercise.answers.join("  |  ");
    let accepted_answers: Vec<String> = exercise.answers.clone();

    view! {
        <form on:submit=move |event: SubmitEvent| {
            event.prevent_default();
            submit_typed_answer(typed, answered, &accepted_answers, &answer_text, on_submit);
        }>
            <p class="prompt">{exercise.prompt_source.clone()}</p>
            <TypedInput typed answered />
            <CheckButton answered />
        </form>
    }
}

#[component]
fn DictationView(exercise: DictationExercise, on_submit: Callback<SavedExerciseResult>) -> impl IntoView {
    let typed: RwSignal<String> = RwSignal::new(String::new());
    let answered: RwSignal<bool> = RwSignal::new(false);
    let answer_text: String = format!("{} = {}", exercise.audio_target, exercise.hint_source);
    let accepted_answers: Vec<String> = vec![exercise.audio_target.clone()];
    let autoplay_clip: AudioClip = audio_clip(&exercise.clip_key, &exercise.audio_target, VoiceChoiceKind::OwnText);
    let playback_store: PlaybackStore = use_playback_store();

    Effect::new(move |_| playback_store.play_at_normal_speed(autoplay_clip.clone()));

    view! {
        <form on:submit=move |event: SubmitEvent| {
            event.prevent_default();
            submit_typed_answer(typed, answered, &accepted_answers, &answer_text, on_submit);
        }>
            <div class="row">
                <PlayButton clip_key=exercise.clip_key.clone() text=exercise.audio_target.clone() label="Play" />
                <PlayButton clip_key=exercise.clip_key.clone() text=exercise.audio_target.clone() slow=true />
            </div>
            <TypedInput typed answered />
            <CheckButton answered />
        </form>
    }
}

#[component]
fn OrderWordsView(exercise: OrderWordsExercise, on_submit: Callback<SavedExerciseResult>) -> impl IntoView {
    let candidate_tokens: Vec<String> = exercise.words.iter().chain(exercise.distractors.iter()).cloned().collect();
    let tokens: Arc<Vec<String>> = Arc::new(shuffled_avoiding_identity(&candidate_tokens, platform::random_unit));
    let placed_token_ids: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());
    let answered: RwSignal<bool> = RwSignal::new(false);
    let sentence: String = exercise.sentence();

    let answer_tokens: Arc<Vec<String>> = tokens.clone();
    let placed_buttons = move || {
        placed_token_ids
            .get()
            .into_iter()
            .map(|token_id| {
                let token_text: String = answer_tokens[token_id].clone();

                view! {
                    <button
                        type="button"
                        class="token"
                        disabled=move || answered.get()
                        on:click=move |_| placed_token_ids.update(|placed| placed.retain(|placed_id| *placed_id != token_id))
                    >
                        {token_text}
                    </button>
                }
            })
            .collect_view()
    };

    let bank_buttons: Vec<_> = (0..tokens.len())
        .map(|token_id| {
            let token_text: String = tokens[token_id].clone();

            view! {
                <button
                    type="button"
                    class="token"
                    disabled=move || answered.get() || placed_token_ids.get().contains(&token_id)
                    on:click=move |_| placed_token_ids.update(|placed| placed.push(token_id))
                >
                    {token_text}
                </button>
            }
        })
        .collect();

    let submit_tokens: Arc<Vec<String>> = tokens.clone();
    let check = move |_| {
        if answered.get_untracked() {
            return;
        }

        answered.set(true);

        let attempt: String = placed_token_ids
            .get_untracked()
            .into_iter()
            .map(|token_id| submit_tokens[token_id].as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        on_submit.run(choice_result(attempt == sentence, &sentence));
    };

    view! {
        <p class="prompt">{exercise.prompt_source.clone()}</p>
        <div class="tokens answer-area">{placed_buttons}</div>
        <div class="tokens bank">{bank_buttons}</div>
        <button
            type="button"
            class="primary"
            disabled=move || answered.get() || placed_token_ids.get().is_empty()
            on:click=check
        >
            "Check"
        </button>
    }
}

#[component]
fn MatchView(exercise: MatchExercise, on_submit: Callback<SavedExerciseResult>) -> impl IntoView {
    let pairs: Arc<MatchExercise> = Arc::new(exercise);
    let pair_indexes: Vec<usize> = (0..pairs.pairs.len()).collect();
    let left_order: Vec<usize> = shuffled(&pair_indexes, platform::random_unit);
    let right_order: Vec<usize> = shuffled(&pair_indexes, platform::random_unit);
    let selected_left: RwSignal<Option<usize>> = RwSignal::new(None);
    let selected_right: RwSignal<Option<usize>> = RwSignal::new(None);
    let matched_pair_indexes: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());
    let flashing_wrong: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());
    let mistake_count: RwSignal<u32> = RwSignal::new(0);
    let playback_store: PlaybackStore = use_playback_store();

    let resolve_selection = {
        let pairs: Arc<MatchExercise> = pairs.clone();

        move || {
            let (Some(left_index), Some(right_index)) = (selected_left.get_untracked(), selected_right.get_untracked())
            else {
                return;
            };

            selected_left.set(None);
            selected_right.set(None);

            if left_index == right_index {
                matched_pair_indexes.update(|matched| matched.push(left_index));

                if matched_pair_indexes.get_untracked().len() == pairs.pairs.len() {
                    let answer_text: String = pairs
                        .pairs
                        .iter()
                        .map(|pair| format!("{} = {}", pair.target, pair.source))
                        .collect::<Vec<String>>()
                        .join("; ");

                    on_submit.run(choice_result(mistake_count.get_untracked() == 0, &answer_text));
                }

                return;
            }

            mistake_count.update(|count| *count += 1);
            flashing_wrong.set(vec![left_index, right_index]);
            set_timeout(move || flashing_wrong.set(Vec::new()), Duration::from_millis(WRONG_FLASH_MILLISECONDS));
        }
    };

    let left_buttons: Vec<_> = left_order
        .into_iter()
        .map(|pair_index| {
            let pair = pairs.pairs[pair_index].clone();
            let resolve_selection = resolve_selection.clone();
            let clip: AudioClip = audio_clip(&pair.clip_key, &pair.target, VoiceChoiceKind::OwnText);
            let pending: Memo<bool> = indicated_pending(playback_store, PlaybackTarget::for_clip(&clip, false));

            view! {
                <button
                    type="button"
                    class="choice"
                    class:selected=move || selected_left.get() == Some(pair_index)
                    class:right=move || matched_pair_indexes.get().contains(&pair_index)
                    class:wrong=move || selected_left.get().is_none() && flashing_wrong.get().contains(&pair_index)
                    class:pending=pending
                    aria-busy=move || pending.get().then_some("true")
                    disabled=move || matched_pair_indexes.get().contains(&pair_index)
                    on:click=move |_| {
                        selected_left.set(Some(pair_index));
                        playback_store.play_at_normal_speed(clip.clone());
                        resolve_selection();
                    }
                >
                    {pair.target.clone()}
                    {move || pending.get().then(|| view! { <PendingIcon /> })}
                </button>
            }
        })
        .collect();

    let right_buttons: Vec<_> = right_order
        .into_iter()
        .map(|pair_index| {
            let pair_text: String = pairs.pairs[pair_index].source.clone();
            let resolve_selection = resolve_selection.clone();

            view! {
                <button
                    type="button"
                    class="choice"
                    class:selected=move || selected_right.get() == Some(pair_index)
                    class:right=move || matched_pair_indexes.get().contains(&pair_index)
                    disabled=move || matched_pair_indexes.get().contains(&pair_index)
                    on:click=move |_| {
                        selected_right.set(Some(pair_index));
                        resolve_selection();
                    }
                >
                    {pair_text}
                </button>
            }
        })
        .collect();

    let prompt: String = pairs
        .prompt
        .clone()
        .unwrap_or_else(|| "Match each phrase with its translation".to_string());

    view! {
        <p class="prompt">{prompt}</p>
        <div class="match-columns">
            <div class="choices">{left_buttons}</div>
            <div class="choices">{right_buttons}</div>
        </div>
    }
}

/// How an open answer's feedback is shown.
#[derive(Debug, Clone, PartialEq)]
pub enum OpenAnswerOutcomeKind {
    Graded(GraderResponse),
    /// No model graded the answer, so the learner compares it with the example answers.
    SelfCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpenAnswerPhaseKind {
    Writing,
    /// Checked while the model was still loading.
    WaitingForModel,
    Grading,
    Answered,
}

impl OpenAnswerPhaseKind {
    fn pending_label(self) -> Option<&'static str> {
        match self {
            OpenAnswerPhaseKind::WaitingForModel => Some("Loading the model"),
            OpenAnswerPhaseKind::Grading => Some("Grading"),
            OpenAnswerPhaseKind::Writing | OpenAnswerPhaseKind::Answered => None,
        }
    }
}

/// Ends an open answer with its outcome, unless the session has gone in the meantime.
#[derive(Clone, Copy)]
struct OpenAnswerCompletion {
    phase: RwSignal<OpenAnswerPhaseKind>,
    outcome: RwSignal<Option<OpenAnswerOutcomeKind>>,
    on_submit: Callback<SavedExerciseResult>,
}

impl OpenAnswerCompletion {
    fn complete(self, outcome: OpenAnswerOutcomeKind, answer_text: String) {
        if self.phase.is_disposed() {
            return;
        }

        self.outcome.set(Some(outcome));
        self.phase.set(OpenAnswerPhaseKind::Answered);
        self.on_submit.run(SavedExerciseResult {
            status: AnswerStatus::Unscored,
            answer_text,
        });
    }

    fn grade(self, grading_store: GradingStore, submission: OpenAnswerSubmission) {
        self.phase.set(OpenAnswerPhaseKind::Grading);

        spawn_local(async move {
            let answer_text: String = submission.answer.clone();
            let graded: Result<GraderResponse, GradingFailureKind> = grading_store.grade(submission).await;

            let outcome: OpenAnswerOutcomeKind = match graded {
                Ok(response) => OpenAnswerOutcomeKind::Graded(response),
                Err(failure) => {
                    log::info!("showing the self-check for an answer the model did not grade; [failure={failure:?}]");
                    OpenAnswerOutcomeKind::SelfCheck
                },
            };

            self.complete(outcome, answer_text);
        });
    }
}

fn create_submission(grading_context: &OpenAnswerGradingContext, exercise: &OpenAnswerExercise, answer: &str) -> OpenAnswerSubmission {
    OpenAnswerSubmission {
        grader_prompt: grading_context.grader_prompt,
        level: grading_context.level.clone(),
        prompt_source: exercise.prompt_source.clone(),
        context_source: exercise.context_source.clone(),
        answer: answer.to_string(),
    }
}

/// Loads a downloaded model once while the question is open, then reads the grader's prompt into its cache once.
fn prepare_model_while_writing(
    grading_store: GradingStore,
    phase: RwSignal<OpenAnswerPhaseKind>,
    warm_up_submission: OpenAnswerSubmission,
) {
    let load_requested: StoredValue<bool> = StoredValue::new(false);
    let warmed_up: StoredValue<bool> = StoredValue::new(false);

    Effect::new(move |_| {
        if phase.get() != OpenAnswerPhaseKind::Writing {
            return;
        }

        match grading_store.state() {
            GraderStateKind::Downloaded | GraderStateKind::InUseInAnotherPage if !load_requested.get_value() => {
                load_requested.set_value(true);
                grading_store.load_model();
            },
            GraderStateKind::Ready if !warmed_up.get_value() => {
                warmed_up.set_value(true);
                grading_store.warm_up(warm_up_submission.clone());
            },
            _ => {},
        }
    });
}

/// Grades an answer checked during the model's load once the load ends, or falls back to the self-check.
fn grade_once_loaded(
    grading_store: GradingStore,
    completion: OpenAnswerCompletion,
    waiting_submission: StoredValue<Option<OpenAnswerSubmission>>,
) {
    Effect::new(move |_| {
        if completion.phase.get() != OpenAnswerPhaseKind::WaitingForModel {
            return;
        }

        let grader_state: GraderStateKind = grading_store.state();

        if grader_state == GraderStateKind::Loading {
            return;
        }

        let Some(submission) = waiting_submission.get_value()
        else {
            return;
        };

        match grader_state {
            GraderStateKind::Ready => completion.grade(grading_store, submission),
            _ => completion.complete(OpenAnswerOutcomeKind::SelfCheck, submission.answer),
        }
    });
}

#[component]
fn OpenAnswerCheckButton(phase: RwSignal<OpenAnswerPhaseKind>) -> impl IntoView {
    let pending_shown: RwSignal<bool> = RwSignal::new(false);

    Effect::new(move |_| {
        if phase.get().pending_label().is_some() && !pending_shown.get_untracked() {
            set_timeout(move || pending_shown.set(true), pending_indication::pending_indication_delay());
        }
    });

    let pending_label = move || phase.get().pending_label().filter(|_| pending_shown.get());

    view! {
        <p class="visually-hidden" role="status" aria-live="polite">
            {pending_label}
        </p>
        <button
            type="submit"
            class="primary check"
            class:pending=move || pending_label().is_some()
            aria-busy=move || pending_label().is_some().then_some("true")
            disabled=move || phase.get() != OpenAnswerPhaseKind::Writing
        >
            {move || pending_label().unwrap_or("Check")}
            {move || pending_label().is_some().then(|| view! { <PendingIcon /> })}
        </button>
    }
}

#[component]
fn OpenAnswerView(
    exercise: OpenAnswerExercise,
    on_submit: Callback<SavedExerciseResult>,
    outcome: RwSignal<Option<OpenAnswerOutcomeKind>>,
) -> impl IntoView {
    let written: RwSignal<String> = RwSignal::new(String::new());
    let phase: RwSignal<OpenAnswerPhaseKind> = RwSignal::new(OpenAnswerPhaseKind::Writing);
    let textarea_ref: NodeRef<html::Textarea> = NodeRef::new();
    let grading_store: GradingStore = use_grading_store();
    let grading_context: Option<OpenAnswerGradingContext> = use_context::<OpenAnswerGradingContext>();
    let completion: OpenAnswerCompletion = OpenAnswerCompletion {
        phase,
        outcome,
        on_submit,
    };
    let waiting_submission: StoredValue<Option<OpenAnswerSubmission>> = StoredValue::new(None);

    Effect::new(move |_| {
        if let Some(textarea) = textarea_ref.get() {
            _ = textarea.focus();
        }
    });

    if let Some(grading_context) = &grading_context {
        prepare_model_while_writing(grading_store, phase, create_submission(grading_context, &exercise, ""));
        grade_once_loaded(grading_store, completion, waiting_submission);
    }

    let graded_course: bool = grading_context.is_some();
    let submitted_exercise: OpenAnswerExercise = exercise.clone();
    let check = move || {
        let answer_text: String = written.get_untracked().trim().to_string();

        if phase.get_untracked() != OpenAnswerPhaseKind::Writing || answer_text.is_empty() {
            return;
        }

        let submission: Option<OpenAnswerSubmission> = grading_context
            .as_ref()
            .map(|grading_context| create_submission(grading_context, &submitted_exercise, &answer_text));

        match (submission, grading_store.state_untracked()) {
            (Some(submission), GraderStateKind::Ready) => completion.grade(grading_store, submission),
            (Some(submission), GraderStateKind::Loading) => {
                waiting_submission.set_value(Some(submission));
                phase.set(OpenAnswerPhaseKind::WaitingForModel);
            },
            _ => completion.complete(OpenAnswerOutcomeKind::SelfCheck, answer_text),
        }
    };

    // A form submits on Enter only from a single-line field, so Enter in the textarea starts a new line.
    view! {
        <form on:submit=move |event: SubmitEvent| {
            event.prevent_default();
            check();
        }>
            <p class="prompt">{exercise.prompt_source.clone()}</p>
            {exercise.context_source.clone().map(|context_source| view! { <p class="hint">{context_source}</p> })}
            <textarea
                node_ref=textarea_ref
                class="open-answer"
                rows=OPEN_ANSWER_ROWS
                autocomplete="off"
                autocapitalize="off"
                spellcheck="false"
                prop:value=move || written.get()
                on:input:target=move |event| written.set(event.target().value())
                disabled=move || phase.get() != OpenAnswerPhaseKind::Writing
            ></textarea>
            <OpenAnswerCheckButton phase />
            {move || {
                let shows_download_status: bool = graded_course && phase.get() != OpenAnswerPhaseKind::Answered;

                shows_download_status.then(|| view! { <ModelDownloadStatus /> })
            }}
        </form>
    }
}

#[component]
fn ExampleAnswers(exercise: OpenAnswerExercise) -> impl IntoView {
    let example_items: Vec<_> = exercise
        .example_answers
        .iter()
        .enumerate()
        .map(|(example_index, example_answer)| {
            // The exercise's clip speaks the first example answer.
            let play_button = (example_index == 0).then(|| {
                view! { <PlayButton clip_key=exercise.clip_key.clone() text=example_answer.clone() label="Hear it" /> }
            });

            view! {
                <li>
                    <span>{example_answer.clone()}</span>
                    {play_button}
                </li>
            }
        })
        .collect();

    view! {
        <div class="field-label">"Example answers"</div>
        <ul class="example-answers">{example_items}</ul>
        {exercise.explanation.clone().map(|explanation| view! { <p class="explanation">{explanation}</p> })}
    }
}

/// The feedback on an open answer: the grader's findings, or the example answers to compare with.
#[component]
pub fn OpenAnswerFeedback(exercise: OpenAnswerExercise, answer_text: String, outcome: Option<OpenAnswerOutcomeKind>) -> impl IntoView {
    let written_answer = view! {
        <div class="field-label">"Your answer"</div>
        <p class="written-answer">{answer_text}</p>
    };

    match outcome {
        Some(OpenAnswerOutcomeKind::Graded(response)) => {
            let target_language_name: String = use_context::<OpenAnswerGradingContext>()
                .map(|grading_context| grading_context.target_language_name)
                .unwrap_or_default();

            view! {
                <GradedFeedbackHeading heading=grading::mistake_count_heading(&response.findings) />
                {written_answer}
                <GraderFindings response target_language_name />
                <ExampleAnswers exercise />
            }
            .into_any()
        },
        Some(OpenAnswerOutcomeKind::SelfCheck) | None => view! {
            <strong>{SELF_CHECK_HEADING}</strong>
            {written_answer}
            <ExampleAnswers exercise />
            <SelfCheckNotice />
        }
        .into_any(),
    }
}
