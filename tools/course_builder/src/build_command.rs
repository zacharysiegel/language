use std::fs;
use std::path::PathBuf;

use shared::AppError;
use shared::course::{Course, Curriculum, CurriculumUnit, Unit};

use crate::audio_plan::{self, AudioPlan};
use crate::audio_synthesis;
use crate::course_source;
use crate::validate_command::{self, UnitValidationReport};
use crate::workspace::WorkspacePaths;

pub fn run_build(paths: &WorkspacePaths, skip_audio: bool, allow_missing: bool) -> Result<bool, AppError> {
    let curriculum: Curriculum = course_source::read_curriculum(paths)?;
    let all_units: Vec<&CurriculumUnit> = curriculum.units.iter().collect();
    let report: UnitValidationReport = validate_command::validate_curriculum_units(paths, &all_units, !allow_missing)?;

    validate_command::print_issues(&report);

    if report.error_count() > 0 {
        eprintln!("build stopped; [errors={}]", report.error_count());
        return Ok(false);
    }

    let mut plan: AudioPlan = AudioPlan::default();
    let compiled_units: Vec<Unit> = report
        .loaded_units
        .iter()
        .map(|unit| audio_plan::compile_unit(unit, &mut plan))
        .collect();

    let course: Course = Course {
        title: curriculum.title.clone(),
        levels: curriculum.levels.clone(),
        units: compiled_units,
    };

    write_course(paths, &course)?;

    if skip_audio {
        log::info!("audio skipped; [distinct_clips={}]", plan.utterances.len());
        return Ok(true);
    }

    let audio_directory: PathBuf = paths.audio_directory();

    audio_synthesis::synthesize_missing_audio(&plan, &audio_directory)?;

    let removed_count: usize = audio_synthesis::remove_unreferenced_audio(&plan, &audio_directory)?;

    if removed_count > 0 {
        log::info!("removed unreferenced audio; [count={removed_count}]");
    }

    log::info!("build complete; [units={} distinct_clips={}]", course.units.len(), plan.utterances.len());

    Ok(true)
}

fn write_course(paths: &WorkspacePaths, course: &Course) -> Result<(), AppError> {
    fs::create_dir_all(&paths.compiled_course_directory)?;

    let course_text: String = serde_json::to_string(course)?;
    let course_path: PathBuf = paths.compiled_course_path();

    fs::write(&course_path, format!("{course_text}\n"))?;

    log::info!("wrote course; [path={} units={} bytes={}]", course_path.display(), course.units.len(), course_text.len());

    Ok(())
}
