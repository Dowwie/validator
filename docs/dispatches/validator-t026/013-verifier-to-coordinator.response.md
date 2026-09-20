# T026 repaired exact-filter focused recheck

Verdict: **Ready**.

The repaired frozen candidate closes all six proof gaps from verdict010. The
focused assertions are derived from fixed inputs, exercise the required public
paths, and preserve the settled T026 matrices and candidate behavior. No current
contract defect remains within this bounded recheck.

## Candidate identity and scope

`012-coordinator-repaired-manifest.md` has the required SHA-256
`4fc0552b87d895a5597977a12884fb09b6c1e5c2145152ede8551b54abb85895`.
Every artifact listed in manifest012 reconciles exactly before review. I repeated
the repaired test and unchanged production/test hashes after execution; they
remain equal to the frozen manifest.

I read complete verdict010, repair prompt011 and developer response011. I
inspected only the six repaired proof areas in `tests/conformance.rs`, their fixed
input helpers, and the justified matrix/replay regressions. I did not inspect
T027, T028 or optional hardening.

## Six repaired proofs

1. **S15 full origins:** the filter inspects both fixed episode IDs through the
   public API and asserts their expected classes and Q1/Q2 source bindings. The
   self-comparison asserts fixed source counts plus the Q1/Q2 model and complete
   fixed configuration values on both baseline and candidate sides. Expected
   values come from the literal fixture definitions, not the current result.
2. **S24 relocation:** after moving the run and deleting the original golden,
   predictions, config and both evidence files, the filter retains successful
   public inspection, performs public comparison on the relocated run, checks the
   receipt path and proves the actual `comparison.json` was published.
3. **S26 receipts:** a real evaluation receipt is retained from a second
   evaluation of the fixed inputs. Its result path is asserted against the actual
   absolute `report.json` path and its SHA-256 against an independent hash of the
   exact report bytes. The independent comparison receipt path/hash proof remains.
4. **S27 privacy:** public inspection of a fixed unknown episode produces a real
   typed `E_ID` diagnostic. The diagnostic's actual machine serialization is
   checked for absence of both sentinels, alongside the existing report privacy
   assertions and successful exact selected-input inspection.
5. **M17 ordering:** the filter asserts the exact `transitions.per_label` sequence
   `[A,B]` before checking A `present -> absent` and B `absent -> present` counts.
6. **M18 conditional metrics:** the fixed three-row inputs independently yield two
   answered rows per side, one-row overlap and one correct answered row per side.
   The filter therefore correctly fixes both selective exact-match values at
   `0.5`, both statuses at `defined`, delta `0.0`, and the answered populations.
   It also rejects root/final winner or improvement representations.

## Executed evidence

Each focused filter executed exactly one nonzero test and exited 0:

- `case_s15`
- `case_s24`
- `case_s26`
- `case_s27`
- `case_m17`
- `case_m18`

The justified regressions `artifact_adversarial_matrix`,
`replay_binding_and_result_tampering`, and the real CLI
`publication_and_privacy_matrix` each executed one test and exited 0.

Inventory commands exited 0 and list exactly 91 conformance tests and 11 CLI
tests. I reused the repaired developer's frozen full-suite evidence: 44 library,
11 CLI, 91 conformance and zero doc tests pass; format and diff checks pass; the
locked release evidence is unchanged. I did not rerun warning-denied Clippy: the
only repaired file is the frozen test file, inspection found no source-identity or
diagnostic uncertainty, and response011 records exactly the accepted 17
production plus three duplicate lib-test staged diagnostics.

No source, schema, test, fixture, governance, plan, index, session note,
acceptance record or Fizzy state was edited during this recheck. This Ready verdict
recommends owner acceptance of repaired manifest012 for T026; it does not authorize
T027/T028 integration or claim their work was reviewed.
