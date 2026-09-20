# Combined T029-T031 acceptance-input handoff

Status: **Ready for owner acceptance**. T029, T030 and T031 each have a complete
independent Ready verdict. No expected value was fitted to Validator output, and
no performance evaluation has run before this boundary.

## T030 independent native oracle

- Repaired author handoff: [response007](../validator-t030/007-oracle-author-to-coordinator.response.md),
  SHA-256 `4398e24f7bc93b1576cf72cfe9fa184a148ba5482817923f8e5afe6a6c407b64`.
- Independent Ready verdict: [verdict008](../validator-t030/008-verifier-to-coordinator.response.md),
  SHA-256 `c8349c4212eb1a19295e44fb853fe459b1c9dc48e4c5c6592a0d79765d7f3b3e`.
- Oracle script: `scripts/acceptance/oracle_chord.py`, SHA-256
  `773272be28d6db7f7c6f2440c66f0a1cb6fc709240adf185911acdfc3cebffcd`.
- Protected expected result:
  `/Users/dowwie/.local/share/validator/acceptance/chord630/expected.json`,
  SHA-256 `33fad4ba44b782a869cc1f5f6cdc2fb30ae9321588ed49225154b7b67ce1eb03`.
- Accepted source manifest/map remain respectively
  `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e`
  and `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4`.

The verifier independently reconstructed the corrected 383/220 labeled
partition and every repaired challenge result from the protected raw evidence.
Two deterministic private runs matched the protected bytes; parent permissions,
`/tmp` mode, source metadata and owner-only output modes remained intact.

## T029 oracle-isolated preparation

- Repair handoff: [response007](../validator-t029/007-preparation-developer-to-coordinator.response.md),
  SHA-256 `c04f0f2c16f89d1a35f2e44e3f0610ea3d3ca02761ec3d263241c3e193600f75`.
- Independent Ready verdict: [verdict008](../validator-t029/008-verifier-to-coordinator.response.md),
  SHA-256 `6921a1cce5e06dc94aa04e909fce2c5530e946bb343e005c8f55392a38605cfb`.
- Workspace and protected preparation script SHA-256:
  `083ad9461c82564a6f0091f31291e8dc9df193f681610f4b3540dd5f67cfa7ed`.
- Protected canonical root:
  `/Users/dowwie/.local/share/validator/acceptance/chord630/`, mode `0700`;
  every canonical file is mode `0600`.

The final canonical identities are:

| Artifact | SHA-256 |
|---|---|
| `golden.json` | `9a8896ea9afefd9bef0e3610595d4469d617fadd371529fd344f88437aa0389b` |
| `choice.json` | `f6683631465bd9f8ea7b0772e75b6699fed8cc161cacc5bd582faa5d32e7f572` |
| `score.json` | `325db741173eec3c33dc5ad038733000ccbbd36df3d19cd4a24b82c92dc15df3` |
| `config-full.json` | `4bf3016eacdc6d6cfe559a4182929d4ddff6ffed08b23a66edc3a54a728ed95b` |
| `config-random400.json` | `faa707f9e91dc2a17befac4ad922f5a989ae4f9b779210972f794e660180719c` |
| `config-supplements230.json` | `52556988eef040f685237ede92230039489fca3eea6fb1549cc4a0f6ad7fe653` |
| `config-targeted.json` | `a5142ec439eb6407c54e5840af6bcbad39b924813c52a25132b2a351bf4f0951` |
| `choice-random400.json` | `c8091c703c394617f79fcf30a43f16b56b827e8ac1d55b0e47f3b6009d9d21f4` |
| `choice-supplements230.json` | `54a3c215b808a7edba8f53b14d4754fb4a79061e5df05444ea522cf3ed4e2e36` |
| `choice-targeted.json` | `38e1ec36e757fccb92562af78f80610a753912c5120a4508b828698cf62c0a62` |
| `score-random400.json` | `b56d7059e13e67dcb428654e9a361de32dbab15bfe73afaea14bc30e1d862bb4` |
| `score-supplements230.json` | `40b0ac8f2e062eadd785966f823addf306d43da53a57bd58182499316ef32af1` |
| `score-targeted.json` | `312822f59919c9f4d8f02df2f93e5eae041a1c2794ea7381f3bd23c72b65ba47` |
| `preparation-script.py` | `083ad9461c82564a6f0091f31291e8dc9df193f681610f4b3540dd5f67cfa7ed` |
| `preparation-receipt.json` | `1b13c4cc86878abca909edcf1efb765cb818363578dbb2a9be09a62f873564b8` |

The independent verifier confirmed 630 manifest-ordered direct raw bindings per
route, the complete Score evidence indices, two byte-identical regenerations and
all eight full/subset checks through the accepted installed executable. The
preparation author and verifier remained isolated from T030 oracle results.

## T031 minimal public multi-label oracle

- Final cell correction: [response005](../validator-t031/005-oracle-author-to-coordinator.response.md),
  SHA-256 `cde94b723699236d87a4b212093d585a2b0fc7bfbc4746c5f0246afa15a4d4e2`.
- Independent Ready verdict: [verdict006](../validator-t031/006-verifier-to-coordinator.response.md),
  SHA-256 `725b741a32d5f9daee12197b77f11c022442dff59b9b1d74517cf6097752ab30`.
- Final `expected.json` SHA-256:
  `c97e3a5b0debed6ae4dbf11ec2a67339c9009bb0953763bf56bde77ef297ca81`.
- Final `manifest.json` SHA-256:
  `a61aa36e748e5cc0063d33594e07ef35d0c7bc00c09bd087d9fac570cc8f91b0`.

The final eight-file bundle is `tests/fixtures/acceptance-multi/`. Its six frozen
input hashes are recorded in verdict006. The independent verifier reconciled the
literal-minimum two-episode roles, all raw/final hard and probability values,
comparison categories and transitions, then proved the last correction changed
only the four authorized label-B cells and manifest digest entry. Neither author
nor verifier ran Validator or read private T029/T030 results.

## Current integration state and decision

The already installed T027 executable remains:
`/Users/dowwie/.local/share/validator/install/t027-178fccf38838/bin/validator`,
SHA-256 `4bacbc5947bb70218725916965439837b2afa53d16a04f37ef00a31509a85dde`,
version `validator 0.1.0`.

Owner decision requested: accept the exact T029/T030/T031 input and oracle
identities above as the combined acceptance-input bundle. After acceptance, the
authorized next boundary is T032's minimal installed-operator documentation and
independent usability review; product scoring remains frozen.
