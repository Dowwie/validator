# Independently review the frozen T018-T020 multi-label candidate

Role/model: fresh independent verifier, `gpt-5.6-sol`, high reasoning,
`fork_turns: none`. Read-only verifier; do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.

Required complete verdict:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/023-verifier-to-coordinator.response.md`.

Save the full substantive verdict there before returning. In chat, return only
the response path, SHA-256 and verdict. Your response file is your only permitted
repository write. Do not modify production, tests, fixtures, schemas, governance,
plans, artifact index, session notes, acceptance records or Fizzy. Do not create
another agent.

## Read and reconcile the frozen candidate

Read completely:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`,
  `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- owner prompt001 and corrections006/009/012/015/020;
- `docs/dispatches/validator-t018-t020/022-coordinator-candidate-manifest.md`;
- T018, T019 and T020 task JSON plus every linked normative specification section;
- complete responses003,004,013,014,016,018 and021;
- every frozen source/schema/test/oracle file listed in manifest022.

Manifest022 SHA-256 is
`918fd6e13608940232de86f3bfb4b4dfba81c7d5cf618657225cef260e05c177`.
Recompute every listed hash before relying on the candidate. If any differs,
return `Blocked` with the exact mismatch; do not review a moving candidate.

This is the single combined review authorized by owner001. The candidate is not
warning-free: only the exact 20 owner-authorized production `dead_code`
diagnostics recorded in response021 may remain. T027/T035 retain the future clean
gate. Any ordinary warning, suppression, fake consumer, widened export, changed
tolerance or failed behavior gate is a finding.

## Review T018 checked admission

Independently verify from code and focused evidence that:

- explicit tagged task/target/outcome/probability dispatch prevents task inference
  and cross-kind coercion; only classifier sources are admitted for multi-label;
- label sets use the checked vocabulary in canonical order, reject duplicate/
  unknown labels before set conversion, and keep answered empty sets distinct from
  whole-episode abstention and missing rows;
- marginals require exactly every vocabulary key, preserve finite `[0,1]` values
  unchanged, permit sums such as `[0.9,0.8]`, never normalize, and remain distinct
  from categorical probabilities and retained reported-confidence observations;
- wrong task/source/policy/outcome/probability shapes, top-level confidence,
  scored-choice/count/partial/per-label-abstention inputs and missing/extra rows or
  marginal keys reject through stable diagnostics;
- shared exact digest, UUID, source/evidence/preparation, observation, selection,
  sorted alignment and opaque RawValue behavior remains one concrete backbone;
  existing large-integer, `1e400`, null and literal-number-key regressions remain.

## Review T019 hard accounting

Independently verify:

- the evaluator borrows only `MultiLabelEvaluation`; raw/final remain distinct and
  equal under current `as_recorded` without placeholder threshold behavior;
- `N/U/G/D/E`, exact/wrong/coverage/abstention/selective metrics carry correct
  episode populations, ratios and statuses;
- per-label TP/FP/FN/TN use only answered `G`, each sums to `G`, all counts sum to
  `G*K`, support uses all `N`, and answered/predicted support are correct;
- per-label, micro/macro and Hamming metrics use the exact named denominators,
  answered label-decision population, undefined zero-fill/status rules,
  `no_answered_predictions` for `N>0,G=0` and `no_data` for `N=0`;
- episode evidence retains vocabulary-ordered expected/predicted/matched/missed/
  extra sets, correctness and observations; abstentions use null differences and
  never fabricate empty/negative decisions;
- the hand-authored oracle is independent. Recalculate the two-row
  `[A,B,C]` case (`TP/FP/FN/TN=1/1/1/3`, exact `1/2`, micro F1 `1/2`, macro F1
  `1/3`, Hamming `1/3`), empty answer, one/all abstention and empty population;
  prove the equal binary-count pair genuinely has exact accuracy `1/2` versus `0`.

## Review T020 marginal and application boundary

Independently verify:

- complete marginals score all selected `N*K` label decisions including
  abstentions; no silent row selection, clipping, normalization, argmax, joint-set
  probability, confidence or ECE is synthesized;
- positive log loss uses `-ln(p)`, negative uses stable `-ln_1p(-p)`, the observed
  branch avoids `0*ln(0)`, zero assigned to the observed outcome yields explicit
  positive infinity, and mean infinity/status/populations propagate correctly;
- binary Brier and per-label/mean populations are correct. Independently calculate
  `[.8,.7]/{A}` mean Brier `.265` and mean log loss
  `(-ln(.8)-ln(.3))/2`; confirm one-label binary `.04` remains distinct from
  categorical `.08`; inspect `0/.1/1` bins for boundary and reference positivity;
- closed application dispatch uses one shared load/evidence/publication path for
  check/evaluate and stored-run rebuild/inspect across both tasks. Multi-label
  comparison and threshold policy remain absent and assigned to T022/T021;
- relocated inspection verifies contained snapshot/evidence bindings through the
  existing stored-run interface, succeeds after original inputs/evidence vanish,
  preserves exact opaque payload for explicit inspect, and rejects contained
  evidence tampering as `E_PROVENANCE` without fallback to original paths;
- check/report/inspection schemas define closed coherent single/multi alternatives,
  required nested fields, legal finite/null/infinite/status forms and reject
  foreign task keys/near misses while preserving opaque values only where allowed;
- the real CLI produces one schema-valid JSON document, preserves private normal
  output/error behavior, refuses existing destinations without changing the tree,
  and retains accepted single-label/replay regressions.

## Checks and verdict

Rerun at least these exact checks on the frozen files, using serialized Cargo
commands where they share build resources:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance equal_counts_distinct_exact_sets -- --nocapture
cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture
cargo test --locked --test conformance single_report_schema -- --nocapture
cargo test --locked --test conformance check_inspection_schema_contract -- --nocapture
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
cargo test --locked --test cli relocated_run_inspection -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
ruby docs/plans/validator/verify-plan.rb
git diff --check
```

Use unchanged valid evidence where it settles a criterion; stop additional
exploration once the scoped code/schema/oracle review and commands support a
verdict. Do not audit T021 policy behavior, T022 multi-label comparison,
intersection, installation, unrelated style or future staged-warning cleanup.

Return exactly one verdict:

- `Ready`: every current T018-T020 criterion is satisfied and only the authorized
  20 staged diagnostics remain;
- `Revise`: a reproducible current criterion is violated and a bounded correction
  exists;
- `Blocked`: the candidate is unstable or an external condition prevents judgment.

For each finding, name the current criterion, exact location, reproduction,
consequence and smallest correction. Record manifest reconciliation, independent
oracle math, commands/exits/counts, observed release-binary hash, exact Clippy
residual status, material limits and final verdict. Passing tests do not override
a code/schema/oracle violation; optional improvements are not findings.
