use std::sync::Arc;

use chrono::NaiveDate;
use leptos::prelude::*;
use shared::course::{Course, Unit, VocabularyItem};
use shared::progress::ReviewDirectionKind;
use shared::review::{ReviewCard, select_due_cards};

use crate::audio_player::{PlayButton, audio_clip, play_in_background};
use crate::course_data::course_gate;
use crate::platform::{self, AudioClip};
use crate::progress_state::{ProgressStore, use_progress_store};

const SESSION_CARD_LIMIT: usize = 20;
const RANDOM_DIRECTION_THRESHOLD: f64 = 0.5;

#[derive(Debug, Clone, PartialEq)]
struct ReviewItem {
    card_id: String,
    prompt_is_portuguese: bool,
}

#[component]
pub fn ReviewPage() -> impl IntoView {
    course_gate(|course: Arc<Course>| view! { <ReviewSetup course /> })
}

fn direction_from_code(code: &str) -> ReviewDirectionKind {
    match code {
        "english_to_portuguese" => ReviewDirectionKind::EnglishToPortuguese,
        "mixed" => ReviewDirectionKind::Mixed,
        _ => ReviewDirectionKind::PortugueseToEnglish,
    }
}

fn build_session_items(progress_store: ProgressStore) -> Vec<ReviewItem> {
    let now_milliseconds: i64 = platform::now_milliseconds();

    progress_store.read(|progress| {
        let direction: ReviewDirectionKind = progress.settings.review_direction;

        select_due_cards(progress.cards.values(), now_milliseconds, SESSION_CARD_LIMIT)
            .into_iter()
            .map(|card| ReviewItem {
                card_id: card.id.clone(),
                prompt_is_portuguese: match direction {
                    ReviewDirectionKind::PortugueseToEnglish => true,
                    ReviewDirectionKind::EnglishToPortuguese => false,
                    ReviewDirectionKind::Mixed => platform::random_unit() < RANDOM_DIRECTION_THRESHOLD,
                },
            })
            .collect()
    })
}

#[component]
fn ReviewSetup(course: Arc<Course>) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let session_items: RwSignal<Option<Vec<ReviewItem>>> = RwSignal::new(None);

    let due_count = move || {
        let now_milliseconds: i64 = platform::now_milliseconds();

        progress_store.read(|progress| progress.cards.values().filter(|card| card.due_milliseconds <= now_milliseconds).count())
    };
    let total_count = move || progress_store.read(|progress| progress.cards.len());
    let current_direction = move || progress_store.read(|progress| progress.settings.review_direction);

    view! {
        {move || match session_items.get() {
            Some(items) => view! {
                <ReviewSession
                    course=course.clone()
                    items
                    on_exit=Callback::new(move |()| session_items.set(None))
                />
            }
            .into_any(),
            None => view! {
                <h1>"Review"</h1>
                <div class="review-count">
                    <span class="review-figure numeric">{move || due_count()}</span>
                    <span>"words ready to review now"</span>
                    <span class="muted numeric">{move || total_count().saturating_sub(due_count())}" scheduled for later"</span>
                </div>
                <p class="muted">
                    "A word is ready when its next review time has come. Words you remember come back after a longer wait each time; words you forget come back in a minute. The number next to Review in the menu is the count of words ready now."
                </p>
                <div class="row">
                    <label class="field">
                        <span class="field-label">"Direction"</span>
                        <select
                            on:change:target=move |event| {
                                let direction: ReviewDirectionKind = direction_from_code(&event.target().value());

                                progress_store.update(|progress| progress.settings.review_direction = direction);
                            }
                        >
                            <option value="portuguese_to_english" selected=move || current_direction() == ReviewDirectionKind::PortugueseToEnglish>
                                "Portuguese to English"
                            </option>
                            <option value="english_to_portuguese" selected=move || current_direction() == ReviewDirectionKind::EnglishToPortuguese>
                                "English to Portuguese"
                            </option>
                            <option value="mixed" selected=move || current_direction() == ReviewDirectionKind::Mixed>"Mixed"</option>
                        </select>
                    </label>
                </div>
                {move || {
                    if due_count() == 0 {
                        view! { <p class="muted">"No words are ready. Add a unit's words to your deck from its overview page, or pass its practice; words you have already reviewed return when their wait is over."</p> }.into_any()
                    } else {
                        view! {
                            <button
                                type="button"
                                class="primary"
                                on:click=move |_| session_items.set(Some(build_session_items(progress_store)))
                            >
                                "Start review"
                            </button>
                        }
                        .into_any()
                    }
                }}
            }
            .into_any(),
        }}
    }
}

#[component]
fn ReviewSession(course: Arc<Course>, items: Vec<ReviewItem>, on_exit: Callback<()>) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let items: Arc<Vec<ReviewItem>> = Arc::new(items);
    let current_index: RwSignal<usize> = RwSignal::new(0);
    let revealed: RwSignal<bool> = RwSignal::new(false);
    let remembered_count: RwSignal<usize> = RwSignal::new(0);
    let total_count: usize = items.len();

    let grade_current = {
        let items: Arc<Vec<ReviewItem>> = items.clone();

        move |remembered: bool| {
            let Some(item) = items.get(current_index.get_untracked()) else {
                return;
            };
            let now_milliseconds: i64 = platform::now_milliseconds();
            let today: NaiveDate = platform::today();

            progress_store.update(|progress| {
                let graded: Option<ReviewCard> = progress.cards.get(&item.card_id).map(|card| card.graded(remembered, now_milliseconds));

                if let Some(graded) = graded {
                    progress.cards.insert(item.card_id.clone(), graded);
                }

                progress.record_review_activity(today, 1);
            });

            if remembered {
                remembered_count.update(|count| *count += 1);
            }

            revealed.set(false);
            current_index.update(|index| *index += 1);
        }
    };

    let card_items: Arc<Vec<ReviewItem>> = items.clone();
    let card_course: Arc<Course> = course.clone();

    view! {
        <h1>"Review"</h1>
        {move || {
            let index: usize = current_index.get();

            match card_items.get(index) {
                Some(item) => match card_course.find_vocabulary(&item.card_id) {
                    Some((unit, vocabulary_item)) => card_view(
                        unit,
                        vocabulary_item,
                        item.prompt_is_portuguese,
                        index,
                        total_count,
                        revealed,
                        grade_current.clone(),
                        on_exit,
                    )
                    .into_any(),
                    None => view! { <p class="status error">"This card refers to a word that is no longer in the course."</p> }.into_any(),
                },
                None => view! {
                    <h2>"Session complete"</h2>
                    <p>{format!("You remembered {} of {total_count}.", remembered_count.get())}</p>
                    <button type="button" class="primary" on:click=move |_| on_exit.run(())>"Done"</button>
                }
                .into_any(),
            }
        }}
    }
}

struct CardProgress {
    index: usize,
    total_count: usize,
}

fn card_view(
    unit: &Unit,
    item: &VocabularyItem,
    prompt_is_portuguese: bool,
    index: usize,
    total_count: usize,
    revealed: RwSignal<bool>,
    grade: impl Fn(bool) + Clone + Send + Sync + 'static,
    on_exit: Callback<()>,
) -> impl IntoView {
    let card_progress: CardProgress = CardProgress { index, total_count };
    let autoplay_clip: AudioClip = audio_clip(&item.audio, &item.pt);

    Effect::new(move |_| {
        if prompt_is_portuguese || revealed.get() {
            play_in_background(autoplay_clip.clone(), false);
        }
    });

    let prompt_text: String = if prompt_is_portuguese { item.pt.clone() } else { item.en.clone() };
    let answer_text: String = if prompt_is_portuguese { item.en.clone() } else { item.pt.clone() };
    let unit_title: String = unit.title.clone();
    let item_audio: Option<String> = item.audio.clone();
    let item_portuguese: String = item.pt.clone();
    let example_audio: Option<String> = item.example.audio.clone();
    let example_portuguese: String = item.example.pt.clone();
    let example_english: String = item.example.en.clone();
    let note: Option<String> = item.note.clone();
    let literal: Option<String> = item.literal.clone();
    let grade_forgot = grade.clone();

    view! {
        <div class="session-header">
            <span>{format!("Card {} of {}", card_progress.index + 1, card_progress.total_count)}</span>
            <button type="button" class="link" on:click=move |_| on_exit.run(())>"Quit"</button>
        </div>
        <progress max=card_progress.total_count value=card_progress.index></progress>
        <div class="flashcard">
            <div class="muted">{unit_title}</div>
            <div class="flashcard-prompt">{prompt_text}</div>
            {move || {
                if revealed.get() {
                    let note = note.clone();
                    let grade_remembered = grade.clone();
                    let grade_forgot = grade_forgot.clone();

                    view! {
                        <div class="flashcard-answer">
                            <PlayButton audio=item_audio.clone() text=item_portuguese.clone() />
                            {answer_text.clone()}
                        </div>
                        {literal.clone().map(|literal| view! { <div class="literal">"literally: "{literal}</div> })}
                        <div class="example">
                            <PlayButton audio=example_audio.clone() text=example_portuguese.clone() />
                            {example_portuguese.clone()}" "<span class="muted">{example_english.clone()}</span>
                        </div>
                        {note.map(|note| view! { <div class="note">{note}</div> })}
                        <div class="row">
                            <button type="button" on:click=move |_| grade_forgot(false)>"Forgot"</button>
                            <button type="button" class="primary" on:click=move |_| grade_remembered(true)>"Remembered"</button>
                        </div>
                    }
                    .into_any()
                } else {
                    view! { <button type="button" class="primary" on:click=move |_| revealed.set(true)>"Show answer"</button> }.into_any()
                }
            }}
        </div>
    }
}
