use std::fs;
use std::path::{Path, PathBuf};

use shared::artifact::content_address;

use crate::model_hosting::HostedModel;
use crate::model_hosting::gguf_file::{self, GgufKeyValue, GgufTensorInfo};
use crate::model_hosting::gguf_split;

pub const MODEL_FILE_NAME: &str = "model.gguf";

const F32_TYPE_ID: u32 = 0;

struct FixtureTensor {
    name: &'static str,
    dimensions: Vec<i64>,
}

fn string_key_value(key: &str, value: &str) -> GgufKeyValue {
    let mut encoded: Vec<u8> = Vec::new();

    gguf_file::encode_string(&mut encoded, key);
    encoded.extend_from_slice(&gguf_file::VALUE_TYPE_STRING.to_le_bytes());
    gguf_file::encode_string(&mut encoded, value);

    GgufKeyValue {
        key: key.to_string(),
        value_type: gguf_file::VALUE_TYPE_STRING,
        encoded,
    }
}

fn string_array_key_value(key: &str, values: &[&str]) -> GgufKeyValue {
    let mut encoded: Vec<u8> = Vec::new();

    gguf_file::encode_string(&mut encoded, key);
    encoded.extend_from_slice(&gguf_file::VALUE_TYPE_ARRAY.to_le_bytes());
    encoded.extend_from_slice(&gguf_file::VALUE_TYPE_STRING.to_le_bytes());
    encoded.extend_from_slice(&(values.len() as u64).to_le_bytes());

    for value in values {
        gguf_file::encode_string(&mut encoded, value);
    }

    GgufKeyValue {
        key: key.to_string(),
        value_type: gguf_file::VALUE_TYPE_ARRAY,
        encoded,
    }
}

fn tensor_bytes(tensor_index: usize, size_bytes: u64) -> Vec<u8> {
    (0..size_bytes).map(|byte_index| (tensor_index as u64 * 31 + byte_index) as u8 | 1).collect()
}

/// Four F32 tensors of 32, 24, 80 and 16 bytes, the last with a trailing dimension of 1, after a stale `split.count`.
pub fn write_fixture_model(directory: &Path) -> PathBuf {
    let fixture_tensors: Vec<FixtureTensor> = vec![
        FixtureTensor {
            name: "token_embd.weight",
            dimensions: vec![8],
        },
        FixtureTensor {
            name: "blk.0.attn_q.weight",
            dimensions: vec![3, 2],
        },
        FixtureTensor {
            name: "blk.0.ffn_up.weight",
            dimensions: vec![20],
        },
        FixtureTensor {
            name: "output_norm.weight",
            dimensions: vec![4, 1],
        },
    ];
    let key_values: Vec<GgufKeyValue> = vec![
        string_key_value("general.architecture", "gemma4"),
        string_array_key_value("tokenizer.ggml.tokens", &["<pad>", "olá", "mundo"]),
        gguf_file::encode_key_value_u16(gguf_split::SPLIT_COUNT_KEY, 0),
    ];
    let mut tensor_infos: Vec<GgufTensorInfo> = Vec::new();
    let mut data_bytes: u64 = 0;

    for fixture_tensor in &fixture_tensors {
        let size_bytes: u64 = 4 * fixture_tensor.dimensions.iter().product::<i64>() as u64;

        tensor_infos.push(GgufTensorInfo {
            name: fixture_tensor.name.to_string(),
            dimensions: fixture_tensor.dimensions.clone(),
            type_id: F32_TYPE_ID,
            offset: data_bytes,
            size_bytes,
        });
        data_bytes += gguf_file::pad_to_alignment(size_bytes, gguf_file::GGUF_DEFAULT_ALIGNMENT);
    }

    let key_value_references: Vec<&GgufKeyValue> = key_values.iter().collect();
    let tensor_info_references: Vec<&GgufTensorInfo> = tensor_infos.iter().collect();
    let tensor_offsets: Vec<u64> = tensor_infos.iter().map(|tensor_info| tensor_info.offset).collect();
    let mut file_bytes: Vec<u8> = gguf_file::encode_header(&key_value_references, &tensor_info_references, &tensor_offsets, gguf_file::GGUF_DEFAULT_ALIGNMENT);

    for (tensor_index, tensor_info) in tensor_infos.iter().enumerate() {
        file_bytes.extend(tensor_bytes(tensor_index, tensor_info.size_bytes));
        file_bytes.resize(gguf_file::pad_to_alignment(file_bytes.len() as u64, gguf_file::GGUF_DEFAULT_ALIGNMENT) as usize, 0);
    }

    let path: PathBuf = directory.join(MODEL_FILE_NAME);

    fs::write(&path, file_bytes).unwrap();

    path
}

pub fn fixture_hosted_model(model_file: &Path, part_count: u16) -> HostedModel {
    let bytes: Vec<u8> = fs::read(model_file).unwrap();

    HostedModel {
        name: "fixture".to_string(),
        file_name: MODEL_FILE_NAME.to_string(),
        sha256: content_address::sha256_hex(&bytes),
        size_bytes: bytes.len() as u64,
        part_count,
    }
}
