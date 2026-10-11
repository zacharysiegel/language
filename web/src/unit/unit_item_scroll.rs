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
    use leptos::prelude::{document, window};
    use leptos::wasm_bindgen::JsCast;
    use web_sys::{DomRect, FocusOptions, HtmlCollection, HtmlElement};

    use super::*;

    /// Scrolls the item's top to the top of the viewport, less the item's scroll margin.
    pub fn jump_to_item(element_id: &str, focus_change: ItemFocusChangeKind) {
        let Some(item) = document().get_element_by_id(element_id)
        else {
            return;
        };

        item.scroll_into_view();

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
}
