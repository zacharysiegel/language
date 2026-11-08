use crate::artifact::repository_key;

/* Google's GGUF of Gemma 4 12B instruction-tuned, quantized to Q4_0 with quantization-aware training (Apache 2.0),
   at a pinned commit of its Hugging Face repository. */
pub const GRADING_MODEL_NAME: &str = "gemma-4-12b-it-qat-q4_0";
pub const GRADING_MODEL_FILE_NAME: &str = "gemma-4-12b-it-qat-q4_0.gguf";
pub const GRADING_MODEL_SHA256: &str = "93567e57a8fe10b23569b9d9ec38cd005deedf71e29477c421a4b83f418a538b";
pub const GRADING_MODEL_SIZE_BYTES: u64 = 6_975_879_296;
pub const GRADING_MODEL_HUGGING_FACE_REPOSITORY: &str = "google/gemma-4-12B-it-qat-q4_0-gguf";
pub const GRADING_MODEL_HUGGING_FACE_COMMIT: &str = "29d097773436b69ff9feafd636ab4cf873786537";
pub const GRADING_MODEL_HUGGING_FACE_URL: &str = "https://huggingface.co/google/gemma-4-12B-it-qat-q4_0-gguf/resolve/29d097773436b69ff9feafd636ab4cf873786537/gemma-4-12b-it-qat-q4_0.gguf";

/* In llama.cpp's split format, as R2 model hosting publishes the file. */
pub const GRADING_MODEL_PART_COUNT: u16 = 14;

/// The unsplit file's key, relative to the repository base.
pub fn grading_model_file_key() -> String {
    repository_key::model_file_key(GRADING_MODEL_NAME, GRADING_MODEL_SHA256, GRADING_MODEL_FILE_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grading_model_hugging_face_url_resolves_the_file_at_the_pinned_commit() {
        assert_eq!(
            GRADING_MODEL_HUGGING_FACE_URL,
            format!("https://huggingface.co/{GRADING_MODEL_HUGGING_FACE_REPOSITORY}/resolve/{GRADING_MODEL_HUGGING_FACE_COMMIT}/{GRADING_MODEL_FILE_NAME}"),
        );
    }
}
