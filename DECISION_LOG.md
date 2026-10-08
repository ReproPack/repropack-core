# Decision log

## D-0001 — Three independent repositories

- Decision: Keep core, TypeScript, and Python in separate repositories.
- Context: Interoperability must be demonstrated between independent implementations.
- Reasoning: Separate histories and native dependencies prevent accidental Rust-wrapper architecture.
- Alternatives: Monorepo; Rust library with language bindings. Both weaken independence.
- Consequences: Documentation and conformance coordination require explicit synchronization.
- Date/phase: 2026-10-09 / Phase 0
- Status: accepted

## D-0002 — Proposed ZIP container plus canonical JSON manifest

- Decision: Evaluate ZIP with a required UTF-8 JSON manifest and content-addressed evidence for v0.1.
- Context: Common inspection tools, mature libraries, portability, and bounded extraction matter more than streaming.
- Reasoning: ZIP is widely available, while a simple manifest keeps semantics language-neutral. The design must reject unsafe paths and bounded resources.
- Alternatives: tar, directory-only format, CBOR/protobuf. These remain evaluation inputs for Phase 1.
- Consequences: Archive metadata and compression determinism need explicit rules; ZIP is not a trust boundary.
- Date/phase: 2026-10-09 / Phase 0
- Status: proposed

## D-0003 — No execution or replay in v0.1

- Decision: Inspection, validation, verification, and extraction never execute bundle content.
- Reasoning: Replay introduces a separate sandbox, network, privilege, and resource threat model.
- Date/phase: 2026-10-09 / Phase 0
- Status: accepted

## D-0004 — MIT license

- Decision: Use MIT consistently across all repositories.
- Reasoning: A permissive license supports independent implementations and interoperability tooling; contributor and security terms remain explicit in project policy.
- Date/phase: 2026-10-09 / Phase 0
- Status: accepted
