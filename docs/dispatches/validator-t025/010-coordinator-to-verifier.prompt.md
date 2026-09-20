# Independently verify the frozen T025 numerical candidate

Role/model: fresh independent verifier, `gpt-5.6-sol`, high reasoning,
`fork_turns: none`. Do not delegate. You did not implement this candidate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/010-verifier-to-coordinator.response.md`.

Save the full substantive Ready/Revise/Blocked verdict there before returning. In
chat return only the path, SHA-256 and verdict. Do not edit source, schemas, tests,
fixtures, governance, plans, index, session notes, acceptance records or Fizzy.
Temporary read-only probes may live outside the repository. Do not delegate.

Reconcile frozen manifest009 SHA-256
`8071c565c7c4f544cf3072c993df11ea70d3f80a71cbc85c28fd93a99aea6076`
and every governed/current hash before review. Stop Blocked on any mismatch. Read
global/repository AGENTS, manage-dev-team, complete T025, owner prompt001, owner
repair prompt004, the full response chain 005-008, the ratified numerical and
data-model verification sections and every T025-owned coverage row before treating
the implementation handoffs as evidence. T026 and later remain outside scope.

## Independent mathematical and oracle review

Derive expectations from the specification and raw cases before consulting current
product results. Do not use `src/evaluation*`, production metric/count helpers or
production output to generate an expected value. You may inspect necessary product
implementation after the independent expectation is fixed to determine whether a
demonstrated mismatch is a product defect.

Inspect the complete T025-owned test/fixture candidate and necessary surrounding
public API helpers. Confirm the exhaustive oracles construct direct counts before
calling production and do not reuse production decision/count/scoring logic.
Independently reconstruct at least:

- the bounded enumeration cardinalities and accounting identities;
- F04's matrix, accuracy `5/8`, class F1 wire operands `4/7`, `4/5`, `2/4`, and
  macro-F1 `131/210`;
- S03's D/E/U/G/N and direct ratios; S07/S08 analytical loss/Brier endpoints;
  S09/S10 signal-versus-choice semantics and S12 adjacent-binary64 boundaries;
- E01/E04 probability-bearing abstention populations and E07 strict `.99` rejection;
- M01 aggregate and per-label identities, M05 `.265` and analytical log loss,
  M11 categorical `.08` versus marginal `.04`, and M12's equal per-label counts
  with different exact-set results;
- undefined, no-data, no-answered, not-applicable and positive-infinity precedence,
  plus exact ratio/population count/unit/scope fields.

Inspect every exact S01-S12, E01-E04/E07 and M01-M15/M20 test. Confirm each named
filter covers every subcase in its normative row through the public path; a single
headline value, handwritten result object, broad error acceptance or umbrella-only
execution is insufficient. Confirm `full_numeric_conformance` reuses and executes
all required independent cases while every exact filter remains separately listed.
Check that invalid cases produce the precise public error category, publish no
successful report, and do not admit filtering/scoring fallbacks.

Review the owner-authorized abstention correction in model/report/inspection
schema and focused evidence. Confirm a multi-label whole abstention emits literal
`status: "abstained"` with present null matched/missed/extra fields in raw/final
report and verified inspection output, while a declared ordinary class named
`ABSTAIN` remains legal and distinct from the typed abstention column. Do not
reopen broader schema work already accepted at T024.

## Bounded executed evidence

Use the existing target directory. Run one list confirmation, the independent
exhaustive/umbrella filters and the following representative discriminating cases:

```text
cargo test --locked --test conformance -- --list
cargo test --locked --test conformance exhaustive_single_label -- --nocapture
cargo test --locked --test conformance exhaustive_multi_label -- --nocapture
cargo test --locked --test conformance f04_asymmetric_oracle -- --nocapture
cargo test --locked --test conformance full_numeric_conformance -- --nocapture
cargo test --locked --test conformance case_s03 -- --nocapture
cargo test --locked --test conformance case_s08 -- --nocapture
cargo test --locked --test conformance case_s09 -- --nocapture
cargo test --locked --test conformance case_s11 -- --nocapture
cargo test --locked --test conformance case_s12 -- --nocapture
cargo test --locked --test conformance case_e01 -- --nocapture
cargo test --locked --test conformance case_e03 -- --nocapture
cargo test --locked --test conformance case_e04 -- --nocapture
cargo test --locked --test conformance case_e07 -- --nocapture
cargo test --locked --test conformance case_m01 -- --nocapture
cargo test --locked --test conformance case_m03 -- --nocapture
cargo test --locked --test conformance case_m05 -- --nocapture
cargo test --locked --test conformance case_m06 -- --nocapture
cargo test --locked --test conformance case_m09 -- --nocapture
cargo test --locked --test conformance case_m12 -- --nocapture
cargo test --locked --test conformance case_m13 -- --nocapture
cargo test --locked --test conformance case_m14 -- --nocapture
cargo test --locked --test conformance case_m15 -- --nocapture
cargo test --locked --test conformance case_m20 -- --nocapture
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
```

The list must show 74 conformance tests and every exact filter. The full locked
suite must pass 44 library, 10 CLI, 74 conformance and zero documentation tests.
Clippy may exit 101 only for the exact accepted 17 production plus three duplicate
lib-test staged diagnostics; identify any new warning as a finding. Reuse the
developer's unchanged format/release/diff evidence; do not rerun a release build or
create a fresh target by habit. Stop executing once scoped evidence supports the
verdict.

Return **Ready** only if the exact manifest, independent oracle boundary, repaired
wire evidence, exhaustive accounting, every assigned row, umbrella and bounded
gates satisfy T025. Return **Revise** for each reproduced current-contract defect
with requirement, file/line, exact input/expected/actual evidence, consequence and
smallest correction. Return **Blocked** only when a frozen-candidate/evidence
condition prevents judgment. Do not request optional coverage, alternate
representations, generic audit tooling, production cleanup, T026/T027 checks or
private-data work. Stop when the bounded review supports one verdict.
