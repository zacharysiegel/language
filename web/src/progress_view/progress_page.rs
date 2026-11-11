use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::AppError;
use shared::course::{CourseOutline, UnitOutline};
use shared::progress::{CourseProgress, Progress, QuizResult};
use shared::interface_text;
use shared::review::{self, REVIEW_INTERVAL_DAYS, count_by_box};

use crate::course_data::{self, course_gate, use_route_course_id};
use crate::progress_state::{ProgressStore, remember_route_course, use_progress_store};

#[component]
pub fn ProgressPage() -> impl IntoView {
    let course_id: Memo<String> = use_route_course_id();

    remember_route_course(use_progress_store(), course_id);

    course_gate(move || course_id.get(), |course: Arc<CourseOutline>| view! { <ProgressSummary course /> })
}

#[component]
fn ProgressSummary(course: Arc<CourseOutline>) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let level_rows = {
        let course: Arc<CourseOutline> = course.clone();

        move || progress_store.read(|progress| level_completion_rows(&course, &progress.course_progress(&course.id)))
    };
    let quiz_results = {
        let course: Arc<CourseOutline> = course.clone();

        move || progress_store.read(|progress| quiz_results_view(&course, &progress.course_progress(&course.id)))
    };

    let box_course_id: String = course.id.clone();
    let box_rows = move || {
        let counts = progress_store.read(|progress| count_by_box(progress.course_progress(&box_course_id).cards.values()));

        counts
            .into_iter()
            .enumerate()
            .map(|(box_index, count)| view! { <tr><td>{review::review_box_label(REVIEW_INTERVAL_DAYS[box_index])}</td><td class="numeric">{count}</td></tr> })
            .collect_view()
    };

    view! {
        <h1>"Progress"</h1>
        <h2>"Levels"</h2>
        <table class="grid"><tbody>{level_rows}</tbody></table>
        <h2>"Quiz results"</h2>
        {quiz_results}
        <h2>"Review deck"</h2>
        <table class="grid"><tbody>{box_rows}</tbody></table>
        <DataTransfer />
    }
}

fn level_completion_rows(course: &CourseOutline, course_progress: &CourseProgress) -> impl IntoView + use<> {
    course
        .levels
        .iter()
        .map(|level| {
            let level_unit_ids: Vec<&str> = course.units_at_level(&level.code).map(|unit| unit.id.as_str()).collect();
            let completed_unit_count: usize = course_progress.completed_unit_count_among(level_unit_ids.iter().copied());

            view! {
                <tr>
                    <td>{level.code.clone()}" · "{level.title.clone()}</td>
                    <td class="muted tabular-figures">{format!("{completed_unit_count} of {} units completed", level_unit_ids.len())}</td>
                </tr>
            }
        })
        .collect_view()
}

fn quiz_results_view(course: &CourseOutline, course_progress: &CourseProgress) -> AnyView {
    let quiz_results: Vec<QuizResult> = course_progress.quiz_results(course.units.iter().map(|unit| unit.id.as_str()));

    if quiz_results.is_empty() {
        return view! { <p class="muted">"You have not taken a quiz yet."</p> }.into_any();
    }

    let quiz_result_rows: Vec<_> = quiz_results
        .into_iter()
        .filter_map(|quiz_result| quiz_result_row(course, quiz_result))
        .collect();

    view! { <table class="grid"><tbody>{quiz_result_rows}</tbody></table> }.into_any()
}

fn quiz_result_row(course: &CourseOutline, quiz_result: QuizResult) -> Option<impl IntoView + use<>> {
    let unit: &UnitOutline = course.find_unit(&quiz_result.unit_id)?;
    let unit_href: String = course_data::unit_path(&course.id, &unit.id);
    let unit_title: String = unit.title.clone();
    let attempts_text: String = interface_text::counted_noun_phrase(quiz_result.attempt_count as usize, "attempt", "attempts");
    let score_text: String = format!("Best score: {}% over {attempts_text}", quiz_result.best_score_percent);

    Some(view! {
        <tr>
            <td><A href=unit_href>{unit_title}</A></td>
            <td class="muted tabular-figures">{score_text}</td>
        </tr>
    })
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
