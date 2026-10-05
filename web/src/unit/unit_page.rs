use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_params_map, use_query_map};
use shared::course::{Course, Unit};

use crate::course_data::course_gate;
use crate::unit::{
    DialoguesSection, GrammarSection, OverviewSection, PracticeSection, ReadingsSection, VocabularySection,
};

const TAB_QUERY_KEY: &str = "tab";
const UNIT_ID_PARAM: &str = "unit_id";

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

    course_gate(move |course: Arc<Course>| {
        let unit_id: String = params.read().get(UNIT_ID_PARAM).unwrap_or_default();
        let active_tab: UnitTabKind = query
            .read()
            .get(TAB_QUERY_KEY)
            .and_then(|code| UnitTabKind::try_from(code.as_str()).ok())
            .unwrap_or(UnitTabKind::Overview);

        match course.find_unit(&unit_id) {
            Some(unit) => unit_view(&course, unit.clone(), active_tab).into_any(),
            None => view! { <p class="status">"That unit does not exist."</p> }.into_any(),
        }
    })
}

fn unit_view(course: &Course, unit: Unit, active_tab: UnitTabKind) -> impl IntoView + use<> {
    let tab_links: Vec<_> = UnitTabKind::ALL
        .into_iter()
        .filter(|tab| tab.available_for(&unit))
        .map(|tab| {
            view! {
                <A href=format!("/unit/{}?{TAB_QUERY_KEY}={}", unit.id, tab.code()) attr:class=if tab == active_tab { "tab active" } else { "tab" }>
                    {tab.label()}
                </A>
            }
        })
        .collect();

    let unit_position: usize = course.units.iter().position(|candidate| candidate.id == unit.id).unwrap_or(0);
    let previous_target: Option<(String, String)> = unit_position
        .checked_sub(1)
        .and_then(|position| course.units.get(position))
        .map(|previous| (format!("/unit/{}", previous.id), format!("← {}", previous.title)));
    let next_target: Option<(String, String)> = course
        .units
        .get(unit_position + 1)
        .map(|next| (format!("/unit/{}", next.id), format!("{} →", next.title)));
    let previous_link = previous_target.map(|(href, label)| view! { <A href=href>{label}</A> });
    let next_link = next_target.map(|(href, label)| view! { <A href=href>{label}</A> });

    let heading: String = format!("{} {}: {}", unit.level.code(), unit.order, unit.title);
    let section = match active_tab {
        UnitTabKind::Overview => view! { <OverviewSection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Vocabulary => view! { <VocabularySection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Dialogues => view! { <DialoguesSection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Grammar => view! { <GrammarSection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Readings => view! { <ReadingsSection unit=unit.clone() /> }.into_any(),
        UnitTabKind::Practice => view! { <PracticeSection unit=unit.clone() /> }.into_any(),
    };

    view! {
        <h1>{heading}</h1>
        <div class="tabs">{tab_links}</div>
        {section}
        <div class="unit-navigation">
            <span>{previous_link}</span>
            <span>{next_link}</span>
        </div>
    }
}
