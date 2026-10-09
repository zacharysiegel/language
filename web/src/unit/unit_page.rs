use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_params_map, use_query_map};
use shared::course::{CourseOutline, Dialogue, Exercise, GrammarNote, Reading, SpokenClip, Unit, VocabularyItem};
use shared::progress::AudioEngineKind;

use crate::course_data::{
    self, CourseStore, LoadState, TAB_QUERY_KEY, UNIT_ID_PARAM, course_gate, use_course_store, use_route_course_id,
};
use crate::pending_indication::PageLoadStatus;
use crate::progress_state::{ProgressStore, remember_route_course, use_progress_store};
use crate::recording_prefetch::{self, RecordingPrefetchId};
use crate::unit::{
    DialoguesSection, GrammarSection, OverviewSection, PracticeSection, ReadingsSection, UnitStatusControl, VocabularySection,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitTabKind {
    Overview,
    Vocabulary,
    Dialogues,
    Grammar,
    Readings,
    Practice,
}

impl UnitTabKind {
    const ALL: [UnitTabKind; 6] = [
        UnitTabKind::Overview,
        UnitTabKind::Grammar,
        UnitTabKind::Dialogues,
        UnitTabKind::Readings,
        UnitTabKind::Vocabulary,
        UnitTabKind::Practice,
    ];

    fn code(self) -> &'static str {
        match self {
            UnitTabKind::Overview => "overview",
            UnitTabKind::Vocabulary => "vocabulary",
            UnitTabKind::Dialogues => "dialogues",
            UnitTabKind::Grammar => "grammar",
            UnitTabKind::Readings => "readings",
            UnitTabKind::Practice => "practice",
        }
    }

    fn label(self) -> &'static str {
        match self {
            UnitTabKind::Overview => "Overview",
            UnitTabKind::Vocabulary => "Vocabulary",
            UnitTabKind::Dialogues => "Dialogues",
            UnitTabKind::Grammar => "Principles",
            UnitTabKind::Readings => "Reading",
            UnitTabKind::Practice => "Practice",
        }
    }

    fn available_for(self, unit: &Unit) -> bool {
        self != UnitTabKind::Readings || !unit.readings.is_empty()
    }

    fn available_tabs(unit: &Unit) -> Vec<UnitTabKind> {
        UnitTabKind::ALL.into_iter().filter(|tab| tab.available_for(unit)).collect()
    }

    fn spoken_clips(self, unit: &Unit) -> Vec<SpokenClip> {
        match self {
            UnitTabKind::Overview => Vec::new(),
            UnitTabKind::Vocabulary => unit.vocabulary.iter().flat_map(VocabularyItem::spoken_clips).collect(),
            UnitTabKind::Dialogues => unit.dialogues.iter().flat_map(Dialogue::spoken_clips).collect(),
            UnitTabKind::Grammar => unit.grammar.iter().flat_map(GrammarNote::spoken_clips).collect(),
            UnitTabKind::Readings => unit.readings.iter().flat_map(Reading::spoken_clips).collect(),
            UnitTabKind::Practice => unit.exercises.iter().flat_map(Exercise::spoken_clips).collect(),
        }
    }
}

impl TryFrom<&str> for UnitTabKind {
    type Error = ();

    fn try_from(code: &str) -> Result<UnitTabKind, ()> {
        UnitTabKind::ALL.into_iter().find(|tab| tab.code() == code).ok_or(())
    }
}

#[component]
pub fn UnitPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let course_store: CourseStore = use_course_store();
    let course_id: Memo<String> = use_route_course_id();
    let unit_id: Memo<String> = Memo::new(move |_| params.read().get(UNIT_ID_PARAM).unwrap_or_default());
    // A unit not yet requested is loading too, so the status shown does not restart when the request begins.
    let unit_state: Memo<LoadState<Unit>> = Memo::new(move |_| course_store.unit_state(&course_id.get(), &unit_id.get()).unwrap_or(LoadState::Loading));
    let active_tab: Memo<UnitTabKind> = Memo::new(move |_| {
        query
            .read()
            .get(TAB_QUERY_KEY)
            .and_then(|code| UnitTabKind::try_from(code.as_str()).ok())
            .unwrap_or(UnitTabKind::Overview)
    });

    let progress_store: ProgressStore = use_progress_store();
    let audio_engine: Memo<AudioEngineKind> = Memo::new(move |_| progress_store.read(|progress| progress.settings.audio_engine));

    remember_route_course(progress_store, course_id);
    prefetch_shown_tab_recordings(unit_state, active_tab, audio_engine);

    Effect::new(move |_| {
        let outline_loaded: bool = matches!(course_store.outline_state(&course_id.get()), Some(LoadState::Loaded(_)));

        if outline_loaded {
            course_store.ensure_unit_loaded(&course_id.get(), &unit_id.get());
        }
    });

    course_gate(move || course_id.get(), move |course: Arc<CourseOutline>| {
        if course.find_unit(&unit_id.get()).is_none() {
            return view! { <p class="status">"That unit does not exist."</p> }.into_any();
        }

        match unit_state.get() {
            LoadState::Loaded(unit) => unit_view(&course, (*unit).clone(), active_tab.get()).into_any(),
            LoadState::Failed(message) => view! { <p class="status error">"This unit could not be loaded. ("{message}")"</p> }.into_any(),
            LoadState::Loading => view! { <PageLoadStatus message="Loading the unit…" /> }.into_any(),
        }
    })
}

/// Downloads the recordings the shown tab can play, until another tab or unit is shown or the Voice setting changes.
fn prefetch_shown_tab_recordings(unit_state: Memo<LoadState<Unit>>, active_tab: Memo<UnitTabKind>, audio_engine: Memo<AudioEngineKind>) {
    Effect::new(move |_| {
        let LoadState::Loaded(unit) = unit_state.get()
        else {
            return;
        };

        let spoken_clips: Vec<SpokenClip> = active_tab.get().spoken_clips(&unit);
        let prefetch_id: RecordingPrefetchId = recording_prefetch::start_prefetch(spoken_clips, audio_engine.get());

        on_cleanup(move || recording_prefetch::cancel_prefetch(prefetch_id));
    });
}

fn unit_view(course: &CourseOutline, unit: Unit, active_tab: UnitTabKind) -> impl IntoView + use<> {
    let available_tabs: Vec<UnitTabKind> = UnitTabKind::available_tabs(&unit);
    let tab_links: Vec<_> = available_tabs
        .iter()
        .copied()
        .map(|tab| {
            view! {
                <A href=course_data::unit_tab_path(&course.id, &unit.id, tab.code()) attr:class=if tab == active_tab { "tab active" } else { "tab" }>
                    {tab.label()}
                </A>
            }
        })
        .collect();

    let tab_position: usize = available_tabs.iter().position(|tab| *tab == active_tab).unwrap_or(0);
    let previous_tab: Option<UnitTabKind> = tab_position.checked_sub(1).and_then(|position| available_tabs.get(position).copied());
    let next_tab: Option<UnitTabKind> = available_tabs.get(tab_position + 1).copied();
    let previous_link = previous_tab.map(|tab| tab_step_link(course, &unit, tab, format!("← {}", tab.label())));
    let next_link = next_tab.map(|tab| tab_step_link(course, &unit, tab, format!("{} →", tab.label())));

    let caption: String = format!("{} · {}", unit.level, unit.order);
    let title: String = unit.title.clone();
    let section = match active_tab {
        UnitTabKind::Overview => view! { <OverviewSection course_id=course.id.clone() unit=unit.clone() /> }.into_any(),
        UnitTabKind::Vocabulary => view! { <VocabularySection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Dialogues => view! { <DialoguesSection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Grammar => view! { <GrammarSection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Readings => view! { <ReadingsSection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Practice => view! { <PracticeSection course_id=course.id.clone() unit=unit.clone() /> }.into_any(),
    };

    view! {
        <div class="lesson-header">
            <span class="page-caption">{caption}</span>
            <h1>{title}</h1>
            <UnitStatusControl course_id=course.id.clone() unit_id=unit.id.clone() />
        </div>
        <div class="tabs">{tab_links}</div>
        {section}
        <div class="unit-navigation">
            <span>{previous_link}</span>
            <span>{next_link}</span>
        </div>
    }
}

fn tab_step_link(course: &CourseOutline, unit: &Unit, tab: UnitTabKind, label: String) -> impl IntoView + use<> {
    view! { <A href=course_data::unit_tab_path(&course.id, &unit.id, tab.code())>{label}</A> }
}
