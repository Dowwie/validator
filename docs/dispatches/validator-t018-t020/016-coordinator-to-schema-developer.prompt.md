# Repair stored evidence verification during inspection

Role/model: active sole check/inspection-schema developer,
`gpt-5.6-terra`, high reasoning, `fork_turns: none`. Do not delegate or restart
orientation.

Repository: `/Users/dowwie/MyProjects/validator`.
Governing schema assignment: prompt014. Owner authorization: prompt015.

Required repair response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/016-schema-developer-to-coordinator.response.md`.

Save the full repair-specific handoff there and still save the complete prompt014
schema handoff at its original response014 path before returning. Do not edit
governance, plans, dispatches, artifact-index, session notes, acceptance records
or Fizzy.

Before Rust edits, read `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`,
the two task branches in inspection/rebuild, `StoredRun::verify_evidence`, the
accepted comparison replay usage and existing relocation/tamper tests.

Current boundary hashes:

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `d08e50c7ab32bffa0e9791bdcb32ffa27f8c60e0ed8fc586b23d643e18588ca0` |
| check schema | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| inspection schema | `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176` |
| completed report schema | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| `tests/conformance.rs` | `ebc92b53736900dcdd2466ab339385c7136f5d3d558b89deb037610dbceb246a` |
| `tests/cli.rs` | `b0969aa48a4590fa24c3ef3dfd23e6da38e60aa2ca3c8414c3ed83822c0d46cb` |

Stop and report an exact mismatch before editing.

## Exact repair

The isolated failure is:

- `cargo test --locked --test cli relocated_run_inspection -- --nocapture`
  returns `E_IO` before schema validation after relocation;
- the stored run and contained `evidence/0.bin` are intact;
- both generalized inspection rebuild branches call `load_evidence_bindings` on
  stored `predictions.json`, resolving the original relative evidence path after
  that original is unavailable;
- accepted comparison replay already calls
  `stored.verify_evidence(evaluation.sources())`.

Make the smallest `src/app.rs` change so both single-label and multi-label
inspection/rebuild branches use the existing stored-run verifier and the verified
contained bindings to rebuild the report. Keep original paths as provenance only.
Do not add a fallback, duplicate path resolution, copy/delete evidence, change
artifact formats, weaken report equality, change diagnostics/tolerances, or touch
comparison/policy/public APIs/dependencies.

Preserve the single-label relocated reproduction with original inputs/evidence
unavailable. Extend the existing real multi-label relocated-inspection case with
bound evidence, demonstrate success after relocation/original removal, then
mutate contained evidence and assert the existing precise failure contract. Reuse
the relevant existing tamper/binding helper or assertion; do not build an audit
matrix.

Run Cargo commands serially:

```text
cargo test --locked --test cli relocated_run_inspection -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture
cargo fmt --all -- --check
git diff --check
```

Then finish every prompt014 schema command and response. Response016 must state
the root cause, exact source/test changes, failed-before/passed-after evidence,
tamper result, commands/exits and hashes, and confirm report schema and other
production files are unchanged. Do not end the assignment merely to report the
authorized repair.
