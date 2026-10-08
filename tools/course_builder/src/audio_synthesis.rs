use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::thread;
use std::time::{Duration, Instant};

use shared::AppError;
use shared::artifact::content_address;
use shared::course_shard::ClipAudioEntry;

use crate::artifact::ClipRecordings;
use crate::audio_encoding::{self, AUDIO_FILE_EXTENSION, PARTIAL_FILE_EXTENSION};
use crate::audio_plan::{AudioPlan, Utterance};
use crate::kokoro::kokoro_synthesis;
use crate::speech_engine::{self, ResolvedUtterance, SpeechEngineKind, SpeechLanguage};
use crate::work_pool;
use crate::workspace::WorkspacePaths;

const SAY_WORKER_COUNT: usize = 6;
const MINIMUM_VALID_FILE_BYTES: u64 = 500;
const SAY_TIMEOUT: Duration = Duration::from_secs(120);
const SAY_POLL_INTERVAL: Duration = Duration::from_millis(50);
const REVISION_FILE_NAME: &str = ".pipeline_revision";

/// Synthesizes each engine's missing clips, with the engines running at the same time.
pub fn synthesize_missing_audio(plan: &AudioPlan, paths: &WorkspacePaths, engines: &[SpeechEngineKind]) -> Result<(), AppError> {
    // AppError is not Send, so each engine's thread reports its failure as text.
    let engine_results: Vec<Result<(), String>> = thread::scope(|scope| {
        let engine_threads: Vec<thread::ScopedJoinHandle<'_, Result<(), String>>> = engines
            .iter()
            .map(|engine| scope.spawn(move || synthesize_missing_engine_audio(plan, paths, *engine).map_err(|error| error.to_string())))
            .collect();

        engine_threads
            .into_iter()
            .map(|engine_thread| engine_thread.join().unwrap_or_else(|_| Err("an engine's synthesis thread panicked".to_string())))
            .collect()
    });

    let failure_messages: Vec<String> = engine_results.into_iter().filter_map(Result::err).collect();

    if !failure_messages.is_empty() {
        return Err(AppError::from(failure_messages.join("; ")));
    }

    Ok(())
}

fn synthesize_missing_engine_audio(plan: &AudioPlan, paths: &WorkspacePaths, engine: SpeechEngineKind) -> Result<(), AppError> {
    let engine_directory: PathBuf = paths.engine_clip_directory(engine);

    prepare_engine_directory(&engine_directory, engine)?;

    let missing_clips: Vec<(String, ResolvedUtterance)> = plan
        .utterances
        .iter()
        .filter(|(clip_key, _)| !is_valid_audio_file(&clip_file_path(&engine_directory, clip_key)))
        .map(|(clip_key, utterance)| resolve(engine, utterance).map(|resolved| (clip_file_name(clip_key), resolved)))
        .collect::<Result<Vec<(String, ResolvedUtterance)>, AppError>>()?;

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

fn clip_file_name(clip_key: &str) -> String {
    format!("{clip_key}.{AUDIO_FILE_EXTENSION}")
}

fn clip_file_path(engine_directory: &Path, clip_key: &str) -> PathBuf {
    engine_directory.join(clip_file_name(clip_key))
}

fn resolve(engine: SpeechEngineKind, utterance: &Utterance) -> Result<ResolvedUtterance, AppError> {
    let language: &'static SpeechLanguage = speech_engine::speech_language(&utterance.language)?;

    Ok(ResolvedUtterance {
        language,
        profile: engine.profile_for(language, utterance.persona),
        text: utterance.text.clone(),
    })
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
        "macos_say synthesis",
        clips,
        SAY_WORKER_COUNT,
        |(_, utterance)| format!("{} [{}]", utterance.text, utterance.profile.voice_id),
        || Ok(()),
        |(), (file_name, utterance)| say_utterance(&engine_directory.join(file_name), utterance),
    )
}

fn say_utterance(destination: &Path, utterance: &ResolvedUtterance) -> Result<(), AppError> {
    let temporary_path: PathBuf = std::env::temp_dir().join(format!(
        "language-{}-{}.aiff",
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
        .arg(speech_engine::spoken_text_for(utterance.language, &utterance.text))
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

/// The recordings in the clip store of every engine whose clips come from its current pipeline, whether or not
/// this build synthesized with it.
pub fn collect_clip_recordings(plan: &AudioPlan, paths: &WorkspacePaths) -> Result<ClipRecordings, AppError> {
    let mut recordings: ClipRecordings = ClipRecordings {
        audio_index: BTreeMap::new(),
        file_by_audio_sha256: BTreeMap::new(),
        pipeline_revision_by_engine_code: BTreeMap::new(),
    };

    for engine in SpeechEngineKind::ALL {
        let engine_directory: PathBuf = paths.engine_clip_directory(engine);

        if !holds_current_pipeline_clips(&engine_directory, engine) {
            log::info!("no current clips for an engine; [engine={}]", engine.code());
            continue;
        }

        let recorded_count: usize = collect_engine_recordings(plan, engine, &engine_directory, &mut recordings)?;

        recordings.pipeline_revision_by_engine_code.insert(engine.code().to_string(), engine.pipeline_revision());
        log::info!("collected recordings; [engine={} clips={recorded_count} of={}]", engine.code(), plan.utterances.len());
    }

    Ok(recordings)
}

fn holds_current_pipeline_clips(engine_directory: &Path, engine: SpeechEngineKind) -> bool {
    let stored_revision: Option<String> = fs::read_to_string(engine_directory.join(REVISION_FILE_NAME)).ok();

    stored_revision.as_deref() == Some(engine.pipeline_revision().to_string().as_str())
}

fn collect_engine_recordings(
    plan: &AudioPlan,
    engine: SpeechEngineKind,
    engine_directory: &Path,
    recordings: &mut ClipRecordings,
) -> Result<usize, AppError> {
    let mut recorded_count: usize = 0;

    for clip_key in plan.utterances.keys() {
        let clip_path: PathBuf = clip_file_path(engine_directory, clip_key);

        if !is_valid_audio_file(&clip_path) {
            continue;
        }

        let bytes: Vec<u8> = fs::read(&clip_path)?;
        let audio_sha256: String = content_address::sha256_hex(&bytes);

        recordings.audio_index.entry(clip_key.clone()).or_default().push(ClipAudioEntry {
            clip_key: clip_key.clone(),
            engine: engine.voice_engine(),
            audio_sha256: audio_sha256.clone(),
            audio_size_bytes: bytes.len() as u64,
        });
        recordings.file_by_audio_sha256.insert(audio_sha256, clip_path);
        recorded_count += 1;
    }

    Ok(recorded_count)
}

/// Removes, in every engine's clip store, the clips the plan no longer names.
pub fn remove_unreferenced_clips(plan: &AudioPlan, paths: &WorkspacePaths) -> Result<usize, AppError> {
    let mut removed_count: usize = 0;

    for engine in SpeechEngineKind::ALL {
        let engine_directory: PathBuf = paths.engine_clip_directory(engine);

        if engine_directory.exists() {
            removed_count += remove_unreferenced_files(plan, &engine_directory)?;
        }
    }

    Ok(removed_count)
}

fn remove_unreferenced_files(plan: &AudioPlan, directory: &Path) -> Result<usize, AppError> {
    let mut removed_count: usize = 0;

    for directory_entry in fs::read_dir(directory)? {
        let path: PathBuf = directory_entry?.path();
        let clip_key: String = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or_default().to_string();

        if is_audio_file(&path) && !plan.utterances.contains_key(&clip_key) {
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
