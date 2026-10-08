use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::AppError;
use shared::course::CourseOutline;
use shared::progress::Progress;
use shared::review::{REVIEW_INTERVAL_DAYS, count_by_box};

use crate::course_data::{self, course_gate, use_route_course_id};
use crate::platform;
use crate::progress_state::{self, ProgressStore, remember_route_course, use_progress_store};

#[component]
pub fn ProgressPage() -> impl IntoView {
    let course_id: Memo<String> = use_route_course_id();

    remember_route_course(use_progress_store(), course_id);

    course_gate(move || course_id.get(), |course: Arc<CourseOutline>| view! { <ProgressSummary course /> })
}

fn box_label(box_index: usize) -> String {
    match REVIEW_INTERVAL_DAYS[box_index] {
        0 => "New or forgotten".to_string(),
        days => format!("Next review after {days} day(s)"),
    }
}

#[component]
fn ProgressSummary(course: Arc<CourseOutline>) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let unit_rows: Vec<_> = course
        .units
        .iter()
        .map(|unit| {
            let unit_id: String = unit.id.clone();
            let course_id: String = course.id.clone();
            let unit_href: String = course_data::unit_path(&course_id, &unit_id);
            let unit_label: String = format!("{} {}: {}", unit.level, unit.order, unit.title);
            let status = move || {
                let unit_progress = progress_store.read(|progress| progress.course_progress(&course_id).unit_progress(&unit_id));

                progress_state::unit_status_view(unit_progress)
            };

            view! {
                <tr>
                    <td><A href=unit_href>{unit_label}</A></td>
                    <td>{status}</td>
                </tr>
            }
        })
        .collect();

    let box_course_id: String = course.id.clone();
    let box_rows = move || {
        let counts = progress_store.read(|progress| count_by_box(progress.course_progress(&box_course_id).cards.values()));

        counts
            .into_iter()
            .enumerate()
            .map(|(box_index, count)| view! { <tr><td>{box_label(box_index)}</td><td class="numeric">{count}</td></tr> })
            .collect_view()
    };

    let unit_count: usize = course.units.len();
    let streak_days = move || progress_store.read(|progress| progress.streak_days(platform::today()));
    let completed_course_id: String = course.id.clone();
    let completed_count = move || progress_store.read(|progress| progress.course_progress(&completed_course_id).completed_unit_count());

    view! {
        <h1>"Progress"</h1>
        <div class="course-summary">
            <span class="numeric muted">{move || format!("{} / {unit_count} units completed", completed_count())}</span>
            <span class="numeric muted">{move || format!("Study streak {} day(s)", streak_days())}</span>
        </div>
        <h2>"Units"</h2>
        <table class="grid"><tbody>{unit_rows}</tbody></table>
        <h2>"Review deck"</h2>
        <table class="grid"><tbody>{box_rows}</tbody></table>
        <DataTransfer />
    }
}

#[component]
fn DataTransfer() -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let transfer_text: RwSignal<String> = RwSignal::new(String::new());
    let message: RwSignal<Option<String>> = RwSignal::new(None);
    let reset_requested: RwSignal<bool> = RwSignal::new(false);

    let export_progress = move |_| {
        let serialized: Result<String, AppError> = progress_store.read(Progress::to_json);

        match serialized {
            Ok(text) => transfer_text.set(text),
            Err(error) => message.set(Some(format!("Could not export: {}", error.message))),
        }
    };

    let import_progress = move |_| {
        let parsed: Result<Progress, AppError> = Progress::from_json(&transfer_text.get_untracked());

        match parsed {
            Ok(imported) => {
                progress_store.replace(imported);
                message.set(Some("Progress imported.".to_string()));
            },
            Err(error) => message.set(Some(format!("Could not import: {}", error.message))),
        }
    };

    let reset_progress = move |_| {
        if !reset_requested.get_untracked() {
            reset_requested.set(true);
            return;
        }

        progress_store.replace(Progress::default());
        reset_requested.set(false);
        message.set(Some("All progress was reset.".to_string()));
    };

    view! {
        <h2>"Back up or move your progress"</h2>
        <p class="muted">"Progress is saved in this browser only. Copy the text below to keep a backup, or paste a backup to restore it."</p>
        <div class="row">
            <button type="button" on:click=export_progress>"Export"</button>
            <button type="button" on:click=import_progress>"Import from the text box"</button>
        </div>
        <textarea
            rows="8"
            placeholder="Exported progress appears here. Paste a backup here to import it."
            prop:value=move || transfer_text.get()
            on:input:target=move |event| transfer_text.set(event.target().value())
        ></textarea>
        <p class="muted">{move || message.get()}</p>
        <h2>"Reset"</h2>
        <button type="button" on:click=reset_progress>
            {move || if reset_requested.get() { "Click again to erase all progress" } else { "Reset all progress" }}
        </button>
    }
}
