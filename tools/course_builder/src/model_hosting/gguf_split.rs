use std::fs::File;
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

use shared::AppError;

use crate::model_hosting::gguf_file::{self, GgufHeader, GgufKeyValue, GgufTensorInfo};

pub const SPLIT_NUMBER_KEY: &str = "split.no";
pub const SPLIT_COUNT_KEY: &str = "split.count";
pub const SPLIT_TENSOR_COUNT_KEY: &str = "split.tensors.count";

const WRITE_BUFFER_BYTES: usize = 8 * 1024 * 1024;

/// A part of llama.cpp's split format, itself a GGUF file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GgufSplitPart {
    /// Counts from 1.
    pub part_number: u16,
    /// Into the source's tensors, in the source's order.
    pub tensor_indices: Vec<usize>,
    pub header: Vec<u8>,
    pub alignment: u64,
    pub size_bytes: u64,
}

/// Starts a part before a tensor that would take the part's padded tensor data past `maximum_part_tensor_bytes`,
/// as `llama-gguf-split --split-max-size` does; a tensor larger than that has a part of its own.
pub fn plan_split(source: &GgufHeader, maximum_part_tensor_bytes: u64) -> Result<Vec<GgufSplitPart>, AppError> {
    let tensor_groups: Vec<Vec<usize>> = group_tensors_into_parts(source, maximum_part_tensor_bytes);
    let part_count: u16 =
        u16::try_from(tensor_groups.len()).map_err(|_| AppError::from(format!("a split has more parts than GGUF counts; [parts={}]", tensor_groups.len())))?;
    let tensor_count: i32 = i32::try_from(source.tensor_infos.len())
        .map_err(|_| AppError::from(format!("a model has more tensors than GGUF counts; [tensors={}]", source.tensor_infos.len())))?;

    let parts: Vec<GgufSplitPart> = tensor_groups
        .into_iter()
        .zip(0..part_count)
        .map(|(tensor_indices, part_index)| plan_part(source, part_index, part_count, tensor_count, tensor_indices))
        .collect();

    Ok(parts)
}

/* llama-gguf-split measures a part by its tensors padded to GGUF's default alignment, whatever the file's alignment. */
fn group_tensors_into_parts(source: &GgufHeader, maximum_part_tensor_bytes: u64) -> Vec<Vec<usize>> {
    let mut tensor_groups: Vec<Vec<usize>> = vec![Vec::new()];
    let mut part_tensor_bytes: u64 = 0;

    for (tensor_index, tensor_info) in source.tensor_infos.iter().enumerate() {
        let padded_size_bytes: u64 = gguf_file::pad_to_alignment(tensor_info.size_bytes, gguf_file::GGUF_DEFAULT_ALIGNMENT);
        let part_has_tensors: bool = tensor_groups.last().is_some_and(|tensor_group| !tensor_group.is_empty());

        if part_has_tensors && part_tensor_bytes + padded_size_bytes > maximum_part_tensor_bytes {
            tensor_groups.push(Vec::new());
            part_tensor_bytes = 0;
        }

        part_tensor_bytes += padded_size_bytes;
        tensor_groups.last_mut().expect("a split has a part").push(tensor_index);
    }

    tensor_groups
}

fn plan_part(source: &GgufHeader, part_index: u16, part_count: u16, tensor_count: i32, tensor_indices: Vec<usize>) -> GgufSplitPart {
    let split_key_values: [GgufKeyValue; 3] = [
        gguf_file::encode_key_value_u16(SPLIT_NUMBER_KEY, part_index),
        gguf_file::encode_key_value_i32(SPLIT_TENSOR_COUNT_KEY, tensor_count),
        gguf_file::encode_key_value_u16(SPLIT_COUNT_KEY, part_count),
    ];
    /* The order llama-gguf-split writes: the source's metadata in the first part only, then the split keys with
    `split.count` last, since it sets that key again once the count is known. */
    let mut key_values: Vec<&GgufKeyValue> = Vec::new();

    if part_index == 0 {
        key_values.extend(source.key_values.iter().filter(|key_value| !is_split_key(&key_value.key)));
    }

    key_values.extend(split_key_values.iter());

    let alignment: u64 = if part_index == 0 { source.alignment } else { gguf_file::GGUF_DEFAULT_ALIGNMENT };
    let tensor_infos: Vec<&GgufTensorInfo> = tensor_indices.iter().map(|&tensor_index| &source.tensor_infos[tensor_index]).collect();
    let mut tensor_offsets: Vec<u64> = Vec::new();
    let mut tensor_data_bytes: u64 = 0;

    for tensor_info in &tensor_infos {
        tensor_offsets.push(tensor_data_bytes);
        tensor_data_bytes += gguf_file::pad_to_alignment(tensor_info.size_bytes, alignment);
    }

    let header: Vec<u8> = gguf_file::encode_header(&key_values, &tensor_infos, &tensor_offsets, alignment);
    let size_bytes: u64 = header.len() as u64 + tensor_data_bytes;

    GgufSplitPart {
        part_number: part_index + 1,
        tensor_indices,
        header,
        alignment,
        size_bytes,
    }
}

fn is_split_key(key: &str) -> bool {
    key == SPLIT_NUMBER_KEY || key == SPLIT_COUNT_KEY || key == SPLIT_TENSOR_COUNT_KEY
}

pub fn write_part(source_file: &mut File, source: &GgufHeader, part: &GgufSplitPart, destination: &Path) -> Result<(), AppError> {
    let destination_file: File = File::create(destination)
        .map_err(|error| AppError::from(format!("creating a model part failed; [path={} error={error}]", destination.display())))?;
    let mut writer: BufWriter<File> = BufWriter::with_capacity(WRITE_BUFFER_BYTES, destination_file);

    writer.write_all(&part.header)?;

    for &tensor_index in &part.tensor_indices {
        let tensor_info: &GgufTensorInfo = &source.tensor_infos[tensor_index];
        let padding_bytes: u64 = gguf_file::pad_to_alignment(tensor_info.size_bytes, part.alignment) - tensor_info.size_bytes;

        copy_tensor_data(source_file, source, tensor_info, &mut writer)?;
        writer.write_all(&vec![0; padding_bytes as usize])?;
    }

    writer.flush()?;

    let written_size_bytes: u64 = writer.get_ref().metadata()?.len();

    if written_size_bytes != part.size_bytes {
        return Err(AppError::from(format!(
            "a model part has another size than planned; [path={} planned={} written={written_size_bytes}]",
            destination.display(),
            part.size_bytes,
        )));
    }

    Ok(())
}

fn copy_tensor_data(source_file: &mut File, source: &GgufHeader, tensor_info: &GgufTensorInfo, writer: &mut impl Write) -> Result<(), AppError> {
    source_file.seek(SeekFrom::Start(source.data_offset + tensor_info.offset))?;

    let copied_bytes: u64 = io::copy(&mut (&mut *source_file).take(tensor_info.size_bytes), writer)?;

    if copied_bytes != tensor_info.size_bytes {
        return Err(AppError::from(format!("the model file ends inside a tensor; [tensor={}]", tensor_info.name)));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use shared::artifact::repository_key;

    use super::*;
    use crate::model_hosting::gguf_file::GgufReader;
    use crate::model_hosting::test_model_files;

    struct WrittenSplit {
        source_path: PathBuf,
        source: GgufHeader,
        parts: Vec<GgufSplitPart>,
        part_paths: Vec<PathBuf>,
    }

    fn split_fixture_model(directory: &Path, maximum_part_tensor_bytes: u64) -> WrittenSplit {
        let source_path: PathBuf = test_model_files::write_fixture_model(directory);
        let source: GgufHeader = gguf_file::read_gguf_header_of_file(&source_path).unwrap();
        let parts: Vec<GgufSplitPart> = plan_split(&source, maximum_part_tensor_bytes).unwrap();
        let mut source_file: File = File::open(&source_path).unwrap();
        let part_count: u16 = u16::try_from(parts.len()).unwrap();
        let mut part_paths: Vec<PathBuf> = Vec::new();

        for part in &parts {
            let part_path: PathBuf = directory.join(repository_key::model_part_file_name("model", part.part_number, part_count));

            write_part(&mut source_file, &source, part, &part_path).unwrap();
            part_paths.push(part_path);
        }

        WrittenSplit {
            source_path,
            source,
            parts,
            part_paths,
        }
    }

    fn read_tensor_data(path: &Path, header: &GgufHeader, tensor_info: &GgufTensorInfo) -> Vec<u8> {
        let file_bytes: Vec<u8> = fs::read(path).unwrap();
        let start: usize = (header.data_offset + tensor_info.offset) as usize;

        file_bytes[start..start + tensor_info.size_bytes as usize].to_vec()
    }

    fn key_names(header: &GgufHeader) -> Vec<&str> {
        header.key_values.iter().map(|key_value| key_value.key.as_str()).collect()
    }

    fn tensor_names(header: &GgufHeader) -> Vec<&str> {
        header.tensor_infos.iter().map(|tensor_info| tensor_info.name.as_str()).collect()
    }

    #[test]
    fn write_part_writes_gguf_files_split_between_tensors_with_the_metadata_in_the_first() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();

        let written_split: WrittenSplit = split_fixture_model(directory.path(), 64);
        let part_headers: Vec<GgufHeader> = written_split
            .part_paths
            .iter()
            .map(|part_path| gguf_file::read_gguf_header(&mut GgufReader::new(File::open(part_path).unwrap())).unwrap())
            .collect();
        let tensor_names_by_part: Vec<Vec<&str>> = part_headers.iter().map(tensor_names).collect();

        assert_eq!(
            tensor_names_by_part,
            vec![vec!["token_embd.weight", "blk.0.attn_q.weight"], vec!["blk.0.ffn_up.weight"], vec!["output_norm.weight"]],
        );
        assert_eq!(
            key_names(&part_headers[0]),
            vec!["general.architecture", "tokenizer.ggml.tokens", SPLIT_NUMBER_KEY, SPLIT_TENSOR_COUNT_KEY, SPLIT_COUNT_KEY],
        );
        assert_eq!(part_headers[0].key_values[1], written_split.source.key_values[1]);
        assert_eq!(
            part_headers[2].key_values,
            vec![
                gguf_file::encode_key_value_u16(SPLIT_NUMBER_KEY, 2),
                gguf_file::encode_key_value_i32(SPLIT_TENSOR_COUNT_KEY, 4),
                gguf_file::encode_key_value_u16(SPLIT_COUNT_KEY, 3),
            ],
        );
        assert_eq!(part_headers[2].tensor_infos[0].dimensions, vec![4]);

        for ((part, part_path), part_header) in written_split.parts.iter().zip(&written_split.part_paths).zip(&part_headers) {
            assert_eq!(fs::metadata(part_path).unwrap().len(), part.size_bytes);

            for (part_tensor_info, &tensor_index) in part_header.tensor_infos.iter().zip(&part.tensor_indices) {
                let source_tensor_info: &GgufTensorInfo = &written_split.source.tensor_infos[tensor_index];

                assert_eq!(
                    read_tensor_data(part_path, part_header, part_tensor_info),
                    read_tensor_data(&written_split.source_path, &written_split.source, source_tensor_info),
                );
            }
        }
    }

    #[test]
    fn plan_split_gives_a_first_tensor_larger_than_the_limit_a_part_of_its_own() {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let source_path: PathBuf = test_model_files::write_fixture_model(directory.path());
        let source: GgufHeader = gguf_file::read_gguf_header_of_file(&source_path).unwrap();

        let parts: Vec<GgufSplitPart> = plan_split(&source, 16).unwrap();
        let tensor_indices_by_part: Vec<Vec<usize>> = parts.iter().map(|part| part.tensor_indices.clone()).collect();

        assert_eq!(tensor_indices_by_part, vec![vec![0], vec![1], vec![2], vec![3]]);
    }
}
