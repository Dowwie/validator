# Recheck the refrozen two-episode T031 oracle

Role/model: independent verifier, `gpt-5.6-sol`, high reasoning,
`fork_turns: none`. Remain read-only and do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #193 — Freeze Validator multi-label practical oracle](http://localhost:3006/1/cards/193).

Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t031/004-verifier-to-coordinator.response.md`.

Read global/repository AGENTS, the T031 contract, verdict002 and repaired author
handoff response003. The frozen response003 SHA-256 is
`e2798adb1d86860a2943ba539e9385b7a4e6470466ba487902355108ee295e52`;
`expected.json` is
`0737c59d45b371c46d38cda71696bc16d8df49125ca72f96238548c4eb47ff98`.
Reconcile all eight files under `tests/fixtures/acceptance-multi/` and the seven
non-self manifest hashes before reviewing content.

Recheck only the repaired T031 boundary. Independently confirm that the bundle
contains exactly two episodes with these required roles: one overlapping known
failure/recovery and one answered-empty-set/whole-abstention regression; both
full runs are compatible, the targeted candidate selects only the first, and
the intersection excludes the second. Recompute every expected raw/final hard
count and metric, per-label result, exact rational Brier value, analytic log-loss
identity, status, selected/answered ID set, comparison category, changed ID and
both per-label transition tables. Confirm the four formerly wrong B transition
cells are now correct, every transition table sums to its comparison population,
complete marginal populations match each run, and no stale four-case value
remains.

Use only the normative T031 multi-label sections, published input schemas, direct
inspection and temporary arithmetic probes. Do not run Validator or production
scoring, read private T029/T030 artifacts/results, build a maintained oracle,
add cases or broaden review. Reuse verdict002's settled conclusions where the
repair did not change their criterion, but bind all evidence to this candidate.

Save one complete Ready/Revise/Blocked verdict with exact candidate identities,
independently executed checks and any smallest criterion-backed finding. Run
`git diff --check`, save the response, then return only its path, SHA-256 and
verdict.
