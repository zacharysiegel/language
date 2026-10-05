use std::collections::HashSet;
use std::fmt;

use crate::course::{
    CultureNote, CurriculumUnit, Dialogue, DictationExercise, Exercise, ExerciseKind, FillBlankExercise, GrammarNote,
    ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise, OrderWordsExercise, PhrasePair, Reading, SpeakerGenderKind,
    TranslateToPortugueseExercise, Unit, VocabularyItem,
};

const MINIMUM_OBJECTIVES: usize = 3;
const MINIMUM_VOCABULARY: usize = 18;
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

const MINIMUM_EXERCISES_BY_KIND: [(ExerciseKind, usize); 7] = [
    (ExerciseKind::MultipleChoice, 5),
    (ExerciseKind::FillBlank, 5),
    (ExerciseKind::TranslateToPortuguese, 3),
    (ExerciseKind::OrderWords, 3),
    (ExerciseKind::ListeningChoice, 3),
    (ExerciseKind::Dictation, 2),
    (ExerciseKind::Match, 1),
];

const MARKUP_CHARACTERS: &str = "*_#<>[]{}|\\";
const DASH_CHARACTERS: &str = "—–";
const FILL_BLANK_FORBIDDEN_ANSWER_CHARACTERS: &str = ".!?,";

/* Stems match any word beginning with them; the remaining lists match whole words or whole phrases. */
const EUROPEAN_PORTUGUESE_STEMS: [&str; 12] = [
    "autocarro",
    "comboio",
    "telemóvel",
    "telemovel",
    "telemóveis",
    "telemoveis",
    "rapariga",
    "frigorífico",
    "frigorifico",
    "miúdo",
    "miudo",
    "machimbombo",
];
const EUROPEAN_PORTUGUESE_WORDS: [&str; 7] = ["ecrã", "ecra", "equipa", "equipas", "facto", "factos", "estás"];
const EUROPEAN_PORTUGUESE_PHRASES: [&str; 6] = [
    "pequeno almoço",
    "pequeno almoco",
    "casa de banho",
    "tu és",
    "bilhete de identidade",
    "passadeira",
];
const EUROPEAN_PROGRESSIVE_AUXILIARIES: [&str; 2] = ["estou", "estamos"];
const INFINITIVE_ENDINGS: [&str; 4] = ["ar", "er", "ir", "or"];
const INFINITIVE_LOOKALIKES: [&str; 1] = ["favor"];

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

    fn check_spoken_portuguese(&mut self, location: &str, text: &str) {
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
        if contains_european_portuguese(text) {
            self.error(location, format!("European Portuguese form in spoken text: {text}"));
        }
    }

    fn check_pair(&mut self, location: &str, pair: &PhrasePair) {
        self.check_spoken_portuguese(&format!("{location}.pt"), &pair.pt);
        self.require_text(&format!("{location}.en"), "en", &pair.en);
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
            self.error(location, "answer must be an index into choices");
        }
    }
}

fn contains_european_portuguese(text: &str) -> bool {
    let lowercase_text: String = text.to_lowercase();
    let tokens: Vec<&str> = lowercase_text
        .split(|character: char| !character.is_alphabetic())
        .filter(|token| !token.is_empty())
        .collect();

    let stem_found: bool = tokens
        .iter()
        .any(|token| EUROPEAN_PORTUGUESE_STEMS.iter().any(|stem| token.starts_with(stem)));
    let word_found: bool = tokens.iter().any(|token| EUROPEAN_PORTUGUESE_WORDS.contains(token));
    let padded_tokens: String = format!(" {} ", tokens.join(" "));
    let phrase_found: bool = EUROPEAN_PORTUGUESE_PHRASES
        .iter()
        .any(|phrase| padded_tokens.contains(&format!(" {phrase} ")));

    stem_found || word_found || phrase_found || contains_european_progressive(&tokens)
}

fn contains_european_progressive(tokens: &[&str]) -> bool {
    tokens.windows(3).any(|window| {
        let [auxiliary, preposition, verb]: [&str; 3] = [window[0], window[1], window[2]];
        let is_infinitive: bool = verb.chars().count() > 3
            && INFINITIVE_ENDINGS.iter().any(|ending| verb.ends_with(ending))
            && !INFINITIVE_LOOKALIKES.contains(&verb);

        EUROPEAN_PROGRESSIVE_AUXILIARIES.contains(&auxiliary) && preposition == "a" && is_infinitive
    })
}

fn check_multiple_choice(collector: &mut IssueCollector, location: &str, exercise: &MultipleChoiceExercise) {
    collector.require_text(&format!("{location}.prompt"), "prompt", &exercise.prompt);
    collector.check_choices(location, 3, 5, &exercise.choices, exercise.answer);
}

fn check_fill_blank(collector: &mut IssueCollector, location: &str, exercise: &FillBlankExercise) {
    collector.require_text(&format!("{location}.hint_en"), "hint_en", &exercise.hint_en);

    let blank_count: usize = exercise.prompt_pt.matches(FillBlankExercise::BLANK_MARKER).count();

    if blank_count != 1 {
        collector.error(location, format!("prompt_pt must contain exactly one ___ (found {blank_count})"));
    }
    if exercise.answer.is_empty() || exercise.answer.iter().any(|accepted| accepted.trim().is_empty()) {
        collector.error(location, "answer must be a non-empty array of strings");
    }

    let punctuated: bool = exercise
        .answer
        .iter()
        .any(|accepted| accepted.contains(|character: char| FILL_BLANK_FORBIDDEN_ANSWER_CHARACTERS.contains(character)));

    if punctuated {
        collector.error(location, "fill_blank answers must not contain punctuation");
    }
}

fn check_translate(collector: &mut IssueCollector, location: &str, exercise: &TranslateToPortugueseExercise) {
    collector.require_text(&format!("{location}.prompt_en"), "prompt_en", &exercise.prompt_en);

    if exercise.answers.is_empty() {
        collector.error(location, "answers must be a non-empty array");
    }

    for accepted in &exercise.answers {
        collector.check_spoken_portuguese(&format!("{location}.answers"), accepted);
    }
}

fn check_order_words(collector: &mut IssueCollector, location: &str, exercise: &OrderWordsExercise) {
    collector.require_text(&format!("{location}.prompt_en"), "prompt_en", &exercise.prompt_en);

    if exercise.words.len() < 3 || exercise.words.len() > 12 {
        collector.error(location, "words must be an array of 3 to 12 tokens in correct order");
        return;
    }

    for word in &exercise.words {
        if word.trim().is_empty() || word.contains(char::is_whitespace) {
            collector.error(location, format!("token must be a single word without spaces: \"{word}\""));
        }
    }

    collector.check_spoken_portuguese(&format!("{location}.words"), &exercise.sentence());

    let distractors_valid: bool = exercise.distractors.len() <= MAXIMUM_ORDER_WORDS_DISTRACTORS
        && exercise.distractors.iter().all(|distractor| !distractor.trim().is_empty());

    if !distractors_valid {
        collector.error(location, "distractors must be an array of at most 3 strings");
    }
}

fn check_listening_choice(collector: &mut IssueCollector, location: &str, exercise: &ListeningChoiceExercise) {
    collector.check_spoken_portuguese(&format!("{location}.audio_pt"), &exercise.audio_pt);
    collector.check_choices(location, 3, 4, &exercise.choices, exercise.answer);
}

fn check_dictation(collector: &mut IssueCollector, location: &str, exercise: &DictationExercise) {
    collector.check_spoken_portuguese(&format!("{location}.audio_pt"), &exercise.audio_pt);
    collector.require_text(&format!("{location}.hint_en"), "hint_en", &exercise.hint_en);
}

fn check_match(collector: &mut IssueCollector, location: &str, exercise: &MatchExercise) {
    if exercise.pairs.len() < 4 || exercise.pairs.len() > 6 {
        collector.error(location, "pairs must be an array of 4 to 6 items");
        return;
    }

    for (pair_index, pair) in exercise.pairs.iter().enumerate() {
        collector.check_pair(&format!("{location}.pairs[{pair_index}]"), pair);
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
        Exercise::TranslateToPortuguese(exercise) => {
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

        collector.check_spoken_portuguese(&format!("{location}.pt"), &item.pt);
        collector.require_text(&format!("{location}.en"), "en", &item.en);
        collector.check_pair(&format!("{location}.example"), &item.example);

        if let Some(note) = &item.note {
            collector.require_text(&format!("{location}.note"), "note", note);
        }
        if let Some(part_of_speech) = &item.pos {
            collector.require_text(&format!("{location}.pos"), "pos", part_of_speech);
        }

        let inserted: bool = seen_items.insert(item.pt.to_lowercase());

        if !inserted {
            collector.error(&location, format!("duplicate vocabulary item: {}", item.pt));
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
        collector.check_spoken_portuguese(&format!("{line_location}.pt"), &line.pt);
        collector.require_text(&format!("{line_location}.en"), "en", &line.en);

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

        if !seen_ids.insert(&dialogue.id) {
            collector.error(&location, format!("duplicate dialogue id: {}", dialogue.id));
        }

        check_dialogue(collector, &location, dialogue);
    }
}

fn check_grammar_note(collector: &mut IssueCollector, location: &str, note: &GrammarNote) {
    collector.require_text(&format!("{location}.title"), "title", &note.title);

    let explanation_characters: usize = note.explanation.chars().count();

    if collector.require_text(&format!("{location}.explanation"), "explanation", &note.explanation)
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

    if collector.require_text("culture.text", "text", &culture.text) && text_characters < MINIMUM_CULTURE_CHARACTERS {
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

fn check_readings(collector: &mut IssueCollector, unit: &Unit) {
    if unit.level.requires_readings() && unit.readings.is_empty() {
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

pub fn validate_unit(curriculum_unit: &CurriculumUnit, unit: &Unit) -> Vec<ValidationIssue> {
    let mut collector: IssueCollector = IssueCollector {
        unit_id: &curriculum_unit.id,
        issues: Vec::new(),
    };

    if unit.id != curriculum_unit.id {
        collector.error("id", format!("expected {}, found {}", curriculum_unit.id, unit.id));
    }
    if unit.level != curriculum_unit.level {
        collector.error("level", format!("expected {}", curriculum_unit.level.code()));
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
    check_readings(&mut collector, unit);
    check_exercises(&mut collector, &unit.exercises);

    collector.issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn european_portuguese_detects_stems_words_phrases_and_progressive() {
        assert!(contains_european_portuguese("Vou de autocarro."));
        assert!(contains_european_portuguese("O comboio chegou."));
        assert!(contains_european_portuguese("Onde fica a casa de banho?"));
        assert!(contains_european_portuguese("Estou a fazer o jantar."));
        assert!(contains_european_portuguese("Tu estás cansado?"));
    }

    #[test]
    fn european_portuguese_accepts_brazilian_forms() {
        assert!(!contains_european_portuguese("Vou de ônibus."));
        assert!(!contains_european_portuguese("Estou fazendo o jantar."));
        assert!(!contains_european_portuguese("Estou a favor da ideia."));
        assert!(!contains_european_portuguese("Onde fica o banheiro?"));
    }
}
