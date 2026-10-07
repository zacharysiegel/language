use shared::AppError;
use shared::course::{Course, Curriculum, CurriculumUnit, Unit};

use crate::artifact::{self, BuiltBundle, ClipRecordings};
use crate::audio_plan::{self, AudioPlan};
use crate::audio_synthesis;
use crate::course_source;
use crate::speech_engine::SpeechEngineKind;
use crate::validate_command::{self, UnitValidationReport};
use crate::workspace::WorkspacePaths;

pub struct BuildOptions {
    pub engines: Vec<SpeechEngineKind>,
    pub skip_audio: bool,
    pub allow_missing: bool,
}

pub fn run_build(paths: &WorkspacePaths, options: &BuildOptions) -> Result<bool, AppError> {
    let curriculum: Curriculum = course_source::read_curriculum(paths)?;
    let all_units: Vec<&CurriculumUnit> = curriculum.units.iter().collect();
    let report: UnitValidationReport = validate_command::validate_curriculum_units(paths, &all_units, !options.allow_missing)?;

    validate_command::print_issues(&report);

    if report.error_count() > 0 {
        eprintln!("build stopped; [errors={}]", report.error_count());
        return Ok(false);
    }

    let mut plan: AudioPlan = AudioPlan::new();
    let keyed_units: Vec<Unit> = report
        .loaded_units
        .iter()
        .map(|unit| audio_plan::assign_clip_keys(unit, &mut plan))
        .collect();
    let course: Course = Course {
        title: curriculum.title.clone(),
        levels: curriculum.levels.clone(),
        units: keyed_units,
    };

    if options.skip_audio {
        log::info!("audio synthesis skipped; [distinct_clips={}]", plan.utterances.len());
    } else {
        synthesize_audio(paths, &plan, &options.engines)?;
    }

    let recordings: ClipRecordings = audio_synthesis::collect_clip_recordings(&plan, paths)?;
    let bundle: BuiltBundle = artifact::write_bundle(paths, &course, &plan, &recordings)?;

    artifact::refresh_embedded_bundle(paths, &bundle)?;
    artifact::prune_bundles(paths)?;

    log::info!(
        "build complete; [version={} units={} distinct_clips={}]",
        bundle.manifest.version,
        course.units.len(),
        plan.utterances.len(),
    );

    Ok(true)
}

fn synthesize_audio(paths: &WorkspacePaths, plan: &AudioPlan, engines: &[SpeechEngineKind]) -> Result<(), AppError> {
    audio_synthesis::synthesize_missing_audio(plan, paths, engines)?;

    let removed_count: usize = audio_synthesis::remove_unreferenced_clips(plan, paths)?;

    if removed_count > 0 {
        log::info!("removed clips no unit uses; [count={removed_count}]");
    }

    Ok(())
}
