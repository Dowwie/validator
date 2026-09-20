# Accept T026 and finish the warning-free public implementation gate

Existing Sol-high coordinator; repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to `docs/dispatches/validator-t027/001-coordinator-to-owner.response.md`.

## Accepted predecessor and active authority

Owner read complete T026 implementation/repair responses and verdicts, including
the preserved Ready013, superseding source-origin addendum015, final Ready018 and
coordinator handoff019. Accept the exact completed T026 candidate:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t026/017-coordinator-source-field-manifest.md` | `bb0ea141de203596fcd9ee3c7db8b7bf37f8a2033e7858d05074b9bf03e3cf44` |
| `docs/dispatches/validator-t026/018-verifier-to-coordinator.response.md` | `b6d8ea67f0eccaaa0f3813e5e45dd557bda1bc8f553632145f2ba78c8eb94c98` |
| `docs/dispatches/validator-t026/019-coordinator-to-owner.response.md` | `c54af31f1c27670a581f2b9604c70e0db531345205a429693a76393bc70180f6` |

Record acceptance and close Card188. All numerical, integrity, replay, source,
schema and exact-filter corrections remain part of the accepted baseline. T027 is
now authorized. The separately authorized read-only T028 review may continue; its
protected bundle is not a test dependency or a T027 write target. Maintain four
active roles, one developer and one verifier. T029+ remain undispatched.

## Required outcome and role correction

Deliver the prescribed public Rust implementation with genuine checked type/module
boundaries, all required public conformance/structure evidence, and the literal
format/Clippy/test/release gates passing without warnings or bypasses. This is the
mandatory point where the staged17+3 diagnostic allowance ends. No diagnostic may
be deferred to T029 or final acceptance. Python preparation cannot be a Rust
consumer.

Read complete T027, the main specification's project organization/dependency/test
sections, shared data-model structures/ownership/structure checks, accepted
execution contract and manage-dev-team skill. Correct T027's historical author
role to developer; independent Sol verification remains read-only. Use one fresh
Terra-high/fork-none sole developer for each local output below. No worker delegates.

## Two coherent local outputs

1. **Real structural integration and clean existing suite.** Inspect actual
   construction and ownership flows, map every staged diagnostic to its real
   resolution, and make the smallest required integration. Report the concrete
   flow/file plan to the coordinator before editing so task/physical/input ownership
   is synchronized. Then proceed within this authority without an extra approval
   round unless a material contract choice is unresolved. Compile at the first
   coherent change, preserve all existing behavior/tests, and finish with clean
   format, warning-denied Clippy, full existing tests and locked release. Save a
   complete source/diagnostic/behavior handoff before output2.
2. **Exact structure and offline proof.** A fresh developer adds/completes all
   twelve exact `structure_dm01` through `structure_dm12` filters and
   `offline_cli_contract`, reusing existing complete real-API/process scenarios.
   Each named filter must prove its assigned row; a wrapper name alone is not
   evidence. Reconcile all earlier mandated public test IDs against Cargo's actual
   list without removing/renaming accepted filters. Complete the public source/
   conformance/structure evidence mapping, then run the final gates below. Freeze
   once for one independent combined T027 review.

Do not repeat an unfinished broad bundle after an incomplete handoff. Return its
exact cause and smallest coherent next result for owner reassessment. Local
implementation is not independent acceptance.

## Structural boundaries and smallest-change scope

The specification's named shared structures must perform their actual required
roles. In particular, do not silence unused TaskDefinition/task wrappers/Episode
by adding token construction, discard calls, fake consumers, underscore renames,
public exports or test-only gating of required production types. Integrate them
into real checked dataset/evaluation/row construction and ownership where required.
Opaque input is owned once and borrowed for scoring; it is not cloned into metric
or comparison rows. Use closed task dispatch and task-specific outputs, policies,
probability vectors and results. No Task trait, registry or extension framework.

Preserve strict DTO privacy, explicit tags, recursive duplicate-key rejection,
required schema-version/input checks, unknown-field rejection, exact opaque JSON
values and source/observation/preparation validation. Do not remove validation
guards to eliminate unread wire fields. Resolve redundant representations at the
actual decoding/checked-construction boundary. Truly unused non-required helpers
created by this build may be removed; genuinely test-only convenience helpers may
be test-local. Every such disposition must be explained against actual use.

Preserve the single Cargo binary/library package, thin main/CLI/lib API and the
specified module dependency directions. Validation, evaluation and comparison
remain pure; app owns orchestration/recomputation and artifacts owns filesystem/
hash/publication. Review actual signatures/imports and invariants, not only file
names. Keep exports limited to the real application API and required types, with
documented error behavior and deny(missing_docs). Respect the project complexity
limit and consolidate only repetition implicated by this required work.

Initial source ownership for required diagnostic/structural integration includes
`src/model.rs`, `src/model/common.rs`, both task model modules,
`src/validation.rs` and `src/validation/wire.rs`, plus the existing `src/lib.rs`
and test targets. Minimal caller/signature wiring in existing app/evaluation/
comparison/artifact modules is authorized when required by the checked-ownership
change; record the actual purpose before editing. This is not authorization to
rewrite scoring, change numerical results or redesign publication/CLI/reporting.
Any actual behavior/contract discrepancy outside the stated structural gate
returns to owner with a reproduction before expansion.

Keep Cargo versions/toolchain stable. Manifest/lock changes are allowed only for
a demonstrated required feature/dependency-boundary correction under the spec's
needed-features rule, using provisioned dependencies; no version upgrades or new
framework. Preserve serde_json arbitrary_precision/raw_value/std and their
accepted opaque-number/marker behavior. No suppression, lint/test bypass, fake
success, fallback, unimplemented path or weakened oracle/schema.

## Evidence scope and offline verification

T027's coverage clause means actual accepted proof for the public implementation,
conformance and structure obligations through T027. It cannot require already-
completed T028-T035 practical/operator evidence before those tasks run. Keep those
later proof obligations explicitly pending; the full433-block/AC/DoD closure stays
at T035. This clarifies sequencing, not a waiver of any final requirement.

Use the existing coverage map and accepted handoffs to connect relevant clauses,
owning code and actual test/command evidence. Do not write433 new essays, invent a
state engine or add a second task tracker. A complete Cargo list plus the passing
full suite can reconcile unchanged earlier tests; do not rerun every old filter
individually by habit. New DM/offline filters must each run nonzero. Preserve all
63 S/M/E cases and the independent fixtures/derivations.

The host has `/usr/bin/sandbox-exec`; owner verified this profile executes:
`(version 1) (allow default) (deny network*)`.
Run the public test suite with prepared locked dependencies and process-local
network denial, without changing host network settings or touching external
services. Use Cargo offline mode for that run. The CLI test uses Cargo's built
executable and saved public synthetic inputs, with no credentials/global-installed
binary/private-data dependency. Do not weaken the sandbox or call an ordinary
online run offline if a real access dependency is discovered.

## Final commands, review and tracking

Run serially using existing targets:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline
git diff --check
```

Every command must exit0. No staged Clippy exception remains at this boundary.
Keep source/schema/fixture hashes, exact inventory, per-diagnostic dispositions,
module/type review and offline evidence in the complete handoff. Preserve the
unborn/untracked workspace; no staging, commit, cleanup of unrelated outputs or
private bundle changes. Retain the existing caffeinate PID84732 hold.

When the independent verifier role is free after T028, provision a fresh Sol-high/
fork-none read-only T027 verifier. Give it the exact frozen candidate and original
requirements, not instructions to agree. It independently checks genuine structure,
every DM row, public proof/inventory and clean/offline gates; no optional redesign,
duplicate framework or private-data audit. One bounded finding-driven repair and
focused recheck are authorized within this scope; unresolved repetition returns
to owner. Reuse unchanged evidence where it actually applies.

Coordinator owns Fizzy, index/accepted-evidence/session updates and actual file
ownership routing. Search/update the appropriate General-board card, keep its
first incomplete step accurate and verify rendered notes. Serialize shared docs
with T028 review artifacts. Index every new artifact, verify links, run verify-plan
and diff checks, and return a complete combined handoff plus independent verdict
for owner acceptance. Do not start T029+ automatically.
