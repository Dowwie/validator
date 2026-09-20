# Independently verify the frozen T027 structural/offline candidate

Role/model: fresh independent verifier, `gpt-5.6-sol`, high reasoning,
`fork_turns: none`. Do not delegate. You did not implement this candidate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #190 — Verify Validator module boundaries and offline suite](http://localhost:3006/1/cards/190).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t027/009-verifier-to-coordinator.response.md`.

Save the full substantive Ready/Revise/Blocked verdict before returning only path,
SHA-256 and verdict. Do not edit source, tests, schemas, fixtures, governance,
plans, index, session/acceptance records or Fizzy. Temporary read-only probes may
live outside the repository. Do not delegate.

Reconcile candidate manifest008 SHA-256
`1fb631264e5b6a396031be184ab6eb320786529cb53ff00aa976f50d77d01fa5` and every listed hash
before review. Stop Blocked on mismatch. Read global/repository AGENTS, complete
T027, owner prompts001/004, both full developer responses002/007, governing spec/
data-model sections, execution contract and the accepted T026 baseline.

## Genuine structure review

Review actual definitions, construction, ownership, signatures, imports and
callers. Confirm:

- private tagged wire DTOs retain version/tag/unknown-field/duplicate-key and
  opaque raw-value behavior, with concrete schema-version-2 enforcement;
- TaskDefinition and both wrappers perform real closed admission work; no token
  construction, discard, fake consumer, registry/trait or public SDK expansion;
- each golden row constructs exactly one Episode owning opaque input/id/target;
  scoring borrows ID/target, metrics/comparison never clone opaque input, and
  inspection discloses it only after verified replay while preserving exact JSON;
- task-specific policies, outputs, evaluations and evaluator signatures prevent
  cross-kind construction; strict source/observation/preparation guards remain;
- validation/evaluation/comparison purity, app orchestration/recomputation,
  artifacts filesystem ownership, thin documented lib/main/CLI API, module
  directions, complexity and dependency/toolchain boundaries remain intact;
- removed/test-local helpers are genuinely non-required conveniences and no
  required production type is hidden solely to silence Clippy.

## DM and offline evidence

Inspect every exact `structure_dm01` through `structure_dm12` filter against its
complete T027 row. Each must execute discriminating real public behavior; a wrapper
name is not proof. Apply owner004: static type/shared-path claims also need direct
source/signature/caller review, while runtime behavior does not need a new compiler-
test/source-text framework or private-model export.

Inspect `offline_cli_contract` for Cargo's built executable, saved public synthetic
inputs, both task kinds and relevant commands, with no installed binary,
credentials, private data or network dependency. Reconcile every planned through-
T027 public test ID in Cargo's list. Confirm later T028-T035 proof is honestly
pending rather than treated as a T027 blocker or silently claimed complete.

Run all thirteen new filters nonzero and then the literal final commands serially:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline
git diff --check
```

Every command must exit 0; both full suites must report 44 library, 12 CLI, 103
conformance and zero doc tests. Run `verify-plan.rb` and reconcile manifest hashes
after execution. Use the existing target; no fresh build tree or private-data audit.

Return **Ready** only if the exact candidate has genuine checked boundaries, every
DM/offline proof and clean/offline gates with no known blocker. Return **Revise**
for each reproducible current-contract defect with exact requirement/location/
input/expected/actual/consequence and smallest correction. Return **Blocked** only
when identity/evidence prevents judgment. Stop when this bounded review supports a
verdict; do not request optional redesign or later-task implementation.
