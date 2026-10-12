use leptos::ev::{self, PointerEvent};
use leptos::html;
use leptos::prelude::*;
use leptos::web_sys::HtmlOListElement;
use shared::course::unit_item_label;

use crate::item_index::ItemFocusChangeKind;
use crate::item_index::item_index_scroll;

/// How far down the viewport, as a proportion of its height, an item's top must have scrolled to make it the current item.
const CURRENT_ITEM_ACTIVATION_LINE_PROPORTION: f64 = 0.25;
/// Scroll offsets this close are the same position, since a browser can round a scroll to a fraction of a pixel.
const SAME_SCROLL_OFFSET_TOLERANCE: f64 = 1.0;
const PRIMARY_POINTER_BUTTON: i16 = 0;
const TOUCH_POINTER_TYPE: &str = "touch";
/// The class of an element an index entry jumps to.
pub const ITEM_INDEX_TARGET_CLASS: &str = "item-index-target";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemIndexEntry {
    /// The id of the element the entry jumps to; an index's entries are in the page order of their elements.
    pub target_element_id: String,
    /// A few characters, to fit the rail's width.
    pub visible_text: String,
    pub accessible_label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RailDragState {
    Idle,
    Pressed(i32),
}

/// A rail of entries along the page edge, for jumping to an item by tapping or dragging, shown only for enough entries.
#[component]
pub fn ItemIndex(index_label: &'static str, entries: Vec<ItemIndexEntry>) -> impl IntoView {
    if entries.len() < unit_item_label::UNIT_ITEM_INDEX_MINIMUM_ITEM_COUNT {
        return ().into_any();
    }

    let element_ids: StoredValue<Vec<String>> = StoredValue::new(entries.iter().map(|entry| entry.target_element_id.clone()).collect());
    let current_item_index: RwSignal<usize> = RwSignal::new(0);
    // Where a jump left the page, so the jumped-to item stays current even when the page cannot scroll it to the top.
    let jump_scroll_offset: StoredValue<Option<f64>> = StoredValue::new(None);
    let drag: StoredValue<RailDragState> = StoredValue::new(RailDragState::Idle);
    let rail_ref: NodeRef<html::Ol> = NodeRef::new();

    // Unlike the current item, kept when the page moves away from where the jump left it.
    let latest_jumped_item_index: StoredValue<usize> = StoredValue::new(0);

    let jump_to_item = move |item_index: usize, focus_change: ItemFocusChangeKind| {
        element_ids.with_value(|element_ids| item_index_scroll::jump_to_item(&element_ids[item_index], focus_change));
        jump_scroll_offset.set_value(Some(item_index_scroll::scroll_offset()));
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
        let scroll_offset: f64 = item_index_scroll::scroll_offset();
        let still_where_jump_left: bool = jump_scroll_offset
            .get_value()
            .is_some_and(|jumped_offset| (jumped_offset - scroll_offset).abs() < SAME_SCROLL_OFFSET_TOLERANCE);

        if still_where_jump_left {
            return;
        }

        jump_scroll_offset.set_value(None);

        let scrolled_item_index: usize = element_ids
            .with_value(|element_ids| item_index_scroll::current_item_index(element_ids, CURRENT_ITEM_ACTIVATION_LINE_PROPORTION));

        set_if_changed(current_item_index, scrolled_item_index);
    };

    let entry_index_under_pointer = move |client_y: f64| -> Option<usize> {
        let rail: HtmlOListElement = rail_ref.get_untracked()?;

        item_index_scroll::nearest_entry_index(&rail, client_y)
    };

    Effect::new(move |_| follow_scrolling());

    let scroll_listener: WindowListenerHandle = window_event_listener(ev::scroll, move |_| follow_scrolling());
    let resize_listener: WindowListenerHandle = window_event_listener(ev::resize, move |_| follow_scrolling());

    on_cleanup(move || {
        scroll_listener.remove();
        resize_listener.remove();
    });

    let start_drag = move |event: PointerEvent| {
        if event.button() != PRIMARY_POINTER_BUTTON || !event.is_primary() {
            return;
        }

        // Keeps a mouse press from selecting text or focusing the entry; a click still follows.
        event.prevent_default();
        drag.set_value(RailDragState::Pressed(event.pointer_id()));

        // Jumps even to the current entry, since the press may land while the page is still scrolling.
        let Some(entry_index) = entry_index_under_pointer(f64::from(event.client_y()))
        else {
            return;
        };

        jump_to_item(entry_index, ItemFocusChangeKind::KeepFocus);

        // An instant scroll does not end a touch fling, which would carry the page on past the item.
        if event.pointer_type() == TOUCH_POINTER_TYPE {
            item_index_scroll::hold_page_still(repeat_latest_jump);
        }
    };

    let continue_drag = move |event: PointerEvent| {
        let RailDragState::Pressed(pointer_id) = drag.get_value()
        else {
            return;
        };

        if pointer_id != event.pointer_id() {
            return;
        }

        // Captured only once the pointer moves, so a tap still ends in a click on the entry it pressed.
        if let Some(rail) = rail_ref.get_untracked() {
            item_index_scroll::capture_pointer(&rail, pointer_id);
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
        let RailDragState::Pressed(pointer_id) = drag.get_value()
        else {
            return;
        };

        if pointer_id != event.pointer_id() {
            return;
        }

        drag.set_value(RailDragState::Idle);
    };

    let entry_views: Vec<_> = entries
        .into_iter()
        .enumerate()
        .map(|(item_index, entry)| {
            view! {
                <li>
                    <button
                        type="button"
                        aria-label=entry.accessible_label
                        aria-current=move || (current_item_index.get() == item_index).then_some("location")
                        on:click=move |_| jump_to_item(item_index, ItemFocusChangeKind::MoveFocus)
                    >
                        {entry.visible_text}
                    </button>
                </li>
            }
        })
        .collect();

    view! {
        <nav class="item-index" aria-label=index_label>
            <ol
                node_ref=rail_ref
                on:pointerdown=start_drag
                on:pointermove=continue_drag
                on:pointerup=end_drag
                on:pointercancel=end_drag
            >
                {entry_views}
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
