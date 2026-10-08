use serde::{Deserialize, Serialize};

use crate::course::CourseLevel;

/// A course's plan: its identity and languages, its levels, and the units each level holds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Curriculum {
    pub id: String,
    pub title: String,
    pub target_language: String,
    pub source_language: String,
    pub levels: Vec<CourseLevel>,
    pub units: Vec<CurriculumUnit>,
    /// A course that exists to test the apps, not to teach.
    #[serde(default)]
    pub test_course: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CurriculumUnit {
    pub id: String,
    pub level: String,
    pub order: u32,
    pub title: String,
    pub goals: Vec<String>,
    pub grammar: Vec<String>,
    pub vocabulary_themes: Vec<String>,
}
