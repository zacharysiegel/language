use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::course::{Course, LevelSummary, Unit};
use shared::progress::UnitProgress;

use crate::course_data::course_gate;
use crate::progress_state::{ProgressStore, use_progress_store};

#[component]
pub fn HomePage() -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();

    course_gate(move |course: Arc<Course>| {
        let next_unit_id: Option<String> = next_incomplete_unit_id(&course, progress_store);
        let level_sections: Vec<_> = course
            .levels
            .iter()
            .map(|level| level_section(&course, level, progress_store))
            .collect();

        view! {
            <h1>{course.title.clone()}</h1>
            {move || {
                let completed_count: usize = progress_store.read(|progress| progress.completed_unit_count());
                view! { <p class="summary">{completed_count}" of "{course.units.len()}" units completed."</p> }
            }}
            {next_unit_id.map(|unit_id| view! { <p><A href=format!("/unit/{unit_id}") attr:class="button primary">"Continue"</A></p> })}
            {level_sections}
        }
    })
}

fn next_incomplete_unit_id(course: &Course, progress_store: ProgressStore) -> Option<String> {
    progress_store.read(|progress| {
        course
            .units
            .iter()
            .find(|unit| !progress.unit_progress(&unit.id).completed)
            .map(|unit| unit.id.clone())
    })
}

fn level_section(course: &Course, level: &LevelSummary, progress_store: ProgressStore) -> impl IntoView + use<> {
    let unit_items: Vec<_> = course
        .units_at_level(level.code)
        .map(|unit| unit_item(unit, progress_store))
        .collect();

    view! {
        <section class="level">
            <h2>{level.code.code()}" "<span class="muted">{level.title.clone()}</span></h2>
            <ul class="units">{unit_items}</ul>
        </section>
    }
}

fn unit_item(unit: &Unit, progress_store: ProgressStore) -> impl IntoView + use<> {
    let unit_id: String = unit.id.clone();
    let href: String = format!("/unit/{unit_id}");
    let unit_order: u32 = unit.order;
    let unit_title: String = unit.title.clone();
    let status = move || {
        let unit_progress: UnitProgress = progress_store.read(|progress| progress.unit_progress(&unit_id));

        if unit_progress.completed {
            format!("Completed, best {}%", (unit_progress.best_proportion * 100.0).round())
        } else if unit_progress.attempt_count > 0 {
            format!("In progress, best {}%", (unit_progress.best_proportion * 100.0).round())
        } else {
            "Not started".to_string()
        }
    };

    view! {
        <li>
            <A href=href>
                <span class="unit-number">{unit_order}</span>
                <span class="unit-title">{unit_title}</span>
            </A>
            <span class="muted">{status}</span>
        </li>
    }
}
