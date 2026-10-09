use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::PathBuf;

use shared::AppError;
use shared::artifact::{self, CourseIndex, CourseIndexEntry, CourseManifest, repository_key};

use crate::artifact::artifact_bundle::BuiltBundle;
use crate::artifact::course_index_update;
use crate::artifact::json_document;
use crate::artifact::local_files;
use crate::course_source;
use crate::workspace::WorkspacePaths;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddedCourseRemovalKind {
    Removed,
    NotEmbedded,
}

fn embedded_course_index_path(paths: &WorkspacePaths) -> PathBuf {
    paths.embedded_bundle_directory.join(repository_key::COURSE_INDEX_KEY)
}

fn embedded_latest_manifest_path(paths: &WorkspacePaths, course_id: &str) -> PathBuf {
    paths.embedded_bundle_directory.join(repository_key::course_latest_manifest_key(course_id))
}

/// `None` when the bundle has no course index.
pub fn read_embedded_course_index(paths: &WorkspacePaths) -> Result<Option<CourseIndex>, AppError> {
    let index_path: PathBuf = embedded_course_index_path(paths);
    let read: Result<Vec<u8>, io::Error> = fs::read(&index_path);

    match read {
        Ok(index_bytes) => Ok(Some(artifact::parse_course_index(&index_bytes)?)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(AppError::from(format!("could not read the embedded course index; [path={} error={error}]", index_path.display()))),
    }
}

/// `None` when the bundle does not hold the course.
pub fn read_embedded_course_manifest_bytes(paths: &WorkspacePaths, course_id: &str) -> Result<Option<Vec<u8>>, AppError> {
    let manifest_path: PathBuf = embedded_latest_manifest_path(paths, course_id);
    let read: Result<Vec<u8>, io::Error> = fs::read(&manifest_path);

    match read {
        Ok(manifest_bytes) => Ok(Some(manifest_bytes)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(AppError::from(format!("could not read an embedded manifest; [path={} error={error}]", manifest_path.display()))),
    }
}

pub fn read_embedded_publication_label(paths: &WorkspacePaths, course_id: &str) -> Option<String> {
    let manifest_bytes: Vec<u8> = read_embedded_course_manifest_bytes(paths, course_id).ok()??;
    let embedded_manifest: CourseManifest = artifact::parse_course_manifest(&manifest_bytes, Some(course_id)).ok()?;

    Some(embedded_manifest.publication_label)
}

fn write_embedded_course_index(paths: &WorkspacePaths, embedded_index: &CourseIndex) -> Result<(), AppError> {
    fs::create_dir_all(&paths.embedded_bundle_directory)?;

    json_document::write_json_document(&embedded_course_index_path(paths), embedded_index)
}

/// Replaces the course's embedded publication and index entry with the bundle's; refuses a test course.
pub fn refresh_embedded_course(paths: &WorkspacePaths, bundle: &BuiltBundle) -> Result<(), AppError> {
    let course_id: &str = &bundle.manifest.course_id;
    let publication_label: &str = &bundle.manifest.publication_label;

    if bundle.manifest.test_course {
        return Err(AppError::from(format!("the embedded bundle never holds a test course; [course_id={course_id}]")));
    }

    let index_entry: CourseIndexEntry = course_index_update::index_entry_for(&bundle.manifest);
    let mut embedded_index: CourseIndex = read_embedded_course_index(paths)?.unwrap_or_else(course_index_update::empty_course_index);
    let embedded_manifest_bytes: Option<Vec<u8>> = read_embedded_course_manifest_bytes(paths, course_id)?;
    let bundle_manifest_bytes: Vec<u8> = fs::read(bundle.directory.join(artifact::MANIFEST_FILENAME))?;
    let already_embedded: bool = embedded_manifest_bytes.as_ref() == Some(&bundle_manifest_bytes) && embedded_index.courses.get(course_id) == Some(&index_entry);

    if already_embedded {
        return Ok(());
    }

    local_files::remove_directory_if_present(&paths.embedded_course_directory(course_id))?;

    for file_entry in bundle.manifest.file_entries() {
        let destination: PathBuf = paths
            .embedded_bundle_directory
            .join(repository_key::publication_file_key(course_id, publication_label, &file_entry.relative_path));

        local_files::copy_creating_parent(&bundle.directory.join(&file_entry.relative_path), &destination)?;
    }

    local_files::copy_creating_parent(&bundle.directory.join(artifact::MANIFEST_FILENAME), &embedded_latest_manifest_path(paths, course_id))?;

    embedded_index.courses.insert(course_id.to_string(), index_entry);
    write_embedded_course_index(paths, &embedded_index)?;

    log::info!(
        "refreshed the embedded course; [course_id={course_id} publication_label={publication_label} path={}]",
        paths.embedded_course_directory(course_id).display(),
    );

    Ok(())
}

pub fn drop_embedded_course(paths: &WorkspacePaths, course_id: &str) -> Result<EmbeddedCourseRemovalKind, AppError> {
    let course_directory: PathBuf = paths.embedded_course_directory(course_id);
    let held_files: bool = course_directory.exists();

    local_files::remove_directory_if_present(&course_directory)?;

    let removed_index_entry: Option<CourseIndexEntry> = remove_embedded_index_entry(paths, course_id)?;

    if !held_files && removed_index_entry.is_none() {
        return Ok(EmbeddedCourseRemovalKind::NotEmbedded);
    }

    log::info!("dropped a course from the embedded bundle; [course_id={course_id}]");

    Ok(EmbeddedCourseRemovalKind::Removed)
}

fn remove_embedded_index_entry(paths: &WorkspacePaths, course_id: &str) -> Result<Option<CourseIndexEntry>, AppError> {
    let embedded_index: Option<CourseIndex> = read_embedded_course_index(paths)?;

    let Some(mut embedded_index) = embedded_index
    else {
        return Ok(None);
    };
    let removed_entry: Option<CourseIndexEntry> = embedded_index.courses.remove(course_id);

    if removed_entry.is_some() {
        write_embedded_course_index(paths, &embedded_index)?;
    }

    Ok(removed_entry)
}

/// Drops every embedded course whose id is absent from the data or is a test course there.
pub fn drop_embedded_courses_not_taught_in_the_data(paths: &WorkspacePaths) -> Result<(), AppError> {
    let embedded_course_ids: BTreeSet<String> = list_embedded_course_ids(paths)?;
    let data_course_ids: Vec<String> = course_source::list_course_ids(paths)?;

    for course_id in embedded_course_ids {
        let taught_in_the_data: bool = data_course_ids.contains(&course_id) && !course_source::is_test_course_in_the_data(paths, &course_id)?;

        if !taught_in_the_data {
            drop_embedded_course(paths, &course_id)?;
        }
    }

    Ok(())
}

/* The courses the index lists, and any course directory the index has lost track of. */
fn list_embedded_course_ids(paths: &WorkspacePaths) -> Result<BTreeSet<String>, AppError> {
    let mut course_ids: BTreeSet<String> = BTreeSet::new();
    let embedded_index: Option<CourseIndex> = read_embedded_course_index(paths)?;

    if let Some(embedded_index) = embedded_index {
        course_ids.extend(embedded_index.courses.into_keys());
    }

    let courses_directory: PathBuf = paths.embedded_bundle_directory.join(repository_key::COURSES_DIRECTORY);

    if courses_directory.is_dir() {
        for directory_entry in fs::read_dir(&courses_directory)? {
            course_ids.insert(directory_entry?.file_name().to_string_lossy().into_owned());
        }
    }

    Ok(course_ids)
}

#[cfg(test)]
mod tests {
    use shared::course::Course;

    use super::*;
    use crate::audio_plan::AudioPlan;
    use crate::test_courses::{self, FixtureWorkspace, SECOND_COURSE_ID, TEACHING_COURSE_ID, TEST_COURSE_ID};

    fn embedded_course_ids(paths: &WorkspacePaths) -> Vec<String> {
        read_embedded_course_index(paths).unwrap().unwrap().courses.into_keys().collect()
    }

    #[test]
    fn refresh_embedded_course_replaces_only_that_course_and_keeps_its_publication_label() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let mut teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let first_teaching_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);
        let second_bundle: BuiltBundle = workspace.build_bundle(&second_course, &plan);

        teaching_course.title = "Renamed".to_string();
        let renamed_teaching_bundle: BuiltBundle = workspace.build_bundle(&teaching_course, &plan);

        refresh_embedded_course(&workspace.paths, &first_teaching_bundle).unwrap();
        refresh_embedded_course(&workspace.paths, &second_bundle).unwrap();
        let second_course_manifest_before: Vec<u8> = read_embedded_course_manifest_bytes(&workspace.paths, SECOND_COURSE_ID).unwrap().unwrap();
        refresh_embedded_course(&workspace.paths, &renamed_teaching_bundle).unwrap();

        let embedded_index: CourseIndex = read_embedded_course_index(&workspace.paths).unwrap().unwrap();
        let teaching_publication_directory: PathBuf = workspace
            .paths
            .embedded_bundle_directory
            .join(repository_key::publication_directory_key(TEACHING_COURSE_ID, &renamed_teaching_bundle.manifest.publication_label));
        let replaced_publication_directory: PathBuf = workspace
            .paths
            .embedded_bundle_directory
            .join(repository_key::publication_directory_key(TEACHING_COURSE_ID, &first_teaching_bundle.manifest.publication_label));

        assert_eq!(embedded_index.courses[TEACHING_COURSE_ID].title, "Renamed");
        assert_eq!(embedded_index.courses.len(), 2);
        assert_eq!(
            read_embedded_publication_label(&workspace.paths, TEACHING_COURSE_ID),
            Some(renamed_teaching_bundle.manifest.publication_label.clone()),
        );
        assert_eq!(
            read_embedded_course_manifest_bytes(&workspace.paths, TEACHING_COURSE_ID).unwrap().unwrap(),
            fs::read(renamed_teaching_bundle.directory.join(artifact::MANIFEST_FILENAME)).unwrap(),
        );
        assert_eq!(read_embedded_course_manifest_bytes(&workspace.paths, SECOND_COURSE_ID).unwrap().unwrap(), second_course_manifest_before);
        assert!(renamed_teaching_bundle.manifest.file_entries().all(|entry| teaching_publication_directory.join(&entry.relative_path).is_file()));
        assert!(!replaced_publication_directory.exists());
        assert!(!teaching_publication_directory.join(repository_key::RECORDINGS_FILENAME).exists());
    }

    #[test]
    fn refresh_embedded_course_refuses_a_test_course() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let test_course: Course = test_courses::fixture_course(&mut plan, TEST_COURSE_ID, true);
        let test_course_bundle: BuiltBundle = workspace.build_bundle(&test_course, &plan);

        assert!(refresh_embedded_course(&workspace.paths, &test_course_bundle).is_err());
        assert!(!workspace.paths.embedded_bundle_directory.exists());
    }

    #[test]
    fn drop_embedded_courses_not_taught_in_the_data_drops_an_absent_course_and_a_test_course() {
        let workspace: FixtureWorkspace = FixtureWorkspace::new();
        let mut plan: AudioPlan = AudioPlan::new();
        let teaching_course: Course = test_courses::fixture_course(&mut plan, TEACHING_COURSE_ID, false);
        let mut second_course: Course = test_courses::fixture_course(&mut plan, SECOND_COURSE_ID, false);
        let absent_course: Course = test_courses::fixture_course(&mut plan, "pt_br.en.absent", false);

        for course in [&teaching_course, &second_course, &absent_course] {
            let bundle: BuiltBundle = workspace.build_bundle(course, &plan);

            refresh_embedded_course(&workspace.paths, &bundle).unwrap();
        }

        fs::remove_dir_all(workspace.paths.course_directory("pt_br.en.absent")).unwrap();
        second_course.test_course = true;
        workspace.write_curriculum(&second_course);

        drop_embedded_courses_not_taught_in_the_data(&workspace.paths).unwrap();

        assert_eq!(embedded_course_ids(&workspace.paths), vec![TEACHING_COURSE_ID.to_string()]);
        assert!(!workspace.paths.embedded_course_directory(SECOND_COURSE_ID).exists());
        assert!(!workspace.paths.embedded_course_directory("pt_br.en.absent").exists());
        assert!(workspace.paths.embedded_course_directory(TEACHING_COURSE_ID).exists());
    }
}
