use sha2::{Digest, Sha256};

use crate::AppError;

pub const SHA256_HEX_LENGTH: usize = 64;

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest: [u8; 32] = Sha256::digest(bytes).into();

    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn is_sha256_hex(text: &str) -> bool {
    text.len() == SHA256_HEX_LENGTH && text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/* The hash is the last component of the name, so a name alone says which bytes it must hold. */
pub fn content_addressed_file_name(stem: &str, sha256: &str, extension: &str) -> String {
    format!("{stem}-{sha256}.{extension}")
}

pub fn verify_sha256(label: &str, bytes: &[u8], expected_sha256: &str) -> Result<(), AppError> {
    let actual_sha256: String = sha256_hex(bytes);

    if actual_sha256 != expected_sha256 {
        return Err(AppError::from(format!(
            "content hash mismatch; [file={label} expected={expected_sha256} actual={actual_sha256}]",
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hex_matches_the_known_digest_of_the_empty_input() {
        assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn is_sha256_hex_accepts_only_lowercase_hex_of_the_right_length() {
        assert!(is_sha256_hex(&"ab".repeat(32)));
        assert!(!is_sha256_hex(&"AB".repeat(32)));
        assert!(!is_sha256_hex("abc"));
        assert!(!is_sha256_hex(&"zz".repeat(32)));
    }

    #[test]
    fn verify_sha256_rejects_other_bytes() {
        let expected: String = sha256_hex(b"one");

        verify_sha256("file", b"one", &expected).unwrap();
        assert!(verify_sha256("file", b"two", &expected).is_err());
    }

    #[test]
    fn content_addressed_file_name_ends_in_the_hash() {
        assert_eq!(content_addressed_file_name("unit-a1", "abc", "sqlite"), "unit-a1-abc.sqlite");
    }
}
