use std::collections::{BTreeMap, BTreeSet};
use std::mem;
use std::sync::Arc;

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use shared::AppError;
use shared::artifact::{ArtifactOriginKind, CourseIndexEntry, CourseIndexSource};
use shared::course::{CourseOutline, Unit, UnitOutline};
use shared::course_shard::{CourseShardContents, UnitShardContents};

use crate::course_data::{self, COURSE_ID_PARAM, LoadedCoursePublication};
use crate::platform;

#[derive(Debug, Clone)]
pub enum CourseIndexLoadState {
    Loading,
    Loaded(Arc<CourseIndexSource>),
    Failed(String),
}

/// How this page load's request for the repository's course index has ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryIndexLoadKind {
    Pending,
    Loaded,
    Failed,
}

/// How far a course's publication has loaded. Equality is by identity, so a memo over one course notifies only
/// when that course's load moves on.
#[derive(Debug, Clone)]
pub enum CoursePublicationLoadState {
    Loading,
    Loaded(Arc<LoadedCoursePublication>),
    Failed(String),
}

impl PartialEq for CoursePublicationLoadState {
    fn eq(&self, other: &CoursePublicationLoadState) -> bool {
        match (self, other) {
            (CoursePublicationLoadState::Loading, CoursePublicationLoadState::Loading) => true,
            (CoursePublicationLoadState::Loaded(left), CoursePublicationLoadState::Loaded(right)) => Arc::ptr_eq(left, right),
            (CoursePublicationLoadState::Failed(left), CoursePublicationLoadState::Failed(right)) => left == right,
            _ => false,
        }
    }
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

/// Whether a publication was shown for a course, and whether it serves other files than the one shown before.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationAdoptionKind {
    AdoptedWithOtherFiles,
    AdoptedWithTheSameFiles,
    Unavailable,
}

/// A unit by its course and its own id.
pub type UnitKey = (String, String);

#[derive(Clone, Copy)]
pub struct CourseStore {
    index: RwSignal<CourseIndexLoadState>,
    repository_index: RwSignal<RepositoryIndexLoadKind>,
    /// By course id; a course is present once its publication has been requested.
    publications: RwSignal<BTreeMap<String, CoursePublicationLoadState>>,
    /// Courses whose pointer is read once the repository index load ends.
    courses_awaiting_pointer: StoredValue<BTreeSet<String>>,
    outlines: RwSignal<BTreeMap<String, ShardLoadState<CourseOutline>>>,
    units: RwSignal<BTreeMap<UnitKey, ShardLoadState<Unit>>>,
}

impl CourseStore {
    pub fn index_state(&self) -> CourseIndexLoadState {
        self.index.get()
    }

    pub fn repository_index_state(&self) -> RepositoryIndexLoadKind {
        self.repository_index.get()
    }

    /// The shown course index's entry for the course; `None` before an index loads or for a course it does not list.
    pub fn index_entry(&self, course_id: &str) -> Option<CourseIndexEntry> {
        self.index.with(|index| match index {
            CourseIndexLoadState::Loaded(source) => source.course_index.courses.get(course_id).cloned(),
            CourseIndexLoadState::Loading | CourseIndexLoadState::Failed(_) => None,
        })
    }

    pub fn publication_state(&self, course_id: &str) -> Option<CoursePublicationLoadState> {
        self.publications.with(|publications| publications.get(course_id).cloned())
    }

    pub fn outline_state(&self, course_id: &str) -> Option<ShardLoadState<CourseOutline>> {
        self.outlines.with(|outlines| outlines.get(course_id).cloned())
    }

    pub fn unit_state(&self, course_id: &str, unit_id: &str) -> Option<ShardLoadState<Unit>> {
        self.units.with(|units| units.get(&(course_id.to_string(), unit_id.to_string())).cloned())
    }

    fn show_course_index(&self, source: CourseIndexSource) {
        log::info!(
            "showing a course index; [origin={:?} repository={} course_count={}]",
            source.origin,
            source.repository_base_url,
            source.course_index.courses.len(),
        );

        self.index.set(CourseIndexLoadState::Loaded(Arc::new(source)));
    }

    fn fail_course_index_unless_shown(&self, message: String) {
        let nothing_shown: bool = matches!(self.index.get_untracked(), CourseIndexLoadState::Loading);

        if nothing_shown {
            log::error!("could not load the course index; [error={message}]");
            self.index.set(CourseIndexLoadState::Failed(message));
        }
    }

    /// The shown course index when it was read from a repository, the only kind a course pointer is read through.
    fn shown_repository_index(&self) -> Option<Arc<CourseIndexSource>> {
        self.index.with_untracked(|index| match index {
            CourseIndexLoadState::Loaded(source) if source.origin == ArtifactOriginKind::Repository => Some(source.clone()),
            CourseIndexLoadState::Loaded(_) | CourseIndexLoadState::Loading | CourseIndexLoadState::Failed(_) => None,
        })
    }

    /// Replaces the course's shown publication; its outline and units load again unless it serves the same files.
    fn show_publication(&self, publication: LoadedCoursePublication) -> PublicationAdoptionKind {
        let course_id: String = publication.course_id().to_string();
        let previous: Option<Arc<LoadedCoursePublication>> = self.shown_publication(&course_id);
        let outcome: PublicationAdoptionKind = match previous {
            Some(previous) if previous.serves_the_same_files_as(&publication) => PublicationAdoptionKind::AdoptedWithTheSameFiles,
            Some(_) | None => PublicationAdoptionKind::AdoptedWithOtherFiles,
        };

        log::info!(
            "showing a course's publication; [course_id={course_id} publication_label={} repository={} outcome={outcome:?}]",
            publication.publication_label(),
            publication.source.repository_base_url,
        );

        if outcome == PublicationAdoptionKind::AdoptedWithOtherFiles {
            self.forget_loaded_shards(&course_id);
        }

        self.publications.update(|publications| {
            publications.insert(course_id, CoursePublicationLoadState::Loaded(Arc::new(publication)));
        });

        outcome
    }

    fn forget_loaded_shards(&self, course_id: &str) {
        self.outlines.update(|outlines| {
            outlines.remove(course_id);
        });
        self.units.update(|units| units.retain(|(unit_course_id, _), _| unit_course_id != course_id));
    }

    fn shown_publication(&self, course_id: &str) -> Option<Arc<LoadedCoursePublication>> {
        self.publications.with_untracked(|publications| match publications.get(course_id) {
            Some(CoursePublicationLoadState::Loaded(publication)) => Some(publication.clone()),
            Some(CoursePublicationLoadState::Loading | CoursePublicationLoadState::Failed(_)) | None => None,
        })
    }

    fn is_still_shown(&self, publication: &LoadedCoursePublication) -> bool {
        self.shown_publication(publication.course_id())
            .is_some_and(|shown| shown.serves_the_same_files_as(publication))
    }

    fn fail_publication_unless_shown(&self, course_id: &str, message: String) {
        self.publications.update(|publications| {
            let shown_or_absent: bool = !matches!(publications.get(course_id), Some(CoursePublicationLoadState::Loading));

            if shown_or_absent {
                return;
            }

            publications.insert(course_id.to_string(), CoursePublicationLoadState::Failed(message));
        });
    }

    /// Starts loading the course's publication unless this page load has requested it already.
    pub fn ensure_publication_loaded(&self, course_id: &str) {
        let already_requested: bool = self.publications.with_untracked(|publications| publications.contains_key(course_id));

        if already_requested {
            return;
        }

        self.publications.update(|publications| {
            publications.insert(course_id.to_string(), CoursePublicationLoadState::Loading);
        });

        let store: CourseStore = *self;
        let course_id: String = course_id.to_string();

        spawn_local(async move {
            store.show_local_publication(&course_id).await;
            store.read_pointer_once_the_repository_index_load_ends(course_id).await;
        });
    }

    /// Shows the cached publication, else the embedded one.
    async fn show_local_publication(&self, course_id: &str) {
        let cached: Option<LoadedCoursePublication> = course_data::load_cached_course_publication(course_id).await;

        if let Some(cached) = cached {
            self.show_publication(cached);
            return;
        }

        let embedded: Result<LoadedCoursePublication, AppError> = course_data::load_embedded_course_publication(course_id).await;

        match embedded {
            Ok(embedded) => {
                self.show_publication(embedded);
            },
            Err(error) => log::info!("no embedded publication of the course; [course_id={course_id} error={error}]"),
        }
    }

    async fn read_pointer_once_the_repository_index_load_ends(&self, course_id: String) {
        let repository_index: RepositoryIndexLoadKind = self.repository_index.get_untracked();

        match repository_index {
            RepositoryIndexLoadKind::Pending => self.courses_awaiting_pointer.update_value(|course_ids| {
                course_ids.insert(course_id);
            }),
            RepositoryIndexLoadKind::Loaded | RepositoryIndexLoadKind::Failed => {
                self.show_latest_publication(&course_id).await;
            },
        }
    }

    fn finish_repository_index_load(&self, repository_index: RepositoryIndexLoadKind) {
        self.repository_index.set(repository_index);

        let awaiting_course_ids: BTreeSet<String> = self.courses_awaiting_pointer.try_update_value(mem::take).unwrap_or_default();
        let store: CourseStore = *self;

        for course_id in awaiting_course_ids {
            spawn_local(async move {
                store.show_latest_publication(&course_id).await;
            });
        }
    }

    /// Reads the course's pointer through the shown repository index and shows the publication it names.
    async fn show_latest_publication(&self, course_id: &str) -> PublicationAdoptionKind {
        let Some(course_index) = self.shown_repository_index()
        else {
            self.fail_publication_unless_shown(course_id, "no course index from a repository could be read".to_string());
            return PublicationAdoptionKind::Unavailable;
        };

        let latest: Result<LoadedCoursePublication, AppError> = course_data::load_latest_course_publication(&course_index, course_id).await;

        match latest {
            Ok(latest) => self.show_publication(latest),
            Err(error) => {
                log::warn!("the course's latest publication is unavailable; [course_id={course_id} error={error}]");
                self.fail_publication_unless_shown(course_id, error.message.clone());
                PublicationAdoptionKind::Unavailable
            },
        }
    }

    /// After a shard of `publication` failed to load, shows the publication the course's pointer names now.
    async fn recover_from_shard_failure(&self, publication: &LoadedCoursePublication) -> PublicationAdoptionKind {
        if publication.source.origin != ArtifactOriginKind::Repository {
            return PublicationAdoptionKind::Unavailable;
        }

        log::info!("a shard failed to load, reading the course's pointer again; [course_id={}]", publication.course_id());

        self.show_latest_publication(publication.course_id()).await
    }

    /// Starts loading the course's outline from its shown publication unless it is loaded or loading.
    pub fn ensure_outline_loaded(&self, course_id: &str) {
        let Some(publication) = self.shown_publication(course_id)
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

        spawn_local(async move {
            let loaded: Result<CourseShardContents, AppError> = course_data::load_course_shard(&publication).await;

            store.finish_outline_load(&publication, loaded).await;
        });
    }

    async fn finish_outline_load(&self, publication: &LoadedCoursePublication, loaded: Result<CourseShardContents, AppError>) {
        if !self.is_still_shown(publication) {
            return;
        }

        let course_id: String = publication.course_id().to_string();
        let outline_state: ShardLoadState<CourseOutline> = match loaded {
            Ok(contents) => {
                platform::register_clip_audio(&publication.audio_base_url, &contents.clip_audio);
                ShardLoadState::Loaded(Arc::new(contents.outline))
            },
            Err(error) => {
                let recovery: PublicationAdoptionKind = self.recover_from_shard_failure(publication).await;

                if recovery == PublicationAdoptionKind::AdoptedWithOtherFiles || !self.is_still_shown(publication) {
                    return;
                }

                log::error!("could not load a course; [course_id={course_id} error={error}]");
                ShardLoadState::Failed(error.message.clone())
            },
        };

        self.outlines.update(|outlines| {
            outlines.insert(course_id, outline_state);
        });
    }

    /// Starts loading the unit's lesson from the course's shown publication unless it is loaded or loading.
    pub fn ensure_unit_loaded(&self, course_id: &str, unit_id: &str) {
        let Some(publication) = self.shown_publication(course_id)
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
            let loaded: Result<UnitShardContents, AppError> = course_data::load_unit_shard(&publication, &unit_key.1).await;

            store.finish_unit_load(&publication, unit_key, unit_outline, loaded).await;
        });
    }

    async fn finish_unit_load(
        &self,
        publication: &LoadedCoursePublication,
        unit_key: UnitKey,
        unit_outline: UnitOutline,
        loaded: Result<UnitShardContents, AppError>,
    ) {
        if !self.is_still_shown(publication) {
            return;
        }

        let unit_state: ShardLoadState<Unit> = match loaded {
            Ok(contents) => {
                platform::register_clip_audio(&publication.audio_base_url, &contents.clip_audio);
                ShardLoadState::Loaded(Arc::new(unit_outline.with_lesson(contents.lesson)))
            },
            Err(error) => {
                let recovery: PublicationAdoptionKind = self.recover_from_shard_failure(publication).await;

                if recovery == PublicationAdoptionKind::AdoptedWithOtherFiles || !self.is_still_shown(publication) {
                    return;
                }

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
        index: RwSignal::new(CourseIndexLoadState::Loading),
        repository_index: RwSignal::new(RepositoryIndexLoadKind::Pending),
        publications: RwSignal::new(BTreeMap::new()),
        courses_awaiting_pointer: StoredValue::new(BTreeSet::new()),
        outlines: RwSignal::new(BTreeMap::new()),
        units: RwSignal::new(BTreeMap::new()),
    };

    // Effects run only once the client has attached, so the server render stays at the loading state.
    Effect::new(move |_| spawn_local(load_course_index_into(store)));

    provide_context(store);
}

/// Shows the cached course index, else the embedded one, then the repository's when it loads.
async fn load_course_index_into(store: CourseStore) {
    course_data::forget_legacy_manifest();

    show_local_course_index(store).await;

    let latest: Result<CourseIndexSource, AppError> = course_data::load_latest_course_index().await;
    let repository_index: RepositoryIndexLoadKind = match latest {
        Ok(latest) => {
            store.show_course_index(latest);
            RepositoryIndexLoadKind::Loaded
        },
        Err(error) => {
            log::warn!("the repository's course index is unavailable; [error={error}]");
            store.fail_course_index_unless_shown(error.message.clone());
            RepositoryIndexLoadKind::Failed
        },
    };

    store.finish_repository_index_load(repository_index);
}

async fn show_local_course_index(store: CourseStore) {
    let cached: Option<CourseIndexSource> = course_data::load_cached_course_index().await;

    if let Some(cached) = cached {
        store.show_course_index(cached);
        return;
    }

    let embedded: Result<CourseIndexSource, AppError> = course_data::load_embedded_course_index().await;

    match embedded {
        Ok(embedded) => store.show_course_index(embedded),
        Err(error) => log::warn!("could not read the embedded course index; [error={error}]"),
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

fn failure_view(subject: &'static str, message: String) -> AnyView {
    view! {
        <p class="status error">
            {subject}" could not be loaded. Publish the course content with "<code>"./scripts/dev/build-local-repository.sh"</code>
            " first. ("{message}")"
        </p>
    }
    .into_any()
}

/// Renders `render` once a course index has loaded, and a status message before that.
pub fn course_index_gate<V>(render: impl Fn(Arc<CourseIndexSource>) -> V + Send + Sync + 'static) -> impl IntoView
where
    V: IntoView + 'static,
{
    let store: CourseStore = use_course_store();

    move || match store.index_state() {
        CourseIndexLoadState::Loading => status_view("Loading the courses…"),
        CourseIndexLoadState::Failed(message) => failure_view("The course list", message),
        CourseIndexLoadState::Loaded(course_index) => render(course_index).into_any(),
    }
}

/// Renders `render` once the course's outline has loaded, loading its publication and outline if needed, and a
/// status message before that.
pub fn course_gate<V>(
    course_id: impl Fn() -> String + Send + Sync + Copy + 'static,
    render: impl Fn(Arc<CourseOutline>) -> V + Send + Sync + 'static,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    let store: CourseStore = use_course_store();
    let listed_in_the_index: Memo<bool> = Memo::new(move |_| store.index_entry(&course_id()).is_some());
    let publication_state: Memo<Option<CoursePublicationLoadState>> = Memo::new(move |_| store.publication_state(&course_id()));
    let outline_state: Memo<Option<ShardLoadState<CourseOutline>>> = Memo::new(move |_| store.outline_state(&course_id()));

    Effect::new(move |_| {
        if !listed_in_the_index.get() {
            return;
        }

        store.ensure_publication_loaded(&course_id());

        let publication_loaded: bool = matches!(publication_state.get(), Some(CoursePublicationLoadState::Loaded(_)));

        if publication_loaded {
            store.ensure_outline_loaded(&course_id());
        }
    });

    move || {
        let repository_index_ended: bool = store.repository_index_state() != RepositoryIndexLoadKind::Pending;

        match (store.index_state(), publication_state.get(), outline_state.get()) {
            (CourseIndexLoadState::Failed(message), _, _) => failure_view("The course list", message),
            (CourseIndexLoadState::Loaded(_), _, _) if repository_index_ended && !listed_in_the_index.get() => {
                view! { <p class="status">"That course does not exist."</p> }.into_any()
            },
            (_, Some(CoursePublicationLoadState::Failed(message)), _) | (_, _, Some(ShardLoadState::Failed(message))) => {
                failure_view("The course content", message)
            },
            (_, _, Some(ShardLoadState::Loaded(outline))) => render(outline).into_any(),
            _ => status_view("Loading the course…"),
        }
    }
}
