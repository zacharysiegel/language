use std::collections::HashSet;
use std::fmt;

use crate::course::{
    CultureNote, Curriculum, CurriculumUnit, Dialogue, DictationExercise, Exercise, ExerciseKind, FillBlankExercise, GrammarNote,
    GrammarTable, GroupedTable, ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise, OrderWordsExercise, PhrasePair,
    Reading, SpeakerGenderKind, TranslateToTargetExercise, Unit, VocabularyItem, grouped_table, target_language_rules,
};

const MINIMUM_OBJECTIVES: usize = 3;
const MINIMUM_VOCABULARY: usize = 12;
const MINIMUM_DIALOGUES: usize = 2;
const MINIMUM_DIALOGUE_LINES: usize = 6;
const MINIMUM_SPEAKERS: usize = 2;
const MAXIMUM_SPEAKERS: usize = 4;
const MINIMUM_GRAMMAR_NOTES: usize = 2;
const MINIMUM_GRAMMAR_EXPLANATION_CHARACTERS: usize = 250;
const MINIMUM_GRAMMAR_EXAMPLES: usize = 4;
const MINIMUM_CULTURE_CHARACTERS: usize = 200;
const MINIMUM_READING_PARAGRAPHS: usize = 2;
const MINIMUM_READING_QUESTIONS: usize = 2;
const MINIMUM_EXERCISES: usize = 22;
const MAXIMUM_SPOKEN_CHARACTERS: usize = 220;
const MAXIMUM_ORDER_WORDS_DISTRACTORS: usize = 3;
// A phone shows the two sides of a match in two narrow columns.
const MAXIMUM_MATCH_SIDE_CHARACTERS: usize = 35;

// A line of the phone label column in half letter widths, measured at the narrower iOS width.
const TABLE_FIELD_LABEL_LINE_HALF_WIDTHS: usize = 23;
const TABLE_FIELD_LABEL_LETTER_HALF_WIDTHS: usize = 2;
const TABLE_FIELD_LABEL_SPACE_HALF_WIDTHS: usize = 2;
const TABLE_FIELD_LABEL_NARROW_CHARACTER_HALF_WIDTHS: usize = 1;
const TABLE_FIELD_LABEL_NARROW_CHARACTERS: &str = "IÍií,.;:!?'()/";
const TABLE_FIELD_LABEL_MAXIMUM_LINES: usize = 2;

const MINIMUM_EXERCISES_BY_KIND: [(ExerciseKind, usize); 7] = [
    (ExerciseKind::MultipleChoice, 5),
    (ExerciseKind::FillBlank, 5),
    (ExerciseKind::TranslateToTarget, 3),
    (ExerciseKind::OrderWords, 3),
    (ExerciseKind::ListeningChoice, 3),
    (ExerciseKind::Dictation, 2),
    (ExerciseKind::Match, 1),
];

const MARKUP_CHARACTERS: &str = "*_#<>[]{}|\\";
const DASH_CHARACTERS: &str = "—–";
const FILL_BLANK_FORBIDDEN_ANSWER_CHARACTERS: &str = ".!?,";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeverityKind {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: SeverityKind,
    pub unit_id: String,
    pub location: String,
    pub message: String,
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}: {}", self.unit_id, self.location, self.message)
    }
}

struct IssueCollector<'a> {
    unit_id: &'a str,
    target_language: &'a str,
    issues: Vec<ValidationIssue>,
}

impl IssueCollector<'_> {
    fn error(&mut self, location: &str, message: impl Into<String>) {
        self.push(SeverityKind::Error, location, message.into());
    }

    fn warning(&mut self, location: &str, message: impl Into<String>) {
        self.push(SeverityKind::Warning, location, message.into());
    }

    fn push(&mut self, severity: SeverityKind, location: &str, message: String) {
        self.issues.push(ValidationIssue {
            severity,
            unit_id: self.unit_id.to_string(),
            location: location.to_string(),
            message,
        });
    }

    fn require_text(&mut self, location: &str, label: &str, text: &str) -> bool {
        let present: bool = !text.trim().is_empty();

        if !present {
            self.error(location, format!("{label} must be a non-empty string"));
        }

        present
    }

    fn check_spoken_target(&mut self, location: &str, text: &str) {
        if !self.require_text(location, "spoken text", text) {
            return;
        }

        if text.contains(['\n', '\t']) {
            self.error(location, "spoken text contains a newline or tab");
        }
        if text.contains(|character: char| MARKUP_CHARACTERS.contains(character)) {
            self.error(location, format!("spoken text contains markup characters: {text}"));
        }
        if text.contains('/') {
            self.error(location, format!("spoken text contains a slash (text-to-speech reads it aloud): {text}"));
        }
        if text.contains('(') {
            self.error(location, format!("spoken text contains parentheses: {text}"));
        }
        if text.contains(FillBlankExercise::BLANK_MARKER) {
            self.error(location, format!("spoken text contains a blank: {text}"));
        }
        if text.contains(|character: char| DASH_CHARACTERS.contains(character)) {
            self.error(location, format!("spoken text contains a dash character: {text}"));
        }

        let character_count: usize = text.chars().count();

        if character_count > MAXIMUM_SPOKEN_CHARACTERS {
            self.error(location, format!("spoken text is too long ({character_count} characters)"));
        }
        if let Some(message) = target_language_rules::target_language_issue(self.target_language, text) {
            self.error(location, message);
        }
    }

    fn check_pair(&mut self, location: &str, pair: &PhrasePair) {
        self.check_spoken_target(&format!("{location}.target"), &pair.target);
        self.require_text(&format!("{location}.source"), "source", &pair.source);
        self.check_literal(location, &pair.source_literal, &pair.source);
    }

    fn check_literal(&mut self, location: &str, literal: &Option<String>, natural: &str) {
        let Some(literal) = literal
        else {
            return;
        };

        self.require_text(&format!("{location}.source_literal"), "source_literal", literal);

        if literal.trim().eq_ignore_ascii_case(natural.trim()) {
            self.warning(&format!("{location}.source_literal"), "the literal rendering repeats the natural translation; omit it");
        }
    }

    fn check_explanation(&mut self, location: &str, explanation: &Option<String>) {
        if let Some(explanation) = explanation {
            self.require_text(&format!("{location}.explanation"), "explanation", explanation);
        }
    }

    fn check_choices(&mut self, location: &str, minimum: usize, maximum: usize, choices: &[String], answer: usize) {
        if choices.len() < minimum || choices.len() > maximum {
            self.error(location, format!("choices must be an array of {minimum} to {maximum} strings"));
            return;
        }

        for choice in choices {
            self.require_text(location, "choice", choice);
        }

        let distinct_choices: HashSet<&String> = choices.iter().collect();

        if distinct_choices.len() != choices.len() {
            self.error(location, "choices must be unique");
        }
        if answer >= choices.len() {
            self.error(location, "answer_index must be an index into choices");
        }
    }
}

fn check_multiple_choice(collector: &mut IssueCollector, location: &str, exercise: &MultipleChoiceExercise) {
    collector.require_text(&format!("{location}.prompt"), "prompt", &exercise.prompt);
    collector.check_choices(location, 3, 5, &exercise.choices, exercise.answer_index);
}

fn check_fill_blank(collector: &mut IssueCollector, location: &str, exercise: &FillBlankExercise) {
    collector.require_text(&format!("{location}.hint_source"), "hint_source", &exercise.hint_source);

    let blank_count: usize = exercise.prompt_target.matches(FillBlankExercise::BLANK_MARKER).count();

    if blank_count != 1 {
        collector.error(location, format!("prompt_target must contain exactly one ___ (found {blank_count})"));
    }
    if exercise.answers.is_empty() || exercise.answers.iter().any(|accepted| accepted.trim().is_empty()) {
        collector.error(location, "answers must be a non-empty array of strings");
    }

    let punctuated: bool = exercise
        .answers
        .iter()
        .any(|accepted| accepted.contains(|character: char| FILL_BLANK_FORBIDDEN_ANSWER_CHARACTERS.contains(character)));

    if punctuated {
        collector.error(location, "fill_blank answers must not contain punctuation");
    }

    let has_single_blank_and_answer: bool = blank_count == 1 && !exercise.answers.is_empty();

    if has_single_blank_and_answer {
        collector.check_spoken_target(&format!("{location}.completed_sentence"), &exercise.completed_sentence());
    }
}

fn check_translate(collector: &mut IssueCollector, location: &str, exercise: &TranslateToTargetExercise) {
    collector.require_text(&format!("{location}.prompt_source"), "prompt_source", &exercise.prompt_source);

    if exercise.answers.is_empty() {
        collector.error(location, "answers must be a non-empty array");
    }

    for accepted in &exercise.answers {
        collector.check_spoken_target(&format!("{location}.answers"), accepted);
    }
}

fn check_order_words(collector: &mut IssueCollector, location: &str, exercise: &OrderWordsExercise) {
    collector.require_text(&format!("{location}.prompt_source"), "prompt_source", &exercise.prompt_source);

    if exercise.words.len() < 3 || exercise.words.len() > 12 {
        collector.error(location, "words must be an array of 3 to 12 tokens in correct order");
        return;
    }

    for word in &exercise.words {
        if word.trim().is_empty() || word.contains(char::is_whitespace) {
            collector.error(location, format!("token must be a single word without spaces: \"{word}\""));
        }
    }

    collector.check_spoken_target(&format!("{location}.words"), &exercise.sentence());

    let distractors_valid: bool = exercise.distractors.len() <= MAXIMUM_ORDER_WORDS_DISTRACTORS
        && exercise.distractors.iter().all(|distractor| !distractor.trim().is_empty());

    if !distractors_valid {
        collector.error(location, "distractors must be an array of at most 3 strings");
    }
}

fn check_listening_choice(collector: &mut IssueCollector, location: &str, exercise: &ListeningChoiceExercise) {
    collector.check_spoken_target(&format!("{location}.audio_target"), &exercise.audio_target);
    collector.check_choices(location, 3, 4, &exercise.choices, exercise.answer_index);
}

fn check_dictation(collector: &mut IssueCollector, location: &str, exercise: &DictationExercise) {
    collector.check_spoken_target(&format!("{location}.audio_target"), &exercise.audio_target);
    collector.require_text(&format!("{location}.hint_source"), "hint_source", &exercise.hint_source);
}

fn check_match(collector: &mut IssueCollector, location: &str, exercise: &MatchExercise) {
    if exercise.pairs.len() < 4 || exercise.pairs.len() > 6 {
        collector.error(location, "pairs must be an array of 4 to 6 items");
        return;
    }

    for (pair_index, pair) in exercise.pairs.iter().enumerate() {
        let pair_location: String = format!("{location}.pairs[{pair_index}]");

        collector.check_pair(&pair_location, pair);
        check_match_side_length(collector, &format!("{pair_location}.target"), &pair.target);
        check_match_side_length(collector, &format!("{pair_location}.source"), &pair.source);
    }
}

fn check_match_side_length(collector: &mut IssueCollector, location: &str, side: &str) {
    let character_count: usize = side.chars().count();

    if character_count > MAXIMUM_MATCH_SIDE_CHARACTERS {
        collector.error(
            location,
            format!("match side is too long for a phone ({character_count} characters, at most {MAXIMUM_MATCH_SIDE_CHARACTERS})"),
        );
    }
}

fn check_exercise(collector: &mut IssueCollector, location: &str, exercise: &Exercise) {
    match exercise {
        Exercise::MultipleChoice(exercise) => {
            collector.check_explanation(location, &exercise.explanation);
            check_multiple_choice(collector, location, exercise);
        },
        Exercise::FillBlank(exercise) => {
            collector.check_explanation(location, &exercise.explanation);
            check_fill_blank(collector, location, exercise);
        },
        Exercise::TranslateToTarget(exercise) => {
            collector.check_explanation(location, &exercise.explanation);
            check_translate(collector, location, exercise);
        },
        Exercise::OrderWords(exercise) => {
            collector.check_explanation(location, &exercise.explanation);
            check_order_words(collector, location, exercise);
        },
        Exercise::ListeningChoice(exercise) => {
            collector.check_explanation(location, &exercise.explanation);
            check_listening_choice(collector, location, exercise);
        },
        Exercise::Dictation(exercise) => {
            collector.check_explanation(location, &exercise.explanation);
            check_dictation(collector, location, exercise);
        },
        Exercise::Match(exercise) => {
            collector.check_explanation(location, &exercise.explanation);
            check_match(collector, location, exercise);
        },
    }
}

fn check_multiple_choice_only(collector: &mut IssueCollector, location: &str, exercises: &[Exercise]) {
    for (exercise_index, exercise) in exercises.iter().enumerate() {
        let exercise_location: String = format!("{location}[{exercise_index}]");

        check_exercise(collector, &exercise_location, exercise);

        if exercise.kind() != ExerciseKind::MultipleChoice {
            collector.error(&exercise_location, "type must be one of multiple_choice");
        }
    }
}

fn check_vocabulary(collector: &mut IssueCollector, vocabulary: &[VocabularyItem]) {
    if vocabulary.len() < MINIMUM_VOCABULARY {
        collector.error("vocabulary", format!("must be an array of at least {MINIMUM_VOCABULARY} items"));
    }

    let mut seen_items: HashSet<String> = HashSet::new();

    for (item_index, item) in vocabulary.iter().enumerate() {
        let location: String = format!("vocabulary[{item_index}]");

        collector.check_spoken_target(&format!("{location}.target"), &item.target);
        collector.require_text(&format!("{location}.source"), "source", &item.source);
        collector.check_literal(&location, &item.source_literal, &item.source);
        collector.check_pair(&format!("{location}.example"), &item.example);

        if let Some(note) = &item.note {
            collector.require_text(&format!("{location}.note"), "note", note);
        }
        if let Some(part_of_speech) = &item.part_of_speech {
            collector.require_text(&format!("{location}.pos"), "pos", part_of_speech);
        }

        let inserted: bool = seen_items.insert(item.target.to_lowercase());

        if !inserted {
            collector.error(&location, format!("duplicate vocabulary item: {}", item.target));
        }
    }
}

fn check_dialogue(collector: &mut IssueCollector, location: &str, dialogue: &Dialogue) {
    collector.require_text(&format!("{location}.id"), "id", &dialogue.id);
    collector.require_text(&format!("{location}.title"), "title", &dialogue.title);
    collector.require_text(&format!("{location}.context"), "context", &dialogue.context);

    if dialogue.lines.len() < MINIMUM_DIALOGUE_LINES {
        collector.error(location, format!("lines must be an array of at least {MINIMUM_DIALOGUE_LINES} lines"));
        return;
    }

    let mut speaker_genders: Vec<(&str, SpeakerGenderKind)> = Vec::new();

    for (line_index, line) in dialogue.lines.iter().enumerate() {
        let line_location: String = format!("{location}.lines[{line_index}]");

        collector.require_text(&format!("{line_location}.speaker"), "speaker", &line.speaker);
        collector.check_spoken_target(&format!("{line_location}.target"), &line.target);
        collector.require_text(&format!("{line_location}.source"), "source", &line.source);
        collector.check_literal(&line_location, &line.source_literal, &line.source);

        let known_gender: Option<SpeakerGenderKind> = speaker_genders
            .iter()
            .find(|(speaker, _)| *speaker == line.speaker)
            .map(|(_, gender)| *gender);

        match known_gender {
            Some(gender) if gender != line.gender => {
                collector.error(&line_location, format!("speaker {} changes gender within a dialogue", line.speaker));
            },
            Some(_) => {},
            None => speaker_genders.push((&line.speaker, line.gender)),
        }
    }

    if speaker_genders.len() < MINIMUM_SPEAKERS {
        collector.error(location, format!("needs at least {MINIMUM_SPEAKERS} distinct speakers"));
    }
    if speaker_genders.len() > MAXIMUM_SPEAKERS {
        collector.error(location, format!("at most {MAXIMUM_SPEAKERS} distinct speakers"));
    }
}

fn check_dialogues(collector: &mut IssueCollector, dialogues: &[Dialogue]) {
    if dialogues.len() < MINIMUM_DIALOGUES {
        collector.error("dialogues", format!("must be an array of at least {MINIMUM_DIALOGUES} dialogues"));
        return;
    }

    let mut seen_ids: HashSet<&str> = HashSet::new();

    for (dialogue_index, dialogue) in dialogues.iter().enumerate() {
        let location: String = format!("dialogues[{dialogue_index}]");

        let first_use_of_id: bool = seen_ids.insert(&dialogue.id);

        if !first_use_of_id {
            collector.error(&location, format!("duplicate dialogue id: {}", dialogue.id));
        }

        check_dialogue(collector, &location, dialogue);
    }
}

fn check_grammar_note(collector: &mut IssueCollector, location: &str, note: &GrammarNote) {
    collector.require_text(&format!("{location}.title"), "title", &note.title);

    let explanation_characters: usize = note.explanation.chars().count();

    let explanation_present: bool = collector.require_text(&format!("{location}.explanation"), "explanation", &note.explanation);

    if explanation_present
        && explanation_characters < MINIMUM_GRAMMAR_EXPLANATION_CHARACTERS
    {
        collector.error(
            location,
            format!("explanation is too short ({explanation_characters} characters, need {MINIMUM_GRAMMAR_EXPLANATION_CHARACTERS})"),
        );
    }

    if note.examples.len() < MINIMUM_GRAMMAR_EXAMPLES {
        collector.error(location, format!("examples must be an array of at least {MINIMUM_GRAMMAR_EXAMPLES} items"));
    }

    for (example_index, example) in note.examples.iter().enumerate() {
        collector.check_pair(&format!("{location}.examples[{example_index}]"), example);
    }

    if let Some(table) = &note.table {
        let malformed: bool = table.rows.is_empty()
            || table.headers.is_empty()
            || table.rows.iter().any(|row| row.len() != table.headers.len());

        if malformed {
            collector.error(&format!("{location}.table"), "table needs headers and rows as wide as the headers");
            return;
        }

        check_table_field_labels(collector, &format!("{location}.table"), table);
    }
}

/// The lines a header takes as the label beside a value in a phone layout, wrapping between words.
fn table_field_label_line_count(header: &str) -> usize {
    let mut line_count: usize = 0;
    let mut current_line_half_widths: usize = 0;

    for word in header.split_whitespace() {
        let word_half_widths: usize = table_field_label_half_widths(word);
        let widened_line_half_widths: usize = current_line_half_widths + TABLE_FIELD_LABEL_SPACE_HALF_WIDTHS + word_half_widths;

        if line_count > 0 && widened_line_half_widths <= TABLE_FIELD_LABEL_LINE_HALF_WIDTHS {
            current_line_half_widths = widened_line_half_widths;
            continue;
        }

        // A word wider than a line overflows or breaks, so it counts every line it fills.
        let word_line_count: usize = word_half_widths.div_ceil(TABLE_FIELD_LABEL_LINE_HALF_WIDTHS);

        line_count += word_line_count;
        current_line_half_widths = word_half_widths - (word_line_count - 1) * TABLE_FIELD_LABEL_LINE_HALF_WIDTHS;
    }

    line_count
}

fn table_field_label_half_widths(word: &str) -> usize {
    word.chars()
        .map(|character| {
            if TABLE_FIELD_LABEL_NARROW_CHARACTERS.contains(character) {
                TABLE_FIELD_LABEL_NARROW_CHARACTER_HALF_WIDTHS
            } else {
                TABLE_FIELD_LABEL_LETTER_HALF_WIDTHS
            }
        })
        .sum()
}

/// Checks the headers set as labels beside their values on a phone; the group and lead headers span the full width.
fn check_table_field_labels(collector: &mut IssueCollector, location: &str, table: &GrammarTable) {
    let lead_column: usize = match grouped_table::grouped_table(table) {
        GroupedTable::Grouped { .. } => 1,
        GroupedTable::Stacked { .. } => 0,
    };

    for (column, header) in table.headers.iter().enumerate().skip(lead_column + 1) {
        let line_count: usize = table_field_label_line_count(header);
        let header_location: String = format!("{location}.headers[{column}]");

        if line_count > TABLE_FIELD_LABEL_MAXIMUM_LINES {
            collector.error(
                &header_location,
                format!("header \"{header}\" takes {line_count} lines beside its value on a phone (at most {TABLE_FIELD_LABEL_MAXIMUM_LINES})"),
            );
        } else if line_count > 1 {
            collector.warning(&header_location, format!("header \"{header}\" takes {line_count} lines beside its value on a phone"));
        }
    }
}

fn check_grammar(collector: &mut IssueCollector, grammar: &[GrammarNote]) {
    if grammar.len() < MINIMUM_GRAMMAR_NOTES {
        collector.error("grammar", format!("must be an array of at least {MINIMUM_GRAMMAR_NOTES} notes"));
        return;
    }

    for (note_index, note) in grammar.iter().enumerate() {
        check_grammar_note(collector, &format!("grammar[{note_index}]"), note);
    }
}

fn check_culture(collector: &mut IssueCollector, culture: &CultureNote) {
    collector.require_text("culture.title", "title", &culture.title);

    let text_characters: usize = culture.text.chars().count();

    let text_present: bool = collector.require_text("culture.text", "text", &culture.text);

    if text_present && text_characters < MINIMUM_CULTURE_CHARACTERS {
        collector.error("culture.text", format!("too short (need {MINIMUM_CULTURE_CHARACTERS} characters)"));
    }
}

fn check_reading(collector: &mut IssueCollector, location: &str, reading: &Reading) {
    collector.require_text(&format!("{location}.id"), "id", &reading.id);
    collector.require_text(&format!("{location}.title"), "title", &reading.title);

    if reading.paragraphs.len() < MINIMUM_READING_PARAGRAPHS {
        collector.error(location, format!("paragraphs must be an array of at least {MINIMUM_READING_PARAGRAPHS} items"));
    }

    for (paragraph_index, paragraph) in reading.paragraphs.iter().enumerate() {
        collector.check_pair(&format!("{location}.paragraphs[{paragraph_index}]"), paragraph);
    }

    if reading.questions.len() < MINIMUM_READING_QUESTIONS {
        collector.error(
            location,
            format!("questions must be an array of at least {MINIMUM_READING_QUESTIONS} multiple_choice exercises"),
        );
    }

    check_multiple_choice_only(collector, &format!("{location}.questions"), &reading.questions);
}

fn check_readings(collector: &mut IssueCollector, unit: &Unit, requires_readings: bool) {
    if requires_readings && unit.readings.is_empty() {
        collector.error("readings", "required (at least 1) for B1 and B2");
    }

    for (reading_index, reading) in unit.readings.iter().enumerate() {
        check_reading(collector, &format!("readings[{reading_index}]"), reading);
    }
}

fn check_exercises(collector: &mut IssueCollector, exercises: &[Exercise]) {
    for (exercise_index, exercise) in exercises.iter().enumerate() {
        check_exercise(collector, &format!("exercises[{exercise_index}]"), exercise);
    }

    if exercises.len() < MINIMUM_EXERCISES {
        collector.error("exercises", format!("need at least {MINIMUM_EXERCISES} (found {})", exercises.len()));
    }

    for (kind, minimum) in MINIMUM_EXERCISES_BY_KIND {
        let count: usize = exercises.iter().filter(|exercise| exercise.kind() == kind).count();

        if count < minimum {
            collector.error("exercises", format!("need at least {minimum} of type {} (found {count})", kind.code()));
        }
    }

    let prompts: Vec<String> = exercises.iter().map(Exercise::prompt_text).collect();
    let distinct_prompts: HashSet<&String> = prompts.iter().collect();

    if distinct_prompts.len() != prompts.len() {
        collector.warning("exercises", "contains duplicate prompts");
    }
}

pub fn validate_unit(curriculum: &Curriculum, curriculum_unit: &CurriculumUnit, unit: &Unit) -> Vec<ValidationIssue> {
    let mut collector: IssueCollector = IssueCollector {
        unit_id: &curriculum_unit.id,
        target_language: &curriculum.target_language,
        issues: Vec::new(),
    };
    let requires_readings: bool = match curriculum.levels.iter().find(|level| level.code == unit.level) {
        Some(level) => level.requires_readings,
        None => {
            collector.error("level", format!("the curriculum has no level {}", unit.level));
            false
        },
    };

    if unit.id != curriculum_unit.id {
        collector.error("id", format!("expected {}, found {}", curriculum_unit.id, unit.id));
    }
    if unit.level != curriculum_unit.level {
        collector.error("level", format!("expected {}", curriculum_unit.level));
    }
    if unit.order != curriculum_unit.order {
        collector.error("order", format!("expected {}", curriculum_unit.order));
    }

    collector.require_text("title", "title", &unit.title);

    let objectives_valid: bool =
        unit.objectives.len() >= MINIMUM_OBJECTIVES && unit.objectives.iter().all(|objective| !objective.trim().is_empty());

    if !objectives_valid {
        collector.error("objectives", format!("must be an array of at least {MINIMUM_OBJECTIVES} strings"));
    }

    check_vocabulary(&mut collector, &unit.vocabulary);
    check_dialogues(&mut collector, &unit.dialogues);
    check_grammar(&mut collector, &unit.grammar);
    check_culture(&mut collector, &unit.culture);
    check_readings(&mut collector, unit, requires_readings);
    check_exercises(&mut collector, &unit.exercises);

    collector.issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_field_label_line_count_wraps_between_words_within_a_line() {
        assert_eq!(table_field_label_line_count("Sound"), 1);
        assert_eq!(table_field_label_line_count("Accent when"), 1);
        assert_eq!(table_field_label_line_count("English sound"), 2);
        assert_eq!(table_field_label_line_count("você, ele, ela, a gente"), 2);
        assert_eq!(table_field_label_line_count("Sounds like, for an English speaker"), 4);
    }

    #[test]
    fn table_field_label_line_count_counts_narrow_characters_as_half_a_letter() {
        assert_eq!(table_field_label_line_count("Subjunctive?"), 1);
        assert_eq!(table_field_label_line_count("Fact (indicative)"), 2);
    }

    #[test]
    fn table_field_label_line_count_counts_every_line_a_long_word_fills() {
        assert_eq!(table_field_label_line_count("Comprehensibility"), 2);
        assert_eq!(table_field_label_line_count("No commitment (subjunctive)"), 3);
    }

    #[test]
    fn check_table_field_labels_skips_the_lead_column_and_grades_by_line_count() {
        let table: GrammarTable = GrammarTable {
            headers: vec![
                "A long lead header that spans the width".to_string(),
                "Sound".to_string(),
                "English sound".to_string(),
                "Sounds like, for an English speaker".to_string(),
            ],
            rows: vec![vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()]],
        };
        let mut collector: IssueCollector = IssueCollector {
            unit_id: "unit",
            target_language: "pt-BR",
            issues: Vec::new(),
        };

        check_table_field_labels(&mut collector, "grammar[0].table", &table);

        let graded_locations: Vec<(SeverityKind, &str)> =
            collector.issues.iter().map(|issue| (issue.severity, issue.location.as_str())).collect();

        assert_eq!(
            graded_locations,
            vec![
                (SeverityKind::Warning, "grammar[0].table.headers[2]"),
                (SeverityKind::Error, "grammar[0].table.headers[3]"),
            ],
        );
    }

    #[test]
    fn check_match_side_length_rejects_a_side_longer_than_the_maximum() {
        let fitting_side: String = "a".repeat(MAXIMUM_MATCH_SIDE_CHARACTERS);
        let long_side: String = "ã".repeat(MAXIMUM_MATCH_SIDE_CHARACTERS + 1);
        let mut collector: IssueCollector = IssueCollector {
            unit_id: "unit",
            target_language: "pt-BR",
            issues: Vec::new(),
        };

        check_match_side_length(&mut collector, "exercises[0].pairs[0].source", &fitting_side);
        check_match_side_length(&mut collector, "exercises[0].pairs[1].source", &long_side);

        let graded_locations: Vec<(SeverityKind, &str)> =
            collector.issues.iter().map(|issue| (issue.severity, issue.location.as_str())).collect();

        assert_eq!(graded_locations, vec![(SeverityKind::Error, "exercises[0].pairs[1].source")]);
    }
}
