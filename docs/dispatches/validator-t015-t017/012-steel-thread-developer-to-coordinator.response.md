# T017 steel-thread developer handoff

## Status and scope

Candidate evidence is complete and pending the required independent verifier and
owner acceptance. No production source, schema, Cargo input, tolerance,
governance, dispatch, plan, artifact-index, session-note, or Fizzy file changed.
No T018, multi-label, intersection, winner/significance, broad acceptance, or
release work was performed.

All governing and frozen starting hashes in dispatch012 matched before editing.
The final frozen production and schema hashes still match dispatch012 exactly.
The only permanent implementation artifacts are the seven fixed fixture files,
the `steel_thread_end_to_end` test and its bounded helpers in `tests/cli.rs`, and
`docs/acceptance/validator-v1.md`.

## Fixed fixture and independent oracle

The fixture fixes the requested UUID-sorted four rows, vocabulary `[A,B,C]`,
references `[A,A,B,C]`, as-recorded policy, baseline outcomes
`[A,abstention,A,C]`, and candidate outcomes `[B,A,B,C]`. It binds source-a
through `source-a/evidence.bin` and source-b through the genuine parent-relative
`../source-b/evidence.bin`. Both evidence files have the same basename and
different bytes. Baseline episode 2 retains the categorical observation
`{x:0.5,y:0.49}` and opaque input integer `9007199254740993`.

`expected.json` is a hand-authored oracle, never generated or modified by
production scoring. It records:

- baseline `D=2`, `E=1`, `U=1`; accuracy `2/4=1/2`; wrong-class and abstention
  rates `1/4`; coverage `3/4`; selective accuracy `2/3`; selective risk `1/3`;
  class coverages `[1/2,1,1]`; macro-F1 `1/2`;
- Brier `3/10=0.30`; log loss
  `(-2*ln(0.7)-ln(0.3)-ln(0.8))/4 = 0.53511656087940263`; argmax accuracy `3/4`;
- all IDs in categorical probability-bin accounting; confidence IDs 1, 3 and 4
  included and abstained ID 2 excluded; and
- recovered IDs 2 and 3, regressed ID 1, both-correct ID 4, and no
  neither-correct ID.

## Criterion evidence

`tests/cli.rs:steel_thread_end_to_end` is a real `CARGO_BIN_EXE_validator`
process scenario. It uses published output schemas, not test-only application
APIs or a second evaluator.

| Dispatch condition | Exact assertion in `steel_thread_end_to_end` |
| --- | --- |
| Admission and check output | `check` exits 0, validates the check schema, returns all sorted IDs, two sources, and `both` signals. |
| Immutable five-entry evaluations | Baseline and candidate `evaluate` commands exit 0; receipts and reports validate; each receipt's SHA-256 is recomputed from exact report bytes; each report has three snapshots and two evidence entries. |
| Evidence binding | Stored `evidence/0.bin` and `evidence/1.bin` equal the independently read source-a/source-b fixture bytes and are unequal. |
| Relocation/replay | Both runs are renamed, then the full original input tree and parent-relative evidence source are deleted; `inspect` and `compare` still succeed only from stored artifacts. |
| Inspection and opaque payload | Inspection schema validates; episode 2 output contains the exact integer spelling and observation `x=0.5`, `y=0.49`. |
| Comparison and receipt | Comparison and receipt schemas validate; SHA-256 of exact `comparison.json` matches the receipt; all selected IDs are preserved; `winner` and `significance` are absent. |
| Hand oracle | The process test asserts every listed count/fraction/finite value within the existing fixture tolerance, class coverages, bin ID sets, and transition ID sets. |
| Privacy | Check, evaluation receipts/reports, comparison receipt/report, and failure output omit the opaque integer and both synthetic evidence contents. |
| Duplicate key | A duplicate serialized prediction `schema_version` exits 2 with `E_SCHEMA` at `schema`; no output tree exists. |
| Scoring/observation separation | Submitted `0.99` categorical probability exits 2 with `E_PROBABILITY` at `validation`; no output tree exists. The baseline's retained `0.99` categorical observation succeeds. |
| Stored evidence tamper | After original removal, changed stored evidence causes compare to exit 2 with `E_PROVENANCE` at `replay`; no comparison output exists. |
| Evaluation no-overwrite | Re-evaluating to the existing baseline directory exits 3 with `E_OUTPUT_EXISTS` at `filesystem`; the tree snapshot is byte-identical and no temporary sibling remains. |
| Comparison no-overwrite | Re-comparing to the existing comparison directory gives the same exit/code/stage, unchanged tree snapshot, and no temporary sibling. |

## Candidate identity

| Artifact | SHA-256 |
| --- | --- |
| `tests/fixtures/steel-thread/golden.json` | `eacd3362fc6bf571ed9c3547cefbf9190ad79adcc198b047e6c03fa6387387ce` |
| `tests/fixtures/steel-thread/baseline.json` | `e7fa154c47520c4324a3a932ee04904eff26f04058077257498dac37f2679f53` |
| `tests/fixtures/steel-thread/candidate.json` | `3961f960b452cc0aa7e959e77e243f3159390cd42e179b3d8fd467be0b6f1307` |
| `tests/fixtures/steel-thread/config.json` | `3ef2d28326921d818de66bb063037ab861146b514cea5e54de659b14616018b0` |
| `tests/fixtures/steel-thread/expected.json` | `c48b86dd0d416ca496da11e9f87b8e60435d6dd2203f0f62b448f064f0b8ac7e` |
| `tests/fixtures/steel-thread/source-a/evidence.bin` | `37b2f4a1187d75c02c91fd75440bdfae04770a7cd9bd1e5beceb04e06d1c10f2` |
| `tests/fixtures/steel-thread/source-b/evidence.bin` | `717249018fddf920f110df342456d22e4f846f470576883375d934c74f97e309` |
| `tests/cli.rs` | `425568409236bc3f6a834a424916b678e2db1c78cf4a5e487d3f6a4f22050dc0` |
| `docs/acceptance/validator-v1.md` | `98dba75ec4834090132908d28d7066d05ec47918bd180853639597f9ad34cc65` |
| `target/release/validator` | `6fbc83da5f0ec589105802dfdeaf6a3a2e8504b111409164d0c01eed9536ae09` |

The complete candidate hypothesis, path, decision boundary, oracle, command
results, source/schema hashes, privacy and failure evidence are also recorded in
`docs/acceptance/validator-v1.md`. Its status is explicitly `candidate evidence
pending independent verifier and owner acceptance` and it makes no release or
project-completion claim.

## Required command results

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo test --locked --test cli steel_thread_end_to_end -- --nocapture` | 0 | One named real-binary test passed. |
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Exactly 24 inherited owner-staged production dead-code diagnostics. |
| `cargo test --all-features --locked` | 0 | 40 unit, 6 CLI, 11 conformance, and 0 doc tests passed. |
| `cargo build --release --locked` | 0 | Passed. |
| `ruby docs/plans/validator/verify-plan.rb` | 0 | Passed: 35 task contracts, 433 source blocks, 63 cases, 12 structure checks, 8 ACs, 5 DoD clauses, artifact index and links. |
| `git diff --check` | 0 | Passed. |

`cargo test --locked --test cli -- --list` lists
`steel_thread_end_to_end: test`, and the focused command ran one test.

The accepted staged Clippy inventory is unchanged at 24 diagnostics:
`checked_mul`; `TaskDefinition` and its methods; `SingleLabelTask`;
`MultiLabelTask`; multi-label vocabulary and `LabelSet` APIs; `Episode`;
`ObservationSet::values`; `EvaluationConfig::policy`; `MetricUnit::LabelDecision`;
remaining `MetricResult` accessors; `signal_availability`; and strict wire DTO
fields for schema version, task kind, opaque input, and staged decision policy.
There is no new warning class, suppression, fake use, widened export, source or
schema drift, tolerance change, or behavior blocker.

## Frozen production and schema confirmation

The following hashes are unchanged from dispatch012: `src/artifacts.rs`
`8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c`,
`src/app.rs` `28418611219e65eb32ffcdf6ae470f48bca944f68112e9997ee87c1f134b7ad7`,
`src/cli.rs` `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695`,
`src/comparison.rs` `a0288c9d79f116b03925802a52771dff0e4894506a23701b9b15e6cb36b02961`,
`src/evaluation.rs` `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891`,
`src/evaluation/single_label.rs` `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d`,
`src/lib.rs` `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7`,
`src/model.rs` `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66`,
`src/model/common.rs` `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3`,
`src/model/single_label.rs` `5cff2210c215bc84e6a95777649a6e0700a3547056fbb8a93365f9d1e6b1a019`,
`src/validation.rs` `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf`,
and `src/validation/wire.rs`
`46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3`.

All six output-schema hashes also match dispatch012: check
`675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad`,
comparison `24002dce44ffe08c9900c71a2db3a5c81e7b5b274bd05fd647c46468bf2c75d6`,
error `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5`,
inspection `5e7990c05d9f87a29768c73d6d39a1579d39a9c842ce790cb780ff616182fcfa`,
receipt `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00`,
and report `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46`.

No blocker is known. This is a developer handoff for the combined independent
T015–T017 review, not a completion or acceptance verdict.
