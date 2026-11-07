use shared::grading::{self, GraderFinding, GraderResponse};

const BYTES_PER_GIGABYTE: f64 = 1_000_000_000.0;
const PERMILLE: u64 = 1000;

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

    /// Thousandths of the download done, which change often enough to show progress and seldom enough to redraw it.
    pub fn downloaded_permille(&self) -> u64 {
        let total_byte_count: u64 = self.total_or_expected_byte_count();

        (self.downloaded_byte_count.min(total_byte_count) * PERMILLE / total_byte_count).min(PERMILLE)
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
    Loading,
    Ready,
    /// The model failed to load in this browser.
    Failed,
}

/// What the browser reports about itself and its storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrowserCapabilityFacts {
    /// A WebGPU adapter that is not a fallback adapter, which runs on the CPU.
    pub has_webgpu_adapter: bool,
    /// Absent when the browser does not estimate its storage.
    pub available_storage_byte_count: Option<u64>,
    pub model_downloaded: bool,
    pub earlier_load_failed: bool,
}

pub fn assess_browser_capability(facts: &BrowserCapabilityFacts) -> GraderStateKind {
    if !facts.has_webgpu_adapter || facts.earlier_load_failed {
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
            available_storage_byte_count: Some(grading::GRADING_MODEL_SIZE_BYTES),
            model_downloaded: false,
            earlier_load_failed: false,
        }
    }

    #[test]
    fn assess_browser_capability_needs_an_adapter_room_for_the_model_and_no_failed_load() {
        let mut without_adapter: BrowserCapabilityFacts = capable_facts();
        without_adapter.has_webgpu_adapter = false;
        let mut without_room: BrowserCapabilityFacts = capable_facts();
        without_room.available_storage_byte_count = Some(grading::GRADING_MODEL_SIZE_BYTES - 1);
        let mut without_estimate: BrowserCapabilityFacts = capable_facts();
        without_estimate.available_storage_byte_count = None;
        let mut after_failed_load: BrowserCapabilityFacts = capable_facts();
        after_failed_load.model_downloaded = true;
        after_failed_load.earlier_load_failed = true;

        assert_eq!(assess_browser_capability(&capable_facts()), GraderStateKind::NotDownloaded);
        assert_eq!(assess_browser_capability(&without_adapter), GraderStateKind::Unsupported);
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
    fn description_states_gigabytes_against_the_expected_size_until_the_total_is_known() {
        let progress: ModelDownloadProgress = ModelDownloadProgress {
            downloaded_byte_count: 1_260_000_000,
            total_byte_count: None,
        };

        assert_eq!(progress.description(), "1.3 of 7.0 GB");
        assert_eq!(progress.downloaded_permille(), 180);
    }
}
