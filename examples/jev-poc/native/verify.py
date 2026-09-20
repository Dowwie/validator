#!/usr/bin/env python3
"""Execute pinned application logic, then replay its fixture outputs through Validator.

Sources must already be fetched. This command has no fetch or inference operation.
All model replies and reference targets here are explicit software-test fixtures.
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

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
from adapters import FOREMAN_LABELS  # noqa: E402
from casebooks import make_books  # noqa: E402
from poc import command, encoded, native_run, prepare, read_json, write_private  # noqa: E402


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sources", type=Path, required=True)
    parser.add_argument("--validator", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    binary = args.validator.resolve()
    if not binary.is_file():
        parser.error("--validator must be an existing executable")
    sources, out = args.sources.resolve(), args.out.resolve()
    out.mkdir(parents=True, exist_ok=False, mode=0o700)
    books = make_books()
    environment = dict(os.environ)
    # Upstream test code receives no provider credentials, even on a developer host.
    for name in list(environment):
        if any(x in name.upper() for x in ("API_KEY", "TOKEN", "SECRET")):
            environment.pop(name)
    evidence = {}
    for name in ("router", "sift", "foreman"):
        write_private(out / f"{name}.book.json", encoded(books[name]))
        script = HERE / ("foreman.py" if name == "foreman" else f"{name}.mjs")
        invocation = ([sys.executable] if name == "foreman" else ["node"]) + [str(script), "--upstream", str(sources / name), "--out", str(out / f"{name}.capture.json")]
        if name != "router":
            invocation += ["--book", str(out / f"{name}.book.json")]
        run = subprocess.run(invocation, capture_output=True, text=True, env=environment, timeout=60)
        write_private(out / f"{name}.log", (run.stdout + run.stderr).encode())
        if run.returncode:
            raise RuntimeError(f"{name} native test failed: {run.stdout} {run.stderr}")
        capture = read_json(out / f"{name}.capture.json")
        if name == "router":
            # This gold is a hand-derived *software policy* fixture, not human-approved
            # semantic routing gold. No invented human approval is written anywhere.
            book = dict(books[name], task={"kind": "single_label", "labels": ["haiku", "sonnet", "opus", "fable"]}, cases=[])
            for row in capture["cases"]:
                book["cases"].append({"case_id": row["id"], "family_id": row["id"], "partition": "development",
                                      "model_input": row["input"], "proposed_expected": {"type": "class", "label": row["expected"]["tier"]},
                                      "rationale": "hand-derived deterministic policy fixture", "review": {"state": "pending", "reviewer": None}})
            baseline = {"schema_version": 1, "example": name, "origin": "synthetic_fixture", "complete": True,
                        "source": {"model": "synthetic-policy-fixture", "repository": capture["repository"], "commit": capture["commit"]},
                        "records": [{"case_id": r["id"], "output": r["baseline"], "request": r["input"], "configuration": {"minConfidence": .3}} for r in capture["cases"]]}
            candidate = dict(baseline, records=[{"case_id": r["id"], "output": r["candidate"], "request": r["input"], "configuration": {"minConfidence": .5}} for r in capture["cases"]])
            baseline_policy = candidate_policy = {}
            native_assertions = capture["passed"]
        else:
            selected = {r["case_id"] for r in capture["records"]}
            book = dict(books[name], cases=[c for c in books[name]["cases"] if c["case_id"] in selected])
            baseline = candidate = capture
            baseline_policy = {"threshold": .5}
            candidate_policy = ({"threshold": .7} if name == "sift" else {"thresholds": {n: (.8 if n == "ready_to_finish" else .5) for n in FOREMAN_LABELS}})
            native_assertions = len(capture["tests"])
        runs = {}
        for arm, data, policy in [("baseline", baseline, baseline_policy), ("candidate", candidate, candidate_policy)]:
            prepared = out / name / f"{arm}-prepared"
            prepare(book, data, prepared, policy=policy, fixture=True)
            report, receipt = native_run(binary, prepared, out / name / arm)
            if len(report["episodes"]) != len(book["cases"]):
                raise AssertionError("population was changed")
            runs[arm] = (report, receipt)
        receipt = command(binary, ["compare", "--baseline", out/name/"baseline", "--candidate", out/name/"candidate", "--out", out/name/"comparison"])
        comparison_path = Path(receipt["result_path"])
        assert hashlib.sha256(comparison_path.read_bytes()).hexdigest() == receipt["result_sha256"]
        comparison = read_json(comparison_path)
        # Independently hand-derived changed-case/transition expectations for these
        # synthetic replies; never inferred by trusting the adapter's predictions.
        wanted = {"router": (1, 0, 1), "sift": (1, 1, 0), "foreman": (1, 1, 0)}[name]
        for key, n in zip(("changed_final_outcomes", "recovered", "regressed"), wanted):
            assert comparison["transitions"][key]["count"] == n, (name, key, comparison["transitions"][key], n)
        # The application capture and all review-pending authoring survive as evidence.
        relocated = out/name/"relocated"
        shutil.copytree(out/name/"baseline", relocated)
        first_id = runs["baseline"][0]["episodes"][0]["id"]
        command(binary,["inspect","--run",relocated,"--episode",first_id])
        shutil.rmtree(relocated)
        evidence[name] = {"native_assertions": native_assertions, "fixture_episodes": len(book["cases"]),
                          "origin": "synthetic_fixture", "model_accuracy_claim": None,
                          "native_capture_sha256": hashlib.sha256((out/f"{name}.capture.json").read_bytes()).hexdigest(),
                          "baseline_report_sha256": runs["baseline"][1]["result_sha256"],
                          "candidate_report_sha256": runs["candidate"][1]["result_sha256"],
                          "comparison_sha256": receipt["result_sha256"],
                          "expected_transition_assertions_passed": True,
                          "checks": ["actual upstream runtime", "check", "evaluate", "inspect", "paired comparison", "relocated inspect"]}
    summary = {"schema_version": 1, "status": "passed", "live_model_calls": 0, "human_approvals_created": 0,
               "validator_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(), "results": evidence,
               "limitations": ["All application model responses are explicit fixtures; these results do not establish Jev accuracy.",
                               "Foreman exercises upstream's injected-client path, not a live or installed TypeSafe SDK transport.",
                               "Native source retrieval/dependency installation is separate; this command is offline."]}
    write_private(out/"summary.json", encoded(summary))
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
