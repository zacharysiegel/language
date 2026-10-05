use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};

use shared::AppError;

pub const AUDIO_FILE_EXTENSION: &str = "m4a";
pub const PARTIAL_FILE_EXTENSION: &str = "m4a.part";

const OUTPUT_SAMPLE_RATE: &str = "24000";
const OUTPUT_BIT_RATE: &str = "48k";

fn output_arguments(command: &mut Command, partial_path: &Path) {
    command
        .args(["-ac", "1", "-ar", OUTPUT_SAMPLE_RATE, "-c:a", "aac", "-b:a", OUTPUT_BIT_RATE])
        .args(["-movflags", "+faststart", "-f", "mp4"])
        .arg(partial_path);
}

/// Encodes an audio file into `destination`, which only appears once the encode has finished.
pub fn encode_file(input_path: &Path, destination: &Path) -> Result<(), AppError> {
    let partial_path: PathBuf = destination.with_extension(PARTIAL_FILE_EXTENSION);
    let mut command: Command = Command::new("ffmpeg");

    command.args(["-v", "error", "-y", "-i"]).arg(input_path);
    output_arguments(&mut command, &partial_path);

    finish_encode(command.status()?, &partial_path, destination)
}

/// Encodes mono 32-bit float samples into `destination`, which only appears once the encode has finished.
pub fn encode_samples(samples: &[f32], sample_rate: u32, destination: &Path) -> Result<(), AppError> {
    let partial_path: PathBuf = destination.with_extension(PARTIAL_FILE_EXTENSION);
    let mut command: Command = Command::new("ffmpeg");

    command
        .args(["-v", "error", "-y", "-f", "f32le", "-ar"])
        .arg(sample_rate.to_string())
        .args(["-ac", "1", "-i", "pipe:0"]);
    output_arguments(&mut command, &partial_path);

    let mut child: Child = command.stdin(Stdio::piped()).spawn()?;
    let sample_bytes: Vec<u8> = samples.iter().flat_map(|sample| sample.to_le_bytes()).collect();

    if let Some(mut standard_input) = child.stdin.take() {
        standard_input.write_all(&sample_bytes)?;
    }

    finish_encode(child.wait()?, &partial_path, destination)
}

fn finish_encode(status: ExitStatus, partial_path: &Path, destination: &Path) -> Result<(), AppError> {
    if !status.success() {
        // The partial file may not exist when ffmpeg failed before writing.
        let _ = fs::remove_file(partial_path);

        return Err(AppError::from(format!("ffmpeg exited unsuccessfully; [status={status}]")));
    }

    fs::rename(partial_path, destination)?;

    Ok(())
}
