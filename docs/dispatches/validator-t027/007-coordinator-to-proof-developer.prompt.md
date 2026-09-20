# Complete exact DM structure filters and the locked offline CLI proof

Role/model: fresh sole developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #190 — Verify Validator module boundaries and offline suite](http://localhost:3006/1/cards/190).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t027/007-proof-developer-to-coordinator.response.md`.

Save the full substantive handoff before returning only path, SHA-256 and status.
You are the sole writer. Do not edit governance, plans, index, acceptance/session
records or Fizzy. Do not touch T028 protected inputs or start T029/T030. Do not
delegate.

Read `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, complete T027,
owner prompts001/004, output1 response002, the governing specification/data-model
sections, execution contract and manage-dev-team skill. Treat output1's clean
candidate as the fixed source baseline. This output owns exact proof, not another
architecture pass.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `002-structure-developer-to-coordinator.response.md` | `3eb3225a406491626343972cec004d0b9ea585f596f176a1cfcb2753f97de7cb` |
| `src/model.rs` | `992adf37259839f71dbffa931ac1a341894e90f29283bf34f0f393f1a762ce56` |
| `src/model/common.rs` | `f5f626654b33c63255de26c0e4a07ba79696b980042fa240e2741d73e54bfc8c` |
| `src/model/single_label.rs` | `1acd5af54ef01b2cc49d6e8c9c7a973ee04e529e4000e4cf0b2edf8fc1a10222` |
| `src/model/multi_label.rs` | `43e80100dd9a5c59232ac48fe1925c6bd1d4f2192b86b492027052f8776bd873` |
| `src/validation.rs` | `1ae4707c48c2310e45340ccbc24109eee616d74eaf03a664c247905ba840ebbb` |
| `src/validation/wire.rs` | `1a6d6887dc3ed2fd1a02c337592cf43dd97b59c2dcf6f361459e396634f27ee4` |
| `src/app.rs` | `f741c03b17788303c7ae79ff43a11f9605cb63a6bc565a98d40f5f55b131d64b` |
| `tests/conformance.rs` | `d0be253475cc718ad7ed67d50e9096b7dc2a4a5a1ddccdb096662e06d4f05b00` |
| `tests/cli.rs` | `042d6933411fe35f0a9c0b8bcfb22645fc1bc6c9eb6bca3f8c68df9cb3232f4a` |

## Exact proof scope

Write only `tests/conformance.rs` and `tests/cli.rs` unless a required behavioral
test reproduces a production defect. If it does, freeze exact input/expected/
actual evidence and stop for coordinator routing before any production edit.

Add or complete exactly named, nonzero filters `structure_dm01` through
`structure_dm12`. Each must execute discriminating real public API or process
behavior for its ratified row:

- DM01 both task kinds through the shared admission/orchestration/artifact path;
- DM02 cross-kind targets, probabilities and policies rejected through concrete
  checked task-specific boundaries;
- DM03 reordered submitted label sets preserve scores/canonical order, duplicates
  reject before set conversion;
- DM04 `[0.9,0.8]` is valid unchanged marginals and invalid categorical scoring;
- DM05 empty set, abstention, missing row and missing marginal key remain distinct;
- DM06 zero-episode restriction retains probability applicability;
- DM07 evidence-bearing abstentions retain distributions/observations and correct
  selected versus raw-answered population/ID disclosures;
- DM08 observed categorical `[0.5,0.49]` round-trips unnormalized while the same
  scoring vector rejects, with no observation-to-signal/fallback path;
- DM09 scalar/auxiliary observations round-trip with definitions/source references
  without generating target types or metrics;
- DM10 broken observation definitions/preparation bindings fail before scoring;
- DM11 equal per-label aggregates can coexist with different exact-set accuracy;
- DM12 unsupported counts, mixed task kinds, partial labels and per-label
  abstentions reject without extension hooks or inferred interpretations.

Reuse complete existing public scenarios/helpers without duplicating the workflow,
but a wrapper name alone is not evidence. Each filter must contain or call
discriminating assertions for every assigned subcase and execute exactly nonzero.
Preserve every accepted filter name and fixture/oracle.

Owner004 governs structural proof: map every DM row to its real behavioral test
and, where applicable, explicit direct review of concrete source signatures,
callers, ownership and shared paths. DM01 needs both workflows plus shared-path
review; DM02 needs public rejections plus evaluator/caller type review. Do not add
compiler-test infrastructure, source-text-matching tests, public exports of private
models or redundant runtime copies to imitate static proof. Record the static
review evidence in response007 for the independent verifier.

Add/complete exact nonzero `offline_cli_contract` using Cargo's built executable
and saved public synthetic inputs only. It must cover the public operations without
network, credentials, a global installed binary or private data. Run the complete
suite inside the exact prepared network-denial sandbox with Cargo offline mode:

```text
sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline
```

Do not weaken the sandbox or substitute a normal online-capable run. Stop with the
exact access seam if a real dependency appears.

Reconcile every earlier mandated public test ID against Cargo's actual `--list`
output and the accepted coverage map. A complete inventory plus passing full suite
is enough for unchanged tests; do not rerun all historical filters individually.
Run each new DM/offline filter individually and preserve its nonzero result.

Finish serially with the exact owner gates:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline
git diff --check
```

Every command must exit 0 with no warning allowance. Run `ruby
docs/plans/validator/verify-plan.rb` for public mapping/link reconciliation. Do not
change Cargo/toolchain versions, schemas, fixtures, numerical outputs, tolerances,
publication/replay behavior or opaque JSON semantics.

Response007 must include per-DM behavioral assertions, necessary static review,
offline-process proof, complete test inventory/coverage reconciliation, exact
commands/exits/counts, changed/unchanged hashes and any pending T028-T035 proof
obligations that T027 does not claim. Return only when output2 and all final gates
are complete for one frozen combined independent review.
