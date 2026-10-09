mod artifact;
mod audio_encoding;
mod audio_plan;
mod audio_synthesis;
mod build_command;
mod check_embedded_bundle_command;
mod cors_command;
mod course_source;
mod kokoro;
mod publication_label;
mod publish_command;
mod remove_course_command;
mod remove_unused_recordings_command;
mod secrets;
#[cfg(test)] // a helper only tests use
mod test_courses;
mod speech_engine;
mod unlock_command;
mod validate_command;
mod work_pool;
mod workspace;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use shared::AppError;

use crate::artifact::UnlistedCourseInclusionKind;
use crate::build_command::{BuildOptions, TestCourseInclusionKind};
use crate::cors_command::CorsActionKind;
use crate::publish_command::{PublishOptions, RepositoryLocationKind};
use crate::speech_engine::SpeechEngineKind;
use crate::workspace::WorkspacePaths;

#[derive(Parser)]
#[command(about = "Validates the course data, builds each course into a publication of SQLite shards and recordings, and publishes them")]
struct CommandLine {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    /// Check unit files against the curriculum.
    Validate(ValidateArguments),
    /// Validate, synthesize any missing audio, and write each course's bundle of shards, recordings and a manifest.
    Build(BuildArguments),
    /// Publish courses' bundles to a repository: every course its index lists, unless told otherwise.
    Publish(PublishArguments),
    /// Take a course out of a repository's index, then delete its publications.
    RemoveCourse(RemoveCourseArguments),
    /// Delete the recordings no publication in a repository names.
    RemoveUnusedRecordings(RemoveUnusedRecordingsArguments),
    /// Check that the embedded bundle holds exactly the publications Cloudflare R2's pointers name.
    CheckEmbeddedBundle(CheckEmbeddedBundleArguments),
    /// Apply `tools/course_builder/r2-cors.json` to the R2 bucket so the app's origin may read it.
    SetArtifactCors(SetArtifactCorsArguments),
    /// Remove the repository lease a writing command left behind when it stopped without releasing it.
    Unlock(UnlockArguments),
}

#[derive(Args)]
struct RemoveCourseArguments {
    #[command(subcommand)]
    target: RemoveCourseTargetArgument,
}

#[derive(Subcommand)]
enum RemoveCourseTargetArgument {
    /// The local repository directory.
    Local {
        /// The repository directory; `LANGUAGE_LOCAL_REPOSITORY_ROOT`, else `build/repository/`, when omitted.
        #[arg(long)]
        root: Option<PathBuf>,

        /// The course to remove.
        #[arg(long = publish_command::COURSE_FLAG)]
        course_id: String,
    },
    /// The R2 bucket; reads the bucket settings as `publish cloudflare-r2` does.
    CloudflareR2 {
        /// The course to remove.
        #[arg(long = publish_command::COURSE_FLAG)]
        course_id: String,
    },
}

#[derive(Args)]
struct RemoveUnusedRecordingsArguments {
    #[command(subcommand)]
    target: WritableTargetArgument,
}

#[derive(Subcommand)]
enum WritableTargetArgument {
    /// The local repository directory.
    Local {
        /// The repository directory; `LANGUAGE_LOCAL_REPOSITORY_ROOT`, else `build/repository/`, when omitted.
        #[arg(long)]
        root: Option<PathBuf>,
    },
    /// The R2 bucket; reads the bucket settings as `publish cloudflare-r2` does.
    CloudflareR2,
}

#[derive(Args)]
struct CheckEmbeddedBundleArguments {
    #[command(subcommand)]
    target: CheckedTargetArgument,
}

#[derive(Subcommand)]
enum CheckedTargetArgument {
    /// Read the R2 bucket's course index and pointers; reads the bucket settings as `publish cloudflare-r2` does.
    CloudflareR2,
}

#[derive(Args)]
struct UnlockArguments {
    #[command(subcommand)]
    target: UnlockTargetArgument,
}

#[derive(Subcommand)]
enum UnlockTargetArgument {
    /// Delete the lease object in the R2 bucket; reads the bucket settings as `publish cloudflare-r2` does.
    CloudflareR2,
}

#[derive(Args)]
struct SetArtifactCorsArguments {
    /// Report the bucket's policy without changing it.
    #[arg(long)]
    list: bool,
}

#[derive(Args)]
struct ValidateArguments {
    /// The course to check; every course when omitted.
    #[arg(long = "course")]
    course_id: Option<String>,

    /// Units to check; every existing unit file when omitted.
    unit_ids: Vec<String>,

    /// Treat a curriculum unit without a file as an error.
    #[arg(long)]
    all: bool,
}

#[derive(Args)]
struct BuildArguments {
    /// The course to build; every course when omitted.
    #[arg(long = "course")]
    course_id: Option<String>,

    /// An engine that speaks the course; repeat for several. Every engine is used when none is named.
    #[arg(long = "engine", value_enum)]
    engines: Vec<SpeechEngineKind>,

    /// Write the bundle from the recordings already made, without synthesizing.
    #[arg(long)]
    skip_audio: bool,

    /// Build only the unit files that exist.
    #[arg(long)]
    allow_missing: bool,

    /// Include the courses marked as test courses, for local development and testing.
    #[arg(long = build_command::INCLUDE_TEST_COURSES_FLAG)]
    include_test_courses: bool,
}

#[derive(Args)]
struct PublishArguments {
    #[command(subcommand)]
    target: PublishTargetArgument,

    /// A course to publish from its newest bundle, whether or not the target's index lists it; repeat for several.
    #[arg(long = publish_command::COURSE_FLAG, global = true)]
    course_ids: Vec<String>,

    /// One course's bundle directory to publish, in place of its newest bundle.
    #[arg(long, global = true)]
    bundle: Option<PathBuf>,

    /// Also publish every built course in `data/courses/` that the target's index does not list.
    #[arg(long = publish_command::INCLUDE_UNLISTED_COURSES_FLAG, global = true, conflicts_with_all = ["course_ids", "bundle"])]
    include_unlisted_courses: bool,
}

#[derive(Subcommand)]
enum PublishTargetArgument {
    /// Copy into a directory that `repository_server` serves.
    Local {
        /// The repository directory; `LANGUAGE_LOCAL_REPOSITORY_ROOT`, else `build/repository/`, when omitted.
        #[arg(long)]
        root: Option<PathBuf>,

        /// The URL the repository directory is served at.
        #[arg(long)]
        public_base_url: Option<String>,
    },
    /// Upload to Cloudflare R2; reads the bucket settings from `.env` and the secret access key from `secrets.yaml`.
    CloudflareR2,
    /// Log every upload, pointer write and index write as if the repository were empty, without making any.
    Dry,
}

fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let command_line: CommandLine = CommandLine::parse();
    let result: Result<bool, AppError> = run(command_line.command);

    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        },
    }
}

fn chosen_engines(named: Vec<SpeechEngineKind>) -> Vec<SpeechEngineKind> {
    if named.is_empty() {
        return SpeechEngineKind::ALL.to_vec();
    }

    named
}

fn test_course_inclusion(include_test_courses: bool) -> TestCourseInclusionKind {
    if include_test_courses {
        return TestCourseInclusionKind::Include;
    }

    TestCourseInclusionKind::Exclude
}

fn unlisted_course_inclusion(include_unlisted_courses: bool) -> UnlistedCourseInclusionKind {
    if include_unlisted_courses {
        return UnlistedCourseInclusionKind::Include;
    }

    UnlistedCourseInclusionKind::Exclude
}

fn writable_location(target: WritableTargetArgument) -> RepositoryLocationKind {
    match target {
        WritableTargetArgument::Local { root } => RepositoryLocationKind::Local {
            root,
            public_base_url: None,
        },
        WritableTargetArgument::CloudflareR2 => RepositoryLocationKind::CloudflareR2,
    }
}

fn run(command: CommandKind) -> Result<bool, AppError> {
    let paths: WorkspacePaths = WorkspacePaths::locate()?;

    match command {
        CommandKind::Validate(arguments) => validate_command::run_validate(&paths, arguments.course_id.as_deref(), &arguments.unit_ids, arguments.all),
        CommandKind::Build(arguments) => build_command::run_build(
            &paths,
            &BuildOptions {
                course_id: arguments.course_id,
                engines: chosen_engines(arguments.engines),
                skip_audio: arguments.skip_audio,
                allow_missing: arguments.allow_missing,
                test_course_inclusion: test_course_inclusion(arguments.include_test_courses),
            },
        ),
        CommandKind::Publish(arguments) => {
            let location: RepositoryLocationKind = match arguments.target {
                PublishTargetArgument::Local { root, public_base_url } => RepositoryLocationKind::Local { root, public_base_url },
                PublishTargetArgument::CloudflareR2 => RepositoryLocationKind::CloudflareR2,
                PublishTargetArgument::Dry => RepositoryLocationKind::Dry,
            };
            let options: PublishOptions = PublishOptions {
                course_ids: arguments.course_ids,
                bundle: arguments.bundle,
                unlisted_course_inclusion: unlisted_course_inclusion(arguments.include_unlisted_courses),
            };

            publish_command::run_publish(&paths, location, options)
        },
        CommandKind::RemoveCourse(arguments) => match arguments.target {
            RemoveCourseTargetArgument::Local { root, course_id } => {
                remove_course_command::run_remove_course(&paths, writable_location(WritableTargetArgument::Local { root }), &course_id)
            },
            RemoveCourseTargetArgument::CloudflareR2 { course_id } => {
                remove_course_command::run_remove_course(&paths, writable_location(WritableTargetArgument::CloudflareR2), &course_id)
            },
        },
        CommandKind::RemoveUnusedRecordings(arguments) => {
            remove_unused_recordings_command::run_remove_unused_recordings(&paths, writable_location(arguments.target))
        },
        CommandKind::CheckEmbeddedBundle(arguments) => match arguments.target {
            CheckedTargetArgument::CloudflareR2 => check_embedded_bundle_command::run_check_embedded_bundle_cloudflare_r2(&paths),
        },
        CommandKind::SetArtifactCors(arguments) => {
            let action: CorsActionKind = if arguments.list { CorsActionKind::List } else { CorsActionKind::Apply };

            cors_command::run_set_artifact_cors(action)
        },
        CommandKind::Unlock(arguments) => match arguments.target {
            UnlockTargetArgument::CloudflareR2 => unlock_command::run_unlock_cloudflare_r2(),
        },
    }
}
