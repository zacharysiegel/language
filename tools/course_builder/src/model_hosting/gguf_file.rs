use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use shared::AppError;

use crate::model_hosting::ggml_tensor_type;

pub const GGUF_MAGIC: [u8; 4] = *b"GGUF";
pub const GGUF_VERSION: u32 = 3;
pub const GGUF_DEFAULT_ALIGNMENT: u64 = 32;
pub const GENERAL_ALIGNMENT_KEY: &str = "general.alignment";

/* GGUF's `enum gguf_type` values. */
pub const VALUE_TYPE_UINT8: u32 = 0;
pub const VALUE_TYPE_INT8: u32 = 1;
pub const VALUE_TYPE_UINT16: u32 = 2;
pub const VALUE_TYPE_INT16: u32 = 3;
pub const VALUE_TYPE_UINT32: u32 = 4;
pub const VALUE_TYPE_INT32: u32 = 5;
pub const VALUE_TYPE_FLOAT32: u32 = 6;
pub const VALUE_TYPE_BOOL: u32 = 7;
pub const VALUE_TYPE_STRING: u32 = 8;
pub const VALUE_TYPE_ARRAY: u32 = 9;
pub const VALUE_TYPE_UINT64: u32 = 10;
pub const VALUE_TYPE_INT64: u32 = 11;
pub const VALUE_TYPE_FLOAT64: u32 = 12;

/* Version 1 counted with 32-bit integers; llama.cpp reads only versions 2 and 3. */
const OLDEST_READABLE_VERSION: u32 = 2;
const MAXIMUM_DIMENSION_COUNT: u32 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GgufKeyValue {
    pub key: String,
    pub value_type: u32,
    /// The key, the value's type and the value, as the file encodes them.
    pub encoded: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GgufTensorInfo {
    pub name: String,
    /// ggml's `ne`, as many as the file states.
    pub dimensions: Vec<i64>,
    pub type_id: u32,
    /// Relative to the start of the data section.
    pub offset: u64,
    pub size_bytes: u64,
}

/// A GGUF file's header: everything before its tensor data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GgufHeader {
    pub key_values: Vec<GgufKeyValue>,
    pub tensor_infos: Vec<GgufTensorInfo>,
    pub alignment: u64,
    /// From the start of the file.
    pub data_offset: u64,
}

pub fn read_gguf_header_of_file(path: &Path) -> Result<GgufHeader, AppError> {
    let file: File = File::open(path).map_err(|error| AppError::from(format!("opening a GGUF file failed; [path={} error={error}]", path.display())))?;
    let mut reader: GgufReader<BufReader<File>> = GgufReader::new(BufReader::new(file));

    read_gguf_header(&mut reader).map_err(|error| AppError::from(format!("reading a GGUF header failed; [path={} error={error}]", path.display())))
}

pub fn read_gguf_header<R: Read>(reader: &mut GgufReader<R>) -> Result<GgufHeader, AppError> {
    let magic: [u8; 4] = reader.read_array()?;

    if magic != GGUF_MAGIC {
        return Err(AppError::from("the file does not begin with the GGUF magic"));
    }

    let version: u32 = reader.read_u32()?;

    if !(OLDEST_READABLE_VERSION..=GGUF_VERSION).contains(&version) {
        return Err(AppError::from(format!("the GGUF version is not one llama.cpp reads; [version={version}]")));
    }

    let tensor_count: u64 = reader.read_u64()?;
    let key_value_count: u64 = reader.read_u64()?;
    let mut key_values: Vec<GgufKeyValue> = Vec::new();

    for _ in 0..key_value_count {
        key_values.push(read_key_value(reader)?);
    }

    let alignment: u64 = read_alignment(&key_values)?;
    let mut tensor_infos: Vec<GgufTensorInfo> = Vec::new();

    for _ in 0..tensor_count {
        tensor_infos.push(read_tensor_info(reader)?);
    }

    Ok(GgufHeader {
        key_values,
        tensor_infos,
        alignment,
        data_offset: pad_to_alignment(reader.position, alignment),
    })
}

fn read_key_value<R: Read>(reader: &mut GgufReader<R>) -> Result<GgufKeyValue, AppError> {
    let mut encoded: Vec<u8> = Vec::new();
    let key: String = reader.read_string()?;
    let value_type: u32 = reader.read_u32()?;

    encode_string(&mut encoded, &key);
    encoded.extend_from_slice(&value_type.to_le_bytes());

    if value_type == VALUE_TYPE_ARRAY {
        let element_type: u32 = reader.read_u32()?;
        let element_count: u64 = reader.read_u64()?;

        encoded.extend_from_slice(&element_type.to_le_bytes());
        encoded.extend_from_slice(&element_count.to_le_bytes());

        for _ in 0..element_count {
            read_scalar_value(reader, &key, element_type, &mut encoded)?;
        }
    } else {
        read_scalar_value(reader, &key, value_type, &mut encoded)?;
    }

    Ok(GgufKeyValue { key, value_type, encoded })
}

fn read_scalar_value<R: Read>(reader: &mut GgufReader<R>, key: &str, value_type: u32, encoded: &mut Vec<u8>) -> Result<(), AppError> {
    let size_bytes: usize = match value_type {
        VALUE_TYPE_UINT8 | VALUE_TYPE_INT8 | VALUE_TYPE_BOOL => 1,
        VALUE_TYPE_UINT16 | VALUE_TYPE_INT16 => 2,
        VALUE_TYPE_UINT32 | VALUE_TYPE_INT32 | VALUE_TYPE_FLOAT32 => 4,
        VALUE_TYPE_UINT64 | VALUE_TYPE_INT64 | VALUE_TYPE_FLOAT64 => 8,
        VALUE_TYPE_STRING => {
            let text: String = reader.read_string()?;

            encode_string(encoded, &text);
            return Ok(());
        },
        _ => return Err(AppError::from(format!("a metadata value has a type GGUF does not define; [key={key} type={value_type}]"))),
    };

    encoded.extend_from_slice(&reader.read_bytes(size_bytes as u64)?);

    Ok(())
}

fn read_alignment(key_values: &[GgufKeyValue]) -> Result<u64, AppError> {
    let Some(alignment_key_value) = key_values.iter().find(|key_value| key_value.key == GENERAL_ALIGNMENT_KEY)
    else {
        return Ok(GGUF_DEFAULT_ALIGNMENT);
    };

    if alignment_key_value.value_type != VALUE_TYPE_UINT32 {
        return Err(AppError::from(format!("{GENERAL_ALIGNMENT_KEY} is not of type u32")));
    }

    let value_offset: usize = alignment_key_value.encoded.len() - 4;
    let alignment: u32 = u32::from_le_bytes(alignment_key_value.encoded[value_offset..].try_into().unwrap());

    if !alignment.is_power_of_two() {
        return Err(AppError::from(format!("{GENERAL_ALIGNMENT_KEY} is not a power of two; [alignment={alignment}]")));
    }

    Ok(u64::from(alignment))
}

fn read_tensor_info<R: Read>(reader: &mut GgufReader<R>) -> Result<GgufTensorInfo, AppError> {
    let name: String = reader.read_string()?;
    let dimension_count: u32 = reader.read_u32()?;

    if dimension_count > MAXIMUM_DIMENSION_COUNT {
        return Err(AppError::from(format!("a tensor has more dimensions than ggml allows; [tensor={name} dimensions={dimension_count}]")));
    }

    let mut dimensions: Vec<i64> = Vec::new();

    for _ in 0..dimension_count {
        dimensions.push(reader.read_i64()?);
    }

    let type_id: u32 = reader.read_u32()?;
    let offset: u64 = reader.read_u64()?;
    let size_bytes: u64 = ggml_tensor_type::tensor_size_bytes(&name, type_id, &dimensions)?;

    Ok(GgufTensorInfo {
        name,
        dimensions,
        type_id,
        offset,
        size_bytes,
    })
}

pub fn pad_to_alignment(position: u64, alignment: u64) -> u64 {
    position.div_ceil(alignment) * alignment
}

pub fn encode_string(encoded: &mut Vec<u8>, text: &str) {
    encoded.extend_from_slice(&(text.len() as u64).to_le_bytes());
    encoded.extend_from_slice(text.as_bytes());
}

pub fn encode_key_value_u16(key: &str, value: u16) -> GgufKeyValue {
    encode_scalar_key_value(key, VALUE_TYPE_UINT16, &value.to_le_bytes())
}

pub fn encode_key_value_i32(key: &str, value: i32) -> GgufKeyValue {
    encode_scalar_key_value(key, VALUE_TYPE_INT32, &value.to_le_bytes())
}

fn encode_scalar_key_value(key: &str, value_type: u32, value_bytes: &[u8]) -> GgufKeyValue {
    let mut encoded: Vec<u8> = Vec::new();

    encode_string(&mut encoded, key);
    encoded.extend_from_slice(&value_type.to_le_bytes());
    encoded.extend_from_slice(value_bytes);

    GgufKeyValue {
        key: key.to_string(),
        value_type,
        encoded,
    }
}

/* llama.cpp writes `ggml_n_dims`: the dimensions up to the last one above 1, and at least one. */
fn written_dimension_count(dimensions: &[i64]) -> usize {
    let last_significant_index: Option<usize> = dimensions.iter().rposition(|&dimension| dimension > 1);

    last_significant_index.map_or(1, |index| index + 1)
}

/// The header of a GGUF file holding `tensor_infos` in order, each at the offset `tensor_offsets` gives.
pub fn encode_header(key_values: &[&GgufKeyValue], tensor_infos: &[&GgufTensorInfo], tensor_offsets: &[u64], alignment: u64) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();

    encoded.extend_from_slice(&GGUF_MAGIC);
    encoded.extend_from_slice(&GGUF_VERSION.to_le_bytes());
    encoded.extend_from_slice(&(tensor_infos.len() as u64).to_le_bytes());
    encoded.extend_from_slice(&(key_values.len() as u64).to_le_bytes());

    for key_value in key_values {
        encoded.extend_from_slice(&key_value.encoded);
    }

    for (tensor_info, tensor_offset) in tensor_infos.iter().zip(tensor_offsets) {
        encode_tensor_info(&mut encoded, tensor_info, *tensor_offset);
    }

    let padded_length: u64 = pad_to_alignment(encoded.len() as u64, alignment);

    encoded.resize(padded_length as usize, 0);

    encoded
}

fn encode_tensor_info(encoded: &mut Vec<u8>, tensor_info: &GgufTensorInfo, tensor_offset: u64) {
    let dimension_count: usize = written_dimension_count(&tensor_info.dimensions);

    encode_string(encoded, &tensor_info.name);
    encoded.extend_from_slice(&(dimension_count as u32).to_le_bytes());

    for index in 0..dimension_count {
        let dimension: i64 = tensor_info.dimensions.get(index).copied().unwrap_or(1);

        encoded.extend_from_slice(&dimension.to_le_bytes());
    }

    encoded.extend_from_slice(&tensor_info.type_id.to_le_bytes());
    encoded.extend_from_slice(&tensor_offset.to_le_bytes());
}

/// Counts the bytes read, which locate the data section.
pub struct GgufReader<R: Read> {
    reader: R,
    position: u64,
}

impl<R: Read> GgufReader<R> {
    pub fn new(reader: R) -> GgufReader<R> {
        GgufReader { reader, position: 0 }
    }

    fn read_array<const LENGTH: usize>(&mut self) -> Result<[u8; LENGTH], AppError> {
        let mut bytes: [u8; LENGTH] = [0; LENGTH];

        self.reader.read_exact(&mut bytes)?;
        self.position += LENGTH as u64;

        Ok(bytes)
    }

    fn read_bytes(&mut self, length: u64) -> Result<Vec<u8>, AppError> {
        let mut bytes: Vec<u8> = Vec::new();
        let read_count: usize = (&mut self.reader).take(length).read_to_end(&mut bytes)?;

        if read_count as u64 != length {
            return Err(AppError::from(format!("the GGUF header ends early; [position={} wanted_bytes={length}]", self.position)));
        }

        self.position += length;

        Ok(bytes)
    }

    fn read_u32(&mut self) -> Result<u32, AppError> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    fn read_u64(&mut self) -> Result<u64, AppError> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    fn read_i64(&mut self) -> Result<i64, AppError> {
        Ok(i64::from_le_bytes(self.read_array()?))
    }

    fn read_string(&mut self) -> Result<String, AppError> {
        let length: u64 = self.read_u64()?;
        let bytes: Vec<u8> = self.read_bytes(length)?;

        String::from_utf8(bytes).map_err(|error| AppError::from(format!("a GGUF string is not UTF-8; [position={} error={error}]", self.position)))
    }
}
