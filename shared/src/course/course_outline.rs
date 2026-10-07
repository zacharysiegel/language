use serde::{Deserialize, Serialize};

use crate::course::{
    Course, CultureNote, Dialogue, Exercise, GrammarNote, LevelKind, LevelSummary, Reading, Unit, VocabularyItem,
    parse_vocabulary_card_id,
};

/// The course without its lessons: what the learner browses, reviews and searches before opening a unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseOutline {
    pub title: String,
    pub levels: Vec<LevelSummary>,
    pub units: Vec<UnitOutline>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitOutline {
    pub id: String,
    pub level: LevelKind,
    pub order: u32,
    pub title: String,
    pub objectives: Vec<String>,
    pub vocabulary: Vec<VocabularyItem>,
}

/// Everything in a unit other than its outline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitLesson {
    pub dialogues: Vec<Dialogue>,
    pub grammar: Vec<GrammarNote>,
    pub culture: CultureNote,
    pub readings: Vec<Reading>,
    pub exercises: Vec<Exercise>,
}

impl CourseOutline {
    pub fn find_unit(&self, unit_id: &str) -> Option<&UnitOutline> {
        self.units.iter().find(|unit| unit.id == unit_id)
    }

    pub fn find_vocabulary(&self, card_id: &str) -> Option<(&UnitOutline, &VocabularyItem)> {
        let (unit_id, item_index): (&str, usize) = parse_vocabulary_card_id(card_id)?;
        let unit: &UnitOutline = self.find_unit(unit_id)?;
        let item: &VocabularyItem = unit.vocabulary.get(item_index)?;

        Some((unit, item))
    }

    pub fn units_at_level(&self, level: LevelKind) -> impl Iterator<Item = &UnitOutline> {
        self.units.iter().filter(move |unit| unit.level == level)
    }
}

impl UnitOutline {
    pub fn with_lesson(&self, lesson: UnitLesson) -> Unit {
        Unit {
            id: self.id.clone(),
            level: self.level,
            order: self.order,
            title: self.title.clone(),
            objectives: self.objectives.clone(),
            vocabulary: self.vocabulary.clone(),
            dialogues: lesson.dialogues,
            grammar: lesson.grammar,
            culture: lesson.culture,
            readings: lesson.readings,
            exercises: lesson.exercises,
        }
    }
}

impl Unit {
    pub fn outline(&self) -> UnitOutline {
        UnitOutline {
            id: self.id.clone(),
            level: self.level,
            order: self.order,
            title: self.title.clone(),
            objectives: self.objectives.clone(),
            vocabulary: self.vocabulary.clone(),
        }
    }

    pub fn lesson(&self) -> UnitLesson {
        UnitLesson {
            dialogues: self.dialogues.clone(),
            grammar: self.grammar.clone(),
            culture: self.culture.clone(),
            readings: self.readings.clone(),
            exercises: self.exercises.clone(),
        }
    }
}

impl Course {
    pub fn outline(&self) -> CourseOutline {
        CourseOutline {
            title: self.title.clone(),
            levels: self.levels.clone(),
            units: self.units.iter().map(Unit::outline).collect(),
        }
    }
}
