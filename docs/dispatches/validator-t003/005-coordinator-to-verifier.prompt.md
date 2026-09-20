# Validator T003 verifier dispatch 005

Role/model: existing verifier, `gpt-5.6-sol`, reasoning `high`, retained verifier
context.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/005-verifier-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome and binding

Independently verify the exact frozen corrected-T003 candidate for its behavioral
and type-boundary criteria. Return one verdict: **Ready**, **Revise**, or
**Blocked**. A Ready verdict may carry the explicitly authorized incomplete-
consumer dead-code integration limit, but must not call Clippy passed, waived, or
complete and must not imply the admission/T014/T017/product gate is complete.

Read before the implementation summary:

- `docs/plans/validator/tasks/T003.json` and
  `docs/plans/validator/execution-contract.md` as corrected.
- `docs/dispatches/validator-build/003-owner-to-coordinator.prompt.md` for corrected
  T002/T003/T004 ownership and dependency wiring.
- `docs/dispatches/validator-t003/003-owner-to-coordinator.prompt.md` for the exact
  interim-warning disposition.
- `docs/specs/validator-v1.md`: Canonical golden dataset and Rust organization.
- `docs/specs/validator-data-model.md`: Design boundary, Shared records and typed
  task data, Decode/validate/align/score, and Source organization.
- `docs/dev-team/validator-build/charter.md` and repository `AGENTS.md`.
- Corrected developer assignment:
  `docs/dispatches/validator-t003/002-coordinator-to-developer.prompt.md`.
- Frozen manifest:
  `docs/dispatches/validator-t003/004-coordinator-candidate-manifest.md`, SHA-256
  `6d6113cfc205806781f738e65770b781cde12f9a2fba2c84c013b39333f6e27b`.
- Then the developer response:
  `docs/dispatches/validator-t003/002-developer-to-coordinator.response.md`, SHA-256
  `3e1e5a546899d1cbdad7a3cb6cbe376c0a83e9e82d5a5ab72fafaee5f0416e01`.

The implementation writer is idle. Recompute and bind these candidate hashes:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `05abe8af935ced43e50b14bc07812569a3984aaff9b9c6ba2c1c28896d88ad0f` |
| `src/model/common.rs` | `fb977bb4c1c9c0eb5508ad846fbc01010bb332a2e57451d3096c31c5cf62b88b` |

If an implementation/evidence/manifest hash changes, stop and return Blocked.
Coordinator dispatch/index writes are outside the candidate.

## Corrected T003 criteria

Inspect the complete scoped change and verify:

- `EpisodeId`/`RunId` are distinct wrappers around the established UUID primitive;
  canonical lowercase hyphenated standard-version values pass while nil, max,
  version-zero, uppercase, brace/URN/simple, and malformed values fail. No ID
  generation or unneeded UUID feature exists.
- `SourceId` and `ArtifactDigest` are distinct, checked, exact types; digest rules
  are lowercase 64-character SHA-256 hex and source IDs preserve nonblank strings
  without trimming/normalization.
- `TaskDefinition` is closed to single-label/multi-label, each owning an exact
  vocabulary with correct minima. Vocabulary order/case/whitespace/Unicode are
  preserved, duplicates/blanks rejected, indexes remain vocabulary-bound and are
  not durable serialized identities, and label sets reject duplicates/order by the
  vocabulary without creating a generic task/plugin system.
- `Episode<Target>` owns one opaque input and target privately; `Outcome<Target>`
  distinguishes answered/abstained and rejects blank reasons. Checked/private
  construction exposes no mutable or wire-Deserialize bypass.
- `Prediction<Output>` and all observation/source/preparation behavior are absent
  under their corrected T004 ownership.
- T001 constants/errors remain intact; UUID dependency/lockfile/features are
  minimal; no suppression, fake runtime caller, placeholder, public SDK expansion,
  test weakening, or unrelated behavior was added.

Assess `src/lib.rs` changing `mod model` to `pub(crate) mod model` and the
`src/model.rs` reexports against the narrow parent-declaration/reexport routing
rule. If either change is unnecessary for T003 or expands the intended boundary,
return a criterion-backed Revise finding with the smallest correction.

## Checks and exact warning distinction

Run and record:

```sh
cargo test --locked --lib -- --list
cargo test --locked --lib canonical_identifiers -- --nocapture
cargo test --locked --lib vocabulary_exact_identity -- --nocapture
cargo test --locked --lib model_boundary_visibility -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo tree --locked -e features -i uuid
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Confirm each named filter runs a nonzero expected count. The final Clippy command
is expected to exit 101 on the frozen candidate. Compare its full output with the
manifest and distinguish:

1. the one unused-import group at `src/model.rs:4-5`; and
2. the twenty incomplete-consumer dead-code diagnostics listed in the manifest.

The owner disposition explicitly covers the incomplete-consumer dead-code only.
Determine whether the import/reexport diagnostic is likewise an unavoidable
temporary consequence of a required T003 reexport, or is an unnecessary new
import/reexport that needs the smallest developer correction. Any additional
warning, semantic defect, unsafe behavior, failed behavioral check, or hash drift
is not covered and must affect the verdict normally. Do not invent more tests once
the current uncertainty is resolved.

## Response contract

Save the complete response before returning. Put the single verdict first. Include
recomputed hashes, criterion-level locations/evidence, exact command exits and test
counts, full Clippy classification, the import/reexport decision, unchanged T001
evidence, remaining integration limit, active processes, and confirmation that no
implementation/test/fixture/schema/acceptance artifact was edited.

For Revise, name requirement, source location, reproduction, consequence, and the
smallest correction. Do not fix code. Return only the required response path.
