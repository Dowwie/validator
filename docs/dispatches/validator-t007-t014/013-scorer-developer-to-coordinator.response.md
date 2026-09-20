# T010 corrected schema handoff

Dispatch 013 corrections are complete. This supersedes response 012.

Corrected SHA-256: golden `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b`; predictions `ef4cf1ccfd80653afc63501fe98900fe897c9eea86e862ae72344e4b2bafab3a`; config `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3`; conformance test `09ef6a87d7c944dca0a94fb566e6837f0b55acffeb0049e68a3b5031d12617ff`. Cargo manifest/lock remain `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` and `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e`.

The schemas now use exact `type: "labels"`; golden uses task-kind conditionals to reject class/labels target mismatches; scored-choice requires nonblank question ID; observation variants have closed scalar versus unit-interval/map forms; nonblank patterns cover contract strings/maps; probabilities and threshold maps are nonempty with nonblank keys. Vocabulary equality, source references, sums, digest equality, duplicate JSON keys, and alignment remain runtime checks.

`cargo test --locked --test conformance input_schema_contract -- --nocapture` exited 0 with 1 passing test after the correction. It retains direct schema-engine execution and legal multi-label validation. The prior 012 checks remain green; current compilation retains 161 known unwired dead-code warnings, unsuppressed. This response itself requires the coordinator’s index reconciliation because the dispatch forbids worker index edits.
