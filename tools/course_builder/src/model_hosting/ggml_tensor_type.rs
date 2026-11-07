use shared::AppError;

const SUPER_BLOCK: u64 = 256;

/* The ids ggml removed are absent. */
const TENSOR_TYPE_LAYOUTS: [TensorTypeLayout; 36] = [
    layout(0, "F32", 1, 4),
    layout(1, "F16", 1, 2),
    layout(2, "Q4_0", 32, 2 + 16),
    layout(3, "Q4_1", 32, 2 + 2 + 16),
    layout(6, "Q5_0", 32, 2 + 4 + 16),
    layout(7, "Q5_1", 32, 2 + 2 + 4 + 16),
    layout(8, "Q8_0", 32, 2 + 32),
    layout(9, "Q8_1", 32, 2 + 2 + 32),
    layout(10, "Q2_K", SUPER_BLOCK, 2 + 2 + SUPER_BLOCK / 16 + SUPER_BLOCK / 4),
    layout(11, "Q3_K", SUPER_BLOCK, 2 + SUPER_BLOCK / 4 + SUPER_BLOCK / 8 + 12),
    layout(12, "Q4_K", SUPER_BLOCK, 2 + 2 + SUPER_BLOCK / 2 + 12),
    layout(13, "Q5_K", SUPER_BLOCK, 2 + 2 + SUPER_BLOCK / 2 + SUPER_BLOCK / 8 + 12),
    layout(14, "Q6_K", SUPER_BLOCK, 2 + SUPER_BLOCK / 2 + SUPER_BLOCK / 4 + SUPER_BLOCK / 16),
    layout(15, "Q8_K", SUPER_BLOCK, 4 + SUPER_BLOCK + SUPER_BLOCK / 8),
    layout(16, "IQ2_XXS", SUPER_BLOCK, 2 + SUPER_BLOCK / 4),
    layout(17, "IQ2_XS", SUPER_BLOCK, 2 + SUPER_BLOCK / 4 + SUPER_BLOCK / 32),
    layout(18, "IQ3_XXS", SUPER_BLOCK, 2 + SUPER_BLOCK / 4 + SUPER_BLOCK / 8),
    layout(19, "IQ1_S", SUPER_BLOCK, 2 + SUPER_BLOCK / 8 + SUPER_BLOCK / 16),
    layout(20, "IQ4_NL", 32, 2 + 16),
    layout(21, "IQ3_S", SUPER_BLOCK, 2 + SUPER_BLOCK / 4 + SUPER_BLOCK / 8 + SUPER_BLOCK / 32 + 4),
    layout(22, "IQ2_S", SUPER_BLOCK, 2 + SUPER_BLOCK / 4 + SUPER_BLOCK / 16),
    layout(23, "IQ4_XS", SUPER_BLOCK, 2 + 2 + SUPER_BLOCK / 2 + SUPER_BLOCK / 64),
    layout(24, "I8", 1, 1),
    layout(25, "I16", 1, 2),
    layout(26, "I32", 1, 4),
    layout(27, "I64", 1, 8),
    layout(28, "F64", 1, 8),
    layout(29, "IQ1_M", SUPER_BLOCK, SUPER_BLOCK / 8 + SUPER_BLOCK / 16 + SUPER_BLOCK / 32),
    layout(30, "BF16", 1, 2),
    layout(34, "TQ1_0", SUPER_BLOCK, 2 + 4 * 13),
    layout(35, "TQ2_0", SUPER_BLOCK, 2 + 64),
    layout(39, "MXFP4", 32, 1 + 16),
    layout(40, "NVFP4", 64, 4 + 32),
    layout(41, "Q1_0", 128, 2 + 16),
    layout(42, "Q2_0", 64, 2 + 16),
    layout(43, "F8_E4M3", 1, 1),
];

/// A tensor type's block layout in ggml's `type_traits` table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TensorTypeLayout {
    /// ggml's `enum ggml_type` value.
    type_id: u32,
    name: &'static str,
    block_element_count: u64,
    block_size_bytes: u64,
}

const fn layout(type_id: u32, name: &'static str, block_element_count: u64, block_size_bytes: u64) -> TensorTypeLayout {
    TensorTypeLayout {
        type_id,
        name,
        block_element_count,
        block_size_bytes,
    }
}

/// `ggml_nbytes` of a contiguous tensor.
pub fn tensor_size_bytes(tensor_name: &str, type_id: u32, dimensions: &[i64]) -> Result<u64, AppError> {
    let found_layout: Option<&TensorTypeLayout> = TENSOR_TYPE_LAYOUTS.iter().find(|tensor_type_layout| tensor_type_layout.type_id == type_id);

    let Some(tensor_type_layout) = found_layout
    else {
        return Err(AppError::from(format!("a tensor has a ggml type this splitter does not know; [tensor={tensor_name} type_id={type_id}]")));
    };

    if dimensions.iter().any(|&dimension| dimension < 0) {
        return Err(AppError::from(format!("a tensor has a negative dimension; [tensor={tensor_name}]")));
    }

    if dimensions.contains(&0) {
        return Ok(0);
    }

    let row_element_count: u64 = dimensions[0] as u64;

    if row_element_count % tensor_type_layout.block_element_count != 0 {
        return Err(AppError::from(format!(
            "a tensor's row is not a whole number of blocks; [tensor={tensor_name} type={} row_elements={row_element_count}]",
            tensor_type_layout.name,
        )));
    }

    let mut size_bytes: u64 = row_element_count / tensor_type_layout.block_element_count * tensor_type_layout.block_size_bytes;

    for &dimension in &dimensions[1..] {
        size_bytes = size_bytes
            .checked_mul(dimension as u64)
            .ok_or_else(|| AppError::from(format!("a tensor's size overflows; [tensor={tensor_name}]")))?;
    }

    Ok(size_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tensor_size_bytes_counts_whole_blocks_of_a_quantized_row() {
        assert_eq!(tensor_size_bytes("token_embd.weight", 2, &[3840, 262_144]).unwrap(), 3840 / 32 * 18 * 262_144);
        assert_eq!(tensor_size_bytes("output_norm.weight", 0, &[3840]).unwrap(), 3840 * 4);
        assert!(tensor_size_bytes("ragged.weight", 2, &[33, 2]).is_err());
        assert!(tensor_size_bytes("removed.weight", 4, &[32]).is_err());
    }
}
