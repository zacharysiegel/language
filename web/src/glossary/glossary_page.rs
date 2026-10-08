use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::course::{self, CourseOutline, UnitOutline, VocabularyItem};
use shared::practice::normalize_answer_ignoring_accents;

use crate::audio_player::PlayButton;
use crate::course_data::{self, course_gate, use_route_course_id};
use crate::progress_state::{remember_route_course, use_progress_store};

const ALL_LEVELS_CODE: &str = "all";

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

    let matches = {
        let course: Arc<CourseOutline> = course.clone();
        let target_name: String = target_name.clone();
        let source_name: String = source_name.clone();

        move || {
            let query: String = normalize_answer_ignoring_accents(&search_text.get());
            let selected_level_code: String = level_code.get();
            let level_filter: Option<&str> = (selected_level_code != ALL_LEVELS_CODE).then_some(selected_level_code.as_str());
            let matching_entries: Vec<(&UnitOutline, &VocabularyItem)> = course
                .units
                .iter()
                .filter(|unit| level_filter.is_none_or(|level| unit.level == level))
                .flat_map(|unit| unit.vocabulary.iter().map(move |item| (unit, item)))
                .filter(|(_, item)| {
                    query.is_empty()
                        || normalize_answer_ignoring_accents(&item.target).contains(&query)
                        || normalize_answer_ignoring_accents(&item.source).contains(&query)
                })
                .collect();

            let total_matches: usize = matching_entries.len();

            let rows: Vec<_> = matching_entries
                .into_iter()
                .map(|(unit, item)| {
                    let unit_href: String = course_data::unit_tab_path(&course.id, &unit.id, "vocabulary");
                    let unit_label: String = format!("{} {}", unit.level, unit.order);

                    view! {
                        <tr>
                            <td><PlayButton clip_key=item.clip_key.clone() text=item.target.clone() /></td>
                            <td class="pt">{item.target.clone()}</td>
                            <td>{item.source.clone()}</td>
                            <td class="literal">{item.source_literal.clone().unwrap_or_default()}</td>
                            <td class="numeric"><A href=unit_href>{unit_label}</A></td>
                        </tr>
                    }
                })
                .collect();

            view! {
                <span class="page-caption">{format!("{total_matches} entries")}</span>
                <table class="grid">
                    <thead><tr><th></th><th>{target_name.clone()}</th><th>{source_name.clone()}</th><th>"Literally"</th><th>"Unit"</th></tr></thead>
                    <tbody>{rows}</tbody>
                </table>
            }
        }
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
        {matches}
    }
}
