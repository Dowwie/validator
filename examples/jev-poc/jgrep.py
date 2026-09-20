"""Replay published jgrep observations against their original author-fixture labels.

Downloading the four pinned public files is an explicit separate step. Replaying
makes no model call. MIT attribution travels with every prepared evidence bundle.
"""
from __future__ import annotations

import hashlib
from pathlib import Path
import urllib.request

from adapters import ContractError, class_out, number

REPOSITORY = "keltokhy/jgrep"
COMMIT = "fdceb6bdf79165a133667b8e57f3b7244545f0a2"
FILES = {
    "cases.json": ("bench/fixtures/code_review.json", "61e368acf3651c67765e06e208070ede790418c9"),
    "results.json": ("docs/benchmarks/code-review.json", "f4032fa0d787a1340df68d0c7c8a9c1489e2c482"),
    "LICENSE": ("LICENSE", "b29e006c91695d4588a414429671cb3932af40b0"),
    "measurement.py": ("bench/code_review.py", "63b017370a3bf0834b7107c5dcaad10af3dcddd5"),
}
MODES = {"diff_lines": "diff", "added_lines": "diff", "diff_hunks": "diff",
         "function_lines": "function", "function_line_context": "function", "functions": "function"}


def blob_sha(data: bytes) -> str:
    return hashlib.sha1(f"blob {len(data)}\0".encode() + data).hexdigest()


def fetch_sources(out: Path):
    from poc import encoded, digest
    # Complete downloads and hash checks before publishing anything.
    payloads, manifest = {}, {"repository": REPOSITORY, "commit": COMMIT, "files": {}}
    for name, (path, expected) in FILES.items():
        url = f"https://raw.githubusercontent.com/{REPOSITORY}/{COMMIT}/{path}"
        with urllib.request.urlopen(url, timeout=30) as response:
            data = response.read(5_000_001)
        if len(data) > 5_000_000 or blob_sha(data) != expected:
            raise ContractError(f"unexpected upstream bytes for {path}")
        payloads[name] = data
        manifest["files"][name] = {"url": url, "git_blob_sha": expected, "sha256": digest(data)}
    out.mkdir(parents=True, exist_ok=False)
    for name, data in payloads.items():
        (out / name).write_bytes(data)
    (out / "manifest.json").write_bytes(encoded(manifest))
    return manifest


def load_sources(root: Path):
    from poc import strict_json, digest
    data = {}
    for name, (_, expected) in FILES.items():
        data[name] = (root / name).read_bytes()
        if blob_sha(data[name]) != expected:
            raise ContractError(f"upstream file identity changed: {name}")
    corpus = strict_json(data["cases.json"])
    results = strict_json(data["results.json"])
    if digest(data["cases.json"]) != results["fixture_sha256"]:
        raise ContractError("recorded observations bind to a different fixture")
    ids = [c["id"] for c in corpus["cases"]]
    modes = [r["mode"] for r in results["runs"]]
    if len(ids) != 20 or len(set(ids)) != 20 or set(modes) != set(MODES) or len(modes) != 6:
        raise ContractError("unexpected published diagnostic population or mode list")
    return corpus, results, data


def build(corpus: dict, published: dict, run: dict):
    """Reconstruct predictions from ALL scored rows, not precomputed summary counts."""
    mode = run["mode"]
    family = MODES[mode]
    labels = ["not_introduced", "introduced"] if family == "diff" else ["not_present", "present"]
    cases = corpus["cases"]
    by_case = {c["id"]: [] for c in cases}
    threshold = number(run["threshold"], probability=True)
    if threshold != 0.5:
        raise ContractError("the frozen measurement contract uses threshold 0.5")
    for row in run["rows"]:
        if row["case"] not in by_case:
            raise ContractError("source row refers to an unknown case")
        number(row["p"], probability=True)
        by_case[row["case"]].append(row)
    prepared_cases, prepared_rows, flags = [], [], set()
    for c in cases:
        cid = c["id"]
        value = c[f"{family}_relevant"]
        if not isinstance(value, bool):
            raise ContractError("source reference must be an actual boolean")
        rows = by_case[cid]
        # The pinned measurement script enumerates ALL fixture files and accepts
        # only successful scanner exit 0/1. Zero eligible records is a no-match in
        # that recorded experiment, not a rule for arbitrary incomplete captures.
        chosen = any(row["p"] >= threshold for row in rows)
        if chosen:
            flags.add(cid)
        observations = {"eligible_record_count": {"kind": "scalar", "value": len(rows)}}
        if rows:
            observations["maximum_record_signal"] = {"kind": "scalar", "value": max(row["p"] for row in rows)}
        prepared_cases.append({"case_id": cid, "family_id": cid, "partition": "development",
            "model_input": {"before": c["before"], "after": c["after"], "task": corpus[f"{family}_task"]},
            "proposed_expected": class_out(labels[int(value)], labels), "rationale": c["reason"],
            "review": {"state": "upstream_author_fixture", "reviewer": None, "method": None}})
        prepared_rows.append({"case_id": cid, "prediction": {
            "outcome": class_out(labels[int(chosen)], labels), "observations": observations}})
    positive = {c["id"] for c in cases if c[f"{family}_relevant"]}
    actual_sets = {"true_positive_cases": flags & positive,
                   "false_positive_cases": flags - positive,
                   "missed_positive_cases": positive - flags}
    # Published summary sets are an independent cross-check of adapter grouping.
    for key, expected in actual_sets.items():
        recorded = run[key]
        if len(recorded) != len(set(recorded)) or set(recorded) != expected:
            raise ContractError(f"reconstructed rows disagree with the published {key}")
    model = run.get("meter", {}).get("model")
    if not isinstance(model, str) or not model:
        raise ContractError("recorded producing model is unavailable")
    book = {"schema_version": 1, "example": "jgrep", "reference_authority": "upstream_author_fixture",
            "source": {"repository": REPOSITORY, "commit": COMMIT},
            "task": {"kind": "single_label", "labels": labels}, "cases": prepared_cases,
            "configuration": {"task": corpus[f"{family}_task"], "label_field": f"{family}_relevant"}}
    capture = {"schema_version": 1, "example": "jgrep", "origin": "published_replay",
               "source": {"model": model, "requested_model": published["requested_model"],
                          "repository": REPOSITORY, "commit": COMMIT},
               "mode": mode, "threshold": threshold, "rows": run["rows"],
               "limitations": published["limitations"], "diagnostics": run.get("diagnostics", "")}
    tp, fp, fn = map(len, (flags & positive, flags - positive, positive - flags))
    tn = len(cases) - tp - fp - fn
    oracle = {"tp": tp, "fp": fp, "fn": fn, "tn": tn, "total": len(cases),
              "precision": tp / (tp + fp) if tp + fp else None,
              "recall": tp / (tp + fn) if tp + fn else None,
              "correct": tp + tn, "accuracy": (tp + tn) / len(cases),
              "false_positive_ids": sorted(flags - positive), "false_negative_ids": sorted(positive - flags)}
    return book, capture, prepared_rows, oracle


def verify_replay(binary: Path, root: Path, out: Path):
    from poc import write_bundle, native_run, command, read_json, digest, encoded
    corpus, published, evidence = load_sources(root)
    out.mkdir(parents=True, exist_ok=False)
    summary = {"status": "passed", "origin": "published_replay", "live_model_calls": 0,
               "repository": REPOSITORY, "commit": COMMIT, "reference_authority": "upstream_author_fixture",
               "fixture_sha256": digest(evidence["cases.json"]), "results_sha256": digest(evidence["results.json"]),
               "limitations": published["limitations"], "modes": {}, "comparisons": {}}
    for run in published["runs"]:
        mode = run["mode"]
        book, capture, rows, oracle = build(corpus, published, run)
        prepared = out / mode / "prepared"
        write_bundle(book, capture, rows, prepared, "development",
                     {"mode": mode, "aggregation": "any_record_at_or_above", "threshold": run["threshold"]},
                     accept_author_fixture=True,
                     extra_evidence={"upstream-cases.json": evidence["cases.json"],
                                     "upstream-results.json": evidence["results.json"],
                                     "upstream-LICENSE": evidence["LICENSE"],
                                     "upstream-measurement.py": evidence["measurement.py"],
                                     "jgrep-adapter.py": Path(__file__).read_bytes()})
        report, receipt = native_run(binary, prepared, out / mode / "run")
        hard = report["final"]
        if hard["total"] != oracle["total"] or hard["correct"] != oracle["correct"] or hard["abstained"] != 0:
            raise ContractError("native counts disagree with the independently reconstructed published case accounting")
        if abs(hard["accuracy"]["value"] - oracle["accuracy"]) > 1e-12:
            raise ContractError("native accuracy disagrees with reference accounting")
        summary["modes"][mode] = dict(oracle, model=capture["source"]["model"],
            report_sha256=receipt["result_sha256"], report=str(Path(mode)/"run/report.json"))
    for base, candidate in (("diff_lines", "diff_hunks"), ("added_lines", "diff_hunks"),
                            ("function_lines", "functions"), ("function_line_context", "functions")):
        name = f"{base}-vs-{candidate}"
        receipt = command(binary, ["compare", "--baseline", out / base / "run", "--candidate", out / candidate / "run", "--out", out / name])
        document = read_json(Path(receipt["result_path"]))
        if digest(Path(receipt["result_path"]).read_bytes()) != receipt["result_sha256"]:
            raise ContractError("comparison receipt digest mismatch")
        summary["comparisons"][name] = {"recovered": document["transitions"]["recovered"]["count"],
            "regressed": document["transitions"]["regressed"]["count"],
            "changed_final_outcomes": document["transitions"]["changed_final_outcomes"]["count"],
            "comparison_sha256": receipt["result_sha256"]}
    (out / "outcomes.json").write_bytes(encoded(summary))
    return summary
