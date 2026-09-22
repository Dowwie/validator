use std::{env, ffi::OsString, path::PathBuf, process::ExitCode};

use serde::Serialize;
use validator::{
    ComparisonOptions, Diagnostic, DiagnosticCode, EvaluationOptions, ExitCategory, compare,
    evaluate,
};

const EVALUATE_HELP: &str = "validator evaluate --dataset PATH --predictions PATH --config PATH --out NEW_RUN_DIR\nEvaluate saved predictions and publish a new run. --out is required and must not exist.";
const COMPARE_HELP: &str = "validator compare REPORT [REPORT ...]\nAggregate at least two saved evaluation reports in argument order.";
const HELP: &str = "Commands:
  evaluate    Score saved predictions and publish a new run
  compare     Report changes across saved evaluation reports

Use validator COMMAND --help for arguments.
--version prints the executable version.
Success means the operation completed, not that the classifier met a quality threshold.";

fn command_help(command: &str) -> Option<&'static str> {
    match command {
        "evaluate" => Some(EVALUATE_HELP),
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
        Some("evaluate") => match evaluation_paths(arguments) {
            Ok((dataset, predictions, config, output)) => success(evaluate(EvaluationOptions {
                dataset,
                predictions,
                config,
                output,
            })),
            Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        Some("compare") => match compare_paths(arguments) {
            Ok(reports) => success(compare(ComparisonOptions { reports })),
            Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        _ => error(Diagnostic::for_code(DiagnosticCode::Schema)),
    }
}

fn compare_paths(arguments: Vec<OsString>) -> std::result::Result<Vec<PathBuf>, ()> {
    let mut paths = Vec::new();
    let mut options = true;
    for argument in arguments {
        if options && argument == "--" {
            options = false;
            continue;
        }
        if options && argument.to_string_lossy().starts_with('-') {
            return Err(());
        }
        paths.push(PathBuf::from(argument));
    }
    if paths.len() < 2 {
        return Err(());
    }
    Ok(paths)
}

fn evaluation_paths(
    arguments: Vec<OsString>,
) -> std::result::Result<(PathBuf, PathBuf, PathBuf, PathBuf), ()> {
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
            "--out" => &mut output,
            _ => return Err(()),
        };
        if slot.replace(PathBuf::from(value)).is_some() {
            return Err(());
        }
    }
    match (dataset, predictions, config, output) {
        (Some(dataset), Some(predictions), Some(config), Some(output)) => {
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
