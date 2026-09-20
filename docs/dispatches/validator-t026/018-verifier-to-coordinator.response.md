# T026 resolved inspection source focused verdict

Verdict: **Ready**.

The frozen source-field correction closes addendum015. Public inspection now
returns exactly the selected episode's resolved, replay-verified source definition
while preserving the raw prediction, opaque input and separate evaluation
configuration. The strict schema and focused single-label, multi-label,
preparation, relocation and privacy evidence support the correction with no known
blocking defect.

## Frozen identity and scope

`017-coordinator-source-field-manifest.md` has the required SHA-256
`bb0ea141de203596fcd9ee3c7db8b7bf37f8a2033e7858d05074b9bf03e3cf44`.
Every hash listed in manifest017 reconciled before review. After focused
execution, manifest017 and the changed `src/app.rs`, inspection schema,
conformance test and CLI test retain their listed identities.

I read owner prompt014, addendum015 and complete developer response016. I
inspected only R1/source resolution and the affected schema, both-task,
preparation, relocation and privacy proofs. The other five repaired findings and
the broad T026 review remain settled. I did not inspect T027/T028.

## Source resolution review

- `InspectionResult` now contains one required `source: Value` while retaining
  the existing `prediction`, `input` and evaluation `configuration` fields.
- `inspect` validates and replay-verifies the stored run, including verified
  evidence bindings, and confirms the stored report equals the rebuilt report
  before constructing disclosure output.
- It then reads the selected raw prediction's `source_id` and clones exactly that
  key from the verified stored report `sources` map. A missing key is an
  `E_INVARIANT` consistency failure. No source registry, fallback or unrelated
  source map is returned.
- The returned source is the report serialization, so optional observation
  definitions and preparation survive and evidence paths are verified stored
  `evidence/N.bin` paths. Original source paths and evidence-file contents are not
  returned or dereferenced.
- The original raw prediction, opaque selected input, final evidence and decoded
  evaluation configuration remain separate and unchanged.

## Schema and discriminating proof

The strict inspection schema requires `source` under top-level
`additionalProperties: false`. Its source definition requires kind, model,
configuration, evidence and observations; optional question, preparation and
observation-definition fields use the existing report-source layout. The
multi-label branch requires `classifier`, while single-label inspection accepts
its legal classifier and scored-choice kinds.

The repository schema filter accepts the fixed single-label and multi-label
documents and rejects a missing source. I also ran a read-only in-memory schema
probe against the frozen schema. It confirmed:

- valid: single-label classifier, single-label scored-choice, multi-label
  classifier;
- invalid: missing source, missing source model, unrelated source property, and
  multi-label scored-choice.

The focused cases derive expected source data from fixed inputs:

- S15 inspects both Q1/Q2 episodes and asserts the selected kind, model,
  configuration, empty evidence and observation maps, while retaining complete
  comparison-side definitions.
- E08 asserts the selected source's observation definition, preparation
  descriptor and three verified stored evidence paths.
- The multi-label abstention case validates real classifier source evidence with
  the strict schema.
- The CLI steel-thread removes original inputs before inspection, then proves the
  selected source and stored evidence path survive relocation. Existing routine
  output and tamper checks preserve privacy and replay rejection.

## Executed evidence

Each command executed one nonzero test and exited 0:

- `cargo test --locked --test conformance case_s15 -- --nocapture`
- `cargo test --locked --test conformance case_e08 -- --nocapture`
- `cargo test --locked --test conformance check_inspection_schema_contract -- --nocapture`
- `cargo test --locked --test conformance multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection -- --nocapture`
- `cargo test --locked --test conformance case_s24 -- --nocapture`
- `cargo test --locked --test conformance case_s27 -- --nocapture`
- `cargo test --locked --test cli steel_thread_end_to_end -- --nocapture`
- `cargo test --locked --test cli publication_and_privacy_matrix -- --nocapture`

I reused response016's frozen broad evidence as directed: the full locked suite
passes 44 library, 11 CLI, 91 conformance and zero doc tests; format, locked
release, diff and plan checks pass; warning-denied Clippy contains only the
accepted 17 production plus three duplicate lib-test staged diagnostics.

No source, schema, test, fixture, governance, plan, index, session note,
acceptance record or Fizzy state was edited during this recheck. This Ready verdict
recommends owner acceptance of manifest017 for the completed T026 correction; it
does not authorize or assess T027/T028.
