use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use shared::course::{Dialogue, Exercise, GrammarNote, PhrasePair, Reading, SpeakerGenderKind, Unit, VocabularyItem};

use crate::speech_engine::{LISTENING_PERSONA_COUNT, PersonaKind};

const CLIP_KEY_HASH_CHARACTERS: usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct Utterance {
    /// The BCP 47 tag of the language the text is in.
    pub language: String,
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

    /* A clip's identity is its language, persona and text only, so an unchanged sentence keeps its recordings across builds. */
    fn clip_key_for(&mut self, language: &str, text: &str, persona: PersonaKind) -> String {
        let identity: String = format!("{language}\n{}\n{text}", persona.key());
        let clip_key: String = sha256_hex(&identity)[..CLIP_KEY_HASH_CHARACTERS].to_string();

        self.utterances.insert(
            clip_key.clone(),
            Utterance {
                language: language.to_string(),
                persona,
                text: text.to_string(),
            },
        );

        clip_key
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

/// The clips of one course's units, spoken in its target language.
struct CourseClips<'plan> {
    language: &'plan str,
    plan: &'plan mut AudioPlan,
}

impl CourseClips<'_> {
    fn narrate(&mut self, text: &str) -> Option<String> {
        Some(self.plan.clip_key_for(self.language, text, PersonaKind::Narrator))
    }

    fn voice_listening_exercise(&mut self, text: &str) -> Option<String> {
        let voice_index: usize = hash_bucket(text, LISTENING_PERSONA_COUNT);

        Some(self.plan.clip_key_for(self.language, text, PersonaKind::ListeningExercise(voice_index)))
    }

    fn speak_as(&mut self, text: &str, persona: PersonaKind) -> String {
        self.plan.clip_key_for(self.language, text, persona)
    }
}

/// Returns `unit` with a clip key on everything the learner can play, registering each clip, in `language`, in `plan`.
pub fn assign_clip_keys(unit: &Unit, language: &str, plan: &mut AudioPlan) -> Unit {
    let mut clips: CourseClips = CourseClips { language, plan };
    let mut keyed: Unit = unit.clone();

    keyed.vocabulary.iter_mut().for_each(|item| key_vocabulary_item(item, &mut clips));
    keyed.dialogues.iter_mut().for_each(|dialogue| key_dialogue(dialogue, &mut clips));
    keyed.grammar.iter_mut().for_each(|note| key_grammar_note(note, &mut clips));
    keyed.readings.iter_mut().for_each(|reading| key_reading(reading, &mut clips));
    keyed.exercises.iter_mut().for_each(|exercise| key_exercise(exercise, &mut clips));

    keyed
}

fn key_pair(pair: &mut PhrasePair, clips: &mut CourseClips) {
    pair.clip_key = clips.narrate(&pair.target);
}

fn key_vocabulary_item(item: &mut VocabularyItem, clips: &mut CourseClips) {
    item.clip_key = clips.narrate(&item.target);
    key_pair(&mut item.example, clips);
}

fn key_grammar_note(note: &mut GrammarNote, clips: &mut CourseClips) {
    note.examples.iter_mut().for_each(|example| key_pair(example, clips));
}

fn key_reading(reading: &mut Reading, clips: &mut CourseClips) {
    reading.paragraphs.iter_mut().for_each(|paragraph| key_pair(paragraph, clips));
    reading.questions.iter_mut().for_each(|question| key_exercise(question, clips));
}

fn key_dialogue(dialogue: &mut Dialogue, clips: &mut CourseClips) {
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

        line.clip_key = Some(clips.speak_as(&line.target, persona_by_speaker[&line.speaker]));
    }
}

fn key_exercise(exercise: &mut Exercise, clips: &mut CourseClips) {
    match exercise {
        Exercise::ListeningChoice(listening) => listening.clip_key = clips.voice_listening_exercise(&listening.audio_target),
        Exercise::Dictation(dictation) => dictation.clip_key = clips.voice_listening_exercise(&dictation.audio_target),
        Exercise::OrderWords(order_words) => order_words.clip_key = clips.narrate(&order_words.sentence()),
        Exercise::FillBlank(fill_blank) => fill_blank.clip_key = clips.narrate(&fill_blank.completed_sentence()),
        Exercise::TranslateToTarget(translate) => {
            translate.clip_key = translate.answers.first().and_then(|answer| clips.narrate(answer));
        },
        Exercise::Match(matching) => matching.pairs.iter_mut().for_each(|pair| key_pair(pair, clips)),
        Exercise::MultipleChoice(_) => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_key_for_is_deterministic_and_specific_to_voice_and_language() {
        let mut plan: AudioPlan = AudioPlan::new();

        let first: String = plan.clip_key_for("pt-BR", "Olá", PersonaKind::Narrator);
        let repeated: String = plan.clip_key_for("pt-BR", "Olá", PersonaKind::Narrator);
        let other_voice: String = plan.clip_key_for("pt-BR", "Olá", PersonaKind::MaleSpeaker(0));
        let other_language: String = plan.clip_key_for("es", "Olá", PersonaKind::Narrator);

        assert_eq!(first, repeated);
        assert_ne!(first, other_voice);
        assert_ne!(first, other_language);
        assert_eq!(plan.utterances.len(), 3);
        assert_eq!(first.len(), CLIP_KEY_HASH_CHARACTERS);
    }

    #[test]
    fn hash_bucket_stays_within_range() {
        for sample in ["a", "b", "Bom dia", "Eu moro no Rio de Janeiro."] {
            assert!(hash_bucket(sample, 7) < 7);
        }
    }
}
