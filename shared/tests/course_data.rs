use std::fs;
use std::path::{Path, PathBuf};

use shared::course::{Curriculum, CurriculumUnit, SeverityKind, Unit, ValidationIssue};

const CURRICULUM_FILE_NAME: &str = "curriculum.json";

fn courses_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/courses")
}

/// Every course under `data/courses/`, with its directory.
fn read_courses() -> Vec<(PathBuf, Curriculum)> {
    let mut courses: Vec<(PathBuf, Curriculum)> = Vec::new();

    for directory_entry in fs::read_dir(courses_directory()).expect("the courses directory exists") {
        let course_directory: PathBuf = directory_entry.expect("a readable entry").path();
        let curriculum_path: PathBuf = course_directory.join(CURRICULUM_FILE_NAME);
        let curriculum_text: String = fs::read_to_string(&curriculum_path).unwrap_or_else(|error| panic!("{}: {error}", curriculum_path.display()));
        let curriculum: Curriculum = serde_json::from_str(&curriculum_text).unwrap_or_else(|error| panic!("{}: {error}", curriculum_path.display()));

        courses.push((course_directory, curriculum));
    }

    assert!(!courses.is_empty());

    courses
}

#[test]
fn every_course_id_names_its_languages_and_matches_its_directory() {
    for (course_directory, curriculum) in read_courses() {
        let directory_name: &str = course_directory.file_name().and_then(|name| name.to_str()).unwrap_or_default();

        assert_eq!(shared::course::course_id_issue(&curriculum), None);
        assert_eq!(directory_name, curriculum.id);
    }
}

#[test]
fn curriculum_unit_orders_are_consecutive_from_one() {
    for (_, curriculum) in read_courses() {
        let orders: Vec<u32> = curriculum.units.iter().map(|unit| unit.order).collect();
        let expected_orders: Vec<u32> = (1..=curriculum.units.len() as u32).collect();

        assert_eq!(orders, expected_orders, "{}", curriculum.id);
    }
}

#[test]
fn every_curriculum_unit_has_a_file_that_passes_validation() {
    let mut error_messages: Vec<String> = Vec::new();

    for (course_directory, curriculum) in read_courses() {
        for curriculum_unit in &curriculum.units {
            let unit_path: PathBuf = course_directory.join("units").join(format!("{}.json", curriculum_unit.id));
            let unit_text: String = fs::read_to_string(&unit_path).unwrap_or_else(|error| panic!("{}: {error}", unit_path.display()));
            let unit: Unit = serde_json::from_str(&unit_text).unwrap_or_else(|error| panic!("{}: {error}", unit_path.display()));

            collect_errors(&curriculum, curriculum_unit, &unit, &mut error_messages);
        }
    }

    assert!(error_messages.is_empty(), "{}", error_messages.join("\n"));
}

fn collect_errors(curriculum: &Curriculum, curriculum_unit: &CurriculumUnit, unit: &Unit, error_messages: &mut Vec<String>) {
    let issues: Vec<ValidationIssue> = shared::course::validate_unit(curriculum, curriculum_unit, unit);

    for issue in issues.iter().filter(|issue| issue.severity == SeverityKind::Error) {
        error_messages.push(format!("{}: {issue}", curriculum.id));
    }
}
