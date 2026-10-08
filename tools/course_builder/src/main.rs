mod artifact;
mod audio_encoding;
mod audio_plan;
mod audio_synthesis;
mod build_command;
mod cors_command;
mod course_source;
mod kokoro;
mod publish_command;
mod secrets;
#[cfg(test)] // a helper only tests use
mod test_courses;
mod speech_engine;
mod validate_command;
mod version_label;
mod work_pool;
mod workspace;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use shared::AppError;

use crate::build_command::BuildOptions;
use crate::cors_command::CorsActionKind;
use crate::publish_command::PublishTargetKind;
use crate::speech_engine::SpeechEngineKind;
use crate::workspace::WorkspacePaths;

#[derive(Parser)]
#[command(about = "Validates the course data, builds it into versioned SQLite shards and recordings, and publishes them")]
struct CommandLine {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    /// Check unit files against the curriculum.
    Validate(ValidateArguments),
    /// Validate, synthesize any missing audio, and write a bundle of shards, recordings and a manifest.
    Build(BuildArguments),
    /// Upload a bundle to a repository, the latest-manifest pointer last.
    Publish(PublishArguments),
    /// Apply `tools/course_builder/r2-cors.json` to the R2 bucket so the app's origin may read it.
    SetArtifactCors(SetArtifactCorsArguments),
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
}

#[derive(Args)]
struct PublishArguments {
    #[command(subcommand)]
    target: PublishTargetArgument,

    /// The bundle directory to publish; the newest under `build/artifacts/` when omitted.
    #[arg(long, global = true)]
    bundle: Option<PathBuf>,
}

#[derive(Subcommand)]
enum PublishTargetArgument {
    /// Copy into a directory that the web server serves at `/repository`.
    Local {
        /// The repository directory; `PORTUGUESE_COURSE_LOCAL_REPOSITORY_ROOT`, else `build/repository/`, when omitted.
        #[arg(long)]
        root: Option<PathBuf>,

        /// The URL the repository directory is served at.
        #[arg(long)]
        public_base_url: Option<String>,
    },
    /// Upload to Cloudflare R2; reads the bucket settings from `.env` and the secret access key from `secrets.yaml`.
    CloudflareR2,
    /// Log every upload without making it.
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
            },
        ),
        CommandKind::Publish(arguments) => {
            let target: PublishTargetKind = match arguments.target {
                PublishTargetArgument::Local { root, public_base_url } => PublishTargetKind::Local { root, public_base_url },
                PublishTargetArgument::CloudflareR2 => PublishTargetKind::CloudflareR2,
                PublishTargetArgument::Dry => PublishTargetKind::Dry,
            };

            publish_command::run_publish(&paths, target, arguments.bundle)
        },
        CommandKind::SetArtifactCors(arguments) => {
            let action: CorsActionKind = if arguments.list { CorsActionKind::List } else { CorsActionKind::Apply };

            cors_command::run_set_artifact_cors(action)
        },
    }
}
