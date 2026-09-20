# T027 focused F1 recheck verdict

Verdict: **Ready**.

The repaired candidate closes F1. Neither task model imports or accepts the
I/O-owned `artifacts::InputArtifacts` type. Both report builders receive only
model-owned `ArtifactSnapshot` references, while `app` extracts those snapshots
from the artifact aggregate at each evaluation and replay call site. Focused
report/privacy, replay/tamper, shared-path, and offline CLI evidence passes with
no new required defect.

## Frozen identity

`011-coordinator-repaired-manifest.md` has the required SHA-256
`178fccf388382f34aa68c9de5cae669c7ab331e5e5612775a28a1a6c6a3639f9`.
The base manifest008 has SHA-256
`1fb631264e5b6a396031be184ab6eb320786529cb53ff00aa976f50d77d01fa5`.

Before execution, every delta hash in manifest011 matched. Every manifest008
entry other than the three declared replaced source files also matched. After
focused execution, manifest011 retained its required identity and every listed
delta artifact still matched; the post-run reconciliation reported zero
mismatches.

I read developer response010 and inspected only F1, the affected report-builder
signatures, their call sites, artifact ownership, and the required focused
behavior. I reused response010's fresh clean full-gate evidence as authorized by
dispatch012 and did not reopen the broader T027 review.

## F1 closure

- `src/model/single_label.rs` imports `ArtifactSnapshot` from
  `crate::model::common` and has no `crate::artifacts` or `InputArtifacts`
  reference. Its `assemble_report` signature receives one ordered tuple of
  `(&ArtifactSnapshot, &ArtifactSnapshot, &ArtifactSnapshot)` and destructures it
  as golden, predictions, and config before building the manifest.
- `src/model/multi_label.rs` has the same model-owned signature and no
  artifact-module dependency. It preserves the manifest order of golden,
  predictions, config, followed by evidence bindings.
- A direct production source check found no `crate::artifacts` or
  `InputArtifacts` occurrence in either task model.
- `src/app.rs` remains the orchestration boundary. Both new-evaluation branches,
  both inspection report-rebuild branches, and both stored-run replay branches
  extract `golden()`, `predictions()`, and `config()` from `InputArtifacts` and
  pass only those snapshot references to the task report builder.
- `src/artifacts.rs` continues to own `InputArtifacts`, file reads, exact-byte
  snapshots and hashing, evidence loading/copying, integrity verification, and
  non-overwriting publication. No filesystem or publication responsibility moved
  into the model.

This satisfies the package-direction requirement in
`docs/specs/validator-v1.md:1085-1092`: model no longer depends on an I/O module,
and app supplies required values through arguments while coordinating artifact,
model, evaluation, replay, and publication work.

## Focused execution

All prescribed commands executed one nonzero test and exited 0:

```text
cargo test --all-features --locked --lib model::single_label::tests::report_sources_and_privacy -- --nocapture
# 1 passed; 43 filtered out

cargo test --all-features --locked --test conformance replay_binding_and_result_tampering -- --nocapture
# 1 passed; 102 filtered out

cargo test --locked --test conformance structure_dm01 -- --nocapture
# 1 passed; 102 filtered out

cargo test --locked --test cli offline_cli_contract -- --nocapture
# 1 passed; 11 filtered out
```

The first check preserves report manifest/source/privacy behavior. The second
confirms rebuilt-report equality, replay, and tamper rejection after the signature
change. DM01 confirms the shared public paths remain operational for both task
kinds. The offline CLI contract exercises saved-input check/evaluate/inspect/
compare workflows and therefore covers report serialization, publication, and
replay through Cargo's built executable.

Response010's post-repair full evidence remains applicable to these exact source
identities: format and warning-denied Clippy passed; the ordinary and
network-denied locked-offline suites both passed 44 library, 12 CLI, 103
conformance, zero binary, and zero doc tests; locked release build, diff check,
and plan verification passed. The focused recheck found no mismatch that
justified repeating those full gates.

No source, test, schema, fixture, plan, governance, index, acceptance record,
session record, or Fizzy state was edited during this recheck. This Ready verdict
is limited to the repaired frozen T027 candidate and closes the sole F1 finding
from verdict009.
