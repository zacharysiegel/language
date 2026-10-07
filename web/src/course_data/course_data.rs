use std::collections::BTreeMap;
use std::sync::Arc;

use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::AppError;
use shared::course::{CourseOutline, Unit, UnitOutline};
use shared::course_shard::UnitShardContents;

use crate::course_data::{self, LoadedCourse};
use crate::platform;

#[derive(Debug, Clone)]
pub enum CourseLoadState {
    Loading,
    Loaded(Arc<LoadedCourse>),
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum UnitLoadState {
    Loading,
    Loaded(Arc<Unit>),
    Failed(String),
}

/* Identity, not contents, so a memo over a unit's state notifies only when that unit's load moves on. */
impl PartialEq for UnitLoadState {
    fn eq(&self, other: &UnitLoadState) -> bool {
        match (self, other) {
            (UnitLoadState::Loading, UnitLoadState::Loading) => true,
            (UnitLoadState::Loaded(left), UnitLoadState::Loaded(right)) => Arc::ptr_eq(left, right),
            (UnitLoadState::Failed(left), UnitLoadState::Failed(right)) => left == right,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct CourseStore {
    state: RwSignal<CourseLoadState>,
    units: RwSignal<BTreeMap<String, UnitLoadState>>,
}

impl CourseStore {
    pub fn state(&self) -> CourseLoadState {
        self.state.get()
    }

    pub fn unit_state(&self, unit_id: &str) -> Option<UnitLoadState> {
        self.units.with(|units| units.get(unit_id).cloned())
    }

    /// Replaces the shown version; units of the previous version are loaded again from the new one.
    fn show(&self, loaded: LoadedCourse) {
        log::info!("showing a course version; [version={} repository={}]", loaded.version(), loaded.source.repository_base_url);

        platform::reset_clip_audio(&loaded.audio_base_url, &loaded.clip_audio);
        self.units.set(BTreeMap::new());
        self.state.set(CourseLoadState::Loaded(Arc::new(loaded)));
    }

    fn shown_version(&self) -> Option<String> {
        self.state.with_untracked(|state| match state {
            CourseLoadState::Loaded(loaded) => Some(loaded.version().to_string()),
            CourseLoadState::Loading | CourseLoadState::Failed(_) => None,
        })
    }

    /// Starts loading the unit's lesson from the shown version unless it is loaded or loading.
    pub fn ensure_unit_loaded(&self, unit_id: &str) {
        let CourseLoadState::Loaded(course) = self.state.get_untracked()
        else {
            return;
        };
        let already_requested: bool = self.units.with_untracked(|units| units.contains_key(unit_id));

        if already_requested {
            return;
        }

        let Some(unit_outline) = course.outline.find_unit(unit_id).cloned()
        else {
            return;
        };

        self.units.update(|units| {
            units.insert(unit_id.to_string(), UnitLoadState::Loading);
        });

        let store: CourseStore = *self;

        spawn_local(async move {
            let loaded: Result<UnitShardContents, AppError> = course_data::load_unit_shard(&course, &unit_outline.id).await;

            store.finish_unit_load(&course, unit_outline, loaded);
        });
    }

    fn finish_unit_load(&self, course: &LoadedCourse, unit_outline: UnitOutline, loaded: Result<UnitShardContents, AppError>) {
        let still_shown: bool = self.shown_version().as_deref() == Some(course.version());

        if !still_shown {
            return;
        }

        let unit_state: UnitLoadState = match loaded {
            Ok(contents) => {
                platform::register_clip_audio(&contents.clip_audio);

                UnitLoadState::Loaded(Arc::new(unit_outline.with_lesson(contents.lesson)))
            },
            Err(error) => {
                log::error!("could not load a unit; [unit_id={} error={error}]", unit_outline.id);

                UnitLoadState::Failed(error.message.clone())
            },
        };

        self.units.update(|units| {
            units.insert(unit_outline.id.clone(), unit_state);
        });
    }
}

pub fn provide_course_store() {
    let store: CourseStore = CourseStore {
        state: RwSignal::new(CourseLoadState::Loading),
        units: RwSignal::new(BTreeMap::new()),
    };

    // Effects run only once the client has attached, so the server render stays at the loading state.
    Effect::new(move |_| spawn_local(load_course_into(store)));

    provide_context(store);
}

/// Shows the cached version at once when there is one, then the latest version if it differs. The embedded
/// bundle is the floor when nothing else can be read.
async fn load_course_into(store: CourseStore) {
    let cached: Option<LoadedCourse> = course_data::load_cached_course().await;
    let cached_version: Option<String> = cached.as_ref().map(|loaded| loaded.version().to_string());

    if let Some(cached) = cached {
        store.show(cached);
    }

    let latest: Result<LoadedCourse, AppError> = course_data::load_latest_course().await;

    match latest {
        Ok(latest) if cached_version.as_deref() != Some(latest.version()) => store.show(latest),
        Ok(_) => {},
        Err(error) if cached_version.is_some() => log::warn!("the latest course is unavailable; [error={error}]"),
        Err(error) => {
            log::warn!("the latest course is unavailable, opening the embedded one; [error={error}]");
            show_embedded_course(store).await;
        },
    }
}

async fn show_embedded_course(store: CourseStore) {
    let embedded: Result<LoadedCourse, AppError> = course_data::load_embedded_course().await;

    match embedded {
        Ok(embedded) => store.show(embedded),
        Err(error) => {
            log::error!("could not load the course; [error={error}]");
            store.state.set(CourseLoadState::Failed(error.message.clone()));
        },
    }
}

pub fn use_course_store() -> CourseStore {
    expect_context::<CourseStore>()
}

/// Renders `render` once the course outline has loaded, and a status message before that.
pub fn course_gate<V>(render: impl Fn(Arc<CourseOutline>) -> V + Send + Sync + 'static) -> impl IntoView
where
    V: IntoView + 'static,
{
    let store: CourseStore = use_course_store();

    move || match store.state() {
        CourseLoadState::Loading => view! { <p class="status">"Loading the course…"</p> }.into_any(),
        CourseLoadState::Failed(message) => view! {
            <p class="status error">
                "The course could not be loaded. Run "<code>"./scripts/build/build-course.sh"</code>" and publish it first. ("{message}")"
            </p>
        }
        .into_any(),
        CourseLoadState::Loaded(course) => render(course.outline.clone()).into_any(),
    }
}
