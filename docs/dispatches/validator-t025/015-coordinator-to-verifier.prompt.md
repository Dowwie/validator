# Recheck the four repaired T025 proof gaps

Resume as the same independent Sol-high verifier. Do not delegate and do not edit
the repository. Save the full Ready/Revise/Blocked verdict to:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/015-verifier-to-coordinator.response.md`.

Reconcile repaired manifest014 SHA-256
`2e908298bef177b9af9ffdd30f2562ad0b4a4f612dd43ec9a24a1e18a15a8e8e`
and every listed hash before reviewing. Read owner prompt012, acknowledgment012,
superseding developer instruction013 and full repair response011. This is the
focused recheck in the existing single repair cycle. Reuse every settled result
from verdict010. Do not restart broad T025 review, reopen product/schema/fixture
work, or inspect T026+.

## Required recheck

1. Confirm the shared direct-ratio test boundary now verifies independently
   expected population count/unit/scope at every caller. Check exhaustive
   single/multi semantics and S03's selected N=4 metrics versus answered G=3
   selective accuracy.
2. Independently correct verdict010's illustrative S12 vector. Under binary64
   `min(floor(10*h),9)`, `next_down(0.9) * 10` rounds to exactly 9, so the combined
   vector is `[2,3,3,3,3,3,3,3,2,4]`. Inspect the stronger actual repair: all 29
   values must run in isolated public evaluations, with one count in the fixed
   expected bin and zero in the other nine, plus mean signal, metadata, episode
   inclusion and signal-one/bin9 assertions. Confirm the expectation table was
   fixed independently rather than derived from output.
3. Confirm M02 now asserts both labels' identities, supports, TP/FP/FN/TN, null
   undefined F1 `0/0` and ratified answered population metadata without weakening
   existing aggregate evidence.
4. Confirm M14 now supplies a class target in the multi-label golden record with
   an otherwise legal multi-label prediction and asserts public `E_CONFIG` plus
   absent output, while retaining the other required subcases.

Run only the focused and justified affected regressions using the existing target:

```text
cargo test --locked --test conformance exhaustive_single_label -- --nocapture
cargo test --locked --test conformance exhaustive_multi_label -- --nocapture
cargo test --locked --test conformance case_s03 -- --nocapture
cargo test --locked --test conformance case_s12 -- --nocapture
cargo test --locked --test conformance case_m02 -- --nocapture
cargo test --locked --test conformance case_m14 -- --nocapture
cargo test --locked --test conformance full_numeric_conformance -- --nocapture
cargo test --locked --test conformance -- --list
```

Each focused filter must pass one with 73 filtered, and the list must remain 74.
Reuse response011's full-test, Clippy, format/diff evidence and response008's
release evidence; no repeated full suite, Clippy or release build is needed absent
a concrete new failure.

Return **Ready** only if all four findings are corrected on the exact repaired
candidate and no blocking affected regression remains. Return **Revise** with a
criterion, reproduction and smallest correction only for a demonstrated remaining
defect. Return **Blocked** only for a manifest/evidence condition that prevents
judgment. Stop when this focused evidence supports the verdict.
