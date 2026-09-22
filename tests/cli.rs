use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

fn inputs() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let (directory, golden, predictions, config, _) = inputs_with_evidence();
    (directory, golden, predictions, config)
}

fn inputs_with_evidence() -> (PathBuf, PathBuf, PathBuf, PathBuf, PathBuf) {
    let directory = std::env::temp_dir().join(format!(
        "validator-cli-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let golden = directory.join("golden.json");
    let predictions = directory.join("predictions.json");
    let config = directory.join("config.json");
    let evidence = directory.join("bound-evidence.bin");
    let golden_bytes = serde_json::to_vec(&json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B","C"]},"episodes":[{"id":"01995c20-7d00-7000-8000-000000000001","expected":{"type":"class","label":"A"},"input":"cli opaque sentinel"}]})).unwrap();
    let digest = format!("{:x}", Sha256::digest(&golden_bytes));
    fs::write(&golden, golden_bytes).unwrap();
    fs::write(&evidence, b"cli secret evidence sentinel").unwrap();
    fs::write(&predictions, serde_json::to_vec(&json!({"schema_version":2,"dataset_sha256":digest,"sources":{"source":{"kind":"classifier","model":"m","configuration":{},"evidence":["bound-evidence.bin"]}},"predictions":[{"id":"01995c20-7d00-7000-8000-000000000001","source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.2,"C":0.1000000005}}}]})).unwrap()).unwrap();
    fs::write(&config, serde_json::to_vec(&json!({"schema_version":2,"population":"cli","role":"development","decision":{"type":"as_recorded"}})).unwrap()).unwrap();
    (directory, golden, predictions, config, evidence)
}

fn command(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(arguments)
        .output()
        .unwrap()
}

fn value(output: &std::process::Output) -> Value {
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}

fn assert_one_json_document(output: &std::process::Output) {
    let mut documents = serde_json::Deserializer::from_slice(&output.stdout).into_iter::<Value>();
    assert!(documents.next().unwrap().is_ok());
    assert!(documents.next().is_none());
}

fn schema(name: &str) -> Value {
    serde_json::from_str(match name {
        "comparison" => include_str!("../schemas/v3/comparison.schema.json"),
        "receipt" => include_str!("../schemas/v2/receipt.schema.json"),
        "report" => include_str!("../schemas/v2/report.schema.json"),
        "error" => include_str!("../schemas/v2/error.schema.json"),
        _ => unreachable!(),
    })
    .unwrap()
}

fn assert_schema(name: &str, output: &std::process::Output) -> Value {
    let document = value(output);
    assert!(
        jsonschema::validator_for(&schema(name))
            .unwrap()
            .is_valid(&document),
        "{name}: {document}"
    );
    document
}

#[test]
fn shared_commands_multi_label() {
    let directory = std::env::temp_dir().join(format!(
        "validator-cli-multi-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let id = "01995c20-7d00-7000-8000-000000000001";
    let golden = directory.join("golden.json");
    let predictions = directory.join("predictions.json");
    let config = directory.join("config.json");
    let evidence = directory.join("bound-evidence.bin");
    let bytes = serde_json::to_vec(&json!({"schema_version":2,"task":{"kind":"multi_label","labels":["A","B"]},"episodes":[{"id":id,"expected":{"type":"labels","labels":["A"]},"input":{"opaque":"multi","secret":"multi secret evidence sentinel"}}]})).unwrap();
    let digest = format!("{:x}", Sha256::digest(&bytes));
    fs::write(&golden, bytes).unwrap();
    fs::write(&evidence, b"multi-label evidence").unwrap();
    fs::write(&predictions, serde_json::to_vec(&json!({"schema_version":2,"dataset_sha256":digest,"sources":{"source":{"kind":"classifier","model":"m","configuration":{},"evidence":["bound-evidence.bin"]}},"predictions":[{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.1}}}]})).unwrap()).unwrap();
    fs::write(&config, b"{\"schema_version\":2,\"population\":\"all\",\"role\":\"development\",\"decision\":{\"type\":\"as_recorded\"}}").unwrap();
    let run = directory.join("run");
    let evaluated = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        run.to_str().unwrap(),
    ]);
    assert!(evaluated.status.success());
    assert_one_json_document(&evaluated);
    assert_schema("receipt", &evaluated);
    assert!(!String::from_utf8_lossy(&evaluated.stdout).contains("multi secret evidence sentinel"));
    let report: Value =
        serde_json::from_slice(&fs::read(run.join("report.json")).unwrap()).unwrap();
    assert!(
        jsonschema::validator_for(&schema("report"))
            .unwrap()
            .is_valid(&report)
    );
    assert!(
        !serde_json::to_string(&report)
            .unwrap()
            .contains("multi secret evidence sentinel")
    );
    let before_existing_destination = tree_snapshot(&directory);
    let existing = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        run.to_str().unwrap(),
    ]);
    assert_eq!(existing.status.code(), Some(3));
    assert_schema("error", &existing);
    assert_eq!(tree_snapshot(&directory), before_existing_destination);
    assert!(!String::from_utf8_lossy(&existing.stdout).contains("multi secret evidence sentinel"));
    assert!(!String::from_utf8_lossy(&existing.stderr).contains("multi secret evidence sentinel"));
    fs::remove_dir_all(directory).unwrap();
}

fn tree_snapshot(root: &Path) -> Vec<(PathBuf, &'static str, Vec<u8>)> {
    fn visit(root: &Path, path: &Path, entries: &mut Vec<(PathBuf, &'static str, Vec<u8>)>) {
        let metadata = fs::symlink_metadata(path).unwrap();
        let kind = if metadata.file_type().is_dir() {
            "directory"
        } else if metadata.file_type().is_file() {
            "file"
        } else {
            "other"
        };
        let bytes = if kind == "file" {
            fs::read(path).unwrap()
        } else {
            Vec::new()
        };
        entries.push((path.strip_prefix(root).unwrap().to_path_buf(), kind, bytes));
        if kind == "directory" {
            let mut children = fs::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();
            children.sort();
            for child in children {
                visit(root, &child, entries);
            }
        }
    }

    let mut entries = Vec::new();
    visit(root, root, &mut entries);
    entries
}

#[test]
fn cli_evaluate() {
    let (directory, golden, predictions, config, evidence) = inputs_with_evidence();
    assert!(!directory.join("run").exists());
    let output = directory.join("run");
    let evaluate = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
    ]);
    assert!(evaluate.status.success());
    let receipt = assert_schema("receipt", &evaluate);
    assert_eq!(receipt["kind"], "evaluation");
    for path in [
        "golden.json",
        "predictions.json",
        "config.json",
        "report.json",
    ] {
        assert!(output.join(path).is_file());
    }
    assert_eq!(
        fs::read(output.join("golden.json")).unwrap(),
        fs::read(&golden).unwrap()
    );
    assert_eq!(
        fs::read(output.join("predictions.json")).unwrap(),
        fs::read(&predictions).unwrap()
    );
    assert_eq!(
        fs::read(output.join("config.json")).unwrap(),
        fs::read(&config).unwrap()
    );
    assert_eq!(
        fs::read(output.join("evidence/0.bin")).unwrap(),
        fs::read(&evidence).unwrap()
    );
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("report.json")).unwrap()).unwrap();
    assert_eq!(report["integrity"]["normalized_count"], 1);
    let expected_sum_error = (0.7_f64 + 0.2 + 0.1000000005 - 1.0).abs();
    let maximum_sum_error = report["integrity"]["maximum_sum_error"].as_f64().unwrap();
    assert!(maximum_sum_error > 0.0);
    assert!(
        (maximum_sum_error - expected_sum_error).abs() <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    let before_existing_destination = tree_snapshot(&directory);
    let existing = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
    ]);
    assert_eq!(existing.status.code(), Some(3));
    assert_schema("error", &existing);
    assert_eq!(tree_snapshot(&directory), before_existing_destination);
    assert!(output.is_dir());
    let failed_after_admission = String::from_utf8_lossy(&existing.stdout);
    assert!(!failed_after_admission.contains("cli secret evidence sentinel"));
    assert!(!String::from_utf8_lossy(&existing.stderr).contains("cli secret evidence sentinel"));
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn cli_help_version_errors() {
    for flag in ["--help", "--version"] {
        let output = command(&[flag]);
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
    let unknown = command(&["compare"]);
    assert_eq!(unknown.status.code(), Some(2));
    let error = assert_schema("error", &unknown);
    assert_eq!(error["kind"], "error");
    assert!(!serde_json::to_string(&error).unwrap().contains("sentinel"));
    for arguments in [
        ["evaluate", "--bogus", "x"].as_slice(),
        ["evaluate", "--dataset"].as_slice(),
        ["evaluate", "--dataset", "a", "--dataset", "b"].as_slice(),
        ["evaluate", "--out", "a", "--out", "b"].as_slice(),
    ] {
        let output = command(arguments);
        assert_eq!(output.status.code(), Some(2));
        assert_schema("error", &output);
    }
    let missing = command(&[
        "evaluate",
        "--dataset",
        "/definitely/missing",
        "--predictions",
        "/definitely/missing",
        "--config",
        "/definitely/missing",
        "--out",
        "/definitely/missing-output",
    ]);
    assert_eq!(missing.status.code(), Some(3));
    assert_schema("error", &missing);

    let (directory, golden, predictions, config) = inputs();
    let mut invalid: Value = serde_json::from_slice(&fs::read(&predictions).unwrap()).unwrap();
    invalid["predictions"][0]["outcome"]["label"] = json!("D");
    fs::write(&predictions, serde_json::to_vec(&invalid).unwrap()).unwrap();
    let output = directory.join("invalid-run");
    let invalid = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
    ]);
    assert_eq!(invalid.status.code(), Some(2));
    assert_schema("error", &invalid);
    assert!(!output.exists());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn receipt_hash_matches_report() {
    let (directory, golden, predictions, config) = inputs();
    let output = directory.join("run");
    let evaluated = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
    ]);
    assert!(evaluated.status.success());
    let receipt = assert_schema("receipt", &evaluated);
    let bytes = fs::read(receipt["result_path"].as_str().unwrap()).unwrap();
    assert_eq!(
        receipt["result_sha256"],
        format!("{:x}", Sha256::digest(bytes))
    );
    fs::remove_dir_all(directory).unwrap();
}

struct SteelThreadInputs {
    directory: PathBuf,
    golden: PathBuf,
    baseline: PathBuf,
    candidate: PathBuf,
    config: PathBuf,
    source_a_bytes: Vec<u8>,
    source_b_bytes: Vec<u8>,
}

fn steel_thread_inputs() -> SteelThreadInputs {
    let directory = std::env::temp_dir().join(format!(
        "validator-steel-thread-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    let input = directory.join("input");
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/steel-thread");
    fs::create_dir_all(input.join("source-a")).unwrap();
    fs::create_dir_all(directory.join("source-b")).unwrap();
    for name in [
        "golden.json",
        "baseline.json",
        "candidate.json",
        "config.json",
    ] {
        fs::copy(fixtures.join(name), input.join(name)).unwrap();
    }
    let source_a_bytes = fs::read(fixtures.join("source-a/evidence.bin")).unwrap();
    let source_b_bytes = fs::read(fixtures.join("source-b/evidence.bin")).unwrap();
    fs::write(input.join("source-a/evidence.bin"), &source_a_bytes).unwrap();
    fs::write(directory.join("source-b/evidence.bin"), &source_b_bytes).unwrap();
    SteelThreadInputs {
        directory,
        golden: input.join("golden.json"),
        baseline: input.join("baseline.json"),
        candidate: input.join("candidate.json"),
        config: input.join("config.json"),
        source_a_bytes,
        source_b_bytes,
    }
}

fn as_string(path: &Path) -> &str {
    path.to_str().unwrap()
}

fn evaluate_steel_thread(
    inputs: &SteelThreadInputs,
    predictions: &Path,
    output: &Path,
) -> std::process::Output {
    command(&[
        "evaluate",
        "--dataset",
        as_string(&inputs.golden),
        "--predictions",
        as_string(predictions),
        "--config",
        as_string(&inputs.config),
        "--out",
        as_string(output),
    ])
}

fn assert_error(output: &std::process::Output, exit: i32, code: &str, stage: &str) {
    assert_eq!(output.status.code(), Some(exit));
    let error = assert_schema("error", output);
    assert_eq!(error["code"], code);
    assert_eq!(error["stage"], stage);
}

fn report_from(receipt: &Value) -> Value {
    let bytes = fs::read(receipt["result_path"].as_str().unwrap()).unwrap();
    assert_eq!(
        receipt["result_sha256"],
        format!("{:x}", Sha256::digest(&bytes))
    );
    let report = serde_json::from_slice(&bytes).unwrap();
    assert!(
        jsonschema::validator_for(&schema("report"))
            .unwrap()
            .is_valid(&report)
    );
    report
}

fn ids(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|id| id.as_str().unwrap().to_owned())
        .collect()
}

fn assert_fraction(metric: &Value, fraction: &Value) {
    let numerator = fraction["numerator"].as_u64().unwrap();
    let denominator = fraction["denominator"].as_u64().unwrap();
    assert_eq!(metric["numerator"], numerator);
    assert_eq!(metric["denominator"], denominator);
    assert!(
        (metric["value"].as_f64().unwrap() - numerator as f64 / denominator as f64).abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
}

#[test]
fn steel_thread_end_to_end() {
    let inputs = steel_thread_inputs();
    let expected: Value = serde_json::from_slice(
        &fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/steel-thread/expected.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let source_secret_a = String::from_utf8(inputs.source_a_bytes.clone()).unwrap();
    let source_secret_b = String::from_utf8(inputs.source_b_bytes.clone()).unwrap();

    let baseline_run = inputs.directory.join("baseline-run");
    let candidate_run = inputs.directory.join("candidate-run");
    let baseline_receipt = evaluate_steel_thread(&inputs, &inputs.baseline, &baseline_run);
    assert!(baseline_receipt.status.success());
    let baseline_receipt = assert_schema("receipt", &baseline_receipt);
    let baseline_report = report_from(&baseline_receipt);
    assert_eq!(
        ids(&baseline_report["population"]["selected_ids"]),
        ids(&expected["ids"])
    );
    assert_eq!(baseline_report["integrity"]["source_count"], 2);
    assert_eq!(baseline_report["integrity"]["signal_availability"], "both");
    let candidate_receipt = evaluate_steel_thread(&inputs, &inputs.candidate, &candidate_run);
    assert!(candidate_receipt.status.success());
    let candidate_receipt = assert_schema("receipt", &candidate_receipt);
    let candidate_report = report_from(&candidate_receipt);
    for report in [&baseline_report, &candidate_report] {
        assert_eq!(report["artifacts"].as_array().unwrap().len(), 5);
        assert_eq!(report["integrity"]["selected_count"], 4);
    }
    assert_eq!(
        fs::read(baseline_run.join("evidence/0.bin")).unwrap(),
        inputs.source_a_bytes
    );
    assert_eq!(
        fs::read(baseline_run.join("evidence/1.bin")).unwrap(),
        inputs.source_b_bytes
    );
    assert_ne!(
        fs::read(baseline_run.join("evidence/0.bin")).unwrap(),
        fs::read(baseline_run.join("evidence/1.bin")).unwrap()
    );

    let existing_before = tree_snapshot(&baseline_run);
    let existing_evaluation = evaluate_steel_thread(&inputs, &inputs.baseline, &baseline_run);
    assert_error(&existing_evaluation, 3, "E_OUTPUT_EXISTS", "filesystem");
    assert_eq!(tree_snapshot(&baseline_run), existing_before);
    assert!(!inputs.directory.join(".baseline-run.validator-0").exists());

    let duplicate = inputs.directory.join("duplicate.json");
    let baseline_text = fs::read_to_string(&inputs.baseline).unwrap();
    fs::write(
        &duplicate,
        baseline_text.replacen(
            "\"schema_version\": 2,",
            "\"schema_version\": 2, \"schema_version\": 2,",
            1,
        ),
    )
    .unwrap();
    let duplicate_output = inputs.directory.join("duplicate-output");
    let duplicate_result = evaluate_steel_thread(&inputs, &duplicate, &duplicate_output);
    assert_error(&duplicate_result, 2, "E_SCHEMA", "schema");
    assert!(!duplicate_output.exists());

    let invalid_probability = inputs.directory.join("invalid-probability.json");
    fs::write(
        &invalid_probability,
        baseline_text.replacen("\"C\": 0.1", "\"C\": 0.09", 1),
    )
    .unwrap();
    let invalid_probability_output = inputs.directory.join("invalid-probability-output");
    let invalid_probability_result =
        evaluate_steel_thread(&inputs, &invalid_probability, &invalid_probability_output);
    assert_error(
        &invalid_probability_result,
        2,
        "E_PROBABILITY",
        "validation",
    );
    assert!(!invalid_probability_output.exists());

    let final_results = &baseline_report["final"];
    for (metric, expected_metric) in [
        ("accuracy", "accuracy"),
        ("wrong_class_rate", "wrong_class_rate"),
        ("abstention_rate", "abstention_rate"),
        ("coverage", "coverage"),
        ("selective_accuracy", "selective_accuracy"),
        ("selective_risk", "selective_risk"),
    ] {
        assert_fraction(
            &final_results[metric],
            &expected["baseline"][expected_metric],
        );
    }
    assert_eq!(
        final_results["correct"],
        expected["baseline"]["counts"]["D"]
    );
    assert_eq!(final_results["wrong"], expected["baseline"]["counts"]["E"]);
    assert_eq!(
        final_results["abstained"],
        expected["baseline"]["counts"]["U"]
    );
    assert!(
        (final_results["macro_f1"]["metric"]["value"]
            .as_f64()
            .unwrap()
            - expected["baseline"]["macro_f1"]["numerator"]
                .as_f64()
                .unwrap()
                / expected["baseline"]["macro_f1"]["denominator"]
                    .as_f64()
                    .unwrap())
        .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    for (actual, expected_coverage) in final_results["classes"]
        .as_array()
        .unwrap()
        .iter()
        .zip(expected["baseline"]["class_coverages"].as_array().unwrap())
    {
        assert_fraction(&actual["coverage"], expected_coverage);
    }
    assert!(
        (baseline_report["probability"]["brier_score"]["value"]
            .as_f64()
            .unwrap()
            - expected["probability"]["brier"]["numerator"]
                .as_f64()
                .unwrap()
                / expected["probability"]["brier"]["denominator"]
                    .as_f64()
                    .unwrap())
        .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert_fraction(
        &baseline_report["probability"]["argmax_accuracy"],
        &expected["probability"]["argmax_accuracy"],
    );
    assert!(
        (baseline_report["probability"]["log_loss"]["value"]
            .as_f64()
            .unwrap()
            - expected["probability"]["log_loss"]["decimal"]
                .as_f64()
                .unwrap())
        .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert_eq!(
        ids(&baseline_report["signals"]["maximum_probability"]["included_ids"]),
        ids(&expected["probability"]["included_ids"])
    );
    assert_eq!(
        ids(&baseline_report["signals"]["confidence"]["included_ids"]),
        ids(&expected["confidence"]["included_ids"])
    );
    assert_eq!(
        ids(&baseline_report["signals"]["confidence"]["excluded_ids"]),
        ids(&expected["confidence"]["excluded_ids"])
    );

    for document in [
        serde_json::to_string(&baseline_receipt).unwrap(),
        serde_json::to_string(&baseline_report).unwrap(),
        serde_json::to_string(&candidate_report).unwrap(),
    ] {
        assert!(!document.contains("9007199254740993"));
        assert!(!document.contains(&source_secret_a));
        assert!(!document.contains(&source_secret_b));
    }
    fs::remove_dir_all(inputs.directory).unwrap();
}

#[test]
fn cli_stdout_schema_matrix() {
    let (directory, golden, predictions, config, _) = inputs_with_evidence();
    let baseline = directory.join("baseline");
    let evaluate = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        baseline.to_str().unwrap(),
    ]);
    assert!(evaluate.status.success());
    assert_one_json_document(&evaluate);
    let receipt = assert_schema("receipt", &evaluate);
    let report: Value =
        serde_json::from_slice(&fs::read(receipt["result_path"].as_str().unwrap()).unwrap())
            .unwrap();
    assert!(
        jsonschema::validator_for(&schema("report"))
            .unwrap()
            .is_valid(&report)
    );

    let candidate = directory.join("candidate");
    let candidate_receipt = command(&[
        "evaluate",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
        "--out",
        candidate.to_str().unwrap(),
    ]);
    assert!(candidate_receipt.status.success());
    assert_one_json_document(&candidate_receipt);
    assert_schema("receipt", &candidate_receipt);

    let compare = command(&[
        "compare",
        baseline.join("report.json").to_str().unwrap(),
        candidate.join("report.json").to_str().unwrap(),
    ]);
    assert!(compare.status.success());
    assert_one_json_document(&compare);
    assert_schema("comparison", &compare);

    for arguments in [
        ["compare"].as_slice(),
        [
            "evaluate",
            "--dataset",
            golden.to_str().unwrap(),
            "--predictions",
            predictions.to_str().unwrap(),
            "--config",
            config.to_str().unwrap(),
            "--out",
            baseline.to_str().unwrap(),
        ]
        .as_slice(),
    ] {
        let error = command(arguments);
        assert!(!error.status.success());
        assert_one_json_document(&error);
        assert_schema("error", &error);
    }
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn publication_and_privacy_matrix() {
    shared_commands_multi_label();
    steel_thread_end_to_end();
}

#[test]
fn offline_cli_contract() {
    cli_evaluate();
    shared_commands_multi_label();
}
