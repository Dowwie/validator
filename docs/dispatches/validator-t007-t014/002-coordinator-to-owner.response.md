# T007-T014 boundary-clarification acknowledgment

I applied the owner's three clarifications without restarting or discarding the
current T007 slice.

1. T010 `input_schema_contract` may call the published schema validator directly,
   including legal multi-label schema alternatives before T018. Scoring/report
   conformance and runtime parity still use the real T014 application API.
2. T009 `signal_population_bins` now owns as-recorded probability/confidence
   populations, abstentions, IDs, bins, and counts. Threshold-rejected answered-row
   preservation is explicitly staged in T021 `decision_policy_boundaries`; no T021
   behavior is implemented early.
3. Each published run contains exactly `golden.json`, `predictions.json`,
   `config.json`, optional `evidence/`, and `report.json`. Manifest metadata lives
   inside `report.json`; no per-run schema or separate manifest file is authorized.

The two exact task-expectation metadata fields and `session-notes.md` now record
that sequencing. Developer follow-up 003 preserves the passing T007 work and
requires completion of T008-T014 under these boundaries. The first developer
handoff is indexed as incomplete; a second incomplete return goes directly to
owner reassessment.

