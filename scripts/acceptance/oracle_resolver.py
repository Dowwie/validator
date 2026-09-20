#!/usr/bin/env python3
"""Build independent resolver250 hard-label expectations from frozen source rows."""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from fractions import Fraction
from pathlib import Path
from typing import Any


FLASH_SHA256 = "c631d0a540f939b1b7d14e41464c4aa5d8877c4b6598b408ead90b714fc45bcf"
PRO_SHA256 = "8f891d22424433201284e5aa32f380a3ccbadbf29fc1a16a8548c073c6f7caa2"
RESIDUE = "RESIDUE"
ABSTAINED = "ABSTAINED"


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
    ).encode() + b"\n"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_jsonl(path: Path, expected_hash: str) -> list[dict[str, Any]]:
    if sha256(path) != expected_hash:
        raise ValueError(f"source hash mismatch: {path}")
    try:
        rows = [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines()]
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"cannot read JSONL {path}") from error
    if len(rows) != 250 or not all(isinstance(row, dict) for row in rows):
        raise ValueError(f"{path} must contain exactly 250 object rows")
    return rows


def identity(row: dict[str, Any]) -> dict[str, Any]:
    return {
        key: row.get(key)
        for key in ("phrase", "quote", "label", "baseline_top1", "label_in_shortlist")
    }


def load_ids(path: Path, rows: list[dict[str, Any]]) -> list[str]:
    try:
        cases = json.loads(path.read_text(encoding="utf-8"))["cases"]
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, KeyError, TypeError) as error:
        raise ValueError("cannot read source ID map") from error
    if len(cases) != len(rows):
        raise ValueError("source ID map count mismatch")
    ids = []
    for index, (case, row) in enumerate(zip(cases, rows, strict=True)):
        expected = hashlib.sha256(canonical_bytes(identity(row))).hexdigest()
        if case.get("source_row") != index or case.get("row_identity_sha256") != expected:
            raise ValueError("source ID map binding mismatch")
        episode = case.get("episode_uuid")
        if not isinstance(episode, str) or episode in ids:
            raise ValueError("source ID map has an invalid or duplicate UUID")
        ids.append(episode)
    return ids


def outcome(row: dict[str, Any], route: str) -> str:
    if route == "baseline":
        return row["baseline_top1"]
    if row["unparseable"]:
        return ABSTAINED
    return row["judge"] if row["judge"] is not None else RESIDUE


def metric(numerator: int, denominator: int) -> dict[str, Any]:
    if denominator == 0:
        return {"denominator": 0, "numerator": 0, "status": "undefined_zero_denominator", "value": None}
    fraction = Fraction(numerator, denominator)
    return {
        "denominator": fraction.denominator,
        "numerator": fraction.numerator,
        "status": "defined",
        "value": float(fraction),
    }


def evaluate(rows: list[dict[str, Any]], ids: list[str], route: str, labels: list[str]) -> dict[str, Any]:
    matrix = {actual: {predicted: 0 for predicted in (*labels, ABSTAINED)} for actual in labels}
    correct_ids = []
    answered_ids = []
    outcomes = {}
    for episode, row in zip(ids, rows, strict=True):
        actual = row["label"]
        predicted = outcome(row, route)
        matrix[actual][predicted] += 1
        outcomes[episode] = predicted
        if predicted != ABSTAINED:
            answered_ids.append(episode)
        if predicted == actual:
            correct_ids.append(episode)
    total = len(rows)
    correct = len(correct_ids)
    answered = len(answered_ids)
    abstained = total - answered
    wrong = answered - correct
    return {
        "answered_ids": answered_ids,
        "correct_ids": correct_ids,
        "decision_counts": {
            "abstained": abstained,
            "answered": answered,
            "correct": correct,
            "wrong": wrong,
        },
        "matrix": matrix,
        "metrics": {
            "abstention_rate": metric(abstained, total),
            "accuracy": metric(correct, total),
            "coverage": metric(answered, total),
            "selective_accuracy": metric(correct, answered),
            "selective_risk": metric(wrong, answered),
            "wrong_class_rate": metric(wrong, total),
        },
        "outcomes": outcomes,
        "selected_ids": ids,
    }


def compare(baseline: dict[str, Any], candidate: dict[str, Any], ids: list[str]) -> dict[str, Any]:
    baseline_correct = set(baseline["correct_ids"])
    candidate_correct = set(candidate["correct_ids"])
    changed = [episode for episode in ids if baseline["outcomes"][episode] != candidate["outcomes"][episode]]
    return {
        "answered_overlap_ids": [
            episode
            for episode in ids
            if episode in baseline["answered_ids"] and episode in candidate["answered_ids"]
        ],
        "both_correct_ids": [episode for episode in ids if episode in baseline_correct & candidate_correct],
        "changed_final_outcome_ids": changed,
        "neither_correct_ids": [episode for episode in ids if episode not in baseline_correct | candidate_correct],
        "recovered_ids": [episode for episode in ids if episode in candidate_correct - baseline_correct],
        "regressed_ids": [episode for episode in ids if episode in baseline_correct - candidate_correct],
    }


def build(flash_path: Path, pro_path: Path, id_map_path: Path) -> dict[str, Any]:
    flash = read_jsonl(flash_path, FLASH_SHA256)
    pro = read_jsonl(pro_path, PRO_SHA256)
    if [identity(row) for row in flash] != [identity(row) for row in pro]:
        raise ValueError("resolver arms have different row identities")
    ids = load_ids(id_map_path, flash)
    labels = sorted(
        {RESIDUE}
        | {row["label"] for row in flash}
        | {row["baseline_top1"] for row in flash}
        | {row["judge"] for row in flash + pro if row["judge"] is not None}
    )
    results = {
        "baseline": evaluate(flash, ids, "baseline", labels),
        "flash": evaluate(flash, ids, "judge", labels),
        "pro": evaluate(pro, ids, "judge", labels),
    }
    return {
        "comparisons": {
            "baseline_to_flash": compare(results["baseline"], results["flash"], ids),
            "baseline_to_pro": compare(results["baseline"], results["pro"], ids),
            "flash_to_pro": compare(results["flash"], results["pro"], ids),
        },
        "inputs": {
            "flash_sha256": FLASH_SHA256,
            "pro_sha256": PRO_SHA256,
            "source_id_map_sha256": sha256(id_map_path),
        },
        "labels": labels,
        "method": "Independent integer recount from frozen JSONL rows; no Validator code or output used.",
        "results": results,
        "schema_version": 1,
        "source_counts": {
            "flash_explicit_residue": sum(row["judge"] is None and not row["unparseable"] for row in flash),
            "flash_unreadable": sum(row["unparseable"] for row in flash),
            "historical_labels": len({row["label"] for row in flash}),
            "pro_explicit_residue": sum(row["judge"] is None and not row["unparseable"] for row in pro),
            "pro_unreadable": sum(row["unparseable"] for row in pro),
            "rows": len(ids),
        },
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--flash", required=True, type=Path)
    parser.add_argument("--pro", required=True, type=Path)
    parser.add_argument("--id-map", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        result = build(args.flash.resolve(), args.pro.resolve(), args.id_map.resolve())
        if args.out.exists():
            raise ValueError(f"refusing to replace existing artifact {args.out}")
        args.out.write_bytes(canonical_bytes(result))
        args.out.chmod(0o600)
    except ValueError as error:
        print(f"oracle_resolver: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
