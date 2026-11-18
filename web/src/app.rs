use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::{ParamSegment, StaticSegment};

use crate::course_data::{
    self, COURSE_ID_PARAM, COURSE_SEGMENT, CourseStore, GLOSSARY_SEGMENT, LoadState, LoadedCoursePublication, PROGRESS_SEGMENT, REVIEW_SEGMENT,
    UNIT_ID_PARAM, UNIT_SEGMENT, provide_course_store, use_course_store,
};
use crate::audio_player::{PlaybackAnnouncement, provide_playback_store};
use crate::courses::CoursesPage;
use crate::glossary::GlossaryPage;
use crate::grading::{GradingModelDownloadSetting, provide_grading_store};
use crate::home::HomePage;
use crate::pending_indication::{PageBusyStore, provide_page_busy_store, use_page_busy_store};
use crate::privacy::{PRIVACY_SEGMENT, PrivacyPage};
use crate::progress_state::{ProgressStore, provide_progress_store, use_progress_store};
use crate::progress_view::ProgressPage;
use crate::review::ReviewPage;
use crate::unit::UnitPage;
use shared::progress::AudioEngineKind;

/// The id of the static shell's placeholder content, which no other document holds.
pub const STATIC_SHELL_ELEMENT_ID: &str = "static-shell";

#[cfg(feature = "ssr")] // the wasm32 client never renders the document shell
pub use ssr::*;

#[cfg(feature = "ssr")] // the wasm32 client never renders the document shell
mod ssr {
    use leptos::hydration::{AutoReload, HydrationScripts};
    use leptos::prelude::*;
    use leptos_meta::HashedStylesheet;

    use super::{App, STATIC_SHELL_ELEMENT_ID};
    use crate::privacy::PrivacyPage;

    const APP_TITLE: &str = "Plainspoken";
    const PRIVACY_TITLE: &str = "Privacy policy · Plainspoken";

    #[component]
    fn DocumentHead(options: LeptosOptions, title: &'static str) -> impl IntoView {
        view! {
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{title}</title>
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32.png" />
                <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
                <HashedStylesheet options=options.clone() id="leptos" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
        }
    }

    /// The document for the requested route.
    pub fn shell(options: LeptosOptions) -> impl IntoView {
        view! {
            <!DOCTYPE html>
            <html lang="en">
                <DocumentHead options title=APP_TITLE />
                <body>
                    <App />
                </body>
            </html>
        }
    }

    /// A document a static host serves, which the client renders over once it starts.
    #[component]
    fn StaticDocument(options: LeptosOptions, title: &'static str, busy: bool, children: Children) -> impl IntoView {
        view! {
            <!DOCTYPE html>
            <html lang="en">
                <DocumentHead options title />
                <body>
                    <nav class="chrome">
                        <a href="/" class="chrome-brand" title="All courses">
                            <span class="chrome-brand-name">"Plainspoken"</span>
                        </a>
                    </nav>
                    <main id=STATIC_SHELL_ELEMENT_ID aria-busy=busy.then_some("true")>
                        {children()}
                    </main>
                </body>
            </html>
        }
    }

    /// The one document a static host serves at every route without a document of its own.
    pub fn static_shell(options: LeptosOptions) -> impl IntoView {
        view! {
            <StaticDocument options title=APP_TITLE busy=true>
                <p class="status" role="status">"Loading…"</p>
            </StaticDocument>
        }
    }

    pub fn static_privacy_document(options: LeptosOptions) -> impl IntoView {
        view! {
            <StaticDocument options title=PRIVACY_TITLE busy=false>
                <PrivacyPage />
            </StaticDocument>
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_course_store();
    provide_progress_store();
    provide_grading_store();
    provide_playback_store();
    provide_page_busy_store();

    let page_busy_store: PageBusyStore = use_page_busy_store();

    view! {
        <Router>
            <NavigationBar />
            <PlaybackAnnouncement />
            <main aria-busy=move || page_busy_store.is_busy().then_some("true")>
                <Routes fallback=|| view! { <p class="status">"Page not found."</p> }>
                    <Route path=StaticSegment("") view=CoursesPage />
                    <Route path=StaticSegment(PRIVACY_SEGMENT) view=PrivacyPage />
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

/// The title the course index gives the course; `None` before an index loads or for a course it does not list.
fn published_course_title(course_store: CourseStore, course_id: &str) -> Option<String> {
    course_store.index_entry(course_id).map(|index_entry| index_entry.title)
}

/// The voice settings the course's shown publication allows; every setting when no publication is shown.
fn offered_audio_engines(course_store: CourseStore, course_id: Option<String>) -> Vec<AudioEngineKind> {
    let publication_state: Option<LoadState<LoadedCoursePublication>> = course_id.and_then(|course_id| course_store.publication_state(&course_id));

    match publication_state {
        Some(LoadState::Loaded(publication)) => AudioEngineKind::choices_for_course(&publication.source.manifest.recorded_voice_engines()),
        Some(LoadState::Loading | LoadState::Failed(_)) | None => AudioEngineKind::ALL.to_vec(),
    }
}

fn audio_engine_option_value(engine: AudioEngineKind) -> &'static str {
    match engine {
        AudioEngineKind::Auto => "auto",
        AudioEngineKind::Kokoro => "kokoro",
        AudioEngineKind::MacosSay => "macos_say",
    }
}

fn audio_engine_option_label(engine: AudioEngineKind) -> &'static str {
    match engine {
        AudioEngineKind::Auto => "Auto",
        AudioEngineKind::Kokoro => "Neural (Kokoro)",
        AudioEngineKind::MacosSay => "macOS",
    }
}

fn audio_engine_of_option_value(value: &str) -> AudioEngineKind {
    AudioEngineKind::ALL
        .into_iter()
        .find(|engine| audio_engine_option_value(*engine) == value)
        .unwrap_or(AudioEngineKind::Auto)
}

#[component]
fn VoiceSetting(offered_engines: Vec<AudioEngineKind>) -> impl IntoView {
    let progress_store: ProgressStore = use_progress_store();
    let audio_engine = move || progress_store.read(|progress| progress.settings.audio_engine);
    let options: Vec<_> = offered_engines
        .into_iter()
        .map(|engine| {
            view! {
                <option value=audio_engine_option_value(engine) selected=move || audio_engine() == engine>
                    {audio_engine_option_label(engine)}
                </option>
            }
        })
        .collect();

    view! {
        <label class="field">
            <span class="field-label">"Voice"</span>
            <select on:change:target=move |event| {
                let engine: AudioEngineKind = audio_engine_of_option_value(&event.target().value());

                progress_store.update(|progress| progress.settings.audio_engine = engine);
            }>
                {options}
            </select>
        </label>
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

    let course_links = move || {
        let indexed_course_id: Option<String> = course_id().filter(|course_id| course_store.index_entry(course_id).is_some());

        indexed_course_id.map(|course_id| {
            let due: usize = progress_store.due_card_count(&course_id);

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
    let voice_setting = move || {
        let offered_engines: Vec<AudioEngineKind> = offered_audio_engines(course_store, course_id());

        (!offered_engines.is_empty()).then(|| view! { <VoiceSetting offered_engines /> })
    };

    view! {
        <nav class="chrome">
            <A href="/" attr:class="chrome-brand" attr:title="All courses" exact=true>
                <span class="chrome-brand-name">"Plainspoken"</span>
                {course_title}
            </A>
            {course_links}
            <div class="chrome-settings">
                {voice_setting}
                <label class="field">
                    <input
                        type="checkbox"
                        prop:checked=slow_audio
                        on:change=move |_| progress_store.update(|progress| progress.settings.slow_audio = !progress.settings.slow_audio)
                    />
                    <span class="field-label">"Slow audio"</span>
                </label>
                <GradingModelDownloadSetting />
            </div>
        </nav>
    }
}
