use std::sync::Arc;

use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::AppError;
use shared::course::Course;

use crate::platform;

const COURSE_URL: &str = "/course/course.json";

#[derive(Debug, Clone)]
pub enum CourseLoadState {
    Loading,
    Loaded(Arc<Course>),
    Failed(String),
}

#[derive(Clone, Copy)]
pub struct CourseStore {
    state: RwSignal<CourseLoadState>,
}

impl CourseStore {
    pub fn state(&self) -> CourseLoadState {
        self.state.get()
    }
}

pub fn provide_course_store() {
    let state: RwSignal<CourseLoadState> = RwSignal::new(CourseLoadState::Loading);

    // Effects run only once the client has attached, so the server render stays at the loading state.
    Effect::new(move |_| {
        spawn_local(async move {
            let loaded: Result<Course, AppError> = load_course().await;

            match loaded {
                Ok(course) => state.set(CourseLoadState::Loaded(Arc::new(course))),
                Err(error) => {
                    log::error!("could not load the course; [error={error}]");
                    state.set(CourseLoadState::Failed(error.message.clone()));
                },
            }
        });
    });

    provide_context(CourseStore { state });
}

async fn load_course() -> Result<Course, AppError> {
    let course_text: String = platform::fetch_text(COURSE_URL).await?;
    let course: Course = serde_json::from_str(&course_text)?;

    Ok(course)
}

pub fn use_course_store() -> CourseStore {
    expect_context::<CourseStore>()
}

/// Renders `render` once the course has loaded, and a status message before that.
pub fn course_gate<V>(render: impl Fn(Arc<Course>) -> V + Send + Sync + 'static) -> impl IntoView
where
    V: IntoView + 'static,
{
    let store: CourseStore = use_course_store();

    move || match store.state() {
        CourseLoadState::Loading => view! { <p class="status">"Loading the course…"</p> }.into_any(),
        CourseLoadState::Failed(message) => view! {
            <p class="status error">
                "The course could not be loaded. Run "<code>"cargo run -p course_builder -- build"</code>" first. ("{message}")"
            </p>
        }
        .into_any(),
        CourseLoadState::Loaded(course) => render(course).into_any(),
    }
}
