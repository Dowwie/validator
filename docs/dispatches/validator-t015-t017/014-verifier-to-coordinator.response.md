# Independent T015-T017 backbone verification

## Verdict

**Ready.** The frozen T015-T017 candidate satisfies the current replay,
inspection, identical-population single-label comparison, immutable publication,
and fixed four-row steel-thread criteria. No current criterion-backed defect was
found. This verdict recommends owner acceptance of the architecture gate only; it
does not claim warning-free status, multi-label correctness, T018+ completion,
release readiness, full acceptance, or project completion.

## Candidate and manifest reconciliation

Manifest013 SHA-256 recomputed as
`46dac241d71c838ce1251ae9ed7914798c84009eeb937a7577f5c16da735c5c3`,
matching dispatch014. I recomputed every hash in manifest013 before relying on
the candidate. All governing records, task contracts, implementation handoffs,
production sources, schemas, tests, fixtures, evidence files, acceptance record,
and the recorded release binary matched exactly. In particular:

- owner prompt001, T015/T016/T017, v1 specification, data model, master plan,
  execution contract, and responses002/005/010/011/012 matched their manifest
  hashes;
- every frozen `src/`, `schemas/v2/`, `tests/`, steel-thread fixture/evidence,
  and `docs/acceptance/validator-v1.md` entry matched;
- the verifier's release rebuild reproduced binary SHA-256
  `6fbc83da5f0ec589105802dfdeaf6a3a2e8504b111409164d0c01eed9536ae09`;
- a final source/schema/test/fixture reconciliation after all commands still
  matched manifest013, so the reviewed candidate did not move.

The repository is an intentionally unborn/untracked workspace. That state did
not prevent exact file-hash reconciliation.

## T015 replay and inspection review

The production path satisfies the T015 boundary:

- `src/artifacts.rs` loads `report.json` and all three exact snapshots through
  contained reads under the canonical run root, verifies exact snapshot digests,
  and rejects absolute, parent, missing, or symlink-resolved escape paths. Evidence
  is rebuilt in UTF-8 source-ID order and source-array order, with exact
  `(source_id, evidence_index, original_path, evidence/n.bin, sha256)` checks and
  manifest cardinality. Replay never resolves or opens `original_path`.
- `src/app.rs` feeds only verified `golden.json`, `predictions.json`, and
  `config.json` bytes through `validate_single_label` and the real evaluator,
  reconstructs the typed report, and checks exact structure, arrays, identifiers,
  statuses, counts, opaque configuration, and recorded values. Its numerical
  tolerance is limited to the explicit finite computed-result paths for report
  metric values, integrity maximum sum error, and signal-bin computed values.
  Metric-shaped source configuration and other opaque/recorded numbers compare by
  exact serialized number spelling.
- The strict admission path rejects duplicate keys and unknown wire fields and
  retains opaque episode input through `RawValue`; the evaluator receives the
  checked typed single-label evaluation rather than replay JSON.
- Inspection first completes the full replay check, requires a selected episode,
  and discloses only the explicit selected input, target, original prediction,
  final outcome, observations/probability/confidence, and recorded configuration.
  Routine reports and fixed diagnostics omit payloads.
- The CLI inspection case succeeds after the original snapshots disappear and
  preserves `9007199254740993`, out-of-range `1e400`, JSON null, and the literal
  `$serde_json::private::Number` key spelling. The inspection schema has a closed
  top-level property set for that explicit result.

The conformance mutation table independently rejects snapshot/evidence damage,
escaping evidence symlinks, missing/extra/swapped or rewritten bindings, changed
counts/status/configuration, and a metric-shaped opaque configuration change;
only a whitelisted computed metric perturbation within tolerance succeeds.

## T016 comparison and publication review

The comparison path satisfies the current single-label identical-population
contract:

- `compare` calls the same verified replay for both sides before constructing a
  comparison. The pure `src/comparison.rs` boundary receives concrete
  `SingleLabelEvaluation` and `SingleLabelResults` references plus narrow typed
  metadata. There is no recursive metric-name discovery or dynamic required
  metric map.
- Compatibility checks the exact golden digest, concrete single-label type,
  ordered vocabulary, evaluation role, ordered selected IDs, categorical
  tolerance, and bin count. Model, source, observation-definition, preparation,
  and decision-policy differences are allowed and recursively reported as typed
  configuration differences. Each side retains its own complete source
  definitions, so equal local source IDs do not imply common origin.
- Raw, final, and probability results remain distinct typed fields. Every metric
  pair preserves both checked metric objects and population metadata, direction,
  a finite candidate-minus-baseline delta only when both statuses are `defined`,
  and a typed null reason otherwise. Selective pairs expose baseline/candidate
  answered IDs and counts plus their overlap; probability fields retain selected
  metric populations and explicit raw-answered/disagreement counts.
- Every episode is placed in exactly one of `both_correct`, `recovered`,
  `regressed`, or `neither_correct`; their checked total equals N. Changed final
  outcomes retain IDs. The complete `(K+1) x (K+1)` class-plus-abstention table is
  emitted, checked to sum to N, and every episode row retains both outcomes,
  correctness values, and source IDs.
- No winner or significance field is produced. `comparison.json` uses the same
  private temporary-sibling, mode-restricted, atomic no-replace publisher as
  evaluation runs; the five-entry evaluation layout remains unchanged. The
  comparison and receipt schemas validate the real CLI documents, and the receipt
  digest equals the SHA-256 of the exact published `comparison.json` bytes.

The valid held-out comparison case demonstrates replay success followed by an
`E_COMPARISON` compatibility rejection, rather than a replay short circuit. The
owning-module test independently rejects every runtime compatibility axis.

## Independent T017 oracle and process evidence

I recalculated the oracle directly from `golden.json`, `baseline.json`, and
`candidate.json` using only the fixed rows and elementary arithmetic, without
calling Validator scoring or copying `expected.json`:

- baseline outcomes give `D=2`, `E=1`, `U=1`; accuracy `2/4=1/2`, wrong-class
  and abstention rates `1/4`, coverage `3/4`, selective accuracy `2/3`, and
  selective risk `1/3`;
- class coverages are `[(1/2),(1/1),(1/1)]`; per-class F1 values are
  `[1/2,0,1]`, so macro-F1 is `1/2`;
- the four categorical rows give Brier `1.2/4=0.30`, log loss
  `(-2*ln(0.7)-ln(0.3)-ln(0.8))/4 = 0.5351165608794026`, and argmax accuracy
  `3/4`;
- all four IDs enter maximum-probability bins. Raw confidence bins contain IDs
  1, 3, and 4 and exclude the abstained ID 2;
- candidate comparison recovers IDs 2 and 3, regresses ID 1, keeps ID 4 in
  `both_correct`, and has no `neither_correct` episode.

The exact golden bytes hash to
`eacd3362fc6bf571ed9c3547cefbf9190ad79adcc198b047e6c03fa6387387ce`,
which is the digest bound by both prediction fixtures. The two same-basename
evidence files have different bytes and hashes; the baseline retains both the
ordinary relative and genuine parent-relative evidence paths. The real-binary
steel-thread test deletes the complete original input tree and parent-relative
source before relocated inspection and comparison.

The process case validates all success documents against the published schemas,
recomputes both evaluation receipt hashes and the comparison receipt hash from
exact result bytes, verifies the stored snapshots and two distinct evidence
copies, and observes the hand oracle above. The opaque integer appears only in
explicit inspection. Duplicate-key input returns exit 2 / `E_SCHEMA` / `schema`;
the submitted 0.99 categorical scoring vector returns exit 2 /
`E_PROBABILITY` / `validation`; changed stored evidence returns exit 2 /
`E_PROVENANCE` / `replay`; existing evaluation and comparison destinations return
exit 3 / `E_OUTPUT_EXISTS` / `filesystem`. Each failure leaves no successful
partial output, and both existing destination trees remain byte-for-byte
unchanged with no temporary sibling.

`docs/acceptance/validator-v1.md` accurately records candidate architecture-gate
evidence and explicitly disclaims release, multi-label acceptance, broad
acceptance, and project completion.

## Commands and observed results

| Command | Exit | Independent result |
|---|---:|---|
| `cargo test --locked --test conformance relocated_run_inspection -- --nocapture` | 0 | Zero selected tests: dispatch014 names the wrong test target. |
| `cargo test --locked --test cli relocated_run_inspection -- --nocapture` | 0 | Supplemental correct target, 1 passed. |
| `cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture` | 0 | 1 passed. |
| `cargo test --locked --test conformance single_comparison_transitions -- --nocapture` | 0 | 1 passed. |
| `cargo test --locked --test conformance comparison_compatibility_and_deltas -- --nocapture` | 0 | 1 passed. |
| `cargo test --locked --test cli cli_compare_receipt -- --nocapture` | 0 | 1 passed. |
| `cargo test --locked comparison::tests::typed_compatibility_facts_reject_each_runtime_axis -- --nocapture` | 0 | 1 unit test passed; other targets selected zero as expected for the filter. |
| `cargo test --locked --test cli steel_thread_end_to_end -- --nocapture` | 0 | 1 real-binary process test passed. |
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Exactly the 24 authorized unique production `dead_code` diagnostics; no other lint class. |
| `cargo test --all-features --locked` | 0 | 40 library + 6 CLI + 11 conformance tests passed; 0 failures; 0 doc tests. |
| `cargo build --release --locked` | 0 | Passed; rebuilt binary hash matched manifest013. |
| `ruby docs/plans/validator/verify-plan.rb` | 0 | Passed 35 task contracts, 433 source blocks, 63 cases, 12 structure checks, 8 ACs, 5 DoD clauses, owners, links, and index. |
| `git diff --check` | 0 | Passed before the verifier response write. |

The 24 unique Clippy diagnostics reconcile exactly to the staged inventory:
`checked_mul`; `TaskDefinition`; its four methods; `SingleLabelTask`; its two
methods; `MultiLabelTask`; its two methods; `LabelVocabulary::for_multi_label`
and `label_set`; `LabelSet`; its two methods; `Episode`; its constructor and
three accessors; `ObservationSet::values`; `EvaluationConfig::policy` field and
accessor; `MetricUnit::LabelDecision`; the remaining `MetricResult` constructor
and accessors; `signal_availability`; wire golden `schema_version`; wire
single-label task `kind`; wire golden episode `input`; wire prediction
`schema_version`; wire config `schema_version`; and the two staged
`RejectBelow` fields. Clippy also prints six test-target duplicates of that same
inventory. There is no suppression, fake consumer, widened public API, changed
tolerance, or additional warning category. The warning-free T027/T035 gates
remain open and are outside this verdict.

## Material limits

- The first exact command in dispatch014 is a target typo and exercises zero
  tests. The correctly targeted CLI filter and the complete CLI/full suites pass,
  so this is not a candidate behavior defect.
- This review covers only the frozen T015-T017 single-label backbone candidate.
  It does not assess intersection comparison, multi-label behavior, T018+,
  installation, private practical acceptance, future warning cleanup, or general
  release readiness.
- The response file is the verifier's only repository write. Artifact-index,
  session-note, acceptance-state, and Fizzy updates remain with the coordinator
  and owner under dispatch014's read-only boundary.

## Final verdict

**Ready** — every current T015-T017 criterion has supporting code, schema,
independent arithmetic, process, and command evidence, and the only warning-denied
Clippy residual is the explicitly authorized 24-diagnostic production inventory.
