use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use crate::course_data::{CourseLoadState, CourseStore, provide_course_store, use_course_store};
use crate::glossary::GlossaryPage;
use crate::home::HomePage;
use crate::platform;
use crate::progress_state::{ProgressStore, provide_progress_store, use_progress_store};
use crate::progress_view::ProgressPage;
use crate::review::ReviewPage;
use crate::unit::UnitPage;
use shared::progress::AudioEngineKind;

#[cfg(feature = "ssr")] // the wasm32 client never renders the document shell
pub use ssr::shell;

#[cfg(feature = "ssr")] // the wasm32 client never renders the document shell
mod ssr {
    use leptos::hydration::{AutoReload, HydrationScripts};
    use leptos::prelude::*;
    use leptos_meta::HashedStylesheet;

    use super::App;

    pub fn shell(options: LeptosOptions) -> impl IntoView {
        view! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>"Português do Brasil"</title>
                    <HashedStylesheet options=options.clone() id="leptos" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                </head>
                <body>
                    <App />
                </body>
            </html>
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_course_store();
    provide_progress_store();

    view! {
        <Router>
            <NavigationBar />
            <main>
                <Routes fallback=|| view! { <p class="status">"Page not found."</p> }>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=(StaticSegment("unit"), ParamSegment("unit_id")) view=UnitPage />
                    <Route path=StaticSegment("review") view=ReviewPage />
                    <Route path=StaticSegment("glossary") view=GlossaryPage />
                    <Route path=StaticSegment("progress") view=ProgressPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavigationBar() -> impl IntoView {
    let course_store: CourseStore = use_course_store();
    let progress_store: ProgressStore = use_progress_store();

    let due_count = move || {
        let CourseLoadState::Loaded(_) = course_store.state() else {
            return 0;
        };
        let now_milliseconds: i64 = platform::now_milliseconds();

        progress_store.read(|progress| {
            progress
                .cards
                .values()
                .filter(|card| card.due_milliseconds <= now_milliseconds)
                .count()
        })
    };
    let slow_audio = move || progress_store.read(|progress| progress.settings.slow_audio);
    let audio_engine = move || progress_store.read(|progress| progress.settings.audio_engine);

    view! {
        <nav class="top">
            <A href="/" attr:class="brand">"Português"</A>
            <A href="/">"Course"</A>
            <A href="/review">
                "Review"
                {move || {
                    (due_count() > 0)
                        .then(|| view! { <span class="badge" title="Words ready to review now">{due_count()}</span> })
                }}
            </A>
            <A href="/glossary">"Glossary"</A>
            <A href="/progress">"Progress"</A>
            <label class="toggle">
                "Voice "
                <select on:change:target=move |event| {
                    let engine: AudioEngineKind = match event.target().value().as_str() {
                        "kokoro" => AudioEngineKind::Kokoro,
                        "macos_say" => AudioEngineKind::MacosSay,
                        _ => AudioEngineKind::Auto,
                    };

                    progress_store.update(|progress| progress.settings.audio_engine = engine);
                }>
                    <option value="auto" selected=move || audio_engine() == AudioEngineKind::Auto>"Auto (best for each clip)"</option>
                    <option value="kokoro" selected=move || audio_engine() == AudioEngineKind::Kokoro>"Neural (Kokoro)"</option>
                    <option value="macos_say" selected=move || audio_engine() == AudioEngineKind::MacosSay>"macOS (Luciana)"</option>
                </select>
            </label>
            <label class="toggle">
                <input
                    type="checkbox"
                    prop:checked=slow_audio
                    on:change=move |_| progress_store.update(|progress| progress.settings.slow_audio = !progress.settings.slow_audio)
                />
                " Slow audio"
            </label>
        </nav>
    }
}
