use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Number, Value, json};

use crate::{Diagnostic, DiagnosticCode, Result};

pub(crate) const COMPARISON_SCHEMA_VERSION: u32 = 3;

#[derive(Clone, Debug)]
pub(crate) struct ReportInput {
    pub(crate) path: String,
    pub(crate) value: Value,
}

#[derive(Clone, Copy)]
enum CellKind {
    Count,
    Metric,
    Scalar,
}
impl CellKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Metric => "metric",
            Self::Scalar => "scalar",
        }
    }
}

struct Item {
    task: String,
    family: &'static str,
    name: &'static str,
    dimensions: Value,
    kind: CellKind,
    value: Value,
}
struct Series {
    task: String,
    family: &'static str,
    name: &'static str,
    dimensions: Value,
    kind: CellKind,
    values: Vec<Value>,
}

pub(crate) fn decode_report(path: String, bytes: &str) -> Result<ReportInput> {
    let value: Value =
        serde_json::from_str(bytes).map_err(|_| Diagnostic::for_code(DiagnosticCode::Parse))?;
    let root = object(&value)?;
    if root.get("schema_version").and_then(Value::as_u64) != Some(2)
        || root.get("kind").and_then(Value::as_str) != Some("evaluation")
        || !matches!(task_kind(root)?, "single_label" | "multi_label")
    {
        return Err(schema());
    }
    Ok(ReportInput { path, value })
}

pub(crate) fn aggregate(reports: &[ReportInput]) -> Result<Value> {
    if reports.len() < 2 {
        return Err(schema());
    }
    let runs = reports
        .iter()
        .enumerate()
        .map(|(i, report)| metadata(i, report))
        .collect::<Result<Vec<_>>>()?;
    let mut series = BTreeMap::new();
    for (index, report) in reports.iter().enumerate() {
        for item in extract(&report.value)? {
            let id = key(&item.task, item.family, item.name, &item.dimensions)?;
            let entry = series.entry(id).or_insert_with(|| Series {
                task: item.task.clone(),
                family: item.family,
                name: item.name,
                dimensions: item.dimensions.clone(),
                kind: item.kind,
                values: vec![Value::Null; reports.len()],
            });
            entry.values[index] = item.value;
        }
    }
    Ok(
        json!({"schema_version":COMPARISON_SCHEMA_VERSION,"kind":"comparison","status":"complete","runs":runs,"metrics":series.into_values().map(series_value).collect::<Result<Vec<_>>>()?,"case_changes":relations(reports)?}),
    )
}

fn metadata(index: usize, report: &ReportInput) -> Result<Value> {
    let root = object(&report.value)?;
    let task = task_kind(root)?;
    let dataset = root
        .get("artifacts")
        .and_then(Value::as_array)
        .and_then(|items| {
            items
                .iter()
                .find(|item| item.get("kind").and_then(Value::as_str) == Some("golden"))
                .and_then(|item| item.get("sha256"))
                .cloned()
        })
        .unwrap_or(Value::Null);
    let macro_name = if task == "single_label" {
        "macro_f1"
    } else {
        "answered_macro_f1"
    };
    Ok(
        json!({"index":index,"report_path":report.path,"identity":copy(root,"identity"),"task":copy(root,"task"),"population":copy(root,"population"),"policy":copy(root,"policy"),"sources":copy(root,"sources"),"source_counts":copy(root,"source_counts"),"composition":copy(root,"composition"),"integrity":copy(root,"integrity"),"dataset_sha256":dataset,"macro_undefined_labels":{"raw":root.get("raw").and_then(|value|value.get(macro_name)).and_then(|value|value.get("undefined_classes")).cloned().unwrap_or(Value::Null),"final":root.get("final").and_then(|value|value.get(macro_name)).and_then(|value|value.get("undefined_classes")).cloned().unwrap_or(Value::Null)},"signal_populations":signal_context(root,task)?}),
    )
}

fn signal_context(root: &Map<String, Value>, task: &str) -> Result<Value> {
    let Some(signals) = root.get("signals").and_then(Value::as_object) else {
        return Ok(json!([]));
    };
    let mut output = Vec::new();
    if task == "single_label" {
        for signal in ["maximum_probability", "confidence"] {
            if let Some(value) = signals.get(signal).and_then(Value::as_object) {
                output.push(json!({"signal":signal,"label":null,"status":copy(value,"status"),"population_scope":copy(value,"population_scope"),"population_count":copy(value,"population_count"),"included_ids":copy(value,"included_ids"),"excluded_ids":copy(value,"excluded_ids")}));
            }
        }
    } else if let Some(labels) = signals.get("labels").and_then(Value::as_array) {
        let mut labels = labels.iter().collect::<Vec<_>>();
        labels.sort_by_key(|value| {
            value
                .get("label")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .as_bytes()
                .to_vec()
        });
        for value in labels {
            let value = object(value)?;
            output.push(json!({"signal":"label_probability","label":string(value,"label")?,"status":copy(signals,"status"),"population_scope":copy(value,"population_scope"),"population_count":copy(value,"population_count"),"included_ids":copy(value,"included_ids"),"excluded_ids":copy(value,"excluded_ids")}));
        }
    }
    Ok(Value::Array(output))
}

fn extract(report: &Value) -> Result<Vec<Item>> {
    let root = object(report)?;
    let task = task_kind(root)?.to_owned();
    let mut out = Vec::new();
    for family in ["raw", "final"] {
        let hard = root
            .get(family)
            .and_then(Value::as_object)
            .ok_or_else(schema)?;
        if task == "single_label" {
            single_hard(&mut out, &task, family, hard, root)?;
        } else {
            multi_hard(&mut out, &task, family, hard)?;
        }
    }
    let probability = root
        .get("probability")
        .and_then(Value::as_object)
        .ok_or_else(schema)?;
    if task == "single_label" {
        single_probability(&mut out, &task, probability)?;
    } else {
        multi_probability(&mut out, &task, probability)?;
    }
    let signals = root
        .get("signals")
        .and_then(Value::as_object)
        .ok_or_else(schema)?;
    if task == "single_label" {
        single_signals(&mut out, &task, signals)?;
    } else {
        multi_signals(&mut out, &task, signals)?;
    }
    for item in &out {
        validate_cell(item)?;
    }
    Ok(out)
}

fn validate_cell(item: &Item) -> Result<()> {
    match item.kind {
        CellKind::Count if item.value.is_null() || item.value.as_u64().is_some() => Ok(()),
        CellKind::Metric => {
            let metric = object(&item.value)?;
            string(metric, "status")?;
            match metric.get("value") {
                None | Some(Value::Null | Value::Number(_)) => Ok(()),
                _ => Err(schema()),
            }
        }
        CellKind::Scalar if item.value.is_null() || item.value.as_f64().is_some() => Ok(()),
        _ => Err(schema()),
    }
}

fn single_hard(
    out: &mut Vec<Item>,
    task: &str,
    family: &'static str,
    hard: &Map<String, Value>,
    root: &Map<String, Value>,
) -> Result<()> {
    fields(
        out,
        task,
        family,
        hard,
        &["total", "correct", "wrong", "abstained", "answered"],
        CellKind::Count,
        &json!({}),
    )?;
    fields(
        out,
        task,
        family,
        hard,
        &[
            "accuracy",
            "wrong_class_rate",
            "abstention_rate",
            "coverage",
            "selective_accuracy",
            "selective_risk",
        ],
        CellKind::Metric,
        &json!({}),
    )?;
    add(
        out,
        task,
        family,
        "macro_f1",
        json!({}),
        CellKind::Metric,
        required(object(required(hard, "macro_f1")?)?, "metric")?.clone(),
    );
    for row in required(hard, "classes")?.as_array().ok_or_else(schema)? {
        let row = object(row)?;
        let dimensions = json!({"label":string(row,"label")?});
        fields(
            out,
            task,
            family,
            row,
            &[
                "support",
                "predicted_support",
                "true_positive",
                "false_positive",
                "false_negative",
            ],
            CellKind::Count,
            &dimensions,
        )?;
        fields(
            out,
            task,
            family,
            row,
            &["precision", "recall", "f1", "coverage"],
            CellKind::Metric,
            &dimensions,
        )?;
    }
    let labels = root
        .get("task")
        .and_then(|value| value.get("labels"))
        .and_then(Value::as_array)
        .ok_or_else(schema)?;
    let matrix = object(required(hard, "matrix")?)?;
    let columns = required(matrix, "columns")?.as_array().ok_or_else(schema)?;
    for (row_index, row) in required(matrix, "rows")?
        .as_array()
        .ok_or_else(schema)?
        .iter()
        .enumerate()
    {
        let actual = labels
            .get(row_index)
            .and_then(Value::as_str)
            .ok_or_else(schema)?;
        for (column_index, cell) in row.as_array().ok_or_else(schema)?.iter().enumerate() {
            let column = object(columns.get(column_index).ok_or_else(schema)?)?;
            let predicted = match column.get("type").and_then(Value::as_str) {
                Some("label") => json!({"type":"label","label":string(column,"label")?}),
                Some("abstention") => json!({"type":"abstention"}),
                _ => return Err(schema()),
            };
            add(
                out,
                task,
                family,
                "confusion_count",
                json!({"actual":actual,"predicted":predicted}),
                CellKind::Count,
                cell.clone(),
            );
        }
    }
    Ok(())
}

fn multi_hard(
    out: &mut Vec<Item>,
    task: &str,
    family: &'static str,
    hard: &Map<String, Value>,
) -> Result<()> {
    fields(
        out,
        task,
        family,
        hard,
        &[
            "total",
            "abstained",
            "answered",
            "exact_matches",
            "wrong_sets",
        ],
        CellKind::Count,
        &json!({}),
    )?;
    fields(
        out,
        task,
        family,
        hard,
        &[
            "exact_match_accuracy",
            "wrong_set_rate",
            "coverage",
            "abstention_rate",
            "selective_exact_match_accuracy",
            "selective_risk",
            "answered_micro_precision",
            "answered_micro_recall",
            "answered_micro_f1",
            "answered_hamming_loss",
        ],
        CellKind::Metric,
        &json!({}),
    )?;
    add(
        out,
        task,
        family,
        "answered_macro_f1",
        json!({}),
        CellKind::Metric,
        required(object(required(hard, "answered_macro_f1")?)?, "metric")?.clone(),
    );
    for row in required(hard, "labels")?.as_array().ok_or_else(schema)? {
        let row = object(row)?;
        let dimensions = json!({"label":string(row,"label")?});
        fields(
            out,
            task,
            family,
            row,
            &[
                "support",
                "answered_support",
                "predicted_support",
                "true_positive",
                "false_positive",
                "false_negative",
                "true_negative",
            ],
            CellKind::Count,
            &dimensions,
        )?;
        fields(
            out,
            task,
            family,
            row,
            &["precision", "recall", "f1"],
            CellKind::Metric,
            &dimensions,
        )?;
    }
    Ok(())
}

fn single_probability(
    out: &mut Vec<Item>,
    task: &str,
    probability: &Map<String, Value>,
) -> Result<()> {
    fields(
        out,
        task,
        "probability",
        probability,
        &["raw_answered_count", "choice_argmax_disagreement_count"],
        CellKind::Count,
        &json!({}),
    )?;
    fields(
        out,
        task,
        "probability",
        probability,
        &["log_loss", "brier_score", "argmax_accuracy"],
        CellKind::Metric,
        &json!({}),
    )
}
fn multi_probability(
    out: &mut Vec<Item>,
    task: &str,
    probability: &Map<String, Value>,
) -> Result<()> {
    fields(
        out,
        task,
        "probability",
        probability,
        &["mean_binary_log_loss", "mean_binary_brier"],
        CellKind::Metric,
        &json!({}),
    )?;
    for row in required(probability, "labels")?
        .as_array()
        .ok_or_else(schema)?
    {
        let row = object(row)?;
        let dimensions = json!({"label":string(row,"label")?});
        fields(
            out,
            task,
            "probability",
            row,
            &["binary_log_loss", "binary_brier"],
            CellKind::Metric,
            &dimensions,
        )?;
    }
    Ok(())
}
fn single_signals(out: &mut Vec<Item>, task: &str, signals: &Map<String, Value>) -> Result<()> {
    fields(
        out,
        task,
        "signals",
        signals,
        &["top_label_ece"],
        CellKind::Metric,
        &json!({}),
    )?;
    for signal in ["maximum_probability", "confidence"] {
        bins(
            out,
            task,
            signal,
            Value::Null,
            object(required(signals, signal)?)?,
            &["count", "correct_count"],
            &["mean_signal", "empirical_accuracy"],
        )?;
    }
    Ok(())
}
fn multi_signals(out: &mut Vec<Item>, task: &str, signals: &Map<String, Value>) -> Result<()> {
    for row in required(signals, "labels")?.as_array().ok_or_else(schema)? {
        let row = object(row)?;
        bins(
            out,
            task,
            "label_probability",
            Value::String(string(row, "label")?.to_owned()),
            row,
            &["count", "positive_count"],
            &["mean_probability", "observed_positive_rate"],
        )?;
    }
    Ok(())
}
fn bins(
    out: &mut Vec<Item>,
    task: &str,
    signal: &'static str,
    label: Value,
    value: &Map<String, Value>,
    counts: &[&'static str],
    scalars: &[&'static str],
) -> Result<()> {
    for bin in required(value, "bins")?.as_array().ok_or_else(schema)? {
        let bin = object(bin)?;
        let dims = json!({"signal":signal,"label":label,"bin":{"index":required(bin,"index")?,"lower":required(bin,"lower")?,"upper":required(bin,"upper")?,"upper_inclusive":required(bin,"upper_inclusive")?}});
        fields(out, task, "signals", bin, counts, CellKind::Count, &dims)?;
        fields(out, task, "signals", bin, scalars, CellKind::Scalar, &dims)?;
    }
    Ok(())
}
fn fields(
    out: &mut Vec<Item>,
    task: &str,
    family: &'static str,
    source: &Map<String, Value>,
    names: &[&'static str],
    kind: CellKind,
    dimensions: &Value,
) -> Result<()> {
    for name in names {
        add(
            out,
            task,
            family,
            name,
            dimensions.clone(),
            kind,
            required(source, name)?.clone(),
        );
    }
    Ok(())
}
fn add(
    out: &mut Vec<Item>,
    task: &str,
    family: &'static str,
    name: &'static str,
    dimensions: Value,
    kind: CellKind,
    value: Value,
) {
    out.push(Item {
        task: task.to_owned(),
        family,
        name,
        dimensions,
        kind,
        value,
    });
}

fn series_value(series: Series) -> Result<Value> {
    Ok(
        json!({"key":{"task_kind":series.task,"family":series.family,"name":series.name,"dimensions":series.dimensions},"kind":series.kind.name(),"values":series.values,"change_from_previous":deltas(&series.values,series.kind,false)?,"change_from_first":deltas(&series.values,series.kind,true)?}),
    )
}
fn deltas(values: &[Value], kind: CellKind, first: bool) -> Result<Vec<Value>> {
    values
        .iter()
        .enumerate()
        .map(|(index, current)| {
            if index == 0 {
                Ok(delta(None, Some("no_reference")))
            } else {
                difference(
                    if first {
                        &values[0]
                    } else {
                        &values[index - 1]
                    },
                    current,
                    kind,
                )
            }
        })
        .collect()
}
fn difference(reference: &Value, current: &Value, kind: CellKind) -> Result<Value> {
    if reference.is_null() {
        return Ok(delta(None, Some("reference_missing")));
    }
    if current.is_null() {
        return Ok(delta(None, Some("current_missing")));
    }
    match kind {
        CellKind::Count => {
            let left = reference.as_u64().ok_or_else(schema)?;
            let right = current.as_u64().ok_or_else(schema)?;
            Ok(delta(
                Some(Value::Number(Number::from(
                    (right as i128) - (left as i128),
                ))),
                None,
            ))
        }
        CellKind::Metric => metric_delta(reference, current),
        CellKind::Scalar => number_delta(Some(reference), Some(current)),
    }
}
fn metric_delta(reference: &Value, current: &Value) -> Result<Value> {
    let reference = object(reference)?;
    let current = object(current)?;
    if string(reference, "status")? != "defined" {
        return Ok(delta(None, Some("reference_not_defined")));
    }
    if string(current, "status")? != "defined" {
        return Ok(delta(None, Some("current_not_defined")));
    }
    number_delta(reference.get("value"), current.get("value"))
}
fn number_delta(reference: Option<&Value>, current: Option<&Value>) -> Result<Value> {
    let Some(left) = reference.and_then(Value::as_f64) else {
        return Ok(delta(None, Some("value_unavailable")));
    };
    let Some(right) = current.and_then(Value::as_f64) else {
        return Ok(delta(None, Some("value_unavailable")));
    };
    let difference = right - left;
    if !difference.is_finite() {
        return Ok(delta(None, Some("non_finite_delta")));
    }
    Ok(delta(
        Some(json!(if difference == 0.0 { 0.0 } else { difference })),
        None,
    ))
}
fn delta(value: Option<Value>, reason: Option<&str>) -> Value {
    json!({"value":value.unwrap_or(Value::Null),"reason":reason})
}

struct Episode {
    correct: bool,
    outcome: String,
}
fn relations(reports: &[ReportInput]) -> Result<Value> {
    let maps = reports.iter().map(episodes).collect::<Result<Vec<_>>>()?;
    let mut out = Vec::new();
    for index in 1..maps.len() {
        out.push(relation(index - 1, index, &maps[index - 1], &maps[index])?);
        if index > 1 {
            out.push(relation(0, index, &maps[0], &maps[index])?);
        }
    }
    Ok(Value::Array(out))
}
fn episodes(report: &ReportInput) -> Result<BTreeMap<String, Episode>> {
    let root = object(&report.value)?;
    let task = task_kind(root)?;
    let mut out = BTreeMap::new();
    for item in required(root, "episodes")?.as_array().ok_or_else(schema)? {
        let item = object(item)?;
        let id = string(item, "id")?.to_owned();
        let correct = item
            .get("final_correct")
            .and_then(Value::as_bool)
            .ok_or_else(schema)?;
        out.insert(
            id,
            Episode {
                correct,
                outcome: outcome(required(item, "final_outcome")?, task)?,
            },
        );
    }
    Ok(out)
}
fn outcome(value: &Value, task: &str) -> Result<String> {
    let value = object(value)?;
    match value.get("type").and_then(Value::as_str) {
        Some("abstention") => Ok("abstention".to_owned()),
        Some("class") if task == "single_label" => Ok(format!("label:{}", string(value, "label")?)),
        Some("labels") if task == "multi_label" => {
            let mut labels = required(value, "labels")?
                .as_array()
                .ok_or_else(schema)?
                .iter()
                .map(|item| item.as_str().map(str::to_owned).ok_or_else(schema))
                .collect::<Result<Vec<_>>>()?;
            labels.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
            Ok(format!(
                "labels:{}",
                serde_json::to_string(&labels)
                    .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?
            ))
        }
        _ => Err(schema()),
    }
}
fn relation(
    from_index: usize,
    to_index: usize,
    from: &BTreeMap<String, Episode>,
    to: &BTreeMap<String, Episode>,
) -> Result<Value> {
    let from_ids = from.keys().cloned().collect::<BTreeSet<_>>();
    let to_ids = to.keys().cloned().collect::<BTreeSet<_>>();
    let shared = from_ids.intersection(&to_ids).cloned().collect::<Vec<_>>();
    let mut both = Vec::new();
    let mut recovered = Vec::new();
    let mut regressed = Vec::new();
    let mut neither = Vec::new();
    let mut changed = Vec::new();
    for id in &shared {
        let left = &from[id];
        let right = &to[id];
        match (left.correct, right.correct) {
            (true, true) => both.push(id.clone()),
            (false, true) => recovered.push(id.clone()),
            (true, false) => regressed.push(id.clone()),
            (false, false) => neither.push(id.clone()),
        }
        if left.outcome != right.outcome {
            changed.push(id.clone());
        }
    }
    Ok(
        json!({"from_index":from_index,"to_index":to_index,"shared_ids":shared,"only_from_ids":from_ids.difference(&to_ids).cloned().collect::<Vec<_>>(),"only_to_ids":to_ids.difference(&from_ids).cloned().collect::<Vec<_>>(),"both_correct":category(both),"recovered":category(recovered),"regressed":category(regressed),"neither_correct":category(neither),"changed_final_outcomes":category(changed)}),
    )
}
fn category(ids: Vec<String>) -> Value {
    json!({"count":ids.len(),"ids":ids})
}

fn key(task: &str, family: &str, name: &str, dimensions: &Value) -> Result<String> {
    Ok(format!(
        "{task}\u{1f}{}\u{1f}{name}\u{1f}{}",
        family_rank(family)?,
        serde_json::to_string(&canonical_value(dimensions))
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?
    ))
}

fn family_rank(family: &str) -> Result<u8> {
    match family {
        "raw" => Ok(0),
        "final" => Ok(1),
        "probability" => Ok(2),
        "signals" => Ok(3),
        _ => Err(schema()),
    }
}

fn canonical_value(value: &Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.iter().map(canonical_value).collect()),
        Value::Object(values) => {
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_by(|(left, _), (right, _)| left.as_bytes().cmp(right.as_bytes()));
            let mut sorted = Map::new();
            for (key, value) in entries {
                sorted.insert(key.clone(), canonical_value(value));
            }
            Value::Object(sorted)
        }
        _ => value.clone(),
    }
}

fn object(value: &Value) -> Result<&Map<String, Value>> {
    value.as_object().ok_or_else(schema)
}
fn required<'a>(value: &'a Map<String, Value>, key: &str) -> Result<&'a Value> {
    value.get(key).ok_or_else(schema)
}
fn string<'a>(value: &'a Map<String, Value>, key: &str) -> Result<&'a str> {
    value.get(key).and_then(Value::as_str).ok_or_else(schema)
}
fn task_kind(value: &Map<String, Value>) -> Result<&str> {
    value
        .get("task")
        .and_then(Value::as_object)
        .and_then(|task| task.get("kind"))
        .and_then(Value::as_str)
        .ok_or_else(schema)
}
fn copy(value: &Map<String, Value>, key: &str) -> Value {
    value.get(key).cloned().unwrap_or(Value::Null)
}
fn schema() -> Diagnostic {
    Diagnostic::for_code(DiagnosticCode::Schema)
}
