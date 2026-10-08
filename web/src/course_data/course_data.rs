use std::collections::BTreeMap;
use std::sync::Arc;

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use shared::AppError;
use shared::course::{CourseOutline, Unit, UnitOutline};
use shared::course_shard::{CourseShardContents, UnitShardContents};

use crate::course_data::{self, COURSE_ID_PARAM, LoadedVersion};
use crate::platform;

#[derive(Debug, Clone)]
pub enum VersionLoadState {
    Loading,
    Loaded(Arc<LoadedVersion>),
    Failed(String),
}

/// How far a course outline or a unit lesson has loaded. Equality is by identity, so a memo over one entry
/// notifies only when that entry's load moves on.
#[derive(Debug, Clone)]
pub enum ShardLoadState<T> {
    Loading,
    Loaded(Arc<T>),
    Failed(String),
}

impl<T> PartialEq for ShardLoadState<T> {
    fn eq(&self, other: &ShardLoadState<T>) -> bool {
        match (self, other) {
            (ShardLoadState::Loading, ShardLoadState::Loading) => true,
            (ShardLoadState::Loaded(left), ShardLoadState::Loaded(right)) => Arc::ptr_eq(left, right),
            (ShardLoadState::Failed(left), ShardLoadState::Failed(right)) => left == right,
            _ => false,
        }
    }
}

/// A unit by its course and its own id.
pub type UnitKey = (String, String);

#[derive(Clone, Copy)]
pub struct CourseStore {
    state: RwSignal<VersionLoadState>,
    outlines: RwSignal<BTreeMap<String, ShardLoadState<CourseOutline>>>,
    units: RwSignal<BTreeMap<UnitKey, ShardLoadState<Unit>>>,
}

impl CourseStore {
    pub fn state(&self) -> VersionLoadState {
        self.state.get()
    }

    pub fn outline_state(&self, course_id: &str) -> Option<ShardLoadState<CourseOutline>> {
        self.outlines.with(|outlines| outlines.get(course_id).cloned())
    }

    pub fn unit_state(&self, course_id: &str, unit_id: &str) -> Option<ShardLoadState<Unit>> {
        self.units.with(|units| units.get(&(course_id.to_string(), unit_id.to_string())).cloned())
    }

    /// Replaces the shown version; outlines and units of the previous version are loaded again from the new one.
    fn show(&self, loaded: LoadedVersion) {
        log::info!("showing a version; [version={} repository={}]", loaded.version(), loaded.source.repository_base_url);

        platform::reset_clip_audio(&loaded.audio_base_url, &[]);
        self.outlines.set(BTreeMap::new());
        self.units.set(BTreeMap::new());
        self.state.set(VersionLoadState::Loaded(Arc::new(loaded)));
    }

    fn shown_version(&self) -> Option<Arc<LoadedVersion>> {
        self.state.with_untracked(|state| match state {
            VersionLoadState::Loaded(loaded) => Some(loaded.clone()),
            VersionLoadState::Loading | VersionLoadState::Failed(_) => None,
        })
    }

    fn is_still_shown(&self, version: &LoadedVersion) -> bool {
        self.shown_version().is_some_and(|shown| shown.version() == version.version())
    }

    /// Starts loading the course's outline from the shown version unless it is loaded or loading.
    pub fn ensure_outline_loaded(&self, course_id: &str) {
        let Some(version) = self.shown_version()
        else {
            return;
        };
        let already_requested: bool = self.outlines.with_untracked(|outlines| outlines.contains_key(course_id));

        if already_requested {
            return;
        }

        self.outlines.update(|outlines| {
            outlines.insert(course_id.to_string(), ShardLoadState::Loading);
        });

        let store: CourseStore = *self;
        let course_id: String = course_id.to_string();

        spawn_local(async move {
            let loaded: Result<CourseShardContents, AppError> = course_data::load_course_shard(&version, &course_id).await;

            if !store.is_still_shown(&version) {
                return;
            }

            let outline_state: ShardLoadState<CourseOutline> = match loaded {
                Ok(contents) => {
                    platform::register_clip_audio(&contents.clip_audio);
                    ShardLoadState::Loaded(Arc::new(contents.outline))
                },
                Err(error) => {
                    log::error!("could not load a course; [course_id={course_id} error={error}]");
                    ShardLoadState::Failed(error.message.clone())
                },
            };

            store.outlines.update(|outlines| {
                outlines.insert(course_id, outline_state);
            });
        });
    }

    /// Starts loading the unit's lesson from the shown version unless it is loaded or loading.
    pub fn ensure_unit_loaded(&self, course_id: &str, unit_id: &str) {
        let Some(version) = self.shown_version()
        else {
            return;
        };
        let Some(ShardLoadState::Loaded(outline)) = self.outlines.with_untracked(|outlines| outlines.get(course_id).cloned())
        else {
            return;
        };
        let unit_key: UnitKey = (course_id.to_string(), unit_id.to_string());
        let already_requested: bool = self.units.with_untracked(|units| units.contains_key(&unit_key));

        if already_requested {
            return;
        }

        let Some(unit_outline) = outline.find_unit(unit_id).cloned()
        else {
            return;
        };

        self.units.update(|units| {
            units.insert(unit_key.clone(), ShardLoadState::Loading);
        });

        let store: CourseStore = *self;

        spawn_local(async move {
            let loaded: Result<UnitShardContents, AppError> = course_data::load_unit_shard(&version, &unit_key.0, &unit_key.1).await;

            if store.is_still_shown(&version) {
                store.finish_unit_load(unit_key, unit_outline, loaded);
            }
        });
    }

    fn finish_unit_load(&self, unit_key: UnitKey, unit_outline: UnitOutline, loaded: Result<UnitShardContents, AppError>) {
        let unit_state: ShardLoadState<Unit> = match loaded {
            Ok(contents) => {
                platform::register_clip_audio(&contents.clip_audio);
                ShardLoadState::Loaded(Arc::new(unit_outline.with_lesson(contents.lesson)))
            },
            Err(error) => {
                log::error!("could not load a unit; [course_id={} unit_id={} error={error}]", unit_key.0, unit_key.1);
                ShardLoadState::Failed(error.message.clone())
            },
        };

        self.units.update(|units| {
            units.insert(unit_key, unit_state);
        });
    }
}

pub fn provide_course_store() {
    let store: CourseStore = CourseStore {
        state: RwSignal::new(VersionLoadState::Loading),
        outlines: RwSignal::new(BTreeMap::new()),
        units: RwSignal::new(BTreeMap::new()),
    };

    // Effects run only once the client has attached, so the server render stays at the loading state.
    Effect::new(move |_| spawn_local(load_version_into(store)));

    provide_context(store);
}

/// Shows the cached version at once when there is one, then the latest version if it differs. The embedded
/// bundle is the floor when nothing else can be read.
async fn load_version_into(store: CourseStore) {
    let cached: Option<LoadedVersion> = course_data::load_cached_version().await;
    let cached_version: Option<String> = cached.as_ref().map(|loaded| loaded.version().to_string());

    if let Some(cached) = cached {
        store.show(cached);
    }

    let latest: Result<LoadedVersion, AppError> = course_data::load_latest_version().await;

    match latest {
        Ok(latest) if cached_version.as_deref() != Some(latest.version()) => store.show(latest),
        Ok(_) => {},
        Err(error) if cached_version.is_some() => log::warn!("the latest version is unavailable; [error={error}]"),
        Err(error) => {
            log::warn!("the latest version is unavailable, opening the embedded one; [error={error}]");
            show_embedded_version(store).await;
        },
    }
}

async fn show_embedded_version(store: CourseStore) {
    let embedded: Result<LoadedVersion, AppError> = course_data::load_embedded_version().await;

    match embedded {
        Ok(embedded) => store.show(embedded),
        Err(error) => {
            log::error!("could not load the courses; [error={error}]");
            store.state.set(VersionLoadState::Failed(error.message.clone()));
        },
    }
}

pub fn use_course_store() -> CourseStore {
    expect_context::<CourseStore>()
}

/// The id of the course the current route is under.
pub fn use_route_course_id() -> Memo<String> {
    let params = use_params_map();

    Memo::new(move |_| params.read().get(COURSE_ID_PARAM).unwrap_or_default())
}

fn status_view(message: &'static str) -> AnyView {
    view! { <p class="status">{message}</p> }.into_any()
}

fn failure_view(message: String) -> AnyView {
    view! {
        <p class="status error">
            "The course could not be loaded. Run "<code>"./scripts/build/build-course.sh"</code>" and publish it first. ("{message}")"
        </p>
    }
    .into_any()
}

/// Renders `render` once a version of the courses has loaded, and a status message before that.
pub fn version_gate<V>(render: impl Fn(Arc<LoadedVersion>) -> V + Send + Sync + 'static) -> impl IntoView
where
    V: IntoView + 'static,
{
    let store: CourseStore = use_course_store();

    move || match store.state() {
        VersionLoadState::Loading => status_view("Loading the courses…"),
        VersionLoadState::Failed(message) => failure_view(message),
        VersionLoadState::Loaded(version) => render(version).into_any(),
    }
}

/// Renders `render` once the course's outline has loaded, loading it if needed, and a status message before that.
pub fn course_gate<V>(
    course_id: impl Fn() -> String + Send + Sync + Copy + 'static,
    render: impl Fn(Arc<CourseOutline>) -> V + Send + Sync + 'static,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    let store: CourseStore = use_course_store();
    let outline_state: Memo<Option<ShardLoadState<CourseOutline>>> = Memo::new(move |_| store.outline_state(&course_id()));

    Effect::new(move |_| {
        let version_loaded: bool = matches!(store.state(), VersionLoadState::Loaded(_));

        if version_loaded {
            store.ensure_outline_loaded(&course_id());
        }
    });

    move || match (store.state(), outline_state.get()) {
        (VersionLoadState::Failed(message), _) | (_, Some(ShardLoadState::Failed(message))) => failure_view(message),
        (VersionLoadState::Loaded(version), _) if version.source.manifest.courses.get(&course_id()).is_none() => {
            view! { <p class="status">"That course does not exist."</p> }.into_any()
        },
        (_, Some(ShardLoadState::Loaded(outline))) => render(outline).into_any(),
        _ => status_view("Loading the course…"),
    }
}
