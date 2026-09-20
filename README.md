# Validator

- Version `0.1.0`: a Rust CLI for single-label and multi-label classification evaluation.
- Commands: `check`, `evaluate`, `inspect`, and `compare`; use `--help` for arguments.
- [Architecture overview](architecture.md) explains the system with component and state diagrams.
- [Installed executable on this workstation](/Users/dowwie/.local/share/validator/install/t027-178fccf38838/bin/validator).
- Verified build: 159 Rust tests, warning-denied Clippy, locked release build, and the network-denied offline suite passed.
- [Verification record](docs/acceptance/validator-v1.md), [saved-evidence workflow replay](docs/acceptance/workflow-replay.md), [specification](docs/specs/validator-v1.md), and [JSON Schemas](schemas/v2/).
- Saved-evidence workflows are stabilized on the installed binary. Pristine-agent acceptance, production promotion, and the final full definition-of-done closure remain separate claims.
