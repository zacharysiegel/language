use clap::ValueEnum;

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

/// Who is speaking a clip, independent of the engine that voices it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonaKind {
    Narrator,
    FemaleSpeaker(usize),
    MaleSpeaker(usize),
    ListeningExercise(usize),
}

const fn profile(voice_id: &'static str, speed: f32) -> VoiceProfile {
    VoiceProfile { voice_id, speed }
}

const KOKORO_NARRATOR: VoiceProfile = profile("pf_dora", 0.95);
const KOKORO_FEMALE_SPEAKERS: [VoiceProfile; 3] = [profile("pf_dora", 1.0), profile("pf_dora", 1.1), profile("pf_dora", 0.9)];
const KOKORO_MALE_SPEAKERS: [VoiceProfile; 4] = [
    profile("pm_alex", 1.0),
    profile("pm_santa", 1.0),
    profile("pm_alex", 1.1),
    profile("pm_santa", 0.9),
];
const KOKORO_LISTENING_POOL: [VoiceProfile; 6] = [
    profile("pf_dora", 1.0),
    profile("pm_alex", 1.0),
    profile("pm_santa", 1.0),
    profile("pf_dora", 0.9),
    profile("pm_alex", 0.9),
    profile("pm_santa", 0.9),
];

const MACOS_NARRATOR: VoiceProfile = profile("Luciana", 1.0);
const MACOS_FEMALE_SPEAKERS: [VoiceProfile; 4] = [
    profile("Luciana", 1.0),
    profile("Flo (Portuguese (Brazil))", 1.0),
    profile("Sandy (Portuguese (Brazil))", 1.0),
    profile("Shelley (Portuguese (Brazil))", 1.0),
];
const MACOS_MALE_SPEAKERS: [VoiceProfile; 3] = [
    profile("Reed (Portuguese (Brazil))", 1.0),
    profile("Eddy (Portuguese (Brazil))", 1.0),
    profile("Rocko (Portuguese (Brazil))", 1.0),
];
const MACOS_LISTENING_POOL: [VoiceProfile; 7] = [
    profile("Luciana", 1.0),
    profile("Flo (Portuguese (Brazil))", 1.0),
    profile("Sandy (Portuguese (Brazil))", 1.0),
    profile("Shelley (Portuguese (Brazil))", 1.0),
    profile("Reed (Portuguese (Brazil))", 1.0),
    profile("Eddy (Portuguese (Brazil))", 1.0),
    profile("Rocko (Portuguese (Brazil))", 1.0),
];

impl SpeechEngineKind {
    pub fn code(self) -> &'static str {
        match self {
            SpeechEngineKind::Kokoro => "kokoro",
            SpeechEngineKind::MacosSay => "macos_say",
        }
    }

    pub fn profile_for(self, persona: PersonaKind) -> VoiceProfile {
        match (self, persona) {
            (SpeechEngineKind::Kokoro, PersonaKind::Narrator) => KOKORO_NARRATOR,
            (SpeechEngineKind::Kokoro, PersonaKind::FemaleSpeaker(index)) => KOKORO_FEMALE_SPEAKERS[index % KOKORO_FEMALE_SPEAKERS.len()],
            (SpeechEngineKind::Kokoro, PersonaKind::MaleSpeaker(index)) => KOKORO_MALE_SPEAKERS[index % KOKORO_MALE_SPEAKERS.len()],
            (SpeechEngineKind::Kokoro, PersonaKind::ListeningExercise(index)) => KOKORO_LISTENING_POOL[index % KOKORO_LISTENING_POOL.len()],
            (SpeechEngineKind::MacosSay, PersonaKind::Narrator) => MACOS_NARRATOR,
            (SpeechEngineKind::MacosSay, PersonaKind::FemaleSpeaker(index)) => MACOS_FEMALE_SPEAKERS[index % MACOS_FEMALE_SPEAKERS.len()],
            (SpeechEngineKind::MacosSay, PersonaKind::MaleSpeaker(index)) => MACOS_MALE_SPEAKERS[index % MACOS_MALE_SPEAKERS.len()],
            (SpeechEngineKind::MacosSay, PersonaKind::ListeningExercise(index)) => MACOS_LISTENING_POOL[index % MACOS_LISTENING_POOL.len()],
        }
    }

    pub fn listening_pool_size(self) -> usize {
        match self {
            SpeechEngineKind::Kokoro => KOKORO_LISTENING_POOL.len(),
            SpeechEngineKind::MacosSay => MACOS_LISTENING_POOL.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_for_wraps_speaker_indexes_around_the_pool() {
        let engine: SpeechEngineKind = SpeechEngineKind::Kokoro;

        assert_eq!(engine.profile_for(PersonaKind::FemaleSpeaker(0)), engine.profile_for(PersonaKind::FemaleSpeaker(3)));
        assert_eq!(engine.profile_for(PersonaKind::ListeningExercise(1)), engine.profile_for(PersonaKind::ListeningExercise(7)));
    }

    #[test]
    fn kokoro_voices_are_the_brazilian_portuguese_voices() {
        for index in 0..KOKORO_LISTENING_POOL.len() {
            let voice_id: &str = SpeechEngineKind::Kokoro.profile_for(PersonaKind::ListeningExercise(index)).voice_id;

            assert!(voice_id.starts_with("pf_") || voice_id.starts_with("pm_"));
        }
    }
}
