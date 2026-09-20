#!/usr/bin/env python3
"""Prepare application captures for Validator; run bounded offline proofs.

This is example support, not a replacement evaluator or an agent/task manager.
Only `fetch-jgrep` accesses the network. `prepare` and `verify` are offline.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import uuid

from adapters import ContractError, FOREMAN_LABELS, decode, harmonize, class_out, labels_out
from casebooks import make_books, write_books

HERE = Path(__file__).resolve().parent
NAMESPACE = uuid.UUID("7ce937f1-91b3-5fba-8763-6587cdb48696")
ORIGINS = {"synthetic_fixture", "observed", "published_replay"}


def strict_json(data: bytes):
    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise ContractError(f"duplicate JSON key: {key}")
            result[key] = value
        return result
    def invalid(value):
        raise ContractError(f"non-finite JSON token: {value}")
    return json.loads(data, object_pairs_hook=pairs, parse_constant=invalid)


def read_json(path: Path):
    return strict_json(path.read_bytes())


def encoded(value) -> bytes:
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=False, allow_nan=False) + "\n").encode()


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def episode_id(example: str, case_id: str) -> str:
    # JSON framing avoids ambiguous concatenations ("a:b", "c") / ("a", "b:c").
    return str(uuid.uuid5(NAMESPACE, json.dumps([example, case_id], separators=(",", ":"))))


def assert_no_credentials(value):
    forbidden = {"apikey", "api_key", "authorization", "access_token", "typesafe_api_key", "jev_api_key"}
    if isinstance(value, dict):
        if any(str(k).lower() in forbidden for k in value):
            raise ContractError("capture contains a credential field; remove transport authentication before saving")
        for v in value.values():
            assert_no_credentials(v)
    elif isinstance(value, list):
        for v in value:
            assert_no_credentials(v)


def validate_book(book: dict):
    if book.get("schema_version") != 1 or not isinstance(book.get("example"), str):
        raise ContractError("unsupported authoring schema")
    task = book.get("task", {})
    labels = task.get("labels")
    if task.get("kind") not in {"single_label", "multi_label"}:
        raise ContractError("unsupported task shape")
    if (not isinstance(labels, list) or len(labels) < 2 or
        not all(isinstance(x, str) and x.strip() for x in labels) or len(set(labels)) != len(labels)):
        raise ContractError("need at least two distinct nonblank frozen labels")
    cases = book.get("cases")
    if not isinstance(cases, list) or not cases:
        raise ContractError("authoring cases must be a nonempty list")
    ids, families = set(), {}
    for c in cases:
        cid, family, partition = c.get("case_id"), c.get("family_id"), c.get("partition")
        if not isinstance(cid, str) or not cid or cid in ids:
            raise ContractError("case IDs must be unique nonempty strings")
        if not isinstance(family, str) or not family:
            raise ContractError("family ID is required")
        if partition not in {"development", "held_out"}:
            raise ContractError("invalid reference partition")
        if family in families and families[family] != partition:
            raise ContractError("related family leaks across development and held-out partitions")
        ids.add(cid); families[family] = partition
        expected = c.get("proposed_expected", {})
        if task["kind"] == "single_label":
            if expected.get("type") != "class":
                raise ContractError("expected class target")
            class_out(expected.get("label"), labels)
        else:
            if expected.get("type") != "labels":
                raise ContractError("expected complete label set")
            labels_out(expected.get("labels"), labels)
        if "model_input" not in c:
            raise ContractError("missing source input")
        validate_model_input(book["example"], c["model_input"])
    return book


def validate_model_input(example: str, value):
    """Source-grounded structural checks, not semantic adjudication of inputs."""
    if not isinstance(value, dict):
        raise ContractError("model_input must be an explicit field projection")
    if set(value) & {"proposed_expected", "rationale", "review", "partition"}:
        raise ContractError("reference-only metadata leaked into the request projection")
    string_fields = {
        "sift": ("query", "text"), "classifier-dev": ("text",),
        "filing": ("fileName", "text"), "jev-align": ("title", "text"),
        "router": ("prompt", "current"), "jgrep": ("before", "after", "task"),
        "compaction": ("goal", "tool_use_id"),
    }
    for key in string_fields.get(example, ()):
        if not isinstance(value.get(key), str):
            raise ContractError(f"{example}.{key} must be a string")
    if example == "tax":
        if not isinstance(value.get("lines"), list) or not all(isinstance(x, str) for x in value["lines"]):
            raise ContractError("tax input requires extracted text lines")
    if example == "upwork":
        if not all(isinstance(value.get(k), dict) for k in ("job", "client", "freelancer")):
            raise ContractError("Upwork state requires job, client, and freelancer objects")
        if not isinstance(value["job"].get("description"), str):
            raise ContractError("Upwork job description must be a string")
    if example == "filing" and (not isinstance(value.get("categories"), list) or
                                  not all(isinstance(x, str) for x in value["categories"])):
        raise ContractError("filing categories must be an explicit list")
    if example == "foreman":
        # FactoryObservation at a7d21d18: these are maps/lists, NOT scalar summaries.
        strings = {"original_job", "run_id", "factory_status", "latest_worker_output", "git_status", "git_diff", "agents_md_instructions"}
        maps = {"worker_exit_status", "worker_elapsed_seconds"}
        lists = {"active_workers", "worker_history", "changed_files", "test_results", "verification_results", "recent_events", "failures"}
        required = strings | maps | lists | {"iteration", "previous_assessment", "previous_intervention", "attempts", "elapsed_factory_seconds"}
        if not required <= set(value) or set(value) - required - {"agents_md_path"}:
            raise ContractError("Foreman observation has missing/unknown fields")
        if not all(isinstance(value[k], str) for k in strings):
            raise ContractError("Foreman observation text fields must be strings")
        if not all(isinstance(value[k], dict) for k in maps) or not all(isinstance(value[k], list) for k in lists):
            raise ContractError("Foreman map/list field shape mismatch")
        for key in ("iteration", "attempts"):
            if isinstance(value[key], bool) or not isinstance(value[key], int) or value[key] < 0:
                raise ContractError("Foreman counters must be nonnegative integers")
    if example == "compaction":
        if not isinstance(value.get("messages"), list):
            raise ContractError("compaction messages must be a list")
        ids = []
        for message in value["messages"]:
            if message.get("role") not in {"user", "assistant"} or not isinstance(message.get("text"), str):
                raise ContractError("unsupported native Message shape")
            if not isinstance(message.get("toolUses"), list):
                raise ContractError("toolUses must be explicit")
            ids.extend(t["tool_use_id"] for t in message["toolUses"])
        if len(ids) != len(set(ids)) or ids.count(value["tool_use_id"]) != 1:
            raise ContractError("compaction focal tool call must be unique and present")


def approved(c: dict) -> bool:
    review = c.get("review", {})
    return (review.get("state") == "approved" and review.get("method") == "human"
            and isinstance(review.get("reviewer"), str) and bool(review["reviewer"].strip()))


def write_bundle(book: dict, capture: dict, rows: list[dict], destination: Path,
                 partition: str, policy: dict, *, fixture: bool = False,
                 accept_author_fixture: bool = False, extra_evidence: dict[str, bytes] | None = None):
    """Write canonical inputs after complete checking, never silently drop a row.

    A partition gets its own golden file: development run snapshots cannot contain
    held-out reference labels. Baseline/candidate hashes remain equal *within* it.
    """
    validate_book(book)
    if capture.get("schema_version") != 1 or capture.get("origin") not in ORIGINS:
        raise ContractError("capture requires schema_version=1 and an explicit origin")
    if capture.get("example") != book["example"]:
        raise ContractError("capture/book adapter mismatch")
    if fixture != (capture["origin"] == "synthetic_fixture"):
        raise ContractError("synthetic captures require fixture mode; fixture mode refuses observed captures")
    selected = [c for c in book["cases"] if c["partition"] == partition]
    if not selected:
        raise ContractError("selected reference population is empty")
    author_fixture = (accept_author_fixture and capture["origin"] == "published_replay"
                      and book.get("reference_authority") == "upstream_author_fixture")
    if not fixture and not author_fixture and not all(approved(c) for c in selected):
        raise ContractError("human approval is missing; do not score proposed gold against observed predictions")
    row_ids = [r.get("case_id") for r in rows]
    if len(set(row_ids)) != len(row_ids) or set(row_ids) != {c["case_id"] for c in selected}:
        raise ContractError("predictions must cover exactly the selected reference IDs")
    assert_no_credentials(capture)
    assert_no_credentials([c["model_input"] for c in selected])
    source = capture.get("source", {})
    if not isinstance(source, dict) or not isinstance(source.get("model"), str) or not source["model"].strip():
        raise ContractError("capture needs the actual established model or an explicitly unresolved requested alias")
    for identity in ("repository", "commit"):
        if identity in source and source[identity] != book.get("source", {}).get(identity):
            raise ContractError("capture and casebook application identities disagree")
    by_id = {r["case_id"]: r for r in rows}
    selected.sort(key=lambda c: episode_id(book["example"], c["case_id"]))
    gold = {"schema_version": 2, "task": book["task"], "episodes": [
        {"id": episode_id(book["example"], c["case_id"]), "input": c["model_input"],
         "expected": c["proposed_expected"]} for c in selected]}
    predictions = []
    source_variants = {}
    for c in selected:
        result = dict(by_id[c["case_id"]]["prediction"])
        result["observations"] = dict(result.get("observations", {}))
        row_model = by_id[c["case_id"]].get("source_model", source["model"])
        row_configuration = by_id[c["case_id"]].get("source_configuration", {})
        if not isinstance(row_model, str) or not row_model.strip() or not isinstance(row_configuration, dict):
            raise ContractError("invalid per-record source model/configuration")
        variant = {"model": row_model, "configuration": row_configuration}
        sid = "application-" + digest(encoded(variant))[:16]
        if sid in source_variants and source_variants[sid] != variant:
            raise ContractError("source identity collision")
        source_variants[sid] = variant
        result.update(id=episode_id(book["example"], c["case_id"]), source_id=sid)
        predictions.append(result)
    demoted = harmonize(predictions)
    definitions = {}
    for row in predictions:
        for name, observation in row.get("observations", {}).items():
            kind = observation["kind"]
            if name in definitions and definitions[name]["kind"] != kind:
                raise ContractError("observation kind changes across records")
            definitions[name] = {"kind": kind, "description": f"Recorded {name}; source/configuration and capture define its meaning. Not a reference label."}
    golden_bytes = encoded(gold)
    config = {"schema_version": 2, "population": f"{book['example']}:{partition}:{capture['origin']}",
              "role": partition, "decision": {"type": "as_recorded"}}
    evidence = {
        "capture.json": encoded(capture),
        "preparation.py": Path(__file__).read_bytes(),
        "adapters.py": (HERE / "adapters.py").read_bytes(),
    }
    evidence.update(extra_evidence or {})
    # Evidence names are adapter-owned basenames, never caller-supplied paths.
    for name in evidence:
        if Path(name).name != name or name in {".", "..", "preparation-receipt.json"}:
            raise ContractError("invalid evidence basename")
    receipt = {
        "schema_version": 1, "example": book["example"], "capture_origin": capture["origin"],
        "reference_authority": book.get("reference_authority"),
        "reference_status": "synthetic_test_only" if fixture else ("upstream_author_fixture" if author_fixture else "human_reviewed"),
        "selected_count": len(selected), "partition": partition,
        "withheld_other_partition_count": len(book["cases"]) - len(selected),
        "policy": policy, "demoted_partial_scoring_families": demoted,
        "dataset_sha256": digest(golden_bytes), "capture_sha256": digest(evidence["capture.json"]),
        "source_id_map": {c["case_id"]: episode_id(book["example"], c["case_id"]) for c in selected},
        "adapter_sha256": digest(evidence["adapters.py"]),
        "preparation_sha256": digest(evidence["preparation.py"]),
        "limitations": ["Hashes do not authenticate provenance claims.", "Synthetic or author-fixture agreement is not production accuracy."],
    }
    evidence["preparation-receipt.json"] = encoded(receipt)
    paths = [f"evidence/{name}" for name in evidence]
    definition = {"kind": "classifier", "model": source["model"],
                  "configuration": {"source": source, "application": book["source"],
                                    "task_configuration": book.get("configuration", {}),
                                    "origin": capture["origin"], "policy": policy,
                                    "reference_authority": receipt["reference_status"],
                                    "demoted_partial_scoring_families": demoted},
                  "evidence": paths, "observation_definitions": definitions,
                  "preparation": {"method": "jev-poc-explicit-adapter", "version": "1",
                                  "configuration": {"example": book["example"], "policy": policy,
                                                    "partition": partition, "demoted_families": demoted},
                                  "evidence_indices": list(range(len(paths)))}}
    sources = {}
    for sid, variant in source_variants.items():
        sources[sid] = dict(definition, model=variant["model"],
                           configuration=dict(definition["configuration"],
                                              originating_configuration=variant["configuration"]))
    artifact = {"schema_version": 2, "dataset_sha256": digest(golden_bytes),
                "sources": sources, "predictions": predictions}
    destination.mkdir(mode=0o700, parents=True, exist_ok=False)
    try:
        (destination / "evidence").mkdir(mode=0o700)
        for name, data in evidence.items():
            write_private(destination / "evidence" / name, data)
        write_private(destination / "golden.json", golden_bytes)
        write_private(destination / "predictions.json", encoded(artifact))
        write_private(destination / "config.json", encoded(config))
        write_private(destination / "preparation-receipt.json", encoded(receipt))
    except BaseException:
        shutil.rmtree(destination)
        raise
    return receipt


def write_private(path: Path, data: bytes):
    with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), "wb") as stream:
        stream.write(data)


def prepare(book, capture, destination, partition="development", policy=None, *, fixture=False):
    policy = policy or {}
    if capture.get("complete") is False:
        raise ContractError("capture is incomplete; unattempted cases cannot become negatives")
    if not isinstance(capture.get("records"), list):
        raise ContractError("capture.records must be a list of case-linked native outputs")
    rows = []
    for record in capture["records"]:
        if set(record) - {"case_id", "output", "request", "model", "attempt", "metadata", "configuration"}:
            raise ContractError("unknown capture record field")
        output = record.get("output")
        prediction = decode(book["example"], output, book["task"], policy)
        native_model = output.get("resolved_model") or output.get("model")
        if native_model and record.get("model") and native_model != record["model"]:
            raise ContractError("native and declared originating models disagree")
        model = native_model or record.get("model") or capture.get("source", {}).get("model")
        identity_field = {"sift": "id", "upwork": "case_id", "jev-align": "story_id"}.get(book["example"])
        if identity_field and identity_field in output and output[identity_field] != record.get("case_id"):
            raise ContractError("native result identity does not match capture case ID")
        rows.append({"case_id": record.get("case_id"), "prediction": prediction,
                     "source_model": model, "source_configuration": record.get("configuration", {})})
    return write_bundle(book, capture, rows, destination, partition, policy, fixture=fixture)


def command(binary: Path, args: list[str], expected_exit=0):
    completed = subprocess.run([str(binary), *map(str, args)], capture_output=True, text=True, timeout=90)
    if completed.returncode != expected_exit:
        raise ContractError(f"validator {' '.join(map(str,args))} exited {completed.returncode}: {completed.stdout} {completed.stderr}")
    try:
        document = strict_json(completed.stdout.encode())
    except (ValueError, TypeError) as exc:
        raise ContractError("validator must emit exactly one JSON document") from exc
    return document


def native_run(binary: Path, prepared: Path, out: Path):
    args = ["--dataset", prepared / "golden.json", "--predictions", prepared / "predictions.json", "--config", prepared / "config.json"]
    check = command(binary, ["check", *args])
    receipt = command(binary, ["evaluate", *args, "--out", out])
    result_path = Path(receipt["result_path"])
    if digest(result_path.read_bytes()) != receipt["result_sha256"]:
        raise ContractError("receipt hash mismatch")
    report = read_json(result_path)
    if check["integrity"]["selected_count"] != len(report["episodes"]):
        raise ContractError("check/evaluate selected population mismatch")
    episode = report["episodes"][0]["id"]
    inspection = command(binary, ["inspect", "--run", out, "--episode", episode])
    if inspection["episode_id"] != episode:
        raise ContractError("inspection mismatch")
    return report, receipt


def fixture_capture(book):
    """Clearly synthetic protocol examples, NOT observed Jev responses."""
    example, labels = book["example"], book["task"]["labels"]
    cases = [c for c in book["cases"] if c["partition"] == "development"][:3]
    records = []
    for i, c in enumerate(cases):
        p = (0.2, 0.5, 0.8)[i]
        if example == "sift":
            output = {"id": c["case_id"], "answers": {"relevant": {"type": "boolean", "probability": p}}, "model": "synthetic-fixture"}
        elif example == "foreman":
            output = {name: (0.8 if n == i else 0.2) for n, name in enumerate(labels)}
        elif example == "upwork":
            output = {"case_id": c["case_id"], "decision": labels[i], "fit": p,
                      "lane_confidence": 0.9, "gates": {"is_technical_engagement": p}, "normalized": {"core_stack_fit": p}}
        elif example == "tax":
            output = {"form": labels[i], "kind": "form_page", "formConfidence": p,
                      "kindConfidence": 0.9, "stepConfidences": [p], "gated": False,
                      "probabilities": {"form": {labels[i]: p, "not_in_this_list": 1-p}}}
        elif example == "classifier-dev":
            if book["task"]["kind"] == "multi_label":
                scores = {name: p for name in labels}
            else:
                scores = {name: (0.7 if n == i else 0.3 / (len(labels)-1)) for n, name in enumerate(labels)}
            output = {"label": labels[i], "confidence": 0.85, "scores": scores, "model": "synthetic-fixture"}
        elif example == "filing":
            output = {"category": labels[i], "categoryConfidence": p,
                      "destinationCategory": "Need review" if p < 0.75 else labels[i]}
        elif example == "compaction":
            output = {"id": f"t{i}", "tool": "Read", "keepCall": p, "keepResult": p,
                      "action": "keep" if p >= 0.5 else "drop_call", "reason": "synthetic demonstration"}
        elif example == "router":
            output = {"tier": labels[i], "reason": "synthetic policy fixture", "changed": True}
        else:
            output = {"story_id": c["case_id"], "probability": p, "resolved_model": "synthetic-fixture"}
        records.append({"case_id": c["case_id"], "output": output})
    short_book = dict(book, cases=cases)
    capture = {"schema_version": 1, "example": example, "origin": "synthetic_fixture",
               "source": {"model": "synthetic-fixture", "not_model_measurement": True}, "records": records}
    return short_book, capture


def verify(binary: Path, out: Path, upstream: Path | None):
    if not binary.is_file():
        raise ContractError("provide an existing Validator executable")
    out.mkdir(parents=True, exist_ok=False)
    summary = {"schema_version": 1, "binary_sha256": digest(binary.read_bytes()),
               "live_model_calls": 0, "human_approvals_created": 0,
               "draft_casebooks": write_books(out / "drafts"), "adapter_fixtures": {}}
    for slug, book in make_books().items():
        book, capture = fixture_capture(book)
        prepared = out / slug / "prepared"
        receipt = prepare(book, capture, prepared, fixture=True)
        report, run_receipt = native_run(binary, prepared, out / slug / "run")
        if len(report["episodes"]) != 3:
            raise ContractError("fixture population was changed")
        comparison_receipt = command(binary, ["compare", "--baseline", out / slug / "run", "--candidate", out / slug / "run", "--out", out / slug / "identical"])
        comparison = read_json(Path(comparison_receipt["result_path"]))
        if comparison["transitions"]["changed_final_outcomes"]["count"] != 0:
            raise ContractError("identical replay changed outcomes")
        relocated = out / slug / "relocated"
        shutil.copytree(out / slug / "run", relocated)
        command(binary, ["inspect", "--run", relocated, "--episode", report["episodes"][0]["id"]])
        (relocated / "evidence" / "0.bin").write_bytes(b"intentional tamper negative fixture")
        error = command(binary, ["inspect", "--run", relocated, "--episode", report["episodes"][0]["id"]], expected_exit=2)
        if error["code"] != "E_PROVENANCE":
            raise ContractError("tamper did not fail as provenance error")
        shutil.rmtree(relocated)
        summary["adapter_fixtures"][slug] = {"status": "passed", "origin": "synthetic_fixture",
            "population": 3, "dataset_sha256": receipt["dataset_sha256"], "report_sha256": run_receipt["result_sha256"],
            "checks": ["check", "evaluate", "inspect", "identical compare", "relocated inspect", "tamper rejected"],
            "accuracy_claim": None}
    if upstream is not None:
        from jgrep import verify_replay
        summary["jgrep_published_replay"] = verify_replay(binary, upstream, out / "jgrep")
    else:
        summary["jgrep_published_replay"] = {"status": "not_run", "reason": "no upstream artifact directory supplied"}
    (out / "summary.json").write_bytes(encoded(summary))
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    drafts = commands.add_parser("drafts", help="write unapproved proposed reference casebooks")
    drafts.add_argument("--out", type=Path, required=True)
    prep = commands.add_parser("prepare", help="convert approved references and native capture records")
    prep.add_argument("--book", type=Path, required=True)
    prep.add_argument("--capture", type=Path, required=True)
    prep.add_argument("--partition", choices=("development", "held_out"), default="development")
    prep.add_argument("--policy", type=Path)
    prep.add_argument("--out", type=Path, required=True)
    prep.add_argument("--fixture", action="store_true", help="synthetic captures only; never an observed-model approval bypass")
    check = commands.add_parser("verify", help="execute native adapter fixtures and optional published-output replay offline")
    check.add_argument("--validator", type=Path, required=True)
    check.add_argument("--out", type=Path, required=True)
    check.add_argument("--upstream", type=Path)
    fetch = commands.add_parser("fetch-jgrep", help="download hash-pinned public jgrep fixtures, outputs, and license; no model call")
    fetch.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == "drafts":
            result = write_books(args.out)
        elif args.command == "prepare":
            result = prepare(read_json(args.book), read_json(args.capture), args.out, args.partition,
                             read_json(args.policy) if args.policy else {}, fixture=args.fixture)
        elif args.command == "fetch-jgrep":
            from jgrep import fetch_sources
            result = fetch_sources(args.out)
        else:
            result = verify(args.validator.resolve(), args.out, args.upstream)
        print(json.dumps(result, indent=2, allow_nan=False))
    except (ContractError, ValueError, OSError, KeyError, TypeError, subprocess.TimeoutExpired) as exc:
        print(json.dumps({"status": "error", "message": str(exc)}), file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
