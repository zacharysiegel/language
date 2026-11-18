use shared::AppError;
use shared::course;

use crate::speech_engine::VoiceProfile;

/// The voices one engine has for each persona in one language.
#[derive(Debug, PartialEq)]
pub struct PersonaVoices {
    pub narrator: VoiceProfile,
    pub female_speakers: &'static [VoiceProfile],
    pub male_speakers: &'static [VoiceProfile],
    pub listening_pool: &'static [VoiceProfile],
}

/// What Kokoro needs to speak one target language.
#[derive(Debug, PartialEq)]
pub struct KokoroSpeech {
    /// The eSpeak NG voice that phonemizes the text for Kokoro.
    pub espeak_voice: &'static str,
    pub voices: PersonaVoices,
}

/// What the speech engines need to speak one target language.
#[derive(Debug, PartialEq)]
pub struct SpeechLanguage {
    /// The BCP 47 tag a course names as its target language.
    pub language_tag: &'static str,
    /// `None` when Kokoro has no voices for the language.
    pub kokoro: Option<KokoroSpeech>,
    pub macos_voices: PersonaVoices,
    /// Function words that cannot stand alone, each with the short phrase it is spoken in.
    pub isolated_word_phrases: &'static [(&'static str, &'static str)],
}

const fn profile(voice_id: &'static str, speed: f32) -> VoiceProfile {
    VoiceProfile { voice_id, speed }
}

pub const BRAZILIAN_PORTUGUESE: SpeechLanguage = SpeechLanguage {
    language_tag: course::BRAZILIAN_PORTUGUESE_LANGUAGE_TAG,
    kokoro: Some(KokoroSpeech {
        espeak_voice: "pt-br",
        voices: PersonaVoices {
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
    }),
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

pub const MEXICAN_SPANISH: SpeechLanguage = SpeechLanguage {
    language_tag: course::MEXICAN_SPANISH_LANGUAGE_TAG,
    kokoro: Some(KokoroSpeech {
        espeak_voice: "es-419",
        voices: PersonaVoices {
            narrator: profile("ef_dora", 0.95),
            female_speakers: &[profile("ef_dora", 1.0), profile("ef_dora", 1.1), profile("ef_dora", 0.9)],
            male_speakers: &[profile("em_alex", 1.0), profile("em_santa", 1.0), profile("em_alex", 1.1), profile("em_santa", 0.9)],
            listening_pool: &[
                profile("ef_dora", 1.0),
                profile("em_alex", 1.0),
                profile("em_santa", 1.0),
                profile("ef_dora", 0.9),
                profile("em_alex", 0.9),
                profile("em_santa", 0.9),
            ],
        },
    }),
    macos_voices: PersonaVoices {
        narrator: profile("Paulina", 1.0),
        female_speakers: &[
            profile("Paulina", 1.0),
            profile("Flo (Spanish (Mexico))", 1.0),
            profile("Sandy (Spanish (Mexico))", 1.0),
            profile("Shelley (Spanish (Mexico))", 1.0),
        ],
        male_speakers: &[
            profile("Reed (Spanish (Mexico))", 1.0),
            profile("Eddy (Spanish (Mexico))", 1.0),
            profile("Rocko (Spanish (Mexico))", 1.0),
        ],
        listening_pool: &[
            profile("Paulina", 1.0),
            profile("Flo (Spanish (Mexico))", 1.0),
            profile("Sandy (Spanish (Mexico))", 1.0),
            profile("Shelley (Spanish (Mexico))", 1.0),
            profile("Reed (Spanish (Mexico))", 1.0),
            profile("Eddy (Spanish (Mexico))", 1.0),
            profile("Rocko (Spanish (Mexico))", 1.0),
        ],
    },
    isolated_word_phrases: &[
        ("y", "y tú"),
        ("o", "o no"),
        ("u", "uno u otro"),
        ("e", "padres e hijos"),
        ("a", "a casa"),
        ("el", "el libro"),
        ("la", "la casa"),
        ("los", "los libros"),
        ("las", "las casas"),
        ("un", "un libro"),
        ("de", "de casa"),
        ("del", "del libro"),
        ("al", "al cine"),
        ("en", "en casa"),
        ("lo", "lo sé"),
        ("me", "me llamo"),
        ("te", "te quiero"),
        ("se", "se llama"),
    ],
};

pub const FRENCH: SpeechLanguage = SpeechLanguage {
    language_tag: course::FRENCH_LANGUAGE_TAG,
    kokoro: None,
    macos_voices: PersonaVoices {
        narrator: profile("Thomas", 1.0),
        female_speakers: &[
            profile("Flo (French (France))", 1.0),
            profile("Sandy (French (France))", 1.0),
            profile("Shelley (French (France))", 1.0),
        ],
        male_speakers: &[
            profile("Thomas", 1.0),
            profile("Jacques", 1.0),
            profile("Eddy (French (France))", 1.0),
            profile("Rocko (French (France))", 1.0),
        ],
        listening_pool: &[
            profile("Thomas", 1.0),
            profile("Flo (French (France))", 1.0),
            profile("Jacques", 1.0),
            profile("Sandy (French (France))", 1.0),
            profile("Eddy (French (France))", 1.0),
            profile("Shelley (French (France))", 1.0),
            profile("Rocko (French (France))", 1.0),
        ],
    },
    isolated_word_phrases: &[
        ("le", "le livre"),
        ("la", "la maison"),
        ("les", "les livres"),
        ("un", "un livre"),
        ("une", "une maison"),
        ("des", "des livres"),
        ("de", "de Paris"),
        ("du", "du pain"),
        ("au", "au café"),
        ("aux", "aux amis"),
        ("à", "à Paris"),
        ("y", "il y a"),
        ("ne", "ne pas"),
    ],
};

pub const GERMAN: SpeechLanguage = SpeechLanguage {
    language_tag: course::GERMAN_LANGUAGE_TAG,
    kokoro: None,
    macos_voices: PersonaVoices {
        narrator: profile("Anna", 1.0),
        female_speakers: &[
            profile("Anna", 1.0),
            profile("Flo (German (Germany))", 1.0),
            profile("Sandy (German (Germany))", 1.0),
            profile("Shelley (German (Germany))", 1.0),
        ],
        male_speakers: &[
            profile("Reed (German (Germany))", 1.0),
            profile("Eddy (German (Germany))", 1.0),
            profile("Rocko (German (Germany))", 1.0),
        ],
        listening_pool: &[
            profile("Anna", 1.0),
            profile("Flo (German (Germany))", 1.0),
            profile("Sandy (German (Germany))", 1.0),
            profile("Shelley (German (Germany))", 1.0),
            profile("Reed (German (Germany))", 1.0),
            profile("Eddy (German (Germany))", 1.0),
            profile("Rocko (German (Germany))", 1.0),
        ],
    },
    isolated_word_phrases: &[
        ("der", "der Mann"),
        ("die", "die Frau"),
        ("das", "das Kind"),
        ("den", "den Mann"),
        ("dem", "dem Mann"),
        ("des", "des Mannes"),
        ("ein", "ein Mann"),
        ("eine", "eine Frau"),
        ("einen", "einen Mann"),
        ("einem", "einem Mann"),
        ("einer", "einer Frau"),
        ("eines", "eines Mannes"),
    ],
};

pub const RUSSIAN: SpeechLanguage = SpeechLanguage {
    language_tag: course::RUSSIAN_LANGUAGE_TAG,
    kokoro: None,
    macos_voices: PersonaVoices {
        narrator: profile("Milena (Enhanced)", 1.0),
        female_speakers: &[profile("Milena (Enhanced)", 1.0), profile("Katya (Enhanced)", 1.0)],
        male_speakers: &[profile("Yuri (Enhanced)", 1.0)],
        listening_pool: &[
            profile("Milena (Enhanced)", 1.0),
            profile("Yuri (Enhanced)", 1.0),
            profile("Katya (Enhanced)", 1.0),
        ],
    },
    isolated_word_phrases: &[
        ("в", "в доме"),
        ("к", "к другу"),
        ("с", "с другом"),
        ("у", "у меня"),
        ("о", "о нём"),
        ("и", "и я"),
        ("а", "а ты"),
    ],
};

pub const UKRAINIAN: SpeechLanguage = SpeechLanguage {
    language_tag: course::UKRAINIAN_LANGUAGE_TAG,
    kokoro: None,
    macos_voices: PersonaVoices {
        narrator: profile("Lesya (Enhanced)", 1.0),
        female_speakers: &[profile("Lesya (Enhanced)", 1.0), profile("Lesya", 1.0)],
        male_speakers: &[profile("Lesya (Enhanced)", 1.0)],
        listening_pool: &[profile("Lesya (Enhanced)", 1.0), profile("Lesya", 1.0)],
    },
    isolated_word_phrases: &[
        ("в", "в місті"),
        ("у", "у мене"),
        ("з", "з другом"),
        ("і", "і я"),
        ("й", "вона й він"),
        ("а", "а ти"),
        ("о", "о п'ятій"),
    ],
};

const SPEECH_LANGUAGES: [&SpeechLanguage; 6] = [&BRAZILIAN_PORTUGUESE, &MEXICAN_SPANISH, &FRENCH, &GERMAN, &RUSSIAN, &UKRAINIAN];

/// The voices for a course's target language; an error names a language no engine is set up to speak.
pub fn speech_language(language_tag: &str) -> Result<&'static SpeechLanguage, AppError> {
    SPEECH_LANGUAGES
        .into_iter()
        .find(|language| language.language_tag == language_tag)
        .ok_or_else(|| AppError::from(format!("no speech voices are set up for the language; [language={language_tag}]")))
}
