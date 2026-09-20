# Complete the existing T020 candidate

Role/model: same sole T020 implementation developer, `gpt-5.6-terra`, high
reasoning, retained task context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Governing assignment: prompt005 plus lint correction006.

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/007-t020-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not overwrite response005 or
response006. Do not edit governance, plans, artifact-index, session notes,
acceptance records or Fizzy.

Response005 SHA-256
`c4ffdafa34a4a6bf65b8ea8d8f635b4f522bfc08841cc2926440d1a63fa58ccf`
is an honest partial handoff. Preserve its valid implementation and evidence.
Do not restart orientation, rewrite working marginal code, add optional cleanup,
or rerun settled filters except where the final combined gate justifies it.

Current changed-source hashes at this completion boundary:

| Artifact | SHA-256 |
|---|---|
| `src/model/multi_label.rs` | `a4f2f7fa678c525f1c3aed34fb1c3931c887fe3359aa13cd7396cbeee228be46` |
| `src/evaluation/multi_label.rs` | `013f014ed1300a4e2173bc0a4a187af53717079c7d80a37575932e242287257b` |
| `src/app.rs` | `d08e50c7ab32bffa0e9791bdcb32ffa27f8c60e0ed8fc586b23d643e18588ca0` |
| `schemas/v2/check.schema.json` | `7d76d4c9a24322db67e44ab4ce9e03bb13cfc2a60aeb82520a2289b4d7efdb57` |
| `schemas/v2/report.schema.json` | `750802861c4c1187a93a8095230b0b85608db06673ec29a6ee185ecc6555b4c9` |
| `tests/conformance.rs` | `78e737361a0ccddce9009cb2073ce2da71dbbea35620f497e864053cd2bedc68` |

The other T020-owned files still match the starting hashes in prompt005. Stop
and report an exact mismatch before editing.

## Complete only the missing original contract

1. Finish stored multi-label replay and `inspect` through the same verified
   artifact/evidence/report-rebuild path. Update the strict inspection schema with
   concrete task alternatives. Preserve opaque input exactly and return the real
   expected target, prediction, final outcome, observations, marginals and config.
2. Add the real public conformance filters
   `multi_label_hard_oracles` and `equal_counts_distinct_exact_sets`. They must call
   `evaluate`, read the published report, validate it against the strict report
   schema and assert the T019 independent rational/status/evidence oracle. The
   equal-count case must prove equal per-label aggregate counts and different
   episode-level exact-set accuracy. Do not expose or call private evaluators.
3. Complete `marginal_loss_and_bins` against the real published report. Retain the
   passing finite `[.8,.7]/{A}` case and add the missing original discriminators:
   marginal scoring on abstention, absent-at-zero zero loss without `0*ln(0)`,
   present-at-zero and absent-at-one positive infinity/status, one-label binary
   Brier `.04` distinct from categorical `.08`, and bins at `0/.1/1` paired to
   reference presence. Use existing tolerances only for computed transcendent
   values.
4. Add exact process filter `shared_commands_multi_label`. Run the real binary
   through check, evaluate and inspect, validate stdout against check/receipt/
   inspection schemas and validate stored report against report schema. Exercise
   relocated stored-run replay, exact opaque payload preservation, atomic output
   refusal/failure behavior relevant to the new task and one-document stdout.
   Multi-label compare remains prohibited and owned by T022.
5. Finish strict check/report/inspection schema discrimination: valid real
   multi-label documents pass; missing required fields, foreign task keys and
   near-miss mixed alternatives fail. Do not alter task-neutral receipt/error
   schemas without a demonstrated failure; report and stop if one appears.
6. Preserve all accepted single-label behavior and the shared I/O/publication
   implementation. Keep current policy `as_recorded`; no T021 thresholds, T022
   comparison, facade, source inclusion, public export or dependency.

Run and record all six exact nonzero filters from prompt005, correct Cargo list
targets, then the full boundary gates:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Apply correction006 exactly when classifying lint. Existing authorized staged
symbols remain allowed; only new unavoidable T020 policy/DTO members require an
exact real T021/T022 consumer. No ordinary warning, suppression, fake use,
visibility expansion or cleanup-only work.

Response007 must provide the complete T020 criterion/schema/test map, exact
commands/exits/listed and selected counts, final hashes, exact residual lint
inventory with existing/new distinction, and confirmation that every item listed
as incomplete in response005 is now complete. If this same assignment cannot be
completed, save the full handoff with the exact remaining blocker and stop; do not
return another progress-only sketch.
