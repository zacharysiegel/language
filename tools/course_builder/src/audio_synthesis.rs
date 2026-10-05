use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use shared::AppError;

use crate::audio_plan::{AudioPlan, Utterance};

const SYNTHESIS_WORKER_COUNT: usize = 6;
const MINIMUM_VALID_FILE_BYTES: u64 = 500;
const PROGRESS_LOG_INTERVAL: usize = 200;
const AUDIO_FILE_EXTENSION: &str = "m4a";

pub fn synthesize_missing_audio(plan: &AudioPlan, audio_directory: &Path) -> Result<(), AppError> {
    fs::create_dir_all(audio_directory)?;

    let missing_entries: Vec<(&String, &Utterance)> = plan
        .utterances
        .iter()
        .filter(|(file_name, _)| !is_valid_audio_file(&audio_directory.join(file_name)))
        .collect();

    log::info!(
        "audio plan; [distinct_clips={} to_synthesize={}]",
        plan.utterances.len(),
        missing_entries.len(),
    );

    let next_entry_index: AtomicUsize = AtomicUsize::new(0);
    let completed_count: AtomicUsize = AtomicUsize::new(0);
    let failure_messages: Mutex<Vec<String>> = Mutex::new(Vec::new());

    thread::scope(|scope| {
        for _ in 0..SYNTHESIS_WORKER_COUNT {
            scope.spawn(|| {
                synthesis_worker(&missing_entries, audio_directory, &next_entry_index, &completed_count, &failure_messages);
            });
        }
    });

    let failures: Vec<String> = failure_messages.into_inner().unwrap_or_default();

    if !failures.is_empty() {
        for failure in failures.iter().take(10) {
            log::error!("audio synthesis failed; [detail={failure}]");
        }

        return Err(AppError::from(format!("audio synthesis failed; [failed_clips={}]", failures.len())));
    }

    Ok(())
}

fn synthesis_worker(
    entries: &[(&String, &Utterance)],
    audio_directory: &Path,
    next_entry_index: &AtomicUsize,
    completed_count: &AtomicUsize,
    failure_messages: &Mutex<Vec<String>>,
) {
    loop {
        let entry_index: usize = next_entry_index.fetch_add(1, Ordering::SeqCst);

        let Some((file_name, utterance)) = entries.get(entry_index) else {
            return;
        };

        let result: Result<(), AppError> = synthesize_utterance(&audio_directory.join(file_name), utterance);

        if let Err(error) = result {
            if let Ok(mut failures) = failure_messages.lock() {
                failures.push(format!("{} [{}]: {error}", utterance.text, utterance.voice));
            }
        }

        let completed: usize = completed_count.fetch_add(1, Ordering::SeqCst) + 1;

        if completed % PROGRESS_LOG_INTERVAL == 0 {
            log::info!("audio synthesis progress; [completed={completed} total={}]", entries.len());
        }
    }
}

fn is_valid_audio_file(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|metadata| metadata.len() >= MINIMUM_VALID_FILE_BYTES)
}

fn synthesize_utterance(destination: &Path, utterance: &Utterance) -> Result<(), AppError> {
    let temporary_path: PathBuf = std::env::temp_dir().join(format!(
        "portuguese-course-{}-{}.aiff",
        std::process::id(),
        destination.file_stem().and_then(|stem| stem.to_str()).unwrap_or("clip"),
    ));

    let speech_result: Result<(), AppError> = run_say(&temporary_path, utterance)
        .and_then(|()| run_ffmpeg(&temporary_path, destination));

    // The temporary file may not exist when speech synthesis failed first.
    let _ = fs::remove_file(&temporary_path);

    speech_result
}

fn run_say(output_path: &Path, utterance: &Utterance) -> Result<(), AppError> {
    let status: ExitStatus = Command::new("say")
        .args(["-v", utterance.voice, "-o"])
        .arg(output_path)
        .arg("--")
        .arg(&utterance.text)
        .status()?;

    require_success("say", status)
}

fn run_ffmpeg(input_path: &Path, output_path: &Path) -> Result<(), AppError> {
    let status: ExitStatus = Command::new("ffmpeg")
        .args(["-v", "error", "-y", "-i"])
        .arg(input_path)
        .args(["-ac", "1", "-ar", "24000", "-c:a", "aac", "-b:a", "40k", "-movflags", "+faststart"])
        .arg(output_path)
        .status()?;

    require_success("ffmpeg", status)
}

fn require_success(program: &str, status: ExitStatus) -> Result<(), AppError> {
    if status.success() {
        return Ok(());
    }

    Err(AppError::from(format!("{program} exited unsuccessfully; [status={status}]")))
}

pub fn remove_unreferenced_audio(plan: &AudioPlan, audio_directory: &Path) -> Result<usize, AppError> {
    let mut removed_count: usize = 0;

    for directory_entry in fs::read_dir(audio_directory)? {
        let path: PathBuf = directory_entry?.path();
        let is_audio_file: bool = path.extension().and_then(|extension| extension.to_str()) == Some(AUDIO_FILE_EXTENSION);
        let file_name: String = path.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string();

        if is_audio_file && !plan.utterances.contains_key(&file_name) {
            fs::remove_file(&path)?;
            removed_count += 1;
        }
    }

    Ok(removed_count)
}
