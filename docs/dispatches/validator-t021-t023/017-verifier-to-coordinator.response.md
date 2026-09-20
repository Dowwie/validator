# Superseding T021-T023 repair disposition

Verdict: **Ready**.

This disposition supersedes only verdict014's evidence-placement conclusion. It
does not replace or alter its inspection, commands, candidate reconciliation, or
probe results. The repaired production/schema behavior resolves the sole finding
from verdict009, and the governing owner clarification establishes that the
existing conformance, CLI and independent binary evidence proves the four repaired
cases collectively. No current-contract defect remains.

## Governing clarification and preserved record

The evidence-placement chain reconciles exactly:

| Artifact | SHA-256 |
|---|---|
| `015-owner-to-coordinator.prompt.md` | `a1aa263f239088f96b83d92ad607b609f5ef066574c43a08536a4132d997403d` |
| `015-coordinator-to-owner.response.md` | `3ffdbac0e5199ebe1dc33fa66a47bef218d3fd91b696b34746fdac0929015864` |
| `016-coordinator-to-verifier.prompt.md` | `eb6e4508ac806149830f257319392cb7c2d0f4f23432e57d3c882bdf7a33c75e` |

The pre-clarification verdict remains preserved at
`014-verifier-to-coordinator.response.md`, SHA-256
`7c2d6598b135eda14cc8b2e30bdf39ed310c41fd4d02c35a79e404aadf5439a2`.
It accurately records the candidate inspection and results available before owner
prompt015 was delivered. Its sole Revise reason was duplicate placement of typed
result/status assertions in the CLI test body; it reported no production, schema,
oracle, numerical or gate failure.

Owner prompt015 governs that placement question: T023 assigns concrete metric
recomputation to conformance and flag routing/receipts/exclusions to CLI evidence.
The required facts need not be asserted identically in both automated test bodies
when the combined evidence and independent binary probe prove them.

## Exact collective evidence placement

| Evidence | Facts proved |
|---|---|
| `tests/conformance.rs` / `intersection_recomputation` | Both concrete application branches; explicit intersection for equal nonempty and equal empty selections in both task families; exact compared populations and zero exclusions; schema validity; exact receipt digest; concrete recomputed typed hard values/statuses and signal-family statuses. |
| `tests/cli.rs` / `cli_intersection` | Actual binary flag routing; requested `scope: "intersection"`; exact compared IDs/counts and zero exclusions for all four cases; schema-valid published documents; exact receipt digest; preserved unequal intersection, default mismatch rejection, immutable-output refusal and duplicate/unknown/value-taking flag errors. |
| Independent actual-binary probe from verdict014 | All of those facts together through the real binary for single-label/multi-label × equal nonempty/equal empty, including typed values/statuses that the checked-in CLI test does not assert. |

The CLI test does **not** assert typed metric values/statuses, and this disposition
does not claim that it does. Its ownership is routing, scope, population,
exclusions, schema and receipt. The conformance test asserts the typed
recomputation, and the independent binary probe closes the real-process connection.

The independently executed binary probe observed:

- single-label equal nonempty: requested and emitted intersection, one compared
  ID, zero exclusions, defined raw hard metrics (baseline accuracy `1.0`,
  candidate accuracy `0.0` for the probe's deliberately differing outcomes),
  valid schema and matching receipt digest;
- multi-label equal nonempty: the same structural result with defined raw
  exact-match accuracy (`1.0` baseline, `0.0` candidate);
- equal empty for both tasks: intersection scope, zero compared IDs, zero
  exclusions, hard status `no_data`, absent probability/marginal family status
  `not_applicable`, valid schema and matching receipt digest.

The checked-in conformance cases independently use equal correct nonempty runs and
assert `1.0` raw/final accuracy or exact-match accuracy, plus the same empty
`no_data`/`not_applicable` statuses. Together these cases prove recomputation rather
than merely checking document shape.

## Repaired finding disposition

- `src/app.rs` dispatches on `options.intersection` alone in both task branches,
  then performs compatibility, `intersection_scope`, checked validated
  restriction, concrete evaluator recomputation and typed comparison assembly.
- Default comparison remains on identical scope; unequal selections without the
  option still reject.
- The comparison schema admits legal zero-exclusion intersection populations via
  the repaired `anyOf` plus required scope conditionals. Identical scope retains
  exact empty exclusion lists and zero counts. Required fields,
  `additionalProperties: false`, and task-family discrimination remain strict.
- Equal nonempty and equal empty selections in both task families emit the
  requested intersection scope, exact population/counts, zero exclusions and
  correct recomputed typed results/statuses.
- Unequal intersection, empty availability, immutable publication, flag errors,
  and accepted T021/T022 behavior remain preserved by affected checks and frozen
  hashes.

## Reused reconciliation and gates

Verdict014 already reconciled repaired manifest013 at SHA-256
`2cd39b7c27d9b0408ff0903441f3afa975616edf3644f3e502f560d6d26f0bc7`,
its complete repair chain, all four repaired file hashes, and every inherited
manifest008 hash. That evidence is reused unchanged as directed.

The settled focused commands are also reused without rerun:

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance intersection_recomputation -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test conformance empty_intersection_availability -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test cli cli_intersection -- --nocapture` | 0 | 1 passed, 8 filtered |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | exactly the authorized 17 production dead-code diagnostics plus 3 duplicate lib-test diagnostics; no repair-owned warning |
| `git diff --check` | 0 | passed |

The repair developer's unchanged full-suite (44 library, 9 CLI, 21 conformance),
format and locked release-build evidence remains applicable. No test, behavior,
oracle, schema rule, tolerance or mandatory gate is waived.

No candidate file, test, schema, governance, plan, index, session note, acceptance
record or Fizzy state was changed for this superseding disposition. This required
response is the only repository artifact written; verdict014 remains intact.
