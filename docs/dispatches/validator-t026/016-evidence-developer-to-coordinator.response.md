# T026 resolved-inspection-source completion handoff

This completes the owner-authorized correction from prompt016. The T028 bundle
and all five settled T026 repairs remained untouched. No contract mismatch
appeared after resolving the selected source from the verified stored report.

## Addendum015 correction mapping

- `src/app.rs` adds the required `InspectionResult.source: Value` field. After
  stored report replay verification and selected raw prediction lookup, `inspect`
  parses that raw row's `source_id` and clones exactly that one entry from the
  verified report `sources` map. It preserves the existing raw prediction,
  opaque input, report episode evidence, and independently decoded evaluation
  `configuration`. A missing resolved source remains an `E_INVARIANT` internal
  consistency failure; no original path is dereferenced.
- `schemas/v2/inspection.schema.json` now requires `source`, defines the existing
  report-source layout (kind/model/configuration, optional question, observations,
  optional preparation, and stored `evidence/*` paths), and limits the multi-label
  branch to `classifier`. This mirrors the existing report serialization without
  adding a version, registry, or alternate validation path.
- `case_s15` now asserts each Q1/Q2 inspection exposes its fixed classifier kind,
  model, configuration, empty stored evidence/observation maps, plus the retained
  selected row binding. Existing comparison-side origin assertions remain.
- `case_e08` asserts inspected source observation definition, preparation
  descriptor, and the three verified stored evidence paths in addition to its
  retained 0.99 observation and prepared scoring vector. The existing multi-label
  abstention inspection test asserts classifier source definition and observation
  definition, so both task kinds validate real inspected sources and the strict
  schema.
- The CLI steel-thread relocation inspection now asserts the resolved selected
  source definition and stored evidence path after originals are removed. Privacy,
  relocation, and routine-output checks remain covered by their existing filters.

## Executed checks

Focused commands all exited 0:

```text
cargo test --locked --test conformance case_s15 -- --nocapture
cargo test --locked --test conformance case_e08 -- --nocapture
cargo test --locked --test conformance check_inspection_schema_contract -- --nocapture
cargo test --locked --test conformance multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection -- --nocapture
cargo test --locked --test conformance case_s24 -- --nocapture
cargo test --locked --test conformance case_s27 -- --nocapture
cargo test --locked --test cli steel_thread_end_to_end -- --nocapture
cargo test --locked --test cli publication_and_privacy_matrix -- --nocapture
```

The literal final boundary also completed:

```text
cargo fmt --all -- --check                                      # exit 0
cargo test --all-features --locked                              # exit 0: 44 library, 11 CLI, 91 conformance, 0 doc
cargo clippy --all-targets --all-features --locked -- -D warnings # exit 101: only accepted 17 production + 3 duplicate lib-test staged dead-code diagnostics
cargo build --release --locked                                  # exit 0
git diff --check                                                # exit 0
ruby docs/plans/validator/verify-plan.rb                        # exit 0: 35 contracts, 433 source blocks, 63 conformance cases, 12 structure checks, 8 ACs, 5 DoD clauses
```

## Frozen identities

| Artifact | SHA-256 |
|---|---|
| `014-owner-to-coordinator.prompt.md` | `a19fed980f87ee4c100029b3b7d636ee84bba0548704727067819c77a6d5a216` |
| `015-verifier-to-coordinator.response.md` | `5721fb83161c839005056236129e4b681169e5c903b081ff69006e403c2e645f` |
| `src/app.rs` (changed) | `8326278b0739f7a55f6dcb09d0c97683e17ad4dc2431e4214391cfa5d24407ad` |
| `schemas/v2/inspection.schema.json` (changed) | `6677b1689d581744636cf480cea690a55c566415e794f995e34c34f1b9da3883` |
| `tests/conformance.rs` (changed) | `339971adbc8a1914065b026cf2c372b51ec0ca1f4901a75eccb0c5eba5cfa59c` |
| `tests/cli.rs` (changed) | `042d6933411fe35f0a9c0b8bcfb22645fc1bc6c9eb6bca3f8c68df9cb3232f4a` |
| `src/artifacts.rs` (unchanged) | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `Cargo.toml` (unchanged) | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` (unchanged) | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `rust-toolchain.toml` (unchanged) | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

The candidate is ready for the same verifier's focused R1/source-field recheck.
