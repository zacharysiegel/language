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
    pub course_id: Option<String>,
    pub engines: Vec<SpeechEngineKind>,
    pub skip_audio: bool,
    pub allow_missing: bool,
}

pub fn run_build(paths: &WorkspacePaths, options: &BuildOptions) -> Result<bool, AppError> {
    let mut plan: AudioPlan = AudioPlan::new();
    let mut courses: Vec<Course> = Vec::new();

    for course_id in validate_command::select_course_ids(paths, options.course_id.as_deref())? {
        let curriculum: Curriculum = course_source::read_curriculum(paths, &course_id)?;
        let all_units: Vec<&CurriculumUnit> = curriculum.units.iter().collect();
        let report: UnitValidationReport = validate_command::validate_curriculum_units(paths, &curriculum, &all_units, !options.allow_missing)?;

        validate_command::print_issues(&report);

        if report.error_count() > 0 {
            eprintln!("build stopped; [course={course_id} errors={}]", report.error_count());
            return Ok(false);
        }

        courses.push(keyed_course(&curriculum, &report.loaded_units, &mut plan));
    }

    if options.skip_audio {
        log::info!("audio synthesis skipped; [distinct_clips={}]", plan.utterances.len());
    } else {
        synthesize_audio(paths, &plan, &options.engines)?;
    }

    let recordings: ClipRecordings = audio_synthesis::collect_clip_recordings(&plan, paths)?;
    let bundle: BuiltBundle = artifact::write_bundle(paths, &courses, &plan, &recordings)?;

    artifact::refresh_embedded_bundle(paths, &bundle)?;
    artifact::prune_bundles(paths)?;

    log::info!(
        "build complete; [version={} courses={} distinct_clips={}]",
        bundle.manifest.version,
        courses.len(),
        plan.utterances.len(),
    );

    Ok(true)
}

pub fn keyed_course(curriculum: &Curriculum, units: &[Unit], plan: &mut AudioPlan) -> Course {
    Course {
        id: curriculum.id.clone(),
        title: curriculum.title.clone(),
        target_language: curriculum.target_language.clone(),
        source_language: curriculum.source_language.clone(),
        levels: curriculum.levels.clone(),
        units: units.iter().map(|unit| audio_plan::assign_clip_keys(unit, &curriculum.target_language, plan)).collect(),
    }
}

fn synthesize_audio(paths: &WorkspacePaths, plan: &AudioPlan, engines: &[SpeechEngineKind]) -> Result<(), AppError> {
    audio_synthesis::synthesize_missing_audio(plan, paths, engines)?;

    let removed_count: usize = audio_synthesis::remove_unreferenced_clips(plan, paths)?;

    if removed_count > 0 {
        log::info!("removed clips no unit uses; [count={removed_count}]");
    }

    Ok(())
}
