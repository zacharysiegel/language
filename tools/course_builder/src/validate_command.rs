use shared::AppError;
use shared::course::{Curriculum, CurriculumUnit, SeverityKind, Unit, ValidationIssue, validate_unit};

use crate::course_source;
use crate::workspace::WorkspacePaths;

pub struct UnitValidationReport {
    pub loaded_units: Vec<Unit>,
    pub missing_unit_ids: Vec<String>,
    pub issues: Vec<ValidationIssue>,
}

impl UnitValidationReport {
    pub fn error_count(&self) -> usize {
        self.count_issues(SeverityKind::Error)
    }

    pub fn warning_count(&self) -> usize {
        self.count_issues(SeverityKind::Warning)
    }

    fn count_issues(&self, severity: SeverityKind) -> usize {
        self.issues.iter().filter(|issue| issue.severity == severity).count()
    }
}

/// A unit file that fails to parse is reported as an error issue rather than aborting the run.
pub fn validate_curriculum_units(
    paths: &WorkspacePaths,
    curriculum_units: &[&CurriculumUnit],
    require_files: bool,
) -> Result<UnitValidationReport, AppError> {
    let mut report: UnitValidationReport = UnitValidationReport {
        loaded_units: Vec::new(),
        missing_unit_ids: Vec::new(),
        issues: Vec::new(),
    };

    for curriculum_unit in curriculum_units {
        let read_result: Result<Option<Unit>, AppError> = course_source::read_unit(paths, &curriculum_unit.id);

        match read_result {
            Ok(Some(unit)) => {
                report.issues.extend(validate_unit(curriculum_unit, &unit));
                report.loaded_units.push(unit);
            },
            Ok(None) => {
                report.missing_unit_ids.push(curriculum_unit.id.clone());

                if require_files {
                    report.issues.push(error_issue(&curriculum_unit.id, "missing unit file"));
                }
            },
            Err(error) => report.issues.push(error_issue(&curriculum_unit.id, &error.message)),
        }
    }

    Ok(report)
}

fn error_issue(unit_id: &str, message: &str) -> ValidationIssue {
    ValidationIssue {
        severity: SeverityKind::Error,
        unit_id: unit_id.to_string(),
        location: "file".to_string(),
        message: message.to_string(),
    }
}

pub fn print_issues(report: &UnitValidationReport) {
    for issue in &report.issues {
        let label: &str = match issue.severity {
            SeverityKind::Error => "error",
            SeverityKind::Warning => "warning",
        };

        eprintln!("{label}: {issue}");
    }
}

pub fn run_validate(paths: &WorkspacePaths, requested_unit_ids: &[String], require_all: bool) -> Result<bool, AppError> {
    let curriculum: Curriculum = course_source::read_curriculum(paths)?;

    for requested_id in requested_unit_ids {
        if !curriculum.units.iter().any(|unit| &unit.id == requested_id) {
            return Err(AppError::from(format!("unknown unit id; [id={requested_id}]")));
        }
    }

    let selected_units: Vec<&CurriculumUnit> = curriculum
        .units
        .iter()
        .filter(|unit| requested_unit_ids.is_empty() || requested_unit_ids.contains(&unit.id))
        .collect();
    let require_files: bool = require_all || !requested_unit_ids.is_empty();
    let report: UnitValidationReport = validate_curriculum_units(paths, &selected_units, require_files)?;

    print_issues(&report);
    println!(
        "checked {} unit(s), {} missing, {} error(s), {} warning(s)",
        report.loaded_units.len(),
        report.missing_unit_ids.len(),
        report.error_count(),
        report.warning_count(),
    );

    Ok(report.error_count() == 0)
}
