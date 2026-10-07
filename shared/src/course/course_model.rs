use serde::{Deserialize, Serialize};

use crate::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LevelKind {
    A1,
    A2,
    B1,
    B2,
}

impl LevelKind {
    pub fn code(self) -> &'static str {
        match self {
            LevelKind::A1 => "A1",
            LevelKind::A2 => "A2",
            LevelKind::B1 => "B1",
            LevelKind::B2 => "B2",
        }
    }

    pub fn requires_readings(self) -> bool {
        matches!(self, LevelKind::B1 | LevelKind::B2)
    }
}

impl TryFrom<&str> for LevelKind {
    type Error = AppError;

    fn try_from(code: &str) -> Result<LevelKind, AppError> {
        match code {
            "A1" => Ok(LevelKind::A1),
            "A2" => Ok(LevelKind::A2),
            "B1" => Ok(LevelKind::B1),
            "B2" => Ok(LevelKind::B2),
            unrecognized => Err(AppError::from(format!("unknown level code; [code={unrecognized}]"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeakerGenderKind {
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "m")]
    Male,
}

impl SpeakerGenderKind {
    pub fn code(self) -> &'static str {
        match self {
            SpeakerGenderKind::Female => "f",
            SpeakerGenderKind::Male => "m",
        }
    }
}

impl TryFrom<&str> for SpeakerGenderKind {
    type Error = AppError;

    fn try_from(code: &str) -> Result<SpeakerGenderKind, AppError> {
        match code {
            "f" => Ok(SpeakerGenderKind::Female),
            "m" => Ok(SpeakerGenderKind::Male),
            unrecognized => Err(AppError::from(format!("unknown speaker gender code; [code={unrecognized}]"))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelSummary {
    pub code: LevelKind,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Course {
    pub title: String,
    pub levels: Vec<LevelSummary>,
    pub units: Vec<Unit>,
}

impl Course {
    pub fn find_unit(&self, unit_id: &str) -> Option<&Unit> {
        self.units.iter().find(|unit| unit.id == unit_id)
    }

    pub fn find_vocabulary(&self, card_id: &str) -> Option<(&Unit, &VocabularyItem)> {
        let (unit_id, item_index): (&str, usize) = parse_vocabulary_card_id(card_id)?;
        let unit: &Unit = self.find_unit(unit_id)?;
        let item: &VocabularyItem = unit.vocabulary.get(item_index)?;

        Some((unit, item))
    }

    pub fn units_at_level(&self, level: LevelKind) -> impl Iterator<Item = &Unit> {
        self.units.iter().filter(move |unit| unit.level == level)
    }
}

pub fn vocabulary_card_id(unit_id: &str, item_index: usize) -> String {
    format!("{unit_id}:{item_index}")
}

pub fn parse_vocabulary_card_id(card_id: &str) -> Option<(&str, usize)> {
    let (unit_id, item_index_text): (&str, &str) = card_id.rsplit_once(':')?;
    let item_index: usize = item_index_text.parse().ok()?;

    Some((unit_id, item_index))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Unit {
    pub id: String,
    pub level: LevelKind,
    pub order: u32,
    pub title: String,
    pub objectives: Vec<String>,
    pub vocabulary: Vec<VocabularyItem>,
    pub dialogues: Vec<Dialogue>,
    pub grammar: Vec<GrammarNote>,
    pub culture: CultureNote,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<Reading>,
    pub exercises: Vec<Exercise>,
}

/// Portuguese text with its English translation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhrasePair {
    pub pt: String,
    pub en: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VocabularyItem {
    pub pt: String,
    pub en: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub example: PhrasePair,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dialogue {
    pub id: String,
    pub title: String,
    pub context: String,
    pub lines: Vec<DialogueLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DialogueLine {
    pub speaker: String,
    pub gender: SpeakerGenderKind,
    pub pt: String,
    pub en: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarNote {
    pub title: String,
    pub explanation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<GrammarTable>,
    pub examples: Vec<PhrasePair>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CultureNote {
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reading {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<PhrasePair>,
    pub questions: Vec<Exercise>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExerciseKind {
    MultipleChoice,
    FillBlank,
    TranslateToPortuguese,
    OrderWords,
    ListeningChoice,
    Dictation,
    Match,
}

impl ExerciseKind {
    pub const ALL: [ExerciseKind; 7] = [
        ExerciseKind::MultipleChoice,
        ExerciseKind::FillBlank,
        ExerciseKind::TranslateToPortuguese,
        ExerciseKind::OrderWords,
        ExerciseKind::ListeningChoice,
        ExerciseKind::Dictation,
        ExerciseKind::Match,
    ];

    pub fn code(self) -> &'static str {
        match self {
            ExerciseKind::MultipleChoice => "multiple_choice",
            ExerciseKind::FillBlank => "fill_blank",
            ExerciseKind::TranslateToPortuguese => "translate_to_pt",
            ExerciseKind::OrderWords => "order_words",
            ExerciseKind::ListeningChoice => "listening_choice",
            ExerciseKind::Dictation => "dictation",
            ExerciseKind::Match => "match",
        }
    }

    pub fn instruction(self) -> &'static str {
        match self {
            ExerciseKind::MultipleChoice => "Choose the answer",
            ExerciseKind::FillBlank => "Fill in the blank",
            ExerciseKind::TranslateToPortuguese => "Translate into Portuguese",
            ExerciseKind::OrderWords => "Put the words in order",
            ExerciseKind::ListeningChoice => "Listen and choose the meaning",
            ExerciseKind::Dictation => "Listen and type what you hear",
            ExerciseKind::Match => "Match the pairs",
        }
    }
}

impl TryFrom<&str> for ExerciseKind {
    type Error = AppError;

    fn try_from(code: &str) -> Result<ExerciseKind, AppError> {
        ExerciseKind::ALL
            .into_iter()
            .find(|kind| kind.code() == code)
            .ok_or_else(|| AppError::from(format!("unknown exercise kind code; [code={code}]")))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Exercise {
    #[serde(rename = "multiple_choice")]
    MultipleChoice(MultipleChoiceExercise),
    #[serde(rename = "fill_blank")]
    FillBlank(FillBlankExercise),
    #[serde(rename = "translate_to_pt")]
    TranslateToPortuguese(TranslateToPortugueseExercise),
    #[serde(rename = "order_words")]
    OrderWords(OrderWordsExercise),
    #[serde(rename = "listening_choice")]
    ListeningChoice(ListeningChoiceExercise),
    #[serde(rename = "dictation")]
    Dictation(DictationExercise),
    #[serde(rename = "match")]
    Match(MatchExercise),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultipleChoiceExercise {
    pub prompt: String,
    pub choices: Vec<String>,
    pub answer: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FillBlankExercise {
    pub prompt_pt: String,
    pub answer: Vec<String>,
    pub hint_en: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

impl FillBlankExercise {
    pub const BLANK_MARKER: &'static str = "___";

    pub fn completed_sentence(&self) -> String {
        let first_answer: &str = self.answer.first().map(String::as_str).unwrap_or("");

        self.prompt_pt.replace(Self::BLANK_MARKER, first_answer)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslateToPortugueseExercise {
    pub prompt_en: String,
    pub answers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderWordsExercise {
    pub prompt_en: String,
    pub words: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub distractors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

impl OrderWordsExercise {
    pub fn sentence(&self) -> String {
        self.words.join(" ")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListeningChoiceExercise {
    pub audio_pt: String,
    pub choices: Vec<String>,
    pub answer: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictationExercise {
    pub audio_pt: String,
    pub hint_en: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchExercise {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    pub pairs: Vec<PhrasePair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

impl Exercise {
    pub fn kind(&self) -> ExerciseKind {
        match self {
            Exercise::MultipleChoice(_) => ExerciseKind::MultipleChoice,
            Exercise::FillBlank(_) => ExerciseKind::FillBlank,
            Exercise::TranslateToPortuguese(_) => ExerciseKind::TranslateToPortuguese,
            Exercise::OrderWords(_) => ExerciseKind::OrderWords,
            Exercise::ListeningChoice(_) => ExerciseKind::ListeningChoice,
            Exercise::Dictation(_) => ExerciseKind::Dictation,
            Exercise::Match(_) => ExerciseKind::Match,
        }
    }

    pub fn explanation(&self) -> Option<&str> {
        let explanation: &Option<String> = match self {
            Exercise::MultipleChoice(exercise) => &exercise.explanation,
            Exercise::FillBlank(exercise) => &exercise.explanation,
            Exercise::TranslateToPortuguese(exercise) => &exercise.explanation,
            Exercise::OrderWords(exercise) => &exercise.explanation,
            Exercise::ListeningChoice(exercise) => &exercise.explanation,
            Exercise::Dictation(exercise) => &exercise.explanation,
            Exercise::Match(exercise) => &exercise.explanation,
        };

        explanation.as_deref()
    }

    pub fn clip_key(&self) -> Option<&str> {
        let clip_key: &Option<String> = match self {
            Exercise::FillBlank(exercise) => &exercise.clip_key,
            Exercise::TranslateToPortuguese(exercise) => &exercise.clip_key,
            Exercise::OrderWords(exercise) => &exercise.clip_key,
            Exercise::ListeningChoice(exercise) => &exercise.clip_key,
            Exercise::Dictation(exercise) => &exercise.clip_key,
            Exercise::MultipleChoice(_) | Exercise::Match(_) => return None,
        };

        clip_key.as_deref()
    }

    /// The Portuguese sentence that the exercise's clip speaks.
    pub fn spoken_text(&self) -> Option<String> {
        match self {
            Exercise::FillBlank(exercise) => Some(exercise.completed_sentence()),
            Exercise::TranslateToPortuguese(exercise) => exercise.answers.first().cloned(),
            Exercise::OrderWords(exercise) => Some(exercise.sentence()),
            Exercise::ListeningChoice(exercise) => Some(exercise.audio_pt.clone()),
            Exercise::Dictation(exercise) => Some(exercise.audio_pt.clone()),
            Exercise::MultipleChoice(_) | Exercise::Match(_) => None,
        }
    }

    /// The question as shown when the exercise is listed for review.
    pub fn prompt_text(&self) -> String {
        match self {
            Exercise::MultipleChoice(exercise) => exercise.prompt.clone(),
            Exercise::FillBlank(exercise) => exercise.prompt_pt.clone(),
            Exercise::TranslateToPortuguese(exercise) => exercise.prompt_en.clone(),
            Exercise::OrderWords(exercise) => exercise.prompt_en.clone(),
            Exercise::ListeningChoice(exercise) => exercise.audio_pt.clone(),
            Exercise::Dictation(exercise) => exercise.audio_pt.clone(),
            Exercise::Match(exercise) => exercise
                .prompt
                .clone()
                .unwrap_or_else(|| ExerciseKind::Match.instruction().to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_exercise(text: &str) -> Result<Exercise, serde_json::Error> {
        serde_json::from_str(text)
    }

    #[test]
    fn exercise_parses_each_kind_from_its_type_tag() {
        let multiple_choice: Exercise =
            parse_exercise(r#"{"type":"multiple_choice","prompt":"p","choices":["a","b","c"],"answer":1}"#).unwrap();
        let translate: Exercise =
            parse_exercise(r#"{"type":"translate_to_pt","prompt_en":"I live here.","answers":["Eu moro aqui."]}"#).unwrap();

        assert_eq!(multiple_choice.kind(), ExerciseKind::MultipleChoice);
        assert_eq!(translate.kind(), ExerciseKind::TranslateToPortuguese);
    }

    #[test]
    fn exercise_rejects_unknown_fields() {
        let result: Result<Exercise, serde_json::Error> =
            parse_exercise(r#"{"type":"dictation","audio_pt":"Oi.","hint_en":"Hi.","surprise":1}"#);

        assert!(result.is_err());
    }

    #[test]
    fn exercise_rejects_unknown_type() {
        let result: Result<Exercise, serde_json::Error> = parse_exercise(r#"{"type":"riddle","prompt":"p"}"#);

        assert!(result.is_err());
    }

    #[test]
    fn fill_blank_completed_sentence_uses_first_answer() {
        let exercise: FillBlankExercise = FillBlankExercise {
            prompt_pt: "Eu ___ brasileiro.".to_string(),
            answer: vec!["sou".to_string()],
            hint_en: "I am Brazilian.".to_string(),
            explanation: None,
            clip_key: None,
        };

        assert_eq!(exercise.completed_sentence(), "Eu sou brasileiro.");
    }

    #[test]
    fn vocabulary_card_id_round_trips() {
        let card_id: String = vocabulary_card_id("a1-01-greetings-introductions", 7);

        assert_eq!(parse_vocabulary_card_id(&card_id), Some(("a1-01-greetings-introductions", 7)));
        assert_eq!(parse_vocabulary_card_id("no-index"), None);
    }
}
