use web_sys::Element;

use crate::app::{App, STATIC_SHELL_ELEMENT_ID};

enum ServedDocumentKind {
    StaticShell,
    ServerRender,
}

// cargo-leptos looks up this export by name
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    _ = console_log::init_with_level(log::Level::Debug);

    let served_document_kind: ServedDocumentKind = read_served_document_kind();

    match served_document_kind {
        ServedDocumentKind::StaticShell => {
            log::debug!("the client is rendering over the static shell");

            // The shell's markup belongs to no route.
            leptos::tachys::dom::body().set_inner_html("");
            leptos::mount::mount_to_body(App);
        },
        ServedDocumentKind::ServerRender => {
            log::debug!("the client is attaching to the served document");

            leptos::mount::hydrate_body(App);
        },
    }
}

fn read_served_document_kind() -> ServedDocumentKind {
    let static_shell_element: Option<Element> = leptos::prelude::document().get_element_by_id(STATIC_SHELL_ELEMENT_ID);

    match static_shell_element {
        Some(_) => ServedDocumentKind::StaticShell,
        None => ServedDocumentKind::ServerRender,
    }
}
