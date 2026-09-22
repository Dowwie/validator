"""Public regression checks for resolver250 preparation semantics."""

import importlib.util
import unittest
import uuid
from pathlib import Path


SCRIPT = Path(__file__).parents[1] / "scripts" / "acceptance" / "prepare_resolver.py"
SPEC = importlib.util.spec_from_file_location("prepare_resolver", SCRIPT)
assert SPEC and SPEC.loader
PREPARE_RESOLVER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PREPARE_RESOLVER)


class ResolverPreparationTests(unittest.TestCase):
    def test_episode_id_is_stable_uuid7(self) -> None:
        identity = {
            "phrase": "example phrase",
            "quote": "example source quote",
            "label": "historical-label",
            "baseline_top1": "baseline-label",
            "label_in_shortlist": True,
        }
        first = PREPARE_RESOLVER.episode_id(3, identity)

        self.assertEqual(first, PREPARE_RESOLVER.episode_id(3, identity))
        self.assertEqual(uuid.UUID(first).version, 7)

    def test_judge_outcome_preserves_residue_and_unreadable_distinction(self) -> None:
        residue = PREPARE_RESOLVER.judge_outcome({"judge": None, "unparseable": False})
        unreadable = PREPARE_RESOLVER.judge_outcome({"judge": None, "unparseable": True})

        self.assertEqual(residue, {"label": "RESIDUE", "type": "class"})
        self.assertEqual(
            unreadable,
            {"reason": "unreadable_source_response", "type": "abstention"},
        )


if __name__ == "__main__":
    unittest.main()
