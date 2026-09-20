# Repair six bounded T026 exact-filter proof gaps

Role/model: retained sole T026 test developer, `gpt-5.6-terra`, high reasoning,
original fresh `fork_turns: none` context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #188 — Verify Validator artifact integrity](http://localhost:3006/1/cards/188).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t026/011-evidence-developer-to-coordinator.response.md`.

Save the full substantive handoff before returning only its path, SHA-256 and terse
status. This is the one finding-driven repair cycle authorized by owner001. The
T028 source writer has saved its atomic checkpoint and yielded; you are the sole
writer. Do not edit governance, plans, index, acceptance/session records or Fizzy.

Read complete verifier verdict010 before editing. Write only
`tests/conformance.rs` unless a demonstrated public contract mismatch forces a
stop for owner scope review. Production, schemas, fixtures, tolerances and every
settled T026/T025 assertion are read-only. Do not start T027 or T028 review.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `009-coordinator-candidate-manifest.md` | `fcb905dcee73958340458b88433e5bd43cc2cc78cfadd37470a19ed50a1efb9f` |
| `010-verifier-to-coordinator.response.md` | `06ded06b8faacd28c19ef80d32288d312cc15563ef83a36140763b5ad71e0b45` |
| `src/app.rs` | `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `tests/conformance.rs` | `87230d024ab18e0b3c1b61a82dec7414593e782f8c769d43ef6839fee55f0812` |
| `tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |

## Exact corrections

1. **S15 full origins.** Inspect both fixed Q1 and Q2 episodes. Assert each
   inspection's available source/origin information and correct row binding. In
   the real self-comparison, assert complete Q1/Q2 source definitions including
   model and configuration on both baseline and candidate sides, plus existing
   source counts. If the public inspection contract cannot expose the origin
   required by the ratified row, freeze exact input/expected/actual evidence and
   stop rather than weakening the case.
2. **S24 relocated comparison.** While every original input/config/evidence file
   remains absent, compare the relocated verified run through the public API,
   assert success and actual `comparison.json` publication/receipt. Retain the
   verified inspection assertion and no-original-lookup proof.
3. **S26 evaluation receipt.** Retain a real evaluation receipt for the scenario
   and assert its absolute `result_path` is the actual `report.json`; independently
   hash exact report bytes and compare to `result_sha256`. Preserve the existing
   independent comparison-receipt proof. Reuse helpers cleanly without removing
   the accepted replay filter.
4. **S27 error privacy.** Trigger one real public failure using the sentinel-bound
   input/evidence, assert the precise typed error, serialize the machine error
   result and prove neither sentinel appears. Retain report privacy and exact
   successful explicit-inspection assertions. Do not substitute a handwritten
   error object.
5. **M17 vocabulary order.** Assert the exact per-label sequence `[A,B]` before
   checking A present-to-absent and B absent-to-present transition cells.
6. **M18 conditional metrics.** From the fixed baseline/candidate inputs, derive
   expected conditional metric availability, values and delta before reading the
   comparison. Assert them plus the existing answered populations/overlap, and
   assert the relevant machine result contains no overall winner/improvement
   representation. Do not infer expected numbers from output.

These are exact-filter proof corrections. Preserve every broader matrix, case and
gate already accepted by verdict010. If stricter assertions reproduce a product
or public-contract mismatch, freeze it and stop; do not silently copy current
output into expectations or edit production.

## Checks

Run each affected filter independently, the two matrices and justified adjacent
regressions, then list/full conformance and the current complete suite:

```text
cargo test --locked --test conformance case_s15 -- --nocapture
cargo test --locked --test conformance case_s24 -- --nocapture
cargo test --locked --test conformance case_s26 -- --nocapture
cargo test --locked --test conformance case_s27 -- --nocapture
cargo test --locked --test conformance case_m17 -- --nocapture
cargo test --locked --test conformance case_m18 -- --nocapture
cargo test --locked --test conformance artifact_adversarial_matrix -- --nocapture
cargo test --locked --test cli publication_and_privacy_matrix -- --nocapture
cargo test --locked --test conformance -- --list
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo fmt --all -- --check
git diff --check
```

Conformance must remain 91 and CLI 11. Full tests must pass. Clippy may contain
only the unchanged accepted 17+3 staged diagnostics. Reuse unchanged release
evidence; no release rebuild by habit.

Response011 must map each verifier finding to the discriminating assertion, list
commands/exits/counts and exact changed/unchanged hashes. Return only when all six
proofs are complete for the same verifier's focused recheck or a concrete public
contract mismatch is frozen.
