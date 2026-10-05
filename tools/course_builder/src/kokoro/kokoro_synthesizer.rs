use std::collections::HashMap;
use std::fs;
use std::path::Path;

use ort::session::Session;
use ort::session::builder::SessionBuilder;
use ort::value::Tensor;
use shared::AppError;

use crate::kokoro::kokoro_models::KokoroFiles;

pub const SAMPLE_RATE: u32 = 24000;

const STYLE_WIDTH: usize = 256;
const MAXIMUM_TOKENS: usize = 510;
const PADDING_TOKEN: i64 = 0;

fn ort_error(context: &str, error: impl std::fmt::Display) -> AppError {
    AppError::from(format!("{context}; [error={error}]"))
}

pub fn initialize_runtime(library_path: &Path) -> Result<(), AppError> {
    let builder = ort::init_from(library_path).map_err(|error| ort_error("could not load the ONNX Runtime library", error))?;

    builder.commit();

    Ok(())
}

/// Maps each phoneme symbol to the token Kokoro was trained with.
pub struct KokoroVocabulary {
    token_by_symbol: HashMap<char, i64>,
}

impl KokoroVocabulary {
    pub fn load(config_path: &Path) -> Result<KokoroVocabulary, AppError> {
        let config_text: String = fs::read_to_string(config_path)?;
        let config: serde_json::Value = serde_json::from_str(&config_text)?;
        let entries: &serde_json::Map<String, serde_json::Value> = config["vocab"]
            .as_object()
            .ok_or_else(|| AppError::from("the Kokoro config has no vocabulary".to_string()))?;
        let mut token_by_symbol: HashMap<char, i64> = HashMap::new();

        for (symbol, token) in entries {
            if let (Some(character), Some(token)) = (symbol.chars().next(), token.as_i64()) {
                token_by_symbol.insert(character, token);
            }
        }

        Ok(KokoroVocabulary { token_by_symbol })
    }

    /// Symbols outside the vocabulary are dropped, as the reference implementation does.
    pub fn tokens_for(&self, phonemes: &str) -> Vec<i64> {
        let mut tokens: Vec<i64> = phonemes.chars().filter_map(|symbol| self.token_by_symbol.get(&symbol).copied()).collect();

        tokens.truncate(MAXIMUM_TOKENS);

        tokens
    }
}

/// Each voice is a table of style vectors, one per possible token count.
pub struct VoiceTables {
    style_table_by_voice: HashMap<String, Vec<f32>>,
}

impl VoiceTables {
    pub fn load(files: &KokoroFiles, voice_ids: &[&str]) -> Result<VoiceTables, AppError> {
        let mut style_table_by_voice: HashMap<String, Vec<f32>> = HashMap::new();

        for voice_id in voice_ids {
            let bytes: Vec<u8> = fs::read(files.voice_path(voice_id))?;
            let table: Vec<f32> = bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();

            style_table_by_voice.insert((*voice_id).to_string(), table);
        }

        Ok(VoiceTables { style_table_by_voice })
    }

    fn style_for(&self, voice_id: &str, token_count: usize) -> Result<Vec<f32>, AppError> {
        let table: &Vec<f32> = self
            .style_table_by_voice
            .get(voice_id)
            .ok_or_else(|| AppError::from(format!("voice was not loaded; [voice={voice_id}]")))?;
        let start: usize = token_count * STYLE_WIDTH;

        table
            .get(start..start + STYLE_WIDTH)
            .map(<[f32]>::to_vec)
            .ok_or_else(|| AppError::from(format!("voice table is too short; [voice={voice_id} tokens={token_count}]")))
    }
}

pub struct KokoroSynthesizer {
    session: Session,
}

impl KokoroSynthesizer {
    pub fn load(model_path: &Path, intra_op_threads: usize) -> Result<KokoroSynthesizer, AppError> {
        let builder: SessionBuilder = Session::builder().map_err(|error| ort_error("could not create a session", error))?;
        let mut builder: SessionBuilder = builder
            .with_intra_threads(intra_op_threads)
            .map_err(|error| ort_error("could not set the thread count", error))?;
        let session: Session = builder
            .commit_from_file(model_path)
            .map_err(|error| ort_error("could not load the Kokoro model", error))?;

        Ok(KokoroSynthesizer { session })
    }

    pub fn synthesize(&mut self, tokens: &[i64], voice_tables: &VoiceTables, voice_id: &str, speed: f32) -> Result<Vec<f32>, AppError> {
        let style: Vec<f32> = voice_tables.style_for(voice_id, tokens.len())?;
        let mut padded_tokens: Vec<i64> = Vec::with_capacity(tokens.len() + 2);

        padded_tokens.push(PADDING_TOKEN);
        padded_tokens.extend_from_slice(tokens);
        padded_tokens.push(PADDING_TOKEN);

        let token_count: usize = padded_tokens.len();
        let input_ids: Tensor<i64> = Tensor::from_array(([1usize, token_count], padded_tokens)).map_err(|error| ort_error("could not build the token tensor", error))?;
        let style_tensor: Tensor<f32> = Tensor::from_array(([1usize, STYLE_WIDTH], style)).map_err(|error| ort_error("could not build the style tensor", error))?;
        let speed_tensor: Tensor<f32> = Tensor::from_array(([1usize], vec![speed])).map_err(|error| ort_error("could not build the speed tensor", error))?;

        let outputs = self
            .session
            .run(ort::inputs!["input_ids" => input_ids, "style" => style_tensor, "speed" => speed_tensor])
            .map_err(|error| ort_error("Kokoro inference failed", error))?;
        let (_, samples) = outputs[0]
            .try_extract_tensor::<f32>()
            .map_err(|error| ort_error("could not read the Kokoro output", error))?;

        Ok(samples.to_vec())
    }
}
