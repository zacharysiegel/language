use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use shared::artifact::content_address;
use shared::course::{Course, Curriculum, Unit};
use shared::course_shard::{ClipAudioEntry, VoiceEngineKind};

use crate::artifact::{self, BuiltBundle, ClipRecordings, CourseBundleOutcomeKind};
use crate::audio_plan::AudioPlan;
use crate::build_command;
use crate::course_source;
use crate::workspace::WorkspacePaths;

pub const TEACHING_COURSE_ID: &str = "pt_br.en.teaching";
pub const SECOND_COURSE_ID: &str = "pt_br.en.second";
pub const TEST_COURSE_ID: &str = "pt_br.en.testing";
pub const RECORDINGS_PER_FIXTURE_COURSE: usize = 3;

const UNITS_PER_FIXTURE_COURSE: usize = 2;

/// Every course in the repository's data, with clip keys assigned in `plan`.
pub fn read_keyed_courses(plan: &mut AudioPlan) -> Vec<Course> {
    let paths: WorkspacePaths = WorkspacePaths::locate().unwrap();

    course_source::list_course_ids(&paths)
        .unwrap()
        .iter()
        .map(|course_id| {
            let curriculum: Curriculum = course_source::read_curriculum(&paths, course_id).unwrap();
            let units: Vec<Unit> = course_source::read_present_units(&paths, &curriculum).unwrap();

            build_command::keyed_course(&curriculum, &units, plan)
        })
        .collect()
}

/// A copy of the data's teaching course, cut to a few units, under `course_id`.
pub fn fixture_course(plan: &mut AudioPlan, course_id: &str, test_course: bool) -> Course {
    let mut course: Course = read_keyed_courses(plan).into_iter().find(|course| !course.test_course).unwrap();

    course.units.truncate(UNITS_PER_FIXTURE_COURSE);
    course.id = course_id.to_string();
    course.test_course = test_course;

    course
}

/// A temporary workspace with its own data, bundles, local repository, embedded bundle and clip directory.
pub struct FixtureWorkspace {
    _directory: tempfile::TempDir,
    pub paths: WorkspacePaths,
    pub clip_directory: PathBuf,
}

impl FixtureWorkspace {
    pub fn new() -> FixtureWorkspace {
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let paths: WorkspacePaths = WorkspacePaths::at_root(directory.path());
        let clip_directory: PathBuf = directory.path().join("clips");

        fs::create_dir_all(&clip_directory).unwrap();
        fs::create_dir_all(&paths.courses_directory).unwrap();

        FixtureWorkspace {
            _directory: directory,
            paths,
            clip_directory,
        }
    }

    /// Writes the course's curriculum into the workspace's data, without its units.
    pub fn write_curriculum(&self, course: &Course) {
        let curriculum: Curriculum = Curriculum {
            id: course.id.clone(),
            title: course.title.clone(),
            target_language: course.target_language.clone(),
            source_language: course.source_language.clone(),
            levels: course.levels.clone(),
            units: Vec::new(),
            test_course: course.test_course,
        };
        let curriculum_path: PathBuf = self.paths.curriculum_path(&course.id);

        fs::create_dir_all(curriculum_path.parent().unwrap()).unwrap();
        fs::write(&curriculum_path, serde_json::to_string(&curriculum).unwrap()).unwrap();
    }

    pub fn recordings_for(&self, course: &Course) -> ClipRecordings {
        recordings_for(&self.clip_directory, course)
    }

    /// Writes the course's curriculum, then a bundle of it, which must hold new course content.
    pub fn build_bundle(&self, course: &Course, plan: &AudioPlan) -> BuiltBundle {
        self.write_curriculum(course);

        let outcome: CourseBundleOutcomeKind = artifact::write_course_bundle(&self.paths, course, plan, &self.recordings_for(course)).unwrap();

        match outcome {
            CourseBundleOutcomeKind::Written(bundle) => bundle,
            CourseBundleOutcomeKind::Unchanged(bundle) => panic!("the fixture's course content was unchanged; [path={}]", bundle.directory.display()),
        }
    }
}

/// Kokoro recordings, distinct per course, for the first few vocabulary clips only; the rest of the course has none.
pub fn recordings_for(clip_directory: &Path, course: &Course) -> ClipRecordings {
    let mut recordings: ClipRecordings = ClipRecordings {
        audio_index: BTreeMap::new(),
        file_by_audio_sha256: BTreeMap::new(),
        pipeline_revision_by_engine_code: BTreeMap::from([
            (VoiceEngineKind::Kokoro.code().to_string(), 3),
            (VoiceEngineKind::MacosSay.code().to_string(), 1),
        ]),
    };
    let clip_keys: Vec<String> = course.units[0]
        .vocabulary
        .iter()
        .filter_map(|item| item.clip_key.clone())
        .take(RECORDINGS_PER_FIXTURE_COURSE)
        .collect();

    for clip_key in clip_keys {
        let bytes: Vec<u8> = format!("{} recording of {clip_key}", course.id).into_bytes();
        let audio_sha256: String = content_address::sha256_hex(&bytes);
        let path: PathBuf = clip_directory.join(format!("{}-{clip_key}.m4a", course.id));

        fs::write(&path, &bytes).unwrap();
        recordings.audio_index.entry(clip_key.clone()).or_default().push(ClipAudioEntry {
            clip_key,
            engine: VoiceEngineKind::Kokoro,
            audio_sha256: audio_sha256.clone(),
            audio_size_bytes: bytes.len() as u64,
        });
        recordings.file_by_audio_sha256.insert(audio_sha256, path);
    }

    recordings
}
