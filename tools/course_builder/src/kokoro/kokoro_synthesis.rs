use std::path::Path;

use shared::AppError;

use crate::audio_encoding;
use crate::kokoro::kokoro_artifacts;
use crate::kokoro::kokoro_models::{self, KokoroFiles};
use crate::kokoro::kokoro_phonemizer::{Phonemizer, split_into_chunks};
use crate::kokoro::kokoro_short_words::{self, CARRIER_PHONEMES};
use crate::kokoro::kokoro_synthesizer::{self, KokoroSynthesizer, KokoroVocabulary, SAMPLE_RATE, VoiceTables};
use crate::speech_engine::{ResolvedUtterance, spoken_text_for};
use crate::work_pool;

const INFERENCE_WORKER_COUNT: usize = 4;
const SHORT_INPUT_TOKEN_LIMIT: usize = 5;
const INTRA_OP_THREADS_PER_WORKER: usize = 4;

struct PreparedChunk {
    phonemes: String,
    tokens: Vec<i64>,
    carrier_tokens: Option<Vec<i64>>,
}

struct PreparedClip<'a> {
    file_name: &'a str,
    text: &'a str,
    voice_id: &'static str,
    speed: f32,
    chunks: Vec<PreparedChunk>,
}

pub fn synthesize_clips(
    clips: &[(String, ResolvedUtterance)],
    audio_directory: &Path,
    models_directory: &Path,
) -> Result<(), AppError> {
    if clips.is_empty() {
        return Ok(());
    }

    let files: KokoroFiles = kokoro_models::ensure_kokoro_files(models_directory)?;

    kokoro_synthesizer::initialize_runtime(&files.onnx_runtime_library)?;

    let prepared_clips: Vec<PreparedClip> = prepare_clips(clips, &files)?;
    let voice_ids: Vec<&str> = distinct_voice_ids(&prepared_clips);
    let voice_tables: VoiceTables = VoiceTables::load(&files, &voice_ids)?;

    work_pool::run_work_pool(
        "kokoro synthesis",
        &prepared_clips,
        INFERENCE_WORKER_COUNT,
        |clip| format!("{} [{}]", clip.text, clip.voice_id),
        || KokoroSynthesizer::load(&files.model, INTRA_OP_THREADS_PER_WORKER),
        |synthesizer, clip| synthesize_clip(synthesizer, &voice_tables, clip, audio_directory),
    )
}

/* eSpeak keeps process-wide state, so every clip is phonemized here on one thread before inference fans out. */
fn prepare_clips<'a>(clips: &'a [(String, ResolvedUtterance)], files: &KokoroFiles) -> Result<Vec<PreparedClip<'a>>, AppError> {
    let phonemizer: Phonemizer = Phonemizer::initialize()?;
    let vocabulary: KokoroVocabulary = KokoroVocabulary::load(&files.config)?;
    let mut prepared_clips: Vec<PreparedClip> = Vec::with_capacity(clips.len());

    for (file_name, utterance) in clips {
        let chunks: Vec<PreparedChunk> = split_into_chunks(&spoken_text_for(&utterance.text))
            .iter()
            .map(|chunk| {
                let phonemes: String = phonemizer.phonemize(chunk);
                let tokens: Vec<i64> = vocabulary.tokens_for(&phonemes);

                let carrier_tokens: Option<Vec<i64>> = (tokens.len() <= SHORT_INPUT_TOKEN_LIMIT && !phonemes.contains(' ')).then(|| {
                    let bare_phonemes: &str = phonemes.trim_end_matches(|symbol: char| ".,!?;:…".contains(symbol));

                    vocabulary.tokens_for(&format!("{bare_phonemes}, {CARRIER_PHONEMES}"))
                });

                PreparedChunk { phonemes, tokens, carrier_tokens }
            })
            .filter(|chunk| !chunk.tokens.is_empty())
            .collect();

        if chunks.is_empty() {
            return Err(AppError::from(format!("text produced no phonemes; [text={}]", utterance.text)));
        }

        prepared_clips.push(PreparedClip {
            file_name,
            text: &utterance.text,
            voice_id: utterance.profile.voice_id,
            speed: utterance.profile.speed,
            chunks,
        });
    }

    Ok(prepared_clips)
}

fn distinct_voice_ids<'a>(prepared_clips: &[PreparedClip<'a>]) -> Vec<&'static str> {
    let mut voice_ids: Vec<&'static str> = prepared_clips.iter().map(|clip| clip.voice_id).collect();

    voice_ids.sort_unstable();
    voice_ids.dedup();

    voice_ids
}

/* Kokoro is unreliable for one or two sounds on their own, so a very short input is spoken before a carrier word and cut out at the pause. */
fn synthesize_chunk(
    synthesizer: &mut KokoroSynthesizer,
    voice_tables: &VoiceTables,
    clip: &PreparedClip,
    chunk: &PreparedChunk,
) -> Result<Vec<f32>, AppError> {
    if let Some(carrier_tokens) = &chunk.carrier_tokens {
        let carrier_samples: Vec<f32> = synthesizer.synthesize(carrier_tokens, voice_tables, clip.voice_id, clip.speed)?;

        if let Some(word_samples) = kokoro_short_words::leading_word_before_pause(&carrier_samples) {
            return Ok(word_samples);
        }
    }

    synthesizer.synthesize(&chunk.tokens, voice_tables, clip.voice_id, clip.speed)
}

fn synthesize_clip(
    synthesizer: &mut KokoroSynthesizer,
    voice_tables: &VoiceTables,
    clip: &PreparedClip,
    audio_directory: &Path,
) -> Result<(), AppError> {
    let mut samples: Vec<f32> = Vec::new();

    for chunk in &clip.chunks {
        let chunk_samples: Vec<f32> = synthesize_chunk(synthesizer, voice_tables, clip, chunk)?;

        samples.extend(kokoro_artifacts::remove_spurious_noise(&chunk_samples, &chunk.phonemes));
    }

    audio_encoding::encode_samples(&samples, SAMPLE_RATE, &audio_directory.join(clip.file_name))
}
