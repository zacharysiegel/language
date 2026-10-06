use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::thread;
use std::time::{Duration, Instant};

use shared::AppError;

use crate::audio_encoding::{self, AUDIO_FILE_EXTENSION, PARTIAL_FILE_EXTENSION};
use crate::audio_plan::{AudioPlan, Utterance};
use crate::kokoro::kokoro_synthesis;
use crate::speech_engine::{ResolvedUtterance, SpeechEngineKind, spoken_text_for};
use crate::work_pool;
use crate::workspace::WorkspacePaths;

const SAY_WORKER_COUNT: usize = 6;
const MINIMUM_VALID_FILE_BYTES: u64 = 500;
const SAY_TIMEOUT: Duration = Duration::from_secs(120);
const SAY_POLL_INTERVAL: Duration = Duration::from_millis(50);
const REVISION_FILE_NAME: &str = ".pipeline_revision";

pub fn synthesize_missing_audio(plan: &AudioPlan, paths: &WorkspacePaths, engine: SpeechEngineKind) -> Result<(), AppError> {
    let engine_directory: PathBuf = paths.engine_audio_directory(engine);

    prepare_engine_directory(&engine_directory, engine)?;

    let missing_clips: Vec<(String, ResolvedUtterance)> = plan
        .utterances
        .iter()
        .filter(|(file_name, _)| !is_valid_audio_file(&engine_directory.join(file_name)))
        .map(|(file_name, utterance)| (file_name.clone(), resolve(engine, utterance)))
        .collect();

    log::info!(
        "audio plan; [engine={} distinct_clips={} to_synthesize={}]",
        engine.code(),
        plan.utterances.len(),
        missing_clips.len(),
    );

    match engine {
        SpeechEngineKind::Kokoro => kokoro_synthesis::synthesize_clips(&missing_clips, &engine_directory, &paths.speech_models_directory),
        SpeechEngineKind::MacosSay => synthesize_with_macos_say(&missing_clips, &engine_directory),
    }
}

fn resolve(engine: SpeechEngineKind, utterance: &Utterance) -> ResolvedUtterance {
    ResolvedUtterance {
        profile: engine.profile_for(utterance.persona),
        text: utterance.text.clone(),
    }
}

/* Clips made by an older version of the pipeline are discarded, because they carry the same file names. */
fn prepare_engine_directory(engine_directory: &Path, engine: SpeechEngineKind) -> Result<(), AppError> {
    let revision_path: PathBuf = engine_directory.join(REVISION_FILE_NAME);
    let expected_revision: String = engine.pipeline_revision().to_string();
    let stored_revision: Option<String> = fs::read_to_string(&revision_path).ok();

    if engine_directory.exists() && stored_revision.as_deref() != Some(expected_revision.as_str()) {
        log::info!("discarding clips from another pipeline revision; [engine={}]", engine.code());
        fs::remove_dir_all(engine_directory)?;
    }

    fs::create_dir_all(engine_directory)?;
    fs::write(&revision_path, &expected_revision)?;
    remove_partial_files(engine_directory)
}

fn is_valid_audio_file(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|metadata| metadata.len() >= MINIMUM_VALID_FILE_BYTES)
}

fn synthesize_with_macos_say(clips: &[(String, ResolvedUtterance)], engine_directory: &Path) -> Result<(), AppError> {
    work_pool::run_work_pool(
        clips,
        SAY_WORKER_COUNT,
        |(_, utterance)| format!("{} [{}]", utterance.text, utterance.profile.voice_id),
        || Ok(()),
        |(), (file_name, utterance)| say_utterance(&engine_directory.join(file_name), utterance),
    )
}

fn say_utterance(destination: &Path, utterance: &ResolvedUtterance) -> Result<(), AppError> {
    let temporary_path: PathBuf = std::env::temp_dir().join(format!(
        "portuguese-course-{}-{}.aiff",
        std::process::id(),
        destination.file_stem().and_then(|stem| stem.to_str()).unwrap_or("clip"),
    ));

    let speech_result: Result<(), AppError> = run_say(&temporary_path, utterance)
        .and_then(|()| audio_encoding::encode_file(&temporary_path, destination));

    // The temporary file may not exist when speech synthesis failed first.
    let _ = fs::remove_file(&temporary_path);

    speech_result
}

fn run_say(output_path: &Path, utterance: &ResolvedUtterance) -> Result<(), AppError> {
    let mut child: Child = Command::new("say")
        .args(["-v", utterance.profile.voice_id, "-o"])
        .arg(output_path)
        .arg("--")
        .arg(spoken_text_for(&utterance.text))
        .spawn()?;
    let started: Instant = Instant::now();

    let status: ExitStatus = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }

        if started.elapsed() > SAY_TIMEOUT {
            child.kill()?;
            child.wait()?;

            return Err(AppError::from(format!("say did not finish; [timeout_seconds={}]", SAY_TIMEOUT.as_secs())));
        }

        thread::sleep(SAY_POLL_INTERVAL);
    };

    if !status.success() {
        return Err(AppError::from(format!("say exited unsuccessfully; [status={status}]")));
    }

    Ok(())
}

/// Removes, in every engine's directory and in the audio root, the clips the plan no longer names.
pub fn remove_unreferenced_audio(plan: &AudioPlan, audio_directory: &Path) -> Result<usize, AppError> {
    let mut removed_count: usize = 0;

    for directory_entry in fs::read_dir(audio_directory)? {
        let path: PathBuf = directory_entry?.path();

        if path.is_dir() {
            removed_count += remove_unreferenced_files(plan, &path)?;
        } else if is_audio_file(&path) {
            fs::remove_file(&path)?;
            removed_count += 1;
        }
    }

    Ok(removed_count)
}

fn remove_unreferenced_files(plan: &AudioPlan, directory: &Path) -> Result<usize, AppError> {
    let mut removed_count: usize = 0;

    for directory_entry in fs::read_dir(directory)? {
        let path: PathBuf = directory_entry?.path();
        let file_name: String = path.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string();

        if is_audio_file(&path) && !plan.utterances.contains_key(&file_name) {
            fs::remove_file(&path)?;
            removed_count += 1;
        }
    }

    Ok(removed_count)
}

fn is_audio_file(path: &Path) -> bool {
    path.extension().and_then(|extension| extension.to_str()) == Some(AUDIO_FILE_EXTENSION)
}

fn remove_partial_files(directory: &Path) -> Result<(), AppError> {
    for directory_entry in fs::read_dir(directory)? {
        let path: PathBuf = directory_entry?.path();
        let is_partial_file: bool = path.to_string_lossy().ends_with(PARTIAL_FILE_EXTENSION);

        if is_partial_file {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}
