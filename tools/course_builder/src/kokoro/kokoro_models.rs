use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use sha2::{Digest, Sha256};
use shared::AppError;

const KOKORO_VOICES_DIRECTORY: &str = "voices";
const ONNX_RUNTIME_DIRECTORY: &str = "onnxruntime";
const ONNX_RUNTIME_LIBRARY_PATH: &str = "onnxruntime/onnxruntime-osx-arm64-1.30.0/lib/libonnxruntime.dylib";
const ONNX_RUNTIME_ARCHIVE_NAME: &str = "onnxruntime.tgz";

struct ModelFile {
    relative_path: &'static str,
    url: &'static str,
    sha256: &'static str,
}

/* Pinned to exact revisions so a later upstream change cannot alter the voices. */
const MODEL_FILES: [ModelFile; 5] = [
    ModelFile {
        relative_path: "config.json",
        url: "https://huggingface.co/hexgrad/Kokoro-82M/resolve/f3ff3571791e39611d31c381e3a41a3af07b4987/config.json",
        sha256: "5abb01e2403b072bf03d04fde160443e209d7a0dad49a423be15196b9b43c17f",
    },
    ModelFile {
        relative_path: "model.onnx",
        url: "https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/resolve/1939ad2a8e416c0acfeecc08a694d14ef25f2231/onnx/model.onnx",
        sha256: "8fbea51ea711f2af382e88c833d9e288c6dc82ce5e98421ea61c058ce21a34cb",
    },
    ModelFile {
        relative_path: "voices/pf_dora.bin",
        url: "https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/resolve/1939ad2a8e416c0acfeecc08a694d14ef25f2231/voices/pf_dora.bin",
        sha256: "3da7b5b2d91847ebf5646f57631af6ececae3c29a89cd300f06edf9aa6cfe9ee",
    },
    ModelFile {
        relative_path: "voices/pm_alex.bin",
        url: "https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/resolve/1939ad2a8e416c0acfeecc08a694d14ef25f2231/voices/pm_alex.bin",
        sha256: "0175c753f59c54e7fd5a995bedef0c5ff2fb67e0043dd3dcb2ae74ec2acbeb2a",
    },
    ModelFile {
        relative_path: "voices/pm_santa.bin",
        url: "https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/resolve/1939ad2a8e416c0acfeecc08a694d14ef25f2231/voices/pm_santa.bin",
        sha256: "8b012db3185778afe2e45a62cbad69db73021774fe68dda634bcc748a982eede",
    },
];

const ONNX_RUNTIME_ARCHIVE: ModelFile = ModelFile {
    relative_path: ONNX_RUNTIME_ARCHIVE_NAME,
    url: "https://github.com/microsoft/onnxruntime/releases/download/v1.30.0/onnxruntime-osx-arm64-1.30.0.tgz",
    sha256: "6ebb5062a934537c352937821f9fe9718e7de1a2db1122a93dd363ffd53a7012",
};

#[derive(Debug, Clone)]
pub struct KokoroFiles {
    pub onnx_runtime_library: PathBuf,
    pub model: PathBuf,
    pub config: PathBuf,
    voices_directory: PathBuf,
}

impl KokoroFiles {
    pub fn voice_path(&self, voice_id: &str) -> PathBuf {
        self.voices_directory.join(format!("{voice_id}.bin"))
    }
}

/// Downloads any missing model file and the ONNX Runtime library, and checks every download against its pinned checksum.
pub fn ensure_kokoro_files(models_directory: &Path) -> Result<KokoroFiles, AppError> {
    fs::create_dir_all(models_directory.join(KOKORO_VOICES_DIRECTORY))?;

    for model_file in &MODEL_FILES {
        ensure_file(models_directory, model_file)?;
    }

    let library_path: PathBuf = models_directory.join(ONNX_RUNTIME_LIBRARY_PATH);

    if !library_path.exists() {
        ensure_file(models_directory, &ONNX_RUNTIME_ARCHIVE)?;
        extract_onnx_runtime(models_directory)?;
    }

    Ok(KokoroFiles {
        onnx_runtime_library: library_path,
        model: models_directory.join("model.onnx"),
        config: models_directory.join("config.json"),
        voices_directory: models_directory.join(KOKORO_VOICES_DIRECTORY),
    })
}

fn ensure_file(models_directory: &Path, model_file: &ModelFile) -> Result<(), AppError> {
    let destination: PathBuf = models_directory.join(model_file.relative_path);

    if destination.exists() && checksum_matches(&destination, model_file.sha256)? {
        return Ok(());
    }

    log::info!("downloading speech model file; [path={} url={}]", model_file.relative_path, model_file.url);

    let partial_path: PathBuf = destination.with_extension("partial");
    let status: ExitStatus = Command::new("curl")
        .args(["--fail", "--location", "--silent", "--show-error", "--output"])
        .arg(&partial_path)
        .arg(model_file.url)
        .status()?;

    if !status.success() {
        return Err(AppError::from(format!("download failed; [url={} status={status}]", model_file.url)));
    }

    if !checksum_matches(&partial_path, model_file.sha256)? {
        fs::remove_file(&partial_path)?;

        return Err(AppError::from(format!("checksum mismatch after download; [path={}]", model_file.relative_path)));
    }

    fs::rename(&partial_path, &destination)?;

    Ok(())
}

fn checksum_matches(path: &Path, expected_sha256: &str) -> Result<bool, AppError> {
    let bytes: Vec<u8> = fs::read(path)?;
    let digest: [u8; 32] = Sha256::digest(&bytes).into();
    let computed: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();

    Ok(computed == expected_sha256)
}

fn extract_onnx_runtime(models_directory: &Path) -> Result<(), AppError> {
    let extraction_directory: PathBuf = models_directory.join(ONNX_RUNTIME_DIRECTORY);

    fs::create_dir_all(&extraction_directory)?;

    let status: ExitStatus = Command::new("tar")
        .arg("-xzf")
        .arg(models_directory.join(ONNX_RUNTIME_ARCHIVE_NAME))
        .arg("-C")
        .arg(&extraction_directory)
        .status()?;

    if !status.success() {
        return Err(AppError::from(format!("could not extract the ONNX Runtime archive; [status={status}]")));
    }

    Ok(())
}
