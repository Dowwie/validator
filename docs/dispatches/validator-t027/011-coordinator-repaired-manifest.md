# Frozen repaired T027 candidate manifest

Status: frozen for the same verifier's focused F1 recheck. Base candidate is
manifest008 SHA-256
`1fb631264e5b6a396031be184ab6eb320786529cb53ff00aa976f50d77d01fa5`.
Every artifact listed there remains identical except the three source files below.

| Artifact | SHA-256 |
|---|---|
| `009-verifier-to-coordinator.response.md` | `d1265d622b8c36eaf0ad6fd6cdb54f79bd4ff855f14bf49a669d37bf8bd7c199` |
| `010-coordinator-to-structure-developer.prompt.md` | `ceabe4a1199314987f64e43b86527ff65656b4d6f16cf0394f172ed2dfd6595c` |
| `010-structure-developer-to-coordinator.response.md` | `0062924b688487c1b928bb7110d9e6bb9a68b4984bf400f39865c7c1635feb51` |
| `../../../src/model/single_label.rs` | `f03500766388d85106a8792bf119ac2df8db63673c90c81d57b2b17fc62d4f0d` |
| `../../../src/model/multi_label.rs` | `935b15094c2f8e8817287efad55290ce5f8fd6d78fb25f6d335b7d3eaf30c033` |
| `../../../src/app.rs` | `3dbac840a4ddce89b8c8f04ab82583114f380e969cf65c244af411031e0e9f9d` |

The two model report builders now accept only a tuple of three model-owned
`ArtifactSnapshot` references. App extracts those snapshots from InputArtifacts at
evaluation/replay call sites. No model module imports the artifact/I/O module.
Report bytes, manifest ordering, publication/replay, schemas, numerical behavior,
public API, tests and build inputs are unchanged.

Focused report/privacy, replay/tamper, DM01 and offline CLI checks pass. The clean
Clippy, 44/12/103/0 ordinary suite, release build, identical deny-network locked-
offline suite, diff and plan gates all pass. Recheck only F1, affected report/
replay behavior, exact identities and the absence of model-to-I/O dependency.
