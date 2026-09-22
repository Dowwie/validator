use std::{fs, process::Command};

use serde_json::{Value, json};

fn metric(value: f64) -> Value {
    json!({"value":value,"status":"defined","numerator":1,"denominator":1,"population_count":1})
}

fn report(correct: u64, metric_value: f64, answers: &[(&str, bool, &str)]) -> Value {
    let hard = json!({
        "kind":"single_label","total":10,"correct":correct,"wrong":10-correct,"abstained":0,"answered":10,
            "matrix":{"columns":[{"type":"label","label":"yes"},{"type":"label","label":"no"},{"type":"abstention"}],"rows":[[correct,1,0],[1,0,0]]},
        "classes":[{"label":"yes","support":5,"predicted_support":5,"true_positive":4,"false_positive":1,"false_negative":1,"precision":metric(0.8),"recall":metric(0.8),"f1":metric(0.8),"coverage":metric(1.0)},{"label":"no","support":5,"predicted_support":5,"true_positive":4,"false_positive":1,"false_negative":1,"precision":metric(0.8),"recall":metric(0.8),"f1":metric(0.8),"coverage":metric(1.0)}],
        "accuracy":metric(metric_value),"wrong_class_rate":metric(0.1),"abstention_rate":metric(0.0),"coverage":metric(1.0),"selective_accuracy":metric(metric_value),"selective_risk":metric(0.1),"macro_f1":{"metric":metric(0.8),"undefined_classes":[]}
    });
    json!({
        "schema_version":2,"kind":"evaluation","status":"complete","identity":{"run_id":"recorded","created_at":"not-a-date"},
        "artifacts":[{"kind":"golden","sha256":"intentionally-untrusted"}],"sources":{},"source_counts":{},"composition":"empty",
        "population":{"role":"held_out"},"task":{"kind":"single_label","labels":["yes","no"]},"policy":{"decision":{"type":"as_recorded"}},"integrity":{"signal_availability":"none"},
        "raw":hard,"final":hard,"probability":{"kind":"single_label","raw_answered_count":10,"choice_argmax_disagreement_count":null,"log_loss":metric(1.0),"brier_score":metric(0.5),"argmax_accuracy":metric(0.9)},
        "signals":{"kind":"single_label","maximum_probability":{"status":"none","population_scope":"all","population_count":0,"included_ids":[],"excluded_ids":[],"bins":[]},"confidence":{"status":"none","population_scope":"answered","population_count":0,"included_ids":[],"excluded_ids":[],"bins":[]},"top_label_ece":metric(0.99)},
        "episodes":answers.iter().map(|(id,final_correct,label)|json!({"id":id,"final_correct":final_correct,"final_outcome":{"type":"class","label":label}})).collect::<Vec<_>>()
    })
}

fn catalog_single_report() -> Value {
    let hard = |base: u64, undefined_class: &str| {
        json!({
            "kind":"single_label",
            "total":base + 1,"correct":base + 2,"wrong":base + 3,"abstained":base + 4,"answered":base + 5,
            "accuracy":metric((base + 10) as f64),"wrong_class_rate":metric((base + 11) as f64),"abstention_rate":metric((base + 12) as f64),"coverage":metric((base + 13) as f64),"selective_accuracy":metric((base + 14) as f64),"selective_risk":metric((base + 15) as f64),"macro_f1":{"metric":metric((base + 16) as f64),"undefined_classes":[undefined_class]},
            "classes":[
                {"label":"yes","support":base + 20,"predicted_support":base + 21,"true_positive":base + 22,"false_positive":base + 23,"false_negative":base + 24,"precision":metric((base + 25) as f64),"recall":metric((base + 26) as f64),"f1":metric((base + 27) as f64),"coverage":metric((base + 28) as f64)},
                {"label":"ABSTAIN","support":base + 30,"predicted_support":base + 31,"true_positive":base + 32,"false_positive":base + 33,"false_negative":base + 34,"precision":metric((base + 35) as f64),"recall":metric((base + 36) as f64),"f1":metric((base + 37) as f64),"coverage":metric((base + 38) as f64)}
            ],
            "matrix":{"columns":[{"type":"label","label":"yes"},{"type":"label","label":"ABSTAIN"},{"type":"abstention"}],"rows":[[base + 40,base + 41,base + 42],[base + 43,base + 44,base + 45]]}
        })
    };
    json!({
        "schema_version":2,"kind":"evaluation","status":"complete","identity":{"run_id":"single","created_at":"n/a"},
        "artifacts":[],"sources":{},"source_counts":{},"composition":"empty","population":{},"task":{"kind":"single_label","labels":["yes","ABSTAIN"]},"policy":{},"integrity":{"signal_availability":"top_label"},
        "raw":hard(100, "yes"),"final":hard(200, "ABSTAIN"),
        "probability":{"kind":"single_label","raw_answered_count":301,"choice_argmax_disagreement_count":302,"log_loss":metric(303.0),"brier_score":metric(304.0),"argmax_accuracy":metric(305.0)},
        "signals":{"kind":"single_label","maximum_probability":{"status":"defined","population_scope":"all","population_count":7,"included_ids":["m"],"excluded_ids":["mx"],"bins":[{"index":0,"lower":0.0,"upper":0.5,"upper_inclusive":false,"count":401,"correct_count":402,"mean_signal":0.991,"empirical_accuracy":0.113}]},"confidence":{"status":"defined","population_scope":"answered","population_count":8,"included_ids":["c"],"excluded_ids":["cx"],"bins":[{"index":0,"lower":0.0,"upper":0.5,"upper_inclusive":false,"count":411,"correct_count":412,"mean_signal":0.771,"empirical_accuracy":0.223}]},"top_label_ece":metric(306.0)},
        "episodes":[{"id":"s","final_correct":true,"final_outcome":{"type":"class","label":"yes"}}]
    })
}

fn catalog_multi_report() -> Value {
    let hard = |offset: u64, undefined_class: &str| {
        json!({
            "total":offset+1,"abstained":offset+2,"answered":offset+3,"exact_matches":offset+4,"wrong_sets":offset+5,
            "exact_match_accuracy":metric((offset+10) as f64),"wrong_set_rate":metric((offset+11) as f64),"coverage":metric((offset+12) as f64),"abstention_rate":metric((offset+13) as f64),"selective_exact_match_accuracy":metric((offset+14) as f64),"selective_risk":metric((offset+15) as f64),"answered_micro_precision":metric((offset+16) as f64),"answered_micro_recall":metric((offset+17) as f64),"answered_micro_f1":metric((offset+18) as f64),"answered_hamming_loss":metric((offset+19) as f64),"answered_macro_f1":{"metric":metric((offset+20) as f64),"undefined_classes":[undefined_class]},
            "labels":[
              {"label":"A","support":offset+21,"answered_support":offset+22,"predicted_support":offset+23,"true_positive":offset+24,"false_positive":offset+25,"false_negative":offset+26,"true_negative":offset+27,"precision":metric((offset+28) as f64),"recall":metric((offset+29) as f64),"f1":metric((offset+30) as f64)},
              {"label":"B","support":offset+31,"answered_support":offset+32,"predicted_support":offset+33,"true_positive":offset+34,"false_positive":offset+35,"false_negative":offset+36,"true_negative":offset+37,"precision":metric((offset+38) as f64),"recall":metric((offset+39) as f64),"f1":metric((offset+40) as f64)}
            ]
        })
    };
    json!({
      "schema_version":2,"kind":"evaluation","status":"complete","identity":{"run_id":"multi","created_at":"n/a"},"artifacts":[],"sources":{},"source_counts":{},"composition":"empty","population":{},"task":{"kind":"multi_label","labels":["A","B"]},"policy":{},"integrity":{"signal_availability":"marginals"},
      "raw":hard(100, "A"),"final":hard(200, "B"),
      "probability":{"kind":"multi_label","mean_binary_log_loss":metric(301.0),"mean_binary_brier":metric(302.0),"labels":[{"label":"A","binary_log_loss":metric(303.0),"binary_brier":metric(304.0)},{"label":"B","binary_log_loss":metric(305.0),"binary_brier":metric(306.0)}]},
      "signals":{"kind":"multi_label","status":"defined","labels":[{"label":"A","population_scope":"all","population_count":9,"included_ids":["a"],"excluded_ids":["ax"],"bins":[{"index":0,"lower":0.0,"upper":0.5,"upper_inclusive":false,"count":401,"positive_count":402,"mean_probability":0.345,"observed_positive_rate":0.876}]},{"label":"B","population_scope":"all","population_count":10,"included_ids":["b"],"excluded_ids":["bx"],"bins":[{"index":0,"lower":0.0,"upper":0.5,"upper_inclusive":false,"count":403,"positive_count":404,"mean_probability":0.456,"observed_positive_rate":0.765}]}]},
      "episodes":[{"id":"m","final_correct":true,"final_outcome":{"type":"labels","labels":["A"]}}]
    })
}

fn expected_catalog_row(
    task_kind: &str,
    family: &str,
    name: &str,
    dimensions: Value,
    kind: &str,
    value: Value,
) -> Value {
    let values = if task_kind == "single_label" {
        json!([value, null])
    } else {
        json!([null, value])
    };
    json!({"key":{"task_kind":task_kind,"family":family,"name":name,"dimensions":dimensions},"kind":kind,"values":values})
}

fn sort_expected_catalog(rows: &mut [Value]) {
    let canonical_dimensions = |dimensions: &Value| {
        fn canonical(value: &Value) -> String {
            match value {
                Value::Array(values) => format!(
                    "[{}]",
                    values.iter().map(canonical).collect::<Vec<_>>().join(",")
                ),
                Value::Object(values) => {
                    let mut entries = values.iter().collect::<Vec<_>>();
                    entries.sort_by_key(|(key, _)| *key);
                    format!(
                        "{{{}}}",
                        entries
                            .into_iter()
                            .map(|(key, value)| format!(
                                "{}:{}",
                                serde_json::to_string(key).unwrap(),
                                canonical(value)
                            ))
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                }
                _ => value.to_string(),
            }
        }
        canonical(dimensions)
    };
    rows.sort_by_key(|row| {
        let key = &row["key"];
        (
            key["task_kind"].as_str().unwrap().to_owned(),
            match key["family"].as_str().unwrap() {
                "raw" => 0,
                "final" => 1,
                "probability" => 2,
                "signals" => 3,
                _ => unreachable!(),
            },
            key["name"].as_str().unwrap().to_owned(),
            canonical_dimensions(&key["dimensions"]),
        )
    });
}

fn expected_single_catalog() -> Vec<Value> {
    let mut rows = Vec::new();
    for (family, base) in [("raw", 100_u64), ("final", 200)] {
        for (name, value) in [
            ("total", base + 1),
            ("correct", base + 2),
            ("wrong", base + 3),
            ("abstained", base + 4),
            ("answered", base + 5),
        ] {
            rows.push(expected_catalog_row(
                "single_label",
                family,
                name,
                json!({}),
                "count",
                json!(value),
            ));
        }
        for (name, value) in [
            ("accuracy", base + 10),
            ("wrong_class_rate", base + 11),
            ("abstention_rate", base + 12),
            ("coverage", base + 13),
            ("selective_accuracy", base + 14),
            ("selective_risk", base + 15),
            ("macro_f1", base + 16),
        ] {
            rows.push(expected_catalog_row(
                "single_label",
                family,
                name,
                json!({}),
                "metric",
                metric(value as f64),
            ));
        }
        for (label, offset) in [("yes", 20_u64), ("ABSTAIN", 30)] {
            for (name, value) in [
                ("support", base + offset),
                ("predicted_support", base + offset + 1),
                ("true_positive", base + offset + 2),
                ("false_positive", base + offset + 3),
                ("false_negative", base + offset + 4),
            ] {
                rows.push(expected_catalog_row(
                    "single_label",
                    family,
                    name,
                    json!({"label":label}),
                    "count",
                    json!(value),
                ));
            }
            for (name, value) in [
                ("precision", base + offset + 5),
                ("recall", base + offset + 6),
                ("f1", base + offset + 7),
                ("coverage", base + offset + 8),
            ] {
                rows.push(expected_catalog_row(
                    "single_label",
                    family,
                    name,
                    json!({"label":label}),
                    "metric",
                    metric(value as f64),
                ));
            }
        }
        for (actual, predicted, value) in [
            ("yes", json!({"type":"label","label":"yes"}), base + 40),
            ("yes", json!({"type":"label","label":"ABSTAIN"}), base + 41),
            ("yes", json!({"type":"abstention"}), base + 42),
            ("ABSTAIN", json!({"type":"label","label":"yes"}), base + 43),
            (
                "ABSTAIN",
                json!({"type":"label","label":"ABSTAIN"}),
                base + 44,
            ),
            ("ABSTAIN", json!({"type":"abstention"}), base + 45),
        ] {
            rows.push(expected_catalog_row(
                "single_label",
                family,
                "confusion_count",
                json!({"actual":actual,"predicted":predicted}),
                "count",
                json!(value),
            ));
        }
    }
    for (name, kind, value) in [
        ("raw_answered_count", "count", json!(301)),
        ("choice_argmax_disagreement_count", "count", json!(302)),
        ("log_loss", "metric", metric(303.0)),
        ("brier_score", "metric", metric(304.0)),
        ("argmax_accuracy", "metric", metric(305.0)),
    ] {
        rows.push(expected_catalog_row(
            "single_label",
            "probability",
            name,
            json!({}),
            kind,
            value,
        ));
    }
    rows.push(expected_catalog_row(
        "single_label",
        "signals",
        "top_label_ece",
        json!({}),
        "metric",
        metric(306.0),
    ));
    for (signal, count, correct_count, first_scalar, second_scalar) in [
        (
            "maximum_probability",
            401,
            402,
            ("mean_signal", json!(0.991)),
            ("empirical_accuracy", json!(0.113)),
        ),
        (
            "confidence",
            411,
            412,
            ("mean_signal", json!(0.771)),
            ("empirical_accuracy", json!(0.223)),
        ),
    ] {
        let dimensions = json!({"signal":signal,"label":null,"bin":{"index":0,"lower":0.0,"upper":0.5,"upper_inclusive":false}});
        rows.push(expected_catalog_row(
            "single_label",
            "signals",
            "count",
            dimensions.clone(),
            "count",
            json!(count),
        ));
        rows.push(expected_catalog_row(
            "single_label",
            "signals",
            "correct_count",
            dimensions.clone(),
            "count",
            json!(correct_count),
        ));
        rows.push(expected_catalog_row(
            "single_label",
            "signals",
            first_scalar.0,
            dimensions.clone(),
            "scalar",
            first_scalar.1,
        ));
        rows.push(expected_catalog_row(
            "single_label",
            "signals",
            second_scalar.0,
            dimensions,
            "scalar",
            second_scalar.1,
        ));
    }
    sort_expected_catalog(&mut rows);
    rows
}

fn expected_multi_catalog() -> Vec<Value> {
    let mut rows = Vec::new();
    for (family, base) in [("raw", 100_u64), ("final", 200)] {
        for (name, value) in [
            ("total", base + 1),
            ("abstained", base + 2),
            ("answered", base + 3),
            ("exact_matches", base + 4),
            ("wrong_sets", base + 5),
        ] {
            rows.push(expected_catalog_row(
                "multi_label",
                family,
                name,
                json!({}),
                "count",
                json!(value),
            ));
        }
        for (name, value) in [
            ("exact_match_accuracy", base + 10),
            ("wrong_set_rate", base + 11),
            ("coverage", base + 12),
            ("abstention_rate", base + 13),
            ("selective_exact_match_accuracy", base + 14),
            ("selective_risk", base + 15),
            ("answered_micro_precision", base + 16),
            ("answered_micro_recall", base + 17),
            ("answered_micro_f1", base + 18),
            ("answered_hamming_loss", base + 19),
            ("answered_macro_f1", base + 20),
        ] {
            rows.push(expected_catalog_row(
                "multi_label",
                family,
                name,
                json!({}),
                "metric",
                metric(value as f64),
            ));
        }
        for (label, offset) in [("A", 21_u64), ("B", 31)] {
            for (name, value) in [
                ("support", base + offset),
                ("answered_support", base + offset + 1),
                ("predicted_support", base + offset + 2),
                ("true_positive", base + offset + 3),
                ("false_positive", base + offset + 4),
                ("false_negative", base + offset + 5),
                ("true_negative", base + offset + 6),
            ] {
                rows.push(expected_catalog_row(
                    "multi_label",
                    family,
                    name,
                    json!({"label":label}),
                    "count",
                    json!(value),
                ));
            }
            for (name, value) in [
                ("precision", base + offset + 7),
                ("recall", base + offset + 8),
                ("f1", base + offset + 9),
            ] {
                rows.push(expected_catalog_row(
                    "multi_label",
                    family,
                    name,
                    json!({"label":label}),
                    "metric",
                    metric(value as f64),
                ));
            }
        }
    }
    for (name, value) in [
        ("mean_binary_log_loss", 301.0),
        ("mean_binary_brier", 302.0),
    ] {
        rows.push(expected_catalog_row(
            "multi_label",
            "probability",
            name,
            json!({}),
            "metric",
            metric(value),
        ));
    }
    for (label, log_loss, brier) in [("A", 303.0, 304.0), ("B", 305.0, 306.0)] {
        rows.push(expected_catalog_row(
            "multi_label",
            "probability",
            "binary_log_loss",
            json!({"label":label}),
            "metric",
            metric(log_loss),
        ));
        rows.push(expected_catalog_row(
            "multi_label",
            "probability",
            "binary_brier",
            json!({"label":label}),
            "metric",
            metric(brier),
        ));
    }
    for (label, count, positive_count, mean_probability, observed_positive_rate) in
        [("A", 401, 402, 0.345, 0.876), ("B", 403, 404, 0.456, 0.765)]
    {
        let dimensions = json!({"signal":"label_probability","label":label,"bin":{"index":0,"lower":0.0,"upper":0.5,"upper_inclusive":false}});
        rows.push(expected_catalog_row(
            "multi_label",
            "signals",
            "count",
            dimensions.clone(),
            "count",
            json!(count),
        ));
        rows.push(expected_catalog_row(
            "multi_label",
            "signals",
            "positive_count",
            dimensions.clone(),
            "count",
            json!(positive_count),
        ));
        rows.push(expected_catalog_row(
            "multi_label",
            "signals",
            "mean_probability",
            dimensions.clone(),
            "scalar",
            json!(mean_probability),
        ));
        rows.push(expected_catalog_row(
            "multi_label",
            "signals",
            "observed_positive_rate",
            dimensions,
            "scalar",
            json!(observed_positive_rate),
        ));
    }
    sort_expected_catalog(&mut rows);
    rows
}

fn compare_document(reports: &[Value]) -> Value {
    let directory = std::env::temp_dir().join(uuid::Uuid::now_v7().to_string());
    fs::create_dir(&directory).unwrap();
    let paths = reports
        .iter()
        .enumerate()
        .map(|(index, report)| {
            let path = directory.join(format!("{index}.json"));
            fs::write(&path, serde_json::to_vec(report).unwrap()).unwrap();
            path
        })
        .collect::<Vec<_>>();
    let output = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare"])
        .args(&paths)
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
    fs::remove_dir_all(directory).unwrap();
    serde_json::from_slice(&output.stdout).unwrap()
}

fn metric_row<'a>(document: &'a Value, family: &str, name: &str, dimensions: Value) -> &'a Value {
    document["metrics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["key"] == json!({"task_kind":"single_label","family":family,"name":name,"dimensions":dimensions})
        })
        .unwrap()
}

fn delta(document: &Value, family: &str, name: &str, dimensions: Value, index: usize) -> Value {
    metric_row(document, family, name, dimensions)["change_from_previous"][index].clone()
}

#[test]
fn compare_extracts_hand_authored_complete_single_and_multi_catalogs() {
    let directory = std::env::temp_dir().join(uuid::Uuid::now_v7().to_string());
    fs::create_dir(&directory).unwrap();
    let single = directory.join("single.json");
    let multi = directory.join("multi.json");
    fs::write(
        &single,
        serde_json::to_vec(&catalog_single_report()).unwrap(),
    )
    .unwrap();
    fs::write(&multi, serde_json::to_vec(&catalog_multi_report()).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare"])
        .args([&single, &multi])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    let actual_catalog = document["metrics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| json!({"key":row["key"],"kind":row["kind"],"values":row["values"]}))
        .collect::<Vec<_>>();
    let expected_single = expected_single_catalog();
    let expected_multi = expected_multi_catalog();
    assert_eq!(expected_single.len(), 86);
    assert_eq!(expected_multi.len(), 86);
    assert_eq!(
        actual_catalog
            .iter()
            .filter(|row| row["key"]["task_kind"] == "single_label")
            .count(),
        86
    );
    assert_eq!(
        actual_catalog
            .iter()
            .filter(|row| row["key"]["task_kind"] == "multi_label")
            .count(),
        86
    );
    let expected_catalog = expected_multi
        .into_iter()
        .chain(expected_single)
        .collect::<Vec<_>>();
    assert_eq!(
        document["runs"][0]["macro_undefined_labels"],
        json!({"raw":["yes"],"final":["ABSTAIN"]})
    );
    assert_eq!(
        document["runs"][1]["macro_undefined_labels"],
        json!({"raw":["A"],"final":["B"]})
    );
    assert_eq!(
        document["runs"][0]["signal_populations"],
        json!([
            {"signal":"maximum_probability","label":null,"status":"defined","population_scope":"all","population_count":7,"included_ids":["m"],"excluded_ids":["mx"]},
            {"signal":"confidence","label":null,"status":"defined","population_scope":"answered","population_count":8,"included_ids":["c"],"excluded_ids":["cx"]}
        ])
    );
    assert_eq!(
        document["runs"][1]["signal_populations"],
        json!([
            {"signal":"label_probability","label":"A","status":"defined","population_scope":"all","population_count":9,"included_ids":["a"],"excluded_ids":["ax"]},
            {"signal":"label_probability","label":"B","status":"defined","population_scope":"all","population_count":10,"included_ids":["b"],"excluded_ids":["bx"]}
        ])
    );
    assert_eq!(actual_catalog, expected_catalog);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn compare_uses_recorded_series_values_and_case_relations() {
    let directory = std::env::temp_dir().join(uuid::Uuid::now_v7().to_string());
    fs::create_dir(&directory).unwrap();
    let reports = [
        report(
            8,
            0.123,
            &[
                ("a", true, "yes"),
                ("b", false, "no"),
                ("c", true, "yes"),
                ("d", false, "no"),
            ],
        ),
        report(
            9,
            0.456,
            &[
                ("a", true, "yes"),
                ("b", true, "yes"),
                ("c", false, "no"),
                ("d", false, "no"),
            ],
        ),
        report(
            7,
            0.789,
            &[
                ("a", false, "no"),
                ("b", true, "yes"),
                ("c", true, "yes"),
                ("d", false, "no"),
            ],
        ),
    ];
    let paths = reports
        .iter()
        .enumerate()
        .map(|(index, report)| {
            let path = directory.join(format!("{index}.json"));
            fs::write(&path, serde_json::to_vec(report).unwrap()).unwrap();
            path
        })
        .collect::<Vec<_>>();
    let output = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare"])
        .args(&paths)
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    let schema: Value =
        serde_json::from_str(include_str!("../schemas/v3/comparison.schema.json")).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    assert!(validator.is_valid(&value));
    let mut missing_runs = value.clone();
    missing_runs.as_object_mut().unwrap().remove("runs");
    assert!(!validator.is_valid(&missing_runs));
    assert_eq!(value["schema_version"], 3);
    let correct = value["metrics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["key"]["family"] == "final" && item["key"]["name"] == "correct")
        .unwrap();
    assert_eq!(correct["values"], json!([8, 9, 7]));
    assert_eq!(correct["change_from_previous"][1]["value"], 1);
    assert_eq!(correct["change_from_previous"][2]["value"], -2);
    assert_eq!(correct["change_from_first"][2]["value"], -1);
    assert_eq!(value["case_changes"].as_array().unwrap().len(), 3);
    assert_eq!(value["case_changes"][0]["recovered"]["ids"], json!(["b"]));
    assert_eq!(value["case_changes"][0]["regressed"]["ids"], json!(["c"]));
    assert_eq!(value["case_changes"][2]["recovered"]["ids"], json!(["b"]));
    assert_eq!(value["case_changes"][2]["regressed"]["ids"], json!(["a"]));
    let old_options = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare", "--baseline"])
        .output()
        .unwrap();
    assert_eq!(old_options.status.code(), Some(2));
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn compare_preserves_delta_availability_precision_and_recorded_special_values() {
    let mut first = report(1, 0.0, &[("one", false, "yes")]);
    let mut second = report(2, 0.0, &[("one", false, "yes")]);
    let mut third = report(3, 0.0, &[("one", false, "yes")]);
    first["raw"]["total"] = json!(0_u64);
    second["raw"]["total"] = json!(u64::MAX);
    third["raw"]["total"] = json!(0_u64);
    first["raw"]["correct"] = Value::Null;
    second["raw"]["correct"] = json!(1);
    third["raw"]["correct"] = Value::Null;
    first["raw"]["accuracy"] = json!({"value":null,"status":"not_applicable"});
    second["raw"]["accuracy"] = metric(f64::MAX);
    third["raw"]["accuracy"] = json!({"value":null,"status":"no_data"});
    first["raw"]["macro_f1"]["metric"] = metric(0.1);
    second["raw"]["macro_f1"]["metric"] = json!({"value":0.2,"status":"partial"});
    third["raw"]["macro_f1"]["metric"] = metric(0.3);
    first["probability"]["brier_score"] =
        json!({"value":null,"status":"defined","special_value":"positive_infinity"});
    second["probability"]["brier_score"] = metric(1.0);
    third["probability"]["brier_score"] = metric(2.0);
    first["probability"]["argmax_accuracy"] = metric(0.0);
    second["probability"]["argmax_accuracy"] = metric(-0.0);
    third["probability"]["argmax_accuracy"] = metric(1.0);
    first["probability"]["log_loss"] = metric(f64::MAX);
    second["probability"]["log_loss"] = metric(-f64::MAX);
    third["probability"]["log_loss"] = metric(0.0);
    first["raw"]["selective_accuracy"] = json!({"status":"defined"});
    second["raw"]["selective_accuracy"] = metric(1.0);
    third["raw"]["selective_accuracy"] = metric(2.0);
    for (report, mean_signal) in [
        (&mut first, json!(null)),
        (&mut second, json!(0.4)),
        (&mut third, json!(null)),
    ] {
        report["signals"]["maximum_probability"]["status"] = json!("defined");
        report["signals"]["maximum_probability"]["bins"] = json!([{"index":0,"lower":0.0,"upper":1.0,"upper_inclusive":true,"count":0,"correct_count":0,"mean_signal":mean_signal,"empirical_accuracy":null}]);
    }
    let document = compare_document(&[first, second, third]);
    assert_eq!(
        metric_row(&document, "raw", "total", json!({}))["change_from_previous"],
        json!([
            {"value":null,"reason":"no_reference"},
            {"value":18446744073709551615_i128,"reason":null},
            {"value":-18446744073709551615_i128,"reason":null}
        ])
    );
    assert_eq!(
        metric_row(&document, "raw", "correct", json!({}))["change_from_previous"],
        json!([
            {"value":null,"reason":"no_reference"},
            {"value":null,"reason":"reference_missing"},
            {"value":null,"reason":"current_missing"}
        ])
    );
    assert_eq!(
        delta(&document, "raw", "accuracy", json!({}), 1),
        json!({"value":null,"reason":"reference_not_defined"})
    );
    assert_eq!(
        delta(&document, "raw", "accuracy", json!({}), 2),
        json!({"value":null,"reason":"current_not_defined"})
    );
    assert_eq!(
        delta(&document, "raw", "macro_f1", json!({}), 1),
        json!({"value":null,"reason":"current_not_defined"})
    );
    assert_eq!(
        delta(&document, "raw", "macro_f1", json!({}), 2),
        json!({"value":null,"reason":"reference_not_defined"})
    );
    assert_eq!(
        metric_row(&document, "probability", "brier_score", json!({}))["values"][0],
        json!({"value":null,"status":"defined","special_value":"positive_infinity"})
    );
    assert_eq!(
        delta(&document, "probability", "brier_score", json!({}), 1),
        json!({"value":null,"reason":"value_unavailable"})
    );
    assert_eq!(
        delta(&document, "raw", "selective_accuracy", json!({}), 1),
        json!({"value":null,"reason":"value_unavailable"})
    );
    assert_eq!(
        delta(&document, "probability", "log_loss", json!({}), 1),
        json!({"value":null,"reason":"non_finite_delta"})
    );
    assert_eq!(
        delta(
            &document,
            "signals",
            "mean_signal",
            json!({"signal":"maximum_probability","label":null,"bin":{"index":0,"lower":0.0,"upper":1.0,"upper_inclusive":true}}),
            1
        ),
        json!({"value":null,"reason":"reference_missing"})
    );
    assert_eq!(
        delta(
            &document,
            "signals",
            "mean_signal",
            json!({"signal":"maximum_probability","label":null,"bin":{"index":0,"lower":0.0,"upper":1.0,"upper_inclusive":true}}),
            2
        ),
        json!({"value":null,"reason":"current_missing"})
    );
    let zero = delta(&document, "probability", "argmax_accuracy", json!({}), 1)["value"]
        .as_f64()
        .unwrap();
    assert_eq!(zero, 0.0);
    assert!(zero.is_sign_positive());
}

#[test]
fn compare_joins_coordinates_and_applies_stored_case_outcomes() {
    let mut first = catalog_single_report();
    let mut second = catalog_single_report();
    second["task"]["labels"] = json!(["ABSTAIN", "yes"]);
    for family in ["raw", "final"] {
        second[family]["classes"].as_array_mut().unwrap().reverse();
        second[family]["matrix"]["rows"]
            .as_array_mut()
            .unwrap()
            .reverse();
        second[family]["matrix"]["columns"] = json!([{"type":"abstention"},{"type":"label","label":"ABSTAIN"},{"type":"label","label":"yes"}]);
        for row in second[family]["matrix"]["rows"].as_array_mut().unwrap() {
            let old = row.as_array().unwrap().clone();
            *row = json!([old[2], old[1], old[0]]);
        }
    }
    second["artifacts"] = json!([{"kind":"golden","sha256":"different"}]);
    second["population"] = json!({"role":"development"});
    second["policy"] = json!({"decision":{"type":"different"}});
    first["signals"]["maximum_probability"]["bins"]
        .as_array_mut()
        .unwrap()
        .push(json!({"index":1,"lower":0.5,"upper":1.0,"upper_inclusive":true,"count":501,"correct_count":502,"mean_signal":0.501,"empirical_accuracy":0.502}));
    second["signals"]["maximum_probability"]["bins"]
        .as_array_mut()
        .unwrap()
        .push(json!({"index":1,"lower":0.5,"upper":1.0,"upper_inclusive":true,"count":501,"correct_count":502,"mean_signal":0.501,"empirical_accuracy":0.502}));
    second["signals"]["maximum_probability"]["bins"][0]["upper"] = json!(0.6);
    second["signals"]["maximum_probability"]["bins"][0]["upper_inclusive"] = json!(true);
    second["signals"]["maximum_probability"]["bins"]
        .as_array_mut()
        .unwrap()
        .reverse();
    first["episodes"] = json!([
        {"id":"shared","final_correct":false,"final_outcome":{"type":"class","label":"yes"}},
        {"id":"only-first","final_correct":true,"final_outcome":{"type":"class","label":"yes"}}
    ]);
    second["episodes"] = json!([
        {"id":"shared","final_correct":false,"final_outcome":{"type":"class","label":"ABSTAIN"}},
        {"id":"only-second","final_correct":false,"final_outcome":{"type":"class","label":"yes"}}
    ]);
    let document = compare_document(&[first, second]);
    assert_eq!(
        metric_row(&document, "raw", "support", json!({"label":"yes"}))["values"],
        json!([120, 120])
    );
    assert_eq!(
        metric_row(
            &document,
            "raw",
            "confusion_count",
            json!({"actual":"yes","predicted":{"type":"label","label":"yes"}})
        )["values"],
        json!([140, 140])
    );
    assert_eq!(
        metric_row(
            &document,
            "signals",
            "mean_signal",
            json!({"signal":"maximum_probability","label":null,"bin":{"index":1,"lower":0.5,"upper":1.0,"upper_inclusive":true}})
        )["values"],
        json!([0.501, 0.501])
    );
    assert_eq!(
        metric_row(
            &document,
            "signals",
            "mean_signal",
            json!({"signal":"maximum_probability","label":null,"bin":{"index":0,"lower":0.0,"upper":0.5,"upper_inclusive":false}})
        )["values"],
        json!([0.991, null])
    );
    assert_eq!(
        metric_row(
            &document,
            "signals",
            "mean_signal",
            json!({"signal":"maximum_probability","label":null,"bin":{"index":0,"lower":0.0,"upper":0.6,"upper_inclusive":true}})
        )["values"],
        json!([null, 0.991])
    );
    let relation = &document["case_changes"][0];
    assert_eq!(relation["shared_ids"], json!(["shared"]));
    assert_eq!(relation["only_from_ids"], json!(["only-first"]));
    assert_eq!(relation["only_to_ids"], json!(["only-second"]));
    assert_eq!(
        relation["neither_correct"],
        json!({"count":1,"ids":["shared"]})
    );
    assert_eq!(
        relation["changed_final_outcomes"],
        json!({"count":1,"ids":["shared"]})
    );

    let mut multi_first = catalog_multi_report();
    let mut multi_second = catalog_multi_report();
    multi_first["episodes"] = json!([
        {"id":"empty","final_correct":false,"final_outcome":{"type":"labels","labels":[]}},
        {"id":"set","final_correct":false,"final_outcome":{"type":"labels","labels":["A","B"]}},
        {"id":"wrong","final_correct":false,"final_outcome":{"type":"labels","labels":["A"]}},
        {"id":"reason","final_correct":false,"final_outcome":{"type":"abstention","reason":"first"}},
        {"id":"only-first","final_correct":true,"final_outcome":{"type":"labels","labels":["A"]}}
    ]);
    multi_second["episodes"] = json!([
        {"id":"empty","final_correct":false,"final_outcome":{"type":"abstention"}},
        {"id":"set","final_correct":false,"final_outcome":{"type":"labels","labels":["B","A"]}},
        {"id":"wrong","final_correct":false,"final_outcome":{"type":"labels","labels":["B"]}},
        {"id":"reason","final_correct":false,"final_outcome":{"type":"abstention","reason":"second"}},
        {"id":"only-second","final_correct":false,"final_outcome":{"type":"labels","labels":["A"]}}
    ]);
    let multi = compare_document(&[multi_first, multi_second]);
    let relation = &multi["case_changes"][0];
    assert_eq!(
        relation["neither_correct"],
        json!({"count":4,"ids":["empty","reason","set","wrong"]})
    );
    assert_eq!(
        relation["changed_final_outcomes"],
        json!({"count":2,"ids":["empty","wrong"]})
    );
    assert_eq!(relation["only_from_ids"], json!(["only-first"]));
    assert_eq!(relation["only_to_ids"], json!(["only-second"]));
}

#[test]
fn compare_cli_is_deterministic_ordered_and_has_report_only_error_boundaries() {
    let directory = std::env::temp_dir().join(uuid::Uuid::now_v7().to_string());
    fs::create_dir(&directory).unwrap();
    let first = directory.join("first.json");
    let second = directory.join("second.json");
    let mut first_report = report(1, 0.1, &[("a", true, "yes")]);
    first_report["identity"]["run_id"] = json!("z");
    first_report["identity"]["created_at"] = json!("later");
    let mut second_report = report(2, 0.2, &[("a", false, "no")]);
    second_report["identity"]["run_id"] = json!("a");
    second_report["identity"]["created_at"] = json!("earlier");
    fs::write(&first, serde_json::to_vec(&first_report).unwrap()).unwrap();
    fs::write(&second, serde_json::to_vec(&second_report).unwrap()).unwrap();
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_validator"))
            .args(["compare"])
            .args([&first, &second, &first])
            .output()
            .unwrap()
    };
    let one = invoke();
    let two = invoke();
    assert!(one.status.success());
    assert_eq!(one.stdout, two.stdout);
    let document: Value = serde_json::from_slice(&one.stdout).unwrap();
    assert_eq!(document["runs"][0]["identity"]["run_id"], "z");
    assert_eq!(document["runs"][1]["identity"]["run_id"], "a");
    assert_eq!(document["runs"][2]["identity"]["run_id"], "z");
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 2);
    let too_few = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare", first.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(too_few.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<Value>(&too_few.stdout).unwrap()["code"],
        "E_SCHEMA"
    );
    let old_flag = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare", "--baseline"])
        .output()
        .unwrap();
    assert_eq!(old_flag.status.code(), Some(2));
    let malformed = directory.join("malformed.json");
    fs::write(&malformed, "not json").unwrap();
    let malformed = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare"])
        .args([&first, &malformed])
        .output()
        .unwrap();
    assert_eq!(malformed.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<Value>(&malformed.stdout).unwrap()["code"],
        "E_PARSE"
    );
    let wrong_type = directory.join("wrong-type.json");
    fs::write(
        &wrong_type,
        br#"{"schema_version":"2","kind":"evaluation","task":{"kind":"single_label"}}"#,
    )
    .unwrap();
    let wrong_type = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare"])
        .args([&first, &wrong_type])
        .output()
        .unwrap();
    assert_eq!(wrong_type.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<Value>(&wrong_type.stdout).unwrap()["code"],
        "E_SCHEMA"
    );
    let wrong_consumed = directory.join("wrong-consumed.json");
    let mut wrong_consumed_report = first_report.clone();
    wrong_consumed_report["raw"]["total"] = json!("ten");
    fs::write(
        &wrong_consumed,
        serde_json::to_vec(&wrong_consumed_report).unwrap(),
    )
    .unwrap();
    let multi = directory.join("multi.json");
    fs::write(&multi, serde_json::to_vec(&catalog_multi_report()).unwrap()).unwrap();
    let wrong_consumed = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare"])
        .args([&wrong_consumed, &multi])
        .output()
        .unwrap();
    assert_eq!(wrong_consumed.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<Value>(&wrong_consumed.stdout).unwrap()["code"],
        "E_SCHEMA"
    );
    let missing = Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(["compare"])
        .args([&first, &directory.join("missing.json")])
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(3));
    assert_eq!(
        serde_json::from_slice::<Value>(&missing.stdout).unwrap()["code"],
        "E_IO"
    );
    let dashed = directory.join("-report.json");
    fs::write(&dashed, serde_json::to_vec(&first_report).unwrap()).unwrap();
    let dashed = Command::new(env!("CARGO_BIN_EXE_validator"))
        .current_dir(&directory)
        .args(["compare", "--", "-report.json", "second.json"])
        .output()
        .unwrap();
    assert!(dashed.status.success());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn comparison_schema_rejects_wrong_nested_shapes() {
    let document = compare_document(&[
        report(1, 0.1, &[("a", true, "yes")]),
        report(2, 0.2, &[("a", false, "no")]),
    ]);
    let schema: Value =
        serde_json::from_str(include_str!("../schemas/v3/comparison.schema.json")).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    assert!(validator.is_valid(&document));
    let mut wrong_kind = document.clone();
    wrong_kind["metrics"][0]["kind"] = json!("other");
    assert!(!validator.is_valid(&wrong_kind));
    let mut wrong_key = document.clone();
    wrong_key["metrics"][0]["key"]["task_kind"] = json!(1);
    assert!(!validator.is_valid(&wrong_key));
    let mut wrong_dimensions = document.clone();
    wrong_dimensions["metrics"][0]["key"]["dimensions"] = json!([]);
    assert!(!validator.is_valid(&wrong_dimensions));
    let mut wrong_delta = document.clone();
    wrong_delta["metrics"][0]["change_from_previous"][0]["reason"] = json!(1);
    assert!(!validator.is_valid(&wrong_delta));
    let mut wrong_run = document.clone();
    wrong_run["runs"][0]["index"] = json!("0");
    assert!(!validator.is_valid(&wrong_run));
}
