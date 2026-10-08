use leptos::prelude::*;
use shared::progress::UnitProgress;

/// A unit's status tag and best score, as the course and progress pages show it.
pub fn unit_status_view(unit_progress: UnitProgress) -> AnyView {
    let percent: u32 = (unit_progress.best_proportion * 100.0).round() as u32;

    if unit_progress.completed {
        view! { <span class="tag tag-ink">"Completed"</span>" "<span class="numeric muted">{percent}"%"</span> }.into_any()
    } else if unit_progress.attempt_count > 0 {
        view! { <span class="tag tag-active">"In progress"</span>" "<span class="numeric muted">{percent}"%"</span> }.into_any()
    } else {
        view! { <span class="tag">"Not started"</span> }.into_any()
    }
}
