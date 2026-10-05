use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use shared::course::{
    Dialogue, Exercise, GrammarNote, PhrasePair, Reading, SpeakerGenderKind, Unit, VocabularyItem,
};

const AUDIO_PATH_PREFIX: &str = "audio/";
const FILE_NAME_HASH_CHARACTERS: usize = 20;

const NARRATOR_VOICE: &str = "Luciana";
const FEMALE_VOICES: [&str; 4] = [
    "Luciana",
    "Flo (Portuguese (Brazil))",
    "Sandy (Portuguese (Brazil))",
    "Shelley (Portuguese (Brazil))",
];
const MALE_VOICES: [&str; 3] = [
    "Reed (Portuguese (Brazil))",
    "Eddy (Portuguese (Brazil))",
    "Rocko (Portuguese (Brazil))",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utterance {
    pub voice: &'static str,
    pub text: String,
}

/// Every distinct spoken clip of a compiled course, keyed by the file name it is stored under.
#[derive(Debug, Default)]
pub struct AudioPlan {
    pub utterances: BTreeMap<String, Utterance>,
}

impl AudioPlan {
    fn audio_path_for(&mut self, text: &str, voice: &'static str) -> String {
        let file_name: String = format!("{}.m4a", &sha256_hex(&format!("{voice}\n{text}"))[..FILE_NAME_HASH_CHARACTERS]);

        self.utterances.insert(
            file_name.clone(),
            Utterance {
                voice,
                text: text.to_string(),
            },
        );

        format!("{AUDIO_PATH_PREFIX}{file_name}")
    }

    fn narrate(&mut self, text: &str) -> Option<String> {
        Some(self.audio_path_for(text, NARRATOR_VOICE))
    }

    fn voice_listening_exercise(&mut self, text: &str) -> Option<String> {
        let voices: Vec<&'static str> = FEMALE_VOICES.iter().chain(MALE_VOICES.iter()).copied().collect();
        let voice: &'static str = voices[hash_bucket(text, voices.len())];

        Some(self.audio_path_for(text, voice))
    }
}

fn sha256_hex(text: &str) -> String {
    let digest: [u8; 32] = Sha256::digest(text.as_bytes()).into();

    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hash_bucket(text: &str, bucket_count: usize) -> usize {
    let digest: [u8; 32] = Sha256::digest(text.as_bytes()).into();
    let leading_value: u32 = u32::from_be_bytes([digest[0], digest[1], digest[2], digest[3]]);

    leading_value as usize % bucket_count
}

/// Returns `unit` with an audio path on every clip the learner can play, registering each clip in `plan`.
pub fn compile_unit(unit: &Unit, plan: &mut AudioPlan) -> Unit {
    let mut compiled: Unit = unit.clone();

    compiled.vocabulary.iter_mut().for_each(|item| compile_vocabulary_item(item, plan));
    compiled.dialogues.iter_mut().for_each(|dialogue| compile_dialogue(dialogue, plan));
    compiled.grammar.iter_mut().for_each(|note| compile_grammar_note(note, plan));
    compiled.readings.iter_mut().for_each(|reading| compile_reading(reading, plan));
    compiled.exercises.iter_mut().for_each(|exercise| compile_exercise(exercise, plan));

    compiled
}

fn compile_pair(pair: &mut PhrasePair, plan: &mut AudioPlan) {
    pair.audio = plan.narrate(&pair.pt);
}

fn compile_vocabulary_item(item: &mut VocabularyItem, plan: &mut AudioPlan) {
    item.audio = plan.narrate(&item.pt);
    compile_pair(&mut item.example, plan);
}

fn compile_grammar_note(note: &mut GrammarNote, plan: &mut AudioPlan) {
    note.examples.iter_mut().for_each(|example| compile_pair(example, plan));
}

fn compile_reading(reading: &mut Reading, plan: &mut AudioPlan) {
    reading.paragraphs.iter_mut().for_each(|paragraph| compile_pair(paragraph, plan));
    reading.questions.iter_mut().for_each(|question| compile_exercise(question, plan));
}

fn compile_dialogue(dialogue: &mut Dialogue, plan: &mut AudioPlan) {
    let mut voice_by_speaker: BTreeMap<String, &'static str> = BTreeMap::new();
    let mut female_speaker_count: usize = 0;
    let mut male_speaker_count: usize = 0;

    for line in &mut dialogue.lines {
        if !voice_by_speaker.contains_key(&line.speaker) {
            let voice: &'static str = match line.gender {
                SpeakerGenderKind::Female => FEMALE_VOICES[female_speaker_count % FEMALE_VOICES.len()],
                SpeakerGenderKind::Male => MALE_VOICES[male_speaker_count % MALE_VOICES.len()],
            };

            match line.gender {
                SpeakerGenderKind::Female => female_speaker_count += 1,
                SpeakerGenderKind::Male => male_speaker_count += 1,
            }

            voice_by_speaker.insert(line.speaker.clone(), voice);
        }

        line.audio = Some(plan.audio_path_for(&line.pt, voice_by_speaker[&line.speaker]));
    }
}

fn compile_exercise(exercise: &mut Exercise, plan: &mut AudioPlan) {
    match exercise {
        Exercise::ListeningChoice(listening) => listening.audio = plan.voice_listening_exercise(&listening.audio_pt),
        Exercise::Dictation(dictation) => dictation.audio = plan.voice_listening_exercise(&dictation.audio_pt),
        Exercise::OrderWords(order_words) => order_words.audio = plan.narrate(&order_words.sentence()),
        Exercise::FillBlank(fill_blank) => fill_blank.audio = plan.narrate(&fill_blank.completed_sentence()),
        Exercise::TranslateToPortuguese(translate) => {
            translate.audio = translate.answers.first().and_then(|answer| plan.narrate(answer));
        },
        Exercise::Match(matching) => matching.pairs.iter_mut().for_each(|pair| compile_pair(pair, plan)),
        Exercise::MultipleChoice(_) => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_path_for_is_deterministic_and_voice_specific() {
        let mut plan: AudioPlan = AudioPlan::default();

        let first: String = plan.audio_path_for("Olá", "Luciana");
        let repeated: String = plan.audio_path_for("Olá", "Luciana");
        let other_voice: String = plan.audio_path_for("Olá", "Reed (Portuguese (Brazil))");

        assert_eq!(first, repeated);
        assert_ne!(first, other_voice);
        assert_eq!(plan.utterances.len(), 2);
        assert!(first.starts_with("audio/") && first.ends_with(".m4a"));
    }

    #[test]
    fn hash_bucket_stays_within_range() {
        for sample in ["a", "b", "Bom dia", "Eu moro no Rio de Janeiro."] {
            assert!(hash_bucket(sample, 7) < 7);
        }
    }
}
