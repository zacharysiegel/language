use leptos::prelude::*;
use shared::progress::{UnitProgress, UnitStatusKind};

pub const UNIT_STATUS_KINDS: [UnitStatusKind; 3] = [
    UnitStatusKind::NotStarted,
    UnitStatusKind::InProgress,
    UnitStatusKind::Completed,
];

pub fn unit_status_label(status: UnitStatusKind) -> &'static str {
    match status {
        UnitStatusKind::NotStarted => "Not started",
        UnitStatusKind::InProgress => "In progress",
        UnitStatusKind::Completed => "Completed",
    }
}

/// The value a status takes in a form control.
pub fn unit_status_code(status: UnitStatusKind) -> &'static str {
    match status {
        UnitStatusKind::NotStarted => "not_started",
        UnitStatusKind::InProgress => "in_progress",
        UnitStatusKind::Completed => "completed",
    }
}

pub fn unit_status_from_code(code: &str) -> Option<UnitStatusKind> {
    UNIT_STATUS_KINDS.into_iter().find(|status| unit_status_code(*status) == code)
}

/// A unit's status tag and best practice score, as the course and progress pages show it.
pub fn unit_status_view(unit_progress: UnitProgress) -> AnyView {
    let percent: u32 = (unit_progress.best_proportion * 100.0).round() as u32;
    let label: &'static str = unit_status_label(unit_progress.status);

    match unit_progress.status {
        UnitStatusKind::Completed => {
            view! { <span class="tag tag-ink">{label}</span>" "<span class="numeric muted">{percent}"%"</span> }.into_any()
        },
        UnitStatusKind::InProgress if unit_progress.attempt_count > 0 => {
            view! { <span class="tag tag-active">{label}</span>" "<span class="numeric muted">{percent}"%"</span> }.into_any()
        },
        UnitStatusKind::InProgress => view! { <span class="tag tag-active">{label}</span> }.into_any(),
        UnitStatusKind::NotStarted => view! { <span class="tag">{label}</span> }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_status_from_code_reads_every_code() {
        for status in UNIT_STATUS_KINDS {
            assert_eq!(unit_status_from_code(unit_status_code(status)), Some(status));
        }

        assert_eq!(unit_status_from_code("started"), None);
    }
}
