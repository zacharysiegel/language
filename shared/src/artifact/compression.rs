//! Neither R2 nor a plain static file server compresses a generic binary type, so the producer encodes the
//! shards and every platform decodes them here.

use std::io::Read;

use brotli::enc::BrotliEncoderParams;

use crate::AppError;

pub const COMPRESSED_FILENAME_EXTENSION: &str = "br";

const BUFFER_BYTES: usize = 64 * 1024;

pub fn compress(plain_bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let parameters: BrotliEncoderParams = encoder_parameters();
    let mut compressed_bytes: Vec<u8> = Vec::new();

    brotli::BrotliCompress(&mut &plain_bytes[..], &mut compressed_bytes, &parameters)
        .map_err(|error| AppError::from(format!("brotli encode failed; [bytes={} error={error}]", plain_bytes.len())))?;

    Ok(compressed_bytes)
}

/// The highest quality and the largest window; only the producer encodes.
fn encoder_parameters() -> BrotliEncoderParams {
    let mut parameters: BrotliEncoderParams = BrotliEncoderParams::default();
    parameters.quality = 11;
    parameters.lgwin = 22;

    parameters
}

pub fn decompress(compressed_bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut decompressed_bytes: Vec<u8> = Vec::new();

    brotli::Decompressor::new(compressed_bytes, BUFFER_BYTES)
        .read_to_end(&mut decompressed_bytes)
        .map_err(|error| AppError::from(format!("the bytes are not a brotli stream; [bytes={} error={error}]", compressed_bytes.len())))?;

    Ok(decompressed_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_inverts_compress() {
        let plain_bytes: Vec<u8> = b"SQLite format 3\0".repeat(500);

        let compressed_bytes: Vec<u8> = compress(&plain_bytes).unwrap();

        assert!(compressed_bytes.len() < plain_bytes.len());
        assert_eq!(decompress(&compressed_bytes).unwrap(), plain_bytes);
    }

    #[test]
    fn decompress_rejects_bytes_that_are_not_brotli() {
        assert!(decompress(b"SQLite format 3\0 plain, not compressed").is_err());
    }
}
