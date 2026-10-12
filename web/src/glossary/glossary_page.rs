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
use crate::item_index::{ItemIndex, ItemIndexEntry, item_index};
use crate::pending_indication::DelayedStatus;
use crate::progress_state::{remember_route_course, use_progress_store};

const ALL_LEVELS_CODE: &str = "all";
const LEVEL_INDEX_LABEL: &str = "Level index";
const UNIT_INDEX_LABEL: &str = "Unit index";
/// Rows built per animation frame, so the page paints and takes input between batches.
const GLOSSARY_ROWS_PER_FRAME: usize = 300;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GlossaryRailKind {
    Levels,
    Units,
}

impl GlossaryRailKind {
    fn for_level_filter(level_filter: Option<&str>) -> Self {
        match level_filter {
            None => GlossaryRailKind::Levels,
            Some(_) => GlossaryRailKind::Units,
        }
    }

    fn index_label(self) -> &'static str {
        match self {
            GlossaryRailKind::Levels => LEVEL_INDEX_LABEL,
            GlossaryRailKind::Units => UNIT_INDEX_LABEL,
        }
    }
}

struct GlossaryEntry {
    clip_key: Option<String>,
    target: String,
    source: String,
    source_literal: String,
    level: String,
    unit_id: String,
    unit_order: u32,
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

    /// Entries sharing a key share one rail entry.
    fn rail_key(&self, rail_kind: GlossaryRailKind) -> &str {
        match rail_kind {
            GlossaryRailKind::Levels => &self.level,
            GlossaryRailKind::Units => &self.unit_id,
        }
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
                unit_id: unit.id.clone(),
                unit_order: unit.order,
                unit_href: course_data::unit_tab_path(&course.id, &unit.id, "vocabulary"),
                unit_label: format!("{} {}", unit.level, unit.order),
                normalized_target: normalize_answer_ignoring_accents(&item.target),
                normalized_source: normalize_answer_ignoring_accents(&item.source),
            })
        })
        .collect()
}

/// The index of the first matching entry of each rail entry's key, in `entries` order.
fn first_matching_entry_index_per_rail_entry(
    entries: &[GlossaryEntry],
    rail_kind: GlossaryRailKind,
    normalized_query: &str,
    level_filter: Option<&str>,
) -> Vec<usize> {
    let mut indexed_keys: Vec<&str> = Vec::new();
    let mut first_entry_indices: Vec<usize> = Vec::new();

    for (entry_index, entry) in entries.iter().enumerate() {
        let rail_key: &str = entry.rail_key(rail_kind);
        let first_of_its_key: bool = !indexed_keys.contains(&rail_key);

        if first_of_its_key && entry.matches(normalized_query, level_filter) {
            indexed_keys.push(rail_key);
            first_entry_indices.push(entry_index);
        }
    }

    first_entry_indices
}

fn rail_entries(entries: &[GlossaryEntry], rail_kind: GlossaryRailKind, normalized_query: &str, level_filter: Option<&str>) -> Vec<ItemIndexEntry> {
    first_matching_entry_index_per_rail_entry(entries, rail_kind, normalized_query, level_filter)
        .into_iter()
        .map(|entry_index| rail_entry(entry_index, &entries[entry_index], rail_kind))
        .collect()
}

fn rail_entry(entry_index: usize, entry: &GlossaryEntry, rail_kind: GlossaryRailKind) -> ItemIndexEntry {
    let (visible_text, accessible_label): (String, String) = match rail_kind {
        GlossaryRailKind::Levels => (entry.level.clone(), format!("Go to level {}", entry.level)),
        GlossaryRailKind::Units => (entry.unit_order.to_string(), format!("Go to unit {}", entry.unit_label)),
    };

    ItemIndexEntry {
        target_element_id: glossary_row_element_id(entry_index),
        visible_text,
        accessible_label,
    }
}

fn glossary_row_element_id(entry_index: usize) -> String {
    format!("glossary-entry-{entry_index}")
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
    let rail_kind: Memo<GlossaryRailKind> = Memo::new(move |_| level_filter.with(|level| GlossaryRailKind::for_level_filter(level.as_deref())));
    let rail_index: Memo<Vec<ItemIndexEntry>> = {
        let entries: Arc<Vec<GlossaryEntry>> = entries.clone();

        Memo::new(move |_| {
            normalized_query.with(|query| level_filter.with(|level| rail_entries(&entries, rail_kind.get(), query, level.as_deref())))
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
            .map(|entry_index| glossary_row(entry_index, &entries[entry_index], normalized_query, level_filter))
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
        <div class="table-scroll">
            <table class="grid glossary-table" aria-busy=move || building.get().then_some("true")>
                <thead><tr><th></th><th>{target_name.clone()}</th><th>{source_name.clone()}</th><th>"Literally"</th><th>"Unit"</th></tr></thead>
                <tbody node_ref=tbody_ref>
                    <For each=move || 0..rendered_frame_count.get() key=|frame_index: &usize| *frame_index children=frame_rows />
                </tbody>
            </table>
        </div>
        {move || building.get().then(|| view! { <DelayedStatus message="Building the glossary…" /> })}
        {move || view! { <ItemIndex index_label=rail_kind.get().index_label() entries=rail_index.get() /> }}
    }
}

fn glossary_row(
    entry_index: usize,
    entry: &GlossaryEntry,
    normalized_query: Memo<String>,
    level_filter: Memo<Option<String>>,
) -> impl IntoView + use<> {
    let element_id: String = glossary_row_element_id(entry_index);
    let clip_key: Option<String> = entry.clip_key.clone();
    let target: String = entry.target.clone();
    let source: String = entry.source.clone();
    let source_literal: String = entry.source_literal.clone();
    let unit_href: String = entry.unit_href.clone();
    let unit_label: String = entry.unit_label.clone();
    let matches: bool =
        normalized_query.with_untracked(|query| level_filter.with_untracked(|level| entry.matches(query, level.as_deref())));

    view! {
        <tr id=element_id class=item_index::ITEM_INDEX_TARGET_CLASS tabindex="-1" hidden=!matches>
            <td class="glossary-play"><PlayButton clip_key text=target.clone() /></td>
            <td class="pt glossary-target">{target}</td>
            <td class="glossary-source">{source}</td>
            <td class="literal glossary-literal">{source_literal}</td>
            <td class="numeric glossary-unit"><A href=unit_href>{unit_label}</A></td>
        </tr>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glossary_entry(level: &str, unit_order: u32, target: &str) -> GlossaryEntry {
        GlossaryEntry {
            clip_key: None,
            target: target.to_string(),
            source: String::new(),
            source_literal: String::new(),
            level: level.to_string(),
            unit_id: format!("{level}-{unit_order}"),
            unit_order,
            unit_href: String::new(),
            unit_label: format!("{level} {unit_order}"),
            normalized_target: normalize_answer_ignoring_accents(target),
            normalized_source: String::new(),
        }
    }

    fn sample_entries() -> Vec<GlossaryEntry> {
        vec![
            glossary_entry("A1", 1, "casa"),
            glossary_entry("A1", 1, "gato"),
            glossary_entry("A2", 26, "carro"),
            glossary_entry("B1", 40, "gatilho"),
            glossary_entry("A1", 2, "gelo"),
            glossary_entry("B1", 40, "cadeira"),
            glossary_entry("B1", 41, "gatinho"),
        ]
    }

    #[test]
    fn first_matching_entry_index_per_rail_entry_takes_each_level_once_in_entry_order() {
        assert_eq!(first_matching_entry_index_per_rail_entry(&sample_entries(), GlossaryRailKind::Levels, "", None), vec![0, 2, 3]);
    }

    #[test]
    fn first_matching_entry_index_per_rail_entry_skips_entries_the_search_excludes() {
        assert_eq!(first_matching_entry_index_per_rail_entry(&sample_entries(), GlossaryRailKind::Levels, "gat", None), vec![1, 3]);
        assert_eq!(first_matching_entry_index_per_rail_entry(&sample_entries(), GlossaryRailKind::Levels, "zzz", None), Vec::<usize>::new());
    }

    #[test]
    fn first_matching_entry_index_per_rail_entry_takes_each_unit_of_the_filtered_level_once() {
        assert_eq!(first_matching_entry_index_per_rail_entry(&sample_entries(), GlossaryRailKind::Units, "", Some("B1")), vec![3, 6]);
        assert_eq!(first_matching_entry_index_per_rail_entry(&sample_entries(), GlossaryRailKind::Units, "", Some("A1")), vec![0, 4]);
    }

    #[test]
    fn first_matching_entry_index_per_rail_entry_drops_units_the_search_empties() {
        assert_eq!(first_matching_entry_index_per_rail_entry(&sample_entries(), GlossaryRailKind::Units, "ca", Some("B1")), vec![5]);
    }

    #[test]
    fn glossary_rail_kind_lists_units_only_for_a_single_level() {
        assert_eq!(GlossaryRailKind::for_level_filter(None), GlossaryRailKind::Levels);
        assert_eq!(GlossaryRailKind::for_level_filter(Some("A2")), GlossaryRailKind::Units);
    }

    #[test]
    fn rail_entry_shows_the_unit_number_and_names_the_whole_unit() {
        let entry: GlossaryEntry = glossary_entry("A2", 26, "carro");

        assert_eq!(
            rail_entry(2, &entry, GlossaryRailKind::Units),
            ItemIndexEntry {
                target_element_id: glossary_row_element_id(2),
                visible_text: "26".to_string(),
                accessible_label: "Go to unit A2 26".to_string(),
            },
        );
        assert_eq!(
            rail_entry(2, &entry, GlossaryRailKind::Levels),
            ItemIndexEntry {
                target_element_id: glossary_row_element_id(2),
                visible_text: "A2".to_string(),
                accessible_label: "Go to level A2".to_string(),
            },
        );
    }
}
