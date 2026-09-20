# T003 frozen candidate manifest 004

Frozen by the coordinator on 2026-09-18 after reading the complete developer
handoff and applying the owner's explicit interim-warning disposition. The sole
implementation writer is idle and implementation writes are paused until the
independent verifier returns. This candidate is reviewed for corrected T003
behavior and type boundaries, with the exact incomplete-consumer Clippy failure
preserved as an unresolved integration check.

## Accepted baseline

T001 is accepted at candidate-manifest SHA-256
`a12e570a1299b86d6793b177301bd287979ae824e84e8b5e739796ee03e64174`.
Before corrected T003, the affected baseline files were:

| Path | Accepted SHA-256 |
|---|---|
| `Cargo.toml` | `ef2bb8fa026d665fe09f8670e9b15dd27518dffd1f553124857c293cff05b5c2` |
| `Cargo.lock` | `157fcfe04a5fdff11f1e1770f0e3228a822ff32c008215c987d89c63a2678fa9` |
| `src/lib.rs` | `cad8e175086607a8f194176a738e6a10e1b5fc7372f38679b6952246e23e22df` |
| `src/model.rs` | `a16faa58a7d727ea3894509529564b3906dffa76f32f534d6966413e8dae3cb5` |

`src/model/common.rs` did not exist.

## Frozen candidate

| Path | SHA-256 | State |
|---|---|---|
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` | Adds minimal UUID dependency |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Locks UUID resolution |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged accepted input |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` | Changes `model` visibility to `pub(crate)`; no external SDK export |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged accepted input |
| `src/model.rs` | `05abe8af935ced43e50b14bc07812569a3984aaff9b9c6ba2c1c28896d88ad0f` | Common module/reexports, closed task definition, preserved constants |
| `src/model/common.rs` | `fb977bb4c1c9c0eb5508ad846fbc01010bb332a2e57451d3096c31c5cf62b88b` | Checked T003 types and tests |
| `docs/dispatches/validator-t003/002-developer-to-coordinator.response.md` | `3e1e5a546899d1cbdad7a3cb6cbe376c0a83e9e82d5a5ab72fafaee5f0416e01` | Developer evidence |

No other implementation, test, schema, fixture, or acceptance file belongs to
this candidate. `target/` is replaceable build output and excluded.

## Behavioral evidence and known integration check

Developer results on pinned Rust/Cargo 1.98.1, `x86_64-apple-darwin`:

- `cargo test --locked --lib -- --list`: exit 0, six library tests listed.
- Each of `canonical_identifiers`, `vocabulary_exact_identity`, and
  `model_boundary_visibility`: exit 0, exactly one test passed.
- `cargo fmt --all -- --check`: exit 0.
- `cargo test --all-features --locked`: exit 0, six library tests passed;
  non-test library build emitted the known warnings.
- `cargo build --locked --bin validator`: exit 0 with the same warnings.
- Minimal UUID feature-tree inspection: exit 0; no optional UUID features enabled.

The coordinator independently reproduced:

```text
cargo clippy --all-targets --all-features --locked -- -D warnings
exit 101
```

The complete diagnostic set is one unused-import group and twenty dead-code
diagnostics, all in the new private model surface:

- `src/model.rs:4-5`: unused imports for `ArtifactDigest`, `Episode`, `EpisodeId`,
  `LabelIndex`, `LabelSet`, `Outcome`, `RunId`, and `SourceId`.
- `src/model.rs:10`: unused `TaskDefinition`; `src/model.rs:19-36`: its unused
  `single_label`, `multi_label`, `vocabulary`, and `is_single_label` methods.
- `src/model/common.rs:30`, `:34`: never-constructed `EpisodeId` and `RunId`.
- `src/model/common.rs:38`, `:53`: unused `parse_uuid` and `invalid_identifier`.
- `src/model/common.rs:59`, `:75`: never-constructed `SourceId` and unused `as_str`.
- `src/model/common.rs:82`, `:117`: never-constructed `ArtifactDigest` and unused
  `hex_digit`.
- `src/model/common.rs:127`, `:132-184`: never-constructed `LabelVocabulary` and
  its unused constructors/accessors/set builder.
- `src/model/common.rs:207`: never-constructed `LabelIndex`.
- `src/model/common.rs:214`, `:221-226`: never-constructed `LabelSet` and unused
  `is_empty`/`contains`.
- `src/model/common.rs:234`, `:242-257`: never-constructed `Episode` and unused
  constructor/accessors.
- `src/model/common.rs:264`, `:269`, `:276-305`: never-constructed `Outcome`,
  unused `OutcomeState`, and unused constructors/accessors.

The owner authorized behavioral review with this exact failure disclosed. The
verifier must confirm it contains only incomplete-consumer warnings, identify any
unrelated warning or defect normally, and never report Clippy as passed. No warning
was suppressed, no fake caller added, no test weakened, and no public SDK export
made. Clean full Clippy remains required at the first complete T014 path and the
mandatory T017 gate.
