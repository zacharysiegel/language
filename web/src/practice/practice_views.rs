use std::sync::Arc;
use std::time::Duration;

use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use shared::course::{
    DictationExercise, Exercise, FillBlankExercise, ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise,
    OrderWordsExercise, TranslateToTargetExercise,
};
use shared::practice::{AnswerStatus, check_typed_answer, shuffled, shuffled_avoiding_identity};
use shared::progress::VoiceChoiceKind;

use crate::audio_player::{PlayButton, audio_clip, play_in_background};
use crate::platform::{self, AudioClip};

const WRONG_FLASH_MILLISECONDS: u64 = 500;

#[derive(Debug, Clone, PartialEq)]
pub struct ExerciseResult {
    pub status: AnswerStatus,
    pub answer_text: String,
}

impl ExerciseResult {
    pub fn is_correct(&self) -> bool {
        self.status.is_correct()
    }

    fn from_choice(chosen_correctly: bool, answer_text: &str) -> ExerciseResult {
        let status: AnswerStatus = if chosen_correctly { AnswerStatus::Correct } else { AnswerStatus::Wrong };

        ExerciseResult {
            status,
            answer_text: answer_text.to_string(),
        }
    }
}

pub fn exercise_body(exercise: Exercise, on_submit: Callback<ExerciseResult>) -> AnyView {
    match exercise {
        Exercise::MultipleChoice(exercise) => view! { <MultipleChoiceView exercise on_submit /> }.into_any(),
        Exercise::FillBlank(exercise) => view! { <FillBlankView exercise on_submit /> }.into_any(),
        Exercise::TranslateToTarget(exercise) => view! { <TranslateView exercise on_submit /> }.into_any(),
        Exercise::OrderWords(exercise) => view! { <OrderWordsView exercise on_submit /> }.into_any(),
        Exercise::ListeningChoice(exercise) => view! { <ListeningChoiceView exercise on_submit /> }.into_any(),
        Exercise::Dictation(exercise) => view! { <DictationView exercise on_submit /> }.into_any(),
        Exercise::Match(exercise) => view! { <MatchView exercise on_submit /> }.into_any(),
    }
}

fn choice_buttons(
    choices: &[String],
    answer_index: usize,
    answer_text: String,
    on_submit: Callback<ExerciseResult>,
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
                        on_submit.run(ExerciseResult::from_choice(choice_index == answer_index, &answer_text));
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
fn MultipleChoiceView(exercise: MultipleChoiceExercise, on_submit: Callback<ExerciseResult>) -> impl IntoView {
    let answer_text: String = exercise.choices[exercise.answer_index].clone();

    view! {
        <p class="prompt">{exercise.prompt.clone()}</p>
        {choice_buttons(&exercise.choices, exercise.answer_index, answer_text, on_submit)}
    }
}

#[component]
fn ListeningChoiceView(exercise: ListeningChoiceExercise, on_submit: Callback<ExerciseResult>) -> impl IntoView {
    let answer_text: String = format!("{} = {}", exercise.audio_target, exercise.choices[exercise.answer_index]);
    let autoplay_clip: AudioClip = audio_clip(&exercise.clip_key, &exercise.audio_target, VoiceChoiceKind::OwnText);

    Effect::new(move |_| play_in_background(autoplay_clip.clone(), false));

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
    on_submit: Callback<ExerciseResult>,
) {
    let typed_text: String = typed.get_untracked();

    if answered.get_untracked() || typed_text.trim().is_empty() {
        return;
    }

    answered.set(true);

    let status: AnswerStatus = check_typed_answer(&typed_text, accepted_answers);

    on_submit.run(ExerciseResult {
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
fn FillBlankView(exercise: FillBlankExercise, on_submit: Callback<ExerciseResult>) -> impl IntoView {
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
fn TranslateView(exercise: TranslateToTargetExercise, on_submit: Callback<ExerciseResult>) -> impl IntoView {
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
fn DictationView(exercise: DictationExercise, on_submit: Callback<ExerciseResult>) -> impl IntoView {
    let typed: RwSignal<String> = RwSignal::new(String::new());
    let answered: RwSignal<bool> = RwSignal::new(false);
    let answer_text: String = format!("{} = {}", exercise.audio_target, exercise.hint_source);
    let accepted_answers: Vec<String> = vec![exercise.audio_target.clone()];
    let autoplay_clip: AudioClip = audio_clip(&exercise.clip_key, &exercise.audio_target, VoiceChoiceKind::OwnText);

    Effect::new(move |_| play_in_background(autoplay_clip.clone(), false));

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
fn OrderWordsView(exercise: OrderWordsExercise, on_submit: Callback<ExerciseResult>) -> impl IntoView {
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

        on_submit.run(ExerciseResult::from_choice(attempt == sentence, &sentence));
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
fn MatchView(exercise: MatchExercise, on_submit: Callback<ExerciseResult>) -> impl IntoView {
    let pairs: Arc<MatchExercise> = Arc::new(exercise);
    let pair_indexes: Vec<usize> = (0..pairs.pairs.len()).collect();
    let left_order: Vec<usize> = shuffled(&pair_indexes, platform::random_unit);
    let right_order: Vec<usize> = shuffled(&pair_indexes, platform::random_unit);
    let selected_left: RwSignal<Option<usize>> = RwSignal::new(None);
    let selected_right: RwSignal<Option<usize>> = RwSignal::new(None);
    let matched_pair_indexes: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());
    let flashing_wrong: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());
    let mistake_count: RwSignal<u32> = RwSignal::new(0);

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

                    on_submit.run(ExerciseResult::from_choice(mistake_count.get_untracked() == 0, &answer_text));
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

            view! {
                <button
                    type="button"
                    class="choice"
                    class:selected=move || selected_left.get() == Some(pair_index)
                    class:right=move || matched_pair_indexes.get().contains(&pair_index)
                    class:wrong=move || selected_left.get().is_none() && flashing_wrong.get().contains(&pair_index)
                    disabled=move || matched_pair_indexes.get().contains(&pair_index)
                    on:click=move |_| {
                        selected_left.set(Some(pair_index));
                        play_in_background(clip.clone(), false);
                        resolve_selection();
                    }
                >
                    {pair.target.clone()}
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
