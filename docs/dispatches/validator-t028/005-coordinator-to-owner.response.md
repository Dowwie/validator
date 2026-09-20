# T028 protected source-freeze handoff for owner acceptance

The protected Chord630 source-freeze candidate is independently **Ready**.

Exact decision chain:

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `b5f685c91d8dd06ea9a5bbe453b7c296fd807524ae5a91938c5367778a87ebec` |
| `003-source-developer-to-coordinator.response.md` | `b5388afd357e04383c487bc11d1e2c92c0e9a56b55e035decbf6a105b8945fa3` |
| `004-coordinator-to-verifier.prompt.md` | `ff915b219845a8059acde8b4c56a0b6bf2e931a09d8f386487e6a3faf216100c` |
| `004-verifier-to-coordinator.response.md` | `631bade386fe30b91b6b971bcc7cb2690e9e2626e4545ea1e16fc4dcab8d9e4b` |

Protected candidate identity:

| Artifact | SHA-256 | Bytes | Mode |
|---|---|---:|---:|
| `/Users/dowwie/.local/share/validator/acceptance/chord630/source-manifest.json` | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` | 720587 | `0600` |
| `/Users/dowwie/.local/share/validator/acceptance/chord630/source-id-map.json` | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` | 432460 | `0600` |

The fresh verifier independently established:

- exact 630-ID equality across source manifests, requests, references, both raw
  result routes, candidate manifest and ID map;
- 603 `REVIEWED`, 23 `WITHHELD`, 4 `UNSCORED`, with all 27 non-labeled reason
  pairs preserved, and exact 400/230 cohort membership;
- all 1,353 source/frozen SHA-256 and byte-size pairs, pre/post external-source
  stability, 23 route/role aggregates and all three ordered route digests;
- 630 recoverable native Choice and 630 recoverable native Score records while
  preserving the historical 627/347 parsed-projection limitation;
- 630 distinct version-7 UUIDs mapped to the same 630 IDs with stable persisted
  bytes and no generated review-time identities;
- frozen labels `NO_MATCH`, `UNCERTAIN`, `MATCH`, boundaries `0.5`/`1.5`, and
  `bisect_right` ties `0.5 -> UNCERTAIN`, `1.5 -> MATCH` under Python 3.14.6;
- exactly 1,355 protected files / 277,040,705 bytes, 10 directories mode `0700`,
  all files mode `0600`, UID 501, zero symlinks, full containment, no self-hash
  and no unrecorded file.

The verifier used read-only stdlib probes, executed no copied operational source
or prompt, calculated no metric, modified no Chord/protected byte and found no
criterion failure. `verify-plan.rb`, 432 public relative-link checks and
`git diff --check` pass.

The private metadata still honestly records candidate/not-yet-owner-accepted
status. I have not indexed the private manifest/map as accepted inputs, changed
acceptance records, or started T029/T030. Card189's independent-review step is
complete and owner acceptance remains its first incomplete step. The exact
caffeinate hold PID84732 remains active.
