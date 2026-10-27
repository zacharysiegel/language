use leptos::prelude::*;
use shared::progress::UnitStatusKind;

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

/// A unit's status tag, as the course and progress pages show it.
pub fn unit_status_view(status: UnitStatusKind) -> impl IntoView {
    let tag_class: &'static str = match status {
        UnitStatusKind::Completed => "tag tag-ink",
        UnitStatusKind::InProgress => "tag tag-active",
        UnitStatusKind::NotStarted => "tag",
    };

    view! { <span class=tag_class>{unit_status_label(status)}</span> }
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
