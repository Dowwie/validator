# T025 abstention wire and literal-ABSTAIN repair handoff

## Scope and starting identities

I reconciled every pinned starting SHA-256 identity in dispatch005 before editing.
The complete owner/coordinator/oracle records matched their supplied hashes. The
independent oracle body and both expected fixtures were read-only throughout.

The pre-fix frozen exhaustive public path reproduced exactly:

```text
cargo test --locked --test conformance exhaustive_multi_label -- --nocapture
exit 101
tests/conformance.rs:2317:9
left: Null
right: "abstained"
```

The pre-fix report schema's matrix label column had a literal
`not: {"const":"ABSTAIN"}` prohibition. That contradicted the ratified
ordinary-class requirement and prevented a declared `ABSTAIN` class from using
the schema's ordinary label-column branch.

No scoring, count, probability, observation, policy, comparison, fixture, or
oracle-body code was inspected or changed beyond the owning multi-label report
serialization and requested report/inspection schemas and conformance evidence.

## Correction

`src/model/multi_label.rs` now serializes both raw and final whole-episode
multi-label abstentions as closed evidence with the pre-existing `type` and
`reason`, literal `status: "abstained"`, and explicitly emitted JSON-null
`matched`, `missed`, and `extra`. Answered multi-label and all single-label
shapes remain unchanged.

`schemas/v2/report.schema.json` now has a closed `multiAbstention` branch used
only by multi-label episode outcomes. It requires the exact type/reason/status
and three null set-difference fields. The shared bare abstention definition still
serves submitted predictions and single-label output. The inspection schema uses
the same closed branch only for multi-label `final_outcome`; submitted prediction
outcomes retain their input-shape branch.

The report matrix schema now accepts a literal `ABSTAIN` label as an ordinary
label column and requires a distinct typed abstention column using `contains`.
Replacing that typed column with an ordinary `ABSTAIN` label fails because no
typed abstention column remains.

## Focused evidence

The new public report and verified inspection regression runs two real
multi-label abstentions: one with `reason: "manual review"` and one with an
explicitly serialized null reason. It proves presence with `contains_key` for
reason/status/matched/missed/extra in raw and final report outcomes and in both
verified inspection outcomes. It also proves retained marginal probability and
label-marginal observation evidence.

Schema negatives reject deletion of every required abstention field, a changed
status, an array substituted for each null set difference, and the former bare
multi-label abstention shape. A real bare single-label abstention report remains
valid. The real literal-`ABSTAIN` single-label report is valid, shows separate
ordinary and typed-abstention matrix columns, and rejects replacement of the
typed column by an ordinary `ABSTAIN` column.

## Checks

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Formatted. |
| `cargo test --locked --test conformance exhaustive_single_label -- --nocapture` | 0 | 301 independent single-label populations pass. |
| `cargo test --locked --test conformance exhaustive_multi_label -- --nocapture` | 0 | 64 answered pairs plus 8 whole abstentions pass. |
| `cargo test --locked --test conformance multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection -- --nocapture` | 0 | Real report/verified-inspection evidence and schema mutations pass. |
| `cargo test --locked --test conformance single_label_literal_abstain_class_uses_a_typed_abstention_column -- --nocapture` | 0 | Literal class positive and typed-column negative pass. |
| `cargo test --locked --test cli -- --nocapture` | 0 | 10 passed. |
| `cargo build --release --locked` | 0 | Release build succeeds. |
| `cargo clippy --locked --all-targets -- -D warnings` | 101 | Exact pre-existing staged dead-code inventory: 17 library diagnostics plus 3 lib-test diagnostics; no suppression, fake use, or visibility expansion added. |
| `cargo test --locked` | 101 | 44 unit tests and 10 CLI tests pass; conformance is 39 passed/1 failed as recorded below. |
| `git diff --check` | 0 | No whitespace errors. |
| `jq empty schemas/v2/report.schema.json && jq empty schemas/v2/inspection.schema.json` | 0 | Both modified JSON schemas parse. |

The full-suite failure is unrelated to this repair and is preserved verbatim;
I did not correct or reinterpret it:

```text
---- f04_asymmetric_oracle stdout ----

thread 'f04_asymmetric_oracle' (16914060) panicked at tests/conformance.rs:1913:9:
assertion `left == right` failed
  left: Number(2)
 right: Number(1)
```

## Hashes

| Artifact | SHA-256 |
|---|---|
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `tests/conformance.rs` | `6295d99c7124944608556b2ce9f1197af37acab5f8c9c9590cc4a111882ef825` |
| `tests/cli.rs` unchanged | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| Oracle response002 unchanged | `43c0dfa849fce1ff3a4625135e1ea0f8b9306628860c75b255203ee4942fb925` |
| Single-label fixture unchanged | `1fc52d5a24d7900fbef992b0f19227089d83e38d0656b9809fc08f9ed0dcbfe0` |
| Multi-label fixture unchanged | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |

Verdict: **complete bounded correction; ready for coordinator review, with the
separate pre-existing F04 gate failure explicitly retained.**
