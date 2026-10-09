use std::sync::Arc;

use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{HtmlCollection, HtmlElement, HtmlTableSectionElement};
use leptos_router::components::A;
use shared::course::{self, CourseOutline};
use shared::practice::normalize_answer_ignoring_accents;

use crate::audio_player::PlayButton;
use crate::course_data::{self, course_gate, use_route_course_id};
use crate::pending_indication::DelayedStatus;
use crate::progress_state::{remember_route_course, use_progress_store};

const ALL_LEVELS_CODE: &str = "all";
/// Rows built per animation frame, so the page paints and takes input between batches.
const GLOSSARY_ROWS_PER_FRAME: usize = 300;

struct GlossaryEntry {
    clip_key: Option<String>,
    target: String,
    source: String,
    source_literal: String,
    level: String,
    unit_href: String,
    unit_label: String,
    normalized_target: String,
    normalized_source: String,
}

impl GlossaryEntry {
    fn matches(&self, normalized_query: &str, level_filter: Option<&str>) -> bool {
        let level_matches: bool = level_filter.is_none_or(|level| self.level == level);
        let text_matches: bool =
            normalized_query.is_empty() || self.normalized_target.contains(normalized_query) || self.normalized_source.contains(normalized_query);

        level_matches && text_matches
    }
}

fn glossary_entries(course: &CourseOutline) -> Vec<GlossaryEntry> {
    course
        .units
        .iter()
        .flat_map(|unit| {
            unit.vocabulary.iter().map(move |item| GlossaryEntry {
                clip_key: item.clip_key.clone(),
                target: item.target.clone(),
                source: item.source.clone(),
                source_literal: item.source_literal.clone().unwrap_or_default(),
                level: unit.level.clone(),
                unit_href: course_data::unit_tab_path(&course.id, &unit.id, "vocabulary"),
                unit_label: format!("{} {}", unit.level, unit.order),
                normalized_target: normalize_answer_ignoring_accents(&item.target),
                normalized_source: normalize_answer_ignoring_accents(&item.source),
            })
        })
        .collect()
}

/// Hides the rows of `tbody`, in `entries` order, whose entry does not match.
fn hide_filtered_rows(tbody: &HtmlTableSectionElement, entries: &[GlossaryEntry], normalized_query: &str, level_filter: Option<&str>) {
    let rows: HtmlCollection = tbody.rows();

    for (row_index, entry) in entries.iter().enumerate().take(rows.length() as usize) {
        let row: Option<HtmlElement> = rows.item(row_index as u32).and_then(|row| row.dyn_into::<HtmlElement>().ok());
        let Some(row) = row
        else {
            continue;
        };

        row.set_hidden(!entry.matches(normalized_query, level_filter));
    }
}

fn add_rows_frame_by_frame(rendered_frame_count: RwSignal<usize>, frame_count: usize) {
    request_animation_frame(move || {
        let rendered: Option<usize> = rendered_frame_count.try_update(|count| {
            *count += 1;
            *count
        });
        let Some(rendered) = rendered
        else {
            return;
        };

        if rendered < frame_count {
            add_rows_frame_by_frame(rendered_frame_count, frame_count);
        }
    });
}

#[component]
pub fn GlossaryPage() -> impl IntoView {
    let course_id: Memo<String> = use_route_course_id();

    remember_route_course(use_progress_store(), course_id);

    course_gate(move || course_id.get(), |course: Arc<CourseOutline>| view! { <Glossary course /> })
}

#[component]
fn Glossary(course: Arc<CourseOutline>) -> impl IntoView {
    let target_name: String = course::language_name(&course.target_language);
    let source_name: String = course::language_name(&course.source_language);
    let search_text: RwSignal<String> = RwSignal::new(String::new());
    let level_code: RwSignal<String> = RwSignal::new(ALL_LEVELS_CODE.to_string());
    let level_options: Vec<_> = course
        .levels
        .iter()
        .map(|level| view! { <option value=level.code.clone()>{level.code.clone()}</option> })
        .collect();

    let entries: Arc<Vec<GlossaryEntry>> = Arc::new(glossary_entries(&course));
    let normalized_query: Memo<String> = Memo::new(move |_| normalize_answer_ignoring_accents(&search_text.get()));
    let level_filter: Memo<Option<String>> = Memo::new(move |_| Some(level_code.get()).filter(|code| code != ALL_LEVELS_CODE));
    let matching_count: Memo<usize> = {
        let entries: Arc<Vec<GlossaryEntry>> = entries.clone();

        Memo::new(move |_| {
            let normalized_query: String = normalized_query.get();
            let level_filter: Option<String> = level_filter.get();

            entries.iter().filter(|entry| entry.matches(&normalized_query, level_filter.as_deref())).count()
        })
    };

    // Every row is built once, a frame's worth at a time so no frame blocks for long, and hidden when filtered out.
    let frame_count: usize = entries.len().div_ceil(GLOSSARY_ROWS_PER_FRAME);
    let rendered_frame_count: RwSignal<usize> = RwSignal::new(0);
    let building: Memo<bool> = Memo::new(move |_| rendered_frame_count.get() < frame_count);

    Effect::new(move |_| {
        if frame_count > 0 {
            add_rows_frame_by_frame(rendered_frame_count, frame_count);
        }
    });

    let tbody_ref: NodeRef<html::Tbody> = NodeRef::new();

    // One pass over the rows, which costs far less than an effect per row.
    Effect::new({
        let entries: Arc<Vec<GlossaryEntry>> = entries.clone();

        move |_| {
            let normalized_query: String = normalized_query.get();
            let level_filter: Option<String> = level_filter.get();
            let Some(tbody) = tbody_ref.get_untracked()
            else {
                return;
            };

            hide_filtered_rows(&tbody, &entries, &normalized_query, level_filter.as_deref());
        }
    });

    let frame_rows = move |frame_index: usize| {
        let first_entry_index: usize = frame_index * GLOSSARY_ROWS_PER_FRAME;
        let end_entry_index: usize = (first_entry_index + GLOSSARY_ROWS_PER_FRAME).min(entries.len());

        (first_entry_index..end_entry_index)
            .map(|entry_index| glossary_row(&entries[entry_index], normalized_query, level_filter))
            .collect_view()
    };

    view! {
        <h1>"Glossary"</h1>
        <div class="row">
            <input
                type="search"
                placeholder=format!("Search {target_name} or {source_name}")
                prop:value=move || search_text.get()
                on:input:target=move |event| search_text.set(event.target().value())
            />
            <select on:change:target=move |event| level_code.set(event.target().value())>
                <option value=ALL_LEVELS_CODE>"All levels"</option>
                {level_options}
            </select>
        </div>
        <span class="page-caption">{move || format!("{} entries", matching_count.get())}</span>
        <table class="grid" aria-busy=move || building.get().then_some("true")>
            <thead><tr><th></th><th>{target_name.clone()}</th><th>{source_name.clone()}</th><th>"Literally"</th><th>"Unit"</th></tr></thead>
            <tbody node_ref=tbody_ref>
                <For each=move || 0..rendered_frame_count.get() key=|frame_index: &usize| *frame_index children=frame_rows />
            </tbody>
        </table>
        {move || building.get().then(|| view! { <DelayedStatus message="Building the glossary…" /> })}
    }
}

fn glossary_row(entry: &GlossaryEntry, normalized_query: Memo<String>, level_filter: Memo<Option<String>>) -> impl IntoView + use<> {
    let clip_key: Option<String> = entry.clip_key.clone();
    let target: String = entry.target.clone();
    let source: String = entry.source.clone();
    let source_literal: String = entry.source_literal.clone();
    let unit_href: String = entry.unit_href.clone();
    let unit_label: String = entry.unit_label.clone();
    let matches: bool =
        normalized_query.with_untracked(|query| level_filter.with_untracked(|level| entry.matches(query, level.as_deref())));

    view! {
        <tr hidden=!matches>
            <td><PlayButton clip_key text=target.clone() /></td>
            <td class="pt">{target}</td>
            <td>{source}</td>
            <td class="literal">{source_literal}</td>
            <td class="numeric"><A href=unit_href>{unit_label}</A></td>
        </tr>
    }
}
