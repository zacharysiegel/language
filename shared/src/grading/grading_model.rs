use crate::artifact::repository_key;

/* The text GGUF of Ollama's gemma4:12b-it-qat, which Ollama names by this SHA-256. */
pub const GRADING_MODEL_NAME: &str = "gemma4-12b-it-qat";
pub const GRADING_MODEL_SHA256: &str = "faff1a63667fac17ac5e777f47114688fcefea96e220e211aaa8d62c2c4561f1";
pub const GRADING_MODEL_SIZE_BYTES: u64 = 6_975_877_728;
pub const GRADING_MODEL_PART_COUNT: u16 = 14;

/// Relative to the repository base.
pub fn grading_model_first_part_key() -> String {
    repository_key::model_part_key(GRADING_MODEL_NAME, GRADING_MODEL_SHA256, 1, GRADING_MODEL_PART_COUNT)
}
