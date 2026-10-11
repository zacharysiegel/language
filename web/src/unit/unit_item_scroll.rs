use leptos::web_sys::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemFocusChangeKind {
    MoveFocus,
    KeepFocus,
}

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
pub use browser::*;
#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
pub use server::*;

#[cfg(feature = "hydrate")] // needs the browser APIs the server build lacks
mod browser {
    use std::cell::RefCell;

    use leptos::prelude::{document, request_animation_frame, window};
    use leptos::wasm_bindgen::JsCast;
    use web_sys::{
        CssStyleDeclaration,
        DomRect,
        FocusOptions,
        HtmlCollection,
        HtmlElement,
        ScrollBehavior,
        ScrollIntoViewOptions,
        ScrollLogicalPosition,
    };

    use super::*;

    const OVERFLOW_PROPERTY: &str = "overflow";
    const OVERFLOW_HIDDEN: &str = "hidden";

    thread_local! {
        static PAGE_HOLD: RefCell<Option<PageHold>> = const { RefCell::new(None) };
    }

    /// Overlapping holds of the page, of which only the last to end restores the overflow saved by the first.
    struct PageHold {
        prior_overflow: String,
        active_hold_count: usize,
    }

    /// Scrolls the item's top at once to the top of the viewport, less the item's scroll margin, ending a smooth scroll still in motion.
    pub fn jump_to_item(element_id: &str, focus_change: ItemFocusChangeKind) {
        let Some(item) = document().get_element_by_id(element_id)
        else {
            return;
        };

        let scroll_options: ScrollIntoViewOptions = ScrollIntoViewOptions::new();

        scroll_options.set_behavior(ScrollBehavior::Instant);
        scroll_options.set_block(ScrollLogicalPosition::Start);
        item.scroll_into_view_with_scroll_into_view_options(&scroll_options);

        if focus_change == ItemFocusChangeKind::KeepFocus {
            return;
        }

        let Ok(focusable_item) = item.dyn_into::<HtmlElement>()
        else {
            return;
        };

        let focus_options: FocusOptions = FocusOptions::new();

        focus_options.set_prevent_scroll(true);
        _ = focusable_item.focus_with_options(&focus_options);
    }

    /// The index of the last item whose top is at or above the activation line, or 0 when none is.
    pub fn current_item_index(element_ids: &[String], activation_line_proportion: f64) -> usize {
        let viewport_height: f64 = document().document_element().map_or(0.0, |root| f64::from(root.client_height()));
        let activation_line: f64 = viewport_height * activation_line_proportion;
        let crossed_item_count: usize = element_ids
            .iter()
            .map_while(|element_id| document().get_element_by_id(element_id))
            .take_while(|item| item.get_bounding_client_rect().top() <= activation_line)
            .count();

        crossed_item_count.saturating_sub(1)
    }

    /// The index of the rail entry whose vertical center is nearest the pointer.
    pub fn nearest_entry_index(rail: &Element, client_y: f64) -> Option<usize> {
        let entries: HtmlCollection = rail.children();

        (0..entries.length())
            .filter_map(|entry_index| entries.item(entry_index))
            .map(|entry| {
                let entry_rect: DomRect = entry.get_bounding_client_rect();
                let entry_center: f64 = entry_rect.top() + entry_rect.height() / 2.0;

                (entry_center - client_y).abs()
            })
            .enumerate()
            .min_by(|(_, distance), (_, other_distance)| distance.total_cmp(other_distance))
            .map(|(entry_index, _)| entry_index)
    }

    pub fn scroll_offset() -> f64 {
        window().scroll_y().unwrap_or(0.0)
    }

    pub fn capture_pointer(rail: &Element, pointer_id: i32) {
        _ = rail.set_pointer_capture(pointer_id);
    }

    /// Keeps the page from scrolling until a frame has drawn, then calls `after_hold`.
    pub fn hold_page_still(after_hold: impl FnOnce() + 'static) {
        let Some(root_style) = root_style()
        else {
            after_hold();
            return;
        };

        begin_page_hold(&root_style);

        // The second frame is the first one the compositor draws with the page unscrollable.
        request_animation_frame(move || {
            request_animation_frame(move || {
                after_hold();
                end_page_hold(&root_style);
            });
        });
    }

    fn begin_page_hold(root_style: &CssStyleDeclaration) {
        PAGE_HOLD.with_borrow_mut(|page_hold| {
            if let Some(active_page_hold) = page_hold {
                active_page_hold.active_hold_count += 1;
                return;
            }

            let prior_overflow: String = root_style.get_property_value(OVERFLOW_PROPERTY).unwrap_or_default();

            _ = root_style.set_property(OVERFLOW_PROPERTY, OVERFLOW_HIDDEN);
            *page_hold = Some(PageHold {
                prior_overflow,
                active_hold_count: 1,
            });
        });
    }

    fn end_page_hold(root_style: &CssStyleDeclaration) {
        PAGE_HOLD.with_borrow_mut(|page_hold| {
            let Some(active_page_hold) = page_hold.as_mut()
            else {
                return;
            };

            active_page_hold.active_hold_count -= 1;

            if active_page_hold.active_hold_count > 0 {
                return;
            }

            let ended_page_hold: Option<PageHold> = page_hold.take();

            // An empty value removes the declaration, restoring the stylesheet's overflow.
            if let Some(ended_page_hold) = ended_page_hold {
                _ = root_style.set_property(OVERFLOW_PROPERTY, &ended_page_hold.prior_overflow);
            }
        });
    }

    fn root_style() -> Option<CssStyleDeclaration> {
        let root: HtmlElement = document().document_element()?.dyn_into::<HtmlElement>().ok()?;

        Some(root.style())
    }
}

#[cfg(not(feature = "hydrate"))] // has no browser APIs to call
mod server {
    use super::*;

    pub fn jump_to_item(_element_id: &str, _focus_change: ItemFocusChangeKind) {}

    pub fn current_item_index(_element_ids: &[String], _activation_line_proportion: f64) -> usize {
        0
    }

    pub fn nearest_entry_index(_rail: &Element, _client_y: f64) -> Option<usize> {
        None
    }

    pub fn scroll_offset() -> f64 {
        0.0
    }

    pub fn capture_pointer(_rail: &Element, _pointer_id: i32) {}

    pub fn hold_page_still(_after_hold: impl FnOnce() + 'static) {}
}
