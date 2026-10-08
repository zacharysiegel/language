use shared::course::{Course, Curriculum, Unit};

use crate::audio_plan::AudioPlan;
use crate::build_command;
use crate::course_source;
use crate::workspace::WorkspacePaths;

/// Every course in the repository's data, with clip keys assigned in `plan`.
pub fn read_keyed_courses(plan: &mut AudioPlan) -> Vec<Course> {
    let paths: WorkspacePaths = WorkspacePaths::locate().unwrap();

    course_source::list_course_ids(&paths)
        .unwrap()
        .iter()
        .map(|course_id| {
            let curriculum: Curriculum = course_source::read_curriculum(&paths, course_id).unwrap();
            let units: Vec<Unit> = curriculum
                .units
                .iter()
                .filter_map(|curriculum_unit| course_source::read_unit(&paths, course_id, &curriculum_unit.id).unwrap())
                .collect();

            build_command::keyed_course(&curriculum, &units, plan)
        })
        .collect()
}
