# Final focused recheck of T025 per-label metric metadata

Resume as the same independent Sol-high verifier. Do not delegate and do not edit
the repository. Save the full Ready/Revise/Blocked verdict to:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/021-verifier-to-coordinator.response.md`.

Reconcile final repaired manifest020 SHA-256
`deeed4c908fa385de1cd8494a31fdcc7fb8d46646e03b32766691fe007f24fc9`
and every listed hash. Read owner prompt016, owner clarification018 and full
developer response017. Reuse all settled evidence from verdict010 and the accepted
S03/S12/M14 portions of verdict015. This is a final focused unit/operand recheck;
do not restart broad numerical review or inspect T026+.

Independently fix the expectation before public output: every multi-label
per-label precision, recall and F1 uses population count `G`, unit
`label_decision`, scope `answered`; aggregate binary metrics use `G*K` label
decisions; set metrics use episodes.

Confirm:

1. The only production delta is the three `label_metrics` calls from
   `answered_episode_metric` to existing `answered_label_metric`, passing `G` as
   the per-label population. No new helper/formula/status change exists.
2. Raw and final normal answered M01, undefined empty-set M02, all-abstained M04
   and empty-selection results check all three per-label metrics against the fixed
   metadata and unchanged values/statuses/operands.
3. Each M02 per-label F1 explicitly proves that serialized `numerator` and
   `denominator` fields are present and equal integer zero, rather than relying on
   the shared helper's early zero-denominator branch.
4. Aggregate G*K and set-level episode assertions remain, preventing blanket unit
   replacement.
5. The existing persisted report plus real verified inspection/recomputation path
   exposes the same corrected per-label metadata.

Run only justified affected checks using the existing target:

```text
cargo test --locked --test conformance case_m01 -- --nocapture
cargo test --locked --test conformance case_m02 -- --nocapture
cargo test --locked --test conformance case_m04 -- --nocapture
cargo test --locked --test conformance exhaustive_multi_label -- --nocapture
cargo test --locked --test conformance full_numeric_conformance -- --nocapture
cargo test --locked --test conformance multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection -- --nocapture
cargo test --locked --test conformance -- --list
```

Each filter must pass one with 73 filtered and the list must remain 74. Reuse
response017's full test/Clippy/format/release/diff evidence; do not repeat those
gates absent a concrete new failure.

Return **Ready** only if the final manifest, corrected public metadata, literal
zero operands and all affected regressions satisfy the ratified contract with no
remaining blocker. Return **Revise** only for a concrete remaining requirement
defect with reproduction and smallest correction. One further discrepancy returns
to owner; do not generalize scope. Return **Blocked** only for a manifest/evidence
condition preventing judgment. Stop when this evidence supports the verdict.
