use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::course::{CourseLevel, CourseOutline, UnitOutline};
use shared::progress::UnitProgress;

use crate::course_data::{self, course_gate, use_route_course_id};
use crate::progress_state::{self, ProgressStore, remember_route_course, use_progress_store};

#[component]
pub fn HomePage() -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let course_id: Memo<String> = use_route_course_id();

    remember_route_course(progress_store, course_id);

    course_gate(move || course_id.get(), move |course: Arc<CourseOutline>| {
        let next_unit_id: Option<String> = next_incomplete_unit_id(&course, progress_store);
        let level_sections: Vec<_> = course
            .levels
            .iter()
            .map(|level| level_section(&course, level, progress_store))
            .collect();
        let completed_course: Arc<CourseOutline> = course.clone();

        view! {
            <div class="course-title">
                <h1>{course.title.clone()}</h1>
                <span class="page-caption">{format!("{} units", course.units.len())}</span>
            </div>
            <div class="course-summary">
                {move || {
                    let completed_count: usize = progress_store.read(|progress| progress.course_progress(&completed_course.id).completed_unit_count());

                    view! { <span class="numeric muted">{completed_count}" / "{completed_course.units.len()}" units completed"</span> }
                }}
                {next_unit_id.map(|unit_id| view! { <A href=course_data::unit_path(&course.id, &unit_id) attr:class="button primary">"Continue"</A> })}
            </div>
            {level_sections}
        }
    })
}

fn next_incomplete_unit_id(course: &CourseOutline, progress_store: ProgressStore) -> Option<String> {
    progress_store.read(|progress| {
        let course_progress = progress.course_progress(&course.id);

        course
            .units
            .iter()
            .find(|unit| !course_progress.unit_progress(&unit.id).completed)
            .map(|unit| unit.id.clone())
    })
}

fn level_section(course: &CourseOutline, level: &CourseLevel, progress_store: ProgressStore) -> impl IntoView + use<> {
    let unit_items: Vec<_> = course
        .units_at_level(&level.code)
        .map(|unit| unit_item(&course.id, unit, progress_store))
        .collect();

    view! {
        <section class="level-section">
            <h2>{level.code.clone()}" · "{level.title.clone()}</h2>
            <ul class="units">{unit_items}</ul>
        </section>
    }
}

fn unit_item(course_id: &str, unit: &UnitOutline, progress_store: ProgressStore) -> impl IntoView + use<> {
    let course_id: String = course_id.to_string();
    let unit_id: String = unit.id.clone();
    let href: String = course_data::unit_path(&course_id, &unit_id);
    let unit_order: u32 = unit.order;
    let unit_title: String = unit.title.clone();
    let status = move || {
        let unit_progress: UnitProgress = progress_store.read(|progress| progress.course_progress(&course_id).unit_progress(&unit_id));

        progress_state::unit_status_view(unit_progress)
    };

    view! {
        <li>
            <A href=href>
                <span class="unit-number">{unit_order}</span>
                <span class="unit-title">{unit_title}</span>
                <span class="unit-status">{status}</span>
            </A>
        </li>
    }
}
