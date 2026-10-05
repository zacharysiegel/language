use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use shared::course::{
    Dialogue, Exercise, GrammarNote, PhrasePair, Reading, SpeakerGenderKind, Unit, VocabularyItem,
};

use crate::speech_engine::{PersonaKind, SpeechEngineKind, VoiceProfile};

const AUDIO_PATH_PREFIX: &str = "audio/";
const FILE_NAME_HASH_CHARACTERS: usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct Utterance {
    pub profile: VoiceProfile,
    pub text: String,
}

/// Every distinct spoken clip of a compiled course, keyed by the file name it is stored under.
#[derive(Debug)]
pub struct AudioPlan {
    pub engine: SpeechEngineKind,
    pub utterances: BTreeMap<String, Utterance>,
}

impl AudioPlan {
    pub fn new(engine: SpeechEngineKind) -> AudioPlan {
        AudioPlan {
            engine,
            utterances: BTreeMap::new(),
        }
    }

    fn audio_path_for(&mut self, text: &str, persona: PersonaKind) -> String {
        let profile: VoiceProfile = self.engine.profile_for(persona);
        let identity: String = format!("{}\n{}\n{:.2}\n{text}", self.engine.code(), profile.voice_id, profile.speed);
        let file_name: String = format!("{}.m4a", &sha256_hex(&identity)[..FILE_NAME_HASH_CHARACTERS]);

        self.utterances.insert(
            file_name.clone(),
            Utterance {
                profile,
                text: text.to_string(),
            },
        );

        format!("{AUDIO_PATH_PREFIX}{file_name}")
    }

    fn narrate(&mut self, text: &str) -> Option<String> {
        Some(self.audio_path_for(text, PersonaKind::Narrator))
    }

    fn voice_listening_exercise(&mut self, text: &str) -> Option<String> {
        let voice_index: usize = hash_bucket(text, self.engine.listening_pool_size());

        Some(self.audio_path_for(text, PersonaKind::ListeningExercise(voice_index)))
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
    let mut persona_by_speaker: BTreeMap<String, PersonaKind> = BTreeMap::new();
    let mut female_speaker_count: usize = 0;
    let mut male_speaker_count: usize = 0;

    for line in &mut dialogue.lines {
        if !persona_by_speaker.contains_key(&line.speaker) {
            let persona: PersonaKind = match line.gender {
                SpeakerGenderKind::Female => PersonaKind::FemaleSpeaker(female_speaker_count),
                SpeakerGenderKind::Male => PersonaKind::MaleSpeaker(male_speaker_count),
            };

            match line.gender {
                SpeakerGenderKind::Female => female_speaker_count += 1,
                SpeakerGenderKind::Male => male_speaker_count += 1,
            }

            persona_by_speaker.insert(line.speaker.clone(), persona);
        }

        line.audio = Some(plan.audio_path_for(&line.pt, persona_by_speaker[&line.speaker]));
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
        let mut plan: AudioPlan = AudioPlan::new(SpeechEngineKind::Kokoro);

        let first: String = plan.audio_path_for("Olá", PersonaKind::Narrator);
        let repeated: String = plan.audio_path_for("Olá", PersonaKind::Narrator);
        let other_voice: String = plan.audio_path_for("Olá", PersonaKind::MaleSpeaker(0));

        assert_eq!(first, repeated);
        assert_ne!(first, other_voice);
        assert_eq!(plan.utterances.len(), 2);
        assert!(first.starts_with("audio/") && first.ends_with(".m4a"));
    }

    #[test]
    fn audio_path_for_differs_between_engines() {
        let mut kokoro_plan: AudioPlan = AudioPlan::new(SpeechEngineKind::Kokoro);
        let mut say_plan: AudioPlan = AudioPlan::new(SpeechEngineKind::MacosSay);

        assert_ne!(
            kokoro_plan.audio_path_for("Olá", PersonaKind::Narrator),
            say_plan.audio_path_for("Olá", PersonaKind::Narrator),
        );
    }

    #[test]
    fn hash_bucket_stays_within_range() {
        for sample in ["a", "b", "Bom dia", "Eu moro no Rio de Janeiro."] {
            assert!(hash_bucket(sample, 7) < 7);
        }
    }
}
