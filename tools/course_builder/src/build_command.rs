use shared::AppError;
use shared::course::{Course, Curriculum, CurriculumUnit, Unit};

use crate::artifact::{self, BuiltBundle, ClipRecordings};
use crate::audio_plan::{self, AudioPlan};
use crate::audio_synthesis;
use crate::course_source;
use crate::speech_engine::SpeechEngineKind;
use crate::validate_command::{self, UnitValidationReport};
use crate::workspace::WorkspacePaths;

pub const INCLUDE_TEST_COURSES_FLAG: &str = "include-test-courses";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestCourseInclusionKind {
    Exclude,
    Include,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmbeddedBundleRefreshKind {
    Refresh,
    KeepPrevious,
}

pub struct BuildOptions {
    pub course_id: Option<String>,
    pub engines: Vec<SpeechEngineKind>,
    pub skip_audio: bool,
    pub allow_missing: bool,
    pub test_course_inclusion: TestCourseInclusionKind,
}

pub fn run_build(paths: &WorkspacePaths, options: &BuildOptions) -> Result<bool, AppError> {
    let mut plan: AudioPlan = AudioPlan::new();
    let mut courses: Vec<Course> = Vec::new();

    for curriculum in select_built_curricula(paths, options.course_id.as_deref(), options.test_course_inclusion)? {
        let all_units: Vec<&CurriculumUnit> = curriculum.units.iter().collect();
        let report: UnitValidationReport = validate_command::validate_curriculum_units(paths, &curriculum, &all_units, !options.allow_missing)?;

        validate_command::print_issues(&report);

        if report.error_count() > 0 {
            eprintln!("build stopped; [course={} errors={}]", curriculum.id, report.error_count());
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
    let embedded_bundle_refresh: EmbeddedBundleRefreshKind = decide_embedded_bundle_refresh(paths, &courses)?;

    match embedded_bundle_refresh {
        EmbeddedBundleRefreshKind::Refresh => artifact::refresh_embedded_bundle(paths, &bundle)?,
        EmbeddedBundleRefreshKind::KeepPrevious => {
            log::info!("kept the previous embedded bundle, since the bundle leaves out a course that is not a test course; [version={}]", bundle.manifest.version);
        },
    }

    artifact::prune_bundles(paths)?;

    log::info!(
        "build complete; [version={} courses={} distinct_clips={}]",
        bundle.manifest.version,
        courses.len(),
        plan.utterances.len(),
    );

    Ok(true)
}

/// The curricula of the requested course, or of every course; a test course only when test courses are included.
pub fn select_built_curricula(
    paths: &WorkspacePaths,
    requested_course_id: Option<&str>,
    test_course_inclusion: TestCourseInclusionKind,
) -> Result<Vec<Curriculum>, AppError> {
    let mut curricula: Vec<Curriculum> = Vec::new();

    for course_id in validate_command::select_course_ids(paths, requested_course_id)? {
        let curriculum: Curriculum = course_source::read_curriculum(paths, &course_id)?;
        let left_out: bool = curriculum.test_course && test_course_inclusion == TestCourseInclusionKind::Exclude;

        if left_out && requested_course_id.is_some() {
            return Err(AppError::from(format!(
                "the course is a test course, which a build includes only with --{INCLUDE_TEST_COURSES_FLAG}; [course={course_id}]",
            )));
        }

        if left_out {
            log::info!("left out a test course; [course={course_id}]");
            continue;
        }

        curricula.push(curriculum);
    }

    Ok(curricula)
}

pub fn keyed_course(curriculum: &Curriculum, units: &[Unit], plan: &mut AudioPlan) -> Course {
    Course {
        id: curriculum.id.clone(),
        title: curriculum.title.clone(),
        target_language: curriculum.target_language.clone(),
        source_language: curriculum.source_language.clone(),
        levels: curriculum.levels.clone(),
        units: units.iter().map(|unit| audio_plan::assign_clip_keys(unit, &curriculum.target_language, plan)).collect(),
        test_course: curriculum.test_course,
    }
}

/// `Refresh` when `built_courses` holds every course of the data that is not a test course.
fn decide_embedded_bundle_refresh(paths: &WorkspacePaths, built_courses: &[Course]) -> Result<EmbeddedBundleRefreshKind, AppError> {
    for course_id in course_source::list_course_ids(paths)? {
        let curriculum: Curriculum = course_source::read_curriculum(paths, &course_id)?;
        let built: bool = built_courses.iter().any(|course| course.id == course_id);

        if !curriculum.test_course && !built {
            return Ok(EmbeddedBundleRefreshKind::KeepPrevious);
        }
    }

    Ok(EmbeddedBundleRefreshKind::Refresh)
}

fn synthesize_audio(paths: &WorkspacePaths, plan: &AudioPlan, engines: &[SpeechEngineKind]) -> Result<(), AppError> {
    audio_synthesis::synthesize_missing_audio(plan, paths, engines)?;
    remove_clips_no_course_uses(paths)?;

    Ok(())
}

/// Removes every recorded clip that no unit of any course in the data uses, whether or not the build includes the course.
fn remove_clips_no_course_uses(paths: &WorkspacePaths) -> Result<(), AppError> {
    let every_course_clips: Result<AudioPlan, AppError> = plan_every_course_clips(paths);

    let every_course_clips: AudioPlan = match every_course_clips {
        Ok(every_course_clips) => every_course_clips,
        Err(error) => {
            log::warn!("kept every recorded clip, since a course's units could not be read; [error={error}]");
            return Ok(());
        },
    };

    let removed_count: usize = audio_synthesis::remove_unreferenced_clips(&every_course_clips, paths)?;

    if removed_count > 0 {
        log::info!("removed clips no unit uses; [count={removed_count}]");
    }

    Ok(())
}

fn plan_every_course_clips(paths: &WorkspacePaths) -> Result<AudioPlan, AppError> {
    let mut plan: AudioPlan = AudioPlan::new();

    for course_id in course_source::list_course_ids(paths)? {
        let curriculum: Curriculum = course_source::read_curriculum(paths, &course_id)?;

        for unit in course_source::read_present_units(paths, &curriculum)? {
            audio_plan::assign_clip_keys(&unit, &curriculum.target_language, &mut plan);
        }
    }

    Ok(plan)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::audio_encoding;

    const TEACHING_COURSE_ID: &str = "pt_br.en.teaching";
    const TEST_COURSE_ID: &str = "pt_br.en.testing";
    const DATA_TEST_COURSE_ID: &str = "pt_br.en.test_course";
    const UNUSED_CLIP_KEY: &str = "unused";

    fn write_curriculum(paths: &WorkspacePaths, course_id: &str, test_course: bool) {
        let curriculum: Curriculum = Curriculum {
            id: course_id.to_string(),
            title: course_id.to_string(),
            target_language: "pt-BR".to_string(),
            source_language: "en".to_string(),
            levels: Vec::new(),
            units: Vec::new(),
            test_course,
        };
        let curriculum_path: PathBuf = paths.curriculum_path(course_id);

        fs::create_dir_all(curriculum_path.parent().unwrap()).unwrap();
        fs::write(&curriculum_path, serde_json::to_string(&curriculum).unwrap()).unwrap();
    }

    fn workspace_with_a_teaching_and_a_test_course() -> (tempfile::TempDir, WorkspacePaths) {
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let paths: WorkspacePaths = WorkspacePaths::at_root(workspace_directory.path());

        write_curriculum(&paths, TEACHING_COURSE_ID, false);
        write_curriculum(&paths, TEST_COURSE_ID, true);

        (workspace_directory, paths)
    }

    fn selected_course_ids(paths: &WorkspacePaths, requested_course_id: Option<&str>, inclusion: TestCourseInclusionKind) -> Vec<String> {
        let curricula: Vec<Curriculum> = select_built_curricula(paths, requested_course_id, inclusion).unwrap();

        curricula.into_iter().map(|curriculum| curriculum.id).collect()
    }

    #[test]
    fn select_built_curricula_leaves_test_courses_out_by_default() {
        let (_workspace_directory, paths): (tempfile::TempDir, WorkspacePaths) = workspace_with_a_teaching_and_a_test_course();

        assert_eq!(selected_course_ids(&paths, None, TestCourseInclusionKind::Exclude), vec![TEACHING_COURSE_ID]);
    }

    #[test]
    fn select_built_curricula_includes_test_courses_when_asked() {
        let (_workspace_directory, paths): (tempfile::TempDir, WorkspacePaths) = workspace_with_a_teaching_and_a_test_course();

        assert_eq!(selected_course_ids(&paths, None, TestCourseInclusionKind::Include), vec![TEACHING_COURSE_ID, TEST_COURSE_ID]);
        assert_eq!(selected_course_ids(&paths, Some(TEST_COURSE_ID), TestCourseInclusionKind::Include), vec![TEST_COURSE_ID]);
    }

    #[test]
    fn select_built_curricula_refuses_a_requested_test_course_without_the_flag() {
        let (_workspace_directory, paths): (tempfile::TempDir, WorkspacePaths) = workspace_with_a_teaching_and_a_test_course();

        let selected: Result<Vec<Curriculum>, AppError> = select_built_curricula(&paths, Some(TEST_COURSE_ID), TestCourseInclusionKind::Exclude);

        assert!(selected.unwrap_err().to_string().contains(&format!("--{INCLUDE_TEST_COURSES_FLAG}")));
    }

    fn decide_for_built_course_ids(paths: &WorkspacePaths, built_course_ids: &[&str]) -> EmbeddedBundleRefreshKind {
        let mut plan: AudioPlan = AudioPlan::new();
        let built_courses: Vec<Course> = built_course_ids
            .iter()
            .map(|course_id| keyed_course(&course_source::read_curriculum(paths, course_id).unwrap(), &[], &mut plan))
            .collect();

        decide_embedded_bundle_refresh(paths, &built_courses).unwrap()
    }

    #[test]
    fn decide_embedded_bundle_refresh_refreshes_only_when_every_course_but_the_test_courses_is_built() {
        let (_workspace_directory, paths): (tempfile::TempDir, WorkspacePaths) = workspace_with_a_teaching_and_a_test_course();

        assert_eq!(decide_for_built_course_ids(&paths, &[TEACHING_COURSE_ID]), EmbeddedBundleRefreshKind::Refresh);
        assert_eq!(decide_for_built_course_ids(&paths, &[TEACHING_COURSE_ID, TEST_COURSE_ID]), EmbeddedBundleRefreshKind::Refresh);
        assert_eq!(decide_for_built_course_ids(&paths, &[TEST_COURSE_ID]), EmbeddedBundleRefreshKind::KeepPrevious);
        assert_eq!(decide_for_built_course_ids(&paths, &[]), EmbeddedBundleRefreshKind::KeepPrevious);
    }

    fn copy_directory(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).unwrap();

        for directory_entry in fs::read_dir(source).unwrap() {
            let source_path: PathBuf = directory_entry.unwrap().path();
            let destination_path: PathBuf = destination.join(source_path.file_name().unwrap());

            if source_path.is_dir() {
                copy_directory(&source_path, &destination_path);
            } else {
                fs::copy(&source_path, &destination_path).unwrap();
            }
        }
    }

    fn write_clip_file(paths: &WorkspacePaths, engine: SpeechEngineKind, clip_key: &str) -> PathBuf {
        let engine_directory: PathBuf = paths.engine_clip_directory(engine);
        let clip_path: PathBuf = engine_directory.join(format!("{clip_key}.{}", audio_encoding::AUDIO_FILE_EXTENSION));

        fs::create_dir_all(&engine_directory).unwrap();
        fs::write(&clip_path, b"audio").unwrap();

        clip_path
    }

    #[test]
    fn remove_clips_no_course_uses_keeps_the_clips_of_a_course_the_build_leaves_out() {
        let data_paths: WorkspacePaths = WorkspacePaths::locate().unwrap();
        let workspace_directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let paths: WorkspacePaths = WorkspacePaths::at_root(workspace_directory.path());

        copy_directory(&data_paths.course_directory(DATA_TEST_COURSE_ID), &paths.course_directory(DATA_TEST_COURSE_ID));

        let course_clips: AudioPlan = plan_every_course_clips(&paths).unwrap();
        let used_clip_key: &String = course_clips.utterances.keys().next().unwrap();
        let used_clip_paths: Vec<PathBuf> = SpeechEngineKind::ALL.iter().map(|engine| write_clip_file(&paths, *engine, used_clip_key)).collect();
        let unused_clip_paths: Vec<PathBuf> = SpeechEngineKind::ALL.iter().map(|engine| write_clip_file(&paths, *engine, UNUSED_CLIP_KEY)).collect();

        remove_clips_no_course_uses(&paths).unwrap();

        assert!(used_clip_paths.iter().all(|clip_path| clip_path.exists()));
        assert!(unused_clip_paths.iter().all(|clip_path| !clip_path.exists()));
    }
}
