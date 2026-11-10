use shared::grading::{self, GraderFinding, GraderResponse};

const BYTES_PER_GIGABYTE: f64 = 1_000_000_000.0;
const PERMILLE: u64 = 1000;
// Near the upper end of the reply lengths the grading evaluation measured for the grader.
const TYPICAL_GRADER_REPLY_TOKEN_COUNT: u64 = 80;
pub const GRADING_PROGRESS_MAXIMUM_PERMILLE: u64 = GradingStepKind::ALL.len() as u64 * PERMILLE;

/// `part` as a proportion of `whole` in permille, at most the whole.
fn permille_of(part: u64, whole: u64) -> u64 {
    let whole: u64 = whole.max(1);

    part.min(whole) * PERMILLE / whole
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelDownloadProgress {
    pub downloaded_byte_count: u64,
    /// Absent when the repository does not send the model's length.
    pub total_byte_count: Option<u64>,
}

impl ModelDownloadProgress {
    pub const NOT_STARTED: ModelDownloadProgress = ModelDownloadProgress {
        downloaded_byte_count: 0,
        total_byte_count: None,
    };

    pub fn total_or_expected_byte_count(&self) -> u64 {
        self.total_byte_count.unwrap_or(grading::GRADING_MODEL_SIZE_BYTES).max(1)
    }

    pub fn downloaded_permille(&self) -> u64 {
        permille_of(self.downloaded_byte_count, self.total_or_expected_byte_count())
    }

    pub fn description(&self) -> String {
        let downloaded_gigabytes: String = format_gigabyte_figure(self.downloaded_byte_count);
        let total_gigabytes: String = format_gigabytes(self.total_or_expected_byte_count());

        format!("{downloaded_gigabytes} of {total_gigabytes}")
    }
}

/// Where this browser stands with the grading model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraderStateKind {
    /// Before the browser's capability and its downloaded model are checked.
    Unchecked,
    Unsupported,
    NotDownloaded,
    Downloading(ModelDownloadProgress),
    Downloaded,
    /// Another page of this site holds a grading model in memory, or is loading one.
    InUseInAnotherPage,
    Loading,
    Ready,
    /// The model failed to load in this browser.
    Failed,
}

/// The grader's latest report on the generation it runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraderGenerationProgressKind {
    ReadingPrompt {
        /// Includes the tokens reused from the cache.
        read_token_count: u64,
        prompt_token_count: u64,
    },
    WritingReply {
        generated_token_count: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradingStepKind {
    LoadingModel,
    ReadingAnswer,
    WritingFeedback,
}

impl GradingStepKind {
    pub const ALL: [GradingStepKind; 3] = [
        GradingStepKind::LoadingModel,
        GradingStepKind::ReadingAnswer,
        GradingStepKind::WritingFeedback,
    ];

    /// Counted from 1.
    pub fn number(self) -> u64 {
        match self {
            GradingStepKind::LoadingModel => 1,
            GradingStepKind::ReadingAnswer => 2,
            GradingStepKind::WritingFeedback => 3,
        }
    }
}

/// How far the grading of an answer has come.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GradingProgress {
    pub step: GradingStepKind,
    /// The current step's progress; the steps before it are complete.
    pub step_permille: u64,
}

impl GradingProgress {
    /// Each step is an equal share of [`GRADING_PROGRESS_MAXIMUM_PERMILLE`].
    pub fn overall_permille(&self) -> u64 {
        (self.step.number() - 1) * PERMILLE + self.step_permille
    }
}

pub fn grading_progress(grader_state: GraderStateKind, generation_progress: Option<GraderGenerationProgressKind>) -> GradingProgress {
    if grader_state != GraderStateKind::Ready {
        // wllama reports no progress while loading, so the step has no fill.
        return GradingProgress {
            step: GradingStepKind::LoadingModel,
            step_permille: 0,
        };
    }

    match generation_progress {
        Some(GraderGenerationProgressKind::ReadingPrompt {
            read_token_count,
            prompt_token_count,
        }) => GradingProgress {
            step: GradingStepKind::ReadingAnswer,
            step_permille: permille_of(read_token_count, prompt_token_count),
        },
        Some(GraderGenerationProgressKind::WritingReply { generated_token_count }) => GradingProgress {
            step: GradingStepKind::WritingFeedback,
            step_permille: permille_of(generated_token_count, TYPICAL_GRADER_REPLY_TOKEN_COUNT),
        },
        None => GradingProgress {
            step: GradingStepKind::ReadingAnswer,
            step_permille: 0,
        },
    }
}

/// What the browser reports about itself and its storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrowserCapabilityFacts {
    /// A WebGPU adapter that is not a fallback adapter, which runs on the CPU.
    pub has_webgpu_adapter: bool,
    /// wllama's compatibility build cannot grade with the model.
    pub uses_wllama_default_build: bool,
    /// Absent when the browser does not estimate its storage.
    pub available_storage_byte_count: Option<u64>,
    pub model_downloaded: bool,
    pub earlier_load_failed: bool,
}

pub fn assess_browser_capability(facts: &BrowserCapabilityFacts) -> GraderStateKind {
    if !facts.has_webgpu_adapter || !facts.uses_wllama_default_build || facts.earlier_load_failed {
        return GraderStateKind::Unsupported;
    }

    if facts.model_downloaded {
        return GraderStateKind::Downloaded;
    }

    let has_room_for_the_model: bool = facts
        .available_storage_byte_count
        .is_some_and(|available_byte_count| available_byte_count >= grading::GRADING_MODEL_SIZE_BYTES);

    if has_room_for_the_model { GraderStateKind::NotDownloaded } else { GraderStateKind::Unsupported }
}

/// The grader's response with only the findings that apply to `answer`.
pub fn applicable_response(answer: &str, response: GraderResponse) -> GraderResponse {
    let applicable_findings: Vec<GraderFinding> = grading::retain_applicable_findings(answer, response.findings);

    GraderResponse {
        answers_the_question: response.answers_the_question,
        written_in_portuguese: response.written_in_portuguese,
        findings: applicable_findings,
    }
}

/// Decimal gigabytes with one decimal place, as a download size is usually stated.
pub fn format_gigabytes(byte_count: u64) -> String {
    format!("{} GB", format_gigabyte_figure(byte_count))
}

fn format_gigabyte_figure(byte_count: u64) -> String {
    format!("{:.1}", byte_count as f64 / BYTES_PER_GIGABYTE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capable_facts() -> BrowserCapabilityFacts {
        BrowserCapabilityFacts {
            has_webgpu_adapter: true,
            uses_wllama_default_build: true,
            available_storage_byte_count: Some(grading::GRADING_MODEL_SIZE_BYTES),
            model_downloaded: false,
            earlier_load_failed: false,
        }
    }

    #[test]
    fn assess_browser_capability_needs_an_adapter_the_default_build_room_for_the_model_and_no_failed_load() {
        let mut without_adapter: BrowserCapabilityFacts = capable_facts();
        without_adapter.has_webgpu_adapter = false;
        let mut with_compatibility_build: BrowserCapabilityFacts = capable_facts();
        with_compatibility_build.uses_wllama_default_build = false;
        let mut without_room: BrowserCapabilityFacts = capable_facts();
        without_room.available_storage_byte_count = Some(grading::GRADING_MODEL_SIZE_BYTES - 1);
        let mut without_estimate: BrowserCapabilityFacts = capable_facts();
        without_estimate.available_storage_byte_count = None;
        let mut after_failed_load: BrowserCapabilityFacts = capable_facts();
        after_failed_load.model_downloaded = true;
        after_failed_load.earlier_load_failed = true;

        assert_eq!(assess_browser_capability(&capable_facts()), GraderStateKind::NotDownloaded);
        assert_eq!(assess_browser_capability(&without_adapter), GraderStateKind::Unsupported);
        assert_eq!(assess_browser_capability(&with_compatibility_build), GraderStateKind::Unsupported);
        assert_eq!(assess_browser_capability(&without_room), GraderStateKind::Unsupported);
        assert_eq!(assess_browser_capability(&without_estimate), GraderStateKind::Unsupported);
        assert_eq!(assess_browser_capability(&after_failed_load), GraderStateKind::Unsupported);
    }

    #[test]
    fn assess_browser_capability_needs_no_room_for_a_downloaded_model() {
        let mut downloaded_without_room: BrowserCapabilityFacts = capable_facts();
        downloaded_without_room.available_storage_byte_count = Some(0);
        downloaded_without_room.model_downloaded = true;

        assert_eq!(assess_browser_capability(&downloaded_without_room), GraderStateKind::Downloaded);
    }

    #[test]
    fn description_and_downloaded_permille_measure_against_the_expected_size_until_the_total_is_known() {
        let progress: ModelDownloadProgress = ModelDownloadProgress {
            downloaded_byte_count: 1_260_000_000,
            total_byte_count: None,
        };

        assert_eq!(progress.description(), "1.3 of 7.0 GB");
        assert_eq!(progress.downloaded_permille(), 180);
    }

    #[test]
    fn grading_progress_shows_the_load_as_an_unfilled_first_step_whatever_the_generation_reports() {
        let reading: GraderGenerationProgressKind = GraderGenerationProgressKind::ReadingPrompt {
            read_token_count: 500,
            prompt_token_count: 1000,
        };
        let loading_progress: GradingProgress = grading_progress(GraderStateKind::Loading, Some(reading));

        assert_eq!(loading_progress.step, GradingStepKind::LoadingModel);
        assert_eq!(loading_progress.overall_permille(), 0);
    }

    #[test]
    fn grading_progress_fills_the_second_step_by_the_prompt_tokens_read() {
        let reading: GraderGenerationProgressKind = GraderGenerationProgressKind::ReadingPrompt {
            read_token_count: 250,
            prompt_token_count: 1000,
        };
        let reading_progress: GradingProgress = grading_progress(GraderStateKind::Ready, Some(reading));

        assert_eq!(reading_progress.step, GradingStepKind::ReadingAnswer);
        assert_eq!(reading_progress.step_permille, 250);
        assert_eq!(reading_progress.overall_permille(), 1250);
    }

    #[test]
    fn grading_progress_starts_the_second_step_before_the_generation_first_reports() {
        assert_eq!(
            grading_progress(GraderStateKind::Ready, None),
            GradingProgress {
                step: GradingStepKind::ReadingAnswer,
                step_permille: 0,
            },
        );
    }

    #[test]
    fn grading_progress_fills_the_third_step_against_a_typical_reply_and_stops_at_full() {
        let half_typical_reply: GraderGenerationProgressKind = GraderGenerationProgressKind::WritingReply {
            generated_token_count: TYPICAL_GRADER_REPLY_TOKEN_COUNT / 2,
        };
        let longer_than_typical_reply: GraderGenerationProgressKind = GraderGenerationProgressKind::WritingReply {
            generated_token_count: TYPICAL_GRADER_REPLY_TOKEN_COUNT * 2,
        };
        let half_progress: GradingProgress = grading_progress(GraderStateKind::Ready, Some(half_typical_reply));
        let longer_progress: GradingProgress = grading_progress(GraderStateKind::Ready, Some(longer_than_typical_reply));

        assert_eq!(half_progress.step, GradingStepKind::WritingFeedback);
        assert_eq!(half_progress.step_permille, 500);
        assert_eq!(longer_progress.overall_permille(), GRADING_PROGRESS_MAXIMUM_PERMILLE);
    }
}
