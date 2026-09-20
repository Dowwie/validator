# T007-T014 repaired frozen candidate manifest

This manifest supersedes manifest033 only for the five product/test paths below.
All other 43 product, governing, and evidence inputs in
`033-coordinator-final-manifest.md` still match their declared hashes; coordinator
comparison found no unexpected change. Implementation writers are stopped during
the focused recheck.

| Binding | SHA-256 |
|---|---|
| `033-coordinator-final-manifest.md` | `49574acc96a5b8520daf3dd68fabe56ddfd08ff21f5bfb566a282a57c9475112` |
| `034-verifier-to-coordinator.response.md` | `bb1fc45987ded8236ef9a7e244dfc2f8ef0782d826a32bab7832cea0c732236d` |
| `035-t014-completion-developer-to-coordinator.response.md` | `65c347cac6d67ffc400347877f0f652c3a0f2f4eb2b63e6827fa2f2f7899d7ab` |

## Repaired paths

| Path | SHA-256 | Repair |
|---|---|---|
| `src/app.rs` | `819627406e44e184df13d8baf4c1feb115ea0444cf9b7518f83afd2b9e02b875` | Check consumes shared admitted-row normalization diagnostics. |
| `src/model/single_label.rs` | `899963586c722a37af0ce0dd8f28d03b86423c8c9bbce431b14280ae8a6cee98` | Shared non-scoring helper used by check and report; duplicate report derivation removed. |
| `schemas/v2/report.schema.json` | `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46` | Single-label report vocabulary minimum raised from one to two. |
| `tests/cli.rs` | `3b0cc62faedcb00e46e76a407b3bdf384da90b4596d593a4bf53da46273a567e` | Actual check regression for one normalized row and independent nonzero sum error. |
| `tests/conformance.rs` | `982f128ba6a1b66e3071113b3c05a7fd37ef7269d20a7fa8f34644bb596075a2` | Coherent two-label positive report and discriminating one-label rejection. |

## Repair evidence

Developer response035 records:

- the three affected named filters each pass one test;
- full locked tests pass 39 library + 3 CLI + 8 conformance + 0 docs;
- format, release binary build and diff check pass;
- exact warning-denied Clippy still exits 101 only for the same 26 unique
  owner-staged production `dead_code` diagnostics, with no new/ordinary lint;
- release `check` on `[0.7,0.2,0.1000000005]` returns
  `normalized_count: 1` and `maximum_sum_error: 4.999998193255806e-10`;
- the coherent two-label report validates and its one-label mutation fails.

This checkpoint remains not warning-free under owner decision028. T027/T035 retain
the mandatory full clean gate.
