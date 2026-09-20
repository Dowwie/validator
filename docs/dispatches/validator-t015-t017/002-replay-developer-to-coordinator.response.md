# T015 replay and inspection developer handoff

## Scope and frozen input

Completed T015 only. I did not begin T016, alter governance, Fizzy, the artifact
index, session notes, `.zvec-grep`, dependencies, report/input schemas, or the
caffeinate process. Before edits, the complete repaired manifest036 candidate
matched the required hashes:

| Input | SHA-256 |
|---|---|
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `src/app.rs` | `819627406e44e184df13d8baf4c1feb115ea0444cf9b7518f83afd2b9e02b875` |
| `src/artifacts.rs` | `14a7383a204c88a7922dedc6159d2b7ad4819b395bab3a8c2185e83ad39aef61` |
| `src/cli.rs` | `608ab90dcda6cd36f8b4d03575f47efc8f2a5977480a0c4bb670aa37933a0fbe` |
| `src/lib.rs` | `b0c5d00e000747368fc9e18e6459c61417dbbf24f06ac58077e9c4cf382ceac7` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `899963586c722a37af0ce0dd8f28d03b86423c8c9bbce431b14280ae8a6cee98` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| `tests/cli.rs` | `3b0cc62faedcb00e46e76a407b3bdf384da90b4596d593a4bf53da46273a567e` |
| `tests/conformance.rs` | `982f128ba6a1b66e3071113b3c05a7fd37ef7269d20a7fa8f34644bb596075a2` |

Manifest036 and Ready verdict037 were read in full. I also applied the in-flight
exact-comparison correction004 before final verification.

## Implementation and criterion mapping

- `src/artifacts.rs:32-181` adds the private `StoredRun` reader. It canonicalizes
  the supplied run directory and every stored read, rejects absolute/parent paths
  and symlink targets outside the canonical root, reads exact snapshots/evidence
  once, verifies the three snapshot digests, and constructs source IDs in UTF-8
  byte order. It verifies every source-local evidence index, original string,
  ordinal `evidence/n.bin`, digest, and manifest cardinality as `E_PROVENANCE`.
  It never opens or resolves original evidence strings.
- `src/app.rs:203-351` adds the public `InspectionOptions`, `InspectionResult`,
  and `inspect` API. Replay feeds only verified stored snapshot bytes through the
  established `validate_single_label` and pure single-label evaluator, reconstructs
  the report with stored identity/time, and rejects inconsistent results before
  disclosure. Public replay internals remain private.
- `src/app.rs:313-397` implements the binding correction004. Object/array shape,
  ordering, identifiers, statuses, counts, paths, ordinals, source/preparation
  configuration, policy constants, and raw/submitted values compare by exact JSON
  spelling. Fixed absolute/relative tolerance applies only to explicit finite
  computed report result paths: metric `value` fields, integrity maximum sum error,
  and signal-bin mean/empirical accuracy fields.
- `src/app.rs:252-310` extracts the selected episode input, target, exact original
  prediction record, and exact configuration through `RawValue`; this preserves
  `9007199254740993`, `1e400`, null, and the literal
  `$serde_json::private::Number` key without converting them through `f64`.
- `src/cli.rs:45-77` adds exactly `validator inspect --run RUN_DIR --episode UUID`.
  Normal typed errors use the existing one-document error path and retain exits.
- `src/lib.rs` exports only inspect options/result/function. `schemas/v2/inspection.schema.json`
  defines the strict version-2 inspection document.
- `tests/cli.rs:50-122` is the built-binary relocation/privacy/schema/opaque-value
  regression. `tests/conformance.rs:43-236` uses real evaluate/inspect replay for
  successful relocation plus snapshot/evidence damage, missing/extra/swapped
  binding, original/index/path rewrite, report-source rewrite, result count/status,
  symlink escape, exact numeric configuration, and tolerance boundary cases.

## Replay comparison evidence

`replay_binding_and_result_tampering` proves all four correction004 cases:

- unchanged stored run replays successfully;
- `raw.total` changing from `1` to `1.00000000005` fails despite being within the
  numeric tolerance;
- recorded `sources.source.configuration.recorded` changing from `0.5` to
  `0.50000000005` fails;
- computed `raw.accuracy.value` changing from `1.0` to `1.00000000005` succeeds.

The test also proves source evidence is not dereferenced: it removes original
inputs after relocation, and all tamper runs remain local with no service or
credential access.

## Output hashes

| Output | SHA-256 |
|---|---|
| `src/artifacts.rs` | `50d63c1dddfdb6ef93d6613ff532ceb9c8ea86dce051b3d96baa70d082609e38` |
| `src/app.rs` | `eb80bef69e0ef88600eeb4634779c85ba84697cb445fb7ce4a316dde77c2ced3` |
| `src/cli.rs` | `34d0d6cc76dbbbf1682fb202834064dc5a5411ddd2c9a42cebb4ba58f63689df` |
| `src/lib.rs` | `0e05e15480f6256f789c4bd6c68d391f0eb81c7557219f964bbb5efa991ba9c7` |
| `schemas/v2/inspection.schema.json` | `5e7990c05d9f87a29768c73d6d39a1579d39a9c842ce790cb780ff616182fcfa` |
| `tests/cli.rs` | `5fad6fa4f66572f9af4a7ed4d23f9c0474046b0aa6ff0acd7a03fe6cafe4f9e5` |
| `tests/conformance.rs` | `37656035d455a75a38a5fcbd8e245da40bb132d809966020ac66875e124f53cf` |

## Command results

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test cli relocated_run_inspection -- --nocapture` | 0 | 1 passed; built binary relocated a run, original inputs unavailable, inspection schema passed, four opaque regressions preserved, unknown episode returned safe error. |
| `cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture` | 0 | 1 passed; all binding, digest, result, symlink, exactness, and tolerance cases above exercised real replay. |
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo test --all-features --locked` | 0 | 39 library, 4 CLI, 9 conformance, 0 binary-unit, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | 0 | Passed. |
| `git diff --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | The exact existing 26 production dead-code diagnostics remain; no new lint class or T015 diagnostic appeared. |

The staged Clippy inventory is unchanged: `checked_mul`; staged task definitions
and methods; multi-label vocabulary/set contracts; `Episode`; observation access;
evaluation policy/dataset digest; label-decision metrics; metric accessors;
`signal_availability`; and the six strict Serde DTO groups. No suppression, fake
consumer, broadened export, or dependency was added.

There are no unresolved T015 behavior, build, test, format, release, or diff
failures. The only nonzero command is the owner-authorized staged Clippy inventory.
