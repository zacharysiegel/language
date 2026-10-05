use std::fs;
use std::path::{Path, PathBuf};

use shared::course::{Curriculum, CurriculumUnit, SeverityKind, Unit, ValidationIssue, validate_unit};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read_curriculum() -> Curriculum {
    let curriculum_text: String = fs::read_to_string(workspace_root().join("data/curriculum.json")).expect("the curriculum exists");

    serde_json::from_str(&curriculum_text).expect("the curriculum parses")
}

#[test]
fn curriculum_unit_orders_are_consecutive_from_one() {
    let curriculum: Curriculum = read_curriculum();
    let orders: Vec<u32> = curriculum.units.iter().map(|unit| unit.order).collect();
    let expected_orders: Vec<u32> = (1..=curriculum.units.len() as u32).collect();

    assert_eq!(orders, expected_orders);
}

#[test]
fn every_curriculum_unit_has_a_file_that_passes_validation() {
    let curriculum: Curriculum = read_curriculum();
    let mut error_messages: Vec<String> = Vec::new();

    for curriculum_unit in &curriculum.units {
        let unit_path: PathBuf = workspace_root().join(format!("data/units/{}.json", curriculum_unit.id));
        let unit_text: String = fs::read_to_string(&unit_path).unwrap_or_else(|error| panic!("{}: {error}", unit_path.display()));
        let unit: Unit = serde_json::from_str(&unit_text).unwrap_or_else(|error| panic!("{}: {error}", unit_path.display()));

        collect_errors(curriculum_unit, &unit, &mut error_messages);
    }

    assert!(error_messages.is_empty(), "{}", error_messages.join("\n"));
}

fn collect_errors(curriculum_unit: &CurriculumUnit, unit: &Unit, error_messages: &mut Vec<String>) {
    let issues: Vec<ValidationIssue> = validate_unit(curriculum_unit, unit);

    for issue in issues.iter().filter(|issue| issue.severity == SeverityKind::Error) {
        error_messages.push(issue.to_string());
    }
}
