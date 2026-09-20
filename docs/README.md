# Validator documentation

- [Artifact index](artifact-index.md) identifies each document's purpose and authority.
- [Architecture overview](../architecture.md) explains the implemented components, state transitions, capabilities, and boundaries.
- [Validator v1 specification](specs/validator-v1.md) defines the reconciled `1.2-draft` contract, approved as the implementation baseline by the owner's build instruction.
- [Shared data model](specs/validator-data-model.md) defines typed task structures, invariants, and Rust ownership boundaries.
- [Acceptance criteria](specs/validator-v1.md#product-acceptance-criteria) and [definition of done](specs/validator-v1.md#definition-of-done) define the required implementation evidence, including the frozen practical case.
- [Session decisions](../session-notes.md) records the rationale behind the specification.
- Fizzy is authoritative for task state, ownership, blockers, and next actions.

## Authority vocabulary

- **Governing**: instructions that control work in this repository.
- **Ratified**: an owner-approved contract that governs implementation; approval does not assert implementation exists.
- **Proposed**: a reviewable specification; publication does not imply owner ratification or implementation.
- **Reference**: supporting material that does not independently impose requirements on v1.
- **Decision record**: durable context and rationale; not an operational task list.
- **Index**: discovery and authority routing; not a substitute for the linked contract.
