# Complete the interrupted T016 comparison assignment

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/008-comparison-developer-to-coordinator.response.md`.

Save the full substantive handoff at that path before returning. In chat, return
only the response path, its SHA-256 and a terse status. Do not edit governance,
dispatch, plan, artifact-index, session-note or Fizzy files.

## Read first

Read these files completely before editing:

- `/Users/dowwie/.codex/AGENTS.md` and repository `AGENTS.md`;
- `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/dispatches/validator-t015-t017/006-coordinator-to-comparison-developer.prompt.md`,
  which remains the complete authoritative T016 scope, behavior and evidence
  contract;
- `docs/dispatches/validator-t015-t017/007-coordinator-incomplete-handoff.md`,
  which records the honest partial state;
- every governing source and T015 handoff named by prompt006;
- all current partial T016 source, schema and test files before changing them.

The owner already authorized a fresh Terra-high/fork-none replacement after an
incomplete retained-context handoff. You are now the sole writer. Preserve useful
partial work, replace or simplify incomplete work as needed, and complete the
unchanged T016 contract. Do not begin T017.

## Current partial state and required correction

The exact current partial hashes are:

| Artifact | SHA-256 |
|---|---|
| `src/comparison.rs` | `7cf0e1db8c5a5f65c433112c615bc68d6c4de294346ae6b184e7c8a334652429` |
| `src/artifacts.rs` | `95a8c71d82dd180828e50810a483f94ee8be00826ef26e56cce700719e33068e` |
| `src/app.rs` | `43be4af8e98afef8c6f5733932075305cc58ae3d3e24d3718d824e17fbdc9d57` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `schemas/v2/comparison.schema.json` | `d2ff60da8fbc59f07fb555abe758457d5ac8e3b670f3c4091a4cd8eb3a442035` |
| `tests/conformance.rs` | `3159a6af2fe6839e59fac18df8c0a6b549ff7ab9e8916bfcf5a3bdb52a8cd3b6` |
| `tests/cli.rs` | `5fad6fa4f66572f9af4a7ed4d23f9c0474046b0aa6ff0acd7a03fe6cafe4f9e5` |

Stop and report if these differ before your edit. `cargo check --locked` currently
exits 0 with the inherited 26 owner-staged warnings, but that is not T016
completion evidence. The required named T016 tests do not yet exist.

Complete every requirement in prompt006. In particular, correct these known
partial gaps without widening scope:

1. Policy/model/source question differences are permitted and reported. Do not
   reject a pair merely because its decision policy differs. Compatibility still
   enforces every exact axis named in prompt006, including actual numerical and
   metric semantics rather than a guessed JSON key.
2. Use concrete typed T016 comparison models in `src/model/single_label.rs` and a
   pure comparison boundary. A generic `serde_json::Value` assembly that leaves
   required fields untyped is not the required data-model contract.
3. Implement explicit source-definition/configuration/observation/preparation
   differences and general configuration differences, including equal local
   source IDs whose referenced definitions differ.
4. Implement concrete raw/final/probability paired metric records, both statuses
   and populations, candidate-minus-baseline deltas only for two defined values,
   null reasons otherwise, metric directions, and answered-population overlap.
5. Implement exact correctness category IDs/counts, changed-outcome IDs/count,
   and a typed full class-plus-abstention transition table whose total is N.
6. Refactor the existing private atomic publisher at the smallest internal seam so
   evaluation and comparison share no-replace/private-temporary/cleanup mechanics.
   Do not maintain a second copy of that workflow; preserve the evaluation's five
   entries and all existing safety behavior.
7. Add all three exact named task tests from prompt006 with assertions that prove
   the stated cases through real application and CLI paths. A passing compile or
   schema-shaped example does not replace those cases.

All constraints, required commands, staged-Clippy treatment, allowed file scope
and handoff contents in prompt006 remain binding. This continuation does not reset
or weaken them and does not create an extra review gate. Save response008 only
after the full local T016 evidence exists.

