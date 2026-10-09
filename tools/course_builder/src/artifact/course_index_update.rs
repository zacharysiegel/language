use std::collections::BTreeMap;

use shared::AppError;
use shared::artifact::{self, CourseIndex, CourseIndexEntry, CourseManifest, course_index, repository_key};

use crate::artifact::json_document;
use crate::artifact::repository::{ArtifactRepository, ObjectHeaders, StoredObject, WriteConditionKind, WriteOutcomeKind};

/// A repository's course index as read, with what a conditional write of its replacement needs.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredCourseIndex {
    pub course_index: CourseIndex,
    /// `None` when the repository has no course index.
    pub entity_tag: Option<String>,
}

impl StoredCourseIndex {
    fn replacement_condition(&self) -> WriteConditionKind {
        match &self.entity_tag {
            Some(entity_tag) => WriteConditionKind::MatchesEntityTag(entity_tag.clone()),
            None => WriteConditionKind::Absent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseIndexChangeKind {
    Unchanged,
    Added,
    Updated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseIndexRemovalKind {
    Removed,
    NotListed,
}

pub fn empty_course_index() -> CourseIndex {
    CourseIndex {
        course_index_schema_version: course_index::COURSE_INDEX_SCHEMA_VERSION,
        courses: BTreeMap::new(),
    }
}

/// A repository without a course index reads as one that lists no course.
pub async fn read_course_index(repository: &impl ArtifactRepository) -> Result<StoredCourseIndex, AppError> {
    let stored_object: Option<StoredObject> = repository.get_object(repository_key::COURSE_INDEX_KEY).await?;

    let Some(stored_object) = stored_object else {
        return Ok(StoredCourseIndex {
            course_index: empty_course_index(),
            entity_tag: None,
        });
    };
    let course_index: CourseIndex = artifact::parse_course_index(&stored_object.bytes)
        .map_err(|error| AppError::from(format!("the repository's course index is unreadable; [url={} error={error}]", repository.url(repository_key::COURSE_INDEX_KEY))))?;

    Ok(StoredCourseIndex {
        course_index,
        entity_tag: Some(stored_object.entity_tag),
    })
}

pub fn index_entry_for(course_manifest: &CourseManifest) -> CourseIndexEntry {
    CourseIndexEntry {
        title: course_manifest.title.clone(),
        target_language: course_manifest.target_language.clone(),
        source_language: course_manifest.source_language.clone(),
        latest_manifest_key: repository_key::course_latest_manifest_key(&course_manifest.course_id),
        test_course: course_manifest.test_course,
    }
}

/// Writes the index only when the course's entry changes, conditional on `stored_index` being the current index.
pub async fn set_course_index_entry(
    repository: &impl ArtifactRepository,
    stored_index: &StoredCourseIndex,
    course_id: &str,
    entry: CourseIndexEntry,
) -> Result<CourseIndexChangeKind, AppError> {
    let current_entry: Option<&CourseIndexEntry> = stored_index.course_index.courses.get(course_id);

    let change: CourseIndexChangeKind = match current_entry {
        Some(current_entry) if *current_entry == entry => return Ok(CourseIndexChangeKind::Unchanged),
        Some(_) => CourseIndexChangeKind::Updated,
        None => CourseIndexChangeKind::Added,
    };
    let mut replacement_index: CourseIndex = stored_index.course_index.clone();

    replacement_index.course_index_schema_version = course_index::COURSE_INDEX_SCHEMA_VERSION;
    replacement_index.courses.insert(course_id.to_string(), entry);
    write_course_index(repository, &replacement_index, stored_index.replacement_condition()).await?;

    Ok(change)
}

/// Rewrites the index without the course, conditional on the index not changing since it was read.
pub async fn remove_course_index_entry(repository: &impl ArtifactRepository, course_id: &str) -> Result<CourseIndexRemovalKind, AppError> {
    let stored_index: StoredCourseIndex = read_course_index(repository).await?;
    let mut replacement_index: CourseIndex = stored_index.course_index.clone();
    let removed_entry: Option<CourseIndexEntry> = replacement_index.courses.remove(course_id);

    if removed_entry.is_none() {
        return Ok(CourseIndexRemovalKind::NotListed);
    }

    write_course_index(repository, &replacement_index, stored_index.replacement_condition()).await?;

    Ok(CourseIndexRemovalKind::Removed)
}

async fn write_course_index(repository: &impl ArtifactRepository, replacement_index: &CourseIndex, condition: WriteConditionKind) -> Result<(), AppError> {
    let index_bytes: Vec<u8> = json_document::json_document_bytes(replacement_index)?;
    let outcome: WriteOutcomeKind = repository
        .put_bytes(repository_key::COURSE_INDEX_KEY, &index_bytes, ObjectHeaders::for_key(repository_key::COURSE_INDEX_KEY), condition)
        .await?;

    match outcome {
        WriteOutcomeKind::Written => Ok(()),
        WriteOutcomeKind::PreconditionFailed => Err(AppError::from(format!(
            "the course index changed while this command held the repository lock, so another writer ignored the lock; [url={}]",
            repository.url(repository_key::COURSE_INDEX_KEY),
        ))),
    }
}
