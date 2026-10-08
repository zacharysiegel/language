use shared::AppError;

use crate::speech_engine::VoiceProfile;

/// The voices one engine has for each persona in one language.
#[derive(Debug, PartialEq)]
pub struct PersonaVoices {
    pub narrator: VoiceProfile,
    pub female_speakers: &'static [VoiceProfile],
    pub male_speakers: &'static [VoiceProfile],
    pub listening_pool: &'static [VoiceProfile],
}

/// What the speech engines need to speak one target language.
#[derive(Debug, PartialEq)]
pub struct SpeechLanguage {
    /// The BCP 47 tag a course names as its target language.
    pub language_tag: &'static str,
    pub espeak_voice: &'static str,
    pub kokoro_voices: PersonaVoices,
    pub macos_voices: PersonaVoices,
    /// Function words that cannot stand alone, each with the short phrase it is spoken in.
    pub isolated_word_phrases: &'static [(&'static str, &'static str)],
}

const fn profile(voice_id: &'static str, speed: f32) -> VoiceProfile {
    VoiceProfile { voice_id, speed }
}

pub const BRAZILIAN_PORTUGUESE: SpeechLanguage = SpeechLanguage {
    language_tag: "pt-BR",
    espeak_voice: "pt-br",
    kokoro_voices: PersonaVoices {
        narrator: profile("pf_dora", 0.95),
        female_speakers: &[profile("pf_dora", 1.0), profile("pf_dora", 1.1), profile("pf_dora", 0.9)],
        male_speakers: &[profile("pm_alex", 1.0), profile("pm_santa", 1.0), profile("pm_alex", 1.1), profile("pm_santa", 0.9)],
        listening_pool: &[
            profile("pf_dora", 1.0),
            profile("pm_alex", 1.0),
            profile("pm_santa", 1.0),
            profile("pf_dora", 0.9),
            profile("pm_alex", 0.9),
            profile("pm_santa", 0.9),
        ],
    },
    macos_voices: PersonaVoices {
        narrator: profile("Luciana", 1.0),
        female_speakers: &[
            profile("Luciana", 1.0),
            profile("Flo (Portuguese (Brazil))", 1.0),
            profile("Sandy (Portuguese (Brazil))", 1.0),
            profile("Shelley (Portuguese (Brazil))", 1.0),
        ],
        male_speakers: &[
            profile("Reed (Portuguese (Brazil))", 1.0),
            profile("Eddy (Portuguese (Brazil))", 1.0),
            profile("Rocko (Portuguese (Brazil))", 1.0),
        ],
        listening_pool: &[
            profile("Luciana", 1.0),
            profile("Flo (Portuguese (Brazil))", 1.0),
            profile("Sandy (Portuguese (Brazil))", 1.0),
            profile("Shelley (Portuguese (Brazil))", 1.0),
            profile("Reed (Portuguese (Brazil))", 1.0),
            profile("Eddy (Portuguese (Brazil))", 1.0),
            profile("Rocko (Portuguese (Brazil))", 1.0),
        ],
    },
    isolated_word_phrases: &[
        ("o", "o livro"),
        ("a", "a casa"),
        ("os", "os livros"),
        ("as", "as casas"),
        ("e", "e eu"),
        ("é", "é bom"),
        ("à", "à casa"),
        ("às", "às casas"),
        ("de", "de casa"),
        ("do", "do livro"),
        ("da", "da casa"),
        ("no", "no livro"),
        ("na", "na casa"),
        ("em", "em casa"),
        ("ao", "ao livro"),
        ("um", "um livro"),
        ("uns", "uns livros"),
    ],
};

const SPEECH_LANGUAGES: [&SpeechLanguage; 1] = [&BRAZILIAN_PORTUGUESE];

/// The voices for a course's target language; an error names a language no engine is set up to speak.
pub fn speech_language(language_tag: &str) -> Result<&'static SpeechLanguage, AppError> {
    SPEECH_LANGUAGES
        .into_iter()
        .find(|language| language.language_tag == language_tag)
        .ok_or_else(|| AppError::from(format!("no speech voices are set up for the language; [language={language_tag}]")))
}
