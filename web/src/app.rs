use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::{ParamSegment, StaticSegment};

use crate::course_data::{
    self, COURSE_ID_PARAM, COURSE_SEGMENT, CourseStore, GLOSSARY_SEGMENT, PROGRESS_SEGMENT, REVIEW_SEGMENT, UNIT_ID_PARAM, UNIT_SEGMENT,
    VersionLoadState, provide_course_store, use_course_store,
};
use crate::courses::CoursesPage;
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
                    <title>"Language"</title>
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
                    <Route path=StaticSegment("") view=CoursesPage />
                    <Route path=(StaticSegment(COURSE_SEGMENT), ParamSegment(COURSE_ID_PARAM)) view=HomePage />
                    <Route
                        path=(StaticSegment(COURSE_SEGMENT), ParamSegment(COURSE_ID_PARAM), StaticSegment(UNIT_SEGMENT), ParamSegment(UNIT_ID_PARAM))
                        view=UnitPage
                    />
                    <Route path=(StaticSegment(COURSE_SEGMENT), ParamSegment(COURSE_ID_PARAM), StaticSegment(REVIEW_SEGMENT)) view=ReviewPage />
                    <Route path=(StaticSegment(COURSE_SEGMENT), ParamSegment(COURSE_ID_PARAM), StaticSegment(GLOSSARY_SEGMENT)) view=GlossaryPage />
                    <Route path=(StaticSegment(COURSE_SEGMENT), ParamSegment(COURSE_ID_PARAM), StaticSegment(PROGRESS_SEGMENT)) view=ProgressPage />
                </Routes>
            </main>
        </Router>
    }
}

/// The course the page is under, else the one the learner opened last.
fn active_course_id(path: &str, progress_store: ProgressStore) -> Option<String> {
    course_data::course_id_in_path(path).or_else(|| progress_store.read(|progress| progress.settings.last_course_id.clone()))
}

/// The title the published course list gives the course; `None` before the list loads or for an unknown course.
fn published_course_title(course_store: CourseStore, course_id: &str) -> Option<String> {
    match course_store.state() {
        VersionLoadState::Loaded(version) => version.source.manifest.courses.get(course_id).map(|manifest_course| manifest_course.title.clone()),
        VersionLoadState::Loading | VersionLoadState::Failed(_) => None,
    }
}

#[component]
fn NavigationBar() -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let course_store: CourseStore = use_course_store();
    let location = use_location();
    let course_id = move || active_course_id(&location.pathname.get(), progress_store);

    let course_title = move || {
        let course_title: Option<String> = course_id().and_then(|course_id| published_course_title(course_store, &course_id));

        course_title.map(|course_title| view! { <span class="chrome-course">{course_title}</span> })
    };

    let due_count = move |course_id: &str| {
        let now_milliseconds: i64 = platform::now_milliseconds();

        progress_store.read(|progress| progress.course_progress(course_id).due_card_count(now_milliseconds))
    };
    let course_links = move || {
        course_id().map(|course_id| {
            let due: usize = due_count(&course_id);

            view! {
                <A href=course_data::course_path(&course_id) exact=true>"Course"</A>
                <A href=course_data::review_path(&course_id)>
                    "Review"
                    {(due > 0).then(|| view! { <span class="chrome-count numeric" title="Words ready to review now">{due}</span> })}
                </A>
                <A href=course_data::glossary_path(&course_id)>"Glossary"</A>
                <A href=course_data::progress_path(&course_id)>"Progress"</A>
            }
        })
    };
    let slow_audio = move || progress_store.read(|progress| progress.settings.slow_audio);
    let audio_engine = move || progress_store.read(|progress| progress.settings.audio_engine);

    view! {
        <nav class="chrome">
            <A href="/" attr:class="chrome-brand" attr:title="All courses" exact=true>
                <span class="chrome-brand-name">"Language"</span>
                {course_title}
            </A>
            {course_links}
            <div class="chrome-settings">
                <label class="field">
                    <span class="field-label">"Voice"</span>
                    <select on:change:target=move |event| {
                        let engine: AudioEngineKind = match event.target().value().as_str() {
                            "kokoro" => AudioEngineKind::Kokoro,
                            "macos_say" => AudioEngineKind::MacosSay,
                            _ => AudioEngineKind::Auto,
                        };

                        progress_store.update(|progress| progress.settings.audio_engine = engine);
                    }>
                        <option value="auto" selected=move || audio_engine() == AudioEngineKind::Auto>"Auto"</option>
                        <option value="kokoro" selected=move || audio_engine() == AudioEngineKind::Kokoro>"Neural (Kokoro)"</option>
                        <option value="macos_say" selected=move || audio_engine() == AudioEngineKind::MacosSay>"macOS (Luciana)"</option>
                    </select>
                </label>
                <label class="field">
                    <input
                        type="checkbox"
                        prop:checked=slow_audio
                        on:change=move |_| progress_store.update(|progress| progress.settings.slow_audio = !progress.settings.slow_audio)
                    />
                    <span class="field-label">"Slow audio"</span>
                </label>
            </div>
        </nav>
    }
}
