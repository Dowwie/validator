#!/usr/bin/env python3.14
"""Prepare the frozen Chord630 native routes as canonical single-label inputs."""

from __future__ import annotations

import argparse
import base64
import bisect
import hashlib
import json
import math
import os
import shutil
import stat
import sys
from pathlib import Path
from typing import Any


MANIFEST_SHA256 = "860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e"
MAP_SHA256 = "a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4"
LABEL_MAP = {
    "DIFFERENT_CANONICAL": "NO_MATCH",
    "UNCERTAIN": "UNCERTAIN",
    "SAME_CANONICAL": "MATCH",
}
SCORE_LABELS = ("NO_MATCH", "UNCERTAIN", "MATCH")
SCORE_BOUNDARIES = (0.5, 1.5)
BASE_OUTPUTS = (
    "golden.json",
    "choice.json",
    "score.json",
    "config-full.json",
    "config-random400.json",
    "config-supplements230.json",
    "config-targeted.json",
    "preparation-script.py",
    "preparation-receipt.json",
)
SELECTION_NAMES = {
    "config-random400.json": "random400",
    "config-supplements230.json": "supplements230",
    "config-targeted.json": "targeted",
}


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
    ).encode() + b"\n"


def read_json(path: Path) -> Any:
    try:
        return json.loads(path.read_bytes())
    except (OSError, json.JSONDecodeError) as error:
        raise ValueError(f"cannot read JSON {path}") from error


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_new(path: Path, value: Any) -> None:
    if path.exists():
        raise ValueError(f"refusing to replace existing artifact {path}")
    path.write_bytes(canonical_bytes(value))
    path.chmod(0o600)


def copy_script(path: Path) -> None:
    if path.exists():
        raise ValueError(f"refusing to replace existing artifact {path}")
    shutil.copyfile(Path(__file__), path)
    path.chmod(0o600)


def require_private_directory(path: Path, create: bool) -> None:
    if create:
        path.mkdir(mode=0o700)
    if not path.is_dir():
        raise ValueError(f"output is not a directory: {path}")
    if stat.S_IMODE(path.stat().st_mode) != 0o700:
        raise ValueError(f"output directory must be mode 0700: {path}")


def route_score(score: float) -> str:
    if not math.isfinite(score):
        raise ValueError("native score is not finite")
    return SCORE_LABELS[bisect.bisect_right(SCORE_BOUNDARIES, score)]


def canonical_label(value: str) -> str:
    try:
        return LABEL_MAP[value]
    except KeyError as error:
        raise ValueError(f"unknown source label {value!r}") from error


def sibling_evidence_path(bundle: Path, relative: str) -> str:
    return f"../{bundle.name}/{relative}"


def require_single(values: list[Any], description: str) -> Any:
    canonical = {canonical_bytes(value) for value in values}
    if len(canonical) != 1:
        raise ValueError(f"{description} differs between source records")
    return values[0]


def raw_responses(bundle: Path, manifest: dict[str, Any], route: str) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for entry in manifest["source_entries"]:
        if entry["route"] != route or entry["role"] != "native_route_raw_response":
            continue
        record = read_json(bundle / entry["frozen_path"])
        encoded = record.get("raw_response_base64")
        if not isinstance(encoded, str):
            raise ValueError(f"{route} raw response is missing base64 payload")
        try:
            payload = json.loads(base64.b64decode(encoded, validate=True))
        except (ValueError, json.JSONDecodeError) as error:
            raise ValueError(f"{route} raw response has invalid base64 JSON") from error
        source_id = record.get("binding", {}).get("episode_id")
        if not isinstance(source_id, str) or source_id in rows:
            raise ValueError(f"{route} response has invalid or duplicate source ID")
        rows[source_id] = payload
    if len(rows) != 630:
        raise ValueError(f"{route} raw response count is {len(rows)}, expected 630")
    return rows


def raw_evidence_paths(bundle: Path, manifest: dict[str, Any], route: str) -> list[str]:
    entries = [
        entry
        for entry in manifest["source_entries"]
        if entry["route"] == route and entry["role"] == "native_route_raw_response"
    ]
    if len(entries) != 630:
        raise ValueError(f"{route} raw evidence count is {len(entries)}, expected 630")
    paths: list[str] = []
    seen: set[str] = set()
    for entry in entries:
        frozen_path = entry.get("frozen_path")
        recorded_hash = entry.get("sha256")
        if not isinstance(frozen_path, str) or not isinstance(recorded_hash, str):
            raise ValueError(f"{route} raw evidence entry has no path or hash")
        path = bundle / frozen_path
        if frozen_path in seen or not path.is_file() or sha256(path) != recorded_hash:
            raise ValueError(f"{route} raw evidence does not match the accepted manifest")
        seen.add(frozen_path)
        paths.append(sibling_evidence_path(bundle, frozen_path))
    return paths


def route_requests(bundle: Path, route: str) -> dict[str, dict[str, Any]]:
    rows = read_json(bundle / f"sources/{route}/requests.json")
    if not isinstance(rows, list):
        raise ValueError(f"{route} requests are not a list")
    requests: dict[str, dict[str, Any]] = {}
    for row in rows:
        source_id = row.get("episode_id")
        request = row.get("request")
        if not isinstance(source_id, str) or not isinstance(request, dict) or source_id in requests:
            raise ValueError(f"{route} request has invalid or duplicate source ID")
        requests[source_id] = request
    if len(requests) != 630:
        raise ValueError(f"{route} request count is {len(requests)}, expected 630")
    return requests


def route_references(bundle: Path, route: str) -> dict[str, dict[str, Any]]:
    rows = read_json(bundle / f"sources/{route}/references.json")
    if not isinstance(rows, list):
        raise ValueError(f"{route} references are not a list")
    references: dict[str, dict[str, Any]] = {}
    for row in rows:
        source_id = row.get("episode_id")
        if not isinstance(source_id, str) or source_id in references:
            raise ValueError(f"{route} reference has invalid or duplicate source ID")
        references[source_id] = row
    if len(references) != 630:
        raise ValueError(f"{route} reference count is {len(references)}, expected 630")
    return references


def answer(payload: dict[str, Any], name: str) -> dict[str, Any]:
    value = payload.get("answers", {}).get(name)
    if not isinstance(value, dict):
        raise ValueError(f"native response is missing {name} answer")
    return value


def finite_number(value: Any, description: str) -> float:
    if not isinstance(value, (int, float)) or isinstance(value, bool) or not math.isfinite(value):
        raise ValueError(f"{description} is not a finite number")
    return float(value)


def unit_number(value: Any, description: str) -> float:
    number = finite_number(value, description)
    if not 0 <= number <= 1:
        raise ValueError(f"{description} is outside [0, 1]")
    return number


def categorical(values: Any, description: str) -> dict[str, float]:
    if not isinstance(values, dict) or not values:
        raise ValueError(f"{description} is not a nonempty object")
    return {str(key): unit_number(value, description) for key, value in sorted(values.items())}


def choice_prediction(episode_id: str, payload: dict[str, Any]) -> dict[str, Any]:
    identity = answer(payload, "identity")
    recorded = identity.get("choice")
    if not isinstance(recorded, str):
        raise ValueError("Choice native answer is missing its recorded class")
    return {
        "id": episode_id,
        "observations": {
            "identity_distribution": {
                "kind": "categorical",
                "values": categorical(identity.get("probabilities"), "Choice distribution"),
            },
            "reported_confidence": {
                "kind": "reported_confidence",
                "value": unit_number(identity.get("confidence"), "Choice confidence"),
            },
        },
        "outcome": {"label": canonical_label(recorded), "type": "class"},
        "source_id": "native_choice",
    }


def score_prediction(episode_id: str, payload: dict[str, Any]) -> dict[str, Any]:
    identity = answer(payload, "identity")
    score = finite_number(identity.get("score"), "Score scalar")
    observations: dict[str, Any] = {
        "identity_distribution": {
            "kind": "categorical",
            "values": categorical(identity.get("probabilities"), "Score distribution"),
        },
        "identity_score": {"kind": "scalar", "value": score},
        "reported_confidence": {
            "kind": "reported_confidence",
            "value": unit_number(identity.get("confidence"), "Score confidence"),
        },
    }
    for name in ("same_subject", "same_capability", "material_difference"):
        observations[name] = {
            "kind": "bernoulli",
            "value": unit_number(answer(payload, name).get("noul"), f"Score {name}"),
        }
    return {
        "id": episode_id,
        "observations": observations,
        "outcome": {"label": route_score(score), "type": "class"},
        "source_id": "native_score",
    }


def source_definition(
    bundle: Path,
    route: str,
    model: str,
    questions: dict[str, Any],
    score_route: bool,
    raw_evidence: list[str],
) -> dict[str, Any]:
    evidence = [
        "preparation-script.py",
        sibling_evidence_path(bundle, "source-manifest.json"),
        sibling_evidence_path(bundle, "source-id-map.json"),
        sibling_evidence_path(bundle, f"sources/{route}/manifest.json"),
        sibling_evidence_path(bundle, f"sources/{route}/requests.json"),
        sibling_evidence_path(bundle, f"sources/{route}/references.json"),
        "preparation-receipt.json",
    ] + raw_evidence
    configuration: dict[str, Any] = {
        "frozen_route": route,
        "request_questions": questions,
        "saved_manifest": sibling_evidence_path(bundle, f"sources/{route}/manifest.json"),
        "saved_prompt_configuration": [
            sibling_evidence_path(bundle, f"sources/{route}/criteria.json"),
            sibling_evidence_path(bundle, f"sources/{route}/instructions.md"),
            sibling_evidence_path(bundle, f"sources/{route}/original-prompt.md"),
            sibling_evidence_path(bundle, f"sources/{route}/adaptation.diff"),
        ],
    }
    if not score_route:
        configuration["canonical_label_mapping"] = LABEL_MAP
        return {
            "configuration": configuration,
            "evidence": evidence,
            "kind": "classifier",
            "model": model,
            "observation_definitions": {
                "identity_distribution": {
                    "description": "Native selected-choice identity distribution retained without scoring admission.",
                    "kind": "categorical",
                    "question_id": "identity",
                },
                "reported_confidence": {
                    "description": "Native selected-choice reported confidence retained without scoring admission.",
                    "kind": "reported_confidence",
                    "question_id": "identity",
                },
            },
        }
    configuration["scalar_to_canonical_outcome"] = {
        "boundaries": list(SCORE_BOUNDARIES),
        "labels": list(SCORE_LABELS),
        "tie_behavior": "bisect_right: 0.5 maps to UNCERTAIN and 1.5 maps to MATCH",
    }
    return {
        "configuration": configuration,
        "evidence": evidence,
        "kind": "classifier",
        "model": model,
        "observation_definitions": {
            "identity_distribution": {
                "description": "Native score-route identity distribution retained without scoring admission.",
                "kind": "categorical",
                "question_id": "identity",
            },
            "identity_score": {
                "description": "Native score used for the documented scalar-to-class route.",
                "kind": "scalar",
                "question_id": "identity",
            },
            "material_difference": {
                "description": "Native reported probability of material difference retained as an auxiliary observation.",
                "kind": "bernoulli",
                "question_id": "material_difference",
            },
            "reported_confidence": {
                "description": "Native score-route reported confidence retained without scoring admission.",
                "kind": "reported_confidence",
                "question_id": "identity",
            },
            "same_capability": {
                "description": "Native reported probability of same capability retained as an auxiliary observation.",
                "kind": "bernoulli",
                "question_id": "same_capability",
            },
            "same_subject": {
                "description": "Native reported probability of same subject retained as an auxiliary observation.",
                "kind": "bernoulli",
                "question_id": "same_subject",
            },
        },
        "preparation": {
            "configuration": configuration["scalar_to_canonical_outcome"],
            "evidence_indices": [0, 1, 2, 6, *range(7, len(evidence))],
            "method": "frozen_score_bisect_right",
            "version": "chord630-t029-v1",
        },
    }


def prepare(bundle: Path, out: Path) -> None:
    manifest_path = bundle / "source-manifest.json"
    map_path = bundle / "source-id-map.json"
    if sha256(manifest_path) != MANIFEST_SHA256 or sha256(map_path) != MAP_SHA256:
        raise ValueError("bundle does not match the accepted T028 metadata hashes")
    if out.resolve().parent != bundle.resolve().parent:
        raise ValueError("output must be a sibling of the frozen bundle")
    require_private_directory(out, create=not out.exists())
    if any((out / name).exists() for name in BASE_OUTPUTS):
        raise ValueError("base preparation artifacts already exist")

    manifest = read_json(manifest_path)
    id_map = read_json(map_path)
    cases = id_map.get("cases")
    if not isinstance(cases, list) or len(cases) != 630:
        raise ValueError("persisted ID map does not contain 630 cases")
    cases_by_source = {case.get("source_id"): case for case in cases}
    if len(cases_by_source) != 630 or None in cases_by_source:
        raise ValueError("persisted ID map has duplicate or missing source IDs")

    choice_requests = route_requests(bundle, "choice")
    score_requests = route_requests(bundle, "score")
    choice_references = route_references(bundle, "choice")
    score_references = route_references(bundle, "score")
    choice_raw = raw_responses(bundle, manifest, "choice")
    score_raw = raw_responses(bundle, manifest, "score")
    choice_raw_evidence = raw_evidence_paths(bundle, manifest, "choice")
    score_raw_evidence = raw_evidence_paths(bundle, manifest, "score")
    source_ids = sorted(cases_by_source)
    required_ids = set(source_ids)
    for name, collection in (
        ("Choice requests", choice_requests),
        ("Score requests", score_requests),
        ("Choice references", choice_references),
        ("Score references", score_references),
        ("Choice raw responses", choice_raw),
        ("Score raw responses", score_raw),
    ):
        if set(collection) != required_ids:
            raise ValueError(f"{name} do not match the persisted source ID map")

    labeled = [case for case in cases if case["disposition"] == "labeled"]
    excluded = [case for case in cases if case["disposition"] == "withheld_or_unscored"]
    if len(labeled) != 603 or len(excluded) != 27:
        raise ValueError("persisted map does not partition into 603 labeled and 27 exclusions")
    if any(case["reference_disposition"] != "REVIEWED" for case in labeled):
        raise ValueError("labeled map cases are not all reviewed")
    if any(case["reference_disposition"] == "REVIEWED" for case in excluded):
        raise ValueError("excluded map cases include reviewed references")

    golden_rows = []
    choice_rows = []
    score_rows = []
    for case in sorted(labeled, key=lambda item: item["source_id"]):
        source_id = case["source_id"]
        episode_id = case["episode_uuid"]
        choice_request = choice_requests[source_id]
        score_request = score_requests[source_id]
        state = choice_request.get("state")
        if not isinstance(state, dict) or canonical_bytes(state) != canonical_bytes(score_request.get("state")):
            raise ValueError(f"source input disagreement for {source_id}")
        reference = choice_references[source_id]
        if canonical_bytes(reference) != canonical_bytes(score_references[source_id]):
            raise ValueError(f"reference disagreement for {source_id}")
        expected = reference.get("current_label")
        if not isinstance(expected, str):
            raise ValueError(f"reviewed source lacks a current label: {source_id}")
        golden_rows.append(
            {"expected": {"label": canonical_label(expected), "type": "class"}, "id": episode_id, "input": state}
        )
        choice_rows.append(choice_prediction(episode_id, choice_raw[source_id]))
        score_rows.append(score_prediction(episode_id, score_raw[source_id]))

    choice_models = [request.get("model") for request in choice_requests.values()]
    score_models = [request.get("model") for request in score_requests.values()]
    choice_questions = [request.get("questions") for request in choice_requests.values()]
    score_questions = [request.get("questions") for request in score_requests.values()]
    choice_model = require_single(choice_models, "Choice model")
    score_model = require_single(score_models, "Score model")
    choice_question_definition = require_single(choice_questions, "Choice question configuration")
    score_question_definition = require_single(score_questions, "Score question configuration")
    if not isinstance(choice_model, str) or not isinstance(score_model, str):
        raise ValueError("route model identifier is not a string")
    if not isinstance(choice_question_definition, dict) or not isinstance(score_question_definition, dict):
        raise ValueError("route question configuration is not an object")

    golden = {"episodes": golden_rows, "schema_version": 2, "task": {"kind": "single_label", "labels": list(SCORE_LABELS)}}
    golden_path = out / "golden.json"
    write_new(golden_path, golden)
    golden_digest = sha256(golden_path)
    choice = {
        "dataset_sha256": golden_digest,
        "predictions": choice_rows,
        "schema_version": 2,
        "sources": {
            "native_choice": source_definition(
                bundle,
                "choice",
                choice_model,
                choice_question_definition,
                False,
                choice_raw_evidence,
            )
        },
    }
    score = {
        "dataset_sha256": golden_digest,
        "predictions": score_rows,
        "schema_version": 2,
        "sources": {
            "native_score": source_definition(
                bundle,
                "score",
                score_model,
                score_question_definition,
                True,
                score_raw_evidence,
            )
        },
    }
    write_new(out / "choice.json", choice)
    write_new(out / "score.json", score)

    ordered_labeled = sorted(labeled, key=lambda item: item["source_id"])
    by_cohort = {
        "random400": [case["episode_uuid"] for case in ordered_labeled if case["cohort"] == "old_random400"],
        "supplements230": [case["episode_uuid"] for case in ordered_labeled if case["cohort"] == "old_supplements230"],
        "targeted": [case["episode_uuid"] for case in ordered_labeled[:50]],
    }
    if len(by_cohort["random400"]) != 383 or len(by_cohort["supplements230"]) != 220:
        raise ValueError("frozen labeled cohort partition changed")
    configurations = {
        "config-full.json": {"decision": {"type": "as_recorded"}, "population": "chord630_full_labeled", "role": "development", "schema_version": 2},
        "config-random400.json": {"decision": {"type": "as_recorded"}, "episode_ids": by_cohort["random400"], "population": "chord630_old_random400_labeled", "role": "development", "schema_version": 2},
        "config-supplements230.json": {"decision": {"type": "as_recorded"}, "episode_ids": by_cohort["supplements230"], "population": "chord630_old_supplements230_labeled", "role": "development", "schema_version": 2},
        "config-targeted.json": {"decision": {"type": "as_recorded"}, "episode_ids": by_cohort["targeted"], "population": "chord630_first50_labeled_source_ids", "role": "development", "schema_version": 2},
    }
    for name, configuration in configurations.items():
        write_new(out / name, configuration)
    copy_script(out / "preparation-script.py")

    receipt = {
        "canonical_outputs": {name: sha256(out / name) for name in BASE_OUTPUTS if name != "preparation-receipt.json"},
        "exclusions": [
            {
                "cohort": case["cohort"],
                "component": case["component"],
                "episode_id": case["episode_uuid"],
                "reference_disposition": case["reference_disposition"],
                "reference_reason": case["reference_reason"],
                "source_id": case["source_id"],
            }
            for case in sorted(excluded, key=lambda item: item["source_id"])
        ],
        "input_counts": {"labeled": 603, "source_total": 630, "withheld_or_unscored": 27},
        "native_routes": {
            "choice": {"raw_response_count": 630, "source_id": "native_choice"},
            "score": {
                "raw_response_count": 630,
                "scalar_mapping": {"boundaries": list(SCORE_BOUNDARIES), "labels": list(SCORE_LABELS), "tie_behavior": "bisect_right"},
                "source_id": "native_score",
            },
        },
        "omitted_scoring_families": [
            {
                "affected_episode_ids": [case["episode_uuid"] for case in ordered_labeled],
                "family": "probabilities",
                "reason": "Native distributions are retained as categorical observations; no scoring-probability transformation is declared.",
                "route": route,
            }
            for route in ("choice", "score")
        ] + [
            {
                "affected_episode_ids": [case["episode_uuid"] for case in ordered_labeled],
                "family": "confidence",
                "reason": "Native confidence is retained as reported_confidence observation, not submitted as a scoring signal.",
                "route": route,
            }
            for route in ("choice", "score")
        ],
        "schema_version": 1,
        "source_metadata": {"manifest_sha256": MANIFEST_SHA256, "source_id_map_sha256": MAP_SHA256},
        "transformation": {"method": "prepare_chord.py", "version": "chord630-t029-v1"},
    }
    write_new(out / "preparation-receipt.json", receipt)


def prepare_selection(out: Path, selection_path: Path) -> None:
    name = SELECTION_NAMES.get(selection_path.name)
    if name is None:
        raise ValueError("selection must be one of the generated cohort configurations")
    if selection_path.parent.resolve() != out.resolve():
        raise ValueError("selection configuration must belong to the output directory")
    selection = read_json(selection_path)
    episode_ids = selection.get("episode_ids")
    if not isinstance(episode_ids, list) or len(episode_ids) != len(set(episode_ids)):
        raise ValueError("selection must contain unique explicit episode IDs")
    golden = read_json(out / "golden.json")
    whole_ids = {row["id"] for row in golden.get("episodes", [])}
    if not set(episode_ids) <= whole_ids:
        raise ValueError("selection includes an ID outside whole golden")
    for route in ("choice", "score"):
        source = read_json(out / f"{route}.json")
        rows = source.get("predictions")
        if not isinstance(rows, list):
            raise ValueError(f"{route} artifact lacks predictions")
        by_id = {row.get("id"): row for row in rows}
        if len(by_id) != len(rows) or set(episode_ids) - set(by_id):
            raise ValueError(f"{route} cannot align the configured selection")
        subset = {
            "dataset_sha256": source.get("dataset_sha256"),
            "predictions": [by_id[episode_id] for episode_id in episode_ids],
            "schema_version": 2,
            "sources": source.get("sources"),
        }
        write_new(out / f"{route}-{name}.json", subset)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--selection", type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        bundle = args.bundle.resolve()
        out = args.out.resolve()
        if args.selection is None:
            prepare(bundle, out)
        else:
            prepare_selection(out, args.selection.resolve())
    except ValueError as error:
        print(f"prepare_chord: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
