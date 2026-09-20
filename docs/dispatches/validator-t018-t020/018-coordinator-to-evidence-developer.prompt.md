# Complete T020 numerical and CLI evidence, then run the full boundary gates

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/018-evidence-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy. Do not
delegate.

## Read first and fixed prerequisites

Read `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, Rust best-practices,
owner prompts001/006/009/012/015, task T020, the normative multi-label hard/
probability/CLI sections, T019 independent oracle, and completion responses013,
014 and016. Then read the four exact current test bodies and only the production
types they serialize/assert before editing.

The report, check and inspection schemas and relocated stored-evidence repair are
complete local prerequisites. Do not reopen or redesign them. This stage completes
only the original numerical/CLI evidence and runs the full T018-T020 gates.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| report handoff013 | `4cf336a5dafe12ddb59a42f51f51f8c2211ff0a209e3589c0513a6e6547184db` |
| schema handoff014 | `c13d013e38755b8a49510d120a308e6b39a784abdcae6fc99b71749e1ca0e83b` |
| repair handoff016 | `9d53993473218e9b77f4a8043f2dda4e9afc1739f444a6012ad585ebb062993c` |
| `src/model/multi_label.rs` | `a4f2f7fa678c525f1c3aed34fb1c3931c887fe3359aa13cd7396cbeee228be46` |
| `src/evaluation/multi_label.rs` | `013f014ed1300a4e2173bc0a4a187af53717079c7d80a37575932e242287257b` |
| `src/evaluation.rs` | `172ab7db055ca9483e3b64593411bb9e64c1983eecdbd06013fe0f6ce1a483ff` |
| `src/app.rs` | `b8f1b5b32bdff042eb5bca3b667da6fccdeb793bec04bb3b35c9bd7d4ce47b1b` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| check schema | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| report schema | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| inspection schema | `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176` |
| `tests/conformance.rs` | `15507426e673eea44aa04616b3ce8601785f270308cba113ec72d7ad9ba20ff6` |
| `tests/cli.rs` | `c23e38a343fb8eaccb1b784bbdf24cd15c0c4f50f081e461cf70af7add399999` |
| T019 independent oracle | `a65de157415f75567cb907f2dd45ae3d524dcb552dce7c10ef9ca87efbfc63ec` |

Stop and save the exact mismatch before editing if any fixed hash differs.

## Write scope

Primary writes are limited to:

- `tests/conformance.rs`;
- `tests/cli.rs`.

Do not alter the three completed schemas. A minimal T020 production correction is
allowed only after a focused existing-contract test reproduces the defect; record
the requirement, failing command and exact changed paths before the correction.
Do not refactor, alter tolerances, widen exports, add dependencies, suppress
warnings, implement thresholds or add multi-label comparison.

## Complete the exact evidence

1. `multi_label_hard_oracles`: use real public `evaluate` and published reports.
   Cover the T019 independent cases, including:
   - `[A,B,C]`, expected `[{A,B},{}]`, predicted `[{A,C},{}]`: `N=G=2`, exact
     match `1/2`, aggregate TP/FP/FN/TN `1/1/1/3`, micro F1 `1/2`, macro F1
     `1/3`, Hamming `1/3`;
   - exact population counts/episode versus label-decision units/scopes and
     numerator/denominator/status fields;
   - answered empty set, one abstention, all-abstained
     `no_answered_predictions`, empty selection `no_data`;
   - null abstention evidence and genuine empty answered sets;
   - separate raw/final families and episode matched/missed/extra ordering.
   Read expected values from the hand-authored fixture where practical; never
   generate the oracle from production scoring.
2. `equal_counts_distinct_exact_sets`: publish two real two-row runs with expected
   `[{A,B},{A,B}]`, predictions `[{}, {A,B}]` versus `[{A},{B}]` or the exact
   equivalent frozen pair. Assert every label TP/FP/FN/TN aggregate is equal and
   exact-set accuracy is `1/2` versus `0`, with retained episode evidence. Array
   lengths alone are insufficient.
3. `marginal_loss_and_bins`: preserve the finite abstention `[.8,.7]/{A}` result
   and add absent-at-zero zero loss, present-at-zero and absent-at-one positive
   infinity/status, one-label marginal `.8`/present binary Brier `.04` distinct
   from the accepted categorical `[.8,.2]` Brier `.08`, and per-label bin
   boundary/positive-count evidence at `0`, `.1`, `1`. Assert unchanged marginals,
   selected `N` and `N*K` label-decision populations, scopes/units/statuses and
   legal null/special-value representation.
4. `shared_commands_multi_label`: preserve real check/evaluate/relocated inspect,
   contained bound evidence and tamper rejection. Complete the remaining original
   assertions: exact opaque payload preservation, no payload/secret disclosure in
   normal stdout/error, exactly one JSON document on stdout, existing-output
   refusal with the output tree unchanged, and strict check/receipt/report/
   inspection schema validation. Reuse accepted shared artifact helpers/evidence;
   do not duplicate the full single-label failure matrix or test compare.

## Required commands

First list the correct targets and confirm the six exact names exist. Then run
each command separately with nonzero selection:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance equal_counts_distinct_exact_sets -- --nocapture
cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
```

Then run once on the complete candidate:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Apply owner001/correction006 exactly. Previously authorized staged private symbols
remain allowed; only a new unavoidable T020 policy/DTO member requires an exact
real T021/T022 consumer. No ordinary warning, suppression, fake use or visibility
expansion is allowed. Report the exact residual inventory with existing/new status.

Response018 must be complete: independent derivations, criterion-to-real-path
assertion map, exact list/command/exit/count/actual results, final hashes,
production reproduction/correction if any, exact residual lint inventory, and
confirmation that schemas stayed fixed and no T021/T022 work occurred. This handoff
is the local candidate for the single combined independent review; it is not owner
acceptance. If incomplete, record the actual stop condition and smallest unfinished
item for owner reassessment rather than returning a progress sketch.
