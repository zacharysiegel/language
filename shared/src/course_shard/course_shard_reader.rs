use std::collections::BTreeMap;

use crate::AppError;
use crate::course::{
    CourseOutline, CultureNote, Dialogue, DialogueLine, DictationExercise, Exercise, ExerciseKind, FillBlankExercise,
    GrammarNote, GrammarTable, LevelKind, LevelSummary, ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise,
    OrderWordsExercise, PhrasePair, Reading, SpeakerGenderKind, TranslateToPortugueseExercise, UnitLesson, UnitOutline,
    VocabularyItem,
};
use crate::course_shard::{APPLICATION_ID, ClipAudioEntry, CourseShardContents, SCHEMA_VERSION, UnitShardContents, VoiceEngineKind};
use crate::sqlite::{ReadOnlyDatabase, SqlRecord, validate_shard_header};

pub fn read_course_shard(bytes: &[u8]) -> Result<CourseShardContents, AppError> {
    let database: ReadOnlyDatabase = open_shard(bytes)?;

    let title: String = read_course_title(&database)?;
    let levels: Vec<LevelSummary> = read_levels(&database)?;
    let units: Vec<UnitOutline> = read_unit_outlines(&database)?;
    let clip_audio: Vec<ClipAudioEntry> = read_clip_audio(&database)?;

    Ok(CourseShardContents {
        outline: CourseOutline { title, levels, units },
        clip_audio,
    })
}

pub fn read_unit_shard(bytes: &[u8]) -> Result<UnitShardContents, AppError> {
    let database: ReadOnlyDatabase = open_shard(bytes)?;

    let culture: CultureNote = read_culture_note(&database)?;
    let dialogues: Vec<Dialogue> = read_dialogues(&database)?;
    let grammar: Vec<GrammarNote> = read_grammar_notes(&database)?;
    let mut exercises_by_reading: BTreeMap<Option<usize>, Vec<Exercise>> = read_exercises(&database)?;
    let readings: Vec<Reading> = read_readings(&database, &mut exercises_by_reading)?;
    let exercises: Vec<Exercise> = exercises_by_reading.remove(&None).unwrap_or_default();
    let clip_audio: Vec<ClipAudioEntry> = read_clip_audio(&database)?;

    Ok(UnitShardContents {
        lesson: UnitLesson {
            dialogues,
            grammar,
            culture,
            readings,
            exercises,
        },
        clip_audio,
    })
}

fn open_shard(bytes: &[u8]) -> Result<ReadOnlyDatabase, AppError> {
    validate_shard_header(bytes, APPLICATION_ID, SCHEMA_VERSION)?;

    ReadOnlyDatabase::open(bytes)
}

fn read_course_title(database: &ReadOnlyDatabase) -> Result<String, AppError> {
    let records: Vec<SqlRecord> = database.query("select title from course;")?;
    let record: &SqlRecord = records
        .first()
        .ok_or_else(|| AppError::from("course shard: no course record".to_string()))?;

    record.text(0)
}

fn read_levels(database: &ReadOnlyDatabase) -> Result<Vec<LevelSummary>, AppError> {
    let records: Vec<SqlRecord> = database.query("select level_code, title from level order by position asc;")?;
    let mut levels: Vec<LevelSummary> = Vec::with_capacity(records.len());

    for record in &records {
        let code: String = record.text(0)?;

        levels.push(LevelSummary {
            code: LevelKind::try_from(code.as_str())?,
            title: record.text(1)?,
        });
    }

    Ok(levels)
}

fn read_unit_outlines(database: &ReadOnlyDatabase) -> Result<Vec<UnitOutline>, AppError> {
    let mut objectives_by_unit: BTreeMap<String, Vec<String>> = read_objectives(database)?;
    let mut vocabulary_by_unit: BTreeMap<String, Vec<VocabularyItem>> = read_vocabulary(database)?;
    let unit_records: Vec<SqlRecord> =
        database.query("select unit_id, level_code, ordinal, title from unit order by ordinal asc;")?;
    let mut units: Vec<UnitOutline> = Vec::with_capacity(unit_records.len());

    for record in &unit_records {
        let id: String = record.text(0)?;
        let level_code: String = record.text(1)?;
        let order: u32 = u32::try_from(record.position(2)?)
            .map_err(|_| AppError::from(format!("course shard: unit ordinal out of range; [unit_id={id}]")))?;

        units.push(UnitOutline {
            level: LevelKind::try_from(level_code.as_str())?,
            order,
            title: record.text(3)?,
            objectives: objectives_by_unit.remove(&id).unwrap_or_default(),
            vocabulary: vocabulary_by_unit.remove(&id).unwrap_or_default(),
            id,
        });
    }

    Ok(units)
}

fn read_objectives(database: &ReadOnlyDatabase) -> Result<BTreeMap<String, Vec<String>>, AppError> {
    let records: Vec<SqlRecord> =
        database.query("select unit_id, objective from unit_objective order by unit_id asc, position asc;")?;
    let mut objectives_by_unit: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for record in &records {
        objectives_by_unit.entry(record.text(0)?).or_default().push(record.text(1)?);
    }

    Ok(objectives_by_unit)
}

fn read_vocabulary(database: &ReadOnlyDatabase) -> Result<BTreeMap<String, Vec<VocabularyItem>>, AppError> {
    let records: Vec<SqlRecord> = database.query(
        "select unit_id, pt, en, literal, part_of_speech, note, clip_key, example_pt, example_en, example_literal, example_clip_key \
         from vocabulary_item order by unit_id asc, position asc;",
    )?;
    let mut vocabulary_by_unit: BTreeMap<String, Vec<VocabularyItem>> = BTreeMap::new();

    for record in &records {
        let item: VocabularyItem = VocabularyItem {
            pt: record.text(1)?,
            en: record.text(2)?,
            literal: record.optional_text(3)?,
            pos: record.optional_text(4)?,
            note: record.optional_text(5)?,
            clip_key: record.optional_text(6)?,
            example: PhrasePair {
                pt: record.text(7)?,
                en: record.text(8)?,
                literal: record.optional_text(9)?,
                clip_key: record.optional_text(10)?,
            },
        };

        vocabulary_by_unit.entry(record.text(0)?).or_default().push(item);
    }

    Ok(vocabulary_by_unit)
}

fn read_clip_audio(database: &ReadOnlyDatabase) -> Result<Vec<ClipAudioEntry>, AppError> {
    let records: Vec<SqlRecord> = database.query(
        "select clip_key, engine_code, audio_sha256, audio_size_bytes from clip_audio order by clip_key asc, engine_code asc;",
    )?;
    let mut entries: Vec<ClipAudioEntry> = Vec::with_capacity(records.len());

    for record in &records {
        let engine_code: String = record.text(1)?;

        entries.push(ClipAudioEntry {
            clip_key: record.text(0)?,
            engine: VoiceEngineKind::try_from(engine_code.as_str())?,
            audio_sha256: record.text(2)?,
            audio_size_bytes: u64::try_from(record.integer(3)?)
                .map_err(|_| AppError::from("course shard: negative audio size".to_string()))?,
        });
    }

    Ok(entries)
}

fn read_culture_note(database: &ReadOnlyDatabase) -> Result<CultureNote, AppError> {
    let records: Vec<SqlRecord> = database.query("select title, body from culture_note;")?;
    let record: &SqlRecord = records
        .first()
        .ok_or_else(|| AppError::from("unit shard: no culture note record".to_string()))?;

    Ok(CultureNote {
        title: record.text(0)?,
        text: record.text(1)?,
    })
}

/* Columns `pt, en, literal, clip_key` starting at `first_column`. */
fn read_phrase_pair(record: &SqlRecord, first_column: usize) -> Result<PhrasePair, AppError> {
    Ok(PhrasePair {
        pt: record.text(first_column)?,
        en: record.text(first_column + 1)?,
        literal: record.optional_text(first_column + 2)?,
        clip_key: record.optional_text(first_column + 3)?,
    })
}

fn read_dialogues(database: &ReadOnlyDatabase) -> Result<Vec<Dialogue>, AppError> {
    let mut lines_by_dialogue: BTreeMap<usize, Vec<DialogueLine>> = read_dialogue_lines(database)?;
    let records: Vec<SqlRecord> =
        database.query("select position, dialogue_id, title, context from dialogue order by position asc;")?;
    let mut dialogues: Vec<Dialogue> = Vec::with_capacity(records.len());

    for record in &records {
        dialogues.push(Dialogue {
            id: record.text(1)?,
            title: record.text(2)?,
            context: record.text(3)?,
            lines: lines_by_dialogue.remove(&record.position(0)?).unwrap_or_default(),
        });
    }

    Ok(dialogues)
}

fn read_dialogue_lines(database: &ReadOnlyDatabase) -> Result<BTreeMap<usize, Vec<DialogueLine>>, AppError> {
    let records: Vec<SqlRecord> = database.query(
        "select dialogue_position, speaker, speaker_gender, pt, en, literal, clip_key \
         from dialogue_line order by dialogue_position asc, position asc;",
    )?;
    let mut lines_by_dialogue: BTreeMap<usize, Vec<DialogueLine>> = BTreeMap::new();

    for record in &records {
        let gender_code: String = record.text(2)?;
        let line: DialogueLine = DialogueLine {
            speaker: record.text(1)?,
            gender: SpeakerGenderKind::try_from(gender_code.as_str())?,
            pt: record.text(3)?,
            en: record.text(4)?,
            literal: record.optional_text(5)?,
            clip_key: record.optional_text(6)?,
        };

        lines_by_dialogue.entry(record.position(0)?).or_default().push(line);
    }

    Ok(lines_by_dialogue)
}

fn read_grammar_notes(database: &ReadOnlyDatabase) -> Result<Vec<GrammarNote>, AppError> {
    let mut headers_by_note: BTreeMap<usize, Vec<String>> = read_grammar_table_headers(database)?;
    let mut rows_by_note: BTreeMap<usize, Vec<Vec<String>>> = read_grammar_table_rows(database)?;
    let mut examples_by_note: BTreeMap<usize, Vec<PhrasePair>> = read_grammar_examples(database)?;
    let records: Vec<SqlRecord> =
        database.query("select position, title, explanation from grammar_note order by position asc;")?;
    let mut notes: Vec<GrammarNote> = Vec::with_capacity(records.len());

    for record in &records {
        let note_position: usize = record.position(0)?;
        let headers: Option<Vec<String>> = headers_by_note.remove(&note_position);
        let table: Option<GrammarTable> = headers.map(|headers| GrammarTable {
            headers,
            rows: rows_by_note.remove(&note_position).unwrap_or_default(),
        });

        notes.push(GrammarNote {
            title: record.text(1)?,
            explanation: record.text(2)?,
            table,
            examples: examples_by_note.remove(&note_position).unwrap_or_default(),
        });
    }

    Ok(notes)
}

fn read_grammar_table_headers(database: &ReadOnlyDatabase) -> Result<BTreeMap<usize, Vec<String>>, AppError> {
    let records: Vec<SqlRecord> = database
        .query("select note_position, header_text from grammar_table_header order by note_position asc, position asc;")?;
    let mut headers_by_note: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for record in &records {
        headers_by_note.entry(record.position(0)?).or_default().push(record.text(1)?);
    }

    Ok(headers_by_note)
}

fn read_grammar_table_rows(database: &ReadOnlyDatabase) -> Result<BTreeMap<usize, Vec<Vec<String>>>, AppError> {
    let records: Vec<SqlRecord> = database.query(
        "select note_position, row_position, cell_text from grammar_table_cell \
         order by note_position asc, row_position asc, column_position asc;",
    )?;
    let mut rows_by_note: BTreeMap<usize, Vec<Vec<String>>> = BTreeMap::new();
    let mut previous_row_position: Option<(usize, usize)> = None;

    for record in &records {
        let note_position: usize = record.position(0)?;
        let row_position: usize = record.position(1)?;
        let rows: &mut Vec<Vec<String>> = rows_by_note.entry(note_position).or_default();

        if previous_row_position != Some((note_position, row_position)) {
            rows.push(Vec::new());
            previous_row_position = Some((note_position, row_position));
        }

        if let Some(row) = rows.last_mut() {
            row.push(record.text(2)?);
        }
    }

    Ok(rows_by_note)
}

fn read_grammar_examples(database: &ReadOnlyDatabase) -> Result<BTreeMap<usize, Vec<PhrasePair>>, AppError> {
    let records: Vec<SqlRecord> = database.query(
        "select note_position, pt, en, literal, clip_key from grammar_example order by note_position asc, position asc;",
    )?;
    let mut examples_by_note: BTreeMap<usize, Vec<PhrasePair>> = BTreeMap::new();

    for record in &records {
        examples_by_note.entry(record.position(0)?).or_default().push(read_phrase_pair(record, 1)?);
    }

    Ok(examples_by_note)
}

fn read_readings(
    database: &ReadOnlyDatabase,
    exercises_by_reading: &mut BTreeMap<Option<usize>, Vec<Exercise>>,
) -> Result<Vec<Reading>, AppError> {
    let mut paragraphs_by_reading: BTreeMap<usize, Vec<PhrasePair>> = read_reading_paragraphs(database)?;
    let records: Vec<SqlRecord> = database.query("select position, reading_id, title from reading order by position asc;")?;
    let mut readings: Vec<Reading> = Vec::with_capacity(records.len());

    for record in &records {
        let reading_position: usize = record.position(0)?;

        readings.push(Reading {
            id: record.text(1)?,
            title: record.text(2)?,
            paragraphs: paragraphs_by_reading.remove(&reading_position).unwrap_or_default(),
            questions: exercises_by_reading.remove(&Some(reading_position)).unwrap_or_default(),
        });
    }

    Ok(readings)
}

fn read_reading_paragraphs(database: &ReadOnlyDatabase) -> Result<BTreeMap<usize, Vec<PhrasePair>>, AppError> {
    let records: Vec<SqlRecord> = database.query(
        "select reading_position, pt, en, literal, clip_key from reading_paragraph order by reading_position asc, position asc;",
    )?;
    let mut paragraphs_by_reading: BTreeMap<usize, Vec<PhrasePair>> = BTreeMap::new();

    for record in &records {
        paragraphs_by_reading.entry(record.position(0)?).or_default().push(read_phrase_pair(record, 1)?);
    }

    Ok(paragraphs_by_reading)
}

/// The values of each exercise's child tables, keyed by `exercise_id`.
struct ExerciseChildren {
    choices: BTreeMap<i64, Vec<String>>,
    answers: BTreeMap<i64, Vec<String>>,
    words: BTreeMap<i64, Vec<String>>,
    distractors: BTreeMap<i64, Vec<String>>,
    pairs: BTreeMap<i64, Vec<PhrasePair>>,
}

/// Exercises grouped by the reading they belong to (`None` for the unit's own), each group in position order.
fn read_exercises(database: &ReadOnlyDatabase) -> Result<BTreeMap<Option<usize>, Vec<Exercise>>, AppError> {
    let mut children: ExerciseChildren = read_exercise_children(database)?;
    let records: Vec<SqlRecord> = database.query(
        "select exercise_id, reading_position, exercise_kind, prompt, correct_choice_index, hint_en, explanation, clip_key \
         from exercise order by reading_position asc, position asc;",
    )?;
    let mut exercises_by_reading: BTreeMap<Option<usize>, Vec<Exercise>> = BTreeMap::new();

    for record in &records {
        let exercise_id: i64 = record.integer(0)?;
        let reading_position: Option<usize> = record
            .optional_integer(1)?
            .map(|position| {
                usize::try_from(position)
                    .map_err(|_| AppError::from(format!("unit shard: negative reading position; [exercise_id={exercise_id}]")))
            })
            .transpose()?;
        let exercise: Exercise = assemble_exercise(record, exercise_id, &mut children)?;

        exercises_by_reading.entry(reading_position).or_default().push(exercise);
    }

    Ok(exercises_by_reading)
}

fn assemble_exercise(record: &SqlRecord, exercise_id: i64, children: &mut ExerciseChildren) -> Result<Exercise, AppError> {
    let kind_code: String = record.text(2)?;
    let kind: ExerciseKind = ExerciseKind::try_from(kind_code.as_str())?;
    let prompt: Option<String> = record.optional_text(3)?;
    let correct_choice_index: Option<i64> = record.optional_integer(4)?;
    let hint_en: Option<String> = record.optional_text(5)?;
    let explanation: Option<String> = record.optional_text(6)?;
    let clip_key: Option<String> = record.optional_text(7)?;
    let required_prompt = || {
        prompt
            .clone()
            .ok_or_else(|| AppError::from(format!("unit shard: exercise has no prompt; [exercise_id={exercise_id} kind={kind_code}]")))
    };
    let required_hint = || {
        hint_en
            .clone()
            .ok_or_else(|| AppError::from(format!("unit shard: exercise has no hint; [exercise_id={exercise_id} kind={kind_code}]")))
    };
    let required_answer_index = || {
        correct_choice_index
            .and_then(|index| usize::try_from(index).ok())
            .ok_or_else(|| AppError::from(format!("unit shard: exercise has no correct choice; [exercise_id={exercise_id} kind={kind_code}]")))
    };

    let exercise: Exercise = match kind {
        ExerciseKind::MultipleChoice => Exercise::MultipleChoice(MultipleChoiceExercise {
            prompt: required_prompt()?,
            choices: children.choices.remove(&exercise_id).unwrap_or_default(),
            answer: required_answer_index()?,
            explanation,
        }),
        ExerciseKind::FillBlank => Exercise::FillBlank(FillBlankExercise {
            prompt_pt: required_prompt()?,
            answer: children.answers.remove(&exercise_id).unwrap_or_default(),
            hint_en: required_hint()?,
            explanation,
            clip_key,
        }),
        ExerciseKind::TranslateToPortuguese => Exercise::TranslateToPortuguese(TranslateToPortugueseExercise {
            prompt_en: required_prompt()?,
            answers: children.answers.remove(&exercise_id).unwrap_or_default(),
            explanation,
            clip_key,
        }),
        ExerciseKind::OrderWords => Exercise::OrderWords(OrderWordsExercise {
            prompt_en: required_prompt()?,
            words: children.words.remove(&exercise_id).unwrap_or_default(),
            distractors: children.distractors.remove(&exercise_id).unwrap_or_default(),
            explanation,
            clip_key,
        }),
        ExerciseKind::ListeningChoice => Exercise::ListeningChoice(ListeningChoiceExercise {
            audio_pt: required_prompt()?,
            choices: children.choices.remove(&exercise_id).unwrap_or_default(),
            answer: required_answer_index()?,
            explanation,
            clip_key,
        }),
        ExerciseKind::Dictation => Exercise::Dictation(DictationExercise {
            audio_pt: required_prompt()?,
            hint_en: required_hint()?,
            explanation,
            clip_key,
        }),
        ExerciseKind::Match => Exercise::Match(MatchExercise {
            prompt,
            pairs: children.pairs.remove(&exercise_id).unwrap_or_default(),
            explanation,
        }),
    };

    Ok(exercise)
}

fn read_exercise_children(database: &ReadOnlyDatabase) -> Result<ExerciseChildren, AppError> {
    let mut words: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    let mut distractors: BTreeMap<i64, Vec<String>> = BTreeMap::new();

    let word_records: Vec<SqlRecord> = database.query(
        "select exercise_id, word_text, is_distractor from exercise_word order by exercise_id asc, position asc;",
    )?;

    for record in &word_records {
        let is_distractor: bool = record.integer(2)? != 0;
        let target: &mut BTreeMap<i64, Vec<String>> = if is_distractor { &mut distractors } else { &mut words };

        target.entry(record.integer(0)?).or_default().push(record.text(1)?);
    }

    Ok(ExerciseChildren {
        choices: read_exercise_texts(database, "select exercise_id, choice_text from exercise_choice order by exercise_id asc, position asc;")?,
        answers: read_exercise_texts(database, "select exercise_id, answer_text from exercise_answer order by exercise_id asc, position asc;")?,
        words,
        distractors,
        pairs: read_exercise_pairs(database)?,
    })
}

fn read_exercise_texts(database: &ReadOnlyDatabase, sql: &str) -> Result<BTreeMap<i64, Vec<String>>, AppError> {
    let records: Vec<SqlRecord> = database.query(sql)?;
    let mut texts_by_exercise: BTreeMap<i64, Vec<String>> = BTreeMap::new();

    for record in &records {
        texts_by_exercise.entry(record.integer(0)?).or_default().push(record.text(1)?);
    }

    Ok(texts_by_exercise)
}

fn read_exercise_pairs(database: &ReadOnlyDatabase) -> Result<BTreeMap<i64, Vec<PhrasePair>>, AppError> {
    let records: Vec<SqlRecord> = database
        .query("select exercise_id, pt, en, literal, clip_key from exercise_pair order by exercise_id asc, position asc;")?;
    let mut pairs_by_exercise: BTreeMap<i64, Vec<PhrasePair>> = BTreeMap::new();

    for record in &records {
        pairs_by_exercise.entry(record.integer(0)?).or_default().push(read_phrase_pair(record, 1)?);
    }

    Ok(pairs_by_exercise)
}
