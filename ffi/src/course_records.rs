//! The course model as records the native apps receive; each converts from its `shared::course` counterpart.

use shared::artifact::CourseIndexEntry;
use shared::course::{
    self, CourseLevel, CourseOutline, CultureNote, Dialogue, DialogueLine, DictationExercise, Exercise, ExerciseKind,
    FillBlankExercise, GrammarNote, GrammarTable, ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise,
    OrderWordsExercise, PhrasePair, Reading, SpeakerGenderKind, TranslateToTargetExercise, UnitLesson, UnitOutline,
    VocabularyItem,
};

#[derive(Debug, Clone, uniffi::Record)]
pub struct CourseLevelRecord {
    pub code: String,
    pub title: String,
    pub requires_readings: bool,
}

impl From<&CourseLevel> for CourseLevelRecord {
    fn from(level: &CourseLevel) -> CourseLevelRecord {
        CourseLevelRecord {
            code: level.code.clone(),
            title: level.title.clone(),
            requires_readings: level.requires_readings,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct PhrasePairRecord {
    pub target: String,
    pub source: String,
    pub source_literal: Option<String>,
    pub clip_key: Option<String>,
}

impl From<&PhrasePair> for PhrasePairRecord {
    fn from(pair: &PhrasePair) -> PhrasePairRecord {
        PhrasePairRecord {
            target: pair.target.clone(),
            source: pair.source.clone(),
            source_literal: pair.source_literal.clone(),
            clip_key: pair.clip_key.clone(),
        }
    }
}

impl From<&PhrasePairRecord> for PhrasePair {
    fn from(record: &PhrasePairRecord) -> PhrasePair {
        PhrasePair {
            target: record.target.clone(),
            source: record.source.clone(),
            source_literal: record.source_literal.clone(),
            clip_key: record.clip_key.clone(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct VocabularyItemRecord {
    pub card_id: String,
    pub target: String,
    pub source: String,
    pub source_literal: Option<String>,
    pub part_of_speech: Option<String>,
    pub note: Option<String>,
    pub clip_key: Option<String>,
    pub example: PhrasePairRecord,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UnitOutlineRecord {
    pub id: String,
    pub level_code: String,
    pub order: u32,
    pub title: String,
    pub objectives: Vec<String>,
    pub vocabulary: Vec<VocabularyItemRecord>,
}

impl From<&UnitOutline> for UnitOutlineRecord {
    fn from(unit: &UnitOutline) -> UnitOutlineRecord {
        UnitOutlineRecord {
            id: unit.id.clone(),
            level_code: unit.level.clone(),
            order: unit.order,
            title: unit.title.clone(),
            objectives: unit.objectives.clone(),
            vocabulary: unit
                .vocabulary
                .iter()
                .enumerate()
                .map(|(item_index, item)| vocabulary_item_record(&unit.id, item_index, item))
                .collect(),
        }
    }
}

fn vocabulary_item_record(unit_id: &str, item_index: usize, item: &VocabularyItem) -> VocabularyItemRecord {
    VocabularyItemRecord {
        card_id: shared::course::vocabulary_card_id(unit_id, item_index),
        target: item.target.clone(),
        source: item.source.clone(),
        source_literal: item.source_literal.clone(),
        part_of_speech: item.part_of_speech.clone(),
        note: item.note.clone(),
        clip_key: item.clip_key.clone(),
        example: (&item.example).into(),
    }
}

/// A published course as a course list shows it, before its outline is opened.
#[derive(Debug, Clone, uniffi::Record)]
pub struct CourseSummaryRecord {
    pub id: String,
    pub title: String,
    pub target_language_name: String,
    pub source_language_name: String,
}

impl CourseSummaryRecord {
    pub fn from_index_entry(course_id: &str, entry: &CourseIndexEntry) -> CourseSummaryRecord {
        CourseSummaryRecord {
            id: course_id.to_string(),
            title: entry.title.clone(),
            target_language_name: course::language_name(&entry.target_language),
            source_language_name: course::language_name(&entry.source_language),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct CourseOutlineRecord {
    pub id: String,
    pub title: String,
    pub target_language: String,
    pub source_language: String,
    pub target_language_name: String,
    pub source_language_name: String,
    pub levels: Vec<CourseLevelRecord>,
    pub units: Vec<UnitOutlineRecord>,
}

impl From<&CourseOutline> for CourseOutlineRecord {
    fn from(outline: &CourseOutline) -> CourseOutlineRecord {
        CourseOutlineRecord {
            id: outline.id.clone(),
            title: outline.title.clone(),
            target_language: outline.target_language.clone(),
            source_language: outline.source_language.clone(),
            target_language_name: course::language_name(&outline.target_language),
            source_language_name: course::language_name(&outline.source_language),
            levels: outline.levels.iter().map(CourseLevelRecord::from).collect(),
            units: outline.units.iter().map(UnitOutlineRecord::from).collect(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct DialogueLineRecord {
    pub speaker: String,
    pub is_female_speaker: bool,
    pub phrase: PhrasePairRecord,
}

impl From<&DialogueLine> for DialogueLineRecord {
    fn from(line: &DialogueLine) -> DialogueLineRecord {
        DialogueLineRecord {
            speaker: line.speaker.clone(),
            is_female_speaker: line.gender == SpeakerGenderKind::Female,
            phrase: PhrasePairRecord {
                target: line.target.clone(),
                source: line.source.clone(),
                source_literal: line.source_literal.clone(),
                clip_key: line.clip_key.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct DialogueRecord {
    pub id: String,
    pub title: String,
    pub context: String,
    pub lines: Vec<DialogueLineRecord>,
}

impl From<&Dialogue> for DialogueRecord {
    fn from(dialogue: &Dialogue) -> DialogueRecord {
        DialogueRecord {
            id: dialogue.id.clone(),
            title: dialogue.title.clone(),
            context: dialogue.context.clone(),
            lines: dialogue.lines.iter().map(DialogueLineRecord::from).collect(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct GrammarTableRecord {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl From<&GrammarTable> for GrammarTableRecord {
    fn from(table: &GrammarTable) -> GrammarTableRecord {
        GrammarTableRecord {
            headers: table.headers.clone(),
            rows: table.rows.clone(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct GrammarNoteRecord {
    pub title: String,
    pub explanation: String,
    pub table: Option<GrammarTableRecord>,
    pub examples: Vec<PhrasePairRecord>,
}

impl From<&GrammarNote> for GrammarNoteRecord {
    fn from(note: &GrammarNote) -> GrammarNoteRecord {
        GrammarNoteRecord {
            title: note.title.clone(),
            explanation: note.explanation.clone(),
            table: note.table.as_ref().map(GrammarTableRecord::from),
            examples: note.examples.iter().map(PhrasePairRecord::from).collect(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct CultureNoteRecord {
    pub title: String,
    pub text: String,
}

impl From<&CultureNote> for CultureNoteRecord {
    fn from(note: &CultureNote) -> CultureNoteRecord {
        CultureNoteRecord {
            title: note.title.clone(),
            text: note.text.clone(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct ReadingRecord {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<PhrasePairRecord>,
    pub questions: Vec<ExerciseRecord>,
}

impl From<&Reading> for ReadingRecord {
    fn from(reading: &Reading) -> ReadingRecord {
        ReadingRecord {
            id: reading.id.clone(),
            title: reading.title.clone(),
            paragraphs: reading.paragraphs.iter().map(PhrasePairRecord::from).collect(),
            questions: native_exercise_records(&reading.questions),
        }
    }
}

#[derive(Debug, Clone, uniffi::Enum)]
pub enum ExerciseRecord {
    MultipleChoice {
        prompt: String,
        choices: Vec<String>,
        answer_index: u32,
        explanation: Option<String>,
    },
    FillBlank {
        prompt_target: String,
        answers: Vec<String>,
        hint_source: String,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    TranslateToTarget {
        prompt_source: String,
        answers: Vec<String>,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    OrderWords {
        prompt_source: String,
        words: Vec<String>,
        distractors: Vec<String>,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    ListeningChoice {
        audio_target: String,
        choices: Vec<String>,
        answer_index: u32,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    Dictation {
        audio_target: String,
        hint_source: String,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    Match {
        prompt: Option<String>,
        pairs: Vec<PhrasePairRecord>,
        explanation: Option<String>,
    },
}

impl ExerciseRecord {
    /// `None` for an exercise the native apps do not offer.
    fn of_native_exercise(exercise: &Exercise) -> Option<ExerciseRecord> {
        let record: ExerciseRecord = match exercise {
            Exercise::MultipleChoice(exercise) => ExerciseRecord::MultipleChoice {
                prompt: exercise.prompt.clone(),
                choices: exercise.choices.clone(),
                answer_index: exercise.answer_index as u32,
                explanation: exercise.explanation.clone(),
            },
            Exercise::FillBlank(exercise) => ExerciseRecord::FillBlank {
                prompt_target: exercise.prompt_target.clone(),
                answers: exercise.answers.clone(),
                hint_source: exercise.hint_source.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::TranslateToTarget(exercise) => ExerciseRecord::TranslateToTarget {
                prompt_source: exercise.prompt_source.clone(),
                answers: exercise.answers.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::OrderWords(exercise) => ExerciseRecord::OrderWords {
                prompt_source: exercise.prompt_source.clone(),
                words: exercise.words.clone(),
                distractors: exercise.distractors.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::ListeningChoice(exercise) => ExerciseRecord::ListeningChoice {
                audio_target: exercise.audio_target.clone(),
                choices: exercise.choices.clone(),
                answer_index: exercise.answer_index as u32,
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::Dictation(exercise) => ExerciseRecord::Dictation {
                audio_target: exercise.audio_target.clone(),
                hint_source: exercise.hint_source.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::Match(exercise) => ExerciseRecord::Match {
                prompt: exercise.prompt.clone(),
                pairs: exercise.pairs.iter().map(PhrasePairRecord::from).collect(),
                explanation: exercise.explanation.clone(),
            },
            Exercise::OpenAnswer(_) => return None,
        };

        Some(record)
    }
}

/// Open answers are graded only in the web app.
fn is_native_exercise(exercise: &Exercise) -> bool {
    exercise.kind() != ExerciseKind::OpenAnswer
}

fn native_exercise_records(exercises: &[Exercise]) -> Vec<ExerciseRecord> {
    exercises.iter().filter_map(ExerciseRecord::of_native_exercise).collect()
}

/// The lesson without the exercises the native apps do not offer.
pub fn native_lesson(lesson: UnitLesson) -> UnitLesson {
    let mut native_lesson: UnitLesson = lesson;

    native_lesson.exercises.retain(is_native_exercise);

    for reading in &mut native_lesson.readings {
        reading.questions.retain(is_native_exercise);
    }

    native_lesson
}

impl From<&ExerciseRecord> for Exercise {
    fn from(record: &ExerciseRecord) -> Exercise {
        match record.clone() {
            ExerciseRecord::MultipleChoice { prompt, choices, answer_index, explanation } => Exercise::MultipleChoice(MultipleChoiceExercise {
                prompt,
                choices,
                answer_index: answer_index as usize,
                explanation,
            }),
            ExerciseRecord::FillBlank { prompt_target, answers, hint_source, explanation, clip_key } => Exercise::FillBlank(FillBlankExercise {
                prompt_target,
                answers,
                hint_source,
                explanation,
                clip_key,
            }),
            ExerciseRecord::TranslateToTarget { prompt_source, answers, explanation, clip_key } => Exercise::TranslateToTarget(TranslateToTargetExercise {
                prompt_source,
                answers,
                explanation,
                clip_key,
            }),
            ExerciseRecord::OrderWords { prompt_source, words, distractors, explanation, clip_key } => Exercise::OrderWords(OrderWordsExercise {
                prompt_source,
                words,
                distractors,
                explanation,
                clip_key,
            }),
            ExerciseRecord::ListeningChoice { audio_target, choices, answer_index, explanation, clip_key } => {
                Exercise::ListeningChoice(ListeningChoiceExercise {
                    audio_target,
                    choices,
                    answer_index: answer_index as usize,
                    explanation,
                    clip_key,
                })
            },
            ExerciseRecord::Dictation { audio_target, hint_source, explanation, clip_key } => Exercise::Dictation(DictationExercise {
                audio_target,
                hint_source,
                explanation,
                clip_key,
            }),
            ExerciseRecord::Match { prompt, pairs, explanation } => Exercise::Match(MatchExercise {
                prompt,
                pairs: pairs.iter().map(PhrasePair::from).collect(),
                explanation,
            }),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UnitLessonRecord {
    pub dialogues: Vec<DialogueRecord>,
    pub grammar: Vec<GrammarNoteRecord>,
    pub culture: CultureNoteRecord,
    pub readings: Vec<ReadingRecord>,
    pub exercises: Vec<ExerciseRecord>,
}

impl From<&UnitLesson> for UnitLessonRecord {
    fn from(lesson: &UnitLesson) -> UnitLessonRecord {
        UnitLessonRecord {
            dialogues: lesson.dialogues.iter().map(DialogueRecord::from).collect(),
            grammar: lesson.grammar.iter().map(GrammarNoteRecord::from).collect(),
            culture: (&lesson.culture).into(),
            readings: lesson.readings.iter().map(ReadingRecord::from).collect(),
            exercises: native_exercise_records(&lesson.exercises),
        }
    }
}

/// The parts of a unit a learner moves between.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum UnitTabRecord {
    Overview,
    Grammar,
    Dialogues,
    Readings,
    Vocabulary,
    Practice,
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::course::OpenAnswerExercise;

    fn choice() -> Exercise {
        Exercise::MultipleChoice(MultipleChoiceExercise {
            prompt: "Which is a greeting?".to_string(),
            choices: vec!["Oi".to_string(), "Casa".to_string(), "Rua".to_string()],
            answer_index: 0,
            explanation: None,
        })
    }

    fn open_answer() -> Exercise {
        Exercise::OpenAnswer(OpenAnswerExercise {
            prompt_source: "Introduce yourself.".to_string(),
            context_source: None,
            example_answers: vec!["Oi, eu sou a Ana.".to_string()],
            explanation: None,
            clip_key: Some("open-answer-clip".to_string()),
        })
    }

    fn exercise_kinds(exercises: &[Exercise]) -> Vec<ExerciseKind> {
        exercises.iter().map(Exercise::kind).collect()
    }

    #[test]
    fn native_lesson_keeps_every_exercise_but_open_answers() {
        let lesson: UnitLesson = UnitLesson {
            dialogues: Vec::new(),
            grammar: Vec::new(),
            culture: CultureNote {
                title: "Culture".to_string(),
                text: "Text".to_string(),
            },
            readings: vec![Reading {
                id: "r1".to_string(),
                title: "Reading".to_string(),
                paragraphs: Vec::new(),
                questions: vec![open_answer(), choice()],
            }],
            exercises: vec![choice(), open_answer(), choice()],
        };

        let native: UnitLesson = native_lesson(lesson);

        assert_eq!(exercise_kinds(&native.exercises), vec![ExerciseKind::MultipleChoice, ExerciseKind::MultipleChoice]);
        assert_eq!(exercise_kinds(&native.readings[0].questions), vec![ExerciseKind::MultipleChoice]);
    }
}
