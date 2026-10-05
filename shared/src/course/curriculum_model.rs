use serde::{Deserialize, Serialize};

use crate::course::{LevelKind, LevelSummary};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Curriculum {
    pub title: String,
    pub levels: Vec<LevelSummary>,
    pub units: Vec<CurriculumUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CurriculumUnit {
    pub id: String,
    pub level: LevelKind,
    pub order: u32,
    pub title: String,
    pub goals: Vec<String>,
    pub grammar: Vec<String>,
    pub vocabulary_themes: Vec<String>,
}
