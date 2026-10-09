use leptos::prelude::*;
use shared::practice;
use shared::progress::UnitStatusKind;

use crate::progress_state::{self, ProgressStore, use_progress_store};

#[component]
pub fn UnitStatusControl(course_id: String, unit_id: String) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();

    let status: Memo<UnitStatusKind> = {
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        Memo::new(move |_| progress_store.unit_status(&course_id, &unit_id))
    };

    let set_status = move |code: String| {
        let Some(chosen) = progress_state::unit_status_from_code(&code)
        else {
            return;
        };

        progress_store.set_unit_status(&course_id, &unit_id, chosen);
    };

    let status_options: Vec<_> = progress_state::UNIT_STATUS_KINDS
        .into_iter()
        .map(|option_status| {
            view! {
                <option value=progress_state::unit_status_code(option_status) prop:selected=move || status.get() == option_status>
                    {progress_state::unit_status_label(option_status)}
                </option>
            }
        })
        .collect();

    let help_shown: RwSignal<bool> = RwSignal::new(false);
    let caption: String = format!(
        "Starting a practice also starts the unit. Passing the full practice with at least {}% completes it.",
        practice::pass_percent(),
    );

    view! {
        <div class="unit-status-control">
            <label class="field">
                <span class="field-label">"Status"</span>
                <select on:change:target=move |event| set_status(event.target().value())>{status_options}</select>
            </label>
            <button
                type="button"
                class="help"
                aria-label="How a unit's status changes"
                aria-expanded=move || help_shown.get().to_string()
                on:click=move |_| help_shown.update(|shown| *shown = !*shown)
            >
                "?"
            </button>
            {move || help_shown.get().then(|| view! { <p class="muted unit-status-help">{caption.clone()}</p> })}
        </div>
    }
}
