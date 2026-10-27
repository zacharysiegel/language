use std::time::Duration;

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, JsValue};
use leptos::web_sys::{Element, FocusEvent, Node};
use shared::practice;
use shared::progress::UnitStatusKind;

use crate::platform;
use crate::progress_state::{self, ProgressStore, use_progress_store};

const STATUS_HELP_TOOLTIP_ID: &str = "unit-status-help";
const STATUS_HELP_LABEL: &str = "How a unit's status changes";
const STATUS_HELP_AUTO_CLOSE_MILLISECONDS: i64 = 8_000;
const ESCAPE_KEY: &str = "Escape";
const KEYBOARD_FOCUS_SELECTOR: &str = ":focus-visible";

#[derive(Clone, Copy)]
enum AutoCloseCountdown {
    Stopped,
    Running {
        handle: TimeoutHandle,
        deadline_milliseconds: i64,
    },
    Paused {
        remaining_milliseconds: i64,
    },
}

#[derive(Clone, Copy)]
enum CountdownHoldKind {
    PointerOver,
    KeyboardFocusInside,
}

#[derive(Clone, Copy, Default)]
struct CountdownHolds {
    pointer_over: bool,
    keyboard_focus_inside: bool,
}

impl CountdownHolds {
    fn flag_mut(&mut self, hold: CountdownHoldKind) -> &mut bool {
        match hold {
            CountdownHoldKind::PointerOver => &mut self.pointer_over,
            CountdownHoldKind::KeyboardFocusInside => &mut self.keyboard_focus_inside,
        }
    }

    fn is_any_held(self) -> bool {
        self.pointer_over || self.keyboard_focus_inside
    }
}

/// A tooltip that closes by itself when its countdown runs out; the countdown waits while any hold is in place.
#[derive(Clone, Copy)]
struct AutoClosingTooltip {
    is_open: RwSignal<bool>,
    countdown: StoredValue<AutoCloseCountdown>,
    holds: StoredValue<CountdownHolds>,
}

impl AutoClosingTooltip {
    fn new() -> Self {
        Self {
            is_open: RwSignal::new(false),
            countdown: StoredValue::new(AutoCloseCountdown::Stopped),
            holds: StoredValue::new(CountdownHolds::default()),
        }
    }

    fn toggle(self) {
        if self.is_open.get_untracked() {
            self.close();
        } else {
            self.open();
        }
    }

    fn open(self) {
        self.cancel_countdown();
        self.is_open.set(true);

        if self.holds.get_value().is_any_held() {
            self.countdown.set_value(AutoCloseCountdown::Paused {
                remaining_milliseconds: STATUS_HELP_AUTO_CLOSE_MILLISECONDS,
            });
        } else {
            self.start_countdown(STATUS_HELP_AUTO_CLOSE_MILLISECONDS);
        }
    }

    fn close(self) {
        self.cancel_countdown();

        if self.is_open.get_untracked() {
            self.is_open.set(false);
        }
    }

    fn cancel_countdown(self) {
        let countdown: AutoCloseCountdown = self.countdown.get_value();

        if let AutoCloseCountdown::Running { handle, .. } = countdown {
            handle.clear();
        }

        self.countdown.set_value(AutoCloseCountdown::Stopped);
    }

    fn start_countdown(self, remaining_milliseconds: i64) {
        let delay: Duration = Duration::from_millis(u64::try_from(remaining_milliseconds).unwrap_or(0));
        let scheduled: Result<TimeoutHandle, JsValue> = set_timeout_with_handle(move || self.close(), delay);

        let countdown: AutoCloseCountdown = match scheduled {
            Ok(handle) => AutoCloseCountdown::Running {
                handle,
                deadline_milliseconds: platform::now_milliseconds() + remaining_milliseconds,
            },
            Err(error) => {
                log::warn!("could not schedule the tooltip's auto-close; [error={:?}]", error);
                AutoCloseCountdown::Stopped
            },
        };

        self.countdown.set_value(countdown);
    }

    fn hold(self, hold: CountdownHoldKind) {
        self.holds.update_value(|holds| *holds.flag_mut(hold) = true);

        let countdown: AutoCloseCountdown = self.countdown.get_value();

        if let AutoCloseCountdown::Running { handle, deadline_milliseconds } = countdown {
            handle.clear();

            let remaining_milliseconds: i64 = (deadline_milliseconds - platform::now_milliseconds()).max(0);
            self.countdown.set_value(AutoCloseCountdown::Paused { remaining_milliseconds });
        }
    }

    fn release(self, hold: CountdownHoldKind) {
        self.holds.update_value(|holds| *holds.flag_mut(hold) = false);

        if self.holds.get_value().is_any_held() {
            return;
        }

        let countdown: AutoCloseCountdown = self.countdown.get_value();

        if let AutoCloseCountdown::Paused { remaining_milliseconds } = countdown {
            self.start_countdown(remaining_milliseconds);
        }
    }
}

#[component]
pub fn UnitStatusControl(course_id: String, unit_id: String) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();

    let status: Memo<UnitStatusKind> = {
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        Memo::new(move |_| progress_store.unit_status(&course_id, &unit_id))
    };

    let best_score_percent: Memo<Option<u32>> = {
        let course_id: String = course_id.clone();
        let unit_id: String = unit_id.clone();

        Memo::new(move |_| progress_store.unit_best_score_percent(&course_id, &unit_id))
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

    view! {
        <div class="unit-status-control">
            <label class="field">
                <span class="field-label">"Status"</span>
                <select on:change:target=move |event| set_status(event.target().value())>{status_options}</select>
            </label>
            <StatusHelp />
            {move || best_score_percent.get().map(|percent| view! { <span class="unit-best-score numeric muted">"Best score "{percent}"%"</span> })}
        </div>
    }
}

#[component]
fn StatusHelp() -> impl IntoView {
    let tooltip: AutoClosingTooltip = AutoClosingTooltip::new();
    let anchor_ref: NodeRef<html::Div> = NodeRef::new();
    let help_button_ref: NodeRef<html::Button> = NodeRef::new();
    let tooltip_ref: NodeRef<html::Div> = NodeRef::new();

    let close_returning_focus = move || {
        let focus_was_inside: bool = contains_focus(tooltip_ref);

        tooltip.close();

        if !focus_was_inside {
            return;
        }

        if let Some(help_button) = help_button_ref.get_untracked() {
            _ = help_button.focus();
        }
    };

    let press_listener: WindowListenerHandle = window_event_listener(ev::pointerdown, move |event| {
        let target: Node = event_target::<Node>(&event);

        if tooltip.is_open.get_untracked() && !contains_node(anchor_ref, Some(&target)) {
            tooltip.close();
        }
    });

    let keydown_listener: WindowListenerHandle = window_event_listener(ev::keydown, move |event| {
        if tooltip.is_open.get_untracked() && event.key() == ESCAPE_KEY {
            close_returning_focus();
        }
    });

    on_cleanup(move || {
        press_listener.remove();
        keydown_listener.remove();
        tooltip.cancel_countdown();
    });

    let record_focus_entering = move |event: FocusEvent| {
        let focused: Element = event_target::<Element>(&event);
        let is_keyboard_focus: bool = focused.matches(KEYBOARD_FOCUS_SELECTOR).unwrap_or(false);

        if is_keyboard_focus {
            tooltip.hold(CountdownHoldKind::KeyboardFocusInside);
        } else {
            tooltip.release(CountdownHoldKind::KeyboardFocusInside);
        }
    };

    let record_focus_leaving = move |event: FocusEvent| {
        let next_focused: Option<Node> = event.related_target().and_then(|target| target.dyn_into::<Node>().ok());

        if !contains_node(tooltip_ref, next_focused.as_ref()) {
            tooltip.release(CountdownHoldKind::KeyboardFocusInside);
        }
    };

    let pass_rule: String = format!(
        "Passing the full practice with at least {}% completes it.",
        practice::pass_percent(),
    );

    view! {
        <div
            class="tooltip-anchor"
            node_ref=anchor_ref
            on:pointerenter=move |_| tooltip.hold(CountdownHoldKind::PointerOver)
            on:pointerleave=move |_| tooltip.release(CountdownHoldKind::PointerOver)
        >
            <button
                type="button"
                class="help"
                node_ref=help_button_ref
                aria-label=STATUS_HELP_LABEL
                aria-expanded=move || tooltip.is_open.get().to_string()
                aria-controls=STATUS_HELP_TOOLTIP_ID
                on:click=move |_| tooltip.toggle()
            >
                "?"
            </button>
            <div
                id=STATUS_HELP_TOOLTIP_ID
                class="tooltip"
                role="dialog"
                aria-label=STATUS_HELP_LABEL
                node_ref=tooltip_ref
                hidden=move || !tooltip.is_open.get()
                on:focusin=record_focus_entering
                on:focusout=record_focus_leaving
            >
                <ol>
                    <li>"Starting a practice also starts the unit."</li>
                    <li>{pass_rule}</li>
                </ol>
                <button type="button" class="tooltip-close" aria-label="Close" on:click=move |_| close_returning_focus()>
                    "×"
                </button>
            </div>
        </div>
    }
}

fn contains_node(container_ref: NodeRef<html::Div>, node: Option<&Node>) -> bool {
    container_ref
        .get_untracked()
        .is_some_and(|container| container.contains(node))
}

fn contains_focus(container_ref: NodeRef<html::Div>) -> bool {
    let focused: Option<Element> = document().active_element();

    contains_node(container_ref, focused.as_deref())
}
