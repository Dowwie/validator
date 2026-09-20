# T020 final numerical and CLI evidence handoff

Status: the requested public numerical and CLI evidence is complete. The full
boundary evidence is complete except for the unmodified warning-denied Clippy
gate, which exits 101 on the exact inherited inventory below. This is a local
handoff, not owner acceptance or independent-review verdict.

## Identity and write scope

Every prompt018 fixed hash matched before editing. Only these test files changed:

| Artifact | Starting SHA-256 | Final SHA-256 |
|---|---|---|
| `tests/conformance.rs` | `15507426e673eea44aa04616b3ce8601785f270308cba113ec72d7ad9ba20ff6` | `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291` |
| `tests/cli.rs` | `c23e38a343fb8eaccb1b784bbdf24cd15c0c4f50f081e461cf70af7add399999` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` |

All prompt-fixed production files and schemas remain byte-identical. In
particular, `src/model/multi_label.rs`, `src/evaluation/multi_label.rs`,
`src/evaluation.rs`, `src/app.rs`, `src/cli.rs`, and `src/artifacts.rs` retain,
respectively, `a4f2f7fa678c525f1c3aed34fb1c3931c887fe3359aa13cd7396cbeee228be46`,
`013f014ed1300a4e2173bc0a4a187af53717079c7d80a37575932e242287257b`,
`172ab7db055ca9483e3b64593411bb9e64c1983eecdbd06013fe0f6ce1a483ff`,
`b8f1b5b32bdff042eb5bca3b667da6fccdeb793bec04bb3b35c9bd7d4ce47b1b`,
`ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695`, and
`8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c`.
The check/report/inspection schemas stay at
`1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9`,
`7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1`, and
`3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176`.
The independent T019 oracle is unchanged at
`a65de157415f75567cb907f2dd45ae3d524dcb552dce7c10ef9ca87efbfc63ec`.

## Criterion-to-real-path evidence

`multi_label_hard_oracles` calls public `validator::evaluate`, reads the
published report, and validates it against the fixed schema. It proves the
independent two-episode oracle: `N=G=2`, exact accuracy `1/2`, aggregate
`TP/FP/FN/TN=1/1/1/3`, micro-F1 `2/4`, macro-F1 `1/3`, and Hamming `2/6`.
It checks raw/final equality; selected versus answered scopes; episode versus
label-decision units; ordered matched/missed/extra evidence; answered empty sets;
abstention null differences and coverage; all-abstained
`no_answered_predictions`; and empty-selection `no_data`.

`equal_counts_distinct_exact_sets` publishes two real two-row reports, asserts
identical per-label TP/FP/FN/TN arrays, and proves exact-set accuracy `1/2`
versus `0/2` with retained episode correctness evidence.

`marginal_loss_and_bins` uses public evaluate and the published report. An
abstention with reference `{A}` and `[0.8,0.7]` proves mean Brier `.265` and
mean log loss `(-ln(.8)-ln(.3))/2` over selected `N*K=2` label decisions.
It covers absent-at-zero loss zero, present-at-zero and absent-at-one positive
infinity (`value:null`, `special_value:+infinity`), one-label binary Brier `.04`
versus categorical `.08`, and reference-positive bins at `0`, `.1`, and `1`.
The established fixture tolerance is used only for binary64 results.

`shared_commands_multi_label` runs the built binary through check, evaluate, and
relocated inspect. It validates check/receipt/report/inspection schemas, asserts
one JSON document on each successful stdout, proves normal output/error and the
report omit an opaque secret, preserves the exact opaque payload through inspect,
refuses existing output with its tree unchanged, supports input/evidence deletion
after relocation, and rejects changed contained evidence as `E_PROVENANCE` at
replay. No test reproduced a production defect, so no production correction,
tolerance change, export, dependency, or schema change was made.

## Required filters and gates

Cargo list output contained all six exact names. Each filtered command exited 0
and selected one test: `multi_label_checked_admission` (43 filtered),
`cross_task_boundaries` (43 filtered), `multi_label_hard_oracles` (14 filtered),
`equal_counts_distinct_exact_sets` (14 filtered), `marginal_loss_and_bins`
(14 filtered), and `shared_commands_multi_label` (6 filtered).

| Command | Exit | Actual result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | 23 inherited diagnostics below |
| `cargo test --all-features --locked` | 0 | 44 library, 7 CLI, 15 conformance, 0 doc tests passed |
| `cargo build --release --locked --bin validator` | 0 | release binary built |
| `git diff --check` | 0 | passed |

The intentionally unborn/untracked workspace has no tracked patch for
`git diff --check`; prompt-bound and final SHA-256 hashes are the identity record.

## Exact residual lint inventory

The 20 staged private-symbol `dead_code` diagnostics are in `src/model.rs`
(`TaskDefinition`; `SingleLabelTask`; `MultiLabelTask` and listed methods),
`src/model/common.rs` (`LabelSet`, `Episode`, `ObservationSet`,
`EvaluationConfig`, and `MetricResult` members), `src/model/single_label.rs`
(`signal_availability`), and `src/validation/wire.rs` (schema-version/input and
unconsumed later-policy fields). They are all present in prompt-fixed unchanged
production files.

Three ordinary, also pre-existing, Clippy diagnostics remain: needless borrow at
`src/app.rs:369` and `src/app.rs:390`, and identity map at
`src/model/multi_label.rs:463`. I made no suppression, fake use, visibility
change, or out-of-scope cleanup. The smallest unfinished item for owner-directed
repair is disposition/correction of those three ordinary diagnostics; a test-only
scope does not authorize production edits solely to clear them.

No T021 policy implementation, T022 multi-label comparison, later-task work,
governance, planning, artifact-index, session-note, acceptance, or Fizzy change
occurred.
