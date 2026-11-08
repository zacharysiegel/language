use leptos::prelude::*;
use shared::grading::{self, GraderFinding, GraderResponse};

use crate::grading::{self as web_grading, GraderStateKind, GradingStore, ModelDownloadProgress, use_grading_store};
use crate::progress_state::{ProgressStore, use_progress_store};

const SELF_CHECK_NOTICE: &str = "Automatic grading by an AI model is available on computers that can run it.";
const DOWNLOAD_TURNED_OFF_NOTICE: &str = "Automatic grading by an AI model needs the model's download, which is turned off in the settings.";
const AI_MODEL_TAG: &str = "AI model";

fn download_progress_text(progress: &ModelDownloadProgress) -> String {
    format!("Downloading the AI model for automatic grading: {}", progress.description())
}

fn yes_or_no(value: bool) -> &'static str {
    if value { "Yes" } else { "No" }
}

pub fn mistake_count_heading(findings: &[GraderFinding]) -> String {
    match findings.len() {
        0 => "No mistakes found".to_string(),
        1 => "1 mistake found".to_string(),
        count => format!("{count} mistakes found"),
    }
}

#[component]
pub fn GradingModelDownloadSetting() -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let download_enabled = move || progress_store.read(|progress| progress.settings.download_grading_model);
    let label: String = format!("AI grading model ({})", web_grading::format_gigabytes(grading::GRADING_MODEL_SIZE_BYTES));

    view! {
        <label class="field" title="Downloads the model that grades open answers, on computers that can run it">
            <input
                type="checkbox"
                prop:checked=download_enabled
                on:change=move |_| progress_store.update(|progress| progress.settings.download_grading_model = !progress.settings.download_grading_model)
            />
            <span class="field-label">{label}</span>
        </label>
    }
}

/// The grading model's download progress, shown only while it downloads.
#[component]
pub fn ModelDownloadStatus() -> impl IntoView {
    let grading_store: GradingStore = use_grading_store();

    move || {
        let GraderStateKind::Downloading(progress) = grading_store.state()
        else {
            return None;
        };

        Some(view! {
            <div class="model-download">
                <p class="muted">{download_progress_text(&progress)}</p>
                <progress max=1000 value=progress.downloaded_permille()></progress>
            </div>
        })
    }
}

/// Why an answer was not graded, and what grading needs.
#[component]
pub fn SelfCheckNotice() -> impl IntoView {
    let grading_store: GradingStore = use_grading_store();
    let progress_store: ProgressStore = use_progress_store();

    move || {
        let download_enabled: bool = progress_store.read(|progress| progress.settings.download_grading_model);

        match grading_store.state() {
            GraderStateKind::Downloading(_) => view! { <ModelDownloadStatus /> }.into_any(),
            GraderStateKind::Unchecked | GraderStateKind::NotDownloaded if !download_enabled => {
                view! { <p class="muted">{DOWNLOAD_TURNED_OFF_NOTICE}</p> }.into_any()
            },
            _ => view! { <p class="muted">{SELF_CHECK_NOTICE}</p> }.into_any(),
        }
    }
}

#[component]
pub fn GradedFeedbackHeading(heading: String) -> impl IntoView {
    view! {
        <div class="feedback-heading">
            <strong>{heading}</strong>
            <span class="tag">{AI_MODEL_TAG}</span>
        </div>
    }
}

#[component]
pub fn GraderFindings(response: GraderResponse, target_language_name: String) -> impl IntoView {
    let finding_items: Vec<_> = response
        .findings
        .into_iter()
        .map(|finding| {
            view! {
                <li>
                    <div class="finding-change">
                        <span class="finding-quote">{finding.quote}</span>
                        <span class="finding-arrow" aria-label="becomes">"→"</span>
                        <span class="finding-replacement">{finding.correction}</span>
                    </div>
                    <p class="explanation">{finding.explanation}</p>
                </li>
            }
        })
        .collect();

    view! {
        <dl class="grader-checks">
            <dt>"Addresses the task"</dt>
            <dd>{yes_or_no(response.answers_the_question)}</dd>
            <dt>{format!("Written in {target_language_name}")}</dt>
            <dd>{yes_or_no(response.written_in_portuguese)}</dd>
        </dl>
        <ul class="grader-findings">{finding_items}</ul>
    }
}
