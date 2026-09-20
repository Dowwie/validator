# Build independent T025 exhaustive fixtures and oracle filters

Role/model: fresh sole oracle/test developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/002-oracle-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is T025 local output 1 of 3: independently derived fixtures/direct oracles
plus `exhaustive_single_label` and `exhaustive_multi_label`. Later fresh contexts
own the assigned S/E rows and then M rows/full numerical umbrella. Do not begin
those filters, T026 or later work.

Read global/repository AGENTS, complete T025, owner prompt001, acknowledgment001,
ratified numerical/verification sections and assigned coverage. You may read
published schemas/API contracts, `tests/conformance.rs` setup/assertion helpers,
existing independent expected fixtures, and the master-spec F04 source at lines
753-782 solely to reconstruct the adopted fixture. **Do not read or copy
`src/evaluation*`, production metric/count helpers, or any production scoring
implementation to derive expected values. Do not invoke production to generate
expected numbers.** Production is called only after the independent oracle exists,
to obtain actual values for comparison.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t025/001-owner-to-coordinator.prompt.md` | `4c6409c55242078377ff37b87343a41b9b997267d3fbf47dac3d9c889d2f0ab0` |
| `docs/dispatches/validator-t025/001-coordinator-to-owner.response.md` | `8de15bde5bda74bd78250779e07d9c50c3fba87267d4e70664caff2e24197d68` |
| `docs/plans/validator/tasks/T025.json` | `57f4765956789254c170e5fe84c054a7ed8ee95805dd37b3c4d49e3fae3fbb50` |
| `docs/plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `tests/conformance.rs` | `6ad3652d62a447e6a0e32ab3ab69633c2b66041b6e0a3d15a52c76376e9692b6` |
| `tests/fixtures/single-label/expected.json` | `99cb21c88d4d497b8d805537d9e2648304b9aab34ea748ea6752f350641728c5` |
| `tests/fixtures/multi-label/expected.json` | `a65de157415f75567cb907f2dd45ae3d524dcb552dce7c10ef9ca87efbfc63ec` |

Stop and save the exact mismatch before editing. Write only
`tests/conformance.rs` and the existing single-/multi-label expected fixture files.
Preserve every accepted fixture value; extend rather than rewrite history. New
fixture files under these two directories require coordinator indexing, so name
and report them explicitly if truly necessary.

## Independent bounded enumeration

Implement simple direct oracle code in the test boundary, independent of production:

- Single-label vocabulary `[A,B,C]`: enumerate populations of lengths 0, 1 and 2,
  every expected-class combination, every predicted class or typed abstention
  combination, and record-order permutations. Directly construct K×(K+1) counts,
  support/predicted support, D/E/U/G/N, TP/FP/FN, accuracy, wrong/abstention/
  coverage/selective metrics, per-class precision/recall/direct F1/coverage and
  fixed-schema macro zero-fill/status. Prove every accounting identity and the
  distinction between absent and observed-but-always-missed classes.
- Multi-label vocabulary `[A,B,C]`: enumerate all 8 expected subsets × all 8
  answered predicted subsets (64 pairs), plus each of the 8 expected subsets with
  whole-episode abstention. Directly derive N/U/G/D/E, exact match, answered-only
  per-label TP/FP/FN/TN, total versus answered support, every four-count sum G,
  total G×K, micro/macro/Hamming/coverage/selective ratios, matched/missed/extra
  sets or explicit whole-abstained status. Never convert abstention to empty set.
- Independently reconstruct F04 from raw rows
  `A→A,A→A,A→B,A→C,B→B,B→B,C→A,C→C`: accuracy 5/8; class F1
  4/7, 4/5, 1/2; macro-F1 131/210. Preserve the established source/fixture and
  do not import wider master-spec features or superseded conventions.

Use exact integer/rational comparisons where representable. Any floating
comparison uses only the existing ratified fixture tolerance. The oracle may not
call production internally or share production decision/count code. Tests may call
the real public application only after deriving the expected result, then compare
every relevant count/status/population/evidence field.

Implement and run nonzero:

```text
cargo test --locked --test conformance exhaustive_single_label -- --nocapture
cargo test --locked --test conformance exhaustive_multi_label -- --nocapture
```

Also run list confirmation, focused existing F04/accounting regressions, format,
the conformance suite and diff check. A full project gate is deferred to output 3;
do not use this to add assigned row filters early.

Response002 must document the derivation algorithm without production references,
enumeration counts, independent identities/rationals/status rules, F04 reconstruction,
commands/exits/counts and exact changed hashes. If actual public output differs
from the derived oracle or literal wire contract, freeze exact input/expected/
actual evidence and stop; do not tune fixtures/tolerance or edit production/schema.
