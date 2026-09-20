# Independent T021-T023 frozen-candidate verdict

Verdict: **Revise**.

The frozen candidate satisfies the reviewed T021 policy behavior, T022 typed
multi-label comparison behavior, and the unequal-selection/empty-intersection
paths already covered for T023. One current T023 contract defect is reproducible:
an explicit intersection request is silently emitted as `identical` when the two
validated selections are equal, while the published schema cannot represent the
legal requested zero-exclusion intersection.

## Review boundary and identity

I reviewed as a fresh verifier and did not implement or modify the candidate. I
read the repository instructions and documentation authority routing, owner
prompt001, T021/T022/T023 in full, their linked normative specification and data
model sections, the execution contract, developer handoffs002/006/007 (and the
full T022 handoff005 referenced by reconciliation006), manifest008, review
prompt009, and addendum011 with owner prompt010 and coordinator acknowledgment010.

Manifest008 has the required SHA-256
`344c542a392e5387da2bff7a681dfe7f61b1f81c8975cd3e48dc78efe11a68f4`.
Every governing-handoff and frozen implementation/schema/test/dependency hash in
it reconciled before review. The same frozen hashes reconciled again after all
checks:

| Artifact | Final SHA-256 |
|---|---|
| `src/model/common.rs` | `5b91a1aaad30127f63030d885338e8d54d807d5682cf3c077525102167c0e9cc` |
| `src/model/single_label.rs` | `4ec2c89ce43f30a79bdb8054c106608257613491415710e56f3139cfbf748ea0` |
| `src/model/multi_label.rs` | `a0d6b55a7930b4acb1c9379d98b77d66eddc3386788eba3fdfb351cb28402c70` |
| `src/validation.rs` | `fb6148069b4ade27eab57917e0111211274f3aa31c45b4458d3fa1b76a5482a0` |
| `src/evaluation/single_label.rs` | `f0cad620c80d8a1b7cef1847d49179c1b028f33546d40c5fc0b38cf5f002938c` |
| `src/evaluation/multi_label.rs` | `ff11dc9c2afbb5d1387051e245fd9532a90171a8a2c0086e1ea6b0fa1e8de8f3` |
| `src/comparison.rs` | `4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845` |
| `src/app.rs` | `aeee8c0bccde849a769b1a3afc02ac626b8b23565559a28d28979e980a3c7391` |
| `src/cli.rs` | `ed6424fa545b1ccb8418cfc008abe466c7953b9ecad17b0c2f494fc2d4c06969` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `schemas/v2/report.schema.json` | `4078be102abd69034932d38bda2e0c9c68e98b9bf3f59aa64cef116867a543d3` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `schemas/v2/comparison.schema.json` | `005a099c80add634eaa46a8141a402728173436e8acdfc449e8bfb5f75524597` |
| `tests/conformance.rs` | `108cd539f5280acfc3bbb73d61cfd23ccaac03ce8545e631ff82950c02cef92e` |
| `tests/cli.rs` | `9d94d30be6161f00f08e801b93d3062a95aadd4000d6a7f5678613e3e469570d` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |

## Required revision

### Explicit intersection is downgraded for equal selections and its schema forbids the legal requested document

Requirement: the ratified comparison contract permits explicit intersection for
compatible runs and does not require different selections or any exclusion. The
requested scope must remain explicit, with deterministic compared/excluded
populations, and both concrete evaluators must run on the restricted validated
rows.

Locations:

- `src/app.rs:420-423` and `src/app.rs:482-485` enter the intersection branch only
  when `options.intersection` is true **and** the selected vectors differ.
- `src/app.rs:460-462` and `src/app.rs:522-524` therefore use
  `identical_scope` for an explicit intersection request whose selections are
  equal. That bypasses `intersection_scope`, checked restriction, and concrete
  recomputation.
- `src/comparison.rs:62-98` can correctly construct a zero-exclusion
  `intersection` scope; the application never calls it for equal selections.
- `schemas/v2/comparison.schema.json:18` independently requires at least one of
  `baseline_excluded_count` or `candidate_excluded_count` to be at least one, so
  the published schema rejects the legal zero-exclusion intersection document.

Reproduction used only temporary inputs outside the repository:

```text
ruby /tmp/validator-zero-exclusion-probe.rb
```

The probe invoked the actual built CLI for each case:

```text
validator evaluate --dataset GOLDEN --predictions PREDICTIONS --config CONFIG --out RUN
validator compare --baseline BASELINE_RUN --candidate CANDIDATE_RUN --intersection --out COMPARISON
```

For each task, the nonempty configuration used the same one-element
`episode_ids` array on both sides; the empty configuration used
`episode_ids: []` on both sides. Golden digest, task, ordered vocabulary, role and
numerical semantics were compatible. Single-label inputs used class outcomes;
multi-label inputs used label-set outcomes.

| Task | Equal selection | Compare exit | Requested scope | Emitted scope | Compared count | Baseline/candidate exclusions | Emitted schema result | Receipt digest |
|---|---|---:|---|---|---:|---|---|---|
| single-label | one ID | 0 | `intersection` | `identical` | 1 | `[]` / `[]`, counts 0 / 0 | valid | matched bytes |
| single-label | empty | 0 | `intersection` | `identical` | 0 | `[]` / `[]`, counts 0 / 0 | valid | matched bytes |
| multi-label | one ID | 0 | `intersection` | `identical` | 1 | `[]` / `[]`, counts 0 / 0 | valid | matched bytes |
| multi-label | empty | 0 | `intersection` | `identical` | 0 | `[]` / `[]`, counts 0 / 0 | valid | matched bytes |

Each emitted document validates only because the application changed the scope
to `identical`. Changing only the four emitted documents' scope to the requested
`intersection` made all four fail the current schema at the population
alternative: the two exclusion counts are both zero.

Expected: all four commands succeed with `scope: "intersection"`, the same
compared IDs/counts shown above, zero exclusions on both sides, recomputed typed
results, and a schema-valid document. Actual: all four succeed but silently claim
`scope: "identical"`; the current schema cannot validate the correctly scoped
zero-exclusion form.

Smallest correction:

1. In both concrete branches of `app::compare`, dispatch on
   `options.intersection` alone. Run the intersection compatibility check,
   `intersection_scope`, validated restriction, and concrete recomputation even
   when the two selected vectors are equal.
2. Permit zero exclusions in `intersection_population`. Because that makes a
   zero-exclusion population structurally overlap `identical_population`, also
   replace the top-level population `oneOf` with `anyOf` (or rely solely on the
   existing required `scope` conditionals). Keep the `identical` conditional's
   exact empty-list/zero-count constraints unchanged and retain both task-family
   conditionals.
3. Add real API/CLI regressions for equal nonempty and equal empty selections in
   both task branches, asserting the requested scope, lists/counts, recomputed
   output and schema validity.

Do not create artificial exclusions or convert the requested scope.

## Criterion dispositions

### T021 — pass

- Admission checks the closed task-specific policy before population scoring.
  `reject_below` accepts only finite `[0,1]` minima, answered class outcomes and
  the selected confidence/categorical family; `label_thresholds` requires an
  exact vocabulary map, finite `[0,1]` values, answered sets and complete
  marginals. Wrong-task policy, missing family, incomplete map and submitted
  abstention return typed `E_CONFIG`; the public precondition cases leave no run
  directory.
- Single-label evaluation uses strict `value < minimum` rejection. Confidence is
  the submitted confidence; maximum probability is the maximum of the working
  vector. It does not substitute chosen-class probability or force the recorded
  classifier choice to argmax.
- Independently, for threshold `0.5`, `next_down(0.5)` rejects/removes while
  exact `0.5` and `next_up(0.5)` pass/select. The assertions prove scored-choice
  confidence can reject while max probability is `0.6`, and a general
  classifier's recorded non-argmax `B` remains final when working-vector maximum
  `0.6` passes.
- Raw results, probability results, raw answered count and confidence-bin IDs are
  unchanged by final rejection. Multi-label thresholding preserves original raw
  sets/marginals; an all-below row remains an answered `labels: []` outcome.
- Real report and inspection output carries the effective policy. The report and
  inspection schemas bind `reject_below` to single-label and `label_thresholds`
  to multi-label; the concrete negative assertions reject foreign policy shapes.

### T022 — pass

- Comparison consumes typed `MultiLabelEvaluation` and `MultiLabelResults`; it
  does not discover serialized report metrics or use a dynamic metric registry.
  Raw/final pairs and marginal pairs are concrete typed fields with preserved
  directions and candidate-minus-baseline deltas only when both statuses are
  defined.
- Independent fixture derivation for three episodes:
  `{A}->{B}` against expected `{A,B}` is changed and `neither_correct`; whole
  abstention to `[]` against expected `[]` is recovered; `{A}` to whole
  abstention against expected `{A}` is regressed. Thus the category counts are
  `both_correct=0`, `recovered=1`, `regressed=1`, `neither_correct=1`, and changed
  count 3. Label A transitions are present→absent, abstained→absent,
  present→abstained; label B transitions are absent→present,
  abstained→absent, absent→abstained. Each complete 3x3 table sums to N=3.
- Per-side episode evidence retains source ID, final outcome, correctness and
  matched/missed/extra or null abstention evidence. Answered populations are
  baseline IDs `{1,3}`, candidate IDs `{1,2}`, overlap `{1}` with counts 2/2/1;
  equal selected populations are not misreported as equal answered populations.
- Source definitions are resolved locally per side, including observation and
  preparation content through the compared source JSON; restricted source counts
  and composition are derived from actual compared episodes. Run/report identity
  remains the original verified identity.
- Real public API/CLI comparison publishes through the immutable comparison path,
  returns an exact-byte SHA-256 receipt, validates a real multi-label artifact,
  and preserves accepted single-label comparison. Task-foreign and missing hard
  fields are rejected by the comparison schema's concrete alternatives.

### T023 — revise only for the finding above

- `Population::restrict` sorts and checks the request, rejects duplicates and IDs
  outside the existing selection with `E_ID`, preserves dataset digest, role,
  parent/run policy and the complete selected/unselected partition. Both concrete
  evaluation restrictions retain vocabulary, source definitions, policy and
  artifact-wide availability, filter existing aligned rows, and reconstruct only
  through the checked constructors; no prediction JSON is synthesized or decoded.
- Default comparison still rejects selection mismatch. For differing selections,
  intersection relaxes only selection equality after checking golden digest,
  ordered vocabulary/task branch, role and numerical semantics. It sorts common
  and excluded IDs, recomputes both evaluators, derives restricted source
  attribution, and retains original run/report identities.
- Independent unequal-selection expectations match: common ID `...0002`, baseline
  exclusion `...0001`, candidate exclusion `...0003`; single-label raw accuracy
  recomputes 0.0→1.0 and multi-label final exact-match accuracy recomputes
  0.0→1.0. The targeted population is explicit and no winner/full-population
  claim is emitted.
- For empty intersections, both tasks' hard families are `no_data`; originally
  present probability/marginal families are `no_data`; originally absent families
  are `not_applicable`; mixed sides preserve their statuses independently. All
  such deltas are null. `metric_pair` records the relevant baseline or candidate
  non-defined status reason and preserves metric direction. The six tested cells
  (present/present, absent/absent and present/absent for each task) match those
  independently derived expectations.
- Existing real CLI evidence proves mismatch rejection without the flag, flag
  parsing and schema errors for duplicate/unknown/value-taking forms, exact
  receipt, deterministic exclusions, schema-valid unequal intersection, and
  immutable-output refusal.
- The equal-selection explicit-intersection cases remain the sole required
  revision described above.

## Commands and gate evidence

All commands ran in `/Users/dowwie/MyProjects/validator` against the reconciled
candidate.

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --test conformance decision_policy_boundaries -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test conformance policy_preconditions -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test conformance multi_label_comparison_transitions -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test conformance answered_population_overlap -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test cli cli_multi_label_compare_receipt -- --nocapture` | 0 | 1 passed, 8 filtered |
| `cargo test --locked --test conformance intersection_recomputation -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test conformance empty_intersection_availability -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test cli cli_intersection -- --nocapture` | 0 | 1 passed, 8 filtered |
| `cargo test --all-features --locked` | 0 | 44 library, 9 CLI, 21 conformance, 0 doc tests passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | exactly 17 staged production dead-code diagnostics plus 3 matching lib-test diagnostics; no T021-T023 diagnostic |
| temporary four-case real CLI zero-exclusion probe | 0 for every evaluate/compare | defect reproduced in both tasks and both equal population shapes |

The 17 production Clippy locations match the accepted inventory: `src/model.rs`
at 18, 27, 52, 58, 70 and 76; `src/model/common.rs` at 495, 515, 523, 793, 1058
and 1295; `src/model/single_label.rs` at 115; and `src/validation/wire.rs` at 15,
32, 46 and 228. The three lib-test diagnostics are the established wire DTO
duplicates. I reused the frozen developer's passing format, locked release-build
and diff-check evidence as authorized because the candidate hashes remained exact.

No source, test, schema, governance, plan, index, session note, acceptance record
or Fizzy state was changed by this review. This required response is the only
repository artifact written; prompt009 reserves its indexing and operational
follow-up for the coordinator.
