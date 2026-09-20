# Planning completeness audit

Date: 2026-09-18. Authority: **Reference — planning verification only**.

The earlier work-package plan was insufficient for agent dispatch. This revision
was reviewed against the referenced decomposition protocol and the exact local
`1.2-draft` specifications. It provides individual task contracts, concrete
interfaces and artifact owners, causal dependencies, independent verification,
and source-level traceability. No Validator implementation or conformance test was
run or claimed by this audit. This is the planning author's audit, not an
independent subagent verdict.

## Coverage examined

- 35 task contracts with exact artifacts, prerequisite reasons and produced input
  artifacts, acceptance criteria, named tests or explicit verification procedures.
- 433 source blocks cover the governing portions of both specifications. Each
  block binds its exact source lines/hash to implementation and verification tasks.
  This count includes context/examples and is not a claim of 433 independent
  normative requirements. Verifiers must check each normative clause within a block.
- All 63 conformance-table rows have stable IDs and exact named tests: 29
  single-label/shared, 21 multi-label, and 13 evidence-contract cases.
- All 12 structure checks, eight ACs and five DoD clauses have individual mappings.
- The physical map contains 77 exact planned artifact paths. Each module/flow edge
  in the capability map has a producer, consumer or integration task.
- The graph contains 67 justified dependency edges and a valid topological order.
  Independent source/oracle work does not depend on production scoring.

## Specific gaps corrected during review

| Gap or hazard | Correction |
|---|---|
| Work packages left decomposition to agents. | Individual tasks bound the write scope, interfaces, tests, expected evidence and responsibility. |
| Conformance mapping was deferred until implementation. | Every table row is mapped now; compound rows explicitly require all subcases. |
| Early tasks could inherit a compound test requiring future implementations. | Local tests remain with implementers; each full conformance/structure case has an explicit later audit test owner. |
| The old phase diagram included a backward failure edge in a purported DAG. | Construction graph is acyclic; rework is an acceptance procedure. |
| Logical parallelism could permit collisions in shared source/test files. | Reverse artifact ownership identifies shared writers; coordinator must isolate or serialize edits. |
| Aggregate green tests could hide absent filtered tests or mismatched source revisions. | Nonzero test inventory and verified input/source/output hashes are mandatory in handoffs. |
| Preparation and expected values could share the same faulty code. | T030 consumes frozen native sources independently of preparation and production scoring; fresh acceptance cannot write scoring code. |
| Schema success-only checks could validate an incomplete contract. | T024 mutates required fields/types/variants and audits all nine schemas against source requirements. |
| Steel-thread evidence could be mistaken for release evidence. | Separate T017 architecture, T027 public conformance, and T035 full DoD gates. |

## Repeatable planning checks

Run from the repository root:

```sh
ruby docs/plans/validator/verify-plan.rb
git diff --check
```

The plan verifier checks task contract fields, references, acyclic ordering,
source file/block hashes, continuous source-line coverage (excluding navigation,
blank lines and reference bibliographies), case test ownership, reverse task and
artifact mappings, local document links, and artifact-index membership. It is a
small repository-local structural validator, not tasker's schema/CLI integration.

Because the repository starts with untracked content, also check the authored
files directly for whitespace errors; `git diff --check` alone sees no untracked
file changes. Do not stage pre-existing untracked files simply to make the audit
look like a tracked-code diff.

Executed in this planning revision: `ruby docs/plans/validator/verify-plan.rb`
returned `PASS` for all checks above; `git diff --check` passed; a direct
whitespace/conflict-marker scan passed on all 45 authored/updated planning and
documentation files. Inspection of `Cargo.toml` and `src/main.rs` confirmed the
unchanged empty-dependency greeting scaffold. These are planning checks only.

## Authority and limits

All task contracts and design proposals remain Proposed. Mapping a source clause
does not demonstrate its future implementation or prove that a planned test is
sufficient. Independent execution reviewers must compare actual implementation and
assertions against every assigned clause; accepted evidence is recorded in the
future `docs/acceptance/validator-v1.md`, while task state stays in Fizzy.

Source hashes must be explicitly refreshed after reviewed specification/authority
changes. Internal signatures can be chosen within prescribed boundaries; producer
tasks settle nested serialization in concrete types and schemas before consumer
work. Any choice that changes observable semantics returns to the owner.

No development team has been dispatched, no implementation tasks have been
activated, no private acceptance source has been frozen, and no model inference
has been performed. The sole active outcome in this session is this planning revision.
