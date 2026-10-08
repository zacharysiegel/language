use std::ffi::{CStr, CString, c_char, c_void};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

use shared::AppError;

const ESPEAK_DATA_DIRECTORY_VARIABLE: &str = "ESPEAKNG_DATA_DIRECTORY";
const ESPEAK_DATA_DIRECTORY_NAME: &str = "espeak-ng-data";
const TIE_CHARACTER: char = '^';
const PUNCTUATION_CHARACTERS: &str = ";:,.!?¡¿—…\"«»“”";
const SENTENCE_END_CHARACTERS: &str = ".!?";
const MAXIMUM_CHUNK_CHARACTERS: usize = 250;

/* Diphthongs and affricates that eSpeak joins with a tie, mapped to the single symbols Kokoro's vocabulary has. */
const TIED_PHONEME_SYMBOLS: [(&str, &str); 11] = [
    ("a^ɪ", "I"),
    ("a^ʊ", "W"),
    ("d^z", "ʣ"),
    ("d^ʒ", "ʤ"),
    ("e^ɪ", "A"),
    ("o^ʊ", "O"),
    ("ə^ʊ", "Q"),
    ("s^s", "S"),
    ("t^s", "ʦ"),
    ("t^ʃ", "ʧ"),
    ("ɔ^ɪ", "Y"),
];

// eSpeak keeps its voice and clause position in process-wide state; the session holds the voice it last set.
static ESPEAK_SESSION: Mutex<Option<String>> = Mutex::new(None);
static ESPEAK_INITIALIZATION: OnceLock<Result<(), String>> = OnceLock::new();

pub struct Phonemizer;

impl Phonemizer {
    pub fn initialize() -> Result<Phonemizer, AppError> {
        let initialization: &Result<(), String> = ESPEAK_INITIALIZATION.get_or_init(initialize_espeak);

        match initialization {
            Ok(()) => Ok(Phonemizer),
            Err(message) => Err(AppError::from(message.clone())),
        }
    }

    /// Phonemes in the symbol set of Kokoro's vocabulary, with the punctuation of `text` kept in place.
    pub fn phonemize(&self, espeak_voice: &str, text: &str) -> Result<String, AppError> {
        let mut session: MutexGuard<'_, Option<String>> = ESPEAK_SESSION.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        if session.as_deref() != Some(espeak_voice) {
            set_espeak_voice(espeak_voice)?;
            *session = Some(espeak_voice.to_string());
        }

        let text: String = text.replace('«', "\u{201c}").replace('»', "\u{201d}");
        let mut phonemes: String = String::new();
        let mut words: String = String::new();

        for character in text.chars() {
            if PUNCTUATION_CHARACTERS.contains(character) {
                append_words(&mut phonemes, &mut words);
                phonemes.push(character);
            } else {
                words.push(character);
            }
        }

        append_words(&mut phonemes, &mut words);
        drop(session);

        for (tied, symbol) in TIED_PHONEME_SYMBOLS {
            phonemes = phonemes.replace(tied, symbol);
        }

        Ok(phonemes.replace(TIE_CHARACTER, "").replace('-', ""))
    }
}

fn set_espeak_voice(espeak_voice: &str) -> Result<(), AppError> {
    let voice: CString = CString::new(espeak_voice).map_err(|error| AppError::from(error.to_string()))?;

    // SAFETY: `voice` is a valid NUL-terminated string that outlives the call; the caller holds the eSpeak session.
    let status: i32 = unsafe { espeak_rs_sys::espeak_SetVoiceByName(voice.as_ptr()) };

    if status != espeak_rs_sys::espeak_ERROR_EE_OK as i32 {
        return Err(AppError::from(format!("eSpeak NG has no such voice; [voice={espeak_voice} status={status}]")));
    }

    Ok(())
}

fn append_words(phonemes: &mut String, words: &mut String) {
    let trimmed_words: &str = words.trim();

    if !trimmed_words.is_empty() {
        if !phonemes.is_empty() && !phonemes.ends_with(' ') {
            phonemes.push(' ');
        }

        phonemes.push_str(&espeak_phonemes(trimmed_words));
    }

    words.clear();
}

fn initialize_espeak() -> Result<(), String> {
    let data_directory: PathBuf = locate_espeak_data_directory()?;
    let data_path: CString = CString::new(data_directory.to_string_lossy().as_ref()).map_err(|error| error.to_string())?;

    // SAFETY: both pointers are valid, NUL-terminated and outlive the calls; initialization runs once, under the OnceLock.
    let sample_rate: i32 = unsafe {
        espeak_rs_sys::espeak_Initialize(
            espeak_rs_sys::espeak_AUDIO_OUTPUT_AUDIO_OUTPUT_RETRIEVAL,
            0,
            data_path.as_ptr(),
            espeak_rs_sys::espeakINITIALIZE_DONT_EXIT as i32,
        )
    };

    if sample_rate <= 0 {
        return Err(format!("eSpeak NG did not initialize; [data={} code={sample_rate}]", data_directory.display()));
    }

    Ok(())
}

/* espeak-rs-sys leaves the data it installs in its build output directory, next to the one for this binary. */
fn locate_espeak_data_directory() -> Result<PathBuf, String> {
    if let Ok(configured) = std::env::var(ESPEAK_DATA_DIRECTORY_VARIABLE) {
        return Ok(PathBuf::from(configured));
    }

    let executable: PathBuf = std::env::current_exe().map_err(|error| error.to_string())?;
    let mut newest: Option<(std::time::SystemTime, PathBuf)> = None;

    for profile_directory in executable.ancestors().skip(1).take(2) {
        let Ok(build_entries) = fs::read_dir(profile_directory.join("build")) else {
            continue;
        };

        for build_entry in build_entries.flatten() {
            let is_espeak_build: bool = build_entry.file_name().to_string_lossy().starts_with("espeak-rs-sys-");
            let installed: PathBuf = build_entry.path().join("out/share");

            if is_espeak_build && installed.join(ESPEAK_DATA_DIRECTORY_NAME).join("phondata").exists() {
                let modified: std::time::SystemTime = fs::metadata(&installed).and_then(|metadata| metadata.modified()).unwrap_or(std::time::UNIX_EPOCH);

                if newest.as_ref().is_none_or(|(newest_modified, _)| modified > *newest_modified) {
                    newest = Some((modified, installed));
                }
            }
        }
    }

    newest.map(|(_, directory)| directory).ok_or_else(|| {
        format!("could not find the eSpeak NG data; set {ESPEAK_DATA_DIRECTORY_VARIABLE} to the directory that contains {ESPEAK_DATA_DIRECTORY_NAME}")
    })
}

fn espeak_phonemes(words: &str) -> String {
    let mode: i32 = (((TIE_CHARACTER as u32) << 8) | espeak_rs_sys::espeakPHONEMES_TIE | espeak_rs_sys::espeakINITIALIZE_PHONEME_IPA) as i32;
    let Ok(c_words) = CString::new(words) else {
        return String::new();
    };
    let mut cursor: *const c_char = c_words.as_ptr();
    let mut pieces: Vec<String> = Vec::new();

    while !cursor.is_null() {
        // SAFETY: `cursor` points into `c_words`, which outlives the loop; eSpeak advances it clause by clause and sets it to null at the end.
        let piece_pointer: *const c_char = unsafe {
            espeak_rs_sys::espeak_TextToPhonemes(
                &mut cursor as *mut *const c_char as *mut *const c_void,
                espeak_rs_sys::espeakCHARS_UTF8 as i32,
                mode,
            )
        };

        if piece_pointer.is_null() {
            continue;
        }

        // SAFETY: eSpeak returns a NUL-terminated string that stays valid until the next call.
        let piece: String = unsafe { CStr::from_ptr(piece_pointer) }.to_string_lossy().into_owned();

        if !piece.is_empty() {
            pieces.push(piece);
        }
    }

    pieces.join(" ")
}

/// Splits `text` at sentence ends into chunks of at most about 250 characters, because Kokoro limits the length of one pass.
pub fn split_into_chunks(text: &str) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut current: String = String::new();

    for sentence in split_into_sentences(text) {
        let too_long: bool = current.chars().count() + sentence.chars().count() > MAXIMUM_CHUNK_CHARACTERS;

        if !current.is_empty() && too_long {
            chunks.push(std::mem::take(&mut current));
        }

        if !current.is_empty() {
            current.push(' ');
        }

        current.push_str(&sentence);
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

fn split_into_sentences(text: &str) -> Vec<String> {
    let characters: Vec<(usize, char)> = text.char_indices().collect();
    let mut sentences: Vec<String> = Vec::new();
    let mut sentence_start: usize = 0;

    for (position, &(index, character)) in characters.iter().enumerate() {
        let followed_by_space: bool = characters.get(position + 1).is_none_or(|&(_, next)| next.is_whitespace());

        if SENTENCE_END_CHARACTERS.contains(character) && followed_by_space {
            let end: usize = index + character.len_utf8();

            sentences.push(text[sentence_start..end].trim().to_string());
            sentence_start = end;
        }
    }

    if sentence_start < text.len() {
        sentences.push(text[sentence_start..].trim().to_string());
    }

    sentences.retain(|sentence| !sentence.is_empty());

    sentences
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::speech_engine::BRAZILIAN_PORTUGUESE;

    #[test]
    fn phonemize_keeps_punctuation_and_uses_kokoro_symbols() {
        let phonemizer: Phonemizer = Phonemizer::initialize().expect("eSpeak NG initializes");

        assert_eq!(phonemizer.phonemize(BRAZILIAN_PORTUGUESE.espeak_voice, "Nossa, de novo?").unwrap(), "nˈɔsæ, ʤy nˈovʊ?");
        assert_eq!(phonemizer.phonemize(BRAZILIAN_PORTUGUESE.espeak_voice, "Estou com dor de barriga.").unwrap(), "estˌow koŋ dˈor ʤy bˌaxˈiɡæ.");
    }

    #[test]
    fn phonemize_single_word_has_no_punctuation() {
        let phonemizer: Phonemizer = Phonemizer::initialize().expect("eSpeak NG initializes");

        assert_eq!(phonemizer.phonemize(BRAZILIAN_PORTUGUESE.espeak_voice, "desejar").unwrap(), "dˌezeʒˈar");
    }

    #[test]
    fn split_into_chunks_keeps_short_text_whole() {
        assert_eq!(split_into_chunks("Oi. Tudo bem?"), vec!["Oi. Tudo bem?".to_string()]);
    }

    #[test]
    fn split_into_chunks_breaks_long_text_at_sentence_ends() {
        let sentence: String = format!("{}.", "palavra ".repeat(20).trim_end());
        let text: String = format!("{sentence} {sentence} {sentence}");

        let chunks: Vec<String> = split_into_chunks(&text);

        assert_eq!(chunks.len(), 3);
        assert!(chunks.iter().all(|chunk| chunk.chars().count() <= MAXIMUM_CHUNK_CHARACTERS));
    }

    #[test]
    fn split_into_chunks_of_empty_text_is_empty() {
        assert!(split_into_chunks("   ").is_empty());
    }
}

