# T003 developer handoff — blocked candidate

## Delivered candidate

The preserved candidate implements the corrected T003 model foundation:

- `EpisodeId` and `RunId` are distinct wrappers around `uuid::Uuid`. They accept
  canonical lowercase hyphenated RFC 4122 UUIDs with standard versions, while
  rejecting nil, max, uppercase, brace, URN, simple, malformed, and version-zero
  forms. No ID generation exists.
- `SourceId` and `ArtifactDigest` are distinct checked types. Source IDs reject
  all-whitespace values without trimming nonblank values. Digests require exactly
  64 lowercase hexadecimal SHA-256 characters.
- `TaskDefinition` is closed to single-label and multi-label forms. Each owns a
  checked `LabelVocabulary` with the task-specific cardinality floor, ordered exact
  labels, exact lookup, and a vocabulary-bound internal `LabelIndex`/`LabelSet`.
- `Episode<Target>` privately owns its UUID, one `serde_json::Value` input, and one
  concrete target. `Outcome<Target>` privately distinguishes an answer from an
  explicit abstention and rejects blank abstention reasons.
- `Prediction`, `Observation`, `ObservationSet`, source definitions, preparation,
  validation, scoring, replay, filesystem behavior, and CLI behavior were not
  added. Prediction plus its real observation set is reassigned to T004 by the
  governing correction.

All identity/label/model construction failures use T001's typed `E_ID`, `E_LABEL`,
or `E_SCHEMA` diagnostics without interpolating opaque input or credentials.

The candidate is blocked solely by the required no-warning Clippy gate; it must not
be accepted or independently verified as T003-complete in this state.

## Changed paths and candidate hashes

| Path | Purpose | SHA-256 |
|---|---|---|
| `Cargo.toml` | Adds `uuid` with no default or optional generation/serde/RNG features | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` |
| `Cargo.lock` | Locks `uuid` 1.26.1 and its minimal resolution | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | Charter-authorized parent visibility change to `pub(crate) mod model`; it remains inaccessible to external crates | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` |
| `src/model.rs` | Declares/re-exports the common foundation and closed `TaskDefinition`, preserving T001 constants | `05abe8af935ced43e50b14bc07812569a3984aaff9b9c6ba2c1c28896d88ad0f` |
| `src/model/common.rs` | Checked identities, vocabulary-bound labels, episode/outcome records, and local tests | `fb977bb4c1c9c0eb5508ad846fbc01010bb332a2e57451d3096c31c5cf62b88b` |

The coordinator requested preservation of these exact bytes after the gate failure.
The response's final hash is left for the coordinator's candidate freeze.

## Corrected-scope mapping

| Requirement group | Candidate evidence | Deferred ownership |
|---|---|---|
| V026–V044, V041–V043; D003–D015; S13, S21, AC1 | `common.rs` implements typed canonical identities, opaque episode input ownership, closed task kind, exact vocabulary rules, task minima, and checked answer/abstention construction. `canonical_identifiers`, `vocabulary_exact_identity`, and `model_boundary_visibility` each passed once when filtered. | Wire envelopes, duplicate-key checks, record collection/alignment, targets attached to decoded DTOs, and full canonical artifact semantics remain later validation/model tasks. |
| D009–D013, D027, D053, D055 | The private fields and checked constructors prevent free string identities, mutable vocabulary contents, unknown/duplicate labels, and vocabulary-index mixing. `LabelIndex` carries the shared allocation of its creating vocabulary; a same-text index from another vocabulary fails its lookup. `LabelSet` rejects duplicate submitted labels and orders positions by vocabulary order. | Concrete single-label/multi-label target/output types, validated evaluations, policy types, distributions, and evaluator APIs remain their owning tasks. |
| V231–V248, D061–D064 | The model module contains shared records and preserves the single package/binary/library arrangement. No new runtime behavior was introduced. | CLI, app, validation, evaluation, comparison, artifacts, and module wiring beyond the necessary model-parent visibility remain their named tasks. |
| Owner correction `003-owner-to-coordinator.prompt.md` | `Prediction<Output>` and every observation/source/preparation type were intentionally omitted from T003. | T004 owns the complete typed `Prediction<Output>` with `ObservationSet`; this candidate contains no placeholder or raw observation bypass. |

## Required checks

Pinned toolchain and host:

```text
rustc 1.98.1 (48a229cea 2026-09-01)
cargo 1.98.1 (797e8a9bc 2026-08-05)
active toolchain: 1.98.1-x86_64-apple-darwin
host: x86_64-apple-darwin
```

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | Listed six library tests, including all three exact T003 names. |
| `cargo test --locked --lib canonical_identifiers -- --nocapture` | 0 | Ran one matching test; passed, five filtered. |
| `cargo test --locked --lib vocabulary_exact_identity -- --nocapture` | 0 | Ran one matching test; passed, five filtered. |
| `cargo test --locked --lib model_boundary_visibility -- --nocapture` | 0 | Ran one matching test; passed, five filtered. |
| `cargo fmt --all -- --check` | 0 | No formatting differences. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | **Failed.** One unused-import diagnostic and 20 dead-code diagnostics are promoted to errors. |
| `cargo test --all-features --locked` | 0 | Six library tests passed; binary and doc-test targets passed with zero tests. The non-test library build emitted 21 warnings. |
| `cargo build --locked --bin validator` | 0 | Built, but emitted the same 21 warnings. |
| `cargo tree --locked -e features -i uuid` | 0 | Shows only `uuid v1.26.1` as a Validator dependency; no optional UUID features are enabled. |
| `git diff --check` | 0 | No whitespace errors in tracked diff; the repository is unborn and candidate files are untracked. |

The exact Clippy failure is caused by the candidate's intentionally unwired model
surface: `src/model.rs` re-exports eight common types that T004+ will consume, and
the non-test library build has no permitted caller for `TaskDefinition`, the checked
identity constructors, vocabulary/index APIs, `Episode`, or `Outcome`. The direct
import warning and all 20 dead-code warnings become `-D warnings` errors. No lint
was suppressed, no test was weakened, and no placeholder runtime use was added.

## Required owner/coordinator resolution

The current T003 assignment makes a passing no-warning Clippy gate impossible while
also requiring the complete checked model types and forbidding all of the ways they
could become reachable. The mechanical options are:

1. Issue a numbered correction authorizing a small public library boundary for the
   current model API (for example, a public `model` module or selected public
   re-exports in `src/lib.rs`). This changes the external library surface and needs
   an owner decision.
2. Issue a numbered correction authorizing an actual current production consumer
   for the model types. No such behavior exists within T003; a fake consumer would
   violate the prompt.
3. Issue a numbered correction explicitly permitting a documented temporary
   `#[expect(dead_code)]` boundary until the owning tasks wire the types. The
   current prompt and charter prohibit suppression, so this was not added.

## Dependency and authority inputs

`uuid` 1.26.1 is the established UUID primitive requested by the owner correction.
Its `default-features = false` configuration enables no generation, RNG, time,
serde, macro, or framework feature. Its published Rust version floor is 1.85.0,
below the tested package MSRV of 1.98.1.

Accepted T001 inputs used as the baseline:

| Path | Accepted SHA-256 | Current state |
|---|---|---|
| `Cargo.toml` | `ef2bb8fa026d665fe09f8670e9b15dd27518dffd1f553124857c293cff05b5c2` | Changed only for `uuid`. |
| `Cargo.lock` | `157fcfe04a5fdff11f1e1770f0e3228a822ff32c008215c987d89c63a2678fa9` | Changed only for the UUID lock resolution. |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged. |
| `src/lib.rs` | `cad8e175086607a8f194176a738e6a10e1b5fc7372f38679b6952246e23e22df` | Changed only for the charter-authorized model-module visibility attempt. |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged. |
| `src/model.rs` | `a16faa58a7d727ea3894509529564b3906dffa76f32f534d6966413e8dae3cb5` | Changed for T003. |

Current authority input hashes:

| Path | SHA-256 |
|---|---|
| `docs/dispatches/validator-t003/002-coordinator-to-developer.prompt.md` | `832308ec2e30592935476fcf6d55d10be9e82da1244bc231185479a5589463ab` |
| `docs/dispatches/validator-build/003-owner-to-coordinator.prompt.md` | `366c19d1ef4abdda8b525a78ad2ff9a8a0c8ffdd63b60450b7aea44fec5a3f7c` |
| `docs/plans/validator/tasks/T003.json` | `a3c1d3159ec39bf5fb493b5de6d736b4fc2e007f23fa9fae0c8ba041dbbfe329` |
| `docs/plans/validator/execution-contract.md` | `efc2c6b1986057fd20a67e7edb49ae2c2ce63cb0b78da2b13fc16e72df7bdf2d` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/dev-team/validator-build/charter.md` | `cdc7276d9ccedd73a3641b5949e630a45c73a27c52718790927b497689407deb` |
| `AGENTS.md` | `16a210983a47737e371717bbec81f0517cc0a33047a150eb5c2e39012716e070` |

No active Cargo or Rust compiler process remained after checks. No resource surprise
occurred beyond the small, normal UUID crate download. This response does not claim
independent verification, a Ready verdict, or T003 acceptance.
