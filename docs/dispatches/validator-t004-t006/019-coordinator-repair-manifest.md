# T004-T006 completed repair manifest 019

This manifest freezes the completed original repair after owner reassessment and a
fresh sole developer. It supersedes candidate manifest 006 and the partial states
recorded in responses 010/011 and escalation 012. Implementation writes are paused
for one focused independent recheck. The prior repair allowance is not reset.

## Frozen repaired hashes

| Path | SHA-256 | Status |
|---|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` | serde_json `arbitrary_precision`, `raw_value`, and `std` boundary |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Resolved package set unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` | Unchanged |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` | Unchanged |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` | Unchanged |
| `src/model/common.rs` | `7892d11b1d16ef6a548ae20eac60177d2d82a80f43d2426ed8b83196c98b5859` | Checked EvaluationConfig and complete Population invariants |
| `src/model/single_label.rs` | `8195f4555b14618b00605d879c8d09cc9564f387421fc1849a3c21c1f977a266` | Fallible aligned-row/evaluation construction |
| `src/validation.rs` | `4d257a78924bb134c04ad620a90cfc92b107709ed0a2db424476ff01014dfcbe` | Ordered checked config/admission and repair regressions |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` | Strict numeric field DTOs and RawValue-backed JsonNumber |
| `docs/dispatches/validator-t004-t006/014-developer-to-coordinator.response.md` | `ed424418660c82b15331e492f8da8e51eeccdfb290512fb88ac89014275dc0af` | Replacement developer evidence |

The repair began from manifest 006 SHA-256
`0aab3ef37c8635485c4e5c40ebbaa3dedcb03be374fcf9d44e02187f6495ae3d`
and verifier Revise response SHA-256
`7b6cfe73ff1c361130cc8b5cd2bd184fc7d6ffe5bbf5188d950950d7155ecb60`.

## Numeric boundary repair

The recursive duplicate scan retains legal arbitrary-precision JSON numbers.
Typed observation, categorical-probability, and reject-below numeric objects use
strict field DTOs so original numeric tokens reach RawValue-backed `JsonNumber`
without Serde internally tagged content buffering. Closed tags and unknown-field
rejection remain strict.

`large_number_boundary` proves opaque `1e400` survives golden input, source
configuration, and nested preparation configuration, while typed scalar `1e400`
returns `E_OBSERVATION`. Reject-below `minimum: 1e400` decodes as a legal wire DTO
and is rejected as unsupported current policy with `E_CONFIG`. The private-number
marker object remains `E_SCHEMA` in numeric positions and unchanged in opaque
positions. Typed-null, exact large-integer, duplicate, legal-shape, and original-
byte evidence remains passing.

## Checked T006 boundary repair

Checked `EvaluationConfig<SingleLabelPolicy>` owns nonblank description, role,
optional checked parent, omitted-versus-explicit selection, unique sorted checked
IDs, and the admitted as-recorded policy. It is constructed before selection.

`Population` owns the supplied checked dataset digest, the checked configuration,
derived exact dataset/selected counts, and a complete sorted unique disjoint
selected/unselected partition. Fallible `AlignedRow` construction checks episode
identity and vocabulary membership. Fallible `SingleLabelEvaluation` construction
checks row count/order/identity, target vocabulary, and prediction source presence
against the owned population/vocabulary/source map.

The `population_alignment` filter runs three tests: actual-entry/accessor coverage,
shared population invariants, and owning-module rejection of arbitrary mismatched
rows. It covers digest, counts, description, role, parent, omitted/all, explicit
empty, subset, selected/unselected ordering, and invalid construction.

## Evidence and remaining limit

All 14 requested filters pass with nonzero counts: 13 run one test and
`population_alignment` runs three. The library inventory contains 24 tests.
Coordinator reruns independently confirmed `large_number_boundary` one pass,
`population_alignment` three passes, the 24-test inventory, format, planning
verification, and diff check.

`cargo fmt --all -- --check`, full all-feature locked tests, locked binary build,
and `git diff --check` pass. Clippy does not pass: it exits 101 solely on
incomplete-consumer `dead_code`, with 107 production and 19 library-test groups.
No unused import or other warning class, suppression, placeholder, fake caller,
fallback, public API expansion, or T007-or-later behavior is reported. Clean full
Clippy remains mandatory at T014 and T017.

The independent recheck must read `/Users/dowwie/.codex/AGENTS.md` as governing
context in addition to repository instructions. It is limited to the two original
Revise findings, their justified preservation regressions, frozen hash identity,
and the disclosed Clippy classification; unchanged Ready evidence from response
007 is reusable.

