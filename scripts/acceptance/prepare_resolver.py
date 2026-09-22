#!/usr/bin/env python3
"""Prepare the frozen resolver250 study as canonical single-label inputs."""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import stat
import sys
import uuid
from datetime import UTC, datetime
from pathlib import Path
from typing import Any


FLASH_SHA256 = "c631d0a540f939b1b7d14e41464c4aa5d8877c4b6598b408ead90b714fc45bcf"
PRO_SHA256 = "8f891d22424433201284e5aa32f380a3ccbadbf29fc1a16a8548c073c6f7caa2"
RESIDUE_LABEL = "RESIDUE"
UNREADABLE_REASON = "unreadable_source_response"
STUDY_TIMESTAMP_MS = int(datetime(2026, 7, 30, tzinfo=UTC).timestamp() * 1000)
OUTPUT_NAMES = (
    "golden.json",
    "baseline.json",
    "flash.json",
    "pro.json",
    "config-full.json",
    "source-id-map.json",
    "preparation-script.py",
    "preparation-receipt.json",
)


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
    ).encode() + b"\n"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    try:
        rows = [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines()]
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"cannot read JSONL {path}") from error
    if len(rows) != 250 or not all(isinstance(row, dict) for row in rows):
        raise ValueError(f"{path} must contain exactly 250 object rows")
    return rows


def require_source(path: Path, expected_sha256: str) -> list[dict[str, Any]]:
    if sha256(path) != expected_sha256:
        raise ValueError(f"source hash mismatch: {path}")
    return read_jsonl(path)


def row_identity(row: dict[str, Any]) -> dict[str, Any]:
    identity = {
        key: row.get(key)
        for key in ("phrase", "quote", "label", "baseline_top1", "label_in_shortlist")
    }
    if not all(isinstance(identity[key], str) for key in ("phrase", "quote", "label", "baseline_top1")):
        raise ValueError("resolver row has an invalid text identity field")
    if not isinstance(identity["label_in_shortlist"], bool):
        raise ValueError("resolver row has an invalid shortlist identity field")
    return identity


def episode_id(index: int, identity: dict[str, Any]) -> str:
    digest = hashlib.sha256(canonical_bytes(identity)).digest()
    value = bytearray((STUDY_TIMESTAMP_MS + index).to_bytes(6, "big") + digest[:10])
    value[6] = (value[6] & 0x0F) | 0x70
    value[8] = (value[8] & 0x3F) | 0x80
    return str(uuid.UUID(bytes=bytes(value)))


def judge_outcome(row: dict[str, Any]) -> dict[str, Any]:
    judge = row.get("judge")
    unreadable = row.get("unparseable")
    if not isinstance(unreadable, bool):
        raise ValueError("resolver row has an invalid unparseable flag")
    if unreadable:
        if judge is not None:
            raise ValueError("unreadable resolver row unexpectedly has a judge class")
        return {"reason": UNREADABLE_REASON, "type": "abstention"}
    if judge is None:
        return {"label": RESIDUE_LABEL, "type": "class"}
    if not isinstance(judge, str) or not judge:
        raise ValueError("resolver row has an invalid judge class")
    return {"label": judge, "type": "class"}


def class_outcome(label: Any) -> dict[str, Any]:
    if not isinstance(label, str) or not label:
        raise ValueError("resolver row has an invalid class")
    return {"label": label, "type": "class"}


def write_new(path: Path, value: Any) -> None:
    if path.exists():
        raise ValueError(f"refusing to replace existing artifact {path}")
    path.write_bytes(canonical_bytes(value))
    path.chmod(0o600)


def copy_new(source: Path, destination: Path) -> None:
    if destination.exists():
        raise ValueError(f"refusing to replace existing artifact {destination}")
    shutil.copyfile(source, destination)
    destination.chmod(0o600)


def require_private_directory(path: Path) -> None:
    if not path.exists():
        path.mkdir(mode=0o700)
    if not path.is_dir() or stat.S_IMODE(path.stat().st_mode) != 0o700:
        raise ValueError(f"output directory must be mode 0700: {path}")


def source_definition(
    *, model: str, route: str, evidence: list[str], raw_sha256: str
) -> dict[str, Any]:
    return {
        "configuration": {
            "mapping": {
                "explicit_null_verdict": RESIDUE_LABEL,
                "unparseable_response": UNREADABLE_REASON,
            },
            "raw_sha256": raw_sha256,
            "route": route,
            "study": "step12b-judge-precision-2026-07-30",
        },
        "evidence": evidence,
        "kind": "classifier",
        "model": model,
        "preparation": {
            "configuration": {
                "explicit_null_verdict": RESIDUE_LABEL,
                "unparseable_response": UNREADABLE_REASON,
            },
            "evidence_indices": list(range(len(evidence))),
            "method": "resolver250_explicit_outcome_mapping",
            "version": "resolver250-v1",
        },
    }


def prediction_document(
    source_id: str,
    source: dict[str, Any],
    rows: list[dict[str, Any]],
    ids: list[str],
    dataset_sha256: str,
    outcome_key: str,
) -> dict[str, Any]:
    predictions = []
    for episode, row in zip(ids, rows, strict=True):
        outcome = class_outcome(row[outcome_key]) if outcome_key == "baseline_top1" else judge_outcome(row)
        predictions.append({"id": episode, "outcome": outcome, "source_id": source_id})
    return {
        "dataset_sha256": dataset_sha256,
        "predictions": predictions,
        "schema_version": 2,
        "sources": {source_id: source},
    }


def prepare(flash_path: Path, pro_path: Path, out: Path) -> None:
    flash = require_source(flash_path, FLASH_SHA256)
    pro = require_source(pro_path, PRO_SHA256)
    flash_identities = [row_identity(row) for row in flash]
    pro_identities = [row_identity(row) for row in pro]
    if flash_identities != pro_identities or len({canonical_bytes(row) for row in flash_identities}) != 250:
        raise ValueError("resolver arms do not have the same unique row identities")

    require_private_directory(out)
    if any((out / name).exists() for name in OUTPUT_NAMES):
        raise ValueError("base preparation artifacts already exist")
    sources = out / "sources"
    sources.mkdir(mode=0o700)
    copy_new(flash_path, sources / "flash.jsonl")
    copy_new(pro_path, sources / "pro.jsonl")

    ids = [episode_id(index, identity) for index, identity in enumerate(flash_identities)]
    labels = {RESIDUE_LABEL}
    for rows in (flash, pro):
        for row in rows:
            labels.update((row["label"], row["baseline_top1"]))
            if row["judge"] is not None:
                labels.add(row["judge"])
    vocabulary = sorted(labels)
    golden = {
        "episodes": [
            {
                "expected": {"label": row["label"], "type": "class"},
                "id": episode,
                "input": {"phrase": row["phrase"], "quote": row["quote"], "source_row": index},
            }
            for index, (episode, row) in enumerate(zip(ids, flash, strict=True))
        ],
        "schema_version": 2,
        "task": {"kind": "single_label", "labels": vocabulary},
    }
    write_new(out / "golden.json", golden)
    dataset_sha256 = sha256(out / "golden.json")

    common_evidence = ["preparation-script.py", "source-id-map.json", "preparation-receipt.json"]
    baseline_evidence = ["sources/flash.jsonl", "sources/pro.jsonl", *common_evidence]
    flash_evidence = ["sources/flash.jsonl", *common_evidence]
    pro_evidence = ["sources/pro.jsonl", *common_evidence]
    documents = {
        "baseline.json": prediction_document(
            "fused_retrieval_top1",
            source_definition(
                model="fused-retrieval-top-1",
                route="baseline_top1",
                evidence=baseline_evidence,
                raw_sha256=f"{FLASH_SHA256}+{PRO_SHA256}",
            ),
            flash,
            ids,
            dataset_sha256,
            "baseline_top1",
        ),
        "flash.json": prediction_document(
            "deepseek_v4_flash",
            source_definition(
                model="deepseek-v4-flash",
                route="judge",
                evidence=flash_evidence,
                raw_sha256=FLASH_SHA256,
            ),
            flash,
            ids,
            dataset_sha256,
            "judge",
        ),
        "pro.json": prediction_document(
            "deepseek_v4_pro",
            source_definition(
                model="deepseek-v4-pro",
                route="judge",
                evidence=pro_evidence,
                raw_sha256=PRO_SHA256,
            ),
            pro,
            ids,
            dataset_sha256,
            "judge",
        ),
    }
    for name, document in documents.items():
        write_new(out / name, document)
    write_new(
        out / "config-full.json",
        {
            "decision": {"type": "as_recorded"},
            "population": "resolver250_historical_label_agreement",
            "role": "development",
            "schema_version": 2,
        },
    )
    write_new(
        out / "source-id-map.json",
        {
            "cases": [
                {
                    "episode_uuid": episode,
                    "row_identity_sha256": hashlib.sha256(canonical_bytes(identity)).hexdigest(),
                    "source_row": index,
                }
                for index, (episode, identity) in enumerate(zip(ids, flash_identities, strict=True))
            ],
            "schema_version": 1,
        },
    )
    copy_new(Path(__file__), out / "preparation-script.py")

    durable_names = [
        "golden.json",
        "baseline.json",
        "flash.json",
        "pro.json",
        "config-full.json",
        "source-id-map.json",
        "preparation-script.py",
        "sources/flash.jsonl",
        "sources/pro.jsonl",
    ]
    receipt = {
        "canonical_outputs": {name: sha256(out / name) for name in durable_names},
        "input_counts": {
            "rows": 250,
            "historical_label_count": len({row["label"] for row in flash}),
            "vocabulary_count": len(vocabulary),
        },
        "outcome_mapping": {
            "explicit_null_verdict": RESIDUE_LABEL,
            "flash_unreadable_count": sum(row["unparseable"] for row in flash),
            "pro_unreadable_count": sum(row["unparseable"] for row in pro),
            "unparseable_response": UNREADABLE_REASON,
        },
        "schema_version": 1,
        "source_inputs": {"flash_sha256": FLASH_SHA256, "pro_sha256": PRO_SHA256},
        "transformation": {"method": "prepare_resolver.py", "version": "resolver250-v1"},
    }
    write_new(out / "preparation-receipt.json", receipt)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--flash", required=True, type=Path)
    parser.add_argument("--pro", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        prepare(args.flash.resolve(), args.pro.resolve(), args.out.resolve())
    except ValueError as error:
        print(f"prepare_resolver: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
