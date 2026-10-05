use leptos::prelude::*;
use shared::progress::QueuedCards;
use shared::course::{Dialogue, GrammarNote, PhrasePair, Reading, Unit, VocabularyItem};

use crate::audio_player::{PlayAllButton, PlayButton, audio_clip};
use crate::platform::AudioClip;
use crate::practice::ExerciseSession;
use crate::progress_state::{ProgressStore, use_progress_store};

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
        <label class="toggle">
            <input
                type="checkbox"
                prop:checked=move || progress_store.read(|progress| progress.settings.show_translations)
                on:change=move |_| progress_store.update(|progress| progress.settings.show_translations = !progress.settings.show_translations)
            />
            " Show English"
        </label>
    }
}

#[component]
fn PhraseRow(pair: PhrasePair) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();

    view! {
        <li class="phrase">
            <PlayButton audio=pair.audio.clone() text=pair.pt.clone() />
            <span class="pt">{pair.pt.clone()}</span>
            <span class="en" class:hidden=move || !progress_store.read(|progress| progress.settings.show_translations)>
                {pair.en.clone()}
            </span>
        </li>
    }
}

#[component]
pub fn OverviewSection(unit: Unit) -> impl IntoView {
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
        <h2>"You will learn to"</h2>
        <ul>{objectives}</ul>
        <p>
            <button
                type="button"
                on:click=move |_| {
                    let queued: QueuedCards = progress_store.queue_unit_vocabulary_for_review(&unit_id, vocabulary_count);
                    added_message.set(Some(format!(
                        "{} new word(s) added and {} word(s) queued for review again.",
                        queued.added_count, queued.made_due_count,
                    )));
                }
            >
                {format!("Add the {vocabulary_count} words to my review deck")}
            </button>
            " "{move || added_message.get()}
        </p>
        <h2>{unit.culture.title.clone()}</h2>
        {paragraphs(&unit.culture.text)}
    }
}

#[component]
fn VocabularyRow(item: VocabularyItem) -> impl IntoView {
    let part_of_speech = item.pos.clone().map(|pos| view! { <span class="muted">" "{pos}</span> });
    let note = item.note.clone().map(|note| view! { <div class="note">{note}</div> });

    view! {
        <li class="vocabulary-item">
            <div class="vocabulary-main">
                <PlayButton audio=item.audio.clone() text=item.pt.clone() />
                <strong class="pt">{item.pt.clone()}</strong>
                {part_of_speech}
                <span class="en">" — "{item.en.clone()}</span>
            </div>
            <div class="example">
                <PlayButton audio=item.example.audio.clone() text=item.example.pt.clone() />
                <span class="pt">{item.example.pt.clone()}</span>
                <span class="en muted">" "{item.example.en.clone()}</span>
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
fn DialogueView(dialogue: Dialogue) -> impl IntoView {
    let clips: Vec<AudioClip> = dialogue.lines.iter().map(|line| audio_clip(&line.audio, &line.pt)).collect();
    let lines: Vec<_> = dialogue
        .lines
        .iter()
        .map(|line| {
            let speaker: String = line.speaker.clone();

            view! {
                <li class="dialogue-line">
                    <span class="speaker">{speaker}</span>
                    <PhraseRow pair=PhrasePair { pt: line.pt.clone(), en: line.en.clone(), audio: line.audio.clone() } />
                </li>
            }
        })
        .collect();

    view! {
        <article class="dialogue">
            <h3>{dialogue.title.clone()}</h3>
            <p class="muted">{dialogue.context.clone()}</p>
            <PlayAllButton clips />
            <ul class="lines">{lines}</ul>
        </article>
    }
}

#[component]
pub fn DialoguesSection(unit: Unit) -> impl IntoView {
    let dialogues: Vec<_> = unit
        .dialogues
        .iter()
        .map(|dialogue| view! { <DialogueView dialogue=dialogue.clone() /> })
        .collect();

    view! {
        <p><TranslationToggle /></p>
        {dialogues}
    }
}

#[component]
fn GrammarNoteView(note: GrammarNote) -> impl IntoView {
    let table = note.table.clone().map(|table| {
        let header_cells: Vec<_> = table.headers.iter().map(|header| view! { <th>{header.clone()}</th> }).collect();
        let body_rows: Vec<_> = table
            .rows
            .iter()
            .map(|row| {
                let cells: Vec<_> = row.iter().map(|cell| view! { <td>{cell.clone()}</td> }).collect();

                view! { <tr>{cells}</tr> }
            })
            .collect();

        view! {
            <table class="grid">
                <thead><tr>{header_cells}</tr></thead>
                <tbody>{body_rows}</tbody>
            </table>
        }
    });
    let examples: Vec<_> = note.examples.iter().map(|example| view! { <PhraseRow pair=example.clone() /> }).collect();

    view! {
        <article class="grammar-note">
            <h3>{note.title.clone()}</h3>
            {paragraphs(&note.explanation)}
            {table}
            <ul class="lines">{examples}</ul>
        </article>
    }
}

#[component]
pub fn GrammarSection(unit: Unit) -> impl IntoView {
    let notes: Vec<_> = unit.grammar.iter().map(|note| view! { <GrammarNoteView note=note.clone() /> }).collect();

    view! {
        <p><TranslationToggle /></p>
        {notes}
    }
}

#[component]
fn ReadingView(reading: Reading) -> impl IntoView {
    let questions_open: RwSignal<bool> = RwSignal::new(false);
    let clips: Vec<AudioClip> = reading.paragraphs.iter().map(|paragraph| audio_clip(&paragraph.audio, &paragraph.pt)).collect();
    let paragraph_rows: Vec<_> = reading
        .paragraphs
        .iter()
        .map(|paragraph| view! { <PhraseRow pair=paragraph.clone() /> })
        .collect();
    let questions = reading.questions.clone();
    let title: String = reading.title.clone();

    view! {
        <article class="reading">
            <h3>{reading.title.clone()}</h3>
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
        </article>
    }
}

#[component]
pub fn ReadingsSection(unit: Unit) -> impl IntoView {
    let readings: Vec<_> = unit.readings.iter().map(|reading| view! { <ReadingView reading=reading.clone() /> }).collect();

    view! {
        <p><TranslationToggle /></p>
        {readings}
    }
}
