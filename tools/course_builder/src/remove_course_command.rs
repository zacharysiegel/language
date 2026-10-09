use shared::AppError;

use crate::artifact::repository::{self, ArtifactRepositoryKind, RepositoryLockKind};
use crate::artifact::{self, CourseIndexRemovalKind, CourseRemovalReport, EmbeddedCourseRemovalKind, RepositoryTargetKind};
use crate::publish_command::{self, RepositoryLocationKind};
use crate::workspace::WorkspacePaths;

pub fn run_remove_course(paths: &WorkspacePaths, location: RepositoryLocationKind, course_id: &str) -> Result<bool, AppError> {
    let target: RepositoryTargetKind = location.target();
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let report: CourseRemovalReport = runtime.block_on(async {
        let repository: ArtifactRepositoryKind = publish_command::create_repository(paths, location).await?;
        let lock: RepositoryLockKind = repository::acquire_repository_lock(&repository).await?;

        let removed: Result<CourseRemovalReport, AppError> = artifact::remove_course(paths, &repository, target, course_id).await;
        let released: Result<(), AppError> = lock.release(&repository).await;

        repository::combine_with_release(removed, released)
    })?;

    if report.index_removal == CourseIndexRemovalKind::NotListed {
        log::info!("the course index does not list the course; [course_id={course_id}]");
    }

    log::info!(
        "removed the course; [course_id={course_id} target={} deleted_objects={} embedded={} recordings_removed={}]",
        target.argument_name(),
        report.deleted_object_count,
        describe_embedded_removal(report.embedded_removal),
        report.recordings_removed,
    );

    if target == RepositoryTargetKind::CloudflareR2 {
        log::info!("recordings no publication names stay until `remove-unused-recordings cloudflare-r2` removes them");
    }

    Ok(true)
}

fn describe_embedded_removal(embedded_removal: EmbeddedCourseRemovalKind) -> &'static str {
    match embedded_removal {
        EmbeddedCourseRemovalKind::Removed => "removed",
        EmbeddedCourseRemovalKind::NotEmbedded => "not_embedded",
    }
}
