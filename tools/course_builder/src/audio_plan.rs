use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use shared::course::{
    Dialogue, Exercise, GrammarNote, PhrasePair, Reading, SpeakerGenderKind, Unit, VocabularyItem,
};

use crate::speech_engine::{LISTENING_PERSONA_COUNT, PersonaKind};

const CLIP_KEY_HASH_CHARACTERS: usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct Utterance {
    pub persona: PersonaKind,
    pub text: String,
}

/// Every distinct spoken clip of the course, by clip key.
#[derive(Debug, Default)]
pub struct AudioPlan {
    pub utterances: BTreeMap<String, Utterance>,
}

impl AudioPlan {
    pub fn new() -> AudioPlan {
        AudioPlan::default()
    }

    /* A clip's identity is its persona and text only, so an unchanged sentence keeps its recordings across builds. */
    fn clip_key_for(&mut self, text: &str, persona: PersonaKind) -> String {
        let identity: String = format!("{}\n{text}", persona.key());
        let clip_key: String = sha256_hex(&identity)[..CLIP_KEY_HASH_CHARACTERS].to_string();

        self.utterances.insert(
            clip_key.clone(),
            Utterance {
                persona,
                text: text.to_string(),
            },
        );

        clip_key
    }

    fn narrate(&mut self, text: &str) -> Option<String> {
        Some(self.clip_key_for(text, PersonaKind::Narrator))
    }

    fn voice_listening_exercise(&mut self, text: &str) -> Option<String> {
        let voice_index: usize = hash_bucket(text, LISTENING_PERSONA_COUNT);

        Some(self.clip_key_for(text, PersonaKind::ListeningExercise(voice_index)))
    }

    pub fn utterance(&self, clip_key: &str) -> Option<&Utterance> {
        self.utterances.get(clip_key)
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

/// Returns `unit` with a clip key on everything the learner can play, registering each clip in `plan`.
pub fn assign_clip_keys(unit: &Unit, plan: &mut AudioPlan) -> Unit {
    let mut keyed: Unit = unit.clone();

    keyed.vocabulary.iter_mut().for_each(|item| key_vocabulary_item(item, plan));
    keyed.dialogues.iter_mut().for_each(|dialogue| key_dialogue(dialogue, plan));
    keyed.grammar.iter_mut().for_each(|note| key_grammar_note(note, plan));
    keyed.readings.iter_mut().for_each(|reading| key_reading(reading, plan));
    keyed.exercises.iter_mut().for_each(|exercise| key_exercise(exercise, plan));

    keyed
}

fn key_pair(pair: &mut PhrasePair, plan: &mut AudioPlan) {
    pair.clip_key = plan.narrate(&pair.pt);
}

fn key_vocabulary_item(item: &mut VocabularyItem, plan: &mut AudioPlan) {
    item.clip_key = plan.narrate(&item.pt);
    key_pair(&mut item.example, plan);
}

fn key_grammar_note(note: &mut GrammarNote, plan: &mut AudioPlan) {
    note.examples.iter_mut().for_each(|example| key_pair(example, plan));
}

fn key_reading(reading: &mut Reading, plan: &mut AudioPlan) {
    reading.paragraphs.iter_mut().for_each(|paragraph| key_pair(paragraph, plan));
    reading.questions.iter_mut().for_each(|question| key_exercise(question, plan));
}

fn key_dialogue(dialogue: &mut Dialogue, plan: &mut AudioPlan) {
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

        line.clip_key = Some(plan.clip_key_for(&line.pt, persona_by_speaker[&line.speaker]));
    }
}

fn key_exercise(exercise: &mut Exercise, plan: &mut AudioPlan) {
    match exercise {
        Exercise::ListeningChoice(listening) => listening.clip_key = plan.voice_listening_exercise(&listening.audio_pt),
        Exercise::Dictation(dictation) => dictation.clip_key = plan.voice_listening_exercise(&dictation.audio_pt),
        Exercise::OrderWords(order_words) => order_words.clip_key = plan.narrate(&order_words.sentence()),
        Exercise::FillBlank(fill_blank) => fill_blank.clip_key = plan.narrate(&fill_blank.completed_sentence()),
        Exercise::TranslateToPortuguese(translate) => {
            translate.clip_key = translate.answers.first().and_then(|answer| plan.narrate(answer));
        },
        Exercise::Match(matching) => matching.pairs.iter_mut().for_each(|pair| key_pair(pair, plan)),
        Exercise::MultipleChoice(_) => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_key_for_is_deterministic_and_voice_specific() {
        let mut plan: AudioPlan = AudioPlan::new();

        let first: String = plan.clip_key_for("Olá", PersonaKind::Narrator);
        let repeated: String = plan.clip_key_for("Olá", PersonaKind::Narrator);
        let other_voice: String = plan.clip_key_for("Olá", PersonaKind::MaleSpeaker(0));

        assert_eq!(first, repeated);
        assert_ne!(first, other_voice);
        assert_eq!(plan.utterances.len(), 2);
        assert_eq!(first.len(), CLIP_KEY_HASH_CHARACTERS);
    }

    #[test]
    fn hash_bucket_stays_within_range() {
        for sample in ["a", "b", "Bom dia", "Eu moro no Rio de Janeiro."] {
            assert!(hash_bucket(sample, 7) < 7);
        }
    }
}
