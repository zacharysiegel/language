use leptos::prelude::*;
use shared::grading::{self, GraderFinding, GraderResponse};

use crate::grading::{
    self as web_grading, GraderStateKind, GradingProgress, GradingStepKind, GradingStore, ModelDownloadProgress, use_grading_store,
};
use crate::pending_indication;
use crate::progress_state::{ProgressStore, use_progress_store};

const SELF_CHECK_NOTICE: &str = "Automatic grading is not available on this device. In supported web browsers and native desktop applications, an AI model checks your answer and points out each mistake.";
const NOT_GRADED_NOTICE: &str = "Automatic grading failed for this answer.";
const IN_USE_IN_ANOTHER_PAGE_NOTICE: &str = "Automatic grading is in use in another tab or window, so this answer was not graded.";
const DOWNLOAD_SETTING_LABEL: &str = "Download AI grading model";
const DOWNLOAD_PROGRESS_LABEL: &str = "AI grading model download";
const AI_MODEL_TAG: &str = "AI model";
const GRADING_PROGRESS_LABEL: &str = "Automatic grading progress";

fn download_progress_text(progress: &ModelDownloadProgress) -> String {
    format!("Downloading the AI model for automatic grading: {}", progress.description())
}

fn download_turned_off_notice() -> String {
    format!("Automatic grading is off. To use it, turn on \"{DOWNLOAD_SETTING_LABEL}\" in the settings.")
}

fn grading_step_label(step: GradingStepKind) -> &'static str {
    match step {
        GradingStepKind::LoadingModel => "Loading the AI model",
        GradingStepKind::ReadingAnswer => "Reading your answer",
        GradingStepKind::WritingFeedback => "Writing feedback",
    }
}

fn grading_status_text(step: GradingStepKind) -> String {
    format!("Step {} of {}: {}", step.number(), GradingStepKind::ALL.len(), grading_step_label(step))
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
    let label: String = format!("{DOWNLOAD_SETTING_LABEL} ({})", web_grading::format_gigabytes(grading::GRADING_MODEL_SIZE_BYTES));

    view! {
        <label class="field">
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
                <progress max=1000 value=progress.downloaded_permille() aria-label=DOWNLOAD_PROGRESS_LABEL></progress>
            </div>
        })
    }
}

/// The grading of an answer under way, shown once the pending indication delay has passed.
#[component]
pub fn GradingProgressStatus() -> impl IntoView {
    let grading_store: GradingStore = use_grading_store();
    let delay_passed: RwSignal<bool> = RwSignal::new(false);
    let progress: Memo<GradingProgress> =
        Memo::new(move |_| web_grading::grading_progress(grading_store.state(), grading_store.generation_progress()));
    let step: Memo<GradingStepKind> = Memo::new(move |_| progress.get().step);
    let shown_step = move || delay_passed.get().then(|| step.get());

    Effect::new(move |_| {
        set_timeout(move || delay_passed.set(true), pending_indication::pending_indication_delay());
    });

    // The status line is in the page from the start, so a screen reader announces each step written into it.
    view! {
        <div class="grading-progress">
            <p class="muted" role="status" aria-live="polite">{move || shown_step().map(grading_status_text)}</p>
            {move || {
                delay_passed.get().then(|| {
                    view! {
                        <progress
                            max=web_grading::GRADING_PROGRESS_MAXIMUM_PERMILLE
                            value=move || progress.get().overall_permille()
                            aria-label=GRADING_PROGRESS_LABEL
                        ></progress>
                    }
                })
            }}
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfCheckNoticeContent {
    DownloadProgress,
    Text(String),
}

/// Only the unsupported state claims that the browser cannot grade.
pub fn self_check_notice_content(grader_state: GraderStateKind, download_enabled: bool) -> SelfCheckNoticeContent {
    match grader_state {
        GraderStateKind::Downloading(_) => SelfCheckNoticeContent::DownloadProgress,
        GraderStateKind::Unchecked | GraderStateKind::NotDownloaded if !download_enabled => SelfCheckNoticeContent::Text(download_turned_off_notice()),
        GraderStateKind::Unsupported => SelfCheckNoticeContent::Text(SELF_CHECK_NOTICE.to_string()),
        GraderStateKind::InUseInAnotherPage => SelfCheckNoticeContent::Text(IN_USE_IN_ANOTHER_PAGE_NOTICE.to_string()),
        GraderStateKind::Unchecked
        | GraderStateKind::NotDownloaded
        | GraderStateKind::Downloaded
        | GraderStateKind::Loading
        | GraderStateKind::Ready
        | GraderStateKind::Failed => SelfCheckNoticeContent::Text(NOT_GRADED_NOTICE.to_string()),
    }
}

/// Why an answer was not graded, and what grading needs.
#[component]
pub fn SelfCheckNotice() -> impl IntoView {
    let grading_store: GradingStore = use_grading_store();
    let progress_store: ProgressStore = use_progress_store();

    let notice = move || {
        let download_enabled: bool = progress_store.read(|progress| progress.settings.download_grading_model);
        let notice_content: SelfCheckNoticeContent = self_check_notice_content(grading_store.state(), download_enabled);

        match notice_content {
            SelfCheckNoticeContent::DownloadProgress => view! { <ModelDownloadStatus /> }.into_any(),
            SelfCheckNoticeContent::Text(notice_text) => view! { <p>{notice_text}</p> }.into_any(),
        }
    };

    view! { <div class="self-check-notice">{notice}</div> }
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
                        <span class="finding-arrow" aria-hidden="true">"→"</span>
                        <span class="visually-hidden">"becomes"</span>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_check_notice_content_claims_an_unsupported_browser_only_for_the_unsupported_state() {
        let failed_states: [GraderStateKind; 6] = [
            GraderStateKind::Unchecked,
            GraderStateKind::NotDownloaded,
            GraderStateKind::Downloaded,
            GraderStateKind::Loading,
            GraderStateKind::Ready,
            GraderStateKind::Failed,
        ];

        for failed_state in failed_states {
            assert_eq!(self_check_notice_content(failed_state, true), SelfCheckNoticeContent::Text(NOT_GRADED_NOTICE.to_string()));
        }

        assert_eq!(self_check_notice_content(GraderStateKind::Unsupported, true), SelfCheckNoticeContent::Text(SELF_CHECK_NOTICE.to_string()));
    }
}
