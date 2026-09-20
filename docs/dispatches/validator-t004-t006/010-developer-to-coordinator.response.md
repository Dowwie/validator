# T004-T006 repair 010 handoff

I read verifier response 007 and manifest 006. The frozen hashes match the specified baseline before any repair edit.

The two required findings are understood:
1. Removing serde_json arbitrary_precision fixed the private marker collision but caused the duplicate-key scanner to reject syntactically legal 1e400 before opaque retention or semantic E_OBSERVATION handling.
2. The existing T006 result lacks a checked EvaluationConfig model record, Population dataset identity/count invariants, and a sealed/fallible evaluation constructor.

No implementation files were changed during this repair handoff. The bounded repair requires restoring arbitrary_precision together with raw_value and changing JsonNumber to distinguish a parser-issued numeric token from the literal marker object without weakening duplicate scanning. It also requires the specified checked configuration/population/evaluation construction changes and new actual-entry regressions. I did not make a partial change that could regress the already-passing null and marker boundaries.

No command, process, or resource blocker occurred. The current candidate remains at manifest 006 hashes, and no acceptance is claimed.

