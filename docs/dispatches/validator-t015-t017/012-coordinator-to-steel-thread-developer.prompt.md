# Implement the fixed T017 steel-thread architecture gate

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/012-steel-thread-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, dispatch,
plan, artifact-index, session-note or Fizzy files.

## Read first and role boundary

Read completely before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T017.json`;
- `Verification and acceptance` in `docs/specs/validator-v1.md`;
- `Structure acceptance checks` and its typed-boundary context in
  `docs/specs/validator-data-model.md`;
- `Steel-thread decision` in `docs/plans/build-validator.md`;
- `docs/plans/validator/execution-contract.md`;
- `docs/dispatches/validator-t015-t017/001-owner-to-coordinator.prompt.md`;
- T015 response002 and response005;
- T016 response010 and compatibility completion011;
- the current CLI/process tests, schemas and application behavior you exercise.

T015 and T016 are locally complete and remain pending one combined independent
review after this task. You are the sole writer for permanent T017 fixtures, the
one process test and the candidate acceptance-evidence record. The later Sol-high
verifier is read-only except for its dispatch response.

Do not change production source, schemas, Cargo inputs or tolerances in this
assignment. If the fixed thread exposes a product defect, preserve the failing
reproduction, save a truthful incomplete handoff naming the broken seam and stop;
do not add a fallback, weaken an oracle or silently repair outside task ownership.

## Fixed input identity

The governing hashes are:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T017.json` | `8ce5fccc8389c5bc15b44fbb148ba822ea576d7d44cfaa70407d20d63c5cfbe4` |
| `docs/plans/build-validator.md` | `0be8489b0cf5873ca40ae22b8b5fd39425631390d3d151ad3484e59b6c29aafb` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/execution-contract.md` | `e389163c6aa6e26b0800849293f61b5d718bad7edfdd55aad5ab3ee00aed94f2` |
| owner prompt001 | `db1ba26b41b411478550ec18e29e93240ff46ea4196b4cd094d186183a1b2142` |
| T015 response002 | `9e1b78221884ecbcab9dfa60e2677bf0b0b279978261f40172d1f511c5fc231c` |
| T015 response005 | `914cfd24a936595720febcb1dbad30bbb81a27f0200d1410acfc6615300a4238` |
| T016 response010 | `888e4646fe2455acaf690231642c43c152cfe0011546bb34d42cbee0dfb10c3e` |
| T016 response011 | `9a74991343606f64728ed7147bdedaee9de5cef1420e9ee1c100392e3f3818db` |

The frozen source/schema/test hashes are:

| Artifact | SHA-256 |
|---|---|
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `src/app.rs` | `28418611219e65eb32ffcdf6ae470f48bca944f68112e9997ee87c1f134b7ad7` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/comparison.rs` | `a0288c9d79f116b03925802a52771dff0e4894506a23701b9b15e6cb36b02961` |
| `src/evaluation.rs` | `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891` |
| `src/evaluation/single_label.rs` | `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `5cff2210c215bc84e6a95777649a6e0700a3547056fbb8a93365f9d1e6b1a019` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| check schema | `675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad` |
| comparison schema | `24002dce44ffe08c9900c71a2db3a5c81e7b5b274bd05fd647c46468bf2c75d6` |
| error schema | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| inspection schema | `5e7990c05d9f87a29768c73d6d39a1579d39a9c842ce790cb780ff616182fcfa` |
| receipt schema | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| report schema | `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46` |
| `tests/cli.rs` | `c374ad731ea697679775f61fb9656fa024cf7e9893f34be17a1fb08694c58f79` |
| `tests/conformance.rs` | `bdf526a6a60fde605c6c65e5691fd7b4e397642b0166cf216b1a169f05d3e353` |

Stop and report any pre-edit mismatch.

## Owned artifacts and exact fixture

Create only the T017 artifacts plus the required response:

- `tests/fixtures/steel-thread/golden.json`;
- `tests/fixtures/steel-thread/baseline.json`;
- `tests/fixtures/steel-thread/candidate.json`;
- `tests/fixtures/steel-thread/config.json`;
- `tests/fixtures/steel-thread/expected.json`;
- `tests/fixtures/steel-thread/source-a/evidence.bin`;
- `tests/fixtures/steel-thread/source-b/evidence.bin`;
- the `steel_thread_end_to_end` case and bounded helpers in `tests/cli.rs`;
- `docs/acceptance/validator-v1.md` as candidate architecture-gate evidence,
  explicitly pending independent review and making no release/project-completion
  claim.

Freeze these UUID-sorted episode IDs:

```text
01995c20-7d00-7000-8000-000000000001
01995c20-7d00-7000-8000-000000000002
01995c20-7d00-7000-8000-000000000003
01995c20-7d00-7000-8000-000000000004
```

Use vocabulary `[A,B,C]`, references `[A,A,B,C]`, baseline outcomes
`[A,abstention,A,C]`, candidate outcomes `[B,A,B,C]`, source kind `classifier`
and policy `as_recorded`.

Both prediction artifacts contain, in episode order, these probability vectors
and reported-confidence observations:

```text
[0.7,0.2,0.1], [0.7,0.2,0.1], [0.6,0.3,0.1], [0.1,0.1,0.8]
confidence: [0.8,0.8,0.3,1.0]
```

The baseline abstained row, episode 2, also retains categorical observation
`{x:0.5,y:0.49}` with a matching source definition. Put opaque integer
`9007199254740993` in episode 2 input so relocated inspection proves exact
spelling. Bind different synthetic same-basename files through
`source-a/evidence.bin` and a genuinely parent-relative path to
`source-b/evidence.bin`. Freeze exact golden bytes, independently compute their
SHA-256, and bind both predictions to it. Never use production scoring to create
or modify `expected.json`.

`expected.json` must state the hand-derived oracle with exact IDs/formulas:

- baseline `D=2,E=1,U=1`, accuracy `1/2`, wrong-class rate `1/4`, abstention rate
  `1/4`, coverage `3/4`, selective accuracy `2/3`, selective risk `1/3`, class
  coverages `[1/2,1,1]`, macro-F1 `1/2`;
- Brier `0.30`, log loss `(-2*ln(0.7)-ln(0.3)-ln(0.8))/4`, argmax accuracy `3/4`;
- all four selected IDs in probability-bin accounting;
- baseline confidence-bin accounting contains episodes 1, 3 and 4 and excludes
  episode 2;
- recovered IDs 2 and 3, regressed ID 1, both-correct ID 4, neither-correct empty.

Use integer/fraction fields where possible and an explicitly hand-calculated
finite decimal only for the logarithmic oracle. Compare floats only with the
already frozen fixture tolerances.

## One production-shaped process case

Implement exactly the named real-binary test:

```text
cargo test --locked --test cli steel_thread_end_to_end -- --nocapture
```

It copies fixtures into a temporary source tree, preserves relative and
parent-relative evidence resolution, then:

1. runs `check` and validates its single stdout document against the check schema;
2. evaluates baseline and candidate into immutable five-entry runs, validates
   receipts/reports, independently hashes exact result bytes and verifies exact
   snapshot/evidence bindings;
3. relocates both runs, deletes the temporary original input/evidence tree,
   asserts it is unavailable, and proceeds only from stored artifacts;
4. inspects baseline episode 2, validates inspection schema, and proves the
   categorical observation and exact opaque integer;
5. compares relocated runs, validates comparison/receipt schemas and hashes exact
   `comparison.json` bytes;
6. asserts every expected count/value/status, bin population/ID set and transition
   ID set from `expected.json`, plus absence of winner/significance fields;
7. proves routine check/report/receipt/comparison/error output omits the opaque
   input and synthetic evidence contents; only inspect discloses selected input;
8. proves same-basename evidence bytes remain distinct, bound and sufficient.

Within that same scenario, use bounded temporary mutations to assert:

- duplicate serialized JSON key: exit 2, `E_SCHEMA`, no success directory;
- nonconforming `0.99` scoring vector: exit 2, `E_PROBABILITY`, no success
  directory, while the retained `0.99` observation succeeds;
- changed stored evidence: exit 2, `E_PROVENANCE`, no new success output and no
  original-path lookup;
- existing evaluation or comparison destination: exit 3, `E_OUTPUT_EXISTS`,
  existing tree byte-for-byte unchanged and no partial sibling success directory.

Validate every output through its published schema. Assert diagnostic codes,
stages and exits, not only `!success`. Use bounded helpers; do not add another
application implementation, shell oracle, fixture generator, network dependency
or test-only product API.

## Candidate record and gates

Write `docs/acceptance/validator-v1.md` with the fixed hypothesis/path/decision;
fixture, schema, source and built-binary SHA-256 hashes; hand-derived oracle and
actual values; command results; relocation/privacy/binding/schema/receipt/failure
evidence; exact staged Clippy limit; and status `candidate evidence pending
independent verifier and owner acceptance`. State that it is not a release,
multi-label, broad acceptance or project-completion claim.

Run and record:

```text
cargo test --locked --test cli steel_thread_end_to_end -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked
ruby docs/plans/validator/verify-plan.rb
git diff --check
```

Clippy is expected to exit 101 only for the current 24 owner-staged production
dead-code diagnostics. Record every residual symbol and real status. Any new
warning, suppression, fake use, public expansion, source/schema drift, failed
behavior gate or changed tolerance is a blocker.

The response must map every pass/failure condition to exact assertions, record
commands/exits/results and final hashes, confirm production/schema hashes did not
change, identify any blocker, and confirm no T018/multi-label/general acceptance
work. This is candidate evidence for independent review, not an implementation-
completion or owner-acceptance claim.

