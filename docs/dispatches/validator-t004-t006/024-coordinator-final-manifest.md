# T004-T006 final omitted-selection manifest 024

Frozen after the owner-authorized narrow completion. Implementation writes are
paused. This manifest changes only `src/model/common.rs` from repair manifest 019
and binds one final focused independent recheck; all other accepted repair evidence
is reused unchanged.

## Frozen identity

| Path | SHA-256 | Change from manifest 019 |
|---|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` | Unchanged |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` | Unchanged authority hash |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` | Unchanged |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` | Unchanged |
| `src/model/common.rs` | `edfb4bdfe763d43318007d21c9afeb2ef60ad30da561ab1bc3e169f1227eabd5` | Omitted-selects-all constructor guard and owning cases |
| `src/model/single_label.rs` | `8195f4555b14618b00605d879c8d09cc9564f387421fc1849a3c21c1f977a266` | Unchanged |
| `src/validation.rs` | `4d257a78924bb134c04ad620a90cfc92b107709ed0a2db424476ff01014dfcbe` | Unchanged |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` | Unchanged |
| `docs/dispatches/validator-t004-t006/023-developer-to-coordinator.response.md` | `82b98c72f39906a8d14fdd109f163960fd8c9c98ab2b807a556758fef3511de6` | Narrow developer evidence |

Baseline repair manifest 019 SHA-256 is
`2ef9dc6c9c7c8d6c0863c956c291ab52d5e3366f6d566f895b54a1044daad7d2`;
focused verdict 020 SHA-256 is
`4a121340e640d12fb0a3c44e7dac6afd82ddbf16af244729c4647ab0d46423c6`.

## Narrow correction and evidence

`Population::new` now rejects with the existing invariant diagnostic when checked
configuration has omitted requested episode IDs and the proposed partition has any
unselected ID. It preserves valid omitted/all, omitted selection over an empty
dataset, explicit empty/select-none, explicit subset, duplicate/overlap rejection,
sorting, digest, counts, description, role, and parent semantics.

The owning `population_alignment` filter now directly proves the formerly accepted
construction fails, plus positive omitted/all and empty-dataset omitted cases. The
filter runs three tests across owning/model/actual-entry modules and passes. The
developer also reports `validate_before_selection` and `dataset_digest_binding`
one pass each; format, 24-test full suite, build, and diff check pass. Coordinator
reruns confirm the three population tests, format, planning verification, and diff
check.

Clippy remains exit 101 solely on the unchanged incomplete-consumer `dead_code`
classification: 107 production and 19 test groups. Numeric/parser/T004/T005 and all
other T006 evidence from manifest 019 and verifier response 020 is unchanged and
must not be broadly rerun. The final verifier reads `/Users/dowwie/.codex/AGENTS.md`
and checks only this guard, its positive/negative cases, unchanged hashes, and the
specified combined gates.

