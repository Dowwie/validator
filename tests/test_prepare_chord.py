"""Public regression checks for the frozen Chord preparation rules."""

import importlib.util
import unittest
from pathlib import Path


SCRIPT = Path(__file__).parents[1] / "scripts" / "acceptance" / "prepare_chord.py"
SPEC = importlib.util.spec_from_file_location("prepare_chord", SCRIPT)
assert SPEC and SPEC.loader
PREPARE_CHORD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PREPARE_CHORD)


class ScoreRoutingTests(unittest.TestCase):
    def test_scalar_boundaries_keep_documented_ties(self) -> None:
        self.assertEqual(PREPARE_CHORD.route_score(0.49), "NO_MATCH")
        self.assertEqual(PREPARE_CHORD.route_score(0.5), "UNCERTAIN")
        self.assertEqual(PREPARE_CHORD.route_score(1.5), "MATCH")

    def test_choice_labels_use_the_explicit_canonical_mapping(self) -> None:
        self.assertEqual(PREPARE_CHORD.canonical_label("DIFFERENT_CANONICAL"), "NO_MATCH")
        self.assertEqual(PREPARE_CHORD.canonical_label("UNCERTAIN"), "UNCERTAIN")
        self.assertEqual(PREPARE_CHORD.canonical_label("SAME_CANONICAL"), "MATCH")


if __name__ == "__main__":
    unittest.main()
