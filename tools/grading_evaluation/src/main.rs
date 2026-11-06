mod decisor_client;
mod evaluation_report;
mod evaluation_run;
mod finding_scoring;
mod finding_verification;
mod grader_prompt;
mod grading_case;
mod ollama_client;
mod verification_report;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use shared::AppError;
use shared::filesystem;
use shared::grading::ErrorCategoryKind;

use crate::decisor_client::DecisorClient;
use crate::evaluation_report::{ModelSummary, ScoredRun};
use crate::evaluation_run::{RunOptions, RunRecord};
use crate::finding_verification::{VerificationInputKind, VerificationRecord};
use crate::grader_prompt::PromptVariantKind;
use crate::grading_case::{GradingCase, GradingCaseKind};
use crate::ollama_client::OllamaConnection;

const DEFAULT_CASES_PATH: &str = "tools/grading_evaluation/cases/pt_br.en.json";
const DEFAULT_RUNS_DIRECTORY: &str = "build/grading_evaluation";
const DEFAULT_VERIFICATION_DIRECTORY: &str = "build/grading_evaluation_verification";
const DEFAULT_DECISOR_URL: &str = "http://127.0.0.1:8500";
const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";
const REPORT_FILE_NAME: &str = "report.md";
const FAILURES_FILE_NAME: &str = "failures.md";
const SAMPLE_FILE_NAME: &str = "review-sample.md";

#[derive(Parser)]
#[command(
    about = "Measures how well local language models grade a learner's written answer, by locating mistakes in it"
)]
struct CommandLine {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    /// Check the cases file and count its cases by kind, level and error category.
    CheckCases(CheckCasesArguments),
    /// Grade every case with each model and prompt variant through Ollama, and record the raw replies.
    Run(RunArguments),
    /// Score the recorded runs and write the comparison, the failures and a sample for reading the explanations.
    Report(ReportArguments),
    /// Ask a decision model, through decisor, whether each finding of recorded runs is a real mistake.
    Verify(VerifyArguments),
    /// Print the comparison of the recorded verifications.
    VerifyReport(VerifyReportArguments),
}

#[derive(Args)]
struct CheckCasesArguments {
    /// The cases file; `tools/grading_evaluation/cases/pt_br.en.json` when omitted.
    #[arg(long)]
    cases: Option<PathBuf>,
}

#[derive(Args)]
struct RunArguments {
    /// An Ollama model name such as `gemma3:4b`; repeat to compare several.
    #[arg(long = "model", required = true)]
    models: Vec<String>,

    /// A prompt variant to grade with; repeat for several. Both when omitted.
    #[arg(long = "prompt-variant", value_enum)]
    variants: Vec<PromptVariantKind>,

    /// Grade only the cases whose id contains this text.
    #[arg(long)]
    only: Option<String>,

    /// The sampling temperature.
    #[arg(long, default_value_t = 0.0)]
    temperature: f64,

    /// How many times to grade each case; the seed changes between repeats.
    #[arg(long, default_value_t = 1)]
    repeats: u32,

    #[arg(long)]
    cases: Option<PathBuf>,

    /// Where the runs are recorded; `build/grading_evaluation/` when omitted.
    #[arg(long)]
    output: Option<PathBuf>,

    #[arg(long, default_value = DEFAULT_OLLAMA_URL)]
    ollama_url: String,
}

#[derive(Args)]
struct ReportArguments {
    /// The directory of recorded runs; `build/grading_evaluation/` when omitted.
    #[arg(long)]
    runs: Option<PathBuf>,

    #[arg(long)]
    cases: Option<PathBuf>,

    /// How many cases the review sample holds.
    #[arg(long, default_value_t = 12)]
    sample_size: usize,
}

#[derive(Args)]
struct VerifyArguments {
    /// A recorded run to verify, as `<model>:<prompt variant>` such as `gemma3:12b:few_shot`; repeat for several.
    #[arg(long = "run", required = true)]
    runs: Vec<String>,

    /// A name for the decision model decisor is configured with; it names the recorded file.
    #[arg(long)]
    label: String,

    #[arg(long, value_enum, default_value = "bare")]
    input: VerificationInputKind,

    /// The directory of recorded runs; `build/grading_evaluation/` when omitted.
    #[arg(long)]
    runs_directory: Option<PathBuf>,

    #[arg(long)]
    cases: Option<PathBuf>,

    /// Where the verification is recorded; `build/grading_evaluation_verification/` when omitted.
    #[arg(long)]
    output: Option<PathBuf>,

    #[arg(long, default_value = DEFAULT_DECISOR_URL)]
    decisor_url: String,
}

#[derive(Args)]
struct VerifyReportArguments {
    /// The directory of recorded verifications; `build/grading_evaluation_verification/` when omitted.
    #[arg(long)]
    verifications: Option<PathBuf>,
}

fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let command_line: CommandLine = CommandLine::parse();
    let result: Result<(), AppError> = run(command_line.command);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: CommandKind) -> Result<(), AppError> {
    let root: PathBuf = filesystem::find_workspace_root(&std::env::current_dir()?)?;

    match command {
        CommandKind::CheckCases(arguments) => run_check_cases(&resolve(&root, arguments.cases, DEFAULT_CASES_PATH)),
        CommandKind::Run(arguments) => run_run(&root, arguments),
        CommandKind::Report(arguments) => run_report(&root, arguments),
        CommandKind::Verify(arguments) => run_verify(&root, arguments),
        CommandKind::VerifyReport(arguments) => run_verify_report(&root, arguments),
    }
}

fn resolve(root: &Path, given: Option<PathBuf>, default_relative_path: &str) -> PathBuf {
    given.unwrap_or_else(|| root.join(default_relative_path))
}

fn run_check_cases(cases_path: &Path) -> Result<(), AppError> {
    let cases: Vec<GradingCase> = grading_case::load_cases(cases_path)?;

    let mut by_kind: BTreeMap<GradingCaseKind, usize> = BTreeMap::new();
    let mut by_level: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_category: BTreeMap<ErrorCategoryKind, usize> = BTreeMap::new();

    for case in &cases {
        *by_kind.entry(case.kind()).or_default() += 1;
        *by_level.entry(case.level.clone()).or_default() += 1;

        for reference_error in &case.reference.errors {
            *by_category.entry(reference_error.category).or_default() += 1;
        }
    }

    println!("{} cases, valid", cases.len());

    for kind in GradingCaseKind::ALL {
        println!("kind {}: {}", kind.as_str(), by_kind.get(&kind).copied().unwrap_or(0));
    }

    for (level, count) in &by_level {
        println!("level {level}: {count}");
    }

    for category in ErrorCategoryKind::ALL {
        println!(
            "reference errors {}: {}",
            category.as_str(),
            by_category.get(&category).copied().unwrap_or(0)
        );
    }

    Ok(())
}

fn run_run(root: &Path, arguments: RunArguments) -> Result<(), AppError> {
    let cases_path: PathBuf = resolve(root, arguments.cases, DEFAULT_CASES_PATH);
    let output_directory: PathBuf = resolve(root, arguments.output, DEFAULT_RUNS_DIRECTORY);
    let mut cases: Vec<GradingCase> = grading_case::load_cases(&cases_path)?;

    if let Some(only) = &arguments.only {
        cases.retain(|case| case.id.contains(only.as_str()));
    }

    if cases.is_empty() {
        return Err(AppError::from("no case matches the filter"));
    }

    let variants: Vec<PromptVariantKind> = chosen_variants(arguments.variants);
    let connection: OllamaConnection = OllamaConnection::connect(&arguments.ollama_url)?;
    let options: RunOptions = RunOptions {
        models: &arguments.models,
        variants: &variants,
        temperature: arguments.temperature,
        repeats: arguments.repeats.max(1),
        output_directory: &output_directory,
    };

    let written_paths: Vec<PathBuf> = evaluation_run::run_evaluation(&connection, &cases, &options)?;

    let mut summaries: Vec<ModelSummary> = Vec::new();

    for path in &written_paths {
        let record: RunRecord = evaluation_run::read_run_record(path)?;
        let scored_run: ScoredRun = evaluation_report::score_run(&record, &cases)?;

        summaries.push(evaluation_report::summarize(&scored_run, &cases));
    }

    println!("{}", evaluation_report::render_report(&summaries));

    Ok(())
}

fn chosen_variants(named: Vec<PromptVariantKind>) -> Vec<PromptVariantKind> {
    if named.is_empty() {
        return PromptVariantKind::ALL.to_vec();
    }

    named
}

fn run_report(root: &Path, arguments: ReportArguments) -> Result<(), AppError> {
    let cases_path: PathBuf = resolve(root, arguments.cases, DEFAULT_CASES_PATH);
    let runs_directory: PathBuf = resolve(root, arguments.runs, DEFAULT_RUNS_DIRECTORY);
    let cases: Vec<GradingCase> = grading_case::load_cases(&cases_path)?;

    let mut run_paths: Vec<PathBuf> = list_run_files(&runs_directory)?;
    run_paths.sort();

    if run_paths.is_empty() {
        return Err(AppError::from(format!(
            "no recorded runs; [directory={}]",
            runs_directory.display()
        )));
    }

    let mut scored_runs: Vec<ScoredRun> = Vec::new();

    for path in &run_paths {
        let record: RunRecord = evaluation_run::read_run_record(path)?;

        scored_runs.push(evaluation_report::score_run(&record, &cases)?);
    }

    let mut summaries: Vec<ModelSummary> =
        scored_runs.iter().map(|scored_run| evaluation_report::summarize(scored_run, &cases)).collect();
    summaries.sort_by(|left, right| right.f1.total_cmp(&left.f1));

    let report: String = evaluation_report::render_report(&summaries);

    write_output(&runs_directory, REPORT_FILE_NAME, &report)?;
    write_output(
        &runs_directory,
        FAILURES_FILE_NAME,
        &evaluation_report::render_failures(&scored_runs, &cases),
    )?;
    write_output(
        &runs_directory,
        SAMPLE_FILE_NAME,
        &evaluation_report::render_sample(&scored_runs, &cases, arguments.sample_size),
    )?;

    println!("{report}");

    Ok(())
}

fn list_run_files(runs_directory: &Path) -> Result<Vec<PathBuf>, AppError> {
    let mut run_paths: Vec<PathBuf> = Vec::new();

    for entry in std::fs::read_dir(runs_directory)? {
        let path: PathBuf = entry?.path();
        let is_run: bool = path.extension().is_some_and(|extension| extension == "json");

        if is_run {
            run_paths.push(path);
        }
    }

    Ok(run_paths)
}

fn write_output(directory: &Path, file_name: &str, content: &str) -> Result<(), AppError> {
    let path: PathBuf = directory.join(file_name);

    std::fs::write(&path, content)?;
    log::info!("wrote a report file; [path={}]", path.display());

    Ok(())
}

fn run_verify(root: &Path, arguments: VerifyArguments) -> Result<(), AppError> {
    let cases_path: PathBuf = resolve(root, arguments.cases, DEFAULT_CASES_PATH);
    let runs_directory: PathBuf = resolve(root, arguments.runs_directory, DEFAULT_RUNS_DIRECTORY);
    let output_directory: PathBuf = resolve(root, arguments.output, DEFAULT_VERIFICATION_DIRECTORY);
    let cases: Vec<GradingCase> = grading_case::load_cases(&cases_path)?;

    let mut scored_runs: Vec<ScoredRun> = Vec::new();

    for path in list_run_files(&runs_directory)? {
        let record: RunRecord = evaluation_run::read_run_record(&path)?;
        let run_name: String = format!("{}:{}", record.model, record.prompt_variant);

        if arguments.runs.contains(&run_name) {
            scored_runs.push(evaluation_report::score_run(&record, &cases)?);
        }
    }

    if scored_runs.len() != arguments.runs.len() {
        return Err(AppError::from(format!(
            "not every requested run is recorded; [requested={} found={}]",
            arguments.runs.len(),
            scored_runs.len()
        )));
    }

    let client: DecisorClient = DecisorClient::connect(&arguments.decisor_url)?;
    let record: VerificationRecord =
        finding_verification::verify_runs(&client, &scored_runs, &cases, arguments.input, &arguments.label)?;
    let path: PathBuf = finding_verification::write_record(&output_directory, &record)?;

    log::info!("wrote the verification; [path={}]", path.display());
    println!("{}", verification_report::render_verification_report(&[record]));

    Ok(())
}

fn run_verify_report(root: &Path, arguments: VerifyReportArguments) -> Result<(), AppError> {
    let verifications_directory: PathBuf = resolve(root, arguments.verifications, DEFAULT_VERIFICATION_DIRECTORY);
    let records: Vec<VerificationRecord> = finding_verification::read_records(&verifications_directory)?;

    if records.is_empty() {
        return Err(AppError::from(format!(
            "no recorded verifications; [directory={}]",
            verifications_directory.display()
        )));
    }

    let report: String = verification_report::render_verification_report(&records);

    write_output(&verifications_directory, "report.md", &report)?;
    println!("{report}");

    Ok(())
}
