# T015-T017 architecture-gate decision handoff

The frozen T015-T017 candidate is ready for the required owner decision.

## Exact decision inputs

| Artifact | SHA-256 |
|---|---|
| Combined candidate manifest013 | `46dac241d71c838ce1251ae9ed7914798c84009eeb937a7577f5c16da735c5c3` |
| Independent verifier verdict014 | `12019ccc02925b73697686257587c7ea99abdf847c72cc44068056bd90e40a82` |
| T017 developer evidence response012 | `72a1d10b3e18bf6b8e644957a820a4686510606a46f41648510621e7e40b7015` |
| Candidate acceptance record | `98dba75ec4834090132908d28d7066d05ec47918bd180853639597f9ad34cc65` |

The verifier independently reconciled every manifest hash before review and
confirmed the candidate remained frozen after its commands. Its locked release
rebuild reproduced the manifest binary hash.

## Verdict and evidence

Verdict014 is **Ready** with no current criterion-backed defect. It independently:

- reviewed contained exact replay, evidence binding, recomputation and the
  path-sensitive exact/tolerant report comparison boundary;
- confirmed explicit inspection preserves opaque values only through the selected
  disclosure operation;
- reviewed the concrete typed comparison inputs/metric pairs, compatibility,
  source differences, checked categories/transitions and shared atomic publisher;
- recalculated the four-row oracle from the frozen fixture rather than production
  output;
- reran the focused T015/T016/T017 evidence and the full format, test, release,
  plan and diff gates;
- observed 40 unit, 6 CLI and 11 conformance tests passing and reproduced the
  release binary hash;
- observed warning-denied Clippy exit 101 for exactly the 24 owner-authorized
  production `dead_code` diagnostics and no other warning class.

Dispatch014 accidentally named `relocated_run_inspection` under the conformance
target, which selected zero tests. The verifier detected the typo, ran the correct
CLI target (1 passed), and also ran the complete CLI/full suites. It correctly
treated this as a dispatch-command correction rather than a candidate defect.

## Decision boundary and tracking

This handoff supports acceptance only of the T015-T017 single-label architecture
gate: verified relocated replay/inspection, identical-population comparison, and
the fixed production-shaped steel thread. It is not warning-free, multi-label,
intersection, release, private practical-acceptance or project-completion evidence.
The T027/T035 clean gates remain mandatory.

All T015-T017 dispatches, handoffs, fixtures, evidence, candidate record, manifest
and verifier verdict are indexed. `verify-plan.rb` and `git diff --check` pass.
Card183's implementation/review step is complete; owner acceptance is its first
incomplete step. T018 remains paused until the owner accepts this exact manifest
and verdict and issues the next bounded sequence.

