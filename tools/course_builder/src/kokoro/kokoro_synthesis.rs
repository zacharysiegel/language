use std::path::Path;

use shared::AppError;

use crate::audio_encoding;
use crate::audio_plan::Utterance;
use crate::kokoro::kokoro_artifacts;
use crate::kokoro::kokoro_models::{self, KokoroFiles};
use crate::kokoro::kokoro_phonemizer::{Phonemizer, split_into_chunks};
use crate::kokoro::kokoro_synthesizer::{self, KokoroSynthesizer, KokoroVocabulary, SAMPLE_RATE, VoiceTables};
use crate::work_pool;

const INFERENCE_WORKER_COUNT: usize = 4;
const INTRA_OP_THREADS_PER_WORKER: usize = 4;

struct PreparedChunk {
    phonemes: String,
    tokens: Vec<i64>,
}

struct PreparedClip<'a> {
    file_name: &'a str,
    text: &'a str,
    voice_id: &'static str,
    speed: f32,
    chunks: Vec<PreparedChunk>,
}

pub fn synthesize_clips(
    clips: &[(&String, &Utterance)],
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
        &prepared_clips,
        INFERENCE_WORKER_COUNT,
        |clip| format!("{} [{}]", clip.text, clip.voice_id),
        || KokoroSynthesizer::load(&files.model, INTRA_OP_THREADS_PER_WORKER),
        |synthesizer, clip| synthesize_clip(synthesizer, &voice_tables, clip, audio_directory),
    )
}

/* eSpeak keeps process-wide state, so every clip is phonemized here on one thread before inference fans out. */
fn prepare_clips<'a>(clips: &[(&'a String, &'a Utterance)], files: &KokoroFiles) -> Result<Vec<PreparedClip<'a>>, AppError> {
    let phonemizer: Phonemizer = Phonemizer::initialize()?;
    let vocabulary: KokoroVocabulary = KokoroVocabulary::load(&files.config)?;
    let mut prepared_clips: Vec<PreparedClip> = Vec::with_capacity(clips.len());

    for (file_name, utterance) in clips {
        let chunks: Vec<PreparedChunk> = split_into_chunks(&utterance.text)
            .iter()
            .map(|chunk| {
                let phonemes: String = phonemizer.phonemize(chunk);
                let tokens: Vec<i64> = vocabulary.tokens_for(&phonemes);

                PreparedChunk { phonemes, tokens }
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

fn synthesize_clip(
    synthesizer: &mut KokoroSynthesizer,
    voice_tables: &VoiceTables,
    clip: &PreparedClip,
    audio_directory: &Path,
) -> Result<(), AppError> {
    let mut samples: Vec<f32> = Vec::new();

    for chunk in &clip.chunks {
        let chunk_samples: Vec<f32> = synthesizer.synthesize(&chunk.tokens, voice_tables, clip.voice_id, clip.speed)?;

        samples.extend(kokoro_artifacts::remove_spurious_noise(&chunk_samples, &chunk.phonemes));
    }

    audio_encoding::encode_samples(&samples, SAMPLE_RATE, &audio_directory.join(clip.file_name))
}
