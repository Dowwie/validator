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
        "check" => include_str!("../schemas/v2/check.schema.json"),
        "comparison" => include_str!("../schemas/v2/comparison.schema.json"),
        "inspection" => include_str!("../schemas/v2/inspection.schema.json"),
        "receipt" => include_str!("../schemas/v2/receipt.schema.json"),
        "report" => include_str!("../schemas/v2/report.schema.json"),
        "error" => include_str!("../schemas/v2/error.schema.json"),
        _ => unreachable!(),
    })
    .unwrap()
}

#[test]
fn cli_compare_receipt() {
    let directory = std::env::temp_dir().join(format!(
        "validator-cli-comparison-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let golden = directory.join("golden.json");
    let config = directory.join("config.json");
    let evidence = directory.join("evidence.bin");
    let ids = (1..=5)
        .map(|index| format!("01995c20-7d00-7000-8000-{index:012}"))
        .collect::<Vec<_>>();
    let expected = ["A", "B", "A", "B", "A"];
    let golden_bytes = serde_json::to_vec(&json!({
        "schema_version": 2,
        "task": {"kind": "single_label", "labels": ["A", "B"]},
        "episodes": ids.iter().zip(expected).map(|(id, label)| json!({"id": id, "expected": {"type": "class", "label": label}, "input": null})).collect::<Vec<_>>()
    }))
    .unwrap();
    let digest = format!("{:x}", Sha256::digest(&golden_bytes));
    fs::write(&golden, golden_bytes).unwrap();
    fs::write(&config, b"{\"schema_version\":2,\"population\":\"comparison\",\"role\":\"development\",\"decision\":{\"type\":\"as_recorded\"}}").unwrap();
    fs::write(&evidence, b"comparison evidence").unwrap();
    let predictions = |outcomes: [&str; 5],
                       configuration: Value,
                       description: &str,
                       version: &str| {
        json!({
            "schema_version": 2,
            "dataset_sha256": digest,
            "sources": {"source": {"kind": "classifier", "model": "m", "configuration": configuration, "evidence": ["evidence.bin"], "observation_definitions": {"score": {"kind": "scalar", "description": description}}, "preparation": {"method": "prepare", "version": version, "configuration": {"mode": "fixed"}, "evidence_indices": [0]}}},
            "predictions": ids.iter().zip(outcomes).map(|(id, label)| json!({"id": id, "source_id": "source", "outcome": {"type": "class", "label": label}, "observations": {"score": {"kind": "scalar", "value": 0.5}}, "probabilities": {"kind": "categorical", "values": {"A": 0.7, "B": 0.3}}})).collect::<Vec<_>>()
        })
    };
    let baseline_predictions = directory.join("baseline.json");
    let candidate_predictions = directory.join("candidate.json");
    fs::write(
        &baseline_predictions,
        serde_json::to_vec(&predictions(
            ["A", "A", "A", "B", "B"],
            json!({"revision": 1}),
            "baseline observation",
            "1",
        ))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        &candidate_predictions,
        serde_json::to_vec(&predictions(
            ["A", "B", "B", "B", "B"],
            json!({"revision": 2}),
            "candidate observation",
            "2",
        ))
        .unwrap(),
    )
    .unwrap();
    let baseline = directory.join("baseline-run");
    let candidate = directory.join("candidate-run");
    for (predictions, output) in [
        (&baseline_predictions, &baseline),
        (&candidate_predictions, &candidate),
    ] {
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
        assert!(
            evaluated.status.success(),
            "{}",
            String::from_utf8_lossy(&evaluated.stdout)
        );
    }
    let output = directory.join("comparison");
    let compared = command(&[
        "compare",
        "--baseline",
        baseline.to_str().unwrap(),
        "--candidate",
        candidate.to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
    ]);
    assert!(compared.status.success());
    let receipt = assert_schema("receipt", &compared);
    assert_eq!(receipt["kind"], "comparison");
    assert!(receipt["result_path"].as_str().unwrap().starts_with('/'));
    let comparison =
        serde_json::from_slice(&fs::read(output.join("comparison.json")).unwrap()).unwrap();
    assert!(
        jsonschema::validator_for(&schema("comparison"))
            .unwrap()
            .is_valid(&comparison)
    );
    assert_eq!(
        receipt["result_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(output.join("comparison.json")).unwrap())
        )
    );
    assert_eq!(comparison["transitions"]["recovered"]["count"], 1);
    assert_eq!(comparison["transitions"]["regressed"]["count"], 1);
    assert_eq!(comparison["transitions"]["neither_correct"]["count"], 1);
    assert!(
        comparison["configuration_differences"]
            .as_array()
            .unwrap()
            .iter()
            .any(|difference| difference["field_path"] == "sources.source.configuration.revision")
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn cli_intersection() {
    let directory = std::env::temp_dir().join(format!(
        "validator-cli-intersection-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let ids = (1..=3)
        .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
        .collect::<Vec<_>>();
    let golden = directory.join("golden.json");
    let golden_bytes = serde_json::to_vec(&json!({
        "schema_version":2,
        "task":{"kind":"single_label","labels":["A","B"]},
        "episodes":[
            {"id":ids[0],"expected":{"type":"class","label":"A"},"input":null},
            {"id":ids[1],"expected":{"type":"class","label":"B"},"input":null},
            {"id":ids[2],"expected":{"type":"class","label":"A"},"input":null}
        ]
    }))
    .unwrap();
    let digest = format!("{:x}", Sha256::digest(&golden_bytes));
    fs::write(&golden, golden_bytes).unwrap();
    let write_run = |name: &str, selected: &[String], outcomes: &[&str]| {
        let predictions = directory.join(format!("{name}.json"));
        let config = directory.join(format!("{name}-config.json"));
        fs::write(
            &predictions,
            serde_json::to_vec(&json!({
                "schema_version":2,"dataset_sha256":digest,
                "sources":{"source":{"kind":"classifier","model":"m","configuration":{}}},
                "predictions":selected.iter().zip(outcomes).map(|(id, label)| json!({"id":id,"source_id":"source","outcome":{"type":"class","label":label},"probabilities":{"kind":"categorical","values":{"A":0.8,"B":0.2}}})).collect::<Vec<_>>()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            &config,
            serde_json::to_vec(&json!({"schema_version":2,"population":"cli intersection","role":"development","episode_ids":selected,"decision":{"type":"as_recorded"}})).unwrap(),
        )
        .unwrap();
        let output = directory.join(format!("{name}-run"));
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
        output
    };
    let baseline = write_run("baseline", &ids[..2], &["A", "A"]);
    let candidate = write_run("candidate", &ids[1..], &["B", "B"]);
    let output = directory.join("comparison");
    let default = command(&[
        "compare",
        "--baseline",
        baseline.to_str().unwrap(),
        "--candidate",
        candidate.to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
    ]);
    assert_error(&default, 2, "E_COMPARISON", "comparison");
    assert!(!output.exists());
    let compared = command(&[
        "compare",
        "--baseline",
        baseline.to_str().unwrap(),
        "--candidate",
        candidate.to_str().unwrap(),
        "--intersection",
        "--out",
        output.to_str().unwrap(),
    ]);
    assert!(compared.status.success());
    let receipt = assert_schema("receipt", &compared);
    let bytes = fs::read(output.join("comparison.json")).unwrap();
    let comparison: Value = serde_json::from_slice(&bytes).unwrap();
    assert!(
        jsonschema::validator_for(&schema("comparison"))
            .unwrap()
            .is_valid(&comparison)
    );
    assert_eq!(
        receipt["result_sha256"],
        format!("{:x}", Sha256::digest(&bytes))
    );
    assert_eq!(comparison["scope"], "intersection");
    assert_eq!(comparison["population"]["compared_ids"], json!([ids[1]]));
    assert_eq!(
        comparison["population"]["baseline_excluded_ids"],
        json!([ids[0]])
    );
    assert_eq!(
        comparison["population"]["candidate_excluded_ids"],
        json!([ids[2]])
    );
    let assert_equal_intersection = |baseline: &PathBuf,
                                     candidate: &PathBuf,
                                     output: &PathBuf,
                                     expected_ids: Value,
                                     expected_count: u64| {
        let compared = command(&[
            "compare",
            "--baseline",
            baseline.to_str().unwrap(),
            "--candidate",
            candidate.to_str().unwrap(),
            "--intersection",
            "--out",
            output.to_str().unwrap(),
        ]);
        assert!(compared.status.success());
        let receipt = assert_schema("receipt", &compared);
        let bytes = fs::read(output.join("comparison.json")).unwrap();
        assert_eq!(
            receipt["result_sha256"],
            format!("{:x}", Sha256::digest(&bytes))
        );
        let comparison: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(comparison["scope"], "intersection");
        assert_eq!(comparison["population"]["compared_ids"], expected_ids);
        assert_eq!(comparison["population"]["compared_count"], expected_count);
        assert_eq!(comparison["population"]["baseline_excluded_ids"], json!([]));
        assert_eq!(
            comparison["population"]["candidate_excluded_ids"],
            json!([])
        );
        assert_eq!(comparison["population"]["baseline_excluded_count"], 0);
        assert_eq!(comparison["population"]["candidate_excluded_count"], 0);
        assert!(
            jsonschema::validator_for(&schema("comparison"))
                .unwrap()
                .is_valid(&comparison)
        );
    };
    let equal_baseline = write_run("equal-baseline", &ids[..1], &["A"]);
    let equal_candidate = write_run("equal-candidate", &ids[..1], &["A"]);
    assert_equal_intersection(
        &equal_baseline,
        &equal_candidate,
        &directory.join("equal-comparison"),
        json!([ids[0]]),
        1,
    );
    let empty_baseline = write_run("empty-baseline", &ids[..0], &[]);
    let empty_candidate = write_run("empty-candidate", &ids[..0], &[]);
    assert_equal_intersection(
        &empty_baseline,
        &empty_candidate,
        &directory.join("empty-comparison"),
        json!([]),
        0,
    );
    let multi_directory = directory.join("multi");
    fs::create_dir(&multi_directory).unwrap();
    let multi_id = "01995c20-7d00-7000-8000-000000000010";
    let multi_golden = multi_directory.join("golden.json");
    let multi_golden_bytes = serde_json::to_vec(&json!({
        "schema_version":2,
        "task":{"kind":"multi_label","labels":["A","B"]},
        "episodes":[{"id":multi_id,"expected":{"type":"labels","labels":["A"]},"input":null}]
    }))
    .unwrap();
    let multi_digest = format!("{:x}", Sha256::digest(&multi_golden_bytes));
    fs::write(&multi_golden, multi_golden_bytes).unwrap();
    let write_multi_run = |name: &str, selected: bool| {
        let predictions = multi_directory.join(format!("{name}.json"));
        let config = multi_directory.join(format!("{name}-config.json"));
        let selected_ids = selected.then_some(vec![multi_id]);
        let rows = selected.then_some(vec![json!({
            "id":multi_id,
            "source_id":"source",
            "outcome":{"type":"labels","labels":["A"]},
            "probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.2}}
        })]);
        fs::write(
            &predictions,
            serde_json::to_vec(&json!({
                "schema_version":2,
                "dataset_sha256":multi_digest,
                "sources":{"source":{"kind":"classifier","model":"m","configuration":{}}},
                "predictions":rows.unwrap_or_default(),
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            &config,
            serde_json::to_vec(&json!({
                "schema_version":2,
                "population":"cli multi intersection",
                "role":"development",
                "episode_ids":selected_ids.unwrap_or_default(),
                "decision":{"type":"as_recorded"}
            }))
            .unwrap(),
        )
        .unwrap();
        let output = multi_directory.join(format!("{name}-run"));
        let evaluated = command(&[
            "evaluate",
            "--dataset",
            multi_golden.to_str().unwrap(),
            "--predictions",
            predictions.to_str().unwrap(),
            "--config",
            config.to_str().unwrap(),
            "--out",
            output.to_str().unwrap(),
        ]);
        assert!(evaluated.status.success());
        output
    };
    let multi_baseline = write_multi_run("equal-baseline", true);
    let multi_candidate = write_multi_run("equal-candidate", true);
    assert_equal_intersection(
        &multi_baseline,
        &multi_candidate,
        &multi_directory.join("equal-comparison"),
        json!([multi_id]),
        1,
    );
    let multi_empty_baseline = write_multi_run("empty-baseline", false);
    let multi_empty_candidate = write_multi_run("empty-candidate", false);
    assert_equal_intersection(
        &multi_empty_baseline,
        &multi_empty_candidate,
        &multi_directory.join("empty-comparison"),
        json!([]),
        0,
    );
    let immutable = command(&[
        "compare",
        "--baseline",
        baseline.to_str().unwrap(),
        "--candidate",
        candidate.to_str().unwrap(),
        "--intersection",
        "--out",
        output.to_str().unwrap(),
    ]);
    assert_error(&immutable, 3, "E_OUTPUT_EXISTS", "filesystem");
    for arguments in [
        vec![
            "compare",
            "--baseline",
            baseline.to_str().unwrap(),
            "--candidate",
            candidate.to_str().unwrap(),
            "--intersection",
            "--intersection",
            "--out",
            directory.join("duplicate").to_str().unwrap(),
        ],
        vec![
            "compare",
            "--baseline",
            baseline.to_str().unwrap(),
            "--candidate",
            candidate.to_str().unwrap(),
            "--unknown",
            "value",
            "--out",
            directory.join("unknown").to_str().unwrap(),
        ],
        vec![
            "compare",
            "--baseline",
            baseline.to_str().unwrap(),
            "--candidate",
            candidate.to_str().unwrap(),
            "--intersection",
            "value",
            "--out",
            directory.join("missing").to_str().unwrap(),
        ],
    ] {
        let result = command(&arguments);
        assert_error(&result, 2, "E_SCHEMA", "schema");
    }
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn cli_multi_label_compare_receipt() {
    let directory = std::env::temp_dir().join(format!(
        "validator-cli-multi-comparison-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let golden = directory.join("golden.json");
    let config = directory.join("config.json");
    let id = "01995c20-7d00-7000-8000-000000000001";
    let golden_bytes = serde_json::to_vec(&json!({
        "schema_version": 2,
        "task": {"kind": "multi_label", "labels": ["A", "B"]},
        "episodes": [{"id": id, "expected": {"type": "labels", "labels": ["A", "B"]}, "input": null}]
    }))
    .unwrap();
    let digest = format!("{:x}", Sha256::digest(&golden_bytes));
    fs::write(&golden, golden_bytes).unwrap();
    fs::write(&config, b"{\"schema_version\":2,\"population\":\"cli\",\"role\":\"development\",\"decision\":{\"type\":\"as_recorded\"}}").unwrap();
    for (name, labels) in [("baseline", json!(["A"])), ("candidate", json!(["B"]))] {
        let predictions = directory.join(format!("{name}.json"));
        fs::write(&predictions, serde_json::to_vec(&json!({
            "schema_version": 2, "dataset_sha256": digest,
            "sources": {"source": {"kind": "classifier", "model": "m", "configuration": {}}},
            "predictions": [{"id": id, "source_id": "source", "outcome": {"type": "labels", "labels": labels}}]
        })).unwrap()).unwrap();
        let output = directory.join(format!("{name}-run"));
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
    }
    let output = directory.join("comparison");
    let compared = command(&[
        "compare",
        "--baseline",
        directory.join("baseline-run").to_str().unwrap(),
        "--candidate",
        directory.join("candidate-run").to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
    ]);
    assert!(compared.status.success());
    let receipt = assert_schema("receipt", &compared);
    let comparison =
        serde_json::from_slice::<Value>(&fs::read(output.join("comparison.json")).unwrap())
            .unwrap();
    assert!(
        jsonschema::validator_for(&schema("comparison"))
            .unwrap()
            .is_valid(&comparison)
    );
    assert_eq!(comparison["task"]["kind"], "multi_label");
    assert_eq!(
        receipt["result_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(output.join("comparison.json")).unwrap())
        )
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn relocated_run_inspection() {
    let (directory, golden, predictions, config, _evidence) = inputs_with_evidence();
    let opaque = br#"{"schema_version":2,"task":{"kind":"single_label","labels":["A","B","C"]},"episodes":[{"id":"01995c20-7d00-7000-8000-000000000001","expected":{"type":"class","label":"A"},"input":{"large":9007199254740993,"huge":1e400,"null":null,"$serde_json::private::Number":"literal"}}]}"#;
    fs::write(&golden, opaque).unwrap();
    let digest = format!("{:x}", Sha256::digest(opaque));
    let mut prediction: Value = serde_json::from_slice(&fs::read(&predictions).unwrap()).unwrap();
    prediction["dataset_sha256"] = json!(digest);
    fs::write(&predictions, serde_json::to_vec(&prediction).unwrap()).unwrap();
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
    let relocated = directory.join("relocated");
    fs::rename(&run, &relocated).unwrap();
    fs::remove_file(golden).unwrap();
    fs::remove_file(predictions).unwrap();
    fs::remove_file(config).unwrap();
    let inspected = command(&[
        "inspect",
        "--run",
        relocated.to_str().unwrap(),
        "--episode",
        "01995c20-7d00-7000-8000-000000000001",
    ]);
    assert!(inspected.status.success());
    let inspection = assert_schema("inspection", &inspected);
    assert_eq!(inspection["kind"], "inspection");
    let stdout = String::from_utf8(inspected.stdout).unwrap();
    for token in [
        "9007199254740993",
        "1e400",
        "\"null\":null",
        "\"$serde_json::private::Number\":\"literal\"",
    ] {
        assert!(stdout.contains(token), "missing {token}");
    }
    let unknown = command(&[
        "inspect",
        "--run",
        relocated.to_str().unwrap(),
        "--episode",
        "01995c20-7d00-7000-8000-000000000002",
    ]);
    assert_eq!(unknown.status.code(), Some(2));
    let error = assert_schema("error", &unknown);
    assert!(
        !serde_json::to_string(&error)
            .unwrap()
            .contains("9007199254740993")
    );
    fs::remove_dir_all(directory).unwrap();
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
    let check = command(&[
        "check",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
    ]);
    assert!(check.status.success());
    assert_one_json_document(&check);
    assert_schema("check", &check);
    assert!(!String::from_utf8_lossy(&check.stdout).contains("multi secret evidence sentinel"));
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
    let relocated = directory.join("relocated");
    fs::rename(&run, &relocated).unwrap();
    fs::remove_file(golden).unwrap();
    fs::remove_file(predictions).unwrap();
    fs::remove_file(config).unwrap();
    fs::remove_file(evidence).unwrap();
    let inspected = command(&[
        "inspect",
        "--run",
        relocated.to_str().unwrap(),
        "--episode",
        id,
    ]);
    assert!(inspected.status.success());
    assert_one_json_document(&inspected);
    let document = assert_schema("inspection", &inspected);
    assert_eq!(
        document["input"],
        json!({"opaque":"multi","secret":"multi secret evidence sentinel"})
    );
    fs::write(relocated.join("evidence/0.bin"), b"changed evidence").unwrap();
    let tampered = command(&[
        "inspect",
        "--run",
        relocated.to_str().unwrap(),
        "--episode",
        id,
    ]);
    assert_eq!(tampered.status.code(), Some(2));
    let error = assert_schema("error", &tampered);
    assert_eq!(error["code"], "E_PROVENANCE");
    assert_eq!(error["stage"], "replay");
    assert!(
        !serde_json::to_string(&error)
            .unwrap()
            .contains("multi secret evidence sentinel")
    );
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
fn cli_check_evaluate() {
    let (directory, golden, predictions, config, evidence) = inputs_with_evidence();
    let check = command(&[
        "check",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
    ]);
    assert!(check.status.success());
    let check = assert_schema("check", &check);
    assert_eq!(check["kind"], "check");
    assert_eq!(check["integrity"]["normalized_count"], 1);
    let expected_sum_error = (0.7_f64 + 0.2 + 0.1000000005 - 1.0).abs();
    let maximum_sum_error = check["integrity"]["maximum_sum_error"].as_f64().unwrap();
    assert!(maximum_sum_error > 0.0);
    assert!(
        (maximum_sum_error - expected_sum_error).abs() <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
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
        ["check", "--bogus", "x"].as_slice(),
        ["check", "--dataset"].as_slice(),
        ["check", "--dataset", "a", "--dataset", "b"].as_slice(),
        ["check", "--out", "run"].as_slice(),
        ["evaluate", "--out", "a", "--out", "b"].as_slice(),
    ] {
        let output = command(arguments);
        assert_eq!(output.status.code(), Some(2));
        assert_schema("error", &output);
    }
    let missing = command(&[
        "check",
        "--dataset",
        "/definitely/missing",
        "--predictions",
        "/definitely/missing",
        "--config",
        "/definitely/missing",
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

    let checked = command(&[
        "check",
        "--dataset",
        as_string(&inputs.golden),
        "--predictions",
        as_string(&inputs.baseline),
        "--config",
        as_string(&inputs.config),
    ]);
    assert!(checked.status.success());
    let checked_document = assert_schema("check", &checked);
    assert_eq!(
        ids(&checked_document["selected_ids"]),
        ids(&expected["ids"])
    );
    assert_eq!(checked_document["integrity"]["source_count"], 2);
    assert_eq!(checked_document["integrity"]["signal_availability"], "both");

    let baseline_run = inputs.directory.join("baseline-run");
    let candidate_run = inputs.directory.join("candidate-run");
    let baseline_receipt = evaluate_steel_thread(&inputs, &inputs.baseline, &baseline_run);
    assert!(baseline_receipt.status.success());
    let baseline_receipt = assert_schema("receipt", &baseline_receipt);
    let baseline_report = report_from(&baseline_receipt);
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

    let baseline_relocated = inputs.directory.join("baseline-relocated");
    let candidate_relocated = inputs.directory.join("candidate-relocated");
    fs::rename(&baseline_run, &baseline_relocated).unwrap();
    fs::rename(&candidate_run, &candidate_relocated).unwrap();
    fs::remove_dir_all(inputs.directory.join("input")).unwrap();
    fs::remove_dir_all(inputs.directory.join("source-b")).unwrap();
    assert!(!inputs.golden.exists());
    assert!(!inputs.directory.join("source-b/evidence.bin").exists());

    let inspected = command(&[
        "inspect",
        "--run",
        as_string(&baseline_relocated),
        "--episode",
        "01995c20-7d00-7000-8000-000000000002",
    ]);
    assert!(inspected.status.success());
    let inspection = assert_schema("inspection", &inspected);
    let inspection_text = String::from_utf8(inspected.stdout).unwrap();
    assert!(inspection_text.contains("9007199254740993"));
    assert_eq!(
        inspection["observations"]["categorical_observation"]["values"]["x"],
        0.5
    );
    assert_eq!(
        inspection["observations"]["categorical_observation"]["values"]["y"],
        0.49
    );
    assert_eq!(inspection["source"]["kind"], "classifier");
    assert_eq!(inspection["source"]["model"], "steel-thread-a");
    assert_eq!(
        inspection["source"]["configuration"],
        json!({"fixture":"source-a"})
    );
    assert_eq!(inspection["source"]["evidence"], json!(["evidence/0.bin"]));
    assert_eq!(
        inspection["source"]["observations"]["categorical_observation"],
        json!({"kind":"categorical","description":"retained non-scoring categorical observation"})
    );

    let comparison_directory = inputs.directory.join("comparison");
    let compared = command(&[
        "compare",
        "--baseline",
        as_string(&baseline_relocated),
        "--candidate",
        as_string(&candidate_relocated),
        "--out",
        as_string(&comparison_directory),
    ]);
    assert!(compared.status.success());
    let comparison_receipt = assert_schema("receipt", &compared);
    let comparison = serde_json::from_slice::<Value>(
        &fs::read(comparison_directory.join("comparison.json")).unwrap(),
    )
    .unwrap();
    assert!(
        jsonschema::validator_for(&schema("comparison"))
            .unwrap()
            .is_valid(&comparison)
    );
    assert_eq!(
        comparison_receipt["result_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(comparison_directory.join("comparison.json")).unwrap())
        )
    );
    assert_eq!(
        ids(&comparison["population"]["compared_ids"]),
        ids(&expected["ids"])
    );
    for key in ["winner", "significance"] {
        assert!(comparison.get(key).is_none());
    }

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
    for category in ["recovered", "regressed", "both_correct", "neither_correct"] {
        assert_eq!(
            ids(&comparison["transitions"][category]["ids"]),
            ids(&expected["transitions"][category])
        );
    }

    let comparison_before = tree_snapshot(&comparison_directory);
    let existing_comparison = command(&[
        "compare",
        "--baseline",
        as_string(&baseline_relocated),
        "--candidate",
        as_string(&candidate_relocated),
        "--out",
        as_string(&comparison_directory),
    ]);
    assert_error(&existing_comparison, 3, "E_OUTPUT_EXISTS", "filesystem");
    assert_eq!(tree_snapshot(&comparison_directory), comparison_before);
    assert!(!inputs.directory.join(".comparison.validator-0").exists());

    fs::write(
        baseline_relocated.join("evidence/0.bin"),
        b"changed evidence",
    )
    .unwrap();
    let tampered_comparison = inputs.directory.join("tampered-comparison");
    let tampered = command(&[
        "compare",
        "--baseline",
        as_string(&baseline_relocated),
        "--candidate",
        as_string(&candidate_relocated),
        "--out",
        as_string(&tampered_comparison),
    ]);
    assert_error(&tampered, 2, "E_PROVENANCE", "replay");
    assert!(!tampered_comparison.exists());

    for document in [
        String::from_utf8(checked.stdout).unwrap(),
        serde_json::to_string(&baseline_receipt).unwrap(),
        serde_json::to_string(&baseline_report).unwrap(),
        serde_json::to_string(&candidate_report).unwrap(),
        serde_json::to_string(&comparison_receipt).unwrap(),
        serde_json::to_string(&comparison).unwrap(),
        String::from_utf8(tampered.stdout).unwrap(),
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
    let check = command(&[
        "check",
        "--dataset",
        golden.to_str().unwrap(),
        "--predictions",
        predictions.to_str().unwrap(),
        "--config",
        config.to_str().unwrap(),
    ]);
    assert!(check.status.success());
    assert_one_json_document(&check);
    assert_schema("check", &check);

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

    let inspect = command(&[
        "inspect",
        "--run",
        baseline.to_str().unwrap(),
        "--episode",
        "01995c20-7d00-7000-8000-000000000001",
    ]);
    assert!(inspect.status.success());
    assert_one_json_document(&inspect);
    assert_schema("inspection", &inspect);

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

    let comparison = directory.join("comparison");
    let compare = command(&[
        "compare",
        "--baseline",
        baseline.to_str().unwrap(),
        "--candidate",
        candidate.to_str().unwrap(),
        "--out",
        comparison.to_str().unwrap(),
    ]);
    assert!(compare.status.success());
    assert_one_json_document(&compare);
    let comparison_receipt = assert_schema("receipt", &compare);
    let comparison_document: Value = serde_json::from_slice(
        &fs::read(comparison_receipt["result_path"].as_str().unwrap()).unwrap(),
    )
    .unwrap();
    assert!(
        jsonschema::validator_for(&schema("comparison"))
            .unwrap()
            .is_valid(&comparison_document)
    );

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
    cli_check_evaluate();
    shared_commands_multi_label();
    cli_compare_receipt();
    cli_multi_label_compare_receipt();
}
