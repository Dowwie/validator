# T018-T020 coordinator decision handoff

Status: the repaired T018-T020 checkpoint is ready for owner acceptance. T021 has
not started. This handoff does not itself accept, release or complete the product.

## Frozen decision inputs

| Artifact | SHA-256 |
|---|---|
| original candidate manifest022 | `918fd6e13608940232de86f3bfb4b4dfba81c7d5cf618657225cef260e05c177` |
| original Revise verdict023 | `41ef819eeb12042ebebd2e51f86a52589150b95354fb1d990a575bd12fbf34e8` |
| sole repair response024 | `6217ad65a6b7f01ce878e52831f2ecf78fb11a82e84370c08d4cca2bba8bbb48` |
| repaired candidate manifest025 | `e26438bd371f261e5d91a05d55312bfa1d329db75dc4e24e761d75d6bd646f11` |
| independent Ready verdict026 | `7e3885c40aced13a3822d1b3d3564c292035821c0902da3970923ee68f84cae2` |

The same independent Sol-high verifier reconciled all 51 bound repaired-candidate
files exactly. Only `src/validation.rs`, `schemas/v2/report.schema.json` and
`tests/conformance.rs` changed in the single repair cycle.

## Accepted-by-verifier behavior

- T018 supplies concrete checked multi-label types, explicit closed task/target/
  outcome/probability admission and shared exact digest, UUID, source/evidence,
  observation, selection and sorted-alignment behavior. Empty answered sets,
  abstentions, missing rows and missing marginal keys remain distinct; complete
  finite `[0,1]` marginals remain unchanged and unnormalized.
- T019 supplies borrowed concrete hard evaluation, exact `N/U/G/D/E` accounting,
  answered-only per-label `TP/FP/FN/TN`, micro/macro/Hamming/set metrics with exact
  populations/statuses, and ordered episode set-difference evidence. The
  hand-authored two-row/empty/abstention/equal-count oracle is independently
  recalculated and discriminating.
- T020 supplies marginal log/Brier losses and per-label bins over all selected
  `N*K` decisions, including abstentions, explicit infinity endpoints and the
  `.265`, `.04` versus `.08`, and `0/.1/1` boundary oracles. Shared check/evaluate/
  publish/rebuild/inspect handles both task kinds without multi-label comparison or
  threshold execution. Strict check/report/inspection schemas validate coherent
  concrete task families. Relocated inspection uses contained verified evidence,
  preserves exact opaque payload only through inspect and rejects tampering.
- The independent review's two manifestations of one source-kind defect are
  closed. Every declared multi-label source must be a classifier, including unused
  sources; an unused scored-choice source now returns `E_CONFIG`. The report schema
  rejects that same foreign source while valid unused classifiers and legal
  single-label scored-choice sources remain supported.

## Gate evidence and limits

All six exact T018-T020 filters select one and pass. Full locked tests pass with
44 library, 7 CLI, 15 conformance and 0 doc failures. Formatting, release build,
plan/link/index validation and diff checks pass. The independent repaired release
binary SHA-256 is
`420ac50f1759c423b40131913f837f11fa4d8768b23f46ce3c90623f359d7da4`.

Warning-denied Clippy intentionally exits 101 only for the exact 20 owner-authorized
production `dead_code` diagnostics in unchanged private task wrappers, common
accessors, single-label signal helper and strict/later-policy wire fields. No
ordinary warning, suppression, fake consumer, visibility expansion or tolerance
change remains. T027 and T035 retain their warning-free gates.

## Tracking and next boundary

Card184's T020 implementation/local-verification step is complete. The card is
Waiting for owner acceptance with its independent-review step still open. The
artifact index includes every owner/coordinator/developer/verifier prompt,
response, manifest and the independent multi-label oracle. `verify-plan.rb` and
`git diff --check` pass.

Requested owner decision: accept repaired manifest025 under Ready verdict026 and
authorize the next bounded backbone work already defined by the ratified plan.
Until that decision, T021 and later work remain paused and the task-owned
caffeinate hold remains active.
