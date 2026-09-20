# T022 developer handoff — stopped before implementation

Status: **blocked before editing**.

The governing T022 developer dispatch requires an exact pre-edit hash match. Its
declared SHA-256 is:

```text
28ecbba28f7d6c7c6a133a9018862abc0e96733b8da617b39aa8493a40ba9608
```

The SHA-256 of the dispatch available at
`docs/dispatches/validator-t021-t023/003-coordinator-to-t022-developer.prompt.md`
is:

```text
286e6ecc56ccff63e6fd2a5aa352514ef22b796dc36f1b0986a18392c83dec9b
```

All other hashes reached before that mismatch matched their declared values:

| Artifact | Observed SHA-256 |
|---|---|
| T021 owner prompt | `73f5ebe3a192a0baedf9787b55a356d49b03211fa093edfcabf47034cecc15b9` |
| T021 response | `33be455a22ae14c936121cf28c4e7de1ab0ef1c6fcd7daa20d4a4d75a5d71a6e` |
| `src/comparison.rs` | `a0288c9d79f116b03925802a52771dff0e4894506a23701b9b15e6cb36b02961` |
| `src/model/multi_label.rs` | `c019632dfef36ae441bc00cc3be1c7a58b539e9081107066d893b2993807f94f` |
| `src/app.rs` | `3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `tests/conformance.rs` | `f80c6318e61882bc3b577756c9ed618451650282bb751206fbd0ccb77ba7b2e2` |
| `tests/cli.rs` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` |

The schema file is located at `schemas/v2/comparison.schema.json`, while the
initial hash command used the dispatch's abbreviated `schemas/comparison.schema.json`
path. No implementation, schema, test, CLI, or governance files were changed.
