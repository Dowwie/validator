# Session decisions

The latest design candidate is specification `1.2-draft` and its shared data model.
Earlier entries record the narrower design's history; they do not ratify the new
task semantics or wire format. Operational work remains in Fizzy.

## September 18, 2026

- Validator is a Rust CLI for people and agents evaluating single-label classification outputs, including a scored-choice profile for outputs with a selected label, probabilities, and reported confidence.
- The canonical golden record is an episode with `id`, `expected`, and required opaque `input`. No domain schema or input-description mechanism is required.
- Episode IDs are stable UUIDs. Generate UUIDv7 for new episodes during dataset preparation, preserve existing valid UUIDs, and reuse assigned IDs across transformations and runs.
- Agents can author deterministic transformation scripts. Transformations preserve identity and make exclusions explicit; Validator does not infer source field meanings or repair labels.
- Expected labels, reported choices, class probabilities, reported confidence, final decisions, and statistical uncertainty are distinct concepts.
- Reference datasets are fixed for an evaluation. Corrections produce dataset revisions. Runs preserve exact episode membership and per-episode evidence.
- A run is a recorded evaluation. Iteration and refinement pass describe the surrounding work; they do not require separate entities in v1.
- The user requested a validation-only specification. [Validator v1](docs/specs/validator-v1.md) was initially proposed for review and subsequently ratified as recorded below; it does not claim implementation.
- Reports are machine-first, with agents as their primary consumers. JSON artifacts and structured CLI results are required; human-oriented text reports are not part of v1.
- V1 uses saved canonical predictions, fixed diagnostic bins, explicit threshold policies, and strict run comparability. It leaves statistical inference and general acceptance gates outside this version.
- The original master specification remains reference material. Documentation drafting follows the Google developer documentation style skill; the ponytail skill keeps the product boundary small.
- [Fizzy card 180](http://localhost:3006/1/cards/180) tracks the specification-writing outcome. Operational task state remains there.

## Adversarial review disposition

The owner requested that review feedback be assessed and incorporated in one pass,
without creating separate issues. The [Astra-high review](docs/dispatches/spec-adversarial-review/001-reviewer-to-owner.response.md)
records the review of `1.0-draft`, with the later terminology edit disclosed below.
Incorporating the feedback produced
`1.0-draft.2`, which remained Proposed until the owner ratification below.

| Feedback | Disposition and rationale |
|---|---|
| Mixed-pass source attribution | Accepted. A source map and per-prediction `source_id` preserve the existing assembled-pass workflow without duplicating configuration on every row. Reports explicitly identify mixed-source populations and retain source counts. |
| Evidence relocation | Accepted. Bind each source evidence-array entry to a deterministic copied path and digest. Replay uses the binding, never the original path. |
| Empty-intersection statuses | Accepted. Restrict populations without erasing each source run's signal applicability. Applicable empty metrics use `no_data`; absent families use `not_applicable`. |
| Unresolved-reference guarantee | Accepted. Limit validator guarantees to observable structure and vocabulary. Reference settlement remains a preparation responsibility. |
| Receipt digest target | Accepted. Receipts name and hash the exact result file, not its directory. |

Also incorporated the bounded numerical guidance: scoring arithmetic uses binary64,
while inspection preserves opaque JSON numeric values. Existing choices about
all-or-nothing input validity, complete signal families, exact-byte dataset identity,
fixed-schema macro-F1, and the limited metric scope remain unchanged. The revision
adds acceptance cases for the clarified contracts; it does not add a new subsystem.

[Fizzy card 181](http://localhost:3006/1/cards/181) records the review and its
incorporation. No separate finding cards were created.

## Owner ratification

On September 18, 2026, the owner reviewed the material differences from the earlier
master specification, judged them a net positive, and explicitly ratified the
Validator specification. Version `1.0-draft.2` became ratified version `1.0` without
changing its technical requirements. This contract governs v1 implementation;
the master specification remains Reference material.

The ratification covers the revised scope, data and metric semantics, machine-first
reports, and incorporated adversarial feedback. It does not assert that implementation
or implementation acceptance has occurred. No implementation work was started as
part of recording this decision.

## Explicit CLI and Rust organization

The owner requested that developer agents not be left to choose project organization.
Specification `1.0.1` reinforces the existing CLI product boundary and adds a required
single-package binary/library layout, named module responsibilities, dependency
boundaries, test placement, committed lockfile, pinned toolchain, and exact build
checks. The library supports the executable and testing, not a separate SDK product.
The Rust best-practices skill informed these constraints. No data, metric, or command
contract changed, and no implementation files were created or modified.

## Provider-neutral naming

At the owner's request, specification `1.0.2` replaces the provider-named source
kind with `scored_choice`, without a legacy alias. The profile keeps its required
question ID, complete distribution, reported confidence, and maximum-probability
selection check. Generic `classifier` inputs keep their existing rules. This names
an output contract, not a provider, and preserves the useful distinctions between
recorded choice, probabilities, confidence, and threshold rejection.

Provider-specific model naming was also removed from these notes and the historical
review dispatch and response. Those review artifacts are terminology-edited copies,
not byte-identical originals or new reviews. Their findings and verdict are retained;
the recorded specification digest still identifies the original review target.

## Shared core and multi-label scope

The owner requested that the specification catch up with the broadened design,
with priority on reusable data structures. The expanded candidate supports
single-label and complete multi-label classification. Binary tasks remain a
two-class single-label case. Count prediction and object matching are excluded.
The water-fixture example concerned per-label multiplicity within a time window;
its minimum-count matching was valid for that objective, not flawed merely because
it was called optimistic. It is not the core matching rule for all tasks.

[The shared data model](docs/specs/validator-data-model.md) separates reusable episode,
source, population, artifact, and metric-result structures from concrete target,
probability, policy, and result types. Checked construction precedes scoring;
categorical distributions and label marginals are different types. No task plugins,
count variants, partial-label masks, or universal confusion representation are added.

The draft uses wire version 2 because task definitions, typed targets, and tagged
probability evidence change the public format. It rejects the earlier format rather
than guessing or silently migrating it. Multi-label thresholds select labels;
single-label rejection produces abstention. Whole-episode abstention is distinct
from an answered empty set. Multi-label binary metrics use answered episodes and
must disclose coverage; exact-set accuracy also measures the full selected population.

These implementation details are proposed, not newly owner-ratified. The expanded
contract preserves the existing CLI/artifact safeguards and adds task-specific
conformance cases and a practical acceptance workflow. The historical multi-label
document is indexed as Reference and left unchanged. Rust best-practices informed
the checked type boundary; documentation guidance kept authority and normative
requirements explicit. No implementation files were changed.

## Final evidence and acceptance reconciliation

The owner ended design research and requested a bounded specification update.
Version `1.2-draft` incorporates the final scope: evaluate classifier outputs, not
surrounding applications or platforms. The first release still supports exactly
single-label and complete multi-label tasks. Scalar and auxiliary observations
are supporting evidence, not new regression/ordinal tasks or a rule engine.

Predictions retain required outcomes separately from named typed observations and
scoring signals. Explicit abstentions can carry signals; their probabilities remain
scorable, while confidence-versus-choice bins exclude raw abstentions and disclose
the population. The scored-choice profile still requires an actual selected class.

Numerical handling stays strict and all-or-nothing for submitted scoring inputs.
Observation vectors preserve displayed values without automatic normalization or
promotion into metrics. Scalar/mean disagreements do not silently replace native
outputs. Deterministic numerical interpretation and scalar-to-class conversion
belong to preparation, with typed descriptors and bound source/script/receipt
evidence. Observation-only artifacts are an explicit input contract, not fallback
after failed validation. No new tunable tolerance or per-row salvage mode was added.

The specification now names AC1 through AC8 and a project definition of done.
Practical acceptance uses frozen Chord native outputs: account for 630 source
episodes, score 603 labeled references, and document 27 withheld/unscored cases.
Preserve reference limitations and independently verify counts; neither historical
headlines nor model improvement are acceptance oracles. A synthetic multi-label
bundle completes the second task's practical checks. A fresh agent must use the
installed CLI without implementation access or custom scoring code.

The documentation skill kept requirements and completion evidence explicit;
ponytail limited the amendment to evidence types, preparation provenance, and the
existing CLI. The document version changed, but unreleased wire version 2 remains.
Publication does not itself ratify new details or claim the implementation passes
acceptance. No implementation files or external source artifacts were modified.

## Steel-thread planning boundary

The owner clarified that this session is for writing a plan, not building the
utility. The [delivery plan](docs/plans/build-validator.md) proposes a single-label
end-to-end artifact/replay steel thread, followed by multi-label extension, complete
conformance, and independent practical acceptance. It uses the referenced task
decomposition protocol for physical artifact mapping and causal dependencies.

The plan and both `1.2-draft` specifications remain Proposed. No implementation,
baseline ratification, source-data freeze, or model inference is authorized by
publishing the plan. The steel-thread gate is architecture evidence, not project
completion; the full definition of done remains the release gate.

## Agent-ready decomposition revision

The owner judged the ten work packages insufficient for a development team of
agents and requested decomposition that supports correct implementation and
verification against the specifications. The revised
[delivery plan](docs/plans/build-validator.md) provides 35 bounded task contracts,
a capability/physical map, a causal DAG, and source-pinned coverage for every
conformance case, structure check, AC and DoD clause. Independent verification,
shared-file coordination, complete file-based handoffs, and nonzero named-test
execution are explicit acceptance conditions.

The original proportional-effort interpretation compressed the plan too far and
left required task-definition and coverage work to future implementers. The
revision makes that work concrete now. Task contracts are proposed inputs to
future dispatches; Fizzy remains the operational source of truth. This correction
does not authorize building, ratify either specification, freeze private source
inputs, or dispatch a development team.

## Owner-authorized team build

The user instructed the owner to use manage-dev-team to build Validator against
the specifications and plan, supervise the coordinator periodically, and prevent
unnecessary expansion, especially verifier drift. This authorizes the reconciled
`1.2-draft` specifications and existing 35-task plan as the implementation baseline.
Their technical contracts and wire version remain unchanged; authority prose and
source-coverage hashes were updated without moving source clauses.

[The build charter](docs/dev-team/validator-build/charter.md) applies an Astra-high
owner, Sol-high coordinator, Terra-high sole implementation writer and independent
Sol-high read-only verifier. The initial dispatch covers T001–T017 and returns to
the owner before multi-label expansion. Owner check-ins, criterion-bound findings,
one coordinator repair cycle before reassessment, and read-only verification keep
the approved work bounded. Audit tasks do not authorize a second test/code writer.
The existing untracked repository is preserved and review candidates use exact
file hashes; implementation evidence remains distinct from planning checks.

## Admission task routing correction

During the first build tranche, the coordinator identified missing `src/lib.rs`
write scope for T002 module wiring. Owner review also identified that T003's
Prediction record depended on T004's ObservationSet and that prohibiting manifest
changes would encourage custom UUID parsing. The owner authorized required module
and manifest integration, explicitly added the T002/T003 paths, and moved complete
Prediction construction to T004. Task/input/physical/coverage maps were reconciled.
No product requirement, numerical rule or acceptance criterion was weakened.

## Application API checkpoint routing

The owner authorized two exact ordered assignments after T002: T004 -> T005 ->
T006 for checked admission, then T007 -> T008 -> T009 -> T010 -> T011 -> T012 ->
T013 -> T014 for pure scoring through real check/evaluate publication. The latter
sequence resolves the dependency between early conformance cases and the small
application API introduced by T014. Integration tests must use that API; they must
not expose internals, include production source, or depend on a test-only facade.
All constituent task criteria and named tests remain required, with one frozen
review and owner decision per combined candidate. The T014 boundary requires clean
format, complete locked tests, and release build. T014 and T017 run the exact
warning-denied Clippy command and save its real status; only the owner-enumerated
private `dead_code` inventory awaiting ratified T015-T021 consumers may remain
staged. No other warning, suppression, artificial consumer or expanded export is
accepted. Full warning-free gates remain mandatory at T027 and T035. T015-T017
remain the separate architectural gate.

## Intermediate lint sequencing correction

T014 produced real check/evaluate API and process paths while warning-denied Clippy
identified accepted private contracts whose first production consumers are in
T015-T021. Reordering those tasks or deleting/recreating the contracts would add
risk without changing product behavior. Owner dispatch 028 therefore corrects the
intermediate gate: T014/T017 retain exact Clippy execution and exact residual-symbol
reconciliation, but only the enumerated staged `dead_code` set may remain. The full
warning-free gate is unchanged at T027/T035. DTO version/tag guards remain strict,
opaque inputs remain faithful for inspection, and no suppression/fake use/public
API expansion is permitted. T014 still must close all specified process evidence.

## Accepted scoring/application checkpoint and remaining backbone

The owner accepted repaired T007-T014 manifest036 after independent Ready verdict037.
Both review defects were repaired: check and report share truthful non-scoring
normalization diagnostics, and the report schema enforces a coherent minimum-two
single-label vocabulary. The accepted checkpoint remains explicitly not warning-
free: only the exact owner-staged 26 production `dead_code` diagnostics remain,
with mandatory clean gates at T027/T035.

The authorized remaining backbone is T015 -> T016 -> T017 with one atomic local
handoff per task, automatic advance, and one combined frozen independent review at
T017. T015 adds contained verified replay and explicit inspection; T016 adds
identical-population single-label comparison and minimally reuses the safe publisher
for `comparison.json`; T017 supplies the fixed independent four-row steel-thread
oracle and architecture-gate evidence. No T018 work begins before owner acceptance.

## T007-T014 construction-boundary clarifications

During the accepted check/evaluate sequence, the owner clarified three staging
boundaries without changing requirements. T010 input-schema conformance may invoke
the schema engine directly because its multi-label schema alternatives deliberately
precede T018 runtime support; scoring/report conformance still uses the real T014
application API. T009 signal-population evidence covers as-recorded answers and
abstentions, while threshold-rejected answered-row preservation remains in T021's
decision-policy boundary case. Published run directories contain only the specified
`golden.json`, `predictions.json`, `config.json`, optional `evidence/`, and
`report.json`; manifest metadata is inside the report, and schemas remain published
under `schemas/v2/` rather than copied into each run.

## T010 input-schema dialect

The three canonical input schemas use JSON Schema Draft 2020-12 with the exact
`https://json-schema.org/draft/2020-12/schema` URI. They use only local
`#/$defs/...` references and are tested directly with `jsonschema` 0.37.4, which
allows both task-shape alternatives to be checked before multi-label runtime
support exists. The schemas close wire objects and tagged variants while leaving
cross-document vocabulary equality, source binding, digest equality, duplicate
serialized keys, alignment, categorical sums, and preparation-index bounds to
runtime validation. The prediction schema constrains source map keys through the
same nonblank definition used by source IDs and other nonblank contract strings.

## T017 architecture gate accepted; multi-label core authorized

On 2026-09-19, the owner accepted T015-T017 manifest013
`46dac241d71c838ce1251ae9ed7914798c84009eeb937a7577f5c16da735c5c3`
and independent Ready verdict014
`12019ccc02925b73697686257587c7ea99abdf847c72cc44068056bd90e40a82`.
This accepts the single-label replay/inspection/comparison architecture and fixed
steel thread. It does not claim release, multi-label correctness, full acceptance
or project completion. Clippy still has the explicitly staged 24 diagnostics;
T027/T035 remain warning-free gates.

T018 -> T019 -> T020 is authorized as one atomic multi-label core sequence with a
fresh Terra-high/fork-none writer for each task and one combined review after T020.
T021 remains blocked until owner acceptance. T019 owns hard accounting,
independent oracles and local module tests. Its two real application conformance
filters are completed at T020; no facade, public test export or zero-selected
result is valid interim evidence. T020 also owns real multi-label app/replay/CLI
integration and strict check/report/inspection schema alternatives.
## T018-T020 multi-label core accepted; T021-T023 authorized

On 2026-09-19, the owner accepted repaired manifest025
`e26438bd371f261e5d91a05d55312bfa1d329db75dc4e24e761d75d6bd646f11`
and Ready verdict026
`7e3885c40aced13a3822d1b3d3564c292035821c0902da3970923ee68f84cae2`.
Both task kinds now share verified admission, scoring, immutable publication,
contained replay and inspection with strict concrete schemas. The repaired
multi-label source boundary rejects every declared non-classifier source while
preserving unused classifiers and legal single-label scored-choice sources.

The accepted checkpoint retains exactly 20 staged private `dead_code` diagnostics;
it is not warning-free or a release. The owner authorized T021 policy execution,
then T022 typed multi-label comparison, then T023 validated intersection and
recomputation, with fresh sole writers, atomic local handoffs and one combined
independent review before T024.

## T021-T023 policies and comparison accepted; T024 authorized

On 2026-09-19, the owner accepted repaired manifest013
`2cd39b7c27d9b0408ff0903441f3afa975616edf3644f3e502f560d6d26f0bc7`,
superseding Ready disposition017
`0f44bdf19bc2834e563b402b60f2f9669110e452801b7f0da55620d0491b4dcc`
and coordinator handoff018
`1636774148047212a4f4bf28b9d4d55dbcec24c5807f15e98d9289bcf3a78c3b`.
The accepted checkpoint includes both explicit policies, typed comparison for both
tasks and validated intersection recomputation for unequal, equal and empty
populations. Owner decision015 records the exact division of conformance, CLI and
independent process evidence without requiring duplicate assertions.

The accepted checkpoint retains 17 staged production Clippy diagnostics plus
three matching lib-test duplicates. It is not warning-free or a release. T024 is
the only authorized next task: one fresh Terra-high sole writer owns necessary
nine-schema/test changes and a fresh Sol-high verifier reviews the frozen result
read-only. T025 and later tasks remain paused pending owner acceptance of T024.

## T024 schemas accepted; T025 numerical conformance authorized

On 2026-09-19, the owner accepted repaired T024 manifest008
`91d756bbb4253a832461c2e3642a585c00286caf471b3776abf5af8325b5e0d3`,
Ready verdict009
`b26aafa786843511a492bd7ee47aa1f58a06299b3431aa4227b09120c42eb543`
and coordinator handoff010
`e4ee78a0c081d8289a9f6b2491bb09e944d8ce3af50c78a9bec7685e531dee09`.
All nine schemas and assigned T024 evidence are accepted. The exact 17+3 staged
Clippy inventory remains and the checkpoint is not release/full-product acceptance.

T025 alone is authorized in three fresh sole-writer outputs: independent exhaustive
fixtures/oracles; all assigned single-label/evidence rows; then assigned multi-label
rows plus the full numerical umbrella and final gates. Expected values derive from
the ratified mathematics and raw synthetic inputs, never production scoring. A
fresh Sol-high verifier independently reviews the frozen result; T026+ stay paused.

## T025 numerical checkpoint accepted; T026 integrity work authorized

On 2026-09-19, the owner accepted T025 final manifest020
`deeed4c908fa385de1cd8494a31fdcc7fb8d46646e03b32766691fe007f24fc9`,
Ready verdict021
`a6ca4df6161aea825f5721097a1782b7fc520d5eb3f0febff9f1edb6db9ebc43`
and handoff022
`8719d7ead5401e2b90b3454128d4e5ff0c34ad8f1f628bb0b7b07ca77d1d86d1`.
This accepts the independent exhaustive and assigned numerical evidence plus the
bounded abstention, binary64, target-shape and per-label population-unit
corrections. It does not accept the full product or clear the staged 17+3 Clippy
inventory.

T026 is authorized in two serialized fresh Terra-high sole-writer outputs:
artifact/publication safety S23-S28 plus both matrices, then source/comparison/
preparation S15/S16/S18/S19/S20/S22, M17-M19 and E08 with final gates. A fresh
Sol-high verifier reviews one frozen combined candidate. T027 remains paused.

## Conditional T028 source-freeze window

Owner authorized T028 to start only after T026 is frozen and its read-only review
is active. The otherwise idle sole developer may then copy/hash the protected 630-
case Chord source bundle. T026 keeps priority and no two developers may run; T028
must save an atomic checkpoint before yielding to any T026 repair. T028 review
waits for the verifier role. T027, T029 and later work remain paused, and copied
private manifests are not accepted or authoritative until independent review and
owner acceptance.

## T026 accepted; T027 structural/offline gate authorized

Owner accepted T026 manifest017
`bb0ea141de203596fcd9ee3c7db8b7bf37f8a2033e7858d05074b9bf03e3cf44`,
Ready018 `b6d8ea67f0eccaaa0f3813e5e45dd557bda1bc8f553632145f2ba78c8eb94c98`
and handoff019 `c54af31f1c27670a581f2b9604c70e0db531345205a429693a76393bc70180f6`.
T027 is authorized as real checked structural integration followed by exact
DM/offline proof and one combined independent review. Its first output ends the
staged17+3 allowance. T029+ remain undispatched.

## T028 protected source freeze accepted

Owner accepted Ready004
`631bade386fe30b91b6b971bcc7cb2690e9e2626e4545ea1e16fc4dcab8d9e4b`,
handoff005 `5f99e0266761b0b5245ef16d8f4944d6c2f527324d358e403903f48541354d57`,
protected manifest `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e`
and ID map `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4`.
Frozen metadata retains its creation-time candidate status; owner006 is the
subsequent governing acceptance decision. T028 establishes preserved sources,
identities and mapping provenance only. T029 still requires owner dispatch.

## Conditional independent T030 oracle window

T030 may start only after both T027 developer outputs freeze and T027 independent
review is active. Its fresh author is isolated from Validator source, preparation
and prior expected metrics. The performance-independent targeted population is
fixed before measurement: first 50 labeled source IDs in ascending exact string
order, then persisted-UUID mapping. T029 and T031+ remain separately gated.

## Conditional T029 preparation window

T029 may start only after owner acceptance of T027 and a complete frozen T030
author handoff. It may overlap T030's read-only review but cannot read oracle code
or expected results. Its targeted selection uses the same fixed first-50 labeled
source-ID rule, and performance evaluation waits for owner acceptance of T030.

## T027 public implementation accepted

Owner accepted manifest011
`178fccf388382f34aa68c9de5cae669c7ab331e5e5612775a28a1a6c6a3639f9`,
Ready012 `dbc8eb3a4b7793e57c5fe75fbe4efa1dc3722e4562f5e1fc3377eca683e4123e`
and handoff013 `c952f82d5e153ffe27836e3b0ca05979296a6e544815dcc36c626c4407e06ec2`.
The public implementation is warning-free and passes ordinary plus deny-network
offline 159-test gates. Product source/architecture is frozen; only demonstrated
required completion blockers may reopen it.

## Accepted-source executable installed

The frozen T027 source was installed once at
`/Users/dowwie/.local/share/validator/install/t027-178fccf38838/bin/validator`,
SHA-256 `4bacbc5947bb70218725916965439837b2afa53d16a04f37ef00a31509a85dde`,
version `validator 0.1.0`. T032 and fresh operators reuse this path; installation
alone does not claim documentation, operator acceptance or final DoD.

## Acceptance-input bundle independently Ready

T029 preparation Ready verdict
`6921a1cce5e06dc94aa04e909fce2c5530e946bb343e005c8f55392a38605cfb`,
T030 native-oracle Ready verdict
`c8349c4212eb1a19295e44fb853fe459b1c9dc48e4c5c6592a0d79765d7f3b3e`
and T031 public-oracle Ready verdict
`725b741a32d5f9daee12197b77f11c022442dff59b9b1d74517cf6097752ab30`
identify the exact independently reviewed input candidates. The combined handoff
is `docs/dispatches/validator-completion/008-coordinator-to-owner.response.md`.
Performance evaluation had not run at that handoff; the owner decision below
supersedes completion002's continuation instruction.

## First-version closeout at the user's request

On 2026-09-19 the user explicitly stopped development and established the existing
installed `validator 0.1.0` as the first version. The owner accepted the exact
T029/T030/T031 input bundle in completion handoff008 after reading its substantive
review/repair evidence. The accepted T027 binary hash remains
`4bacbc5947bb70218725916965439837b2afa53d16a04f37ef00a31509a85dde` and its
159-test, clean-Clippy, release and network-denied evidence remains the build
verification basis. No fresh operator or final full DoD result is invented.

Development agents were already stopped at the input owner boundary. Remaining
documentation/usability, fresh-operator demonstrations and final DoD closure are
deferred; earlier continuation dispatches no longer authorize them. A concise
README and the current verification-record status identify this version and its
limits. No new implementation, fixture work, review cycle or test suite was run
for closeout.

The long owner-session gap showed no new macOS full-system sleep event; the
original caffeinate PID84732 was still active. The team progressed and then waited
for owner acceptance. The exact session-pause cause was not established. A stronger
task-owned hold (PID72235, session69578) was briefly started during diagnosis; both
task-owned holds are released at this user-directed stop. Other assertions and
system power settings are left alone.

## Final architecture overview

The user separately requested architecture.md as the final documentation job.
The owner used the Google developer documentation style skill and grounded the
overview in the accepted implementation and specifications. Its six Mermaid
diagrams explain system/component boundaries, admission, policy outcomes,
publication, and verified replay; prose covers state storage, metric populations,
comparison, failure behavior and acceptance limits. This documentation request
does not restart implementation or the deferred operator/release gates.

## Workflow replay planning

The user clarified that practical debugging should replace existing evaluators
with Validator in replays of previously run Chord validation workflows. The latest
instruction is to write and save the plan, not execute it. The proposed scope in
`docs/plans/validator-smoke-tests.md` combines CLI smoke checks with historical
regression/integration comparisons. It preserves original runs, reference versions,
native decision routes, and independent expectations. The user-designated archived
conversation supplies context about Chord's distinct validation workstreams; it
does not turn all of them into current assignments. Development and full DoD
closure remain deferred.

## Workflow stabilization planning revision

On 2026-09-19 the user clarified that the objective is to stabilize newly built
Validator through complete workflows. Historical evaluators are not assumed
correct. The available pool comprises dialect translation, Gold-corpus/ETL, L3
taxonomy induction, V2 adjudication, resolver validation, and deduplication/model
validation. Cases earn inclusion through distinct Validator behavior, not project
count. The revised `docs/plans/validator-smoke-tests.md` replaces the fixed two-V2
replay scope with a grounded inventory, coverage obligations, and a proposed
in-contract repair-and-rerun loop.

Planning found retained resolver Flash/Pro outputs with 250 rows and 132 historical
reference labels per arm, including eight unreadable Flash responses. These supply
a many-class candidate without claiming the historical labels are semantically
correct. Native630 retains probabilities/confidence as observations and therefore
needs separate signal/policy coverage; the existing public fixtures supply bounded
additional paths. The resolver evidence is linked from the resolution guide but
lacks exact entries in Chord's artifact index; that external index was not edited.

This instruction authorized plan revision only. No workflow was executed, no
product repair began, and no deferred full-DoD gate was resumed. The plan describes
the proposed execution scope; Fizzy retains operational state.

## Saved-evidence workflow stabilization completed

The user subsequently commissioned execution of the stabilization plan. Native630,
resolver250, and bounded public single-label and multi-label cases completed their
applicable CLI lifecycle on the installed Validator binary. The release rebuild was
byte-identical to that binary. Independent oracles, schema validation, replay,
inspection, comparison, negative integrity checks, ordinary Rust gates, and the
network-denied Cargo-offline suite passed. No Validator implementation defect or
repair candidate was found.

Resolver250 uses deterministic UUIDv7 episode IDs over the exact cross-arm row
identity. A readable null judge verdict is the explicit `RESIDUE` class. Only an
`unparseable: true` response becomes a whole-episode abstention, with the source
failure retained in evidence; placeholder confidence is not a scoring signal.
These results measure agreement with historical labels and do not ratify their
semantic truth.

`docs/acceptance/workflow-replay.md` records the bounded verdict, evidence, coverage,
commands, discrepancies, and limits. Private run evidence and its hash manifest are
under `/Users/dowwie/.local/share/validator/acceptance/workflow-replay-20260919/`.
The completion audit added resolver Pro recovered/regressed inspections and a
93-entry exact command/exit ledger before the final completion claim.
This completion does not claim production promotion, pristine-agent acceptance, or
the deferred full definition of done.
