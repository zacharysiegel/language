use clap::ValueEnum;
use shared::course_shard::VoiceEngineKind;

use crate::speech_engine::{PersonaVoices, SpeechLanguage};

pub const LISTENING_PERSONA_COUNT: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SpeechEngineKind {
    Kokoro,
    MacosSay,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoiceProfile {
    pub voice_id: &'static str,
    pub speed: f32,
}

/// What an engine is asked to speak: the text, its language, and the voice the engine has for the persona.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedUtterance {
    pub language: &'static SpeechLanguage,
    pub profile: VoiceProfile,
    pub text: String,
}

/// Who is speaking a clip, independent of the engine that voices it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonaKind {
    Narrator,
    FemaleSpeaker(usize),
    MaleSpeaker(usize),
    ListeningExercise(usize),
}

impl PersonaKind {
    pub fn key(self) -> String {
        match self {
            PersonaKind::Narrator => "narrator".to_string(),
            PersonaKind::FemaleSpeaker(index) => format!("female_{index}"),
            PersonaKind::MaleSpeaker(index) => format!("male_{index}"),
            PersonaKind::ListeningExercise(index) => format!("listening_{index}"),
        }
    }
}

impl SpeechEngineKind {
    pub const ALL: [SpeechEngineKind; 2] = [SpeechEngineKind::Kokoro, SpeechEngineKind::MacosSay];

    pub fn voice_engine(self) -> VoiceEngineKind {
        match self {
            SpeechEngineKind::Kokoro => VoiceEngineKind::Kokoro,
            SpeechEngineKind::MacosSay => VoiceEngineKind::MacosSay,
        }
    }

    pub fn code(self) -> &'static str {
        self.voice_engine().code()
    }

    /* Changes when the synthesis pipeline alters its output, which discards the engine's existing clips. */
    pub fn pipeline_revision(self) -> u32 {
        match self {
            SpeechEngineKind::Kokoro => 3,
            SpeechEngineKind::MacosSay => 1,
        }
    }

    /// `None` when the engine has no voices for the language.
    pub fn profile_for(self, language: &SpeechLanguage, persona: PersonaKind) -> Option<VoiceProfile> {
        let voices: &PersonaVoices = match self {
            SpeechEngineKind::Kokoro => &language.kokoro.as_ref()?.voices,
            SpeechEngineKind::MacosSay => &language.macos_voices,
        };

        let profile: VoiceProfile = match persona {
            PersonaKind::Narrator => voices.narrator,
            PersonaKind::FemaleSpeaker(index) => voices.female_speakers[index % voices.female_speakers.len()],
            PersonaKind::MaleSpeaker(index) => voices.male_speakers[index % voices.male_speakers.len()],
            PersonaKind::ListeningExercise(index) => voices.listening_pool[index % voices.listening_pool.len()],
        };

        Some(profile)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::speech_engine::{BRAZILIAN_PORTUGUESE, RUSSIAN};

    #[test]
    fn profile_for_wraps_speaker_indexes_around_the_pool() {
        let engine: SpeechEngineKind = SpeechEngineKind::Kokoro;

        assert_eq!(
            engine.profile_for(&BRAZILIAN_PORTUGUESE, PersonaKind::FemaleSpeaker(0)),
            engine.profile_for(&BRAZILIAN_PORTUGUESE, PersonaKind::FemaleSpeaker(3)),
        );
        assert_eq!(
            engine.profile_for(&BRAZILIAN_PORTUGUESE, PersonaKind::ListeningExercise(1)),
            engine.profile_for(&BRAZILIAN_PORTUGUESE, PersonaKind::ListeningExercise(7)),
        );
    }

    #[test]
    fn kokoro_voices_for_brazilian_portuguese_are_its_portuguese_voices() {
        for index in 0..LISTENING_PERSONA_COUNT {
            let voice_id: &str = SpeechEngineKind::Kokoro
                .profile_for(&BRAZILIAN_PORTUGUESE, PersonaKind::ListeningExercise(index))
                .unwrap()
                .voice_id;

            assert!(voice_id.starts_with("pf_") || voice_id.starts_with("pm_"));
        }
    }

    #[test]
    fn profile_for_has_no_kokoro_voice_for_a_language_without_kokoro_voices() {
        assert_eq!(SpeechEngineKind::Kokoro.profile_for(&RUSSIAN, PersonaKind::Narrator), None);
        assert!(SpeechEngineKind::MacosSay.profile_for(&RUSSIAN, PersonaKind::MaleSpeaker(2)).is_some());
    }
}
