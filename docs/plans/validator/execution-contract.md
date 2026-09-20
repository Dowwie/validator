# Validator agent execution contract

Authority: **Ratified**. This is the shared part of each [task contract](tasks/T001.json).
Implementation is authorized. The [build charter](../../dev-team/validator-build/charter.md)
applies the manage-dev-team role boundaries: only the developer writes implementation,
tests and acceptance fixtures; the coordinator and verifier do not. Task files alone
do not establish assignment, acceptance, or completion.

## Readiness and bounded ownership

The coordinator selects one task whose dependency responses have been read and
accepted on an identified integration revision. The developer receives the task
JSON, this contract, linked specification sections, assigned source-block IDs from
[coverage.json](coverage.json), and the concrete artifacts produced by dependencies.
Do not delegate an entire phase or a package described only by its title.

Task effort ranges are planning estimates in engineer-hours, not runtime budgets
or a delivery promise. Each task targets one 2–6-hour sitting and one coherent
responsibility. If a task cannot fit its range, stop before expanding it and split
only the affected task into bounded children with preserved requirements and
causal edges. Do not spend the rest of the project's budget without reassessment.

The artifact list is the task's behavior write scope. Required parent-module
declarations/re-exports and dependency manifest/lock updates are also allowed by
the build charter's narrow integration rule; record each change and its necessity.
Read relevant existing code before
editing. Shared-file changes require the coordinator's integration order. Tests,
`Cargo.toml`/`Cargo.lock`, dispatch records, acceptance records, and the artifact
index are shared too; independent logical tasks are not automatically safe to
edit concurrently. Use isolated worktrees for independent changes or serialize
shared-file writers. Never add a false dependency merely to conceal an edit conflict.
The [physical map](physical-map.json) identifies every shared writer.

## Interface and schema discipline

The task's inputs, outputs, invariants, and dependency artifacts form its interface.
The two specifications override task summaries. Both task kinds instantiate shared
records; only closed concrete dispatch is permitted. No public mutable models,
wire deserialization directly into validated objects, dynamic metric maps,
implicit observation promotion, or unvalidated replay shortcut is acceptable.

Resolve internal function signatures within the specified ownership boundaries.
When nested public serialization details are not fixed by the source specification,
the owning producer task defines concrete Rust fields and JSON Schema together,
with an example and negative schema test, before consumer tasks begin. Record
that choice in session notes. If it changes an observable rule, task meaning, or
required field rather than merely specifying its representation, return it to the
owner as a contract decision; do not silently ratify a different spec.

Dependent agents consume the accepted schema and types. They cannot invent aliases
or alternate shapes. Required fields cannot be left for a later task to infer.
Early implementation commits are unfinished v1 work: no stub or permissive mode
may return a successful result for behavior that has not been implemented.

## Developer evidence

Implement the assigned behavior and its local regression tests in the same change.
Do not defer tests until the later audit tasks. The audit tasks own the full compound
conformance cases and independently check the developer's proof; their existence
is not permission to postpone local verification.

Before handing back a task:

1. List each acceptance criterion and assigned source clause, its implementing
   location, and the actual test or review evidence.
2. Capture the named tests with `cargo test --locked --lib -- --list` or
   `cargo test --locked --test conformance -- --list` / `--test cli -- --list`.
   Confirm each expected test exists and its filtered command executes the expected
   nonzero count. A successful zero-test command proves nothing.
3. Run the relevant pinned checks, retaining full command/exit output:

   ```sh
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features --locked -- -D warnings
   cargo test --all-features --locked
   cargo build --release --locked --bin validator
   ```

   These are mandatory at the steel-thread, integrated-conformance, and final
   delivery gates. At intermediate tasks, run focused tests and format/lint/build
   appropriate to the actual change; previously accepted checks remain evidence
   only for unchanged code. Dependency additions update the lockfile explicitly.
4. Record exact source revision/tree or file hashes (the initial repo is untracked),
   toolchain, fixture/oracle hashes, produced result hashes, and all failures.
5. Update the artifact index for every new document/schema/accepted input/oracle
   or evidence artifact. Record durable decisions in session notes; keep next
   actions and task state only in Fizzy.

Expected numbers must be independent of production scoring. A fixture is not
independent if its expected.json was generated by the evaluator under test.
Validate ratios, statuses, units, populations, ordering, and per-episode evidence,
not only headline values. Do not change or delete a valid oracle to make a test pass.

## Independent verifier and integrator

The verifier must be a different agent from the implementation author. Supply the
specification and task contract before the implementation summary. The verifier
reads the change, reruns the named checks, checks every acceptance criterion, and
looks for prohibited shortcuts at the affected boundary. The saved response states
`Ready`, `Revise`, or `Blocked` with criterion-level evidence and precise locations.
The verifier never fixes implementation or fixtures; corrections return to the developer.

The integrator reads the complete response file, checks that the verified source
hashes equal the integration inputs, and integrates only accepted changes. Shared
file conflicts are resolved against both tasks' criteria, with affected tests
rerun after resolution. A green developer report alone does not close a task.
No implementation task is complete merely because its code compiles.

Owner-authorized coherent assignments may contain an explicitly enumerated causal
sequence when a later task provides the real public entry point needed by earlier
integration evidence. Inside such an assignment, the developer implements and
locally checks each prerequisite before its consumer. Every constituent task keeps
its artifacts, interfaces, named tests, expected results, and criterion evidence;
none is independently verified or accepted until the whole hash-bound candidate
receives one per-task verifier verdict and owner decision.

For the first steel-thread tranche, T004 -> T005 -> T006 is the checked-admission
sequence. After its acceptance, T007 -> T008 -> T009 -> T010 -> T011 -> T012 ->
T013 -> T014 is the single-label scoring/publication/application sequence. The
second sequence places named conformance and CLI integration tests through T014's
real small application API. It must not expose private internals, include production
source into tests, create a test-only facade, add placeholders, or claim a filtered
test before it exists. Clean format, all locked tests, and the locked release build
are mandatory at the T014 boundary. T014 and T017 also run the exact unmodified
Clippy command with `-D warnings` and save its real status and symbol inventory.
Only the owner-enumerated private `dead_code` contracts awaiting their ratified
T015-T021 consumers may remain temporarily unresolved; no other warning class,
lint bypass, suppression, artificial consumer, or widened export is accepted. The
complete warning-free Clippy gate remains mandatory at T027 and T035.

Within the T007 -> T014 sequence, implementation handoffs use the atomic task
units T007, T008, T009, T010, T011, T012, T013, and T014 in dependency order.
The coordinator advances automatically after each complete local handoff without
an intervening independent review or owner decision. These are implementation
milestones only: the sequence still freezes once after T014 for one combined
independent review and owner acceptance, with every task's final named evidence
and criteria reconciled through the real application API.

After owner acceptance of the repaired T007-T014 candidate, T015 -> T016 -> T017
forms the remaining backbone sequence. T015 owns verified replay and inspection;
T016 owns identical-population single-label comparison and may make the smallest
`artifacts.rs` change needed to reuse the existing atomic no-replace publisher for
`comparison.json`; T017 owns the fixed independent steel-thread fixture and gate
evidence. Each task has one atomic implementation handoff and automatic local
advance. The sequence freezes once at T017 for one combined independent review and
owner decision. T018 is prohibited before that acceptance.

The architectural gate is T017. Multi-label expansion depends on its acceptance.
The integrated public gate is T027. Practical acceptance depends on it, frozen
independent inputs/oracles, and tested operator documentation. T035 checks the
entire DoD after both fresh-agent workflows. A gate failure changes the affected
Fizzy state and resumes at the responsible task; it is not an edge that makes
the static construction DAG cyclic.

## File-based dispatch and response

For every developer, verifier, oracle, or acceptance dispatch, write a complete
numbered prompt under `docs/dispatches/validator-<task-id>/` before sending it.
The prompt includes:

- Task ID, outcome, exact scope, requirements, interfaces, dependency artifact
  locations/hashes, common constraints, and allowed write paths.
- Named checks, expected results, role restrictions, and acceptance criteria.
- Requested model/reasoning only if explicitly selected by the owner or applicable
  instructions; context-inheritance setting and mandatory response-file path.
- Relevant Fizzy canonical URL and a prohibition on unapproved scope/delegation.

The runtime message points to the absolute prompt path. Every changed instruction
gets a new numbered prompt; preserve earlier prompts and responses. Responses save
the complete findings, implementation handoff or verdict before returning their
path. Index both files and link them from Fizzy. Failure to produce a response is
recorded as failure, never reconstructed as an invented successful review.

Fresh acceptance dispatches T033/T034 use the named paths in their task contracts
and **no inherited implementation conversation** (`fork_turns="none"` when available).
Supply only installed binary/version/hash, operator documentation, schemas, frozen
source manifests/inputs, independently produced expected results, and the complete
acceptance assignment. They may author deterministic preparation; they must not
read implementation or write scoring code. Give no hidden implementation hints.

## Coverage and actual evidence

[coverage.json](coverage.json) pins source files and each covered source block.
`S01`–`S29`, `M01`–`M21`, and `E01`–`E13` are the three conformance tables in
source order. `DM01`–`DM12` are shared-model checks; `AC1`–`AC8` and `DOD1`–`DOD5`
retain their source meanings. `V…` and `D…` identify other main/model source blocks.
These IDs and test names are planning contracts, not claims that tests exist.

For compound rows, test every listed subcase. An invalid-UUID assertion alone does
not satisfy a row that also requires duplicate IDs/keys, unknown labels/fields,
missing input, and partial signals. For each other block, review every normative
sentence, field and formula; contextual or illustrative prose does not create a
new requirement. Source section bibliographies impose no additional v1 features.

During implementation, `docs/acceptance/validator-v1.md` stores actual clause/case
proof: source ID, task, implementation location, test/review ID, command, expected
and actual result, exit code, source/input/output hashes, and accepted response.
No blank proof counts as coverage. Execution state stays in Fizzy. Plan JSON has no
owner/status/percent-done ledger; `evidence_state: planned_only` describes its
unchanging authority as a proposal, not live implementation state.

Run `ruby docs/plans/validator/verify-plan.rb` after changing a task or mapping.
Source changes fail its hash check; re-review affected clauses and update mappings
explicitly. Plan validation proves structural completeness and reference integrity,
not correctness of future implementation or sufficiency of an agent's judgment.

## Accepted T017 gate and multi-label core handoffs

The owner accepted the frozen T017 architecture gate at manifest013 and Ready
verdict014. T018 -> T019 -> T020 now forms one coherent multi-label core sequence
with atomic local handoffs and one combined independent review after T020. A fresh
Terra-high/fork-none developer context owns each substantial task; prior writers
are retired before the next context starts. T021 is prohibited before owner
acceptance of the T020 checkpoint.

T018 completes private checked multi-label admission and its owning-library tests.
T019 completes hard accounting, independent expected results and owning-module
tests. The T019-named conformance filters depend on T020's real application path:
they must exist and run nonzero at T020, and T019 cannot substitute a facade,
public internal export or zero-selected filter. T020 completes marginal losses and
bins, closed check/evaluate/inspect/replay integration, actual T019/T020
conformance/process cases, and strict multi-label alternatives in the check,
report and inspection schemas. Receipt and error remain task-neutral unless a
demonstrated mismatch requires change.

## T024 schema-audit role resolution

Owner acceptance of repaired T021-T023 manifest013 and Ready disposition017
authorizes T024 only. T024's permanent schema and test changes belong to one fresh
Terra-high/fork-none developer as sole writer. A different fresh
Sol-high/fork-none verifier reviews the frozen T024 candidate read-only and returns
Ready/Revise/Blocked. This resolves the task's historical verifier-author metadata
without changing any product criterion, artifact, case assignment or verification
obligation. T025 and later tasks remain prohibited until owner acceptance of T024.

## T025 numerical-oracle role resolution

Owner acceptance of repaired T024 manifest008 and Ready verdict009 authorizes T025
only. Permanent independent oracle fixtures and conformance tests belong to fresh
Terra-high/fork-none sole-developer contexts, one complete local output at a time.
A different fresh Sol-high/fork-none verifier independently derives the mathematics
and reviews the frozen result read-only. Oracle authors must not read or reuse
production scoring implementations to derive expected values. This resolves the
task's historical verifier-author metadata without changing any product criterion,
case assignment or numerical convention. T026 and later remain prohibited until
owner acceptance of T025.

## T026 integrity-checkpoint role resolution

Owner acceptance of final T025 manifest020 and Ready verdict021 authorizes T026
only. Permanent artifact-integrity production corrections and tests belong to
fresh Terra-high/fork-none sole-developer contexts, one complete local output at
a time. A different fresh Sol-high/fork-none verifier reviews the combined frozen
T026 candidate read-only. This resolves the task's historical verifier-author
metadata without changing any integrity criterion, hostile case or final gate.
T027 and later implementation remain prohibited until owner acceptance of T026.

## Conditional T028 source-freeze window

T028 may start only after the complete T026 candidate is frozen and its independent
read-only review is active. The otherwise idle sole-developer slot may then freeze
the protected acceptance sources without changing the T026 candidate. T026 retains
priority: any finding-driven T026 repair waits only for the current atomic T028
copy/hash/checkpoint and then reclaims the sole writer. T028's independent review
waits until the verifier role is free. This conditional window does not authorize
T027, T029 or later work and does not imply T028 acceptance.
