mod audio_encoding;
mod audio_plan;
mod audio_synthesis;
mod build_command;
mod course_source;
mod kokoro;
mod speech_engine;
mod validate_command;
mod work_pool;
mod workspace;

use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use shared::AppError;

use crate::speech_engine::SpeechEngineKind;
use crate::workspace::WorkspacePaths;

#[derive(Parser)]
#[command(about = "Validates the course data and compiles it, with synthesized audio, for the web app")]
struct CommandLine {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    /// Check unit files against the curriculum.
    Validate(ValidateArguments),
    /// Validate, compile the course and synthesize any missing audio.
    Build(BuildArguments),
}

#[derive(Args)]
struct ValidateArguments {
    /// Units to check; every existing unit file when omitted.
    unit_ids: Vec<String>,

    /// Treat a curriculum unit without a file as an error.
    #[arg(long)]
    all: bool,
}

#[derive(Args)]
struct BuildArguments {
    /// The engine that speaks the course.
    #[arg(long, value_enum, default_value_t = SpeechEngineKind::Kokoro)]
    engine: SpeechEngineKind,

    /// Write the compiled course without synthesizing audio.
    #[arg(long)]
    skip_audio: bool,

    /// Compile only the unit files that exist.
    #[arg(long)]
    allow_missing: bool,
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

fn run(command: CommandKind) -> Result<bool, AppError> {
    let paths: WorkspacePaths = WorkspacePaths::locate()?;

    match command {
        CommandKind::Validate(arguments) => validate_command::run_validate(&paths, &arguments.unit_ids, arguments.all),
        CommandKind::Build(arguments) => build_command::run_build(&paths, arguments.engine, arguments.skip_audio, arguments.allow_missing),
    }
}
