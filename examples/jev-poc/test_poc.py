"""Offline contract tests. Synthetic outputs below are not Jev measurements."""
import copy
import json
import math
import os
from pathlib import Path
import tempfile
import shutil
import subprocess
import unittest

from adapters import ContractError, FOREMAN_LABELS, binary, decode, harmonize, number
from casebooks import make_books
from poc import (encoded, episode_id, fixture_capture, prepare, read_json,
                 strict_json, validate_book, validate_model_input)


class AdapterTests(unittest.TestCase):
    def test_reject_boolean_strings_and_nonfinite_numbers(self):
        for bad in (True, False, "0.5", None, float("nan"), float("inf"), -0.01, 1.01):
            with self.subTest(value=bad), self.assertRaises(ContractError):
                number(bad, probability=True)

    def test_binary_threshold_is_inclusive_and_not_native_confidence(self):
        for p, expected in ((0.499, "no"), (0.5, "yes"), (0.501, "yes")):
            row = binary(p, ["no", "yes"], 0.5)
            self.assertEqual(row["outcome"]["label"], expected)
            self.assertAlmostEqual(sum(row["probabilities"]["values"].values()), 1)
            self.assertNotIn("confidence", row)

    def test_sift_preserves_truncation_without_turning_it_negative(self):
        row = decode("sift", {"answers": {"relevant": {"type": "boolean", "probability": .9}}, "truncated": True}, {"kind": "single_label", "labels": ["no", "yes"]})
        self.assertEqual(row["outcome"]["label"], "yes")
        self.assertEqual(row["observations"]["truncated"]["value"], 1)

    def test_sift_rejects_wrong_answer_shape(self):
        with self.assertRaises(ContractError):
            decode("sift", {"answers": {"relevant": {"type": "choice", "probability": .9}}}, {"kind": "single_label", "labels": ["no", "yes"]})

    def test_foreman_complete_marginals_and_empty_set(self):
        task = {"kind": "multi_label", "labels": list(FOREMAN_LABELS)}
        row = decode("foreman", dict.fromkeys(FOREMAN_LABELS, .1), task)
        self.assertEqual(row["outcome"], {"type": "labels", "labels": []})
        self.assertEqual(len(row["probabilities"]["values"]), 10)
        self.assertNotIn("confidence", row)
        with self.assertRaises(ContractError):
            decode("foreman", {FOREMAN_LABELS[0]: .9}, task)

    def test_foreman_per_label_threshold_equality(self):
        task = {"kind": "multi_label", "labels": list(FOREMAN_LABELS)}
        row = decode("foreman", dict.fromkeys(FOREMAN_LABELS, .5), task,
                     {"thresholds": dict.fromkeys(FOREMAN_LABELS, .5)})
        self.assertEqual(row["outcome"]["labels"], list(FOREMAN_LABELS))

    def test_upwork_final_fit_is_not_a_probability(self):
        row = decode("upwork", {"decision": "review", "fit": .61, "lane_confidence": .97}, {"kind": "single_label", "labels": ["apply", "review", "skip"]})
        self.assertEqual(row["outcome"]["label"], "review")
        self.assertNotIn("probabilities", row)
        self.assertNotIn("confidence", row)

    def test_upwork_policy_is_not_approximately_reimplemented(self):
        with self.assertRaises(ContractError):
            decode("upwork", {"decision": "apply", "fit": .64}, {"kind": "single_label", "labels": ["apply", "review", "skip"]}, {"FIT_APPLY": .7})

    def test_tax_minimum_stage_gate_and_native_confidence_distinct(self):
        task = {"kind": "single_label", "labels": ["form-a", "form-b"]}
        row = decode("tax", {"form": "form-a", "formConfidence": .95, "probabilities": {"sub": {"child": .9, "parent": .1}}}, task, {"minimum": .95})
        self.assertEqual(row["outcome"]["label"], "form-a")
        self.assertNotIn("confidence", row)
        self.assertNotIn("probabilities", row)
        self.assertIn("child", row["observations"]["stage_sub"]["values"])
        row = decode("tax", {"form": "form-a", "formConfidence": .949}, task, {"minimum": .95})
        self.assertEqual(row["outcome"]["type"], "abstention")

    def test_multilabel_uses_all_scores_not_top_label(self):
        task = {"kind": "multi_label", "labels": ["a", "b", "c"]}
        row = decode("classifier-dev", {"label": "a", "scores": {"a": .9, "b": .7, "c": .1}, "confidence": .9}, task)
        self.assertEqual(row["outcome"]["labels"], ["a", "b"])
        self.assertNotIn("confidence", row)
        with self.assertRaises(ContractError):
            decode("classifier-dev", {"label": "a", "scores": {"a": .9}}, task)

    def test_rounded_categorical_is_not_silently_normalized(self):
        row = decode("classifier-dev", {"label": "a", "scores": {"a": .50, "b": .49}, "confidence": .8}, {"kind": "single_label", "labels": ["a", "b"]})
        self.assertNotIn("probabilities", row)
        self.assertEqual(row["observations"]["unscored_categorical"]["values"], {"a": .5, "b": .49})

    def test_filing_quarantine_precedes_confidence_gate(self):
        row = decode("filing", {"category": "Invoices", "categoryConfidence": .99, "destinationCategory": "Suspected prompt injection"}, {"kind": "single_label", "labels": ["Invoices", "Reports"]}, {"disposition": True, "minimum": .9})
        self.assertEqual(row["outcome"]["reason"], "security_quarantine")

    def test_filing_missing_probability_preserves_upstream_behavior(self):
        row = decode("filing", {"category": "Invoices", "categoryConfidence": None, "destinationCategory": "Invoices"}, {"kind": "single_label", "labels": ["Invoices", "Reports"]}, {"disposition": True, "minimum": .9})
        self.assertEqual(row["outcome"]["label"], "Invoices")
        self.assertNotIn("confidence", row)

    def test_compaction_lower_threshold_retains_more(self):
        task = {"kind": "single_label", "labels": ["keep", "drop_result", "drop_call"]}
        out = {"keepCall": .4, "keepResult": .4, "action": "drop_call", "reason": "model decision"}
        self.assertEqual(decode("compaction", out, task)["outcome"]["label"], "drop_call")
        self.assertEqual(decode("compaction", out, task, {"threshold": .35})["outcome"]["label"], "keep")
        out["keepCall"], out["keepResult"] = .1, .5
        self.assertEqual(decode("compaction", out, task, {"threshold": .5})["outcome"]["label"], "keep")

    def test_pinned_compaction_is_not_a_model_prediction(self):
        with self.assertRaises(ContractError):
            decode("compaction", {"reason": "pinned_recent", "action": "keep", "keepCall": 1, "keepResult": 1}, {"kind": "single_label", "labels": ["keep", "drop_result", "drop_call"]})

    def test_router_application_fallback_remains_an_answer(self):
        row = decode("router", {"tier": "sonnet", "reason": "jev unavailable", "changed": False}, {"kind": "single_label", "labels": ["haiku", "sonnet", "opus"]})
        self.assertEqual(row["outcome"]["label"], "sonnet")
        self.assertNotIn("confidence", row)

    def test_align_binary_and_multilabel_are_separate(self):
        row = decode("jev-align", {"probability": .5}, {"kind": "single_label", "labels": ["no", "yes"]})
        self.assertEqual(row["outcome"]["label"], "yes")
        row = decode("jev-align", {"label_probabilities": {"a": .7, "b": .8}}, {"kind": "multi_label", "labels": ["a", "b"]})
        self.assertEqual(row["outcome"]["labels"], ["a", "b"])

    def test_partial_signals_retain_population_and_observations(self):
        rows = [binary(.8, ["no", "yes"], .5), {"outcome": {"type": "abstention"}, "observations": {}}]
        self.assertEqual(harmonize(rows), ["probabilities"])
        self.assertEqual(len(rows), 2)
        self.assertEqual(rows[0]["observations"]["partial_probabilities"]["values"]["yes"], .8)
        self.assertNotIn("probabilities", rows[1])

    def test_explicit_upstream_error_is_not_a_negative(self):
        row = decode("sift", {"error": "timeout"}, {"kind": "single_label", "labels": ["no", "yes"]})
        self.assertEqual(row["outcome"]["type"], "abstention")


class PreparationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.books = make_books()

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.book, self.capture = fixture_capture(self.books["sift"])

    def test_draft_counts_and_no_invented_approval(self):
        counts = {"sift": 80, "foreman": 60, "upwork": 90, "tax": 60,
                  "classifier-dev": 120, "classifier-dev-multi": 96, "filing": 80,
                  "compaction": 120, "router": 60, "jev-align": 80}
        for slug, b in self.books.items():
            with self.subTest(example=slug):
                validate_book(b)
                self.assertEqual(len(b["cases"]), counts[slug])
                self.assertTrue(all(c["review"]["state"] == "pending" for c in b["cases"]))

    def test_family_leak_is_rejected(self):
        b = copy.deepcopy(self.book)
        b["cases"][1]["partition"] = "held_out"
        with self.assertRaises(ContractError):
            validate_book(b)

    def test_foreman_rejects_old_guessed_scalar_shapes(self):
        observation = copy.deepcopy(self.books["foreman"]["cases"][0]["model_input"])
        observation["worker_exit_status"] = None
        with self.assertRaises(ContractError):
            validate_model_input("foreman", observation)

    def test_observed_capture_cannot_use_pending_gold(self):
        capture = dict(self.capture, origin="observed")
        with self.assertRaises(ContractError):
            prepare(self.book, capture, self.root / "out")
        self.assertFalse((self.root / "out").exists())
        with self.assertRaises(ContractError):
            prepare(self.book, capture, self.root / "out", fixture=True)

    def test_fixture_origin_requires_explicit_fixture_mode(self):
        with self.assertRaises(ContractError):
            prepare(self.book, self.capture, self.root / "out")

    def test_incomplete_capture_is_not_scored(self):
        capture = dict(self.capture, complete=False)
        with self.assertRaises(ContractError):
            prepare(self.book, capture, self.root / "out", fixture=True)

    def test_missing_and_duplicate_prediction_ids_fail(self):
        for records in (self.capture["records"][:2], self.capture["records"] + [self.capture["records"][0]]):
            with self.subTest(count=len(records)), self.assertRaises(ContractError):
                prepare(self.book, dict(self.capture, records=records), self.root / "out", fixture=True)

    def test_native_identity_mismatch_fails(self):
        capture = copy.deepcopy(self.capture)
        capture["records"][0]["output"]["id"] = "wrong"
        with self.assertRaises(ContractError):
            prepare(self.book, capture, self.root / "out", fixture=True)

    def test_credentials_are_not_archived(self):
        capture = copy.deepcopy(self.capture)
        capture["source"]["apiKey"] = "not-a-real-secret"
        with self.assertRaises(ContractError):
            prepare(self.book, capture, self.root / "out", fixture=True)

    def test_model_metadata_is_not_collapsed_across_fallbacks(self):
        capture = copy.deepcopy(self.capture)
        capture["records"][0]["output"]["model"] = "synthetic-fallback"
        prepare(self.book, capture, self.root / "out", fixture=True)
        artifact = read_json(self.root / "out/predictions.json")
        self.assertEqual({s["model"] for s in artifact["sources"].values()}, {"synthetic-fixture", "synthetic-fallback"})
        self.assertEqual(len({r["source_id"] for r in artifact["predictions"]}), 2)

    def test_conflicting_native_and_recorded_models_fail(self):
        capture = copy.deepcopy(self.capture)
        capture["records"][0]["model"] = "wrong-model"
        with self.assertRaises(ContractError):
            prepare(self.book, capture, self.root / "out", fixture=True)

    def test_threshold_experiment_keeps_identical_golden_bytes(self):
        prepare(self.book, self.capture, self.root / "a", policy={"threshold": .5}, fixture=True)
        prepare(self.book, self.capture, self.root / "b", policy={"threshold": .7}, fixture=True)
        self.assertEqual((self.root / "a/golden.json").read_bytes(), (self.root / "b/golden.json").read_bytes())
        self.assertNotEqual((self.root / "a/predictions.json").read_bytes(), (self.root / "b/predictions.json").read_bytes())

    def test_no_overwrite_and_owner_only_preparation(self):
        prepare(self.book, self.capture, self.root / "out", fixture=True)
        old = (self.root / "out/predictions.json").read_bytes()
        with self.assertRaises(FileExistsError):
            prepare(self.book, self.capture, self.root / "out", fixture=True)
        self.assertEqual(old, (self.root / "out/predictions.json").read_bytes())
        if os.name == "posix":
            self.assertEqual((self.root / "out").stat().st_mode & 0o777, 0o700)
            self.assertEqual((self.root / "out/golden.json").stat().st_mode & 0o777, 0o600)

    def test_request_projection_cannot_contain_review_metadata(self):
        b = copy.deepcopy(self.book)
        b["cases"][0]["model_input"]["proposed_expected"] = "yes"
        with self.assertRaises(ContractError):
            validate_book(b)

    def test_id_framing_is_stable_and_unambiguous(self):
        self.assertEqual(episode_id("x", "y"), episode_id("x", "y"))
        self.assertNotEqual(episode_id("a:b", "c"), episode_id("a", "b:c"))

    def test_strict_json_rejects_duplicate_keys_and_nan(self):
        for raw in (b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":Infinity}'):
            with self.subTest(raw=raw), self.assertRaises(ContractError):
                strict_json(raw)

    def test_native_gold_and_prediction_schemas(self):
        try:
            import jsonschema
        except ImportError:
            self.skipTest("optional Python JSON Schema checker unavailable; native check remains mandatory")
        root = Path(__file__).resolve().parents[2]
        for slug, b in self.books.items():
            with self.subTest(example=slug):
                b, capture = fixture_capture(b)
                path = self.root / slug
                prepare(b, capture, path, fixture=True)
                for name, schema in (("golden", "golden"), ("predictions", "predictions"), ("config", "config")):
                    jsonschema.Draft202012Validator(read_json(root / f"schemas/v2/{schema}.schema.json")).validate(read_json(path / f"{name}.json"))


class CaptureSafetyTests(unittest.TestCase):
    @unittest.skipUnless(shutil.which("node"), "Node is optional outside the Sift capture example")
    def test_sift_dry_run_has_no_model_calls_or_reference_fields(self):
        with tempfile.TemporaryDirectory() as tmp:
            book = Path(tmp) / "book.json"
            book.write_bytes(encoded(make_books()["sift"]))
            run = subprocess.run(["node", str(Path(__file__).with_name("capture_sift.mjs")), "--book", str(book)], capture_output=True, text=True, timeout=10)
            self.assertEqual(run.returncode, 0, run.stderr)
            result = json.loads(run.stdout)
            self.assertEqual(result["model_calls"], 0)
            self.assertEqual(result["selected"], 5)
            self.assertNotIn("proposed_expected", run.stdout)
            self.assertNotIn("rationale", run.stdout)

    @unittest.skipUnless(shutil.which("node"), "Node is optional outside the Sift capture example")
    def test_pending_gold_blocks_sift_before_live_import_or_output(self):
        with tempfile.TemporaryDirectory() as tmp:
            book, output = Path(tmp) / "book.json", Path(tmp) / "capture.json"
            book.write_bytes(encoded(make_books()["sift"]))
            run = subprocess.run(["node", str(Path(__file__).with_name("capture_sift.mjs")), "--book", str(book), "--execute", "--out", str(output), "--upstream", "/does-not-exist"], capture_output=True, text=True, timeout=10)
            self.assertNotEqual(run.returncode, 0)
            self.assertIn("Human reference approval", run.stderr)
            self.assertFalse(output.exists())


class JgrepTests(unittest.TestCase):
    def test_aggregation_is_case_level_and_task_specific(self):
        from jgrep import build
        corpus = {"diff_task": "introduced error", "function_task": "existing error", "cases": [
            {"id": "a", "before": "old", "after": "new", "diff_relevant": True, "function_relevant": True, "reason": "introduced"},
            {"id": "b", "before": "old", "after": "new", "diff_relevant": False, "function_relevant": True, "reason": "preexisting"}]}
        run = {"mode": "diff_lines", "threshold": .5, "rows": [
            {"case": "a", "p": .2}, {"case": "a", "p": .5}, {"case": "b", "p": .1}],
            "true_positive_cases": ["a"], "false_positive_cases": [], "missed_positive_cases": [],
            "meter": {"model": "synthetic-fixture"}}
        published = {"requested_model": "synthetic-fixture", "limitations": ["synthetic adapter test"]}
        book, _, rows, oracle = build(corpus, published, run)
        self.assertEqual(len(rows), 2)
        self.assertEqual(oracle["tp"], 1)
        self.assertEqual(oracle["tn"], 1)
        self.assertNotIn("probabilities", rows[0]["prediction"])
        self.assertEqual(book["cases"][1]["proposed_expected"]["label"], "not_introduced")
        run["mode"] = "function_lines"; run["missed_positive_cases"] = ["b"]
        book, _, _, oracle = build(corpus, published, run)
        self.assertEqual(book["cases"][1]["proposed_expected"]["label"], "present")
        self.assertEqual(oracle["fn"], 1)

    def test_published_summary_mismatch_is_not_silently_trusted(self):
        from jgrep import build
        corpus = {"diff_task": "task", "cases": [{"id": "a", "before": "x", "after": "y", "diff_relevant": True, "reason": "yes"}]}
        run = {"mode": "diff_lines", "threshold": .5, "rows": [{"case": "a", "p": .9}],
               "true_positive_cases": [], "false_positive_cases": [], "missed_positive_cases": ["a"], "meter": {"model": "fixture"}}
        with self.assertRaises(ContractError):
            build(corpus, {"requested_model": "fixture", "limitations": []}, run)


if __name__ == "__main__":
    unittest.main()
