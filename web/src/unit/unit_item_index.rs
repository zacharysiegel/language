use std::time::Duration;

use leptos::ev::{self, PointerEvent};
use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos::web_sys::HtmlOListElement;
use shared::course::Unit;
use shared::course::unit_item_label;

use crate::unit::ItemFocusChangeKind;
use crate::unit::unit_item_scroll;

/// How far down the viewport, as a proportion of its height, an item's top must have scrolled to make it the current item.
const CURRENT_ITEM_ACTIVATION_LINE_PROPORTION: f64 = 0.25;
/// Scroll offsets this close are the same position, since a browser can round a scroll to a fraction of a pixel.
const SAME_SCROLL_OFFSET_TOLERANCE: f64 = 1.0;
const PRIMARY_POINTER_BUTTON: i16 = 0;
const TOUCH_POINTER_TYPE: &str = "touch";
/// How long the rail stays shown once the page stops scrolling and no pointer holds the rail.
const RAIL_IDLE_HIDE_DELAY: Duration = Duration::from_millis(1_500);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitItemKind {
    GrammarNote,
    Dialogue,
    Reading,
}

impl UnitItemKind {
    fn noun(self) -> &'static str {
        match self {
            UnitItemKind::GrammarNote => "note",
            UnitItemKind::Dialogue => "dialogue",
            UnitItemKind::Reading => "reading",
        }
    }

    fn index_label(self) -> &'static str {
        match self {
            UnitItemKind::GrammarNote => "Note index",
            UnitItemKind::Dialogue => "Dialogue index",
            UnitItemKind::Reading => "Reading index",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RailDragState {
    Idle,
    Pressed(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RailHoverKind {
    Away,
    Over,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RailVisibilityKind {
    Hidden,
    Shown,
}

/// The rail's visibility, shown while the page scrolls or a pointer holds the rail, and hidden after an idle delay.
#[derive(Clone, Copy)]
struct RailPresence {
    visibility: RwSignal<RailVisibilityKind>,
    pending_hide: StoredValue<Option<TimeoutHandle>>,
    hover: StoredValue<RailHoverKind>,
    drag: StoredValue<RailDragState>,
}

impl RailPresence {
    fn new() -> Self {
        Self {
            visibility: RwSignal::new(RailVisibilityKind::Hidden),
            pending_hide: StoredValue::new(None),
            hover: StoredValue::new(RailHoverKind::Away),
            drag: StoredValue::new(RailDragState::Idle),
        }
    }

    fn is_shown(self) -> bool {
        self.visibility.get() == RailVisibilityKind::Shown
    }

    fn is_held(self) -> bool {
        self.hover.get_value() == RailHoverKind::Over || matches!(self.drag.get_value(), RailDragState::Pressed(_))
    }

    fn show(self) {
        if self.visibility.get_untracked() != RailVisibilityKind::Shown {
            self.visibility.set(RailVisibilityKind::Shown);
        }

        self.hide_once_idle();
    }

    /// Starts the idle delay over, unless a pointer holds the rail.
    fn hide_once_idle(self) {
        self.cancel_pending_hide();

        if self.is_held() {
            return;
        }

        let scheduled: Result<TimeoutHandle, JsValue> = set_timeout_with_handle(move || self.hide(), RAIL_IDLE_HIDE_DELAY);

        match scheduled {
            Ok(handle) => self.pending_hide.set_value(Some(handle)),
            Err(error) => log::warn!("could not schedule hiding the item index; [error={:?}]", error),
        }
    }

    fn cancel_pending_hide(self) {
        let pending_hide: Option<TimeoutHandle> = self.pending_hide.get_value();

        if let Some(handle) = pending_hide {
            handle.clear();
        }

        self.pending_hide.set_value(None);
    }

    fn hide(self) {
        self.pending_hide.set_value(None);

        if self.visibility.get_untracked() != RailVisibilityKind::Hidden {
            self.visibility.set(RailVisibilityKind::Hidden);
        }
    }
}

/// The labels of a unit tab's items, in order.
pub fn unit_item_labels(unit: &Unit, item_count: usize) -> Vec<String> {
    (1..=item_count)
        .map(|item_number| unit_item_label::unit_item_label(&unit.level, unit.order, item_number))
        .collect()
}

fn unit_item_element_id(label: &str) -> String {
    format!("unit-item-{label}")
}

fn unit_item_title_id(label: &str) -> String {
    format!("unit-item-title-{label}")
}

/// A box in a unit tab, headed by its label and title, which the tab's index can jump to and focus.
#[component]
pub fn UnitItemArticle(kind_class: &'static str, label: String, title: String, children: Children) -> impl IntoView {
    let element_id: String = unit_item_element_id(&label);
    let title_id: String = unit_item_title_id(&label);
    let labelled_by_id: String = title_id.clone();

    view! {
        <article class=format!("{kind_class} panel unit-item") id=element_id aria-labelledby=labelled_by_id tabindex="-1">
            <header class="unit-item-header" id=title_id>
                <span class="page-caption">{label}</span>
                <h3>{title}</h3>
            </header>
            {children()}
        </article>
    }
}

/// A rail of the tab's item numbers over the page edge, for jumping to an item by tapping or dragging, shown while the page scrolls.
#[component]
pub fn UnitItemIndex(item_kind: UnitItemKind, labels: Vec<String>) -> impl IntoView {
    if labels.len() < unit_item_label::UNIT_ITEM_INDEX_MINIMUM_ITEM_COUNT {
        return ().into_any();
    }

    let element_ids: StoredValue<Vec<String>> = StoredValue::new(labels.iter().map(|label| unit_item_element_id(label)).collect());
    let current_item_index: RwSignal<usize> = RwSignal::new(0);
    // Where a jump left the page, so the jumped-to item stays current even when the page cannot scroll it to the top.
    let jump_scroll_offset: StoredValue<Option<f64>> = StoredValue::new(None);
    let presence: RailPresence = RailPresence::new();
    let rail_ref: NodeRef<html::Ol> = NodeRef::new();

    // Unlike the current item, kept when the page moves away from where the jump left it.
    let latest_jumped_item_index: StoredValue<usize> = StoredValue::new(0);

    let jump_to_item = move |item_index: usize, focus_change: ItemFocusChangeKind| {
        element_ids.with_value(|element_ids| unit_item_scroll::jump_to_item(&element_ids[item_index], focus_change));
        jump_scroll_offset.set_value(Some(unit_item_scroll::scroll_offset()));
        latest_jumped_item_index.set_value(item_index);
        set_if_changed(current_item_index, item_index);
    };

    let repeat_latest_jump = move || {
        let Some(item_index) = latest_jumped_item_index.try_get_value()
        else {
            return;
        };

        jump_to_item(item_index, ItemFocusChangeKind::KeepFocus);
    };

    let follow_scrolling = move || {
        let scroll_offset: f64 = unit_item_scroll::scroll_offset();
        let still_where_jump_left: bool = jump_scroll_offset
            .get_value()
            .is_some_and(|jumped_offset| (jumped_offset - scroll_offset).abs() < SAME_SCROLL_OFFSET_TOLERANCE);

        if still_where_jump_left {
            return;
        }

        jump_scroll_offset.set_value(None);

        let scrolled_item_index: usize =
            element_ids.with_value(|element_ids| unit_item_scroll::current_item_index(element_ids, CURRENT_ITEM_ACTIVATION_LINE_PROPORTION));

        set_if_changed(current_item_index, scrolled_item_index);
    };

    let entry_index_under_pointer = move |client_y: f64| -> Option<usize> {
        let rail: HtmlOListElement = rail_ref.get_untracked()?;

        unit_item_scroll::nearest_entry_index(&rail, client_y)
    };

    Effect::new(move |_| follow_scrolling());

    let scroll_listener: WindowListenerHandle = window_event_listener(ev::scroll, move |_| {
        follow_scrolling();
        presence.show();
    });
    let resize_listener: WindowListenerHandle = window_event_listener(ev::resize, move |_| follow_scrolling());

    on_cleanup(move || {
        scroll_listener.remove();
        resize_listener.remove();
        presence.cancel_pending_hide();
    });

    let start_drag = move |event: PointerEvent| {
        if event.button() != PRIMARY_POINTER_BUTTON || !event.is_primary() {
            return;
        }

        // Keeps a mouse press from selecting text or focusing the entry; a click still follows.
        event.prevent_default();
        presence.drag.set_value(RailDragState::Pressed(event.pointer_id()));
        presence.show();

        // Jumps even to the current entry, since the press may land while the page is still scrolling.
        let Some(entry_index) = entry_index_under_pointer(f64::from(event.client_y()))
        else {
            return;
        };

        jump_to_item(entry_index, ItemFocusChangeKind::KeepFocus);

        // An instant scroll does not end a touch fling, which would carry the page on past the item.
        if event.pointer_type() == TOUCH_POINTER_TYPE {
            unit_item_scroll::hold_page_still(repeat_latest_jump);
        }
    };

    let continue_drag = move |event: PointerEvent| {
        let RailDragState::Pressed(pointer_id) = presence.drag.get_value()
        else {
            return;
        };

        if pointer_id != event.pointer_id() {
            return;
        }

        // Captured only once the pointer moves, so a tap still ends in a click on the entry it pressed.
        if let Some(rail) = rail_ref.get_untracked() {
            unit_item_scroll::capture_pointer(&rail, pointer_id);
        }

        let Some(entry_index) = entry_index_under_pointer(f64::from(event.client_y()))
        else {
            return;
        };

        if entry_index != current_item_index.get_untracked() {
            jump_to_item(entry_index, ItemFocusChangeKind::KeepFocus);
        }
    };

    let end_drag = move |event: PointerEvent| {
        let RailDragState::Pressed(pointer_id) = presence.drag.get_value()
        else {
            return;
        };

        if pointer_id != event.pointer_id() {
            return;
        }

        presence.drag.set_value(RailDragState::Idle);
        presence.hide_once_idle();
    };

    let start_hover = move |_: PointerEvent| {
        presence.hover.set_value(RailHoverKind::Over);
        presence.show();
    };

    let end_hover = move |_: PointerEvent| {
        presence.hover.set_value(RailHoverKind::Away);
        presence.hide_once_idle();
    };

    let entries: Vec<_> = labels
        .iter()
        .enumerate()
        .map(|(item_index, label)| {
            let item_number: usize = item_index + 1;
            let entry_label: String = format!("Go to {} {label}", item_kind.noun());

            view! {
                <li>
                    <button
                        type="button"
                        aria-label=entry_label
                        aria-current=move || (current_item_index.get() == item_index).then_some("location")
                        on:click=move |_| jump_to_item(item_index, ItemFocusChangeKind::MoveFocus)
                    >
                        {item_number}
                    </button>
                </li>
            }
        })
        .collect();

    view! {
        <nav class="unit-item-index" class:shown=move || presence.is_shown() aria-label=item_kind.index_label()>
            <ol
                node_ref=rail_ref
                on:pointerdown=start_drag
                on:pointermove=continue_drag
                on:pointerup=end_drag
                on:pointercancel=end_drag
                on:pointerenter=start_hover
                on:pointerleave=end_hover
            >
                {entries}
            </ol>
        </nav>
    }
    .into_any()
}

fn set_if_changed(signal: RwSignal<usize>, value: usize) {
    if signal.get_untracked() != value {
        signal.set(value);
    }
}
