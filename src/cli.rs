use std::{env, ffi::OsString, path::PathBuf, process::ExitCode};

use serde::Serialize;
use validator::{
    CheckOptions, ComparisonOptions, Diagnostic, DiagnosticCode, EvaluationOptions, ExitCategory,
    InspectionOptions, check, compare, evaluate, inspect,
};

const CHECK_HELP: &str = "validator check --dataset PATH --predictions PATH --config PATH\nValidate inputs and evidence without scoring or publishing a run.";
const EVALUATE_HELP: &str = "validator evaluate --dataset PATH --predictions PATH --config PATH --out NEW_RUN_DIR\nEvaluate saved predictions and publish a new run. --out is required and must not exist.";
const INSPECT_HELP: &str = "validator inspect --run RUN_DIR --episode UUID\nVerify and replay a stored run before returning one selected episode.";
const COMPARE_HELP: &str = "validator compare --baseline RUN_DIR --candidate RUN_DIR --out NEW_COMPARISON_DIR [--intersection]\nVerify and compare paired runs. --out is required and must not exist.";
const HELP: &str = "Commands:
  check       Validate canonical inputs and evidence without scoring
  evaluate    Score saved predictions and publish a new run
  inspect     Verify a run and disclose one selected episode
  compare     Verify two runs and publish paired changes

Use validator COMMAND --help for arguments.
--version prints the executable version.
Success means the operation completed, not that the classifier met a quality threshold.";

fn command_help(command: &str) -> Option<&'static str> {
    match command {
        "check" => Some(CHECK_HELP),
        "evaluate" => Some(EVALUATE_HELP),
        "inspect" => Some(INSPECT_HELP),
        "compare" => Some(COMPARE_HELP),
        _ => None,
    }
}

pub(crate) fn run() -> ExitCode {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let Some(command) = arguments.next() else {
        return error(Diagnostic::for_code(DiagnosticCode::Schema));
    };
    if command == "--help" || command == "-h" {
        println!("validator {}\n\n{HELP}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    if command == "--version" {
        println!("validator {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let arguments: Vec<_> = arguments.collect();
    if arguments.len() == 1
        && (arguments[0] == "--help" || arguments[0] == "-h")
        && let Some(help) = command.to_str().and_then(command_help)
    {
        println!("{help}");
        return ExitCode::SUCCESS;
    }
    match command.to_str() {
        Some("check") => match paths(arguments, false) {
            Ok((dataset, predictions, config, None)) => success(check(CheckOptions {
                dataset,
                predictions,
                config,
            })),
            Ok(_) | Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        Some("evaluate") => match paths(arguments, true) {
            Ok((dataset, predictions, config, Some(output))) => {
                success(evaluate(EvaluationOptions {
                    dataset,
                    predictions,
                    config,
                    output,
                }))
            }
            Ok(_) | Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        Some("inspect") => match inspect_paths(arguments) {
            Ok((run, episode)) => success(inspect(InspectionOptions { run, episode })),
            Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        Some("compare") => match compare_paths(arguments) {
            Ok((baseline, candidate, output, intersection)) => {
                success(compare(ComparisonOptions {
                    baseline,
                    candidate,
                    intersection,
                    output,
                }))
            }
            Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        _ => error(Diagnostic::for_code(DiagnosticCode::Schema)),
    }
}

fn compare_paths(
    arguments: Vec<OsString>,
) -> std::result::Result<(PathBuf, PathBuf, PathBuf, bool), ()> {
    let mut baseline = None;
    let mut candidate = None;
    let mut output = None;
    let mut intersection = false;
    let mut values = arguments.into_iter();
    while let Some(flag) = values.next() {
        let Some(flag) = flag.to_str() else {
            return Err(());
        };
        if flag == "--intersection" {
            if intersection {
                return Err(());
            }
            intersection = true;
            continue;
        }
        let Some(value) = values.next() else {
            return Err(());
        };
        let slot = match flag {
            "--baseline" => &mut baseline,
            "--candidate" => &mut candidate,
            "--out" => &mut output,
            _ => return Err(()),
        };
        if slot.replace(PathBuf::from(value)).is_some() {
            return Err(());
        }
    }
    match (baseline, candidate, output) {
        (Some(baseline), Some(candidate), Some(output)) => {
            Ok((baseline, candidate, output, intersection))
        }
        _ => Err(()),
    }
}

fn inspect_paths(arguments: Vec<OsString>) -> std::result::Result<(PathBuf, String), ()> {
    let mut run = None;
    let mut episode = None;
    let mut values = arguments.into_iter();
    while let Some(flag) = values.next() {
        let Some(flag) = flag.to_str() else {
            return Err(());
        };
        let Some(value) = values.next() else {
            return Err(());
        };
        let slot = match flag {
            "--run" => &mut run,
            "--episode" => &mut episode,
            _ => return Err(()),
        };
        if slot.replace(value).is_some() {
            return Err(());
        }
    }
    match (run, episode.and_then(|value| value.into_string().ok())) {
        (Some(run), Some(episode)) => Ok((PathBuf::from(run), episode)),
        _ => Err(()),
    }
}

fn paths(
    arguments: Vec<OsString>,
    needs_output: bool,
) -> std::result::Result<(PathBuf, PathBuf, PathBuf, Option<PathBuf>), ()> {
    let mut dataset = None;
    let mut predictions = None;
    let mut config = None;
    let mut output = None;
    let mut values = arguments.into_iter();
    while let Some(flag) = values.next() {
        let Some(flag) = flag.to_str() else {
            return Err(());
        };
        let Some(value) = values.next() else {
            return Err(());
        };
        let slot = match flag {
            "--dataset" => &mut dataset,
            "--predictions" => &mut predictions,
            "--config" => &mut config,
            "--out" if needs_output => &mut output,
            _ => return Err(()),
        };
        if slot.replace(PathBuf::from(value)).is_some() {
            return Err(());
        }
    }
    match (dataset, predictions, config, output) {
        (Some(dataset), Some(predictions), Some(config), output) => {
            Ok((dataset, predictions, config, output))
        }
        _ => Err(()),
    }
}

fn success(document: validator::Result<impl Serialize>) -> ExitCode {
    match document {
        Ok(document) => match serde_json::to_string(&document) {
            Ok(document) => {
                println!("{document}");
                ExitCode::SUCCESS
            }
            Err(_) => error(Diagnostic::for_code(DiagnosticCode::Invariant)),
        },
        Err(diagnostic) => error(diagnostic),
    }
}

fn error(diagnostic: Diagnostic) -> ExitCode {
    let category = diagnostic.exit_category();
    match diagnostic.to_machine_json() {
        Ok(document) => println!("{document}"),
        Err(_) => println!(
            "{{\"schema_version\":2,\"kind\":\"error\",\"status\":\"error\",\"code\":\"E_INVARIANT\",\"stage\":\"accounting\",\"affected_ids\":[],\"message\":\"internal accounting invariant failed\"}}"
        ),
    }
    exit_code(category)
}

fn exit_code(category: ExitCategory) -> ExitCode {
    ExitCode::from(category.code() as u8)
}
