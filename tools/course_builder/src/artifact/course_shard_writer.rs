use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use rusqlite::{Connection, Transaction, params};
use shared::AppError;
use shared::course::{Course, CourseLevel, Dialogue, Exercise, GrammarNote, PhrasePair, Reading, Unit, VocabularyItem};
use shared::course_shard::{self, ClipAudioEntry};

use crate::audio_plan::{AudioPlan, Utterance};

/// Every engine's recording of each clip, by clip key.
pub type ClipAudioIndex = BTreeMap<String, Vec<ClipAudioEntry>>;

/// Clips and recordings available to the shards being written.
pub struct ClipCatalog<'a> {
    pub plan: &'a AudioPlan,
    pub audio_index: &'a ClipAudioIndex,
}

/// Writes the course outline: levels, units, their objectives and vocabulary, and the vocabulary's clips.
pub fn write_course_shard(path: &Path, course: &Course, clip_catalog: &ClipCatalog) -> Result<(), AppError> {
    let clip_keys: BTreeSet<String> = course.units.iter().flat_map(|unit| vocabulary_clip_keys(&unit.vocabulary)).collect();

    write_shard(path, &course_shard::course_shard_ddl(), |transaction| {
        insert_clips(transaction, &clip_keys, clip_catalog)?;
        insert_course(transaction, course)
    })
}

/// Writes one unit's lesson and the clips it plays.
pub fn write_unit_shard(path: &Path, unit: &Unit, clip_catalog: &ClipCatalog) -> Result<(), AppError> {
    let clip_keys: BTreeSet<String> = lesson_clip_keys(unit);

    write_shard(path, &course_shard::unit_shard_ddl(), |transaction| {
        insert_clips(transaction, &clip_keys, clip_catalog)?;
        insert_unit_lesson(transaction, unit)
    })
}

fn write_shard(
    path: &Path,
    ddl_parts: &[&str],
    insert_records: impl FnOnce(&Transaction) -> Result<(), AppError>,
) -> Result<(), AppError> {
    if path.exists() {
        fs::remove_file(path)?;
    }

    let mut connection: Connection = Connection::open(path)?;

    connection.pragma_update(None, "application_id", course_shard::APPLICATION_ID)?;
    connection.pragma_update(None, "user_version", course_shard::SCHEMA_VERSION)?;
    connection.pragma_update(None, "foreign_keys", true)?;
    connection.pragma_update(None, "journal_mode", "off")?;

    for ddl in ddl_parts {
        connection.execute_batch(ddl)?;
    }

    let transaction: Transaction = connection.transaction()?;

    insert_records(&transaction)?;
    transaction.commit()?;

    connection.execute_batch("vacuum;")?;

    Ok(())
}

fn vocabulary_clip_keys(vocabulary: &[VocabularyItem]) -> Vec<String> {
    vocabulary
        .iter()
        .flat_map(|item| [item.clip_key.clone(), item.example.clip_key.clone()])
        .flatten()
        .collect()
}

fn pair_clip_keys(pairs: &[PhrasePair]) -> impl Iterator<Item = String> + '_ {
    pairs.iter().filter_map(|pair| pair.clip_key.clone())
}

fn exercise_clip_keys(exercise: &Exercise) -> Vec<String> {
    let mut clip_keys: Vec<String> = exercise.clip_key().map(str::to_string).into_iter().collect();

    if let Exercise::Match(matching) = exercise {
        clip_keys.extend(pair_clip_keys(&matching.pairs));
    }

    clip_keys
}

fn lesson_clip_keys(unit: &Unit) -> BTreeSet<String> {
    let mut clip_keys: BTreeSet<String> = BTreeSet::new();

    for dialogue in &unit.dialogues {
        clip_keys.extend(dialogue.lines.iter().filter_map(|line| line.clip_key.clone()));
    }

    for note in &unit.grammar {
        clip_keys.extend(pair_clip_keys(&note.examples));
    }

    for reading in &unit.readings {
        clip_keys.extend(pair_clip_keys(&reading.paragraphs));
        clip_keys.extend(reading.questions.iter().flat_map(exercise_clip_keys));
    }

    clip_keys.extend(unit.exercises.iter().flat_map(exercise_clip_keys));

    clip_keys
}

fn insert_clips(transaction: &Transaction, clip_keys: &BTreeSet<String>, clip_catalog: &ClipCatalog) -> Result<(), AppError> {
    let mut clip_statement: rusqlite::Statement<'_> =
        transaction.prepare("insert into clip (clip_key, persona, spoken_text) values (?1, ?2, ?3);")?;
    let mut audio_statement: rusqlite::Statement<'_> = transaction.prepare(
        "insert into clip_audio (clip_key, engine_code, audio_sha256, audio_size_bytes) values (?1, ?2, ?3, ?4);",
    )?;

    for clip_key in clip_keys {
        let utterance: &Utterance = clip_catalog
            .plan
            .utterance(clip_key)
            .ok_or_else(|| AppError::from(format!("a shard names a clip the audio plan lacks; [clip_key={clip_key}]")))?;

        clip_statement.execute(params![clip_key, utterance.persona.key(), utterance.text])?;

        let audio_entries: &[ClipAudioEntry] = clip_catalog.audio_index.get(clip_key).map(Vec::as_slice).unwrap_or_default();

        for entry in audio_entries {
            let audio_size_bytes: i64 = i64::try_from(entry.audio_size_bytes)
                .map_err(|_| AppError::from(format!("audio file too large; [clip_key={clip_key}]")))?;

            audio_statement.execute(params![clip_key, entry.engine.code(), entry.audio_sha256, audio_size_bytes])?;
        }
    }

    Ok(())
}

fn insert_course(transaction: &Transaction, course: &Course) -> Result<(), AppError> {
    transaction.execute(
        "insert into course (course_id, title, target_language, source_language) values (?1, ?2, ?3, ?4);",
        params![course.id, course.title, course.target_language, course.source_language],
    )?;

    insert_levels(transaction, &course.levels)?;

    for unit in &course.units {
        insert_unit_outline(transaction, unit)?;
    }

    Ok(())
}

fn insert_levels(transaction: &Transaction, levels: &[CourseLevel]) -> Result<(), AppError> {
    let mut statement: rusqlite::Statement<'_> =
        transaction.prepare("insert into level (level_code, position, title, requires_readings) values (?1, ?2, ?3, ?4);")?;

    for (position, level) in levels.iter().enumerate() {
        statement.execute(params![level.code, position, level.title, level.requires_readings])?;
    }

    Ok(())
}

fn insert_unit_outline(transaction: &Transaction, unit: &Unit) -> Result<(), AppError> {
    transaction.execute(
        "insert into unit (unit_id, level_code, ordinal, title) values (?1, ?2, ?3, ?4);",
        params![unit.id, unit.level, unit.order, unit.title],
    )?;

    let mut objective_statement: rusqlite::Statement<'_> =
        transaction.prepare("insert into unit_objective (unit_id, position, objective) values (?1, ?2, ?3);")?;

    for (position, objective) in unit.objectives.iter().enumerate() {
        objective_statement.execute(params![unit.id, position, objective])?;
    }

    let mut vocabulary_statement: rusqlite::Statement<'_> = transaction.prepare(
        "insert into vocabulary_item (unit_id, position, target_text, source_text, source_literal, part_of_speech, note, clip_key, \
         example_target_text, example_source_text, example_source_literal, example_clip_key) \
         values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12);",
    )?;

    for (position, item) in unit.vocabulary.iter().enumerate() {
        vocabulary_statement.execute(params![
            unit.id,
            position,
            item.target,
            item.source,
            item.source_literal,
            item.part_of_speech,
            item.note,
            item.clip_key,
            item.example.target,
            item.example.source,
            item.example.source_literal,
            item.example.clip_key,
        ])?;
    }

    Ok(())
}

fn insert_unit_lesson(transaction: &Transaction, unit: &Unit) -> Result<(), AppError> {
    transaction.execute(
        "insert into culture_note (title, body) values (?1, ?2);",
        params![unit.culture.title, unit.culture.text],
    )?;

    insert_dialogues(transaction, &unit.dialogues)?;
    insert_grammar_notes(transaction, &unit.grammar)?;
    insert_readings(transaction, &unit.readings)?;

    let mut exercise_writer: ExerciseWriter = ExerciseWriter::prepare(transaction)?;

    for (position, exercise) in unit.exercises.iter().enumerate() {
        exercise_writer.insert(None, position, exercise)?;
    }

    for (reading_position, reading) in unit.readings.iter().enumerate() {
        for (position, question) in reading.questions.iter().enumerate() {
            exercise_writer.insert(Some(reading_position), position, question)?;
        }
    }

    Ok(())
}

fn insert_dialogues(transaction: &Transaction, dialogues: &[Dialogue]) -> Result<(), AppError> {
    let mut dialogue_statement: rusqlite::Statement<'_> =
        transaction.prepare("insert into dialogue (position, dialogue_id, title, context) values (?1, ?2, ?3, ?4);")?;
    let mut line_statement: rusqlite::Statement<'_> = transaction.prepare(
        "insert into dialogue_line (dialogue_position, position, speaker, speaker_gender, target_text, source_text, source_literal, clip_key) \
         values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
    )?;

    for (dialogue_position, dialogue) in dialogues.iter().enumerate() {
        dialogue_statement.execute(params![dialogue_position, dialogue.id, dialogue.title, dialogue.context])?;

        for (position, line) in dialogue.lines.iter().enumerate() {
            line_statement.execute(params![
                dialogue_position,
                position,
                line.speaker,
                line.gender.code(),
                line.target,
                line.source,
                line.source_literal,
                line.clip_key,
            ])?;
        }
    }

    Ok(())
}

fn insert_grammar_notes(transaction: &Transaction, notes: &[GrammarNote]) -> Result<(), AppError> {
    let mut note_statement: rusqlite::Statement<'_> =
        transaction.prepare("insert into grammar_note (position, title, explanation) values (?1, ?2, ?3);")?;
    let mut header_statement: rusqlite::Statement<'_> =
        transaction.prepare("insert into grammar_table_header (note_position, position, header_text) values (?1, ?2, ?3);")?;
    let mut cell_statement: rusqlite::Statement<'_> = transaction.prepare(
        "insert into grammar_table_cell (note_position, row_position, column_position, cell_text) values (?1, ?2, ?3, ?4);",
    )?;
    let mut example_statement: rusqlite::Statement<'_> = transaction.prepare(
        "insert into grammar_example (note_position, position, target_text, source_text, source_literal, clip_key) values (?1, ?2, ?3, ?4, ?5, ?6);",
    )?;

    for (note_position, note) in notes.iter().enumerate() {
        note_statement.execute(params![note_position, note.title, note.explanation])?;

        if let Some(table) = &note.table {
            for (position, header) in table.headers.iter().enumerate() {
                header_statement.execute(params![note_position, position, header])?;
            }

            for (row_position, row) in table.rows.iter().enumerate() {
                for (column_position, cell) in row.iter().enumerate() {
                    cell_statement.execute(params![note_position, row_position, column_position, cell])?;
                }
            }
        }

        for (position, example) in note.examples.iter().enumerate() {
            example_statement.execute(params![note_position, position, example.target, example.source, example.source_literal, example.clip_key])?;
        }
    }

    Ok(())
}

fn insert_readings(transaction: &Transaction, readings: &[Reading]) -> Result<(), AppError> {
    let mut reading_statement: rusqlite::Statement<'_> =
        transaction.prepare("insert into reading (position, reading_id, title) values (?1, ?2, ?3);")?;
    let mut paragraph_statement: rusqlite::Statement<'_> = transaction.prepare(
        "insert into reading_paragraph (reading_position, position, target_text, source_text, source_literal, clip_key) values (?1, ?2, ?3, ?4, ?5, ?6);",
    )?;

    for (reading_position, reading) in readings.iter().enumerate() {
        reading_statement.execute(params![reading_position, reading.id, reading.title])?;

        for (position, paragraph) in reading.paragraphs.iter().enumerate() {
            paragraph_statement.execute(params![
                reading_position,
                position,
                paragraph.target,
                paragraph.source,
                paragraph.source_literal,
                paragraph.clip_key,
            ])?;
        }
    }

    Ok(())
}

/// The columns of the `exercise` table that vary by kind.
struct ExerciseColumns<'a> {
    prompt: Option<&'a str>,
    correct_choice_index: Option<usize>,
    hint_source: Option<&'a str>,
}

struct ExerciseWriter<'transaction> {
    next_exercise_id: i64,
    exercise_statement: rusqlite::Statement<'transaction>,
    choice_statement: rusqlite::Statement<'transaction>,
    answer_statement: rusqlite::Statement<'transaction>,
    word_statement: rusqlite::Statement<'transaction>,
    pair_statement: rusqlite::Statement<'transaction>,
}

impl<'transaction> ExerciseWriter<'transaction> {
    fn prepare(transaction: &'transaction Transaction) -> Result<ExerciseWriter<'transaction>, AppError> {
        Ok(ExerciseWriter {
            next_exercise_id: 0,
            exercise_statement: transaction.prepare(
                "insert into exercise (exercise_id, reading_position, position, exercise_kind, prompt, correct_choice_index, \
                 hint_source, explanation, clip_key) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
            )?,
            choice_statement: transaction
                .prepare("insert into exercise_choice (exercise_id, position, choice_text) values (?1, ?2, ?3);")?,
            answer_statement: transaction
                .prepare("insert into exercise_answer (exercise_id, position, answer_text) values (?1, ?2, ?3);")?,
            word_statement: transaction.prepare(
                "insert into exercise_word (exercise_id, position, word_text, is_distractor) values (?1, ?2, ?3, ?4);",
            )?,
            pair_statement: transaction.prepare(
                "insert into exercise_pair (exercise_id, position, target_text, source_text, source_literal, clip_key) values (?1, ?2, ?3, ?4, ?5, ?6);",
            )?,
        })
    }

    fn insert(&mut self, reading_position: Option<usize>, position: usize, exercise: &Exercise) -> Result<(), AppError> {
        let exercise_id: i64 = self.next_exercise_id;
        let columns: ExerciseColumns = exercise_columns(exercise);

        self.next_exercise_id += 1;
        self.exercise_statement.execute(params![
            exercise_id,
            reading_position,
            position,
            exercise.kind().code(),
            columns.prompt,
            columns.correct_choice_index,
            columns.hint_source,
            exercise.explanation(),
            exercise.clip_key(),
        ])?;

        self.insert_children(exercise_id, exercise)
    }

    fn insert_children(&mut self, exercise_id: i64, exercise: &Exercise) -> Result<(), AppError> {
        match exercise {
            Exercise::MultipleChoice(multiple_choice) => self.insert_choices(exercise_id, &multiple_choice.choices),
            Exercise::ListeningChoice(listening) => self.insert_choices(exercise_id, &listening.choices),
            Exercise::FillBlank(fill_blank) => self.insert_answers(exercise_id, &fill_blank.answers),
            Exercise::TranslateToTarget(translate) => self.insert_answers(exercise_id, &translate.answers),
            Exercise::OrderWords(order_words) => self.insert_words(exercise_id, &order_words.words, &order_words.distractors),
            Exercise::Match(matching) => self.insert_pairs(exercise_id, &matching.pairs),
            Exercise::Dictation(_) => Ok(()),
        }
    }

    fn insert_choices(&mut self, exercise_id: i64, choices: &[String]) -> Result<(), AppError> {
        for (position, choice) in choices.iter().enumerate() {
            self.choice_statement.execute(params![exercise_id, position, choice])?;
        }

        Ok(())
    }

    fn insert_answers(&mut self, exercise_id: i64, answers: &[String]) -> Result<(), AppError> {
        for (position, answer) in answers.iter().enumerate() {
            self.answer_statement.execute(params![exercise_id, position, answer])?;
        }

        Ok(())
    }

    fn insert_words(&mut self, exercise_id: i64, words: &[String], distractors: &[String]) -> Result<(), AppError> {
        let flagged_words = words.iter().map(|word| (word, false)).chain(distractors.iter().map(|word| (word, true)));

        for (position, (word, is_distractor)) in flagged_words.enumerate() {
            self.word_statement.execute(params![exercise_id, position, word, is_distractor])?;
        }

        Ok(())
    }

    fn insert_pairs(&mut self, exercise_id: i64, pairs: &[PhrasePair]) -> Result<(), AppError> {
        for (position, pair) in pairs.iter().enumerate() {
            self.pair_statement.execute(params![exercise_id, position, pair.target, pair.source, pair.source_literal, pair.clip_key])?;
        }

        Ok(())
    }
}

fn exercise_columns(exercise: &Exercise) -> ExerciseColumns<'_> {
    match exercise {
        Exercise::MultipleChoice(multiple_choice) => ExerciseColumns {
            prompt: Some(&multiple_choice.prompt),
            correct_choice_index: Some(multiple_choice.answer_index),
            hint_source: None,
        },
        Exercise::FillBlank(fill_blank) => ExerciseColumns {
            prompt: Some(&fill_blank.prompt_target),
            correct_choice_index: None,
            hint_source: Some(&fill_blank.hint_source),
        },
        Exercise::TranslateToTarget(translate) => ExerciseColumns {
            prompt: Some(&translate.prompt_source),
            correct_choice_index: None,
            hint_source: None,
        },
        Exercise::OrderWords(order_words) => ExerciseColumns {
            prompt: Some(&order_words.prompt_source),
            correct_choice_index: None,
            hint_source: None,
        },
        Exercise::ListeningChoice(listening) => ExerciseColumns {
            prompt: Some(&listening.audio_target),
            correct_choice_index: Some(listening.answer_index),
            hint_source: None,
        },
        Exercise::Dictation(dictation) => ExerciseColumns {
            prompt: Some(&dictation.audio_target),
            correct_choice_index: None,
            hint_source: Some(&dictation.hint_source),
        },
        Exercise::Match(matching) => ExerciseColumns {
            prompt: matching.prompt.as_deref(),
            correct_choice_index: None,
            hint_source: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use shared::course::{CourseOutline, UnitLesson};
    use shared::course_shard::{CourseShardContents, UnitShardContents, VoiceEngineKind};

    use super::*;
    use crate::test_courses;

    /* One recording per engine for every clip, with a hash derived from the key so each is distinct. */
    fn synthetic_audio_index(plan: &AudioPlan) -> ClipAudioIndex {
        plan.utterances
            .keys()
            .map(|clip_key| {
                let entries: Vec<ClipAudioEntry> = VoiceEngineKind::ALL
                    .into_iter()
                    .map(|engine| ClipAudioEntry {
                        clip_key: clip_key.clone(),
                        engine,
                        audio_sha256: shared::artifact::sha256_hex(format!("{clip_key}{}", engine.code()).as_bytes()),
                        audio_size_bytes: 1_000,
                    })
                    .collect();

                (clip_key.clone(), entries)
            })
            .collect()
    }

    #[test]
    fn every_unit_round_trips_through_its_shards() {
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::read_keyed_courses(&mut plan).remove(0);
        let audio_index: ClipAudioIndex = synthetic_audio_index(&plan);
        let clip_catalog: ClipCatalog = ClipCatalog {
            plan: &plan,
            audio_index: &audio_index,
        };
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let course_path: PathBuf = directory.path().join("course.sqlite");

        write_course_shard(&course_path, &course, &clip_catalog).unwrap();

        let course_contents: CourseShardContents = shared::course_shard::read_course_shard(&fs::read(&course_path).unwrap()).unwrap();
        let expected_outline: CourseOutline = course.outline();

        assert_eq!(course_contents.outline, expected_outline);
        assert!(!course_contents.clip_audio.is_empty());

        for unit in &course.units {
            let unit_path: PathBuf = directory.path().join(format!("{}.sqlite", unit.id));

            write_unit_shard(&unit_path, unit, &clip_catalog).unwrap();

            let unit_contents: UnitShardContents = shared::course_shard::read_unit_shard(&fs::read(&unit_path).unwrap()).unwrap();
            let expected_lesson: UnitLesson = unit.lesson();

            assert_eq!(unit_contents.lesson, expected_lesson, "unit {}", unit.id);

            let reassembled: Unit = course_contents.outline.find_unit(&unit.id).unwrap().with_lesson(unit_contents.lesson);

            assert_eq!(&reassembled, unit);
        }
    }

    #[test]
    fn a_unit_shard_carries_every_engine_recording_of_its_clips() {
        let mut plan: AudioPlan = AudioPlan::new();
        let course: Course = test_courses::read_keyed_courses(&mut plan).remove(0);
        let audio_index: ClipAudioIndex = synthetic_audio_index(&plan);
        let clip_catalog: ClipCatalog = ClipCatalog {
            plan: &plan,
            audio_index: &audio_index,
        };
        let unit: &Unit = &course.units[0];
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let unit_path: PathBuf = directory.path().join("unit.sqlite");

        write_unit_shard(&unit_path, unit, &clip_catalog).unwrap();

        let unit_contents: UnitShardContents = shared::course_shard::read_unit_shard(&fs::read(&unit_path).unwrap()).unwrap();
        let expected_clip_count: usize = lesson_clip_keys(unit).len();

        assert_eq!(unit_contents.clip_audio.len(), expected_clip_count * VoiceEngineKind::ALL.len());
    }
}
