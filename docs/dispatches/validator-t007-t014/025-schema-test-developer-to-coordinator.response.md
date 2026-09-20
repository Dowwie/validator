# T013 typed report and output-schema handoff

Status: complete for the bounded T013 unit. No T014 or later behavior was started.

## Frozen inputs

The prescribed T012 response matched SHA-256
`d264550555d10186fcce1e966f902a2385b5468158aff4f2220d0d0328b29982`.
The unchanged frozen inputs retain these hashes:

| Path | SHA-256 |
| --- | --- |
| `Cargo.toml` | `366bef64d9ac1e898ca6fcf1559c7ac744b671068fc3383d8c374530c5242bd8` |
| `Cargo.lock` | `33165fd692aaa3355ad66c391d31236da8f4b9a302704c3228b699ee7958c4ca` |
| `src/lib.rs` | `a61426bfb002b8b5634692e7e6f321f626e3d4d73ec1fabb90e0564c657a9a9c` |
| `src/artifacts.rs` | `14a7383a204c88a7922dedc6159d2b7ad4819b395bab3a8c2185e83ad39aef61` |

No Cargo dependency or artifact-publication behavior changed.

## Delivered implementation

- `src/model/single_label.rs` contains private typed `SingleLabelRunReport` assembly. The caller supplies run identity, creation time, version, checked evaluation/results, snapshots, and evidence. Assembly neither reads files, scores rows, creates identities/timestamps, nor publishes output.
- The report has the closed `schema_version: 2`, `kind: "evaluation"`, `status: "complete"` envelope; identity, manifest, source metadata, population, task/policy/integrity, raw/final hard results, probability results, signal diagnostics, and sorted episode evidence.
- Source maps and observation maps are ordered. Source counts include unused declarations at zero. Evidence references are rewritten only to bound stored paths. The manifest retains evidence source/index/original-path bindings and exact digests.
- Episode reports preserve typed class/abstention outcomes, abstention reason, submitted and working categorical vectors, argmax diagnostics, observations, and confidence. They do not include opaque golden input, provider payloads, or evidence bytes.
- `src/model/common.rs` has the narrow report-only observation conversion and source/preparation/observation-definition accessors required by the typed conversion. `src/evaluation/single_label.rs` carries the checked abstention reason into retained episode evidence.
- Added strict Draft 2020-12 schemas: `schemas/v2/report.schema.json`, `check.schema.json`, `receipt.schema.json`, and `error.schema.json`. The report schema closes the concrete single-label envelope and does not add a permissive multi-label branch. It checks typed metric accounting, matrix columns, source/evidence bindings, reports, bins, and episode evidence. Receipt has closed evaluation/comparison alternatives; check and error implement their current machine contracts.
- `tests/conformance.rs` has `single_report_schema`, which compiles all four schemas and covers complete, hard-label-only, empty, and evidence-bearing-abstention reports; discriminating malformed report mutations; and representative valid/invalid check, receipt, and error documents.
- Same-module `report_sources_and_privacy` builds checked data and actual results. It proves mixed-source/unused-source counting, stored evidence rewrites, canonical episode order, complete metric families, evidence-bearing abstention preservation, empty population behavior, schema validation of serialized reports, and sentinel exclusion.

## Criterion mapping

| Requirement | Location and evidence |
| --- | --- |
| Typed closed report, complete metrics and episode privacy | `src/model/single_label.rs`: `SingleLabelRunReport`, `assemble_report`, `report_sources_and_privacy` |
| Source/preparation/observation report conversion | `src/model/common.rs` report helpers; `src/model/single_label.rs` report source types |
| Abstention reason retention | `src/evaluation/single_label.rs` episode evidence; report decision serialization |
| Strict machine schemas | `schemas/v2/report.schema.json`, `check.schema.json`, `receipt.schema.json`, `error.schema.json` |
| Direct schema matrix | `tests/conformance.rs`: `single_report_schema` |

## Verification

All commands exited 0.

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo test --locked --lib report_sources_and_privacy -- --nocapture` | pass; 1 test, 38 filtered |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | pass; 1 test, 1 filtered |
| `cargo check --locked` | pass |
| `cargo test --locked --lib` | pass; 39 tests |
| `cargo test --locked --test conformance input_schema_contract -- --nocapture` | pass; 1 test, 1 filtered |
| `git diff --check` | pass |

`cargo check --locked` and the conformance test compilation emit the expected pre-T014 private-consumer `dead_code` warnings (211 in the library build). The focused library test emits the existing six warnings. They were not suppressed. T014 owns the real application API and the full Clippy gate.

The command `cargo test --locked --test conformance report_sources_and_privacy -- --nocapture` is deliberately deferred to T014. No static fixture, public export, source inclusion, or test-only facade was added to make that integration filter pass before the real application API exists.

## Output hashes

| Path | SHA-256 |
| --- | --- |
| `src/model/common.rs` | `d30645baf87c011e919945b65464d3fa890ac7d0b9b906f00496d26a290c874e` |
| `src/model/single_label.rs` | `c97ae399fe81896e0e552e0152e99db53addb14fa72043f7c68048f4bfb61e1b` |
| `src/evaluation/single_label.rs` | `ad9dbeb0dde5b90bf23e6f6b1310270c2f8371d4bce836efca3d7a1a9cbfb944` |
| `tests/conformance.rs` | `ca25086989ab2e99b901a607ee2f2ae7c4cbda454817b8bbddae326759b24f47` |
| `schemas/v2/report.schema.json` | `a99d4ab2bfc85180b9ce7619e2399ed721b7e5a60d98459efc6935578e6fdb0b` |
| `schemas/v2/check.schema.json` | `675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |

No unresolved implementation failures remain within T013.
