//! The course model as records the native apps receive; each converts from its `shared::course` counterpart.

use shared::course::{
    CourseOutline, CultureNote, Dialogue, DialogueLine, DictationExercise, Exercise, FillBlankExercise, GrammarNote,
    GrammarTable, LevelKind, LevelSummary, ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise,
    OrderWordsExercise, PhrasePair, Reading, SpeakerGenderKind, TranslateToPortugueseExercise, UnitLesson, UnitOutline,
    VocabularyItem,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum LevelRecord {
    A1,
    A2,
    B1,
    B2,
}

impl From<LevelKind> for LevelRecord {
    fn from(level: LevelKind) -> LevelRecord {
        match level {
            LevelKind::A1 => LevelRecord::A1,
            LevelKind::A2 => LevelRecord::A2,
            LevelKind::B1 => LevelRecord::B1,
            LevelKind::B2 => LevelRecord::B2,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct LevelSummaryRecord {
    pub level: LevelRecord,
    pub code: String,
    pub title: String,
}

impl From<&LevelSummary> for LevelSummaryRecord {
    fn from(summary: &LevelSummary) -> LevelSummaryRecord {
        LevelSummaryRecord {
            level: summary.code.into(),
            code: summary.code.code().to_string(),
            title: summary.title.clone(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct PhrasePairRecord {
    pub pt: String,
    pub en: String,
    pub literal: Option<String>,
    pub clip_key: Option<String>,
}

impl From<&PhrasePair> for PhrasePairRecord {
    fn from(pair: &PhrasePair) -> PhrasePairRecord {
        PhrasePairRecord {
            pt: pair.pt.clone(),
            en: pair.en.clone(),
            literal: pair.literal.clone(),
            clip_key: pair.clip_key.clone(),
        }
    }
}

impl From<&PhrasePairRecord> for PhrasePair {
    fn from(record: &PhrasePairRecord) -> PhrasePair {
        PhrasePair {
            pt: record.pt.clone(),
            en: record.en.clone(),
            literal: record.literal.clone(),
            clip_key: record.clip_key.clone(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct VocabularyItemRecord {
    pub card_id: String,
    pub pt: String,
    pub en: String,
    pub literal: Option<String>,
    pub part_of_speech: Option<String>,
    pub note: Option<String>,
    pub clip_key: Option<String>,
    pub example: PhrasePairRecord,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UnitOutlineRecord {
    pub id: String,
    pub level: LevelRecord,
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
            level: unit.level.into(),
            level_code: unit.level.code().to_string(),
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
        pt: item.pt.clone(),
        en: item.en.clone(),
        literal: item.literal.clone(),
        part_of_speech: item.pos.clone(),
        note: item.note.clone(),
        clip_key: item.clip_key.clone(),
        example: (&item.example).into(),
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct CourseOutlineRecord {
    pub title: String,
    pub levels: Vec<LevelSummaryRecord>,
    pub units: Vec<UnitOutlineRecord>,
}

impl From<&CourseOutline> for CourseOutlineRecord {
    fn from(outline: &CourseOutline) -> CourseOutlineRecord {
        CourseOutlineRecord {
            title: outline.title.clone(),
            levels: outline.levels.iter().map(LevelSummaryRecord::from).collect(),
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
                pt: line.pt.clone(),
                en: line.en.clone(),
                literal: line.literal.clone(),
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
            questions: reading.questions.iter().map(ExerciseRecord::from).collect(),
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
        prompt_pt: String,
        answers: Vec<String>,
        hint_en: String,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    TranslateToPortuguese {
        prompt_en: String,
        answers: Vec<String>,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    OrderWords {
        prompt_en: String,
        words: Vec<String>,
        distractors: Vec<String>,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    ListeningChoice {
        audio_pt: String,
        choices: Vec<String>,
        answer_index: u32,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    Dictation {
        audio_pt: String,
        hint_en: String,
        explanation: Option<String>,
        clip_key: Option<String>,
    },
    Match {
        prompt: Option<String>,
        pairs: Vec<PhrasePairRecord>,
        explanation: Option<String>,
    },
}

impl From<&Exercise> for ExerciseRecord {
    fn from(exercise: &Exercise) -> ExerciseRecord {
        match exercise {
            Exercise::MultipleChoice(exercise) => ExerciseRecord::MultipleChoice {
                prompt: exercise.prompt.clone(),
                choices: exercise.choices.clone(),
                answer_index: exercise.answer as u32,
                explanation: exercise.explanation.clone(),
            },
            Exercise::FillBlank(exercise) => ExerciseRecord::FillBlank {
                prompt_pt: exercise.prompt_pt.clone(),
                answers: exercise.answer.clone(),
                hint_en: exercise.hint_en.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::TranslateToPortuguese(exercise) => ExerciseRecord::TranslateToPortuguese {
                prompt_en: exercise.prompt_en.clone(),
                answers: exercise.answers.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::OrderWords(exercise) => ExerciseRecord::OrderWords {
                prompt_en: exercise.prompt_en.clone(),
                words: exercise.words.clone(),
                distractors: exercise.distractors.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::ListeningChoice(exercise) => ExerciseRecord::ListeningChoice {
                audio_pt: exercise.audio_pt.clone(),
                choices: exercise.choices.clone(),
                answer_index: exercise.answer as u32,
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::Dictation(exercise) => ExerciseRecord::Dictation {
                audio_pt: exercise.audio_pt.clone(),
                hint_en: exercise.hint_en.clone(),
                explanation: exercise.explanation.clone(),
                clip_key: exercise.clip_key.clone(),
            },
            Exercise::Match(exercise) => ExerciseRecord::Match {
                prompt: exercise.prompt.clone(),
                pairs: exercise.pairs.iter().map(PhrasePairRecord::from).collect(),
                explanation: exercise.explanation.clone(),
            },
        }
    }
}

impl From<&ExerciseRecord> for Exercise {
    fn from(record: &ExerciseRecord) -> Exercise {
        match record.clone() {
            ExerciseRecord::MultipleChoice { prompt, choices, answer_index, explanation } => Exercise::MultipleChoice(MultipleChoiceExercise {
                prompt,
                choices,
                answer: answer_index as usize,
                explanation,
            }),
            ExerciseRecord::FillBlank { prompt_pt, answers, hint_en, explanation, clip_key } => Exercise::FillBlank(FillBlankExercise {
                prompt_pt,
                answer: answers,
                hint_en,
                explanation,
                clip_key,
            }),
            ExerciseRecord::TranslateToPortuguese { prompt_en, answers, explanation, clip_key } => {
                Exercise::TranslateToPortuguese(TranslateToPortugueseExercise {
                    prompt_en,
                    answers,
                    explanation,
                    clip_key,
                })
            },
            ExerciseRecord::OrderWords { prompt_en, words, distractors, explanation, clip_key } => Exercise::OrderWords(OrderWordsExercise {
                prompt_en,
                words,
                distractors,
                explanation,
                clip_key,
            }),
            ExerciseRecord::ListeningChoice { audio_pt, choices, answer_index, explanation, clip_key } => {
                Exercise::ListeningChoice(ListeningChoiceExercise {
                    audio_pt,
                    choices,
                    answer: answer_index as usize,
                    explanation,
                    clip_key,
                })
            },
            ExerciseRecord::Dictation { audio_pt, hint_en, explanation, clip_key } => Exercise::Dictation(DictationExercise {
                audio_pt,
                hint_en,
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
            exercises: lesson.exercises.iter().map(ExerciseRecord::from).collect(),
        }
    }
}
