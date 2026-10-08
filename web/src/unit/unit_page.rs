use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_params_map, use_query_map};
use shared::course::{CourseOutline, Unit};

use crate::course_data::{
    self, CourseStore, ShardLoadState, TAB_QUERY_KEY, UNIT_ID_PARAM, course_gate, use_course_store, use_route_course_id,
};
use crate::progress_state::{remember_route_course, use_progress_store};
use crate::unit::{
    DialoguesSection, GrammarSection, OverviewSection, PracticeSection, ReadingsSection, VocabularySection,
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
    let unit_state: Memo<Option<ShardLoadState<Unit>>> = Memo::new(move |_| course_store.unit_state(&course_id.get(), &unit_id.get()));

    remember_route_course(use_progress_store(), course_id);

    Effect::new(move |_| {
        let outline_loaded: bool = matches!(course_store.outline_state(&course_id.get()), Some(ShardLoadState::Loaded(_)));

        if outline_loaded {
            course_store.ensure_unit_loaded(&course_id.get(), &unit_id.get());
        }
    });

    course_gate(move || course_id.get(), move |course: Arc<CourseOutline>| {
        let active_tab: UnitTabKind = query
            .read()
            .get(TAB_QUERY_KEY)
            .and_then(|code| UnitTabKind::try_from(code.as_str()).ok())
            .unwrap_or(UnitTabKind::Overview);

        if course.find_unit(&unit_id.get()).is_none() {
            return view! { <p class="status">"That unit does not exist."</p> }.into_any();
        }

        match unit_state.get() {
            Some(ShardLoadState::Loaded(unit)) => unit_view(&course, (*unit).clone(), active_tab).into_any(),
            Some(ShardLoadState::Failed(message)) => {
                view! { <p class="status error">"This unit could not be loaded. ("{message}")"</p> }.into_any()
            },
            Some(ShardLoadState::Loading) | None => view! { <p class="status">"Loading the unit…"</p> }.into_any(),
        }
    })
}

fn unit_view(course: &CourseOutline, unit: Unit, active_tab: UnitTabKind) -> impl IntoView + use<> {
    let tab_links: Vec<_> = UnitTabKind::ALL
        .into_iter()
        .filter(|tab| tab.available_for(&unit))
        .map(|tab| {
            view! {
                <A href=course_data::unit_tab_path(&course.id, &unit.id, tab.code()) attr:class=if tab == active_tab { "tab active" } else { "tab" }>
                    {tab.label()}
                </A>
            }
        })
        .collect();

    let unit_position: usize = course.units.iter().position(|candidate| candidate.id == unit.id).unwrap_or(0);
    let previous_target: Option<(String, String)> = unit_position
        .checked_sub(1)
        .and_then(|position| course.units.get(position))
        .map(|previous| (course_data::unit_path(&course.id, &previous.id), format!("← {}", previous.title)));
    let next_target: Option<(String, String)> = course
        .units
        .get(unit_position + 1)
        .map(|next| (course_data::unit_path(&course.id, &next.id), format!("{} →", next.title)));
    let previous_link = previous_target.map(|(href, label)| view! { <A href=href>{label}</A> });
    let next_link = next_target.map(|(href, label)| view! { <A href=href>{label}</A> });

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
        </div>
        <div class="tabs">{tab_links}</div>
        {section}
        <div class="unit-navigation">
            <span>{previous_link}</span>
            <span>{next_link}</span>
        </div>
    }
}
