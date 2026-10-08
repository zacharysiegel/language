//! The app's routes under a course, in one place so links and route definitions agree.

pub const COURSE_SEGMENT: &str = "course";
pub const UNIT_SEGMENT: &str = "unit";
pub const REVIEW_SEGMENT: &str = "review";
pub const GLOSSARY_SEGMENT: &str = "glossary";
pub const PROGRESS_SEGMENT: &str = "progress";
pub const COURSE_ID_PARAM: &str = "course_id";
pub const UNIT_ID_PARAM: &str = "unit_id";
pub const TAB_QUERY_KEY: &str = "tab";

pub fn course_path(course_id: &str) -> String {
    format!("/{COURSE_SEGMENT}/{course_id}")
}

pub fn unit_path(course_id: &str, unit_id: &str) -> String {
    format!("/{COURSE_SEGMENT}/{course_id}/{UNIT_SEGMENT}/{unit_id}")
}

pub fn unit_tab_path(course_id: &str, unit_id: &str, tab_code: &str) -> String {
    format!("{}?{TAB_QUERY_KEY}={tab_code}", unit_path(course_id, unit_id))
}

pub fn review_path(course_id: &str) -> String {
    format!("/{COURSE_SEGMENT}/{course_id}/{REVIEW_SEGMENT}")
}

pub fn glossary_path(course_id: &str) -> String {
    format!("/{COURSE_SEGMENT}/{course_id}/{GLOSSARY_SEGMENT}")
}

pub fn progress_path(course_id: &str) -> String {
    format!("/{COURSE_SEGMENT}/{course_id}/{PROGRESS_SEGMENT}")
}

/// The course a path is under, if any.
pub fn course_id_in_path(path: &str) -> Option<String> {
    let mut segments = path.trim_start_matches('/').split('/');
    let first_segment: Option<&str> = segments.next();
    let course_id: Option<&str> = segments.next();

    match (first_segment, course_id) {
        (Some(COURSE_SEGMENT), Some(course_id)) if !course_id.is_empty() => Some(course_id.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn course_id_in_path_reads_the_segment_after_course() {
        assert_eq!(course_id_in_path("/course/pt_br.en.foundations/unit/a1"), Some("pt_br.en.foundations".to_string()));
        assert_eq!(course_id_in_path("/course/pt_br.en.foundations"), Some("pt_br.en.foundations".to_string()));
        assert_eq!(course_id_in_path("/"), None);
        assert_eq!(course_id_in_path("/course/"), None);
    }
}
