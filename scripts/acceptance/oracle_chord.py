#!/usr/bin/env python3
"""Build the fixed Chord630 hard-label acceptance oracle from frozen evidence."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
from bisect import bisect_right
from dataclasses import dataclass
from decimal import Decimal
from fractions import Fraction
from pathlib import Path
from typing import Any


EXPECTED_MANIFEST_SHA256 = "860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e"
EXPECTED_ID_MAP_SHA256 = "a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4"
LABELS = ("DIFFERENT_CANONICAL", "UNCERTAIN", "SAME_CANONICAL")
ABSTENTION = "ABSTAINED"
SCORE_LABELS = ("NO_MATCH", "UNCERTAIN", "MATCH")
SCORE_BOUNDARIES = (Decimal("0.5"), Decimal("1.5"))


@dataclass(frozen=True)
class Episode:
    source_id: str
    episode_uuid: str
    actual: str | None
    choice: str
    score: str
    cohort: str
    component: str


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"cannot decode JSON: {path}") from error


def require_mapping(value: Any, context: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ValueError(f"{context} must be an object")
    return value


def require_string(value: Any, context: str) -> str:
    if not isinstance(value, str) or not value:
        raise ValueError(f"{context} must be a nonempty string")
    return value


def safe_source_path(bundle: Path, relative_path: str) -> Path:
    candidate = (bundle / relative_path).resolve()
    try:
        candidate.relative_to(bundle.resolve())
    except ValueError as error:
        raise ValueError(f"manifest path escapes bundle: {relative_path}") from error
    return candidate


def load_manifest(bundle: Path) -> dict[str, Any]:
    path = bundle / "source-manifest.json"
    if sha256(path) != EXPECTED_MANIFEST_SHA256:
        raise ValueError("unexpected source-manifest.json hash")
    manifest = require_mapping(read_json(path), "source manifest")
    mapping = require_mapping(manifest.get("scalar_to_label_mapping"), "scalar mapping")
    if tuple(mapping.get("labels", ())) != SCORE_LABELS:
        raise ValueError("unexpected frozen score labels")
    if tuple(Decimal(str(item)) for item in mapping.get("boundaries", ())) != SCORE_BOUNDARIES:
        raise ValueError("unexpected frozen score boundaries")
    if mapping.get("actual_tie_behavior") != (
        "bisect_right assigns score 0.5 to UNCERTAIN and score 1.5 to MATCH; no argmax substitution"
    ):
        raise ValueError("unexpected frozen score tie behavior")
    return manifest


def entries_by_path(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    entries = manifest.get("source_entries")
    if not isinstance(entries, list):
        raise ValueError("manifest source_entries must be a list")
    result: dict[str, dict[str, Any]] = {}
    for item in entries:
        entry = require_mapping(item, "manifest source entry")
        relative_path = require_string(entry.get("frozen_path"), "manifest frozen_path")
        if relative_path in result:
            raise ValueError(f"duplicate manifest frozen_path: {relative_path}")
        result[relative_path] = entry
    return result


def checked_json(bundle: Path, entry: dict[str, Any]) -> Any:
    relative_path = require_string(entry.get("frozen_path"), "manifest frozen_path")
    path = safe_source_path(bundle, relative_path)
    expected_hash = require_string(entry.get("sha256"), f"manifest hash for {relative_path}")
    if sha256(path) != expected_hash:
        raise ValueError(f"manifest hash mismatch: {relative_path}")
    return read_json(path)


def raw_predictions(
    bundle: Path, entries: dict[str, dict[str, Any]], route: str
) -> dict[str, str]:
    records = [
        entry
        for entry in entries.values()
        if entry.get("route") == route and entry.get("role") == "native_route_raw_response"
    ]
    if len(records) != 630:
        raise ValueError(f"{route} must have exactly 630 native raw records")
    predictions: dict[str, str] = {}
    for entry in records:
        record = require_mapping(checked_json(bundle, entry), "native route record")
        binding = require_mapping(record.get("binding"), "native record binding")
        source_id = require_string(binding.get("episode_id"), "native record episode_id")
        encoded = require_string(record.get("raw_response_base64"), "native raw response")
        try:
            raw = json.loads(base64.b64decode(encoded, validate=True), parse_float=Decimal)
        except (ValueError, json.JSONDecodeError) as error:
            raise ValueError(f"cannot decode native raw response for {source_id}") from error
        answers = require_mapping(require_mapping(raw, "native response").get("answers"), "native answers")
        identity = require_mapping(answers.get("identity"), "native identity answer")
        if route == "choice":
            prediction = require_string(identity.get("choice"), "native choice")
            if prediction not in LABELS:
                raise ValueError(f"unexpected native choice for {source_id}")
        else:
            scalar = identity.get("score")
            if not isinstance(scalar, Decimal):
                raise ValueError(f"native score must be numeric for {source_id}")
            prediction = SCORE_LABELS[bisect_right(SCORE_BOUNDARIES, scalar)]
            prediction = {
                "NO_MATCH": "DIFFERENT_CANONICAL",
                "UNCERTAIN": "UNCERTAIN",
                "MATCH": "SAME_CANONICAL",
            }[prediction]
        if source_id in predictions:
            raise ValueError(f"duplicate native source ID for {route}")
        predictions[source_id] = prediction
    return predictions


def load_references(bundle: Path, entries: dict[str, dict[str, Any]]) -> dict[str, dict[str, Any]]:
    entry = entries.get("sources/choice/references.json")
    if entry is None:
        raise ValueError("choice reference evidence missing from manifest")
    values = checked_json(bundle, entry)
    if not isinstance(values, list) or len(values) != 630:
        raise ValueError("reference evidence must have exactly 630 records")
    references: dict[str, dict[str, Any]] = {}
    for item in values:
        reference = require_mapping(item, "reference record")
        source_id = require_string(reference.get("episode_id"), "reference episode_id")
        if source_id in references:
            raise ValueError("duplicate reference episode_id")
        state = require_string(reference.get("reference_state"), "reference state")
        if state == "REVIEWED":
            label = require_string(reference.get("current_label"), "reviewed current label")
            if label not in LABELS:
                raise ValueError("unexpected reviewed label")
        elif reference.get("current_label") is not None:
            raise ValueError("unscored reference must not have current_label")
        references[source_id] = reference
    return references


def load_id_map(bundle: Path) -> dict[str, dict[str, Any]]:
    path = bundle / "source-id-map.json"
    if sha256(path) != EXPECTED_ID_MAP_SHA256:
        raise ValueError("unexpected source-id-map.json hash")
    values = require_mapping(read_json(path), "source ID map").get("cases")
    if not isinstance(values, list) or len(values) != 630:
        raise ValueError("source ID map must have exactly 630 cases")
    id_map: dict[str, dict[str, Any]] = {}
    for item in values:
        case = require_mapping(item, "source ID map case")
        source_id = require_string(case.get("source_id"), "map source_id")
        require_string(case.get("episode_uuid"), "map episode_uuid")
        if source_id in id_map:
            raise ValueError("duplicate source ID map case")
        id_map[source_id] = case
    return id_map


def load_exclusions(manifest: dict[str, Any], id_map: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    values = manifest.get("withheld_or_unscored_cases")
    if not isinstance(values, list) or len(values) != 27:
        raise ValueError("manifest exclusions must have exactly 27 cases")
    exclusions = []
    for item in values:
        exclusion = require_mapping(item, "manifest exclusion")
        source_id = require_string(exclusion.get("source_id"), "exclusion source_id")
        mapped = id_map.get(source_id)
        if mapped is None:
            raise ValueError("exclusion absent from source ID map")
        exclusions.append(
            {
                "source_id": source_id,
                "episode_uuid": mapped["episode_uuid"],
                "reference_disposition": require_string(
                    exclusion.get("reference_disposition"), "exclusion disposition"
                ),
                "reference_state": require_mapping(exclusion.get("reference_state"), "exclusion state"),
                "reason": require_mapping(exclusion.get("original_reason"), "exclusion reason"),
                "cohort": require_string(exclusion.get("cohort"), "exclusion cohort"),
                "component": require_string(exclusion.get("component"), "exclusion component"),
            }
        )
    return sorted(exclusions, key=lambda item: item["source_id"])


def metric(numerator: int | Fraction, denominator: int | Fraction, *, status: str = "defined") -> dict[str, Any]:
    numerator_fraction = Fraction(numerator)
    denominator_fraction = Fraction(denominator)
    if denominator_fraction == 0:
        return {
            "value": None,
            "status": "undefined_zero_denominator",
            "numerator": numerator_fraction.numerator,
            "denominator": 0,
        }
    value = numerator_fraction / denominator_fraction
    return {
        "value": float(value),
        "status": status,
        "numerator": value.numerator,
        "denominator": value.denominator,
    }


def no_data_metric() -> dict[str, Any]:
    return {"value": None, "status": "no_data", "numerator": 0, "denominator": 0}


def population_result(episodes: list[Episode], route: str) -> dict[str, Any]:
    if not episodes:
        return {
            "population_count": 0,
            "matrix": {actual: {declared: 0 for declared in (*LABELS, ABSTENTION)} for actual in LABELS},
            "supports": {label: 0 for label in LABELS},
            "predicted_supports": {label: 0 for label in LABELS},
            "decision_counts": {"correct": 0, "wrong": 0, "abstained": 0, "answered": 0},
            "metrics": {name: no_data_metric() for name in ("accuracy", "wrong_class_rate", "abstention_rate", "coverage", "selective_accuracy", "selective_risk", "macro_f1")},
            "per_class": {label: {name: no_data_metric() for name in ("precision", "recall", "f1", "coverage")} for label in LABELS},
        }
    matrix = {actual: {declared: 0 for declared in (*LABELS, ABSTENTION)} for actual in LABELS}
    for episode in episodes:
        matrix[episode.actual][getattr(episode, route)] += 1
    supports = {actual: sum(matrix[actual].values()) for actual in LABELS}
    predicted_supports = {declared: sum(matrix[actual][declared] for actual in LABELS) for declared in LABELS}
    correct = sum(matrix[label][label] for label in LABELS)
    abstained = sum(matrix[actual][ABSTENTION] for actual in LABELS)
    wrong = len(episodes) - correct - abstained
    answered = correct + wrong
    per_class: dict[str, dict[str, Any]] = {}
    f1_values: list[Fraction] = []
    undefined_f1: list[str] = []
    for label in LABELS:
        true_positive = matrix[label][label]
        false_negative = supports[label] - true_positive
        false_positive = predicted_supports[label] - true_positive
        f1_denominator = 2 * true_positive + false_positive + false_negative
        f1 = metric(2 * true_positive, f1_denominator)
        if f1["status"] == "undefined_zero_denominator":
            undefined_f1.append(label)
            f1_values.append(Fraction())
        else:
            f1_values.append(Fraction(f1["numerator"], f1["denominator"]))
        per_class[label] = {
            "support": supports[label],
            "predicted_support": predicted_supports[label],
            "true_positive": true_positive,
            "false_negative": false_negative,
            "false_positive": false_positive,
            "precision": metric(true_positive, true_positive + false_positive),
            "recall": metric(true_positive, true_positive + false_negative),
            "f1": f1,
            "coverage": metric(supports[label] - matrix[label][ABSTENTION], supports[label]),
        }
    macro_status = "contains_undefined_classes" if undefined_f1 else "defined"
    macro_f1 = metric(sum(f1_values, Fraction()), len(LABELS), status=macro_status)
    if undefined_f1:
        macro_f1["undefined_classes"] = undefined_f1
    metrics = {
        "accuracy": metric(correct, len(episodes)),
        "wrong_class_rate": metric(wrong, len(episodes)),
        "abstention_rate": metric(abstained, len(episodes)),
        "coverage": metric(answered, len(episodes)),
        "selective_accuracy": metric(correct, answered),
        "selective_risk": metric(wrong, answered),
        "macro_f1": macro_f1,
    }
    if correct + wrong + abstained != len(episodes) or sum(supports.values()) != len(episodes):
        raise ValueError("hard-decision accounting identity failed")
    if sum(predicted_supports.values()) != answered:
        raise ValueError("predicted-support accounting identity failed")
    return {
        "population_count": len(episodes),
        "matrix": matrix,
        "supports": supports,
        "predicted_supports": predicted_supports,
        "decision_counts": {"correct": correct, "wrong": wrong, "abstained": abstained, "answered": answered},
        "metrics": metrics,
        "per_class": per_class,
    }


def paired_result(episodes: list[Episode]) -> dict[str, Any]:
    matrix = {choice: {score: 0 for score in LABELS} for choice in LABELS}
    changed: list[str] = []
    recovered: list[str] = []
    regressed: list[str] = []
    for episode in episodes:
        matrix[episode.choice][episode.score] += 1
        if episode.choice != episode.score:
            changed.append(episode.episode_uuid)
        if episode.choice != episode.actual and episode.score == episode.actual:
            recovered.append(episode.episode_uuid)
        if episode.choice == episode.actual and episode.score != episode.actual:
            regressed.append(episode.episode_uuid)
    return {
        "choice_to_score_matrix": matrix,
        "changed_count": len(changed),
        "recovered_count": len(recovered),
        "regressed_count": len(regressed),
        "changed_episode_uuids": changed,
        "recovered_episode_uuids": recovered,
        "regressed_episode_uuids": regressed,
    }


def known_failures(episodes: list[Episode]) -> list[dict[str, str]]:
    failures = [episode for episode in episodes if episode.choice != episode.actual or episode.score != episode.actual]
    return [
        {
            "episode_uuid": episode.episode_uuid,
            "actual": episode.actual,
            "choice_prediction": episode.choice,
            "score_prediction": episode.score,
            "cohort": episode.cohort,
            "component": episode.component,
        }
        for episode in failures[:10]
    ]


def build_expected(bundle: Path) -> dict[str, Any]:
    manifest = load_manifest(bundle)
    entries = entries_by_path(manifest)
    choice_predictions = raw_predictions(bundle, entries, "choice")
    score_predictions = raw_predictions(bundle, entries, "score")
    references = load_references(bundle, entries)
    id_map = load_id_map(bundle)
    source_ids = set(references)
    if source_ids != set(choice_predictions) or source_ids != set(score_predictions) or source_ids != set(id_map):
        raise ValueError("native, reference, and ID-map source ID sets must match")
    exclusions = load_exclusions(manifest, id_map)
    excluded_ids = {item["source_id"] for item in exclusions}
    labeled: list[Episode] = []
    for source_id in sorted(source_ids):
        reference = references[source_id]
        state = reference["reference_state"]
        if state == "REVIEWED":
            labeled.append(
                Episode(
                    source_id=source_id,
                    episode_uuid=id_map[source_id]["episode_uuid"],
                    actual=reference["current_label"],
                    choice=choice_predictions[source_id],
                    score=score_predictions[source_id],
                    cohort=reference["cohort"],
                    component=reference["component"],
                )
            )
        elif source_id not in excluded_ids:
            raise ValueError("unreviewed source ID missing from exclusions")
    if len(labeled) != 603 or len(excluded_ids) != 27:
        raise ValueError("unexpected labeled/excluded population counts")
    populations = {
        "full_labeled": labeled,
        "representative": [episode for episode in labeled if episode.cohort == "old_random400"],
        "challenge": [episode for episode in labeled if episode.cohort == "old_supplements230"],
        "targeted_first_50_labeled_source_ids": labeled[:50],
    }
    return {
        "schema_version": 1,
        "case": "chord630",
        "method": {
            "native_routes": {
                "choice": "answers.identity.choice decoded from each frozen raw response",
                "score": "answers.identity.score decoded from each frozen raw response and mapped with frozen bisect_right boundaries",
            },
            "score_mapping": {
                "labels": list(SCORE_LABELS),
                "boundaries": ["0.5", "1.5"],
                "tie_behavior": "bisect_right: 0.5 -> UNCERTAIN; 1.5 -> MATCH",
            },
            "targeted_selection": "first 50 REVIEWED source IDs in ascending exact string order, mapped through source-id-map.json",
            "known_failure_selection": "first 10 labeled cases with a Choice or Score error after ascending exact source-ID ordering",
        },
        "inputs": {
            "source_manifest_sha256": EXPECTED_MANIFEST_SHA256,
            "source_id_map_sha256": EXPECTED_ID_MAP_SHA256,
            "source_population": 630,
            "labeled_population": 603,
            "excluded_population": 27,
        },
        "labels": {"declared": list(LABELS), "abstention": ABSTENTION},
        "exclusions": exclusions,
        "populations": {
            name: {
                "selection": (
                    "all REVIEWED references"
                    if name == "full_labeled"
                    else "REVIEWED references with cohort=old_random400"
                    if name == "representative"
                    else "REVIEWED references with cohort=old_supplements230"
                    if name == "challenge"
                    else "fixed targeted selection"
                ),
                "episode_uuids": [episode.episode_uuid for episode in episodes],
                "choice": population_result(episodes, "choice"),
                "score": population_result(episodes, "score"),
                "paired_transitions": paired_result(episodes),
            }
            for name, episodes in populations.items()
        },
        "known_failures": known_failures(labeled),
    }


def write_json(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    payload = (json.dumps(value, ensure_ascii=True, indent=2, sort_keys=True) + "\n").encode("utf-8")
    path.write_bytes(payload)
    os.chmod(path, 0o600)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    write_json(args.out, build_expected(args.bundle.resolve()))


if __name__ == "__main__":
    main()
