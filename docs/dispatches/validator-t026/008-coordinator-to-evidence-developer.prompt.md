# Implement T026 output 2: source, comparison and preparation evidence

Role/model: fresh sole developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #188 — Verify Validator artifact integrity](http://localhost:3006/1/cards/188).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t026/008-evidence-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is T026 local output 2 of 2. Read global/repository AGENTS, the Rust
best-practices skill at
`/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`, complete T026, owner
prompt001, full output-1 response005, governing spec/coverage rows and existing
comparison/preparation helpers before editing. Preserve output1 and all accepted
T025 behavior. Do not start independent review, T027 or later work.

## Write boundary and independence

Primary write scope is `tests/conformance.rs`, `tests/cli.rs`, and owning test
sections in `src/artifacts.rs`. Reuse real public API/CLI and existing fixtures;
do not create a generic test framework or duplicate workflows. A smallest
production correction in existing `src/artifacts.rs` or `src/app.rs` remains
allowed only after first reproducing an original T026 integrity/replay/publication
defect and preserving exact evidence. Any change to comparison/evaluation/model,
schema, dependency, public API or contract must stop for owner review.

Derive expected IDs/counts/statuses from fixed synthetic inputs before reading the
result. Never infer expected comparison or preparation values from output under
test. Keep current tolerance, numerical expectations, source semantics and T025
repairs unchanged.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `e9d4036002883fef4d94973d5f4519e5729b67afc26b3854bfbb6fbbae880115` |
| `005-safety-developer-to-coordinator.response.md` | `0f8079ac895875992cc37d30a6da54ed94dee883d1504d4488eb8064bc93bfec` |
| `docs/plans/validator/tasks/T026.json` | `bff0e493602b69d73f05ed0268922a549a804610eab27f0a94c9a4b8669f4975` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `src/app.rs` | `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `tests/conformance.rs` | `b0f8f3817245d6bb45e4721fb44dd728fa0231e9c21306199ae350b1fcc9555f` |
| `tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |
| `src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Exact cases

Add one separately listed/executing public-path test for every case below. Each
must assert every stated subcase, not only a headline value.

- `case_s15`: fixed two-row single-label run using distinct source definitions
  (retained A from source Q1, revised B from source Q2). Assert `mixed_source`,
  source counts one each, correct row `source_id`s, both full origin definitions
  through verified inspection, and both sources/configurations represented in the
  comparison output.
- `case_s16`: separately reject missing and unknown prediction source IDs with the
  precise typed public error/no publication. Compare valid runs that reuse the
  same local source ID string with different source configuration; assert the
  comparison exposes the definition change rather than treating IDs as globally
  identical.
- `case_s18`: change whole golden bytes through dataset/input/reference content,
  prove the digest changes, accepted existing run bytes remain unchanged, default
  comparison rejects incompatible golden populations with precise error and no
  output. Do not reduce this to JSON-key reordering.
- `case_s19`: construct different selected populations. Default comparison must
  reject. Explicit intersection must recompute on the paired subset, disclose
  exact baseline/candidate excluded IDs/counts, and preserve original artifact-wide
  signal applicability.
- `case_s20`: exercise all three empty-intersection signal pairings:
  probability/probability => `no_data`/`no_data`; label/label =>
  `not_applicable`/`not_applicable`; probability/label =>
  `no_data`/`not_applicable`. Both hard sides are `no_data`; values and deltas are
  null. Assert real comparison output/receipts.
- `case_s22`: fixed identical population where the candidate recovers one episode
  and regresses another. Assert both exact ordered ID lists survive even though
  headline accuracy is unchanged; do not infer IDs from result.
- `case_m17`: reference `{A,B}` with the same episode changing prediction `{A}` to
  `{B}`. Assert `neither_correct`, changed-outcome ID and vocabulary-ordered
  per-label transition evidence; exact-set categories must not hide the partial
  change.
- `case_m18`: compare multi-label runs on the same selected population with
  different abstentions. Assert explicit baseline/candidate answered populations,
  overlap and conditional metric availability/changes; do not describe a
  population-conditioned change as paired model improvement.
- `case_m19`: make a multi-label intersection empty. Preserve artifact-wide signal
  applicability: present marginals yield `no_data`, absent marginals
  `not_applicable`, with null values/deltas and hard `no_data`.
- `case_e08`: preserve an original retained categorical observation summing `0.99`,
  an explicitly prepared canonical scoring vector, normalized working values, and
  bound preparation script/raw/receipt evidence as distinct fields/artifacts.
  Strict validation must pass without an in-process normalization fallback or any
  calibration claim. Assert preparation descriptor/index/binding and exact
  original-versus-prepared values through report plus verified inspection.

## Complete T026 and final gates

After the ten new cases pass, run every exact T026 case independently:
S15/S16/S18/S19/S20/S22-S28, M17-M19 and E08 (16 total), plus
`artifact_adversarial_matrix` and `publication_and_privacy_matrix`. Confirm all
literal filter names list and execute one nonzero test. Preserve the accepted
`replay_binding_and_result_tampering` filter.

Then run serially:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Expected inventory is 91 conformance, 11 CLI, 44 library and zero doc tests unless
a necessary owning test changes it and is explained. Clippy may exit 101 only for
the exact accepted 17 production plus three duplicate lib-test staged diagnostics;
no suppression/fake use/visibility expansion. All other commands must pass.

Response008 must map every case/subcase to actual assertions, identify real API
versus process evidence, list every exact filter/gate exit and count, record any
reproduced defect before correction, and provide exact changed/unchanged hashes.
Return only when the combined T026 developer boundary is complete and ready for
one frozen independent review, or with a frozen material scope/product discrepancy.
