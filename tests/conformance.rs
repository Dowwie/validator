use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    os::unix::fs::symlink,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

fn raw_field(document: &str, name: &str) -> Box<serde_json::value::RawValue> {
    let mut fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(document).unwrap();
    fields.remove(name).unwrap()
}

#[test]
fn opaque_configurations_survive_report_assembly() {
    for kind in ["single_label", "multi_label"] {
        for text in ["0.5", "not-a-number"] {
            for nested in [false, true] {
                let marker = json!({"$serde_json::private::Number": text});
                let configuration = if nested {
                    json!({"nested": marker})
                } else {
                    marker
                };
                let target = if kind == "single_label" {
                    json!({"type":"class","label":"A"})
                } else {
                    json!({"type":"labels","labels":["A"]})
                };
                let id = "01995c20-7d00-7000-8000-000000000001";
                let (directory, golden, predictions, config) = write_policy_inputs(
                    kind,
                    json!(["A", "B"]),
                    json!([{"id":id,"expected":target,"input":null}]),
                    json!({"source":{"kind":"classifier","model":"oracle","configuration":configuration,"evidence":["preparation.txt"],"preparation":{"method":"audit","version":"1","configuration":configuration,"evidence_indices":[0]}}}),
                    json!([{"id":id,"source_id":"source","outcome":target}]),
                    json!({"schema_version":2,"population":"opaque configuration","role":"development","decision":{"type":"as_recorded"}}),
                );
                fs::write(directory.join("preparation.txt"), b"synthetic provenance").unwrap();
                let submitted = fs::read(&predictions).unwrap();
                validator::evaluate(validator::EvaluationOptions {
                    dataset: golden,
                    predictions,
                    config,
                    output: directory.join("run"),
                })
                .unwrap();
                let report = fs::read_to_string(directory.join("run/report.json")).unwrap();
                let sources = raw_field(&report, "sources");
                let source = raw_field(sources.get(), "source");
                let preparation = raw_field(source.get(), "preparation");
                let expected = serde_json::to_string(&configuration).unwrap();
                assert_eq!(raw_field(source.get(), "configuration").get(), expected);
                assert_eq!(
                    raw_field(preparation.get(), "configuration").get(),
                    expected
                );
                assert_eq!(
                    fs::read(directory.join("run/predictions.json")).unwrap(),
                    submitted
                );
                // Value interprets this private key on decode; rename only the opaque key
                // for the schema engine after asserting its exact original preservation.
                let schema_input = report.replace("$serde_json::private::Number", "opaque-key");
                assert_valid("report", serde_json::from_str(&schema_input).unwrap());
                fs::remove_dir_all(directory).unwrap();
            }
        }
    }
}

fn api_report(predictions: Value, sources: Value, golden_episodes: Value, config: Value) -> Value {
    let directory = std::env::temp_dir().join(format!(
        "validator-conformance-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let golden = directory.join("golden.json");
    let prediction = directory.join("predictions.json");
    let configuration = directory.join("config.json");
    let golden_bytes = serde_json::to_vec(&json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B","C"]},"episodes":golden_episodes})).unwrap();
    let digest = format!("{:x}", Sha256::digest(&golden_bytes));
    fs::write(&golden, &golden_bytes).unwrap();
    fs::write(&prediction, serde_json::to_vec(&json!({"schema_version":2,"dataset_sha256":digest,"sources":sources,"predictions":predictions})).unwrap()).unwrap();
    fs::write(&configuration, serde_json::to_vec(&config).unwrap()).unwrap();
    let output = directory.join("run");
    let receipt = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions: prediction,
        config: configuration,
        output,
    })
    .unwrap();
    let receipt = serde_json::to_value(receipt).unwrap();
    let report = fs::read(receipt["result_path"].as_str().unwrap()).unwrap();
    let report = serde_json::from_slice(&report).unwrap();
    fs::remove_dir_all(directory).unwrap();
    report
}

fn write_policy_inputs(
    task_kind: &str,
    labels: Value,
    episodes: Value,
    sources: Value,
    predictions: Value,
    config: Value,
) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let directory = std::env::temp_dir().join(format!(
        "validator-policy-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let golden = directory.join("golden.json");
    let prediction = directory.join("predictions.json");
    let configuration = directory.join("config.json");
    let golden_bytes = serde_json::to_vec(&json!({
        "schema_version": 2,
        "task": {"kind": task_kind, "labels": labels},
        "episodes": episodes,
    }))
    .unwrap();
    let digest = format!("{:x}", Sha256::digest(&golden_bytes));
    fs::write(&golden, golden_bytes).unwrap();
    fs::write(
        &prediction,
        serde_json::to_vec(&json!({
            "schema_version": 2,
            "dataset_sha256": digest,
            "sources": sources,
            "predictions": predictions,
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(&configuration, serde_json::to_vec(&config).unwrap()).unwrap();
    (directory, golden, prediction, configuration)
}

fn policy_run(
    task_kind: &str,
    labels: Value,
    episodes: Value,
    sources: Value,
    predictions: Value,
    config: Value,
) -> (PathBuf, Value) {
    let (directory, golden, prediction, configuration) =
        write_policy_inputs(task_kind, labels, episodes, sources, predictions, config);
    let receipt = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions: prediction,
        config: configuration,
        output: directory.join("run"),
    })
    .unwrap();
    let receipt = serde_json::to_value(receipt).unwrap();
    let report =
        serde_json::from_slice(&fs::read(receipt["result_path"].as_str().unwrap()).unwrap())
            .unwrap();
    (directory, report)
}

fn assert_policy_config_error(
    task_kind: &str,
    labels: Value,
    episodes: Value,
    sources: Value,
    predictions: Value,
    config: Value,
) {
    let (directory, golden, prediction, configuration) =
        write_policy_inputs(task_kind, labels, episodes, sources, predictions, config);
    let error = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions: prediction,
        config: configuration,
        output: directory.join("run"),
    })
    .unwrap_err();
    assert_eq!(error.code(), validator::DiagnosticCode::Config);
    assert!(!directory.join("run").exists());
    fs::remove_dir_all(directory).unwrap();
}

fn assert_evaluation_error(
    task_kind: &str,
    labels: Value,
    episodes: Value,
    sources: Value,
    predictions: Value,
    config: Value,
    expected: validator::DiagnosticCode,
) -> validator::Diagnostic {
    let (directory, golden, prediction, configuration) =
        write_policy_inputs(task_kind, labels, episodes, sources, predictions, config);
    let output = directory.join("run");
    let error = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions: prediction,
        config: configuration,
        output: output.clone(),
    })
    .unwrap_err();
    assert_eq!(error.code(), expected);
    assert!(!output.exists());
    fs::remove_dir_all(directory).unwrap();
    error
}
#[test]
fn admission_errors_preserve_checked_context() {
    let first = "01995c20-7d00-7000-8000-000000000001";
    let second = "01995c20-7d00-7000-8000-000000000002";
    for kind in ["single_label", "multi_label"] {
        let target = |label| {
            if kind == "single_label" {
                json!({"type":"class","label":label})
            } else {
                json!({"type":"labels","labels":[label]})
            }
        };
        let sources = json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}});
        let gold = json!([{"id":first,"expected":target("A"),"input":null},{"id":second,"expected":target("A"),"input":null}]);
        let row = json!({"id":first,"source_id":"source","outcome":target("A")});
        let config = json!({"schema_version":2,"population":"selected first","role":"development","episode_ids":[first],"decision":{"type":"as_recorded"}});
        let mut malformed_gold = gold.clone();
        malformed_gold[1]["expected"] = target("UNKNOWN");
        let mut duplicate_selection = config.clone();
        duplicate_selection["episode_ids"] = json!([first, first]);
        for (episodes, predictions, selection, code, affected) in [
            (
                malformed_gold,
                json!([row]),
                config.clone(),
                validator::DiagnosticCode::Label,
                second,
            ),
            (
                gold.clone(),
                json!([row, {"id":second,"source_id":"source","outcome":target("A")}]),
                config.clone(),
                validator::DiagnosticCode::Alignment,
                second,
            ),
            (
                gold.clone(),
                json!([row, row]),
                config.clone(),
                validator::DiagnosticCode::DuplicateId,
                first,
            ),
            (
                gold.clone(),
                json!([row]),
                duplicate_selection,
                validator::DiagnosticCode::DuplicateId,
                first,
            ),
        ] {
            let error = assert_evaluation_error(
                kind,
                json!(["A", "B"]),
                episodes,
                sources.clone(),
                predictions,
                selection,
                code,
            );
            let diagnostic: Value = serde_json::from_str(&error.to_json().unwrap()).unwrap();
            assert_eq!(diagnostic["affected_ids"], json!([affected]));
        }
        let probability_kind = if kind == "single_label" {
            "categorical"
        } else {
            "label_marginals"
        };
        let mut partial = json!([row, {"id":second,"source_id":"source","outcome":target("A")}]);
        partial[0]["probabilities"] = json!({"kind":probability_kind,"values":{"A":0.8,"B":0.2}});
        let mut all = config;
        all.as_object_mut().unwrap().remove("episode_ids");
        assert_evaluation_error(
            kind,
            json!(["A", "B"]),
            gold,
            sources,
            partial,
            all,
            validator::DiagnosticCode::Probability,
        );
    }
}

#[test]
fn file_read_errors_identify_the_path() {
    let directory = std::env::temp_dir().join(format!(
        "validator-missing-input-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    let dataset = directory.join("absent.json");
    let error = validator::evaluate(validator::EvaluationOptions {
        dataset: dataset.clone(),
        predictions: directory.join("predictions.json"),
        config: directory.join("config.json"),
        output: directory.join("run"),
    })
    .unwrap_err();
    let diagnostic: Value = serde_json::from_str(&error.to_json().unwrap()).unwrap();
    assert_eq!(diagnostic["code"], "E_IO");
    assert_eq!(diagnostic["path"], dataset.display().to_string());
    assert!(!directory.exists());
}

#[test]
fn threshold_endpoints_and_normalization_disclosure() {
    let id = "01995c20-7d00-7000-8000-000000000001";
    for threshold in [0.0, 1.0] {
        let (directory, report) = policy_run(
            "single_label",
            json!(["A", "B"]),
            json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
            json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
            json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"confidence":threshold}]),
            json!({"schema_version":2,"population":"threshold endpoint","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":threshold}}),
        );
        assert_eq!(report["final"]["answered"], 1);
        assert_eq!(report["final"]["abstained"], 0);
        fs::remove_dir_all(directory).unwrap();
    }
    let (directory, report) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":["A","B"]},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["B"]},"probabilities":{"kind":"label_marginals","values":{"A":0.0,"B":1.0}}}]),
        json!({"schema_version":2,"population":"threshold endpoints","role":"development","decision":{"type":"label_thresholds","thresholds":{"A":0.0,"B":1.0}}}),
    );
    assert_eq!(
        report["episodes"][0]["final_outcome"]["labels"],
        json!(["A", "B"])
    );
    assert_eq!(report["final"]["exact_matches"], 1);
    assert_eq!(
        report["probability"]["mean_binary_log_loss"]["status"],
        "positive_infinity"
    );
    assert_eq!(
        report["integrity"]["categorical_normalization"],
        "not_applicable"
    );
    assert_valid("report", report.clone());
    let mut missing = report.clone();
    missing["integrity"]
        .as_object_mut()
        .unwrap()
        .remove("categorical_normalization");
    assert_invalid("report", missing);
    let mut false_normalization = report;
    false_normalization["integrity"]["categorical_normalization"] = json!("defined");
    assert_invalid("report", false_normalization);
    fs::remove_dir_all(directory).unwrap();
}

type MultiLabelReportRow<'a> = (&'a [&'a str], Option<&'a [&'a str]>, &'a [f64]);

#[test]
fn case_s23() {
    let id = "01995c20-7d00-7000-8000-000000000001";
    let (directory, golden, predictions, config) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        json!({"schema_version":2,"population":"case_s23","role":"development","decision":{"type":"as_recorded"}}),
    );
    let run = directory.join("run");
    validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: run.clone(),
    })
    .unwrap();
    for (name, create) in [
        (
            "file",
            Box::new(|path: &PathBuf| fs::write(path, b"winner").unwrap()) as Box<dyn Fn(&PathBuf)>,
        ),
        (
            "directory",
            Box::new(|path: &PathBuf| fs::create_dir(path).unwrap()) as Box<dyn Fn(&PathBuf)>,
        ),
        (
            "symlink",
            Box::new(|path: &PathBuf| symlink("winner", path).unwrap()) as Box<dyn Fn(&PathBuf)>,
        ),
    ] {
        let output = directory.join(format!("existing-{name}"));
        create(&output);
        let error = validator::evaluate(validator::EvaluationOptions {
            dataset: directory.join("golden.json"),
            predictions: directory.join("predictions.json"),
            config: directory.join("config.json"),
            output: output.clone(),
        })
        .unwrap_err();
        assert_eq!(error.code(), validator::DiagnosticCode::OutputExists);
        match name {
            "file" => assert_eq!(fs::read(output).unwrap(), b"winner"),
            "directory" => assert!(output.is_dir()),
            "symlink" => assert_eq!(fs::read_link(output).unwrap(), PathBuf::from("winner")),
            _ => unreachable!(),
        }
    }
    let failed = directory.join("late-validation-failure");
    fs::write(directory.join("predictions.json"), b"{}").unwrap();
    let error = validator::evaluate(validator::EvaluationOptions {
        dataset: directory.join("golden.json"),
        predictions: directory.join("predictions.json"),
        config: directory.join("config.json"),
        output: failed.clone(),
    })
    .unwrap_err();
    assert_eq!(error.code(), validator::DiagnosticCode::Schema);
    assert!(!failed.exists());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_s26() {
    let id = "01995c20-7d00-7000-8000-000000000001";
    let (directory, golden, predictions, config) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        json!({"schema_version":2,"population":"case_s26","role":"development","decision":{"type":"as_recorded"}}),
    );
    let evaluation_output = directory.join("evaluation-receipt");
    let evaluation_receipt = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: evaluation_output.clone(),
    })
    .unwrap();
    let evaluation_receipt = serde_json::to_value(evaluation_receipt).unwrap();
    assert_eq!(
        evaluation_receipt["result_path"],
        evaluation_output.join("report.json").display().to_string()
    );
    assert_eq!(
        evaluation_receipt["result_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(evaluation_output.join("report.json")).unwrap())
        )
    );
    let report = fs::read(evaluation_output.join("report.json")).unwrap();
    let report_digest = format!("{:x}", Sha256::digest(&report));
    let manifest: Value = serde_json::from_slice(&report).unwrap();
    assert_eq!(manifest["artifacts"][0]["kind"], "golden");
    assert_eq!(report_digest.len(), 64);
    fs::remove_dir_all(directory).unwrap();
}

fn opaque_evaluation_run() -> (PathBuf, PathBuf) {
    let id = "01995c20-7d00-7000-8000-000000000881".to_owned();
    let (directory, golden, predictions, config) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":{"opaque":"S27-opaque-input","integer":9007199254740993_u64}}]),
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"evidence":["evidence.bin"]}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        json!({"schema_version":2,"population":"case_s27","role":"development","decision":{"type":"as_recorded"}}),
    );
    fs::write(directory.join("evidence.bin"), b"S27-evidence-content").unwrap();
    let run = directory.join("run");
    validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: run.clone(),
    })
    .unwrap();
    (directory, run)
}

#[test]
fn case_s27() {
    let (directory, run) = opaque_evaluation_run();
    let report = String::from_utf8(fs::read(run.join("report.json")).unwrap()).unwrap();
    for sentinel in ["S27-opaque-input", "S27-evidence-content"] {
        assert!(!report.contains(sentinel));
    }
    let stored_golden = String::from_utf8(fs::read(run.join("golden.json")).unwrap()).unwrap();
    assert!(stored_golden.contains("S27-opaque-input"));
    assert!(stored_golden.contains("9007199254740993"));
    assert_eq!(
        fs::read(run.join("evidence/0.bin")).unwrap(),
        b"S27-evidence-content"
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_s15() {
    let ids = [
        "01995c20-7d00-7000-8000-000000002615",
        "01995c20-7d00-7000-8000-000000002616",
    ];
    let (directory, golden, predictions, config) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        json!([
            {"id":ids[0],"expected":{"type":"class","label":"A"},"input":null},
            {"id":ids[1],"expected":{"type":"class","label":"B"},"input":null}
        ]),
        json!({
            "Q1":{"kind":"classifier","model":"retained-A","configuration":{"question":"Q1","revision":1}},
            "Q2":{"kind":"classifier","model":"revised-B","configuration":{"question":"Q2","revision":2}}
        }),
        json!([
            {"id":ids[0],"source_id":"Q1","outcome":{"type":"class","label":"A"}},
            {"id":ids[1],"source_id":"Q2","outcome":{"type":"class","label":"B"}}
        ]),
        json!({"schema_version":2,"population":"case_s15","role":"development","decision":{"type":"as_recorded"}}),
    );
    let run = directory.join("run");
    validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: run.clone(),
    })
    .unwrap();
    let report: Value =
        serde_json::from_slice(&fs::read(run.join("report.json")).unwrap()).unwrap();
    assert_eq!(report["composition"], "mixed_source");
    assert_eq!(report["source_counts"], json!({"Q1":1,"Q2":1}));
    assert_eq!(report["episodes"][0]["source_id"], "Q1");
    assert_eq!(report["episodes"][1]["source_id"], "Q2");
    assert_eq!(
        report["sources"]["Q1"]["configuration"],
        json!({"question":"Q1","revision":1})
    );
    assert_eq!(
        report["sources"]["Q2"]["configuration"],
        json!({"question":"Q2","revision":2})
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_s16() {
    let id = "01995c20-7d00-7000-8000-000000002616";
    let episodes = json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]);
    let source = json!({"source":{"kind":"classifier","model":"m","configuration":{}}});
    let config = json!({"schema_version":2,"population":"case_s16","role":"development","decision":{"type":"as_recorded"}});
    for (invalid, expected) in [
        (
            json!({"id":id,"outcome":{"type":"class","label":"A"}}),
            validator::DiagnosticCode::Schema,
        ),
        (
            json!({"id":id,"source_id":"unknown","outcome":{"type":"class","label":"A"}}),
            validator::DiagnosticCode::Provenance,
        ),
    ] {
        let (directory, golden, predictions, config_path) = write_policy_inputs(
            "single_label",
            json!(["A", "B"]),
            episodes.clone(),
            source.clone(),
            json!([invalid]),
            config.clone(),
        );
        let output = directory.join("run");
        let error = validator::evaluate(validator::EvaluationOptions {
            dataset: golden,
            predictions,
            config: config_path,
            output: output.clone(),
        })
        .unwrap_err();
        assert_eq!(error.code(), expected);
        assert!(!output.exists());
        fs::remove_dir_all(directory).unwrap();
    }
}

#[test]
fn case_s18() {
    let id = "01995c20-7d00-7000-8000-000000002618";
    let config = json!({"schema_version":2,"population":"case_s18","role":"development","decision":{"type":"as_recorded"}});
    let make_run = |input: Value| {
        let (directory, golden, predictions, config_path) = write_policy_inputs(
            "single_label",
            json!(["A", "B"]),
            json!([{"id":id,"expected":{"type":"class","label":"A"},"input":input}]),
            json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
            json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
            config.clone(),
        );
        let golden_bytes = fs::read(&golden).unwrap();
        let run = directory.join("run");
        validator::evaluate(validator::EvaluationOptions {
            dataset: golden,
            predictions,
            config: config_path,
            output: run.clone(),
        })
        .unwrap();
        (directory, run, golden_bytes)
    };
    let (_baseline_directory, baseline, baseline_golden) = make_run(json!({"reference":"first"}));
    let retained = fs::read(baseline.join("golden.json")).unwrap();
    let (candidate_directory, _candidate, candidate_golden) =
        make_run(json!({"reference":"revised"}));
    assert_ne!(
        Sha256::digest(&baseline_golden).as_slice(),
        Sha256::digest(&candidate_golden).as_slice()
    );
    assert_eq!(fs::read(baseline.join("golden.json")).unwrap(), retained);
    fs::remove_dir_all(candidate_directory).unwrap();
}

#[test]
fn case_e08() {
    let id = "01995c20-7d00-7000-8000-000000002608";
    let (directory, golden, predictions, config) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"prepared","configuration":{},"evidence":["script.py","raw.json","receipt.json"],"observation_definitions":{"retained":{"kind":"categorical","description":"original returned distribution"}},"preparation":{"method":"external_normalize","version":"1","configuration":{"canonical":"categorical"},"evidence_indices":[0,1,2]}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"retained":{"kind":"categorical","values":{"A":0.5,"B":0.49}}},"probabilities":{"kind":"categorical","values":{"A":0.5,"B":0.5}}}]),
        json!({"schema_version":2,"population":"case_e08","role":"development","decision":{"type":"as_recorded"}}),
    );
    fs::write(directory.join("script.py"), b"canonical preparation script").unwrap();
    fs::write(directory.join("raw.json"), b"{\"A\":0.5,\"B\":0.49}").unwrap();
    fs::write(
        directory.join("receipt.json"),
        b"prepared categorical receipt",
    )
    .unwrap();
    let run = directory.join("run");
    validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: run.clone(),
    })
    .unwrap();
    let report: Value =
        serde_json::from_slice(&fs::read(run.join("report.json")).unwrap()).unwrap();
    assert_eq!(
        report["episodes"][0]["observations"]["retained"]["values"],
        json!({"A":0.5,"B":0.49})
    );
    assert_eq!(
        report["episodes"][0]["probability"]["submitted"],
        json!([0.5, 0.5])
    );
    assert_eq!(
        report["episodes"][0]["probability"]["working"],
        json!([0.5, 0.5])
    );
    assert_eq!(
        report["sources"]["source"]["preparation"]["method"],
        "external_normalize"
    );
    assert_eq!(
        report["sources"]["source"]["preparation"]["evidence_indices"],
        json!([0, 1, 2])
    );
    let artifacts = report["artifacts"].as_array().unwrap();
    assert_eq!(
        artifacts
            .iter()
            .filter(|artifact| artifact["kind"] == "evidence")
            .count(),
        3
    );
    for (index, original) in ["script.py", "raw.json", "receipt.json"]
        .into_iter()
        .enumerate()
    {
        let artifact = &artifacts[index + 3];
        assert_eq!(artifact["source_id"], "source");
        assert_eq!(artifact["evidence_index"], index);
        assert_eq!(artifact["original_path"], original);
        assert_eq!(artifact["path"], format!("evidence/{index}.bin"));
    }
    fs::remove_dir_all(directory).unwrap();
}

fn fixture_report(probability: bool, confidence: bool) -> Value {
    let ids = [
        "01995c20-7d00-7000-8000-000000000001",
        "01995c20-7d00-7000-8000-000000000002",
    ];
    let predictions = ids.iter().zip(["A", "B"]).map(|(id, label)| {
        let mut row = json!({"id":id,"source_id":"source","outcome":{"type":"class","label":label}});
        if probability { row["probabilities"] = json!({"kind":"categorical","values":{"A":if label == "A" { 0.8 } else { 0.1 },"B":if label == "B" { 0.8 } else { 0.1 },"C":0.1}}); }
        if confidence { row["confidence"] = json!(0.8); }
        row
    }).collect::<Vec<_>>();
    api_report(
        json!(predictions),
        json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
        json!([
            {"id":ids[0],"expected":{"type":"class","label":"A"},"input":"opaque sentinel"},
            {"id":ids[1],"expected":{"type":"class","label":"B"},"input":"opaque sentinel"}
        ]),
        json!({"schema_version":2,"population":"test","role":"development","decision":{"type":"as_recorded"}}),
    )
}

fn schema(name: &str) -> Value {
    serde_json::from_str(match name {
        "golden" => include_str!("../schemas/v2/golden.schema.json"),
        "predictions" => include_str!("../schemas/v2/predictions.schema.json"),
        "config" => include_str!("../schemas/v2/config.schema.json"),
        "report" => include_str!("../schemas/v2/report.schema.json"),
        "comparison" => include_str!("../schemas/v2/comparison.schema.json"),
        "receipt" => include_str!("../schemas/v2/receipt.schema.json"),
        "error" => include_str!("../schemas/v2/error.schema.json"),
        _ => unreachable!(),
    })
    .unwrap()
}

fn assert_valid(name: &str, value: Value) {
    assert!(
        jsonschema::validator_for(&schema(name))
            .unwrap()
            .is_valid(&value),
        "{name}: {value}"
    );
}

fn assert_invalid(name: &str, value: Value) {
    assert!(
        !jsonschema::validator_for(&schema(name))
            .unwrap()
            .is_valid(&value),
        "{name}: {value}"
    );
}

fn case(case: &str, schema_name: &str, expected: bool, value: Value) {
    let actual = jsonschema::validator_for(&schema(schema_name))
        .unwrap()
        .is_valid(&value);
    assert_eq!(actual, expected, "{case}");
}

fn report_metric(status: &str, value: Value) -> Value {
    json!({"value":value,"status":status,"population_count":1,"population_unit":"episode","population_scope":"selected","numerator":1,"denominator":1})
}

fn report_bins(status: &str) -> Value {
    json!({
        "status": status,
        "population_scope":"selected",
        "population_count":1,
        "included_ids":["01995c20-7d00-7000-8000-000000000001"],
        "excluded_ids":[],
        "bins": (0..10).map(|index| json!({"index":index,"lower":index as f64 / 10.0,"upper":(index + 1) as f64 / 10.0,"upper_inclusive":index == 9,"count":if index == 8 { 1 } else { 0 },"correct_count":if index == 8 { 1 } else { 0 },"mean_signal":if index == 8 { json!(0.8) } else { Value::Null },"empirical_accuracy":if index == 8 { json!(1.0) } else { Value::Null }})).collect::<Vec<_>>()
    })
}

fn single_report(probability: bool, empty: bool, evidence_abstention: bool) -> Value {
    let metric = report_metric(
        if probability {
            "defined"
        } else {
            "not_applicable"
        },
        if probability { json!(1.0) } else { Value::Null },
    );
    let id = "01995c20-7d00-7000-8000-000000000001";
    let hard = json!({
        "kind":"single_label","total":if empty { 0 } else { 1 },"correct":if empty { 0 } else { 1 },"wrong":0,"abstained":0,"answered":if empty { 0 } else { 1 },
        "matrix":{"columns":[{"type":"label","label":"A"},{"type":"label","label":"B"},{"type":"abstention"}],"rows":[[if empty { 0 } else { 1 },0,0],[0,0,0]]},
        "classes":[{"label":"A","support":if empty { 0 } else { 1 },"predicted_support":if empty { 0 } else { 1 },"true_positive":if empty { 0 } else { 1 },"false_positive":0,"false_negative":0,"precision":metric,"recall":metric,"f1":metric,"coverage":metric},{"label":"B","support":0,"predicted_support":0,"true_positive":0,"false_positive":0,"false_negative":0,"precision":metric,"recall":metric,"f1":metric,"coverage":metric}],
        "accuracy":metric,"wrong_class_rate":metric,"abstention_rate":metric,"coverage":metric,"selective_accuracy":metric,"selective_risk":metric,"macro_f1":{"metric":metric,"undefined_classes":[]}
    });
    let source = json!({"kind":"classifier","model":"model","configuration":{"allowed":"opaque"},"evidence":if evidence_abstention { json!(["evidence/0.bin"]) } else { json!([]) },"observations":{"review":{"kind":"scalar","description":"a retained value"}},"preparation":{"method":"prepare","version":"1","configuration":{},"evidence_indices":[0]}});
    let episode = if empty {
        Value::Null
    } else {
        json!({"id":id,"source_id":"source","expected":{"type":"class","label":"A"},"raw_outcome":if evidence_abstention { json!({"type":"abstention"}) } else { json!({"type":"class","label":"A"}) },"final_outcome":if evidence_abstention { json!({"type":"abstention"}) } else { json!({"type":"class","label":"A"}) },"raw_correct":!evidence_abstention,"final_correct":!evidence_abstention,"rejection_reason":null,"observations":{"review":{"kind":"scalar","value":0.8}},"probability":if probability { json!({"submitted":[0.8,0.2],"working":[0.8,0.2],"argmax":"A","max_probability":0.8,"chosen_probability":if evidence_abstention { Value::Null } else { json!(0.8) },"choice_argmax_disagreement":if evidence_abstention { Value::Null } else { json!(false) },"argmax_correct":true}) } else { Value::Null },"reported_confidence":if probability { json!(0.8) } else { Value::Null }})
    };
    json!({
        "schema_version":2,"kind":"evaluation","status":"complete",
        "identity":{"run_id":"01995c20-7d00-7000-8000-000000000099","created_at":"2026-09-18T00:00:00Z","validator_version":"v","specification_version":"v1"},
        "artifacts": [
            {"kind":"golden","path":"golden.json","sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"},
            {"kind":"predictions","path":"predictions.json","sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"},
            {"kind":"config","path":"config.json","sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"}
        ],
        "sources": if empty { json!({}) } else { json!({"source":source}) },
        "composition":if empty { "empty" } else { "single_source" },"source_counts":if empty { json!({}) } else { json!({"source":1}) },
        "population":{"description":"population","role":"development","dataset_count":if empty { 0 } else { 1 },"selected_count":if empty { 0 } else { 1 },"selected_ids":if empty { json!([]) } else { json!([id]) },"unselected_ids":[]},
        "task":{"kind":"single_label","labels":["A","B"]},"policy":{"decision":{"type":"as_recorded"},"categorical_sum_tolerance":0.000000001,"bin_count":10},
        "integrity":{"source_count":if empty { 0 } else { 1 },"selected_count":if empty { 0 } else { 1 },"prediction_count":if empty { 0 } else { 1 },"missing_ids":[],"extra_ids":[],"signal_availability":if probability { "both" } else { "none" },"normalized_count":0,"maximum_sum_error":0.0},
        "raw":hard,"final":hard,"probability":{"kind":"single_label","log_loss":metric,"brier_score":metric,"argmax_accuracy":metric,"raw_answered_count":if empty { 0 } else { 1 },"choice_argmax_disagreement_count":if probability { json!(0) } else { Value::Null }},
        "signals":{"kind":"single_label","maximum_probability":report_bins(if probability { "defined" } else { "not_applicable" }),"confidence":report_bins(if probability { "defined" } else { "not_applicable" }),"top_label_ece":metric},
        "episodes":if empty { json!([]) } else { json!([episode]) }
    })
}

#[test]
fn single_report_schema() {
    let complete = single_report(true, false, false);
    assert_valid("report", complete.clone());
    assert_valid("report", single_report(false, false, false));
    assert_valid("report", single_report(false, true, false));
    let mut scored_choice = complete.clone();
    scored_choice["sources"]["source"]["kind"] = json!("scored_choice");
    scored_choice["sources"]["source"]["question_id"] = json!("question");
    assert_valid("report", scored_choice);
    let mut evidence_abstention = single_report(true, false, true);
    evidence_abstention["artifacts"].as_array_mut().unwrap().push(json!({"kind":"evidence","path":"evidence/0.bin","sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","source_id":"source","evidence_index":0,"original_path":"input.txt"}));
    assert_valid("report", evidence_abstention);

    let mut missing_top_level = complete.clone();
    missing_top_level.as_object_mut().unwrap().remove("final");
    assert_invalid("report", missing_top_level);
    let mut wrong_tags = complete.clone();
    wrong_tags["kind"] = json!("unknown");
    assert_invalid("report", wrong_tags);
    let mut one_label_task = complete.clone();
    one_label_task["task"]["labels"] = json!(["A"]);
    assert_invalid("report", one_label_task);
    let mut malformed_metric = complete.clone();
    malformed_metric["raw"]["accuracy"]
        .as_object_mut()
        .unwrap()
        .remove("population_unit");
    assert_invalid("report", malformed_metric);
    let mut ambiguous_matrix = complete.clone();
    ambiguous_matrix["raw"]["matrix"]["columns"] =
        json!([{"type":"label","label":"A"},{"type":"label","label":"ABSTAIN"}]);
    assert_invalid("report", ambiguous_matrix);
    let mut malformed_evidence = complete.clone();
    malformed_evidence["artifacts"].as_array_mut().unwrap().push(json!({"kind":"evidence","path":"evidence/not-a-number.bin","sha256":"bad","source_id":"source","evidence_index":0,"original_path":"input.txt"}));
    assert_invalid("report", malformed_evidence);
    let mut forbidden_input = complete;
    forbidden_input["episodes"][0]["input"] = json!("opaque payload");
    assert_invalid("report", forbidden_input);
    let mut multi_policy_on_single = single_report(true, false, false);
    multi_policy_on_single["policy"]["decision"] =
        json!({"type": "label_thresholds", "thresholds": {"A": 0.5}});
    assert_invalid("report", multi_policy_on_single);

    assert_valid(
        "receipt",
        json!({"schema_version":2,"kind":"evaluation","status":"complete","run_id":"01995c20-7d00-7000-8000-000000000099","result_path":"/runs/r/report.json","result_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"}),
    );
    assert_invalid(
        "receipt",
        json!({"schema_version":2,"kind":"evaluation","status":"complete","run_id":"01995c20-7d00-7000-8000-000000000099","result_path":"runs/r/report.json","result_sha256":"bad"}),
    );
    assert_valid(
        "error",
        json!({"schema_version":2,"kind":"error","status":"error","code":"E_SCHEMA","stage":"schema","affected_ids":[],"message":"input does not match the required schema"}),
    );
    assert_invalid(
        "error",
        json!({"schema_version":2,"kind":"error","status":"complete","code":"E_SCHEMA","stage":"schema","affected_ids":[],"message":"x"}),
    );
}

#[test]
fn input_schema_contract() {
    let id = "01995c20-7d00-7000-8000-000000000001";
    assert_valid(
        "golden",
        json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":id,"expected":{"type":"class","label":"A"},"input":null}]}),
    );
    assert_valid(
        "golden",
        json!({"schema_version":2,"task":{"kind":"multi_label","labels":["A"]},"episodes":[{"id":id,"expected":{"type":"labels","labels":[]},"input":[]}] }),
    );
    assert_invalid(
        "golden",
        json!({"schema_version":1,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[]}),
    );

    let prediction_base = json!({"schema_version":2,"dataset_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","sources":{"s":{"kind":"classifier","model":"m","configuration":{}}},"predictions":[{"id":id,"source_id":"s","outcome":{"type":"class","label":"A"}}]});
    let config_base = json!({"schema_version":2,"population":"p","role":"development","decision":{"type":"as_recorded"}});
    case(
        "golden_rejects_labels_for_single",
        "golden",
        false,
        json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":id,"expected":{"type":"labels","labels":[]},"input":null}]}),
    );
    case(
        "golden_rejects_class_for_multi",
        "golden",
        false,
        json!({"schema_version":2,"task":{"kind":"multi_label","labels":["A"]},"episodes":[{"id":id,"expected":{"type":"class","label":"A"},"input":null}]}),
    );
    case(
        "golden_rejects_label_set_tag",
        "golden",
        false,
        json!({"schema_version":2,"task":{"kind":"multi_label","labels":["A"]},"episodes":[{"id":id,"expected":{"type":"label_set","labels":[]},"input":null}]}),
    );
    case(
        "golden_rejects_extra_top_episode_target",
        "golden",
        false,
        json!({"schema_version":2,"extra":1,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":id,"extra":1,"expected":{"type":"class","label":"A","extra":1},"input":null}]}),
    );
    case(
        "golden_rejects_whitespace_label",
        "golden",
        false,
        json!({"schema_version":2,"task":{"kind":"single_label","labels":[" ","B"]},"episodes":[]}),
    );
    let mut scored = prediction_base.clone();
    scored["sources"]["s"] =
        json!({"kind":"scored_choice","model":"m","configuration":{},"question_id":"q"});
    case(
        "source_accepts_scored_choice_question",
        "predictions",
        true,
        scored,
    );
    let mut missing_question = prediction_base.clone();
    missing_question["sources"]["s"] =
        json!({"kind":"scored_choice","model":"m","configuration":{}});
    case(
        "source_rejects_scored_choice_missing_question",
        "predictions",
        false,
        missing_question,
    );
    let mut label_marginals = prediction_base.clone();
    label_marginals["predictions"][0]["outcome"] = json!({"type":"labels","labels":["A"]});
    label_marginals["predictions"][0]["probabilities"] =
        json!({"kind":"label_marginals","values":{"A":0.2}});
    case(
        "prediction_accepts_label_marginals",
        "predictions",
        true,
        label_marginals,
    );
    let mut categorical_probabilities = prediction_base.clone();
    categorical_probabilities["predictions"][0]["probabilities"] =
        json!({"kind":"categorical","values":{"A":0.6,"B":0.4}});
    case(
        "prediction_accepts_categorical_probabilities",
        "predictions",
        true,
        categorical_probabilities,
    );
    let mut empty_categorical_probabilities = prediction_base.clone();
    empty_categorical_probabilities["predictions"][0]["probabilities"] =
        json!({"kind":"categorical","values":{}});
    case(
        "prediction_rejects_empty_categorical_probability_map",
        "predictions",
        false,
        empty_categorical_probabilities,
    );
    let mut empty_label_marginals_probabilities = prediction_base.clone();
    empty_label_marginals_probabilities["predictions"][0]["probabilities"] =
        json!({"kind":"label_marginals","values":{}});
    case(
        "prediction_rejects_empty_label_marginals_probability_map",
        "predictions",
        false,
        empty_label_marginals_probabilities,
    );
    let mut scalar_outside_unit = prediction_base.clone();
    scalar_outside_unit["predictions"][0]["observations"] =
        json!({"o":{"kind":"scalar","value":1.33}});
    case(
        "prediction_accepts_scalar_outside_unit",
        "predictions",
        true,
        scalar_outside_unit,
    );
    let mut bounded_observation = prediction_base.clone();
    bounded_observation["predictions"][0]["observations"] =
        json!({"o":{"kind":"bernoulli","value":2}});
    case(
        "prediction_rejects_out_of_range_bernoulli_observation",
        "predictions",
        false,
        bounded_observation,
    );
    let mut envelope_extra = prediction_base.clone();
    envelope_extra["extra"] = json!(1);
    case(
        "prediction_rejects_extra_envelope_field",
        "predictions",
        false,
        envelope_extra,
    );
    let mut row_extra = prediction_base.clone();
    row_extra["predictions"][0]["extra"] = json!(1);
    case(
        "prediction_rejects_extra_prediction_row_field",
        "predictions",
        false,
        row_extra,
    );
    let mut outcome_extra = prediction_base.clone();
    outcome_extra["predictions"][0]["outcome"]["extra"] = json!(1);
    case(
        "prediction_rejects_extra_outcome_field",
        "predictions",
        false,
        outcome_extra,
    );
    let mut whitespace_model = prediction_base.clone();
    whitespace_model["sources"]["s"]["model"] = json!(" ");
    case(
        "source_rejects_whitespace_only_model",
        "predictions",
        false,
        whitespace_model,
    );
    case(
        "config_accepts_all_policies",
        "config",
        true,
        json!({"schema_version":2,"population":"p","role":"development","decision":{"type":"label_thresholds","thresholds":{"A":0.5}}}),
    );
    let mut empty_thresholds = config_base.clone();
    empty_thresholds["decision"] = json!({"type":"label_thresholds","thresholds":{}});
    case(
        "config_rejects_empty_label_thresholds",
        "config",
        false,
        empty_thresholds,
    );
    let mut decision_extra = config_base.clone();
    decision_extra["decision"]["extra"] = json!(true);
    case(
        "config_rejects_extra_decision_field",
        "config",
        false,
        decision_extra,
    );
    case(
        "config_rejects_whitespace_threshold",
        "config",
        false,
        json!({"schema_version":2,"population":"p","role":"development","decision":{"type":"label_thresholds","thresholds":{" ":0.5}}}),
    );
    case(
        "runtime_source_binding_boundary",
        "predictions",
        true,
        json!({"schema_version":2,"dataset_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","sources":{},"predictions":[{"id":id,"source_id":"absent","outcome":{"type":"abstention"}}]}),
    );
    case(
        "source_key_whitespace_red_green",
        "predictions",
        false,
        json!({"schema_version":2,"dataset_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","sources":{" ":{"kind":"classifier","model":"m","configuration":{}}},"predictions":[]}),
    );
    case(
        "reject_below_complete",
        "config",
        true,
        json!({"schema_version":2,"population":"p","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":0.5}}),
    );
    case(
        "outcome_label_set_rejected",
        "predictions",
        false,
        json!({"schema_version":2,"dataset_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","sources":{},"predictions":[{"id":id,"source_id":"s","outcome":{"type":"label_set","labels":[]}}]}),
    );
    let mut all_observation_variants = prediction_base.clone();
    all_observation_variants["predictions"][0]["observations"] = json!({"scalar":{"kind":"scalar","value":1.33},"bernoulli":{"kind":"bernoulli","value":1},"confidence":{"kind":"reported_confidence","value":0.5},"categorical":{"kind":"categorical","values":{"a":0.5,"b":0.49}},"marginals":{"kind":"label_marginals","values":{"a":0.2}}});
    case(
        "all_observation_variants_legal",
        "predictions",
        true,
        all_observation_variants,
    );
    let mut scalar_values = prediction_base.clone();
    scalar_values["predictions"][0]["observations"] =
        json!({"x":{"kind":"scalar","values":{"a":0.5}}});
    case(
        "observation_rejects_scalar_values_field",
        "predictions",
        false,
        scalar_values,
    );
    let mut vector_value = prediction_base.clone();
    vector_value["predictions"][0]["observations"] =
        json!({"x":{"kind":"categorical","value":0.5}});
    case(
        "observation_rejects_vector_value_field",
        "predictions",
        false,
        vector_value,
    );
    let mut whitespace_vector_key = prediction_base.clone();
    whitespace_vector_key["predictions"][0]["observations"] =
        json!({"x":{"kind":"categorical","values":{" ":0.5}}});
    case(
        "observation_rejects_whitespace_vector_key",
        "predictions",
        false,
        whitespace_vector_key,
    );
    let mut empty_categorical_observation = prediction_base.clone();
    empty_categorical_observation["predictions"][0]["observations"] =
        json!({"x":{"kind":"categorical","values":{}}});
    case(
        "observation_rejects_empty_categorical_values",
        "predictions",
        false,
        empty_categorical_observation,
    );
    let mut empty_label_marginals_observation = prediction_base.clone();
    empty_label_marginals_observation["predictions"][0]["observations"] =
        json!({"x":{"kind":"label_marginals","values":{}}});
    case(
        "observation_rejects_empty_label_marginals_values",
        "predictions",
        false,
        empty_label_marginals_observation,
    );
    assert_invalid(
        "golden",
        json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":id,"expected":{"type":"class","label":"A"}}]}),
    );
    assert_valid(
        "predictions",
        json!({"schema_version":2,"dataset_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","sources":{"s":{"kind":"classifier","model":"m","configuration":{}}},"predictions":[{"id":id,"source_id":"s","outcome":{"type":"abstention"}}]}),
    );
    assert_invalid(
        "predictions",
        json!({"schema_version":2,"dataset_sha256":"bad","sources":{},"predictions":[]}),
    );
    assert_invalid(
        "predictions",
        json!({"schema_version":2,"dataset_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","sources":{},"predictions":[{"id":id,"source_id":"s","outcome":{"type":"unknown"}}]}),
    );
    assert_valid(
        "config",
        json!({"schema_version":2,"population":"all","role":"development","episode_ids":[],"decision":{"type":"as_recorded"}}),
    );
    assert_invalid(
        "config",
        json!({"schema_version":2,"population":"all","role":"development","decision":{"type":"reject_below","minimum":0.5}}),
    );
    assert_invalid(
        "config",
        json!({"schema_version":2,"population":"all","role":"development","decision":{"type":"as_recorded","extra":true}}),
    );
}

#[test]
fn decision_policy_boundaries() {
    let ids = [
        "01995c20-7d00-7000-8000-000000000101",
        "01995c20-7d00-7000-8000-000000000102",
        "01995c20-7d00-7000-8000-000000000103",
    ];
    let single_episodes = ids
        .iter()
        .map(|id| json!({"id": id, "expected": {"type": "class", "label": "A"}, "input": null}))
        .collect::<Vec<_>>();
    let confidence_values = [0.5_f64.next_down(), 0.5, 0.5_f64.next_up()];
    let single_predictions = ids
        .iter()
        .zip(confidence_values)
        .map(|(id, confidence)| {
            json!({"id": id, "source_id": "source", "outcome": {"type": "class", "label": "A"}, "probabilities": {"kind": "categorical", "values": {"A": 0.75, "B": 0.25}}, "confidence": confidence})
        })
        .collect::<Vec<_>>();
    let sources = json!({"source": {"kind": "classifier", "model": "m", "configuration": {}}});
    let single_config = json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "reject_below", "signal": "confidence", "minimum": 0.5}});
    let (single_directory, single_report) = policy_run(
        "single_label",
        json!(["A", "B"]),
        json!(single_episodes),
        sources.clone(),
        json!(single_predictions),
        single_config,
    );
    let (_, single_baseline) = policy_run(
        "single_label",
        json!(["A", "B"]),
        json!(single_episodes),
        sources.clone(),
        json!(single_predictions),
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "as_recorded"}}),
    );
    assert_valid("report", single_report.clone());
    assert_eq!(single_report["raw"], single_baseline["raw"]);
    assert_eq!(single_report["probability"], single_baseline["probability"]);
    assert_eq!(single_report["signals"], single_baseline["signals"]);
    assert_eq!(
        single_report["policy"]["decision"],
        json!({"type": "reject_below", "signal": "confidence", "minimum": 0.5})
    );
    assert_eq!(single_report["final"]["abstained"], 1);
    assert_eq!(
        single_report["episodes"][0]["final_outcome"]["type"],
        "abstention"
    );
    assert_eq!(
        single_report["episodes"][0]["rejection_reason"],
        "below_minimum"
    );
    assert_eq!(
        single_report["episodes"][1]["final_outcome"]["type"],
        "class"
    );
    assert_eq!(
        single_report["episodes"][2]["final_outcome"]["type"],
        "class"
    );
    assert_eq!(
        single_report["signals"]["confidence"]["included_ids"],
        json!(ids)
    );
    assert_eq!(single_report["probability"]["raw_answered_count"], 3);
    fs::remove_dir_all(single_directory).unwrap();

    let scored_id = "01995c20-7d00-7000-8000-000000000104";
    let (_, confidence_report) = policy_run(
        "single_label",
        json!(["A", "B"]),
        json!([{ "id": scored_id, "expected": {"type": "class", "label": "A"}, "input": null }]),
        json!({"source": {"kind": "scored_choice", "model": "m", "configuration": {}, "question_id": "q"}}),
        json!([{ "id": scored_id, "source_id": "source", "outcome": {"type": "class", "label": "A"}, "probabilities": {"kind": "categorical", "values": {"A": 0.6, "B": 0.4}}, "confidence": 0.5_f64.next_down() }]),
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "reject_below", "signal": "confidence", "minimum": 0.5}}),
    );
    assert_eq!(
        confidence_report["episodes"][0]["probability"]["max_probability"],
        0.6
    );
    assert_eq!(
        confidence_report["episodes"][0]["final_outcome"]["type"],
        "abstention"
    );
    let classifier_id = "01995c20-7d00-7000-8000-000000000105";
    let (_, maximum_report) = policy_run(
        "single_label",
        json!(["A", "B"]),
        json!([{ "id": classifier_id, "expected": {"type": "class", "label": "B"}, "input": null }]),
        sources.clone(),
        json!([{ "id": classifier_id, "source_id": "source", "outcome": {"type": "class", "label": "B"}, "probabilities": {"kind": "categorical", "values": {"A": 0.6, "B": 0.4}}, "confidence": 0.9 }]),
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "reject_below", "signal": "max_probability", "minimum": 0.5}}),
    );
    assert_eq!(
        maximum_report["episodes"][0]["probability"]["chosen_probability"],
        0.4
    );
    assert_eq!(
        maximum_report["episodes"][0]["probability"]["choice_argmax_disagreement"],
        true
    );
    assert_eq!(maximum_report["episodes"][0]["final_outcome"]["label"], "B");

    let multi_ids = [
        "01995c20-7d00-7000-8000-000000000201",
        "01995c20-7d00-7000-8000-000000000202",
        "01995c20-7d00-7000-8000-000000000203",
        "01995c20-7d00-7000-8000-000000000204",
    ];
    let multi_episodes = multi_ids
        .iter()
        .map(|id| json!({"id": id, "expected": {"type": "labels", "labels": []}, "input": null}))
        .collect::<Vec<_>>();
    let multi_probabilities = [
        (0.5_f64.next_down(), 0.6),
        (0.5, 0.4),
        (0.5_f64.next_up(), 0.6),
        (0.1, 0.2),
    ];
    let multi_predictions = multi_ids
        .iter()
        .zip(multi_probabilities)
        .map(|(id, (a, b))| {
            json!({"id": id, "source_id": "source", "outcome": {"type": "labels", "labels": ["B"]}, "probabilities": {"kind": "label_marginals", "values": {"A": a, "B": b}}})
        })
        .collect::<Vec<_>>();
    let multi_config = json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "label_thresholds", "thresholds": {"A": 0.5, "B": 0.5}}});
    let (multi_directory, multi_report) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        json!(multi_episodes),
        sources.clone(),
        json!(multi_predictions),
        multi_config,
    );
    let (_, multi_baseline) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        json!(multi_episodes),
        sources,
        json!(multi_predictions),
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "as_recorded"}}),
    );
    assert_valid("report", multi_report.clone());
    assert_eq!(multi_report["raw"], multi_baseline["raw"]);
    assert_eq!(multi_report["probability"], multi_baseline["probability"]);
    assert_eq!(multi_report["signals"], multi_baseline["signals"]);
    assert_eq!(
        multi_report["policy"]["decision"],
        json!({"type": "label_thresholds", "thresholds": {"A": 0.5, "B": 0.5}})
    );
    assert_eq!(
        multi_report["episodes"][0]["final_outcome"]["labels"],
        json!(["B"])
    );
    assert_eq!(
        multi_report["episodes"][1]["final_outcome"]["labels"],
        json!(["A"])
    );
    assert_eq!(
        multi_report["episodes"][2]["final_outcome"]["labels"],
        json!(["A", "B"])
    );
    assert_eq!(
        multi_report["episodes"][3]["final_outcome"]["type"],
        "labels"
    );
    assert_eq!(
        multi_report["episodes"][3]["final_outcome"]["labels"],
        json!([])
    );
    fs::remove_dir_all(multi_directory).unwrap();
}

#[test]
fn policy_preconditions() {
    let single_id = "01995c20-7d00-7000-8000-000000000301";
    let single_episode =
        json!([{ "id": single_id, "expected": {"type": "class", "label": "A"}, "input": null }]);
    let single_sources =
        json!({"source": {"kind": "classifier", "model": "m", "configuration": {}}});
    let single_answer = json!([{ "id": single_id, "source_id": "source", "outcome": {"type": "class", "label": "A"}, "probabilities": {"kind": "categorical", "values": {"A": 0.7, "B": 0.3}} }]);
    assert_policy_config_error(
        "single_label",
        json!(["A", "B"]),
        single_episode.clone(),
        single_sources.clone(),
        single_answer.clone(),
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "label_thresholds", "thresholds": {"A": 0.5, "B": 0.5}}}),
    );
    assert_policy_config_error(
        "single_label",
        json!(["A", "B"]),
        single_episode.clone(),
        single_sources.clone(),
        single_answer,
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "reject_below", "signal": "confidence", "minimum": 0.5}}),
    );
    assert_policy_config_error(
        "single_label",
        json!(["A", "B"]),
        single_episode,
        single_sources.clone(),
        json!([{ "id": single_id, "source_id": "source", "outcome": {"type": "abstention"}, "probabilities": {"kind": "categorical", "values": {"A": 0.7, "B": 0.3}}, "confidence": 0.8 }]),
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "reject_below", "signal": "confidence", "minimum": 0.5}}),
    );

    let multi_id = "01995c20-7d00-7000-8000-000000000302";
    let multi_episode =
        json!([{ "id": multi_id, "expected": {"type": "labels", "labels": []}, "input": null }]);
    let multi_answer = json!([{ "id": multi_id, "source_id": "source", "outcome": {"type": "labels", "labels": []}, "probabilities": {"kind": "label_marginals", "values": {"A": 0.4, "B": 0.6}} }]);
    let multi_config = json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "label_thresholds", "thresholds": {"A": 0.5, "B": 0.5}}});
    assert_policy_config_error(
        "multi_label",
        json!(["A", "B"]),
        multi_episode.clone(),
        single_sources.clone(),
        json!([{ "id": multi_id, "source_id": "source", "outcome": {"type": "labels", "labels": []} }]),
        multi_config.clone(),
    );
    assert_policy_config_error(
        "multi_label",
        json!(["A", "B"]),
        multi_episode.clone(),
        single_sources.clone(),
        multi_answer.clone(),
        json!({"schema_version": 2, "population": "policy", "role": "development", "decision": {"type": "label_thresholds", "thresholds": {"A": 0.5}}}),
    );
    assert_policy_config_error(
        "multi_label",
        json!(["A", "B"]),
        multi_episode,
        single_sources,
        json!([{ "id": multi_id, "source_id": "source", "outcome": {"type": "abstention"}, "probabilities": {"kind": "label_marginals", "values": {"A": 0.4, "B": 0.6}} }]),
        multi_config,
    );
}

#[test]
fn single_matrix_identities() {
    let report = fixture_report(false, false);
    assert_valid("report", report.clone());
    assert_eq!(report["raw"]["total"], 2);
    assert_eq!(report["raw"]["correct"], 2);
    assert_eq!(report["raw"]["wrong"], 0);
    assert_eq!(report["raw"]["abstained"], 0);
    assert_eq!(
        report["raw"]["matrix"]["rows"],
        json!([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]])
    );
}

#[test]
fn f04_asymmetric_oracle() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/single-label/expected.json")).unwrap();
    let ids = (1..=8)
        .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
        .collect::<Vec<_>>();
    let actual = ["A", "A", "A", "A", "B", "B", "C", "C"];
    let predicted = ["A", "A", "B", "C", "B", "B", "A", "C"];
    let report = api_report(
        json!(ids.iter().zip(predicted).map(|(id, label)| json!({"id":id,"source_id":"source","outcome":{"type":"class","label":label}})).collect::<Vec<_>>()),
        json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
        json!(ids.iter().zip(actual).map(|(id, label)| json!({"id":id,"expected":{"type":"class","label":label},"input":null})).collect::<Vec<_>>()),
        json!({"schema_version":2,"population":"f04","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_eq!(
        report["raw"]["matrix"]["rows"],
        json!([[2, 1, 1, 0], [0, 2, 0, 0], [1, 0, 1, 0]])
    );
    assert_eq!(report["raw"]["accuracy"]["numerator"], 5);
    assert_eq!(report["raw"]["accuracy"]["denominator"], 8);
    for (class, expected) in report["raw"]["classes"]
        .as_array()
        .unwrap()
        .iter()
        .zip(oracle["f04"]["class_f1"].as_array().unwrap())
    {
        assert_eq!(class["f1"]["numerator"], expected[0]);
        assert_eq!(class["f1"]["denominator"], expected[1]);
    }
    assert!(
        (report["raw"]["macro_f1"]["metric"]["value"]
            .as_f64()
            .unwrap()
            - 131.0 / 210.0)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
}

#[derive(Clone, Copy, Debug)]
struct DirectSingle {
    matrix: [[u64; 4]; 3],
    total: u64,
    correct: u64,
    wrong: u64,
    abstained: u64,
}

impl DirectSingle {
    fn from_rows(rows: &[(usize, Option<usize>)]) -> Self {
        let mut oracle = Self {
            matrix: [[0; 4]; 3],
            total: rows.len() as u64,
            correct: 0,
            wrong: 0,
            abstained: 0,
        };
        for &(actual, prediction) in rows {
            let column = prediction.unwrap_or(3);
            oracle.matrix[actual][column] += 1;
            match prediction {
                Some(predicted) if predicted == actual => oracle.correct += 1,
                Some(_) => oracle.wrong += 1,
                None => oracle.abstained += 1,
            }
        }
        oracle
    }

    fn answered(self) -> u64 {
        self.correct + self.wrong
    }

    fn class_counts(self, class: usize) -> (u64, u64, u64, u64, u64) {
        let support = self.matrix[class].iter().sum();
        let predicted_support = self.matrix.iter().map(|row| row[class]).sum();
        let tp = self.matrix[class][class];
        (
            support,
            predicted_support,
            tp,
            predicted_support - tp,
            support - tp,
        )
    }
}

fn assert_direct_fraction(
    metric: &Value,
    numerator: u64,
    denominator: u64,
    empty_status: &str,
    population_count: u64,
    population_unit: &str,
    population_scope: &str,
) {
    assert_eq!(metric["population_count"], population_count);
    assert_eq!(metric["population_unit"], population_unit);
    assert_eq!(metric["population_scope"], population_scope);
    assert_eq!(metric["numerator"], numerator);
    assert_eq!(metric["denominator"], denominator);
    if denominator == 0 {
        assert_eq!(metric["value"], Value::Null);
        assert_eq!(metric["status"], empty_status);
    } else {
        assert_eq!(metric["status"], "defined");
        assert!(
            (metric["value"].as_f64().unwrap() - numerator as f64 / denominator as f64).abs()
                <= validator::FIXTURE_ABSOLUTE_TOLERANCE
        );
    }
}

fn assert_single_direct_report(report: &Value, oracle: DirectSingle) {
    let hard = &report["raw"];
    assert_eq!(hard["total"], oracle.total);
    assert_eq!(hard["correct"], oracle.correct);
    assert_eq!(hard["wrong"], oracle.wrong);
    assert_eq!(hard["abstained"], oracle.abstained);
    assert_eq!(hard["answered"], oracle.answered());
    assert_eq!(hard["matrix"]["rows"], json!(oracle.matrix));

    let no_data = if oracle.total == 0 {
        "no_data"
    } else {
        "undefined_zero_denominator"
    };
    assert_direct_fraction(
        &hard["accuracy"],
        oracle.correct,
        oracle.total,
        no_data,
        oracle.total,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &hard["wrong_class_rate"],
        oracle.wrong,
        oracle.total,
        no_data,
        oracle.total,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &hard["abstention_rate"],
        oracle.abstained,
        oracle.total,
        no_data,
        oracle.total,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &hard["coverage"],
        oracle.answered(),
        oracle.total,
        no_data,
        oracle.total,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &hard["selective_accuracy"],
        oracle.correct,
        oracle.answered(),
        if oracle.total == 0 {
            "no_data"
        } else {
            "no_answered_predictions"
        },
        oracle.answered(),
        "episode",
        "answered",
    );
    assert_direct_fraction(
        &hard["selective_risk"],
        oracle.wrong,
        oracle.answered(),
        if oracle.total == 0 {
            "no_data"
        } else {
            "no_answered_predictions"
        },
        oracle.answered(),
        "episode",
        "answered",
    );

    let mut macro_sum = 0.0;
    let mut undefined = Vec::new();
    assert_eq!(hard["classes"].as_array().unwrap().len(), 3);
    for (class, actual) in hard["classes"].as_array().unwrap().iter().enumerate() {
        assert_eq!(actual["label"], ["A", "B", "C"][class]);
        let (support, predicted_support, tp, fp, fn_count) = oracle.class_counts(class);
        assert_eq!(actual["support"], support);
        assert_eq!(actual["predicted_support"], predicted_support);
        assert_eq!(actual["true_positive"], tp);
        assert_eq!(actual["false_positive"], fp);
        assert_eq!(actual["false_negative"], fn_count);
        let metric_status = if oracle.total == 0 {
            "no_data"
        } else {
            "undefined_zero_denominator"
        };
        assert_direct_fraction(
            &actual["precision"],
            tp,
            tp + fp,
            metric_status,
            oracle.total,
            "episode",
            "selected",
        );
        assert_direct_fraction(
            &actual["recall"],
            tp,
            tp + fn_count,
            metric_status,
            oracle.total,
            "episode",
            "selected",
        );
        let f1_denominator = 2 * tp + fp + fn_count;
        assert_direct_fraction(
            &actual["f1"],
            2 * tp,
            f1_denominator,
            metric_status,
            oracle.total,
            "episode",
            "selected",
        );
        assert_direct_fraction(
            &actual["coverage"],
            support - oracle.matrix[class][3],
            support,
            metric_status,
            oracle.total,
            "episode",
            "selected",
        );
        if f1_denominator == 0 {
            undefined.push(json!(["A", "B", "C"][class]));
        } else {
            macro_sum += 2.0 * tp as f64 / f1_denominator as f64;
        }
    }
    let macro_metric = &hard["macro_f1"]["metric"];
    if oracle.total == 0 {
        assert_eq!(macro_metric["status"], "no_data");
        assert_eq!(macro_metric["value"], Value::Null);
    } else {
        assert_eq!(
            macro_metric["status"],
            if undefined.is_empty() {
                "defined"
            } else {
                "contains_undefined_classes"
            }
        );
        assert!(
            (macro_metric["value"].as_f64().unwrap() - macro_sum / 3.0).abs()
                <= validator::FIXTURE_ABSOLUTE_TOLERANCE
        );
    }
    assert_eq!(
        hard["macro_f1"]["undefined_classes"],
        if oracle.total == 0 {
            json!([])
        } else {
            json!(undefined)
        }
    );
    assert_eq!(report["final"], report["raw"]);
}

fn all_sequences(width: usize, alphabet: usize) -> Vec<Vec<usize>> {
    if width == 0 {
        return vec![Vec::new()];
    }
    let prefixes = all_sequences(width - 1, alphabet);
    let mut sequences = Vec::with_capacity(prefixes.len() * alphabet);
    for prefix in prefixes {
        for value in 0..alphabet {
            let mut sequence = prefix.clone();
            sequence.push(value);
            sequences.push(sequence);
        }
    }
    sequences
}

#[test]
fn exhaustive_single_label() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/single-label/expected.json")).unwrap();
    let mut populations = 0;
    for length in 0..=2 {
        for actual in all_sequences(length, 3) {
            for decisions in all_sequences(length, 4) {
                let permutations = if length < 2 {
                    vec![(0..length).collect::<Vec<_>>()]
                } else {
                    vec![vec![0, 1], vec![1, 0]]
                };
                for permutation in permutations {
                    let rows = permutation
                        .iter()
                        .map(|&index| {
                            (
                                actual[index],
                                (decisions[index] < 3).then_some(decisions[index]),
                            )
                        })
                        .collect::<Vec<_>>();
                    let predictions = json!(rows.iter().enumerate().map(|(index, (_, prediction))| {
                        let id = format!("01995c20-7d00-7000-8000-{index:012}");
                        match prediction {
                            Some(label) => json!({"id":id,"source_id":"source","outcome":{"type":"class","label":(["A", "B", "C"][*label])}}),
                            None => json!({"id":id,"source_id":"source","outcome":{"type":"abstention","reason":"oracle"}}),
                        }
                    }).collect::<Vec<_>>());
                    let episodes = json!(rows.iter().enumerate().map(|(index, (actual, _))| {
                        let id = format!("01995c20-7d00-7000-8000-{index:012}");
                        json!({"id":id,"expected":{"type":"class","label":(["A", "B", "C"][*actual])},"input":null})
                    }).collect::<Vec<_>>());
                    let report = api_report(
                        predictions,
                        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
                        episodes,
                        json!({"schema_version":2,"population":"exhaustive_single_label","role":"development","decision":{"type":"as_recorded"}}),
                    );
                    assert_single_direct_report(&report, DirectSingle::from_rows(&rows));
                    populations += 1;
                }
            }
        }
    }
    assert_eq!(populations, fixture["exhaustive"]["single_populations"]);
}

#[derive(Clone, Copy, Debug, Default)]
struct DirectLabel {
    support: u64,
    answered_support: u64,
    predicted_support: u64,
    tp: u64,
    fp: u64,
    fn_count: u64,
    tn: u64,
}

fn label_names(mask: u8) -> Vec<&'static str> {
    ["A", "B", "C"]
        .into_iter()
        .enumerate()
        .filter_map(|(index, label)| (mask & (1 << index) != 0).then_some(label))
        .collect()
}

fn assert_multi_direct_report(report: &Value, labels: &[&str], rows: &[(u8, Option<u8>)]) {
    let total = rows.len() as u64;
    let answered = rows
        .iter()
        .filter(|(_, prediction)| prediction.is_some())
        .count() as u64;
    let exact = rows
        .iter()
        .filter(|(expected, prediction)| *prediction == Some(*expected))
        .count() as u64;
    let counts = direct_multi_counts(labels.len(), rows);
    for family in ["raw", "final"] {
        assert_multi_hard(&report[family], labels, &counts, total, answered, exact);
    }
    assert_multi_episodes(report, labels, rows);
}

fn direct_multi_counts(label_count: usize, rows: &[(u8, Option<u8>)]) -> Vec<DirectLabel> {
    (0..label_count)
        .map(|i| {
            let mut count = DirectLabel::default();
            for &(expected, prediction) in rows {
                let positive = expected & (1 << i) != 0;
                count.support += u64::from(positive);
                if let Some(prediction) = prediction {
                    let predicted = prediction & (1 << i) != 0;
                    count.answered_support += u64::from(positive);
                    count.predicted_support += u64::from(predicted);
                    count.tp += u64::from(positive && predicted);
                    count.fp += u64::from(!positive && predicted);
                    count.fn_count += u64::from(positive && !predicted);
                    count.tn += u64::from(!positive && !predicted);
                }
            }
            count
        })
        .collect()
}

fn assert_multi_hard(
    hard: &Value,
    labels: &[&str],
    counts: &[DirectLabel],
    total: u64,
    answered: u64,
    exact: u64,
) {
    let answered_status = if total == 0 {
        "no_data"
    } else if answered == 0 {
        "no_answered_predictions"
    } else {
        "undefined_zero_denominator"
    };
    let tp = counts.iter().map(|c| c.tp).sum::<u64>();
    let fp = counts.iter().map(|c| c.fp).sum::<u64>();
    let fn_count = counts.iter().map(|c| c.fn_count).sum::<u64>();
    let decisions = answered * labels.len() as u64;
    assert_eq!(
        [
            hard["total"].as_u64().unwrap(),
            hard["answered"].as_u64().unwrap(),
            hard["abstained"].as_u64().unwrap(),
            hard["exact_matches"].as_u64().unwrap(),
            hard["wrong_sets"].as_u64().unwrap()
        ],
        [total, answered, total - answered, exact, answered - exact]
    );
    for (name, numerator) in [
        ("exact_match_accuracy", exact),
        ("wrong_set_rate", answered - exact),
        ("coverage", answered),
        ("abstention_rate", total - answered),
    ] {
        assert_direct_fraction(
            &hard[name],
            numerator,
            total,
            "no_data",
            total,
            "episode",
            "selected",
        );
    }
    for (name, numerator) in [
        ("selective_exact_match_accuracy", exact),
        ("selective_risk", answered - exact),
    ] {
        assert_direct_fraction(
            &hard[name],
            numerator,
            answered,
            answered_status,
            answered,
            "episode",
            "answered",
        );
    }
    let (macro_sum, undefined) =
        assert_multi_labels(hard, labels, counts, answered, answered_status);
    for (name, numerator, denominator) in [
        ("answered_micro_precision", tp, tp + fp),
        ("answered_micro_recall", tp, tp + fn_count),
        ("answered_micro_f1", 2 * tp, 2 * tp + fp + fn_count),
        ("answered_hamming_loss", fp + fn_count, decisions),
    ] {
        assert_direct_fraction(
            &hard[name],
            numerator,
            denominator,
            answered_status,
            decisions,
            "label_decision",
            "answered",
        );
    }
    assert_multi_macro(
        &hard["answered_macro_f1"],
        macro_sum,
        &undefined,
        labels.len(),
        answered,
        answered_status,
    );
}

fn assert_multi_labels(
    hard: &Value,
    labels: &[&str],
    counts: &[DirectLabel],
    answered: u64,
    answered_status: &str,
) -> (f64, Vec<String>) {
    let actual_labels = hard["labels"].as_array().unwrap();
    assert_eq!(actual_labels.len(), labels.len());
    let mut undefined = Vec::new();
    let mut macro_sum = 0.0;
    for ((label, count), actual) in labels.iter().zip(counts).zip(actual_labels) {
        assert_eq!(actual["label"], *label);
        for (field, expected) in [
            ("support", count.support),
            ("answered_support", count.answered_support),
            ("predicted_support", count.predicted_support),
            ("true_positive", count.tp),
            ("false_positive", count.fp),
            ("false_negative", count.fn_count),
            ("true_negative", count.tn),
        ] {
            assert_eq!(actual[field], expected, "{label}/{field}");
        }
        for (name, numerator, denominator) in [
            ("precision", count.tp, count.tp + count.fp),
            ("recall", count.tp, count.tp + count.fn_count),
            ("f1", 2 * count.tp, 2 * count.tp + count.fp + count.fn_count),
        ] {
            assert_direct_fraction(
                &actual[name],
                numerator,
                denominator,
                answered_status,
                answered,
                "label_decision",
                "answered",
            );
        }
        let denominator = 2 * count.tp + count.fp + count.fn_count;
        if denominator == 0 {
            undefined.push((*label).to_owned());
        } else {
            macro_sum += (2 * count.tp) as f64 / denominator as f64;
        }
    }
    (macro_sum, undefined)
}

fn assert_multi_macro(
    macro_result: &Value,
    macro_sum: f64,
    undefined: &[String],
    label_count: usize,
    answered: u64,
    answered_status: &str,
) {
    let decisions = answered * label_count as u64;
    let macro_metric = &macro_result["metric"];
    assert_eq!(macro_metric["population_count"], decisions);
    assert_eq!(macro_metric["population_unit"], "label_decision");
    assert_eq!(macro_metric["population_scope"], "answered");
    if answered == 0 {
        assert_eq!(macro_metric["status"], answered_status);
        assert_eq!(macro_metric["value"], Value::Null);
        assert_eq!(macro_result["undefined_classes"], json!([]));
    } else {
        assert_eq!(
            macro_metric["status"],
            if undefined.is_empty() {
                "defined"
            } else {
                "contains_undefined_classes"
            }
        );
        assert!(
            (macro_metric["value"].as_f64().unwrap() - macro_sum / label_count as f64).abs()
                <= validator::FIXTURE_ABSOLUTE_TOLERANCE
        );
        assert_eq!(macro_result["undefined_classes"], json!(undefined));
    }
}

fn assert_multi_episodes(report: &Value, labels: &[&str], rows: &[(u8, Option<u8>)]) {
    let names = |mask: u8| {
        labels
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, label)| *label)
            .collect::<Vec<_>>()
    };
    let episodes = report["episodes"].as_array().unwrap();
    assert_eq!(episodes.len(), rows.len());
    for (episode, &(expected, predicted)) in episodes.iter().zip(rows) {
        assert_eq!(episode["expected"]["labels"], json!(names(expected)));
        for family in ["raw", "final"] {
            assert_eq!(
                episode[format!("{family}_correct")],
                predicted == Some(expected)
            );
            let outcome = &episode[format!("{family}_outcome")];
            if let Some(predicted) = predicted {
                assert_eq!(outcome["type"], "labels");
                for (field, mask) in [
                    ("labels", predicted),
                    ("matched", expected & predicted),
                    ("missed", expected & !predicted),
                    ("extra", predicted & !expected),
                ] {
                    assert_eq!(outcome[field], json!(names(mask)));
                }
                assert_eq!(outcome["correct"], expected == predicted);
            } else {
                assert_eq!(outcome["type"], "abstention");
                assert_eq!(outcome["status"], "abstained");
                for field in ["matched", "missed", "extra"] {
                    assert!(outcome.as_object().unwrap().contains_key(field));
                    assert_eq!(outcome[field], Value::Null);
                }
            }
        }
    }
}

#[test]
fn exhaustive_multi_label() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/multi-label/expected.json")).unwrap();
    let mut answered_pairs = 0;
    let mut abstentions = 0;
    for expected in 0_u8..8 {
        for predicted in (0_u8..8).map(Some).chain(std::iter::once(None)) {
            let id = "01995c20-7d00-7000-8000-000000000001";
            let outcome = predicted.map_or_else(
                || json!({"type":"abstention","reason":"oracle"}),
                |mask| json!({"type":"labels","labels":label_names(mask)}),
            );
            let (directory, report) = policy_run(
                "multi_label",
                json!(["A", "B", "C"]),
                json!([{"id":id,"expected":{"type":"labels","labels":label_names(expected)},"input":null}]),
                json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
                json!([{"id":id,"source_id":"source","outcome":outcome}]),
                json!({"schema_version":2,"population":"exhaustive_multi_label","role":"development","decision":{"type":"as_recorded"}}),
            );
            assert_multi_direct_report(&report, &["A", "B", "C"], &[(expected, predicted)]);
            assert_eq!(
                report["integrity"]["categorical_normalization"],
                "not_applicable"
            );
            assert_eq!(report["final"], report["raw"]);
            fs::remove_dir_all(directory).unwrap();
            if predicted.is_some() {
                answered_pairs += 1;
            } else {
                abstentions += 1;
            }
        }
    }
    assert_eq!(answered_pairs, fixture["exhaustive"]["answered_pairs"]);
    assert_eq!(abstentions, fixture["exhaustive"]["whole_abstentions"]);
}

#[test]
fn multi_label_asymmetric_populations() {
    let report = multi_label_report_rows(
        &["C", "A", "B"],
        &[
            (&["A", "B"], Some(&["A"]), &[0.6, 0.8, 0.2]),
            (&[], Some(&[]), &[0.2, 0.0, 0.1]),
            (&["A", "C"], None, &[0.7, 0.9, 0.4]),
            (&["C"], Some(&["A"]), &[0.8, 0.3, 0.2]),
        ],
    );
    assert_multi_direct_report(
        &report,
        &["C", "A", "B"],
        &[(6, Some(2)), (0, Some(0)), (3, None), (1, Some(2))],
    );
    assert_valid("report", report);
    let empty = multi_label_report_rows(&["C", "A", "B"], &[]);
    assert_multi_direct_report(&empty, &["C", "A", "B"], &[]);
}

#[test]
fn multi_label_abstentions_preserve_wire_evidence_in_report() {
    let ids = [
        "01995c20-7d00-7000-8000-000000000101",
        "01995c20-7d00-7000-8000-000000000102",
    ];
    let (directory, report) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        json!([
            {"id":ids[0],"expected":{"type":"labels","labels":["A"]},"input":null},
            {"id":ids[1],"expected":{"type":"labels","labels":[]},"input":null}
        ]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{},"observation_definitions":{"review":{"kind":"label_marginals","description":"review"}}}}),
        json!([
            {"id":ids[0],"source_id":"source","outcome":{"type":"abstention","reason":"manual review"},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.1}},"observations":{"review":{"kind":"label_marginals","values":{"A":0.8,"B":0.1}}}},
            {"id":ids[1],"source_id":"source","outcome":{"type":"abstention"},"probabilities":{"kind":"label_marginals","values":{"A":0.2,"B":0.3}},"observations":{"review":{"kind":"label_marginals","values":{"A":0.2,"B":0.3}}}}
        ]),
        json!({"schema_version":2,"population":"abstention_evidence","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_valid("report", report.clone());
    for hard in [&report["raw"], &report["final"]] {
        for label in hard["labels"].as_array().unwrap() {
            for metric in [&label["precision"], &label["recall"], &label["f1"]] {
                assert_direct_fraction(
                    metric,
                    0,
                    0,
                    "no_answered_predictions",
                    0,
                    "label_decision",
                    "answered",
                );
            }
        }
        assert_eq!(hard["exact_match_accuracy"]["population_unit"], "episode");
        assert_eq!(hard["answered_micro_f1"]["population_count"], 0);
        assert_eq!(
            hard["answered_micro_f1"]["population_unit"],
            "label_decision"
        );
    }
    for (episode, reason) in report["episodes"]
        .as_array()
        .unwrap()
        .iter()
        .zip([json!("manual review"), Value::Null])
    {
        for outcome in ["raw_outcome", "final_outcome"] {
            let outcome = episode[outcome].as_object().unwrap();
            for field in ["reason", "status", "matched", "missed", "extra"] {
                assert!(outcome.contains_key(field), "{outcome:?} missing {field}");
            }
            assert_eq!(outcome["type"], "abstention");
            assert_eq!(outcome["reason"], reason);
            assert_eq!(outcome["status"], "abstained");
            assert_eq!(outcome["matched"], Value::Null);
            assert_eq!(outcome["missed"], Value::Null);
            assert_eq!(outcome["extra"], Value::Null);
        }
        assert!(episode["probability"]["marginals"].is_object());
        assert!(episode["observations"]["review"].is_object());
    }

    for field in ["reason", "status", "matched", "missed", "extra"] {
        let mut invalid = report.clone();
        invalid["episodes"][0]["raw_outcome"]
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert_invalid("report", invalid);
    }
    let mut invalid_status = report.clone();
    invalid_status["episodes"][0]["raw_outcome"]["status"] = json!("answered");
    assert_invalid("report", invalid_status);
    for field in ["matched", "missed", "extra"] {
        let mut invalid = report.clone();
        invalid["episodes"][0]["raw_outcome"][field] = json!([]);
        assert_invalid("report", invalid);
    }
    let mut bare_multi_abstention = report.clone();
    bare_multi_abstention["episodes"][0]["raw_outcome"] =
        json!({"type":"abstention","reason":"manual review"});
    assert_invalid("report", bare_multi_abstention);

    let single_abstention = api_report(
        json!([{"id":"01995c20-7d00-7000-8000-000000000201","source_id":"source","outcome":{"type":"abstention"}}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":"01995c20-7d00-7000-8000-000000000201","expected":{"type":"class","label":"A"},"input":null}]),
        json!({"schema_version":2,"population":"single_abstention","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_valid("report", single_abstention);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn single_label_literal_abstain_class_uses_a_typed_abstention_column() {
    let id = "01995c20-7d00-7000-8000-000000000301";
    let (directory, report) = policy_run(
        "single_label",
        json!(["A", "ABSTAIN"]),
        json!([{"id":id,"expected":{"type":"class","label":"ABSTAIN"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"ABSTAIN"}}]),
        json!({"schema_version":2,"population":"literal_abstain","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_valid("report", report.clone());
    assert_eq!(
        report["raw"]["matrix"]["columns"],
        json!([{"type":"label","label":"A"},{"type":"label","label":"ABSTAIN"},{"type":"abstention"}])
    );
    let mut missing_typed_abstention = report;
    missing_typed_abstention["raw"]["matrix"]["columns"][2] =
        json!({"type":"label","label":"ABSTAIN"});
    assert_invalid("report", missing_typed_abstention);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn categorical_loss_oracles() {
    let report = fixture_report(true, false);
    assert_eq!(report["probability"]["log_loss"]["status"], "defined");
    assert_eq!(report["probability"]["brier_score"]["status"], "defined");
    assert_eq!(report["probability"]["argmax_accuracy"]["value"], 1.0);
}

#[test]
fn signal_population_bins() {
    let report = fixture_report(true, true);
    assert_eq!(
        report["signals"]["maximum_probability"]["population_scope"],
        "selected"
    );
    assert_eq!(
        report["signals"]["confidence"]["population_scope"],
        "raw_answered"
    );
    assert_eq!(
        report["signals"]["maximum_probability"]["bins"]
            .as_array()
            .unwrap()
            .len(),
        10
    );
}

#[test]
fn bin_boundary_binary64() {
    let report = fixture_report(true, true);
    let bins = report["signals"]["maximum_probability"]["bins"]
        .as_array()
        .unwrap();
    assert_eq!(bins[8]["lower"], 0.8);
    assert_eq!(bins[8]["upper"], 0.9);
}

#[test]
fn report_sources_and_privacy() {
    let report = fixture_report(false, false);
    assert_eq!(report["source_counts"], json!({"source":2}));
    assert_eq!(report["composition"], "single_source");
    assert!(
        !serde_json::to_string(&report)
            .unwrap()
            .contains("opaque sentinel")
    );
    assert_valid("report", report);
}

#[test]
fn marginal_loss_and_bins() {
    let report = multi_label_report_rows(&["A", "B"], &[(&["A"], None, &[0.8, 0.7])]);
    assert_valid("report", report.clone());
    assert!(
        (report["probability"]["mean_binary_brier"]["value"]
            .as_f64()
            .unwrap()
            - 0.265)
            .abs()
            < 1e-12
    );
    assert!(
        (report["probability"]["mean_binary_log_loss"]["value"]
            .as_f64()
            .unwrap()
            - (-0.8_f64.ln() - (1.0_f64 - 0.7).ln()) / 2.0)
            .abs()
            < 1e-12
    );
    assert_eq!(
        report["signals"]["labels"][0]["bins"][8]["positive_count"],
        1
    );
    assert_eq!(
        report["probability"]["mean_binary_brier"]["population_count"],
        2
    );
    assert_eq!(
        report["probability"]["mean_binary_brier"]["population_unit"],
        "label_decision"
    );
    assert_eq!(
        report["probability"]["mean_binary_brier"]["population_scope"],
        "selected"
    );

    let absent_zero = multi_label_report_rows(&["A"], &[(&[], Some(&[]), &[0.0])]);
    assert_eq!(
        absent_zero["probability"]["labels"][0]["binary_log_loss"]["value"],
        0.0
    );
    assert_eq!(
        absent_zero["probability"]["mean_binary_log_loss"]["value"],
        0.0
    );
    assert_eq!(
        absent_zero["probability"]["mean_binary_log_loss"]["status"],
        "defined"
    );

    for (expected, probability) in [(&["A"][..], 0.0), (&[][..], 1.0)] {
        let infinite =
            multi_label_report_rows(&["A"], &[(expected, Some(expected), &[probability])]);
        for metric in [
            &infinite["probability"]["labels"][0]["binary_log_loss"],
            &infinite["probability"]["mean_binary_log_loss"],
        ] {
            assert_eq!(metric["status"], "positive_infinity");
            assert_eq!(metric["value"], Value::Null);
            assert_eq!(metric["special_value"], "+infinity");
        }
    }

    let one_label = multi_label_report_rows(&["A"], &[(&["A"], Some(&["A"]), &[0.8])]);
    assert!(
        (one_label["probability"]["mean_binary_brier"]["value"]
            .as_f64()
            .unwrap()
            - 0.04)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    let categorical = two_label_categorical_report();
    assert!(
        (categorical["probability"]["brier_score"]["value"]
            .as_f64()
            .unwrap()
            - 0.08)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );

    let bins = multi_label_report_rows(
        &["A"],
        &[
            (&[], Some(&[]), &[0.0]),
            (&[], Some(&[]), &[0.1]),
            (&["A"], Some(&["A"]), &[1.0]),
        ],
    );
    let bins = bins["signals"]["labels"][0]["bins"].as_array().unwrap();
    assert_eq!(bins[0]["lower"], 0.0);
    assert_eq!(bins[0]["upper"], 0.1);
    assert_eq!(bins[0]["count"], 1);
    assert_eq!(bins[0]["positive_count"], 0);
    assert_eq!(bins[1]["count"], 1);
    assert_eq!(bins[1]["positive_count"], 0);
    assert_eq!(bins[9]["upper"], 1.0);
    assert_eq!(bins[9]["upper_inclusive"], true);
    assert_eq!(bins[9]["count"], 1);
    assert_eq!(bins[9]["positive_count"], 1);
}

#[test]
fn multi_label_hard_oracles() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/multi-label/expected.json")).unwrap();
    let report = multi_label_report_rows(
        &["A", "B", "C"],
        &[
            (&["A", "B"], Some(&["A", "C"]), &[0.8, 0.7, 0.2]),
            (&[], Some(&[]), &[0.1, 0.1, 0.1]),
        ],
    );
    assert_valid("report", report.clone());
    let mut foreign_source = report.clone();
    foreign_source["sources"]["source"]["kind"] = json!("scored_choice");
    foreign_source["sources"]["source"]["question_id"] = json!("question");
    assert_invalid("report", foreign_source);
    assert_eq!(report["raw"]["total"], oracle["two_episode"]["N"]);
    assert_eq!(report["raw"]["answered"], oracle["two_episode"]["G"]);
    assert_eq!(report["raw"]["exact_matches"], 1);
    assert_eq!(report["raw"]["wrong_sets"], 1);
    assert_eq!(report["raw"]["abstained"], 0);
    assert_eq!(report["raw"]["exact_match_accuracy"]["numerator"], 1);
    assert_eq!(report["raw"]["exact_match_accuracy"]["denominator"], 2);
    assert_eq!(report["raw"]["answered_micro_f1"]["numerator"], 2);
    assert_eq!(report["raw"]["answered_micro_f1"]["denominator"], 4);
    assert_eq!(report["raw"]["answered_hamming_loss"]["numerator"], 2);
    assert_eq!(report["raw"]["answered_hamming_loss"]["denominator"], 6);
    assert_eq!(
        report["raw"]["answered_macro_f1"]["metric"]["value"],
        1.0 / 3.0
    );
    for metric in [
        &report["raw"]["answered_micro_f1"],
        &report["raw"]["answered_hamming_loss"],
        &report["raw"]["answered_macro_f1"]["metric"],
    ] {
        assert_eq!(metric["status"], "defined");
        assert_eq!(metric["population_scope"], "answered");
        assert_eq!(metric["population_unit"], "label_decision");
        assert_eq!(metric["population_count"], 6);
    }
    let aggregate = report["raw"]["labels"].as_array().unwrap().iter().fold(
        (0, 0, 0, 0),
        |(tp, fp, r#fn, tn), label| {
            (
                tp + label["true_positive"].as_u64().unwrap(),
                fp + label["false_positive"].as_u64().unwrap(),
                r#fn + label["false_negative"].as_u64().unwrap(),
                tn + label["true_negative"].as_u64().unwrap(),
            )
        },
    );
    assert_eq!(aggregate, (1, 1, 1, 3));
    assert_eq!(report["final"], report["raw"]);
    assert_eq!(
        report["episodes"][0]["raw_outcome"]["matched"],
        json!(["A"])
    );
    assert_eq!(report["episodes"][0]["raw_outcome"]["missed"], json!(["B"]));
    assert_eq!(report["episodes"][0]["raw_outcome"]["extra"], json!(["C"]));

    let empty_answer = multi_label_report_rows(&["A", "B"], &[(&[], Some(&[]), &[0.1, 0.1])]);
    assert_eq!(empty_answer["raw"]["answered"], 1);
    assert_eq!(empty_answer["raw"]["exact_matches"], 1);
    assert_eq!(empty_answer["episodes"][0]["raw_outcome"]["type"], "labels");
    assert_eq!(
        empty_answer["episodes"][0]["raw_outcome"]["matched"],
        json!([])
    );
    assert_eq!(
        empty_answer["episodes"][0]["raw_outcome"]["missed"],
        json!([])
    );
    assert_eq!(
        empty_answer["episodes"][0]["raw_outcome"]["extra"],
        json!([])
    );

    let abstention =
        multi_label_report_rows(&["A"], &[(&["A"], None, &[0.8]), (&[], Some(&[]), &[0.1])]);
    assert_eq!(abstention["raw"]["total"], 2);
    assert_eq!(abstention["raw"]["answered"], 1);
    assert_eq!(abstention["raw"]["abstained"], 1);
    assert_eq!(abstention["raw"]["coverage"]["numerator"], 1);
    assert_eq!(abstention["raw"]["coverage"]["denominator"], 2);
    assert_eq!(
        abstention["episodes"][0]["raw_outcome"]["type"],
        "abstention"
    );
    assert_eq!(
        abstention["episodes"][0]["raw_outcome"]["matched"],
        Value::Null
    );
    assert_eq!(
        abstention["episodes"][0]["raw_outcome"]["missed"],
        Value::Null
    );
    assert_eq!(
        abstention["episodes"][0]["raw_outcome"]["extra"],
        Value::Null
    );

    let all_abstained = multi_label_report_rows(&["A"], &[(&["A"], None, &[0.8])]);
    assert_eq!(
        all_abstained["raw"]["answered_micro_f1"]["status"],
        "no_answered_predictions"
    );
    assert_eq!(
        all_abstained["raw"]["answered_hamming_loss"]["status"],
        "no_answered_predictions"
    );
    let no_data = multi_label_report_rows(&["A"], &[]);
    assert_eq!(no_data["raw"]["exact_match_accuracy"]["status"], "no_data");
    assert_eq!(no_data["raw"]["answered_micro_f1"]["status"], "no_data");

    let mut missing_metric_field = report.clone();
    missing_metric_field["raw"]["answered_micro_f1"]
        .as_object_mut()
        .unwrap()
        .remove("population_unit");
    assert_invalid("report", missing_metric_field);
    let mut foreign_hard_field = report.clone();
    foreign_hard_field["raw"]["matrix"] = json!({});
    assert_invalid("report", foreign_hard_field);
    let mut mixed_family = report.clone();
    mixed_family["probability"]["kind"] = json!("single_label");
    assert_invalid("report", mixed_family);
    let mut invalid_infinity = report.clone();
    invalid_infinity["probability"]["mean_binary_log_loss"]["status"] = json!("positive_infinity");
    invalid_infinity["probability"]["mean_binary_log_loss"]["value"] = json!(0.0);
    assert_invalid("report", invalid_infinity);
    let mut fabricated_answer = report;
    fabricated_answer["episodes"][0]["raw_outcome"]["matched"] = Value::Null;
    assert_invalid("report", fabricated_answer);
    let mut single_policy_on_multi = multi_label_report_rows(&["A"], &[(&[], Some(&[]), &[0.1])]);
    single_policy_on_multi["policy"]["decision"] =
        json!({"type": "reject_below", "signal": "confidence", "minimum": 0.5});
    assert_invalid("report", single_policy_on_multi);
}

#[test]
fn equal_counts_distinct_exact_sets() {
    let exact = multi_label_report_rows(
        &["A", "B"],
        &[
            (&["A", "B"], Some(&["A", "B"]), &[0.8, 0.8]),
            (&[], Some(&["A", "B"]), &[0.8, 0.8]),
        ],
    );
    let split = multi_label_report_rows(
        &["A", "B"],
        &[
            (&["A"], Some(&["A", "B"]), &[0.8, 0.8]),
            (&["B"], Some(&["A", "B"]), &[0.8, 0.8]),
        ],
    );
    let counts = |report: &Value| {
        report["raw"]["labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|label| {
                json!([
                    label["true_positive"],
                    label["false_positive"],
                    label["false_negative"],
                    label["true_negative"],
                ])
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(counts(&exact), counts(&split));
    assert_eq!(exact["raw"]["exact_match_accuracy"]["numerator"], 1);
    assert_eq!(exact["raw"]["exact_match_accuracy"]["denominator"], 2);
    assert_eq!(split["raw"]["exact_match_accuracy"]["numerator"], 0);
    assert_eq!(split["raw"]["exact_match_accuracy"]["denominator"], 2);
    assert_eq!(exact["episodes"][0]["raw_outcome"]["correct"], true);
    assert_eq!(split["episodes"][0]["raw_outcome"]["correct"], false);
}

fn multi_label_report_rows(labels: &[&str], rows: &[MultiLabelReportRow<'_>]) -> Value {
    let directory = std::env::temp_dir().join(format!(
        "validator-ml-oracle-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let ids = (1..=rows.len())
        .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
        .collect::<Vec<_>>();
    let golden = directory.join("golden.json");
    let predictions = directory.join("predictions.json");
    let config = directory.join("config.json");
    let bytes = serde_json::to_vec(&json!({
        "schema_version":2,
        "task":{"kind":"multi_label","labels":labels},
        "episodes": rows.iter().zip(&ids).map(|((expected, _, _), id)| json!({
            "id": id,
            "expected":{"type":"labels","labels":expected},
            "input":"opaque payload never appears in a report",
        })).collect::<Vec<_>>(),
    }))
    .unwrap();
    let digest = format!("{:x}", Sha256::digest(&bytes));
    fs::write(&golden, bytes).unwrap();
    fs::write(&predictions, serde_json::to_vec(&json!({
        "schema_version":2,
        "dataset_sha256":digest,
        "sources":{"source":{"kind":"classifier","model":"m","configuration":{}}},
        "predictions": rows.iter().zip(&ids).map(|((_, predicted, marginals), id)| {
            let values = labels.iter().zip(*marginals).collect::<std::collections::BTreeMap<_, _>>();
            let outcome = predicted.map_or_else(
                || json!({"type":"abstention","reason":"review"}),
                |labels| json!({"type":"labels","labels":labels}),
            );
            json!({"id":id,"source_id":"source","outcome":outcome,"probabilities":{"kind":"label_marginals","values":values}})
        }).collect::<Vec<_>>(),
    })).unwrap()).unwrap();
    fs::write(&config, b"{\"schema_version\":2,\"population\":\"all\",\"role\":\"development\",\"decision\":{\"type\":\"as_recorded\"}}").unwrap();
    let receipt = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: directory.join("run"),
    })
    .unwrap();
    let receipt = serde_json::to_value(receipt).unwrap();
    let report =
        serde_json::from_slice(&fs::read(receipt["result_path"].as_str().unwrap()).unwrap())
            .unwrap();
    fs::remove_dir_all(directory).unwrap();
    report
}

fn two_label_categorical_report() -> Value {
    let directory = std::env::temp_dir().join(format!(
        "validator-categorical-brier-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let golden = directory.join("golden.json");
    let predictions = directory.join("predictions.json");
    let config = directory.join("config.json");
    let id = "01995c20-7d00-7000-8000-000000000001";
    let bytes = serde_json::to_vec(&json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":id,"expected":{"type":"class","label":"A"},"input":null}]})).unwrap();
    let digest = format!("{:x}", Sha256::digest(&bytes));
    fs::write(&golden, bytes).unwrap();
    fs::write(&predictions, serde_json::to_vec(&json!({"schema_version":2,"dataset_sha256":digest,"sources":{"source":{"kind":"classifier","model":"m","configuration":{}}},"predictions":[{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.8,"B":0.2}}}]})).unwrap()).unwrap();
    fs::write(&config, b"{\"schema_version\":2,\"population\":\"all\",\"role\":\"development\",\"decision\":{\"type\":\"as_recorded\"}}").unwrap();
    let receipt = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: directory.join("run"),
    })
    .unwrap();
    let receipt = serde_json::to_value(receipt).unwrap();
    let report =
        serde_json::from_slice(&fs::read(receipt["result_path"].as_str().unwrap()).unwrap())
            .unwrap();
    fs::remove_dir_all(directory).unwrap();
    report
}

#[test]
fn case_s13() {
    let id = "01995c20-7d00-7000-8000-000000000901";
    let episodes = json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]);
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{}}});
    let predictions =
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]);
    let config = json!({"schema_version":2,"population":"case_s13","role":"development","decision":{"type":"as_recorded"}});

    let mut invalid_uuid = episodes.clone();
    invalid_uuid[0]["id"] = json!("not-a-uuid");
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        invalid_uuid,
        sources.clone(),
        predictions.clone(),
        config.clone(),
        validator::DiagnosticCode::Id,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        json!([episodes[0].clone(), episodes[0].clone()]),
        sources.clone(),
        predictions.clone(),
        config.clone(),
        validator::DiagnosticCode::DuplicateId,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episodes.clone(),
        sources.clone(),
        json!([predictions[0].clone(), predictions[0].clone()]),
        config.clone(),
        validator::DiagnosticCode::DuplicateId,
    );

    let mut unknown_class = predictions.clone();
    unknown_class[0]["outcome"]["label"] = json!("UNKNOWN");
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episodes.clone(),
        sources.clone(),
        unknown_class,
        config.clone(),
        validator::DiagnosticCode::Label,
    );
    let mut unknown_field = predictions.clone();
    unknown_field[0]["unexpected"] = json!(true);
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episodes.clone(),
        sources.clone(),
        unknown_field,
        config.clone(),
        validator::DiagnosticCode::Schema,
    );
    let mut missing_input = episodes.clone();
    missing_input[0].as_object_mut().unwrap().remove("input");
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        missing_input,
        sources.clone(),
        predictions.clone(),
        config.clone(),
        validator::DiagnosticCode::Schema,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "A"]),
        episodes.clone(),
        sources.clone(),
        predictions.clone(),
        config.clone(),
        validator::DiagnosticCode::Label,
    );

    let second = "01995c20-7d00-7000-8000-000000000902";
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        json!([episodes[0].clone(), {"id":second,"expected":{"type":"class","label":"B"},"input":null}]),
        sources,
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.8,"B":0.2}}},{"id":second,"source_id":"source","outcome":{"type":"class","label":"B"}}]),
        config,
        validator::DiagnosticCode::Probability,
    );
}

#[test]
fn case_s14() {
    let id = "01995c20-7d00-7000-8000-000000000911";
    let other = "01995c20-7d00-7000-8000-000000000912";
    let episodes = json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]);
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{}}});
    let config = json!({"schema_version":2,"population":"case_s14","role":"development","decision":{"type":"as_recorded"}});
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episodes.clone(),
        sources.clone(),
        json!([]),
        config.clone(),
        validator::DiagnosticCode::Alignment,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episodes.clone(),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}},{"id":other,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        config.clone(),
        validator::DiagnosticCode::Alignment,
    );
    let (directory, golden, prediction, configuration) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        episodes,
        sources,
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        config,
    );
    let mut artifact: Value = serde_json::from_slice(&fs::read(&prediction).unwrap()).unwrap();
    artifact["dataset_sha256"] = json!("0".repeat(64));
    fs::write(&prediction, serde_json::to_vec(&artifact).unwrap()).unwrap();
    let output = directory.join("run");
    let error = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions: prediction,
        config: configuration,
        output: output.clone(),
    })
    .unwrap_err();
    assert_eq!(error.code(), validator::DiagnosticCode::Provenance);
    assert!(!output.exists());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_s17() {
    let ids = [
        "01995c20-7d00-7000-8000-000000000921",
        "01995c20-7d00-7000-8000-000000000922",
    ];
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{}}});
    let config = json!({"schema_version":2,"population":"case_s17","role":"development","decision":{"type":"as_recorded"}});
    let rows = |reversed: bool| {
        let order = if reversed { [1, 0] } else { [0, 1] };
        json!(order.into_iter().map(|index| json!({"id":ids[index],"expected":{"type":"class","label":if index == 0 { "A" } else { "B" }},"input":null})).collect::<Vec<_>>())
    };
    let predictions = |reversed: bool| {
        let order = if reversed { [1, 0] } else { [0, 1] };
        json!(order.into_iter().map(|index| {
            let values = if reversed { json!({"B":if index == 0 { 0.1 } else { 0.9 },"A":if index == 0 { 0.9 } else { 0.1 }}) } else { json!({"A":if index == 0 { 0.9 } else { 0.1 },"B":if index == 0 { 0.1 } else { 0.9 }}) };
            json!({"id":ids[index],"source_id":"source","outcome":{"type":"class","label":if index == 0 { "A" } else { "B" }},"probabilities":{"kind":"categorical","values":values}})
        }).collect::<Vec<_>>())
    };
    let (first_directory, first) = policy_run(
        "single_label",
        json!(["A", "B"]),
        rows(false),
        sources.clone(),
        predictions(false),
        config.clone(),
    );
    let (second_directory, second) = policy_run(
        "single_label",
        json!(["A", "B"]),
        rows(true),
        sources,
        predictions(true),
        config,
    );
    assert_eq!(first["raw"], second["raw"]);
    assert_eq!(first["final"], second["final"]);
    assert_eq!(first["probability"], second["probability"]);
    assert_eq!(first["episodes"], second["episodes"]);
    assert_eq!(first["episodes"][0]["id"], ids[0]);
    assert_ne!(
        fs::read(first_directory.join("golden.json")).unwrap(),
        fs::read(second_directory.join("golden.json")).unwrap()
    );
    fs::remove_dir_all(first_directory).unwrap();
    fs::remove_dir_all(second_directory).unwrap();
}

#[test]
fn case_s21() {
    let id = "01995c20-7d00-7000-8000-000000000931";
    let (directory, report) = policy_run(
        "single_label",
        json!(["NO_MATCH", "UNCERTAIN", "MATCH"]),
        json!([{"id":id,"expected":{"type":"class","label":"UNCERTAIN"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"UNCERTAIN"}}]),
        json!({"schema_version":2,"population":"case_s21","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_eq!(report["episodes"][0]["expected"]["label"], "UNCERTAIN");
    assert_eq!(report["status"], "complete");
    fs::remove_dir_all(directory).unwrap();
    assert_invalid(
        "golden",
        json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":id,"expected":null,"input":null}]}),
    );
    assert_invalid(
        "golden",
        json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":id,"input":null}]}),
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"UNDECLARED"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        json!({"schema_version":2,"population":"case_s21","role":"development","decision":{"type":"as_recorded"}}),
        validator::DiagnosticCode::Label,
    );
}

#[test]
fn case_s29() {
    let report = fixture_report(true, true);
    assert_valid("report", report);
    assert_valid(
        "receipt",
        json!({"schema_version":2,"kind":"evaluation","status":"complete","run_id":"01995c20-7d00-7000-8000-000000000941","result_path":"/tmp/report.json","result_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"}),
    );
    assert_valid(
        "error",
        serde_json::from_str(
            &validator::Diagnostic::for_code(validator::DiagnosticCode::Schema)
                .to_machine_json()
                .unwrap(),
        )
        .unwrap(),
    );
}

#[test]
fn case_m16() {
    let id = "01995c20-7d00-7000-8000-000000000951";
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{}}});
    let config = json!({"schema_version":2,"population":"case_m16","role":"development","decision":{"type":"as_recorded"}});
    let episode = json!([{"id":id,"expected":{"type":"labels","labels":[]},"input":null}]);
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        sources.clone(),
        json!([]),
        config.clone(),
        validator::DiagnosticCode::Alignment,
    );
    let (directory, report) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":[]},"probabilities":{"kind":"label_marginals","values":{"A":0.1,"B":0.1}}}]),
        config.clone(),
    );
    assert_eq!(report["raw"]["answered"], 1);
    assert_eq!(report["episodes"][0]["raw_outcome"]["type"], "labels");
    fs::remove_dir_all(directory).unwrap();
    let (directory, abstained) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"abstention"},"probabilities":{"kind":"label_marginals","values":{"A":0.1,"B":0.1}}}]),
        config.clone(),
    );
    assert_eq!(abstained["raw"]["abstained"], 1);
    assert_eq!(
        abstained["episodes"][0]["raw_outcome"]["type"],
        "abstention"
    );
    fs::remove_dir_all(directory).unwrap();
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode,
        sources,
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":[]},"probabilities":{"kind":"label_marginals","values":{"A":0.1}}}]),
        config,
        validator::DiagnosticCode::Probability,
    );
}

#[test]
fn case_m21() {
    let id = "01995c20-7d00-7000-8000-000000000961";
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{}}});
    let config = json!({"schema_version":2,"population":"case_m21","role":"development","decision":{"type":"as_recorded"}});
    let episode = json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]);
    let (directory, unreviewed) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.2}}}]),
        config.clone(),
    );
    assert_eq!(unreviewed["status"], "complete");
    fs::remove_dir_all(directory).unwrap();
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":{"A":1}},"input":null}]),
        sources.clone(),
        json!([]),
        config.clone(),
        validator::DiagnosticCode::Schema,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        config.clone(),
        validator::DiagnosticCode::Config,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"abstention","labels":["A"]}}]),
        config.clone(),
        validator::DiagnosticCode::Schema,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"reference_mask":[true,false]}]),
        config,
        validator::DiagnosticCode::Schema,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode,
        sources,
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]}}]),
        json!({"schema_version":1,"population":"case_m21","role":"development","decision":{"type":"as_recorded"}}),
        validator::DiagnosticCode::Schema,
    );
}

#[test]
fn case_e05() {
    let id = "01995c20-7d00-7000-8000-000000000971";
    let labels = json!(["NO_MATCH", "UNCERTAIN", "MATCH"]);
    let episodes = json!([{"id":id,"expected":{"type":"class","label":"UNCERTAIN"},"input":null}]);
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{},"evidence":["prepared.bin"],"observation_definitions":{"scalar":{"kind":"scalar","description":"native scalar"},"distribution":{"kind":"categorical","description":"displayed vector"}},"preparation":{"method":"native","version":"1","configuration":{},"evidence_indices":[0]}}});
    let mut row = json!({"id":id,"source_id":"source","outcome":{"type":"class","label":"UNCERTAIN"},"observations":{"scalar":{"kind":"scalar","value":1.33},"distribution":{"kind":"categorical","values":{"NO_MATCH":0.24,"UNCERTAIN":0.19,"MATCH":0.57}}}});
    let config = json!({"schema_version":2,"population":"case_e05","role":"development","decision":{"type":"as_recorded"}});
    let evaluate = |prediction_row: Value| {
        let (directory, golden, prediction, configuration) = write_policy_inputs(
            "single_label",
            labels.clone(),
            episodes.clone(),
            sources.clone(),
            json!([prediction_row]),
            config.clone(),
        );
        fs::write(directory.join("prepared.bin"), b"prepared native outcome").unwrap();
        let receipt = validator::evaluate(validator::EvaluationOptions {
            dataset: golden,
            predictions: prediction,
            config: configuration,
            output: directory.join("run"),
        })
        .unwrap();
        let receipt = serde_json::to_value(receipt).unwrap();
        let report: Value =
            serde_json::from_slice(&fs::read(receipt["result_path"].as_str().unwrap()).unwrap())
                .unwrap();
        (directory, report)
    };
    let (directory, retained) = evaluate(row.clone());
    assert_eq!(retained["episodes"][0]["raw_outcome"]["label"], "UNCERTAIN");
    assert_eq!(
        retained["episodes"][0]["observations"]["scalar"]["value"],
        1.33
    );
    assert_eq!(
        retained["probability"]["log_loss"]["status"],
        "not_applicable"
    );
    fs::remove_dir_all(directory).unwrap();
    row["probabilities"] =
        json!({"kind":"categorical","values":{"NO_MATCH":0.24,"UNCERTAIN":0.19,"MATCH":0.57}});
    let (directory, scored) = evaluate(row);
    assert_eq!(scored["probability"]["log_loss"]["status"], "defined");
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_e06() {
    let id = "01995c20-7d00-7000-8000-000000000981";
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{},"evidence":["evidence.bin"],"observation_definitions":{"distribution":{"kind":"categorical","description":"retained"}},"preparation":{"method":"native","version":"1","configuration":{},"evidence_indices":[0]}}});
    let (directory, golden, prediction, configuration) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        sources,
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"distribution":{"kind":"categorical","values":{"A":0.5,"B":0.49}}}}]),
        json!({"schema_version":2,"population":"case_e06","role":"development","decision":{"type":"as_recorded"}}),
    );
    fs::write(directory.join("evidence.bin"), b"prepared native evidence").unwrap();
    let receipt = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions: prediction,
        config: configuration,
        output: directory.join("run"),
    })
    .unwrap();
    let report: Value = serde_json::from_slice(
        &fs::read(
            serde_json::to_value(receipt).unwrap()["result_path"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        report["episodes"][0]["observations"]["distribution"]["values"],
        json!({"A":0.5,"B":0.49})
    );
    assert_eq!(
        report["probability"]["log_loss"]["status"],
        "not_applicable"
    );
    assert_eq!(
        report["sources"]["source"]["preparation"]["method"],
        "native"
    );
    assert!(
        report["artifacts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|artifact| artifact["kind"] == "evidence")
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_e09() {
    let id = "01995c20-7d00-7000-8000-000000000991";
    let (directory, report) = policy_run(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"observation_definitions":{"returned":{"kind":"scalar","description":"returned scalar"},"displayed":{"kind":"categorical","description":"displayed vector"}}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"returned":{"kind":"scalar","value":0.2},"displayed":{"kind":"categorical","values":{"A":0.9,"B":0.1}}}}]),
        json!({"schema_version":2,"population":"case_e09","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_eq!(report["status"], "complete");
    assert_eq!(
        report["episodes"][0]["observations"]["returned"]["value"],
        0.2
    );
    assert_eq!(
        report["episodes"][0]["observations"]["displayed"]["values"],
        json!({"A":0.9,"B":0.1})
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_e10() {
    let id = "01995c20-7d00-7000-8000-000000001001";
    let (directory, auxiliary_only) = policy_run(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"observation_definitions":{"auxiliary":{"kind":"bernoulli","description":"no gold"}}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"auxiliary":{"kind":"bernoulli","value":0.8}}}]),
        json!({"schema_version":2,"population":"case_e10","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_eq!(
        auxiliary_only["episodes"][0]["observations"]["auxiliary"]["value"],
        0.8
    );
    assert_eq!(
        auxiliary_only["probability"]["log_loss"]["status"],
        "not_applicable"
    );
    fs::remove_dir_all(directory).unwrap();
    let scored = multi_label_report_rows(&["A", "B"], &[(&["A"], Some(&["A"]), &[0.8, 0.7])]);
    assert_eq!(
        scored["probability"]["mean_binary_brier"]["status"],
        "defined"
    );
    assert!(
        (scored["probability"]["mean_binary_brier"]["value"]
            .as_f64()
            .unwrap()
            - 0.265)
            .abs()
            < 1e-12
    );
}

#[test]
fn case_e11() {
    let id = "01995c20-7d00-7000-8000-000000001011";
    let episode = json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]);
    let config = json!({"schema_version":2,"population":"case_e11","role":"development","decision":{"type":"as_recorded"}});
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episode.clone(),
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"observation_definitions":{"known":{"kind":"scalar","description":"known"}}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"unknown":{"kind":"scalar","value":1.0}}}]),
        config.clone(),
        validator::DiagnosticCode::Provenance,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episode.clone(),
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"observation_definitions":{"bounded":{"kind":"bernoulli","description":"bounded"}}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"bounded":{"kind":"bernoulli","value":2.0}}}]),
        config.clone(),
        validator::DiagnosticCode::Observation,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episode.clone(),
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"observation_definitions":{"known":{"kind":"scalar","description":"known"}}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"known":{"kind":"unknown","value":1.0}}}]),
        config.clone(),
        validator::DiagnosticCode::Schema,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episode.clone(),
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"evidence":["missing.bin"],"preparation":{"method":"prepared","version":"1","configuration":{},"evidence_indices":[1]}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"}}]),
        config.clone(),
        validator::DiagnosticCode::Provenance,
    );
    let (directory, golden, prediction, configuration) = write_policy_inputs(
        "single_label",
        json!(["A", "B"]),
        episode,
        json!({"source":{"kind":"classifier","model":"m","configuration":{},"observation_definitions":{"scalar":{"kind":"scalar","description":"finite"}}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"scalar":{"kind":"scalar","value":0.0}}}]),
        config,
    );
    let mutated =
        fs::read_to_string(&prediction)
            .unwrap()
            .replacen("\"value\":0.0", "\"value\":1e400", 1);
    fs::write(&prediction, mutated).unwrap();
    let output = directory.join("run");
    let error = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions: prediction,
        config: configuration,
        output: output.clone(),
    })
    .unwrap_err();
    assert_eq!(error.code(), validator::DiagnosticCode::Observation);
    assert!(!output.exists());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_e12() {
    let ids = [
        "01995c20-7d00-7000-8000-000000001021",
        "01995c20-7d00-7000-8000-000000001022",
    ];
    let sources = json!({"source":{"kind":"classifier","model":"m","configuration":{},"observation_definitions":{"auxiliary":{"kind":"scalar","description":"optional"}}}});
    let config = json!({"schema_version":2,"population":"case_e12","role":"development","decision":{"type":"as_recorded"}});
    let episodes =
        json!(ids.map(|id| json!({"id":id,"expected":{"type":"class","label":"A"},"input":null})));
    let mut rows = json!([{"id":ids[0],"source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"auxiliary":{"kind":"scalar","value":1.0}}},{"id":ids[1],"source_id":"source","outcome":{"type":"class","label":"A"}}]);
    let (directory, report) = policy_run(
        "single_label",
        json!(["A", "B"]),
        episodes.clone(),
        sources.clone(),
        rows.clone(),
        config.clone(),
    );
    assert_eq!(
        report["episodes"][0]["observations"]["auxiliary"]["value"],
        1.0
    );
    assert_eq!(report["episodes"][1]["observations"], json!({}));
    fs::remove_dir_all(directory).unwrap();
    rows[0]["probabilities"] = json!({"kind":"categorical","values":{"A":0.8,"B":0.2}});
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        episodes,
        sources,
        rows,
        config,
        validator::DiagnosticCode::Probability,
    );
}

#[test]
fn case_e13() {
    let id = "01995c20-7d00-7000-8000-000000001031";
    let base = || {
        write_policy_inputs(
            "single_label",
            json!(["A", "B"]),
            json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
            json!({"source":{"kind":"classifier","model":"m","configuration":{}}}),
            json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.8,"B":0.2}}}]),
            json!({"schema_version":2,"population":"case_e13","role":"development","decision":{"type":"as_recorded"}}),
        )
    };
    for field in ["probabilities", "observations"] {
        let (directory, golden, prediction, configuration) = base();
        let mut document: Value = serde_json::from_slice(&fs::read(&prediction).unwrap()).unwrap();
        document["predictions"][0]
            .as_object_mut()
            .unwrap()
            .remove("outcome");
        if field == "observations" {
            document["predictions"][0]
                .as_object_mut()
                .unwrap()
                .remove("probabilities");
            document["predictions"][0]["observations"] =
                json!({"scalar":{"kind":"scalar","value":1.0}});
        }
        fs::write(&prediction, serde_json::to_vec(&document).unwrap()).unwrap();
        let output = directory.join("run");
        let error = validator::evaluate(validator::EvaluationOptions {
            dataset: golden,
            predictions: prediction,
            config: configuration,
            output: output.clone(),
        })
        .unwrap_err();
        assert_eq!(error.code(), validator::DiagnosticCode::Schema);
        assert!(!output.exists());
        fs::remove_dir_all(directory).unwrap();
    }
}

#[test]
fn case_s01() {
    let ids = (1..=3)
        .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
        .collect::<Vec<_>>();
    let report = api_report(
        json!(ids.iter().zip(["A", "B", "C"]).map(|(id, label)| json!({"id":id,"source_id":"source","outcome":{"type":"class","label":label}})).collect::<Vec<_>>()),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!(ids.iter().zip(["A", "B", "C"]).map(|(id, label)| json!({"id":id,"expected":{"type":"class","label":label},"input":null})).collect::<Vec<_>>()),
        json!({"schema_version":2,"population":"case_s01","role":"development","decision":{"type":"as_recorded"}}),
    );
    let hard = &report["raw"];
    assert_eq!(
        hard["matrix"]["rows"],
        json!([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0]])
    );
    assert_eq!(hard["total"], 3);
    assert_eq!(hard["correct"], 3);
    assert_eq!(hard["wrong"], 0);
    assert_eq!(hard["abstained"], 0);
    assert_eq!(hard["answered"], 3);
    for class in hard["classes"].as_array().unwrap() {
        assert_eq!(class["support"], 1);
        assert_eq!(class["predicted_support"], 1);
        assert_direct_fraction(
            &class["f1"],
            2,
            2,
            "undefined_zero_denominator",
            3,
            "episode",
            "selected",
        );
    }
    assert_direct_fraction(&hard["accuracy"], 3, 3, "no_data", 3, "episode", "selected");
    assert_eq!(hard["macro_f1"]["metric"]["status"], "defined");
    assert_eq!(hard["macro_f1"]["metric"]["population_count"], 3);
    assert_eq!(hard["macro_f1"]["metric"]["population_unit"], "episode");
    assert_eq!(hard["macro_f1"]["metric"]["population_scope"], "selected");
    assert_eq!(hard["macro_f1"]["metric"]["value"], 1.0);
}

#[test]
fn case_s02() {
    let id = "01995c20-7d00-7000-8000-000000002002";
    let report = api_report(
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"B"}}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"schema_version":2,"population":"case_s02","role":"development","decision":{"type":"as_recorded"}}),
    );
    let hard = &report["raw"];
    assert_eq!(
        hard["matrix"]["rows"],
        json!([[0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]])
    );
    assert_eq!(hard["classes"][0]["false_negative"], 1);
    assert_eq!(hard["classes"][1]["false_positive"], 1);
    assert_eq!(hard["correct"], 0);
    assert_eq!(hard["wrong"], 1);
    assert_eq!(hard["abstained"], 0);
    assert_eq!(hard["answered"], 1);
    assert_eq!(report["episodes"].as_array().unwrap().len(), 1);
    assert_eq!(report["episodes"][0]["id"], id);
    assert_eq!(report["episodes"][0]["raw_correct"], false);
    assert_eq!(
        report["episodes"][0]["raw_outcome"],
        json!({"type":"class","label":"B"})
    );
}

#[test]
fn case_s03() {
    let ids = (1..=4)
        .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
        .collect::<Vec<_>>();
    let (directory, report) = policy_run(
        "single_label",
        json!(["A", "B", "C"]),
        json!(ids.iter().zip(["A", "A", "B", "C"]).map(|(id, label)| json!({"id":id,"expected":{"type":"class","label":label},"input":null})).collect::<Vec<_>>()),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!(ids.iter().zip([("A", 0.8), ("A", 0.4), ("A", 0.8), ("C", 0.8)]).map(|(id, (label, confidence))| json!({"id":id,"source_id":"source","outcome":{"type":"class","label":label},"confidence":confidence})).collect::<Vec<_>>()),
        json!({"schema_version":2,"population":"case_s03","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":0.5}}),
    );
    let hard = &report["final"];
    assert_eq!(hard["total"], 4);
    assert_eq!(hard["correct"], 2);
    assert_eq!(hard["wrong"], 1);
    assert_eq!(hard["abstained"], 1);
    assert_eq!(hard["answered"], 3);
    assert_direct_fraction(&hard["accuracy"], 2, 4, "no_data", 4, "episode", "selected");
    assert_direct_fraction(&hard["coverage"], 3, 4, "no_data", 4, "episode", "selected");
    assert_direct_fraction(
        &hard["selective_accuracy"],
        2,
        3,
        "no_answered_predictions",
        3,
        "episode",
        "answered",
    );
    for (class, (numerator, denominator)) in
        hard["classes"]
            .as_array()
            .unwrap()
            .iter()
            .zip([(1, 2), (1, 1), (1, 1)])
    {
        assert_direct_fraction(
            &class["coverage"],
            numerator,
            denominator,
            "undefined_zero_denominator",
            4,
            "episode",
            "selected",
        );
    }
    assert_eq!(hard["macro_f1"]["metric"]["status"], "defined");
    assert!(
        (hard["macro_f1"]["metric"]["value"].as_f64().unwrap() - 0.5).abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert_eq!(hard["accuracy"]["numerator"], 2);
    assert_eq!(hard["accuracy"]["denominator"], 4);
    assert_eq!(hard["coverage"]["numerator"], 3);
    assert_eq!(hard["coverage"]["denominator"], 4);
    assert_eq!(hard["selective_accuracy"]["numerator"], 2);
    assert_eq!(hard["selective_accuracy"]["denominator"], 3);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_s04() {
    let report = api_report(
        json!([{"id":"01995c20-7d00-7000-8000-000000002004","source_id":"source","outcome":{"type":"class","label":"A"}}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":"01995c20-7d00-7000-8000-000000002004","expected":{"type":"class","label":"A"},"input":null}]),
        json!({"schema_version":2,"population":"case_s04","role":"development","decision":{"type":"as_recorded"}}),
    );
    let hard = &report["raw"];
    assert_direct_fraction(&hard["accuracy"], 1, 1, "no_data", 1, "episode", "selected");
    for class in &hard["classes"].as_array().unwrap()[1..] {
        assert_eq!(class["f1"]["value"], Value::Null);
        assert_eq!(class["f1"]["status"], "undefined_zero_denominator");
    }
    assert_eq!(
        hard["macro_f1"]["metric"]["status"],
        "contains_undefined_classes"
    );
    assert_eq!(hard["macro_f1"]["undefined_classes"], json!(["B", "C"]));
    assert!(
        (hard["macro_f1"]["metric"]["value"].as_f64().unwrap() - 1.0 / 3.0).abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
}

#[test]
fn case_s05() {
    let ids = [
        "01995c20-7d00-7000-8000-000000002051",
        "01995c20-7d00-7000-8000-000000002052",
    ];
    let report = api_report(
        json!(ids.map(|id| json!({"id":id,"source_id":"source","outcome":{"type":"abstention","reason":"review"}}))),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!(ids.map(|id| json!({"id":id,"expected":{"type":"class","label":"A"},"input":null}))),
        json!({"schema_version":2,"population":"case_s05","role":"development","decision":{"type":"as_recorded"}}),
    );
    let hard = &report["raw"];
    assert_eq!(hard["total"], 2);
    assert_eq!(hard["abstained"], 2);
    assert_eq!(hard["answered"], 0);
    assert_direct_fraction(&hard["coverage"], 0, 2, "no_data", 2, "episode", "selected");
    assert_direct_fraction(&hard["accuracy"], 0, 2, "no_data", 2, "episode", "selected");
    for metric in [&hard["selective_accuracy"], &hard["selective_risk"]] {
        assert_eq!(metric["value"], Value::Null);
        assert_eq!(metric["status"], "no_answered_predictions");
    }
    assert_eq!(report["integrity"]["missing_ids"], json!([]));
}

#[test]
fn case_s06() {
    let report = api_report(
        json!([]),
        json!({}),
        json!([]),
        json!({"schema_version":2,"population":"case_s06","role":"development","decision":{"type":"as_recorded"}}),
    );
    let hard = &report["raw"];
    for count in ["total", "correct", "wrong", "abstained", "answered"] {
        assert_eq!(hard[count], 0, "{count}");
    }
    for metric in [
        &hard["accuracy"],
        &hard["coverage"],
        &hard["selective_accuracy"],
    ] {
        assert_eq!(metric["value"], Value::Null);
        assert_eq!(metric["status"], "no_data");
    }
    assert_eq!(report["probability"]["log_loss"]["value"], Value::Null);
    assert_eq!(
        report["probability"]["log_loss"]["status"],
        "not_applicable"
    );
    assert_eq!(
        report["signals"]["maximum_probability"]["status"],
        "not_applicable"
    );
    assert_eq!(report["signals"]["confidence"]["status"], "not_applicable");
}

#[test]
fn case_s07() {
    let id = "01995c20-7d00-7000-8000-000000002007";
    let report = api_report(
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.2,"C":0.1}}}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"schema_version":2,"population":"case_s07","role":"development","decision":{"type":"as_recorded"}}),
    );
    for (metric, expected) in [
        ("log_loss", -0.7_f64.ln()),
        ("brier_score", 0.14),
        ("argmax_accuracy", 1.0),
    ] {
        let metric = &report["probability"][metric];
        assert_eq!(metric["status"], "defined");
        assert_eq!(metric["population_count"], 1);
        assert_eq!(metric["population_unit"], "episode");
        assert_eq!(metric["population_scope"], "selected");
        assert!(
            (metric["value"].as_f64().unwrap() - expected).abs()
                <= validator::FIXTURE_ABSOLUTE_TOLERANCE
        );
    }
}

#[test]
fn case_s08() {
    let id = "01995c20-7d00-7000-8000-000000002008";
    let report = api_report(
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"B"},"probabilities":{"kind":"categorical","values":{"A":1.0,"B":0.0,"C":0.0}}}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"expected":{"type":"class","label":"B"},"input":null}]),
        json!({"schema_version":2,"population":"case_s08","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_eq!(
        report["probability"]["log_loss"]["status"],
        "positive_infinity"
    );
    assert_eq!(report["probability"]["log_loss"]["value"], Value::Null);
    assert_eq!(
        report["probability"]["log_loss"]["special_value"],
        "+infinity"
    );
    assert!(
        (report["probability"]["brier_score"]["value"]
            .as_f64()
            .unwrap()
            - 2.0)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert_valid("report", report);
}

#[test]
fn case_s09() {
    let id = "01995c20-7d00-7000-8000-000000002009";
    let episodes = json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]);
    let source = json!({"source":{"kind":"scored_choice","model":"oracle","configuration":{},"question_id":"q"}});
    let predictions = json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.6,"B":0.38,"C":0.02}},"confidence":0.39}]);
    let (confidence_directory, confidence) = policy_run(
        "single_label",
        json!(["A", "B", "C"]),
        episodes.clone(),
        source.clone(),
        predictions.clone(),
        json!({"schema_version":2,"population":"case_s09","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":0.5}}),
    );
    let (maximum_directory, maximum) = policy_run(
        "single_label",
        json!(["A", "B", "C"]),
        episodes,
        source,
        predictions,
        json!({"schema_version":2,"population":"case_s09","role":"development","decision":{"type":"reject_below","signal":"max_probability","minimum":0.5}}),
    );
    assert_eq!(confidence["raw"], maximum["raw"]);
    assert_eq!(confidence["probability"], maximum["probability"]);
    assert_eq!(confidence["raw"]["correct"], 1);
    assert_eq!(confidence["final"]["abstained"], 1);
    assert_eq!(maximum["final"]["correct"], 1);
    assert_eq!(maximum["final"]["abstained"], 0);
    assert_eq!(confidence["episodes"][0]["raw_outcome"]["label"], "A");
    assert_eq!(
        confidence["episodes"][0]["final_outcome"]["type"],
        "abstention"
    );
    assert_eq!(maximum["episodes"][0]["final_outcome"]["label"], "A");
    assert_eq!(confidence["episodes"][0]["reported_confidence"], 0.39);
    assert_eq!(
        confidence["episodes"][0]["probability"]["max_probability"],
        0.6
    );
    assert!(
        (confidence["probability"]["log_loss"]["value"]
            .as_f64()
            .unwrap()
            + 0.6_f64.ln())
        .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    fs::remove_dir_all(confidence_directory).unwrap();
    fs::remove_dir_all(maximum_directory).unwrap();
}

#[test]
fn case_s10() {
    let id = "01995c20-7d00-7000-8000-000000002010";
    let (directory, report) = policy_run(
        "single_label",
        json!(["A", "B", "C"]),
        json!([{"id":id,"expected":{"type":"class","label":"B"},"input":null}]),
        json!({"source":{"kind":"scored_choice","model":"oracle","configuration":{},"question_id":"q"}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"B"},"probabilities":{"kind":"categorical","values":{"A":0.5,"B":0.5,"C":0.0}},"confidence":0.5}]),
        json!({"schema_version":2,"population":"case_s10","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_eq!(report["raw"]["accuracy"]["value"], 1.0);
    assert_eq!(report["episodes"][0]["raw_outcome"]["label"], "B");
    assert_eq!(report["episodes"][0]["probability"]["argmax"], "A");
    assert_eq!(
        report["episodes"][0]["probability"]["choice_argmax_disagreement"],
        true
    );
    assert_eq!(report["probability"]["argmax_accuracy"]["value"], 0.0);
    assert_eq!(report["probability"]["choice_argmax_disagreement_count"], 1);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_s11() {
    let id = "01995c20-7d00-7000-8000-000000002011";
    let episodes = json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]);
    let config = json!({"schema_version":2,"population":"case_s11","role":"development","decision":{"type":"as_recorded"}});
    let scored_source = json!({"source":{"kind":"scored_choice","model":"oracle","configuration":{},"question_id":"q"}});
    assert_evaluation_error(
        "single_label",
        json!(["A", "B", "C"]),
        episodes.clone(),
        scored_source,
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.2,"B":0.7,"C":0.1}},"confidence":0.7}]),
        config.clone(),
        validator::DiagnosticCode::Config,
    );
    let source = json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}});
    assert_evaluation_error(
        "single_label",
        json!(["A", "B", "C"]),
        episodes.clone(),
        source.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.3}}}]),
        config.clone(),
        validator::DiagnosticCode::Probability,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B", "C"]),
        episodes.clone(),
        source.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.2,"C":0.1,"D":0.0}}}]),
        config.clone(),
        validator::DiagnosticCode::Probability,
    );
    assert_evaluation_error(
        "single_label",
        json!(["A", "B", "C"]),
        episodes,
        source,
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.2,"C":0.2}}}]),
        config,
        validator::DiagnosticCode::Probability,
    );
}

fn binary64_bin_cases() -> Vec<(f64, usize)> {
    let mut values = vec![0.0];
    for index in 1..10 {
        let boundary = index as f64 / 10.0;
        values.extend([boundary.next_down(), boundary, boundary.next_up()]);
    }
    values.push(1.0);
    let expected = [
        0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 9, 9, 9, 9,
    ];
    values.into_iter().zip(expected).collect()
}

#[test]
fn marginal_binary64_bin_boundaries() {
    for (probability, expected_bin) in binary64_bin_cases() {
        let report = multi_label_report_rows(&["A"], &[(&[], Some(&[]), &[probability])]);
        let bins = report["signals"]["labels"][0]["bins"].as_array().unwrap();
        assert_eq!(bins.len(), validator::TEN_BIN_COUNT);
        for (index, bin) in bins.iter().enumerate() {
            assert_eq!(bin["count"], u64::from(index == expected_bin));
            assert_eq!(bin["positive_count"], 0);
            if index == expected_bin {
                assert_eq!(
                    bin["mean_probability"].as_f64().unwrap().to_bits(),
                    probability.to_bits()
                );
                assert_eq!(bin["observed_positive_rate"], 0.0);
            } else {
                assert_eq!(bin["mean_probability"], Value::Null);
                assert_eq!(bin["observed_positive_rate"], Value::Null);
            }
        }
    }
}

#[test]
fn marginal_negative_log_loss_preserves_tiny_probabilities() {
    for probability in [2.0_f64.powi(-54), f64::from_bits(1)] {
        let report = multi_label_report_rows(&["A"], &[(&[], Some(&[]), &[probability])]);
        // For these values the higher-order terms are smaller than half an ULP.
        for metric in [
            &report["probability"]["labels"][0]["binary_log_loss"],
            &report["probability"]["mean_binary_log_loss"],
        ] {
            assert_eq!(metric["status"], "defined");
            assert_eq!(
                metric["value"].as_f64().unwrap().to_bits(),
                probability.to_bits()
            );
        }
    }
}

#[test]
fn categorical_tolerance_neighbors() {
    let lower = 1.0 - validator::CATEGORICAL_SUM_TOLERANCE;
    let upper = 1.0 + validator::CATEGORICAL_SUM_TOLERANCE;
    for (sum, accepted) in [
        (lower.next_down(), false),
        (lower, true),
        (lower.next_up(), true),
        (upper.next_down(), true),
        (upper, false),
        (upper.next_up(), false),
    ] {
        let id = "01995c20-7d00-7000-8000-000000000001";
        let values = [0.5, sum - 0.5];
        let (directory, golden, predictions, config) = write_policy_inputs(
            "single_label",
            json!(["A", "B"]),
            json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
            json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
            json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":values[0],"B":values[1]}}}]),
            json!({"schema_version":2,"population":"tolerance neighbors","role":"development","decision":{"type":"as_recorded"}}),
        );
        let output = directory.join("run");
        let result = validator::evaluate(validator::EvaluationOptions {
            dataset: golden,
            predictions,
            config,
            output: output.clone(),
        });
        if accepted {
            result.unwrap();
            let report: Value =
                serde_json::from_slice(&fs::read(output.join("report.json")).unwrap()).unwrap();
            assert_eq!(report["integrity"]["normalized_count"], 1);
            assert_eq!(
                report["integrity"]["maximum_sum_error"].as_f64().unwrap(),
                (sum - 1.0).abs()
            );
            assert_eq!(
                report["episodes"][0]["probability"]["submitted"],
                json!(values)
            );
            assert_eq!(
                report["episodes"][0]["probability"]["working"],
                json!([values[0] / sum, values[1] / sum])
            );
        } else {
            assert_eq!(
                result.unwrap_err().code(),
                validator::DiagnosticCode::Probability
            );
            assert!(!output.exists());
        }
        fs::remove_dir_all(directory).unwrap();
    }
}

#[test]
fn serialized_probability_key_order_is_irrelevant() {
    for kind in ["single_label", "multi_label"] {
        let id = "01995c20-7d00-7000-8000-000000000001";
        let (target, probability_kind) = if kind == "single_label" {
            (json!({"type":"class","label":"A"}), "categorical")
        } else {
            (
                json!({"type":"labels","labels":["C","A"]}),
                "label_marginals",
            )
        };
        let (directory, golden, predictions, config) = write_policy_inputs(
            kind,
            json!(["C", "A", "B"]),
            json!([{"id":id,"expected":target,"input":null}]),
            json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
            json!([{"id":id,"source_id":"source","outcome":target,"probabilities":{"kind":probability_kind,"values":{"A":0.7,"B":0.2,"C":0.1}}}]),
            json!({"schema_version":2,"population":"key ordering","role":"development","decision":{"type":"as_recorded"}}),
        );
        let mut reports = Vec::new();
        for index in 0..2 {
            let output = directory.join(format!("run-{index}"));
            validator::evaluate(validator::EvaluationOptions {
                dataset: golden.clone(),
                predictions: predictions.clone(),
                config: config.clone(),
                output: output.clone(),
            })
            .unwrap();
            let report: Value =
                serde_json::from_slice(&fs::read(output.join("report.json")).unwrap()).unwrap();
            reports.push(report);
            if index == 0 {
                let original = fs::read_to_string(&predictions).unwrap();
                let reversed = original.replace(
                    "\"values\":{\"A\":0.7,\"B\":0.2,\"C\":0.1}",
                    "\"values\":{\"C\":0.1,\"B\":0.2,\"A\":0.7}",
                );
                assert_ne!(original, reversed);
                fs::write(&predictions, reversed).unwrap();
            }
        }
        for field in ["raw", "final", "probability", "signals", "episodes"] {
            assert_eq!(reports[0][field], reports[1][field]);
        }
        fs::remove_dir_all(directory).unwrap();
    }
}

#[test]
fn case_s12() {
    let threshold_ids = [
        "01995c20-7d00-7000-8000-000000002121",
        "01995c20-7d00-7000-8000-000000002122",
        "01995c20-7d00-7000-8000-000000002123",
    ];
    let threshold_values = [0.5_f64.next_down(), 0.5, 0.5_f64.next_up()];
    let (threshold_directory, threshold_report) = policy_run(
        "single_label",
        json!(["A", "B", "C"]),
        json!(threshold_ids.map(|id| json!({"id":id,"expected":{"type":"class","label":"A"},"input":null}))),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!(threshold_ids.iter().zip(threshold_values).map(|(id, confidence)| json!({"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"confidence":confidence})).collect::<Vec<_>>()),
        json!({"schema_version":2,"population":"case_s12_threshold","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":0.5}}),
    );
    assert_eq!(threshold_report["final"]["abstained"], 1);
    assert_eq!(
        threshold_report["episodes"][0]["final_outcome"]["type"],
        "abstention"
    );
    assert_eq!(
        threshold_report["episodes"][1]["final_outcome"]["type"],
        "class"
    );
    assert_eq!(
        threshold_report["episodes"][2]["final_outcome"]["type"],
        "class"
    );
    fs::remove_dir_all(threshold_directory).unwrap();

    let cases = binary64_bin_cases();
    let values = cases.iter().map(|(value, _)| *value).collect::<Vec<_>>();
    let ids = (1..=values.len())
        .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
        .collect::<Vec<_>>();
    let (directory, report) = policy_run(
        "single_label",
        json!(["A", "B", "C"]),
        json!(ids.iter().map(|id| json!({"id":id,"expected":{"type":"class","label":"A"},"input":null})).collect::<Vec<_>>()),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!(ids.iter().zip(&values).map(|(id, confidence)| json!({"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"confidence":confidence})).collect::<Vec<_>>()),
        json!({"schema_version":2,"population":"case_s12_bins","role":"development","decision":{"type":"as_recorded"}}),
    );
    let bins = report["signals"]["confidence"]["bins"].as_array().unwrap();
    let expected_bins = cases.iter().map(|(_, bin)| *bin);
    assert_eq!((0.9_f64.next_down() * 10.0).floor(), 9.0);
    for (_, id) in values.iter().zip(&ids) {
        assert!(
            report["signals"]["confidence"]["included_ids"]
                .as_array()
                .unwrap()
                .contains(&json!(id))
        );
    }
    for ((value, id), expected_bin) in values.iter().zip(&ids).zip(expected_bins) {
        let (isolated_directory, isolated) = policy_run(
            "single_label",
            json!(["A", "B", "C"]),
            json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
            json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
            json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"A"},"confidence":value}]),
            json!({"schema_version":2,"population":"case_s12_isolated_bins","role":"development","decision":{"type":"as_recorded"}}),
        );
        let isolated_bins = isolated["signals"]["confidence"]["bins"]
            .as_array()
            .unwrap();
        for (index, bin) in isolated_bins.iter().enumerate() {
            assert_eq!(bin["count"], u64::from(index == expected_bin));
            if index == expected_bin {
                assert!(
                    (bin["mean_signal"].as_f64().unwrap() - value).abs()
                        <= validator::FIXTURE_ABSOLUTE_TOLERANCE
                );
            } else {
                assert_eq!(bin["mean_signal"], Value::Null);
            }
        }
        assert_eq!(
            isolated["signals"]["confidence"]["included_ids"],
            json!([id])
        );
        fs::remove_dir_all(isolated_directory).unwrap();
    }
    assert_eq!(bins[0]["lower"], 0.0);
    assert_eq!(bins[9]["upper"], 1.0);
    assert_eq!(bins[9]["upper_inclusive"], true);
    assert!(bins[9]["count"].as_u64().unwrap() >= 2);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_e01() {
    let id = "01995c20-7d00-7000-8000-000000002401";
    let report = api_report(
        json!([{"id":id,"source_id":"source","outcome":{"type":"abstention","reason":"review"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.2,"C":0.1}},"confidence":0.8}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"schema_version":2,"population":"case_e01","role":"development","decision":{"type":"as_recorded"}}),
    );
    assert_direct_fraction(
        &report["raw"]["accuracy"],
        0,
        1,
        "no_data",
        1,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &report["raw"]["coverage"],
        0,
        1,
        "no_data",
        1,
        "episode",
        "selected",
    );
    assert!(
        (report["probability"]["log_loss"]["value"].as_f64().unwrap() + 0.7_f64.ln()).abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert!(
        (report["probability"]["brier_score"]["value"]
            .as_f64()
            .unwrap()
            - 0.14)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert_eq!(report["probability"]["argmax_accuracy"]["value"], 1.0);
    assert_eq!(
        report["episodes"][0]["probability"]["chosen_probability"],
        Value::Null
    );
    assert_eq!(
        report["signals"]["confidence"]["status"],
        "no_answered_predictions"
    );
    assert_eq!(
        report["signals"]["confidence"]["population_scope"],
        "raw_answered"
    );
    assert_eq!(report["signals"]["confidence"]["population_count"], 0);
    assert_eq!(report["signals"]["confidence"]["excluded_ids"], json!([id]));
}

#[test]
fn case_e02() {
    let ids = [
        "01995c20-7d00-7000-8000-000000002421",
        "01995c20-7d00-7000-8000-000000002422",
    ];
    let report = api_report(
        json!([
            {"id":ids[0],"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.2,"C":0.1}},"confidence":0.8},
            {"id":ids[1],"source_id":"source","outcome":{"type":"abstention"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.2,"C":0.1}},"confidence":0.2}
        ]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([
            {"id":ids[0],"expected":{"type":"class","label":"A"},"input":null},
            {"id":ids[1],"expected":{"type":"class","label":"A"},"input":null}
        ]),
        json!({"schema_version":2,"population":"case_e02","role":"development","decision":{"type":"as_recorded"}}),
    );
    let confidence = &report["signals"]["confidence"];
    assert_eq!(confidence["population_scope"], "raw_answered");
    assert_eq!(confidence["population_count"], 1);
    assert_eq!(confidence["included_ids"], json!([ids[0]]));
    assert_eq!(confidence["excluded_ids"], json!([ids[1]]));
    assert_eq!(confidence["bins"][8]["count"], 1);
    assert_eq!(confidence["bins"][2]["count"], 0);
}

#[test]
fn case_e03() {
    let single_id = "01995c20-7d00-7000-8000-000000002431";
    assert_policy_config_error(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":single_id,"expected":{"type":"class","label":"A"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":single_id,"source_id":"source","outcome":{"type":"abstention"},"probabilities":{"kind":"categorical","values":{"A":0.7,"B":0.3}},"confidence":0.8}]),
        json!({"schema_version":2,"population":"case_e03","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":0.5}}),
    );
    let multi_id = "01995c20-7d00-7000-8000-000000002432";
    assert_policy_config_error(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":multi_id,"expected":{"type":"labels","labels":["A"]},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":multi_id,"source_id":"source","outcome":{"type":"abstention"},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.7}}}]),
        json!({"schema_version":2,"population":"case_e03","role":"development","decision":{"type":"label_thresholds","thresholds":{"A":0.5,"B":0.5}}}),
    );
}

#[test]
fn case_e04() {
    let id = "01995c20-7d00-7000-8000-000000002441";
    let (directory, report) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"abstention","reason":"review"},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.7}}}]),
        json!({"schema_version":2,"population":"case_e04","role":"development","decision":{"type":"as_recorded"}}),
    );
    let hard = &report["raw"];
    assert_eq!(hard["abstained"], 1);
    assert_eq!(hard["answered"], 0);
    assert_direct_fraction(&hard["coverage"], 0, 1, "no_data", 1, "episode", "selected");
    for label in hard["labels"].as_array().unwrap() {
        for count in [
            "true_positive",
            "false_positive",
            "false_negative",
            "true_negative",
        ] {
            assert_eq!(label[count], 0, "{} {count}", label["label"]);
        }
    }
    let outcome = &report["episodes"][0]["raw_outcome"];
    assert_eq!(outcome["status"], "abstained");
    assert_eq!(outcome["matched"], Value::Null);
    assert_eq!(outcome["missed"], Value::Null);
    assert_eq!(outcome["extra"], Value::Null);
    assert!(
        (report["probability"]["mean_binary_brier"]["value"]
            .as_f64()
            .unwrap()
            - 0.265)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    let expected_log_loss = (-0.8_f64.ln() - (1.0_f64 - 0.7).ln()) / 2.0;
    assert_eq!(
        report["probability"]["mean_binary_log_loss"]["status"],
        "defined"
    );
    assert!(
        (report["probability"]["mean_binary_log_loss"]["value"]
            .as_f64()
            .unwrap()
            - expected_log_loss)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert_eq!(
        report["probability"]["mean_binary_log_loss"]["population_count"],
        2
    );
    assert_eq!(
        report["probability"]["mean_binary_log_loss"]["population_unit"],
        "label_decision"
    );
    assert_eq!(
        report["probability"]["mean_binary_log_loss"]["population_scope"],
        "selected"
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_e07() {
    let id = "01995c20-7d00-7000-8000-000000002471";
    let (directory, golden, predictions, config) = write_policy_inputs(
        "single_label",
        json!(["NO_MATCH", "UNCERTAIN", "MATCH"]),
        json!([{"id":id,"expected":{"type":"class","label":"MATCH"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"class","label":"MATCH"},"probabilities":{"kind":"categorical","values":{"NO_MATCH":0.24,"UNCERTAIN":0.19,"MATCH":0.56}}}]),
        json!({"schema_version":2,"population":"case_e07","role":"development","decision":{"type":"as_recorded"}}),
    );
    let output = directory.join("run");
    let error = validator::evaluate(validator::EvaluationOptions {
        dataset: golden,
        predictions,
        config,
        output: output.clone(),
    })
    .unwrap_err();
    assert_eq!(error.code(), validator::DiagnosticCode::Probability);
    assert_eq!(error.exit_category().code(), 2);
    assert!(!output.exists());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_m01() {
    let report = multi_label_report_rows(
        &["A", "B", "C"],
        &[
            (&["A", "B"], Some(&["A", "C"]), &[0.8, 0.7, 0.2]),
            (&[], Some(&[]), &[0.1, 0.1, 0.1]),
        ],
    );
    let hard = &report["raw"];
    assert_eq!(hard["total"], 2);
    assert_eq!(hard["answered"], 2);
    assert_eq!(hard["exact_matches"], 1);
    assert_eq!(hard["wrong_sets"], 1);
    assert_eq!(hard["abstained"], 0);
    assert_direct_fraction(
        &hard["exact_match_accuracy"],
        1,
        2,
        "no_data",
        2,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &hard["answered_micro_f1"],
        2,
        4,
        "undefined_zero_denominator",
        6,
        "label_decision",
        "answered",
    );
    assert_direct_fraction(
        &hard["answered_hamming_loss"],
        2,
        6,
        "undefined_zero_denominator",
        6,
        "label_decision",
        "answered",
    );
    assert!(
        (hard["answered_macro_f1"]["metric"]["value"]
            .as_f64()
            .unwrap()
            - 1.0 / 3.0)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    let aggregate = hard["labels"].as_array().unwrap().iter().fold(
        (0, 0, 0, 0),
        |(tp, fp, r#fn, tn), label| {
            assert_eq!(
                label["true_positive"].as_u64().unwrap()
                    + label["false_positive"].as_u64().unwrap()
                    + label["false_negative"].as_u64().unwrap()
                    + label["true_negative"].as_u64().unwrap(),
                2
            );
            (
                tp + label["true_positive"].as_u64().unwrap(),
                fp + label["false_positive"].as_u64().unwrap(),
                r#fn + label["false_negative"].as_u64().unwrap(),
                tn + label["true_negative"].as_u64().unwrap(),
            )
        },
    );
    assert_eq!(aggregate, (1, 1, 1, 3));
    for hard in [&report["raw"], &report["final"]] {
        for (actual, (precision, recall, f1)) in hard["labels"].as_array().unwrap().iter().zip([
            ((1, 1, "defined"), (1, 1, "defined"), (2, 2, "defined")),
            (
                (0, 0, "undefined_zero_denominator"),
                (0, 1, "defined"),
                (0, 1, "defined"),
            ),
            (
                (0, 1, "defined"),
                (0, 0, "undefined_zero_denominator"),
                (0, 1, "defined"),
            ),
        ]) {
            for (metric, (numerator, denominator, status)) in [
                (&actual["precision"], precision),
                (&actual["recall"], recall),
                (&actual["f1"], f1),
            ] {
                assert_direct_fraction(
                    metric,
                    numerator,
                    denominator,
                    status,
                    2,
                    "label_decision",
                    "answered",
                );
            }
        }
    }
    assert_eq!(
        report["episodes"][0]["raw_outcome"]["matched"],
        json!(["A"])
    );
    assert_eq!(report["episodes"][0]["raw_outcome"]["missed"], json!(["B"]));
    assert_eq!(report["episodes"][0]["raw_outcome"]["extra"], json!(["C"]));
}

#[test]
fn case_m02() {
    let report = multi_label_report_rows(&["A", "B"], &[(&[], Some(&[]), &[0.0, 0.0])]);
    let hard = &report["raw"];
    assert_direct_fraction(
        &hard["exact_match_accuracy"],
        1,
        1,
        "no_data",
        1,
        "episode",
        "selected",
    );
    assert_direct_fraction(&hard["coverage"], 1, 1, "no_data", 1, "episode", "selected");
    assert_direct_fraction(
        &hard["answered_hamming_loss"],
        0,
        2,
        "undefined_zero_denominator",
        2,
        "label_decision",
        "answered",
    );
    for hard in [&report["raw"], &report["final"]] {
        for (label, actual) in ["A", "B"]
            .into_iter()
            .zip(hard["labels"].as_array().unwrap())
        {
            assert_eq!(actual["label"], label);
            assert_eq!(actual["support"], 0);
            assert_eq!(actual["answered_support"], 0);
            assert_eq!(actual["predicted_support"], 0);
            assert_eq!(actual["true_positive"], 0);
            assert_eq!(actual["false_positive"], 0);
            assert_eq!(actual["false_negative"], 0);
            assert_eq!(actual["true_negative"], 1);
            for metric in [&actual["precision"], &actual["recall"], &actual["f1"]] {
                assert_direct_fraction(
                    metric,
                    0,
                    0,
                    "undefined_zero_denominator",
                    1,
                    "label_decision",
                    "answered",
                );
            }
            assert_eq!(actual["f1"]["numerator"], 0);
            assert_eq!(actual["f1"]["denominator"], 0);
        }
    }
    assert_eq!(hard["answered_micro_f1"]["value"], Value::Null);
    assert_eq!(
        hard["answered_micro_f1"]["status"],
        "undefined_zero_denominator"
    );
    assert_eq!(hard["answered_macro_f1"]["metric"]["value"], 0.0);
    assert_eq!(
        hard["answered_macro_f1"]["metric"]["status"],
        "contains_undefined_classes"
    );
    assert_eq!(
        hard["answered_macro_f1"]["undefined_classes"],
        json!(["A", "B"])
    );
}

#[test]
fn case_m03() {
    let report =
        multi_label_report_rows(&["A"], &[(&["A"], None, &[0.8]), (&[], Some(&[]), &[0.0])]);
    let hard = &report["raw"];
    assert_eq!(hard["total"], 2);
    assert_eq!(hard["answered"], 1);
    assert_eq!(hard["abstained"], 1);
    assert_eq!(hard["exact_matches"], 1);
    assert_eq!(hard["wrong_sets"], 0);
    assert_direct_fraction(
        &hard["exact_match_accuracy"],
        1,
        2,
        "no_data",
        2,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &hard["selective_exact_match_accuracy"],
        1,
        1,
        "no_answered_predictions",
        1,
        "episode",
        "answered",
    );
    assert_direct_fraction(&hard["coverage"], 1, 2, "no_data", 2, "episode", "selected");
    let label = &hard["labels"][0];
    assert_eq!(label["support"], 1);
    assert_eq!(label["answered_support"], 0);
    assert_eq!(label["true_negative"], 1);
    assert_eq!(report["episodes"][0]["raw_outcome"]["status"], "abstained");
    for field in ["matched", "missed", "extra"] {
        assert_eq!(report["episodes"][0]["raw_outcome"][field], Value::Null);
    }
}

#[test]
fn case_m04() {
    let report = multi_label_report_rows(
        &["A", "B"],
        &[(&["A"], None, &[0.8, 0.2]), (&[], None, &[0.0, 0.0])],
    );
    for hard in [&report["raw"], &report["final"]] {
        assert_eq!(hard["total"], 2);
        assert_eq!(hard["answered"], 0);
        assert_eq!(hard["abstained"], 2);
        assert_direct_fraction(&hard["coverage"], 0, 2, "no_data", 2, "episode", "selected");
        assert_direct_fraction(
            &hard["exact_match_accuracy"],
            0,
            2,
            "no_data",
            2,
            "episode",
            "selected",
        );
        for metric in [
            &hard["selective_exact_match_accuracy"],
            &hard["answered_micro_f1"],
            &hard["answered_hamming_loss"],
        ] {
            assert_eq!(metric["value"], Value::Null);
            assert_eq!(metric["status"], "no_answered_predictions");
        }
        for label in hard["labels"].as_array().unwrap() {
            for metric in [&label["precision"], &label["recall"], &label["f1"]] {
                assert_direct_fraction(
                    metric,
                    0,
                    0,
                    "no_answered_predictions",
                    0,
                    "label_decision",
                    "answered",
                );
            }
        }
    }
    for episode in report["episodes"].as_array().unwrap() {
        assert_eq!(episode["raw_outcome"]["type"], "abstention");
        assert_eq!(episode["raw_outcome"]["status"], "abstained");
        for field in ["matched", "missed", "extra"] {
            assert_eq!(episode["raw_outcome"][field], Value::Null);
        }
    }
    let empty = multi_label_report_rows(&["A", "B"], &[]);
    for hard in [&empty["raw"], &empty["final"]] {
        assert_eq!(hard["total"], 0);
        assert_eq!(hard["answered"], 0);
        for label in hard["labels"].as_array().unwrap() {
            for metric in [&label["precision"], &label["recall"], &label["f1"]] {
                assert_direct_fraction(metric, 0, 0, "no_data", 0, "label_decision", "answered");
            }
        }
        assert_eq!(hard["exact_match_accuracy"]["population_unit"], "episode");
        assert_eq!(
            hard["answered_micro_f1"]["population_unit"],
            "label_decision"
        );
    }
}

#[test]
fn case_m05() {
    let report = multi_label_report_rows(&["A", "B"], &[(&["A"], Some(&["A"]), &[0.8, 0.7])]);
    let probability = &report["probability"];
    let expected_log_loss = (-0.8_f64.ln() - 0.3_f64.ln()) / 2.0;
    assert_eq!(probability["kind"], "multi_label");
    assert_eq!(
        report["episodes"][0]["probability"]["marginals"],
        json!({"A":0.8,"B":0.7})
    );
    assert!(
        (probability["mean_binary_log_loss"]["value"]
            .as_f64()
            .unwrap()
            - expected_log_loss)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert!(
        (probability["mean_binary_brier"]["value"].as_f64().unwrap() - 0.265).abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    for (label, log_loss, brier) in [("A", -0.8_f64.ln(), 0.04), ("B", -0.3_f64.ln(), 0.49)] {
        let actual = probability["labels"]
            .as_array()
            .unwrap()
            .iter()
            .find(|actual| actual["label"] == label)
            .unwrap();
        assert!(
            (actual["binary_log_loss"]["value"].as_f64().unwrap() - log_loss).abs()
                <= validator::FIXTURE_ABSOLUTE_TOLERANCE
        );
        assert!(
            (actual["binary_brier"]["value"].as_f64().unwrap() - brier).abs()
                <= validator::FIXTURE_ABSOLUTE_TOLERANCE
        );
        assert_eq!(actual["binary_log_loss"]["population_count"], 1);
        assert_eq!(
            actual["binary_log_loss"]["population_unit"],
            "label_decision"
        );
        assert_eq!(actual["binary_log_loss"]["population_scope"], "selected");
    }
}

#[test]
fn case_m06() {
    let id = "01995c20-7d00-7000-8000-000000002506";
    let episodes = json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]);
    let sources = json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}});
    let predictions = json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A","B"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.7}}}]);
    let (raw_directory, raw) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        episodes.clone(),
        sources.clone(),
        predictions.clone(),
        json!({"schema_version":2,"population":"case_m06","role":"development","decision":{"type":"as_recorded"}}),
    );
    let (directory, thresholded) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        episodes,
        sources,
        predictions,
        json!({"schema_version":2,"population":"case_m06","role":"development","decision":{"type":"label_thresholds","thresholds":{"A":0.8,"B":0.75}}}),
    );
    assert_eq!(
        thresholded["episodes"][0]["raw_outcome"]["labels"],
        json!(["A", "B"])
    );
    assert_eq!(
        thresholded["episodes"][0]["final_outcome"]["labels"],
        json!(["A"])
    );
    assert_eq!(thresholded["raw"]["exact_matches"], 0);
    assert_eq!(thresholded["final"]["exact_matches"], 1);
    assert_eq!(thresholded["probability"], raw["probability"]);
    assert_eq!(
        thresholded["episodes"][0]["probability"],
        raw["episodes"][0]["probability"]
    );
    fs::remove_dir_all(raw_directory).unwrap();
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_m07() {
    let id = "01995c20-7d00-7000-8000-000000002507";
    let (directory, report) = policy_run(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":[]},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A","B"]},"probabilities":{"kind":"label_marginals","values":{"A":0.2,"B":0.3}}}]),
        json!({"schema_version":2,"population":"case_m07","role":"development","decision":{"type":"label_thresholds","thresholds":{"A":0.5,"B":0.5}}}),
    );
    assert_eq!(report["final"]["answered"], 1);
    assert_eq!(report["final"]["abstained"], 0);
    assert_eq!(
        report["episodes"][0]["final_outcome"],
        json!({"type":"labels","labels":[],"matched":[],"missed":[],"extra":[],"correct":true})
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn case_m08() {
    let report = multi_label_report_rows(&["A", "B"], &[(&[], Some(&[]), &[0.0, 0.0])]);
    for label in report["probability"]["labels"].as_array().unwrap() {
        assert_eq!(label["binary_log_loss"]["status"], "defined");
        assert_eq!(label["binary_log_loss"]["value"], 0.0);
        assert_eq!(label["binary_brier"]["value"], 0.0);
    }
    assert_eq!(report["probability"]["mean_binary_log_loss"]["value"], 0.0);
    assert_eq!(report["probability"]["mean_binary_brier"]["value"], 0.0);
    assert_valid("report", report);
}

#[test]
fn case_m09() {
    for (expected, probability) in [(&["A"][..], 0.0), (&[][..], 1.0)] {
        let report = multi_label_report_rows(&["A"], &[(expected, Some(expected), &[probability])]);
        for metric in [
            &report["probability"]["labels"][0]["binary_log_loss"],
            &report["probability"]["mean_binary_log_loss"],
        ] {
            assert_eq!(metric["value"], Value::Null);
            assert_eq!(metric["status"], "positive_infinity");
            assert_eq!(metric["special_value"], "+infinity");
        }
    }
}

#[test]
fn case_m10() {
    let report = multi_label_report_rows(
        &["A"],
        &[
            (&[], Some(&["A"]), &[0.0]),
            (&["A"], Some(&[]), &[0.1]),
            (&["A"], Some(&[]), &[1.0]),
        ],
    );
    let bins = report["signals"]["labels"][0]["bins"].as_array().unwrap();
    assert_eq!(bins[0]["count"], 1);
    assert_eq!(bins[0]["positive_count"], 0);
    assert_eq!(bins[1]["count"], 1);
    assert_eq!(bins[1]["positive_count"], 1);
    assert_eq!(bins[9]["count"], 1);
    assert_eq!(bins[9]["positive_count"], 1);
    assert_eq!(bins[9]["upper_inclusive"], true);
    assert_eq!(bins[0]["observed_positive_rate"], 0.0);
    assert_eq!(bins[1]["observed_positive_rate"], 1.0);
}

#[test]
fn case_m11() {
    let categorical = two_label_categorical_report();
    let marginal = multi_label_report_rows(&["A"], &[(&["A"], Some(&["A"]), &[0.8])]);
    assert_eq!(categorical["probability"]["kind"], "single_label");
    assert_eq!(marginal["probability"]["kind"], "multi_label");
    assert!(
        (categorical["probability"]["brier_score"]["value"]
            .as_f64()
            .unwrap()
            - 0.08)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
    assert!(
        (marginal["probability"]["mean_binary_brier"]["value"]
            .as_f64()
            .unwrap()
            - 0.04)
            .abs()
            <= validator::FIXTURE_ABSOLUTE_TOLERANCE
    );
}

#[test]
fn case_m12() {
    let exact = multi_label_report_rows(
        &["A", "B"],
        &[
            (&["A", "B"], Some(&[]), &[0.2, 0.2]),
            (&["A", "B"], Some(&["A", "B"]), &[0.8, 0.8]),
        ],
    );
    let split = multi_label_report_rows(
        &["A", "B"],
        &[
            (&["A", "B"], Some(&["A"]), &[0.8, 0.2]),
            (&["A", "B"], Some(&["B"]), &[0.2, 0.8]),
        ],
    );
    let counts = |report: &Value| {
        report["raw"]["labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|label| {
                json!([
                    label["true_positive"],
                    label["false_positive"],
                    label["false_negative"],
                    label["true_negative"]
                ])
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(counts(&exact), counts(&split));
    assert_direct_fraction(
        &exact["raw"]["exact_match_accuracy"],
        1,
        2,
        "no_data",
        2,
        "episode",
        "selected",
    );
    assert_direct_fraction(
        &split["raw"]["exact_match_accuracy"],
        0,
        2,
        "no_data",
        2,
        "episode",
        "selected",
    );
    assert_eq!(
        exact["episodes"][0]["raw_outcome"]["missed"],
        json!(["A", "B"])
    );
    assert_eq!(split["episodes"][0]["raw_outcome"]["matched"], json!(["A"]));
    assert_eq!(split["episodes"][0]["raw_outcome"]["missed"], json!(["B"]));
}

#[test]
fn case_m13() {
    let id = "01995c20-7d00-7000-8000-000000002513";
    let sources = json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}});
    let config = json!({"schema_version":2,"population":"case_m13","role":"development","decision":{"type":"label_thresholds","thresholds":{"A":0.5,"B":0.5}}});
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":["A","A"]},"input":null}]),
        sources.clone(),
        json!([]),
        config.clone(),
        validator::DiagnosticCode::Label,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A","A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.1,"B":0.1}}}]),
        config.clone(),
        validator::DiagnosticCode::Label,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]),
        sources.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["C"]},"probabilities":{"kind":"label_marginals","values":{"A":0.1,"B":0.1}}}]),
        config.clone(),
        validator::DiagnosticCode::Label,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]),
        sources,
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.1}}}]),
        config,
        validator::DiagnosticCode::Probability,
    );
}

#[test]
fn case_m14() {
    let id = "01995c20-7d00-7000-8000-000000002514";
    let source = json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}});
    let as_recorded = json!({"schema_version":2,"population":"case_m14","role":"development","decision":{"type":"as_recorded"}});
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]),
        source.clone(),
        json!([]),
        as_recorded.clone(),
        validator::DiagnosticCode::Config,
    );
    let multi_class_target =
        json!([{"id":id,"expected":{"type":"class","label":"A"},"input":null}]);
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        multi_class_target,
        source.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.2}}}]),
        as_recorded.clone(),
        validator::DiagnosticCode::Config,
    );
    let multi_episode = json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]);
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        multi_episode.clone(),
        source.clone(),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"categorical","values":{"A":0.8,"B":0.2}}}]),
        as_recorded,
        validator::DiagnosticCode::Probability,
    );
    assert_policy_config_error(
        "multi_label",
        json!(["A", "B"]),
        multi_episode,
        source,
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.2}}}]),
        json!({"schema_version":2,"population":"case_m14","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":0.5}}),
    );
}

#[test]
fn case_m15() {
    let id = "01995c20-7d00-7000-8000-000000002515";
    let episode = json!([{"id":id,"expected":{"type":"labels","labels":["A"]},"input":null}]);
    let config = json!({"schema_version":2,"population":"case_m15","role":"development","decision":{"type":"as_recorded"}});
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode.clone(),
        json!({"source":{"kind":"scored_choice","model":"oracle","configuration":{},"question_id":"q"}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.2}}}]),
        config.clone(),
        validator::DiagnosticCode::Config,
    );
    assert_evaluation_error(
        "multi_label",
        json!(["A", "B"]),
        episode,
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}}),
        json!([{"id":id,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.2}},"confidence":0.8}]),
        config,
        validator::DiagnosticCode::Confidence,
    );
}

#[test]
fn case_m20() {
    let ids = [
        "01995c20-7d00-7000-8000-000000002520",
        "01995c20-7d00-7000-8000-000000002521",
    ];
    let sources = json!({"source":{"kind":"classifier","model":"oracle","configuration":{}}});
    let config = json!({"schema_version":2,"population":"case_m20","role":"development","decision":{"type":"as_recorded"}});
    let (first_directory, first) = policy_run(
        "multi_label",
        json!(["C", "A", "B"]),
        json!([
            {"id":ids[0],"expected":{"type":"labels","labels":["A","C"]},"input":null},
            {"id":ids[1],"expected":{"type":"labels","labels":["C"]},"input":null}
        ]),
        sources.clone(),
        json!([
            {"id":ids[0],"source_id":"source","outcome":{"type":"labels","labels":["C","A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8,"B":0.7,"C":0.1}}},
            {"id":ids[1],"source_id":"source","outcome":{"type":"labels","labels":["C"]},"probabilities":{"kind":"label_marginals","values":{"A":0.1,"B":0.2,"C":0.9}}}
        ]),
        config.clone(),
    );
    let (second_directory, second) = policy_run(
        "multi_label",
        json!(["C", "A", "B"]),
        json!([
            {"id":ids[1],"expected":{"type":"labels","labels":["C"]},"input":null},
            {"id":ids[0],"expected":{"type":"labels","labels":["C","A"]},"input":null}
        ]),
        sources,
        json!([
            {"id":ids[1],"source_id":"source","outcome":{"type":"labels","labels":["C"]},"probabilities":{"kind":"label_marginals","values":{"C":0.9,"B":0.2,"A":0.1}}},
            {"id":ids[0],"source_id":"source","outcome":{"type":"labels","labels":["A","C"]},"probabilities":{"kind":"label_marginals","values":{"C":0.1,"B":0.7,"A":0.8}}}
        ]),
        config,
    );
    for field in ["raw", "final", "probability", "signals", "episodes"] {
        assert_eq!(first[field], second[field]);
    }
    let prediction_digest = |report: &Value| {
        report["artifacts"]
            .as_array()
            .unwrap()
            .iter()
            .find(|artifact| artifact["kind"] == "predictions")
            .unwrap()["sha256"]
            .clone()
    };
    assert_ne!(prediction_digest(&first), prediction_digest(&second));
    for report in [&first, &second] {
        for episode in report["episodes"].as_array().unwrap() {
            for outcome in ["raw_outcome", "final_outcome"] {
                let labels = &episode[outcome]["labels"];
                if labels != &Value::Null {
                    let ordered = ["C", "A", "B"]
                        .into_iter()
                        .map(|label| json!(label))
                        .filter(|label| labels.as_array().unwrap().contains(label))
                        .collect::<Vec<_>>();
                    assert_eq!(*labels, Value::Array(ordered));
                }
            }
        }
    }
    fs::remove_dir_all(first_directory).unwrap();
    fs::remove_dir_all(second_directory).unwrap();
}

#[test]
fn full_numeric_conformance() {
    exhaustive_single_label();
    exhaustive_multi_label();
    f04_asymmetric_oracle();
    case_s01();
    case_s02();
    case_s03();
    case_s04();
    case_s05();
    case_s06();
    case_s07();
    case_s08();
    case_s09();
    case_s10();
    case_s11();
    case_s12();
    case_e01();
    case_e02();
    case_e03();
    case_e04();
    case_e07();
    case_m01();
    case_m02();
    case_m03();
    case_m04();
    case_m05();
    case_m06();
    case_m07();
    case_m08();
    case_m09();
    case_m10();
    case_m11();
    case_m12();
    case_m13();
    case_m14();
    case_m15();
    case_m20();
}

#[test]
fn all_schema_contracts() {
    const ID: &str = "01995c20-7d00-7000-8000-000000000001";
    const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    for name in [
        "golden",
        "predictions",
        "config",
        "report",
        "receipt",
        "error",
    ] {
        jsonschema::validator_for(&schema(name)).unwrap();
    }

    let opaque: Value = serde_json::from_str(r#"{"null":null,"large":9007199254740993,"huge":1e400,"$serde_json::private::Number":"literal"}"#).unwrap();
    let golden_single = json!({"schema_version":2,"task":{"kind":"single_label","labels":["A","B"]},"episodes":[{"id":ID,"expected":{"type":"class","label":"A"},"input":opaque}]});
    let golden_multi = json!({"schema_version":2,"task":{"kind":"multi_label","labels":["A"]},"episodes":[{"id":ID,"expected":{"type":"labels","labels":[]},"input":null}]});
    let prediction_classifier = json!({"schema_version":2,"dataset_sha256":DIGEST,"sources":{"source":{"kind":"classifier","model":"m","configuration":opaque,"observation_definitions":{"scalar":{"kind":"scalar","description":"s"},"bernoulli":{"kind":"bernoulli","description":"b"},"confidence":{"kind":"reported_confidence","description":"c"},"categorical":{"kind":"categorical","description":"d"},"marginals":{"kind":"label_marginals","description":"m"}},"preparation":{"method":"prepare","version":"1","configuration":{"null":null},"evidence_indices":[0]}}},"predictions":[{"id":ID,"source_id":"source","outcome":{"type":"class","label":"A"},"probabilities":{"kind":"categorical","values":{"A":0.8,"B":0.2}},"confidence":0.8,"observations":{"scalar":{"kind":"scalar","value":1.5},"bernoulli":{"kind":"bernoulli","value":0.8},"confidence":{"kind":"reported_confidence","value":0.8},"categorical":{"kind":"categorical","values":{"A":0.8}},"marginals":{"kind":"label_marginals","values":{"A":0.8}}}}]});
    let prediction_scored_choice = json!({"schema_version":2,"dataset_sha256":DIGEST,"sources":{"source":{"kind":"scored_choice","model":"m","question_id":"q","configuration":{}}},"predictions":[{"id":ID,"source_id":"source","outcome":{"type":"labels","labels":["A"]},"probabilities":{"kind":"label_marginals","values":{"A":0.8}}},{"id":"01995c20-7d00-7000-8000-000000000002","source_id":"source","outcome":{"type":"abstention"}}]});
    let config_single = json!({"schema_version":2,"population":"p","role":"development","decision":{"type":"reject_below","signal":"confidence","minimum":0.5}});
    let config_multi = json!({"schema_version":2,"population":"p","role":"held_out","decision":{"type":"label_thresholds","thresholds":{"A":0.5}}});
    let config_recorded = json!({"schema_version":2,"population":"p","role":"development","decision":{"type":"as_recorded"}});
    for (name, document) in [
        ("golden", golden_single.clone()),
        ("golden", golden_multi),
        ("predictions", prediction_classifier.clone()),
        ("predictions", prediction_scored_choice),
        ("config", config_single),
        ("config", config_multi),
        ("config", config_recorded),
    ] {
        assert_valid(name, document);
    }

    for (name, document, fields) in [
        (
            "golden",
            golden_single.clone(),
            &["schema_version", "task", "episodes"][..],
        ),
        (
            "predictions",
            prediction_classifier.clone(),
            &["schema_version", "dataset_sha256", "sources", "predictions"][..],
        ),
        (
            "config",
            json!({"schema_version":2,"population":"p","role":"development","decision":{"type":"as_recorded"}}),
            &["schema_version", "population", "role", "decision"][..],
        ),
    ] {
        for field in fields {
            let mut missing = document.clone();
            missing.as_object_mut().unwrap().remove(*field);
            assert_invalid(name, missing);
        }
    }
    let mut missing_input = golden_single.clone();
    missing_input["episodes"][0]
        .as_object_mut()
        .unwrap()
        .remove("input");
    assert_invalid("golden", missing_input);
    let mut wrong_target = golden_single.clone();
    wrong_target["episodes"][0]["expected"] = json!({"type":"labels","labels":[]});
    assert_invalid("golden", wrong_target);
    let mut missing_configuration = prediction_classifier.clone();
    missing_configuration["sources"]["source"]
        .as_object_mut()
        .unwrap()
        .remove("configuration");
    assert_invalid("predictions", missing_configuration);
    let mut wrong_observation = prediction_classifier;
    wrong_observation["predictions"][0]["observations"]["scalar"] =
        json!({"kind":"scalar","values":{"A":0.8}});
    assert_invalid("predictions", wrong_observation);

    let single_report = fixture_report(true, true);
    let multi_report = multi_label_report_rows(&["A", "B"], &[(&["A"], Some(&["A"]), &[0.8, 0.2])]);
    assert_valid("report", single_report.clone());
    assert_valid("report", multi_report.clone());
    for field in [
        "identity",
        "raw",
        "final",
        "probability",
        "signals",
        "episodes",
    ] {
        let mut missing = single_report.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert_invalid("report", missing);
    }
    let mut untyped_observation = single_report.clone();
    untyped_observation["episodes"][0]["observations"] =
        json!({"x":{"kind":"scalar","values":{"A":0.5}}});
    assert_invalid("report", untyped_observation);
    let mut invalid_metric = single_report.clone();
    invalid_metric["raw"]["accuracy"]["status"] = json!("invented");
    assert_invalid("report", invalid_metric);

    let mut report_raw_answered = single_report.clone();
    report_raw_answered["raw"]["accuracy"]["population_scope"] = json!("raw_answered");
    assert_invalid("report", report_raw_answered);
    let mut null_source_configuration = single_report.clone();
    null_source_configuration["sources"]["source"]["configuration"] = Value::Null;
    assert_invalid("report", null_source_configuration);
    let mut null_preparation_configuration = single_report.clone();
    null_preparation_configuration["sources"]["source"]["preparation"] = json!({
        "method": "prepare",
        "version": "1",
        "configuration": null,
        "evidence_indices": []
    });
    assert_invalid("report", null_preparation_configuration);
    let mut absolute_artifact_path = single_report.clone();
    absolute_artifact_path["artifacts"][0]["path"] = json!("/golden.json");
    assert_invalid("report", absolute_artifact_path);
    let mut traversal_artifact_path = single_report.clone();
    traversal_artifact_path["artifacts"][0]["path"] = json!("../golden.json");
    assert_invalid("report", traversal_artifact_path);

    let receipt = json!({"schema_version":2,"kind":"evaluation","status":"complete","run_id":ID,"result_path":"/run/report.json","result_sha256":DIGEST});
    assert_valid("receipt", receipt.clone());
    let mut missing_receipt_field = receipt.clone();
    missing_receipt_field
        .as_object_mut()
        .unwrap()
        .remove("run_id");
    assert_invalid("receipt", missing_receipt_field);
    let mut wrong_evaluation_filename = receipt.clone();
    wrong_evaluation_filename["result_path"] = json!("/run/not-report.json");
    assert_invalid("receipt", wrong_evaluation_filename);
    let comparison_receipt = json!({"schema_version":2,"kind":"comparison","status":"complete","comparison_id":ID,"result_path":"/run/comparison.json","result_sha256":DIGEST});
    assert_valid("receipt", comparison_receipt.clone());
    let mut wrong_comparison_filename = comparison_receipt;
    wrong_comparison_filename["result_path"] = json!("/run/not-comparison.json");
    assert_invalid("receipt", wrong_comparison_filename);
    let error: Value = serde_json::from_str(
        &validator::Diagnostic::for_code(validator::DiagnosticCode::Schema)
            .to_machine_json()
            .unwrap(),
    )
    .unwrap();
    assert_valid("error", error.clone());
    let mut missing_error_field = error.clone();
    missing_error_field.as_object_mut().unwrap().remove("code");
    assert_invalid("error", missing_error_field);
}

#[test]
fn structure_dm01() {
    case_s17();
    case_m20();
}

#[test]
fn structure_dm02() {
    case_m14();
    policy_preconditions();
}

#[test]
fn structure_dm03() {
    case_m20();
    case_m13();
}

#[test]
fn structure_dm04() {
    case_m05();
    case_m14();
}

#[test]
fn structure_dm05() {
    case_m16();
}

#[test]
fn structure_dm07() {
    multi_label_abstentions_preserve_wire_evidence_in_report();
    decision_policy_boundaries();
}

#[test]
fn structure_dm08() {
    case_e06();
    assert_evaluation_error(
        "single_label",
        json!(["A", "B"]),
        json!([{"id":"01995c20-7d00-7000-8000-000000002708","expected":{"type":"class","label":"A"},"input":null}]),
        json!({"source":{"kind":"classifier","model":"oracle","configuration":{},"observation_definitions":{"distribution":{"kind":"categorical","description":"retained"}}}}),
        json!([{"id":"01995c20-7d00-7000-8000-000000002708","source_id":"source","outcome":{"type":"class","label":"A"},"observations":{"distribution":{"kind":"categorical","values":{"A":0.5,"B":0.49}}},"probabilities":{"kind":"categorical","values":{"A":0.5,"B":0.49}}}]),
        json!({"schema_version":2,"population":"observed versus scoring","role":"development","decision":{"type":"as_recorded"}}),
        validator::DiagnosticCode::Probability,
    );
}

#[test]
fn structure_dm09() {
    case_e05();
    case_e10();
}

#[test]
fn structure_dm10() {
    case_e11();
}

#[test]
fn structure_dm11() {
    equal_counts_distinct_exact_sets();
}

#[test]
fn structure_dm12() {
    case_m13();
    case_m14();
    case_m15();
    case_m21();
}
