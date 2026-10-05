use crate::app::App;

// cargo-leptos looks up this export by name
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    _ = console_log::init_with_level(log::Level::Debug);

    log::debug!("the client is attaching to the served document");

    leptos::mount::hydrate_body(App);
}
