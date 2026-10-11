use leptos::prelude::*;
use shared::course::{Dialogue, GrammarNote, PhrasePair, Reading, Unit, VocabularyItem};
use shared::progress::{QueuedCards, VoiceChoiceKind};

use crate::audio_player::{PlayAllButton, PlayButton, audio_clip};
use crate::platform::AudioClip;
use crate::practice::ExerciseSession;
use crate::progress_state::{ProgressStore, use_progress_store};
use crate::unit::{GrammarTableView, UnitItemArticle, UnitItemIndex, UnitItemKind, unit_item_index};

pub fn paragraphs(text: &str) -> Vec<impl IntoView + use<>> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|paragraph| !paragraph.is_empty())
        .map(|paragraph| view! { <p>{paragraph.to_string()}</p> })
        .collect()
}

#[component]
pub fn TranslationToggle() -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();

    view! {
        <label class="field">
            <input
                type="checkbox"
                prop:checked=move || progress_store.read(|progress| progress.settings.show_translations)
                on:change=move |_| progress_store.update(|progress| progress.settings.show_translations = !progress.settings.show_translations)
            />
            <span class="field-label">"Show translations"</span>
        </label>
    }
}

#[component]
fn PhraseRow(pair: PhrasePair, #[prop(optional)] voice_choice: VoiceChoiceKind) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let literal = pair.source_literal.clone().map(|literal| {
        view! {
            <span class="literal" class:hidden=move || !progress_store.read(|progress| progress.settings.show_translations)>
                "literally: "{literal}
            </span>
        }
    });

    view! {
        <li class="phrase">
            <PlayButton clip_key=pair.clip_key.clone() text=pair.target.clone() voice_choice />
            <span class="pt">{pair.target.clone()}</span>
            <span class="en" class:hidden=move || !progress_store.read(|progress| progress.settings.show_translations)>
                {pair.source.clone()}
            </span>
            {literal}
        </li>
    }
}

#[component]
pub fn OverviewSection(course_id: String, unit: Unit) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let unit_id: String = unit.id.clone();
    let vocabulary_count: usize = unit.vocabulary.len();
    let added_message: RwSignal<Option<String>> = RwSignal::new(None);

    let objectives: Vec<_> = unit
        .objectives
        .iter()
        .map(|objective| view! { <li>{objective.clone()}</li> })
        .collect();

    view! {
        <section class="lesson-section">
        <h2>"You will learn to"</h2>
        <ul>{objectives}</ul>
        <div class="row">
            <button
                type="button"
                on:click=move |_| {
                    let queued: QueuedCards = progress_store.queue_unit_vocabulary_for_review(&course_id, &unit_id, vocabulary_count);
                    added_message.set(Some(format!(
                        "{} new word(s) added and {} word(s) queued for review again.",
                        queued.added_count, queued.made_due_count,
                    )));
                }
            >
                {format!("Add the {vocabulary_count} words to my review deck")}
            </button>
            <span class="muted">{move || added_message.get()}</span>
        </div>
        </section>
        <section class="lesson-section">
            <h2>{unit.culture.title.clone()}</h2>
            {paragraphs(&unit.culture.text)}
        </section>
    }
}

#[component]
fn VocabularyRow(item: VocabularyItem) -> impl IntoView {
    let part_of_speech = item.part_of_speech.clone().map(|part_of_speech| view! { <span class="tag">{part_of_speech}</span> });
    let note = item.note.clone().map(|note| view! { <div class="note">{note}</div> });
    let literal = item.source_literal.clone().map(|literal| view! { <div class="literal">"literally: "{literal}</div> });

    view! {
        <li class="vocabulary-item">
            <div class="vocabulary-main">
                <PlayButton clip_key=item.clip_key.clone() text=item.target.clone() />
                <strong class="pt">{item.target.clone()}</strong>
                {part_of_speech}
                <span class="en">{item.source.clone()}</span>
            </div>
            {literal}
            <div class="example">
                <PlayButton clip_key=item.example.clip_key.clone() text=item.example.target.clone() />
                <span class="pt">{item.example.target.clone()}</span>
                <span class="en muted">" "{item.example.source.clone()}</span>
            </div>
            {note}
        </li>
    }
}

#[component]
pub fn VocabularySection(unit: Unit) -> impl IntoView {
    let rows: Vec<_> = unit.vocabulary.iter().map(|item| view! { <VocabularyRow item=item.clone() /> }).collect();

    view! { <ul class="vocabulary">{rows}</ul> }
}

#[component]
fn DialogueView(dialogue: Dialogue, label: String) -> impl IntoView {
    let voice_choice: VoiceChoiceKind = dialogue.voice_choice();
    let clips: Vec<AudioClip> = dialogue
        .lines
        .iter()
        .map(|line| audio_clip(&line.clip_key, &line.target, voice_choice.clone()))
        .collect();
    let lines: Vec<_> = dialogue
        .lines
        .iter()
        .map(|line| {
            let speaker: String = line.speaker.clone();

            view! {
                <li class="dialogue-line">
                    <span class="speaker">{speaker}</span>
                    <PhraseRow
                        pair=PhrasePair { target: line.target.clone(), source: line.source.clone(), source_literal: line.source_literal.clone(), clip_key: line.clip_key.clone() }
                        voice_choice=voice_choice.clone()
                    />
                </li>
            }
        })
        .collect();

    view! {
        <UnitItemArticle kind_class="dialogue" label title=dialogue.title.clone()>
            <p class="muted">{dialogue.context.clone()}</p>
            <PlayAllButton clips />
            <ul class="lines">{lines}</ul>
        </UnitItemArticle>
    }
}

#[component]
pub fn DialoguesSection(unit: Unit) -> impl IntoView {
    let labels: Vec<String> = unit_item_index::unit_item_labels(&unit, unit.dialogues.len());
    let dialogues: Vec<_> = unit
        .dialogues
        .iter()
        .zip(labels.iter().cloned())
        .map(|(dialogue, label)| view! { <DialogueView dialogue=dialogue.clone() label /> })
        .collect();

    view! {
        <div class="row"><TranslationToggle /></div>
        {dialogues}
        <UnitItemIndex item_kind=UnitItemKind::Dialogue labels />
    }
}

#[component]
fn GrammarNoteView(note: GrammarNote, label: String) -> impl IntoView {
    let table = note.table.clone().map(|table| view! { <GrammarTableView table /> });
    let examples: Vec<_> = note.examples.iter().map(|example| view! { <PhraseRow pair=example.clone() /> }).collect();

    view! {
        <UnitItemArticle kind_class="grammar-note" label title=note.title.clone()>
            {paragraphs(&note.explanation)}
            {table}
            <ul class="lines">{examples}</ul>
        </UnitItemArticle>
    }
}

#[component]
pub fn GrammarSection(unit: Unit) -> impl IntoView {
    let labels: Vec<String> = unit_item_index::unit_item_labels(&unit, unit.grammar.len());
    let notes: Vec<_> = unit
        .grammar
        .iter()
        .zip(labels.iter().cloned())
        .map(|(note, label)| view! { <GrammarNoteView note=note.clone() label /> })
        .collect();

    view! {
        <div class="row"><TranslationToggle /></div>
        {notes}
        <UnitItemIndex item_kind=UnitItemKind::GrammarNote labels />
    }
}

#[component]
fn ReadingView(reading: Reading, label: String) -> impl IntoView {
    let questions_open: RwSignal<bool> = RwSignal::new(false);
    let clips: Vec<AudioClip> = reading
        .paragraphs
        .iter()
        .map(|paragraph| audio_clip(&paragraph.clip_key, &paragraph.target, VoiceChoiceKind::OwnText))
        .collect();
    let paragraph_rows: Vec<_> = reading
        .paragraphs
        .iter()
        .map(|paragraph| view! { <PhraseRow pair=paragraph.clone() /> })
        .collect();
    let questions = reading.questions.clone();
    let title: String = reading.title.clone();

    view! {
        <UnitItemArticle kind_class="reading" label title=reading.title.clone()>
            <PlayAllButton clips />
            <ul class="lines paragraphs">{paragraph_rows}</ul>
            {move || {
                if questions_open.get() {
                    view! {
                        <ExerciseSession
                            title=format!("{title}: questions")
                            exercises=questions.clone()
                            on_complete=Callback::new(|_| {})
                            on_exit=Callback::new(move |()| questions_open.set(false))
                        />
                    }
                    .into_any()
                } else {
                    view! {
                        <button type="button" class="primary" on:click=move |_| questions_open.set(true)>
                            "Answer the comprehension questions"
                        </button>
                    }
                    .into_any()
                }
            }}
        </UnitItemArticle>
    }
}

#[component]
pub fn ReadingsSection(unit: Unit) -> impl IntoView {
    let labels: Vec<String> = unit_item_index::unit_item_labels(&unit, unit.readings.len());
    let readings: Vec<_> = unit
        .readings
        .iter()
        .zip(labels.iter().cloned())
        .map(|(reading, label)| view! { <ReadingView reading=reading.clone() label /> })
        .collect();

    view! {
        <div class="row"><TranslationToggle /></div>
        {readings}
        <UnitItemIndex item_kind=UnitItemKind::Reading labels />
    }
}
