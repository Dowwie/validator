"""Turn completed native proof artifacts into a durable, scoped execution record.

Run only after the verification commands succeed. This writes documentation and
outcomes, not reference labels, model predictions, or application source changes.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re

from poc import read_json, encoded


def record(root: Path, evidence: Path, run_url: str, trigger_commit: str):
    summary = read_json(evidence / "runs/summary.json")
    if summary.get("live_model_calls") != 0 or summary.get("human_approvals_created") != 0:
        raise ValueError("this record template describes only the offline execution")
    replay = summary["jgrep_published_replay"]
    if replay.get("status") != "passed" or len(replay.get("modes", {})) != 6:
        raise ValueError("all six published replay modes must pass before recording")
    if any(v["status"] != "passed" for v in summary["adapter_fixtures"].values()):
        raise ValueError("adapter fixture verification is incomplete")
    unit = (evidence / "example-tests.log").read_text()
    matches = re.findall(r"Ran (\d+) tests", unit)
    if not matches or not re.search(r"\nOK(?:\s|$)", unit):
        raise ValueError("successful Python test evidence missing")
    rust = (evidence / "cargo-test.log").read_text()
    results = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed", rust)
    if not results or any(int(failed) for _, failed in results):
        raise ValueError("successful native test evidence missing")
    source_paths = (list((root / "src").rglob("*.rs")) + list((root / "tests").glob("*.rs"))
                    + list((root / "schemas/v2").glob("*.json"))
                    + list((root / "examples/jev-poc").glob("*.py"))
                    + list((root / "examples/jev-poc").glob("*.mjs"))
                    + [root / n for n in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml")])
    summary["verification"] = {
        "actions_run": run_url, "trigger_commit": trigger_commit,
        "python_tests": int(matches[-1]), "rust_tests": sum(int(passed) for passed, _ in results),
        "live_capture": "not_run: no provider credentials and no new human reference approval",
        "source_sha256": {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(source_paths)},
        "log_sha256": {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(evidence.glob("*.log"))},
    }
    outcome_path = root / "examples/jev-poc/outcomes.json"
    outcome_path.write_bytes(encoded(summary))
    rows = []
    for mode, result in replay["modes"].items():
        rows.append(f"| `{mode}` | {result['tp']} | {result['fp']} | {result['fn']} | {result['tn']} | {result['accuracy']:.3f} |")
    comparisons = []
    for name, values in replay["comparisons"].items():
        comparisons.append(f"| `{name}` | {values['recovered']} | {values['regressed']} | {values['changed_final_outcomes']} |")
    count = sum(v['development'] + v['held_out'] for v in summary['draft_casebooks'].values())
    doc = f'''# Jev proof-of-concept execution record

Authority: **Decision record**. This is a bounded execution result, not an owner
acceptance or production-accuracy claim. The implementation is described in the
[example README](../../examples/jev-poc/README.md); machine results and exact source
hashes are in [outcomes.json](../../examples/jev-poc/outcomes.json).

## Executed

The [GitHub Actions run]({run_url}) applied the proposed example changes to
triggering commit `{trigger_commit}` and exercised the pinned Rust toolchain.
The compiled worktree's exact source hashes are retained in outcomes.json; the
triggering commit itself contains the transfer boundary, not a claim that all
applied source files were already committed there. The later evidence commit
adds this record and outputs without changing the tested application source.

* {summary['verification']['rust_tests']} Rust tests passed, including the three new CLI help tests.
* {summary['verification']['python_tests']} Python contract tests passed.
* All {len(summary['adapter_fixtures'])} synthetic adapter profiles completed native check/evaluate,
  inspection, identical comparison, relocated inspection, and tamper rejection.
* Six recorded jgrep arms and four compatible native paired comparisons completed.
* Sift capture passed Node syntax and dry-run checks. No live model calls were made.
* {count} reference proposals were generated; **zero new human approvals** were created.

Formatting, warning-denied Clippy, and the native build ran before the examples.
Logs and full native run bundles are retained in the workflow artifact; the
published upstream bytes and this numerical outcome summary are also committed
so the offline replay does not depend on artifact retention. The artifact may
expire; rerun the documented command to regenerate complete run bundles.

## Recorded Jev replay, not new inference

The source is `keltokhy/jgrep@{replay['commit']}`. Gold is the upstream author's
20-case handwritten diagnostic. It is not held out, not a production sample, and
has not acquired new independent human approval here. Diff judgments and current-
function judgments are separate tasks with different reference labels.

| Recorded input mode | TP | FP | FN | TN | Case accuracy |
|---|---:|---:|---:|---:|---:|
{chr(10).join(rows)}

Case decisions were reconstructed from all published scored rows using the
original any-record-at-or-above-0.5 rule. The published summary ID sets were used
only as an independent cross-check. A record's probability, or their maximum,
was not relabeled as a calibrated case probability. Validator's counts and
accuracy were then checked against separately reconstructed case accounting.

| Comparison (baseline → candidate) | Recovered cases | Regressed cases | Changed decisions |
|---|---:|---:|---:|
{chr(10).join(comparisons)}

The complete-hunk contrast changes both input unit and question framing; it is
not a clean single-variable ablation. These results establish faithful replay
and actionable disagreements, not a universal improvement in Jev. Inspect the
retained false-positive and false-negative IDs in outcomes.json before choosing
a new experiment. No reference labels were changed to make an arm look better.

## Flexibility established

Application-specific adapters preserve native outcomes and auxiliary evidence.
Mixed originating models/configurations receive distinct source definitions.
Partial scoring families are disclosed and retained as observations without
dropping cases, inventing values, or weakening Validator's all-or-none rule.
Rounded categorical distributions are not silently normalized. Score-like fit,
selected-category probability, minimum-stage probability and native confidence
remain different quantities.

The only Rust behavior change is CLI discovery: all four commands are listed,
per-command `--help`/`-h` works, and required output destinations are explicit.
The numerical core, schemas, metrics and persisted run contracts are unchanged.
Unknown commands still return the existing machine error contract.

## Not completed

There are no new Jev accuracy measurements for Sift, Foreman, Upwork, tax, generic
classification, filing, compaction, routing or jev-align. Their current outcomes
are source-shaped synthetic adapter proofs and pending-reference proposals.
The live Sift runner is provided but not live-tested. Other application-native
capture runners and full upstream runtime integration tests are not implemented.

The optional new jgrep cases, tax page-kind extension, router policy-fixture suite,
full-registry tax capture, independent semantic review, production distributions,
and optimization experiments remain unexecuted. The router draft uses a family-
preserving 39/21 split rather than forcing the planned 40/20. Template correlation
and synthetic-reference bias limit every drafted corpus.

The next substantive proof is a human-reviewed Sift subset through the provided
bounded capture runner, followed by native run → review → recommend. No task
management, autonomous remediation, gold revision, or provider telemetry was added.
'''
    path = root / "docs/acceptance/jev-poc-execution.md"
    path.write_text(doc, encoding="utf-8")
    return {"record": str(path), "rust_tests": summary['verification']['rust_tests'],
            "python_tests": summary['verification']['python_tests'], "draft_cases": count}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--run-url", required=True)
    parser.add_argument("--trigger-commit", required=True)
    args = parser.parse_args()
    print(json.dumps(record(args.root, args.evidence, args.run_url, args.trigger_commit), indent=2))
