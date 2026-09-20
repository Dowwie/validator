use std::{env, ffi::OsString, path::PathBuf, process::ExitCode};

use serde::Serialize;
use validator::{
    CheckOptions, ComparisonOptions, Diagnostic, DiagnosticCode, EvaluationOptions, ExitCategory,
    InspectionOptions, check, compare, evaluate, inspect,
};

pub(crate) fn run() -> ExitCode {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let Some(command) = arguments.next() else {
        return error(Diagnostic::for_code(DiagnosticCode::Schema));
    };
    if command == "--help" || command == "-h" {
        println!(
            "validator check|evaluate --dataset PATH --predictions PATH --config PATH [--out PATH]; validator inspect --run RUN_DIR --episode UUID"
        );
        return ExitCode::SUCCESS;
    }
    if command == "--version" {
        println!("validator {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    match command.to_str() {
        Some("check") => match paths(arguments.collect(), false) {
            Ok((dataset, predictions, config, None)) => success(check(CheckOptions {
                dataset,
                predictions,
                config,
            })),
            Ok(_) | Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        Some("evaluate") => match paths(arguments.collect(), true) {
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
        Some("inspect") => match inspect_paths(arguments.collect()) {
            Ok((run, episode)) => success(inspect(InspectionOptions { run, episode })),
            Err(()) => error(Diagnostic::for_code(DiagnosticCode::Schema)),
        },
        Some("compare") => match compare_paths(arguments.collect()) {
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
