use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::course::{Course, LevelKind, Unit, VocabularyItem};
use shared::practice::normalize_answer_ignoring_accents;

use crate::audio_player::PlayButton;
use crate::course_data::course_gate;

const ALL_LEVELS_CODE: &str = "all";

#[component]
pub fn GlossaryPage() -> impl IntoView {
    course_gate(|course: Arc<Course>| view! { <Glossary course /> })
}

fn level_from_code(code: &str) -> Option<LevelKind> {
    LevelKind::try_from(code).ok()
}

#[component]
fn Glossary(course: Arc<Course>) -> impl IntoView {
    let search_text: RwSignal<String> = RwSignal::new(String::new());
    let level_code: RwSignal<String> = RwSignal::new(ALL_LEVELS_CODE.to_string());
    let level_options: Vec<_> = course
        .levels
        .iter()
        .map(|level| view! { <option value=level.code.code()>{level.code.code()}</option> })
        .collect();

    let matches = {
        let course: Arc<Course> = course.clone();

        move || {
            let query: String = normalize_answer_ignoring_accents(&search_text.get());
            let level_filter: Option<LevelKind> = level_from_code(&level_code.get());
            let matching_entries: Vec<(&Unit, &VocabularyItem)> = course
                .units
                .iter()
                .filter(|unit| level_filter.is_none_or(|level| unit.level == level))
                .flat_map(|unit| unit.vocabulary.iter().map(move |item| (unit, item)))
                .filter(|(_, item)| {
                    query.is_empty()
                        || normalize_answer_ignoring_accents(&item.pt).contains(&query)
                        || normalize_answer_ignoring_accents(&item.en).contains(&query)
                })
                .collect();

            let total_matches: usize = matching_entries.len();

            let rows: Vec<_> = matching_entries
                .into_iter()
                .map(|(unit, item)| {
                    let unit_href: String = format!("/unit/{}?tab=vocabulary", unit.id);
                    let unit_label: String = format!("{} {}", unit.level.code(), unit.order);

                    view! {
                        <tr>
                            <td><PlayButton audio=item.audio.clone() text=item.pt.clone() /></td>
                            <td class="pt">{item.pt.clone()}</td>
                            <td>{item.en.clone()}</td>
                            <td><A href=unit_href>{unit_label}</A></td>
                        </tr>
                    }
                })
                .collect();

            view! {
                <p class="muted">{format!("{total_matches} match(es)")}</p>
                <table class="grid">
                    <thead><tr><th></th><th>"Portuguese"</th><th>"English"</th><th>"Unit"</th></tr></thead>
                    <tbody>{rows}</tbody>
                </table>
            }
        }
    };

    view! {
        <h1>"Glossary"</h1>
        <p class="row">
            <input
                type="search"
                placeholder="Search Portuguese or English"
                prop:value=move || search_text.get()
                on:input:target=move |event| search_text.set(event.target().value())
            />
            <select on:change:target=move |event| level_code.set(event.target().value())>
                <option value=ALL_LEVELS_CODE>"All levels"</option>
                {level_options}
            </select>
        </p>
        {matches}
    }
}
