use leptos::prelude::*;
use shared::course::Unit;
use shared::course::unit_item_label;

use crate::item_index::{ItemIndex, ItemIndexEntry, item_index};

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
        <article
            class=format!("{kind_class} panel unit-item {}", item_index::ITEM_INDEX_TARGET_CLASS)
            id=element_id
            aria-labelledby=labelled_by_id
            tabindex="-1"
        >
            <header class="unit-item-header" id=title_id>
                <span class="page-caption">{label}</span>
                <h3>{title}</h3>
            </header>
            {children()}
        </article>
    }
}

/// A rail of the tab's item numbers along the page edge.
#[component]
pub fn UnitItemIndex(item_kind: UnitItemKind, labels: Vec<String>) -> impl IntoView {
    let entries: Vec<ItemIndexEntry> = labels
        .iter()
        .enumerate()
        .map(|(item_index, label)| ItemIndexEntry {
            target_element_id: unit_item_element_id(label),
            visible_text: (item_index + 1).to_string(),
            accessible_label: format!("Go to {} {label}", item_kind.noun()),
        })
        .collect();

    view! { <ItemIndex index_label=item_kind.index_label() entries /> }
}
