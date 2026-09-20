# T020 ordinary-lint blocker before candidate freeze

Status: owner disposition is required before freezing or dispatching the combined
T018-T020 review. Prompt018 completed every requested numerical/CLI assertion and
all non-lint gates, but unmodified warning-denied Clippy exits 101 for 20 already
authorized staged dead-code diagnostics plus three ordinary T020 diagnostics. The
test-only writer correctly did not expand into production cleanup.

## Complete preserved evidence

Prompt018 response SHA-256 is
`55f141be479a3a09ed0454ab3f82181ba7dcb91e78a4c8c7c769dec3d0b30077`.
All six exact filters exist, select one test and pass. Full locked results are 44
library, 7 CLI, 15 conformance and 0 doc failures. Formatting, release build and
diff checks pass. The fixed schemas and production hashes from responses013/014/
016 remained unchanged; only `tests/conformance.rs` and `tests/cli.rs` changed to
`f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291`
and `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275`.

The hard oracle now proves the full two-row rational accounting, empty answer,
abstention, all-abstained and empty-population cases. Equal binary aggregates with
different exact-set accuracy are proven through two published runs. Marginal
finite/zero/infinite/scaling/bin cases pass. The real CLI proves schema validation,
one-document stdout, privacy, exact opaque inspection, immutable existing-output
refusal, relocation and contained-evidence tamper rejection.

## Exact ordinary diagnostics

Unmodified Clippy identifies only these ordinary diagnostics in addition to the
20 owner-authorized staged private-symbol diagnostics:

1. `src/app.rs:369`: `&evidence` is a needless borrow; the parameter is already
   `&[EvidenceBinding]`.
2. `src/app.rs:390`: the same needless borrow in the multi-label report branch.
3. `src/model/multi_label.rs:463`: an identity `.map(|(label, value)| (label,
   value))` before `collect::<BTreeMap<_, _>>()`.

These are direct Clippy-specified removals with no behavior, interface, tolerance,
schema or test-oracle change. They were introduced in the T020 production path and
are not covered by owner001/correction006's staged-warning allowance. No
suppression, fake use or widened visibility is acceptable.

## Requested smallest correction

Authorize one sole production writer to make only the three mechanical removals,
then run formatting, unmodified Clippy, the six exact filters, full locked tests,
release build and diff check as the final frozen candidate. Reuse prompt018's
unchanged evidence but rerun the full boundary because two production files change.
No cleanup beyond these three diagnostics, no staged-dead-code work and no new
review scope is requested.

The combined independent review remains unopened and Card184 remains at T020. On
the corrected complete gates, the coordinator will freeze exact hashes and proceed
directly to that single review without another coordinator suite rerun.
