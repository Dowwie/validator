# Independently review the frozen T015-T017 backbone candidate

Role/model: independent verifier, `gpt-5.6-sol`, high reasoning,
`fork_turns: none`. Read-only verifier; do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.

Required complete verdict:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/014-verifier-to-coordinator.response.md`.

Save the full substantive verdict there before returning. In chat, return only
the response path, SHA-256 and verdict. Your response file is your only permitted
repository write. Do not modify production, tests, fixtures, schemas, acceptance
records, governance, plan, artifact index, session notes or Fizzy. Do not create
another agent.

## Read and candidate identity

Read completely:

- `/Users/dowwie/.codex/AGENTS.md` and repository `AGENTS.md`;
- `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md` for the verifier role;
- `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/dispatches/validator-t015-t017/001-owner-to-coordinator.prompt.md`;
- `docs/dispatches/validator-t015-t017/013-coordinator-candidate-manifest.md`;
- T015/T016/T017 task JSON files and their complete linked specification sections;
- T015 responses002/005, T016 responses010/011 and T017 response012;
- `docs/acceptance/validator-v1.md` and every frozen source/schema/test/fixture
  listed by manifest013.

Manifest013 SHA-256 is
`46dac241d71c838ce1251ae9ed7914798c84009eeb937a7577f5c16da735c5c3`.
Recompute every manifest hash before relying on the candidate. If any hash differs,
return `Blocked` with the exact mismatch; do not review a moving candidate.

This is the one combined independent review authorized after T017. T018 and later
work is outside scope. The candidate is intentionally not warning-free: owner001
permits only the exact 24 currently staged production `dead_code` diagnostics;
T027/T035 remain warning-free gates. No other warning, suppression, fake use,
public expansion, changed tolerance or failed behavior gate is accepted.

## Review T015 replay and inspection

Independently verify from code and evidence that:

- stored runs check exact snapshot/report/evidence digests, complete deterministic
  one-to-one bindings and contained regular stored paths, including symlink
  escape rejection, without dereferencing original paths;
- replay re-enters real strict admission/evaluation and compares exact structure,
  counts and recorded data; numerical tolerance applies only to full whitelisted
  finite computed-result paths, never metric-shaped source configuration or
  opaque data;
- relocated inspection works after original sources disappear and preserves the
  selected input's large integer, out-of-range opaque number, null and literal
  serde-number-key spelling while routine reports/errors omit payloads;
- the inspect API/CLI and strict schema expose only the explicit selected payload.

## Review T016 comparison and publication

Independently verify that:

- both runs pass T015 replay before comparison; compatibility enforces golden
  digest, concrete single-label kind, ordered labels, role, selected IDs and
  numerical semantics while allowing/reporting policy/model/source differences;
- the pure boundary consumes recomputed typed single-label evaluations/results
  plus narrow typed metadata, with explicit hard/probability pair fields. Reject
  any recursive string metric discovery or dynamic required metric map;
- raw/final/probability families remain separate; each concrete pair preserves
  checked statuses, population metadata, direction, defined-only candidate-minus-
  baseline delta, typed null reason otherwise, and answered populations/overlap;
- same local source IDs do not imply common origin and definition/configuration/
  observation/preparation differences are explicit;
- every episode belongs to exactly one correctness category, exact IDs/counts and
  changed outcomes are retained, the complete class-plus-abstention table sums to
  N, and episode rows preserve both outcomes/correctness/source IDs;
- comparison emits no winner/significance claim and `comparison.json` uses the
  same private atomic no-replace/cleanup publisher as evaluations without changing
  the five-entry evaluation layout;
- the strict schema and real CLI receipt bind exact published bytes.

## Independently check T017

Recalculate the four-row oracle from the fixture itself, without using production
scoring or merely copying `expected.json`. Confirm:

- `D=2,E=1,U=1`, accuracy `1/2`, wrong/abstention `1/4`, coverage `3/4`, selective
  accuracy `2/3`, risk `1/3`, class coverages `[1/2,1,1]`, macro-F1 `1/2`;
- Brier `0.30`, log loss `(-2*ln(0.7)-ln(0.3)-ln(0.8))/4`, argmax accuracy `3/4`;
- probability bins contain all four IDs, baseline confidence bins contain IDs
  1/3/4 and exclude abstained ID2;
- comparison recovers IDs2/3, regresses ID1, keeps ID4 correct, and has no
  neither-correct row;
- exact frozen bytes bind the prediction digest; same-basename relative and
  parent-relative evidence remain distinct and stored; original source deletion
  precedes relocated inspect/compare;
- opaque integer is disclosed only by inspect; schemas and receipt hashes are
  independently enforced;
- duplicate key, invalid 0.99 scoring vector, tampered stored evidence and
  existing destinations return the specified code/stage/exit and leave no
  successful partial output or overwritten tree;
- `docs/acceptance/validator-v1.md` accurately records candidate evidence and
  does not overclaim release, multi-label, full acceptance or completion.

## Checks and verdict

Rerun at least these exact checks on the frozen files:

```text
cargo test --locked --test conformance relocated_run_inspection -- --nocapture
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture
cargo test --locked --test conformance single_comparison_transitions -- --nocapture
cargo test --locked --test conformance comparison_compatibility_and_deltas -- --nocapture
cargo test --locked --test cli cli_compare_receipt -- --nocapture
cargo test --locked comparison::tests::typed_compatibility_facts_reject_each_runtime_axis -- --nocapture
cargo test --locked --test cli steel_thread_end_to_end -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked
ruby docs/plans/validator/verify-plan.rb
git diff --check
```

Use unchanged valid evidence where a criterion is already fully settled; stop
additional exploration once the contract and commands support a verdict. Do not
audit intersection, multi-label, T018+, installation, unrelated style or future
warning cleanup.

Return exactly one verdict:

- `Ready`: every current T015-T017 criterion is satisfied and only the authorized
  24 staged diagnostics remain;
- `Revise`: a reproducible current criterion is violated and a bounded correction
  exists;
- `Blocked`: verification cannot proceed because the candidate is unstable or an
  external condition prevents judgment.

For every finding, name the criterion, exact location, reproduction, consequence
and smallest correction. Record manifest reconciliation, independent oracle math,
commands/exits/counts, exact Clippy residual status, material limits and the final
verdict. A passing test alone does not override a code/schema/fixture violation;
an optional improvement is not a finding.

