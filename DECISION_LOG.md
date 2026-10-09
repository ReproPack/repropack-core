# Decision log

## D-0001 — Three independent repositories

- Decision: Keep core, TypeScript, and Python in separate repositories.
- Context: Interoperability must be demonstrated between independent implementations.
- Reasoning: Separate histories and native dependencies prevent accidental Rust-wrapper architecture.
- Alternatives: Monorepo; Rust library with language bindings. Both weaken independence.
- Consequences: Documentation and conformance coordination require explicit synchronization.
- Date/phase: 2026-10-09 / Phase 0
- Status: accepted

## D-0005 — Freeze ZIP plus JSON Schema manifest for v0.1

- Decision: v0.1 bundles use ZIP with root `manifest.json`, evidence under `evidence/`, UTF-8 JSON, and a Draft 2020-12 schema.
- Context: Phase 1 required a practical cross-language contract that is inspectable with common tools and supports bounded extraction.
- Reasoning: ZIP has mature libraries in Rust, TypeScript, and Python; JSON is easy to inspect and validate; strict path/type/limit rules address archive risks. Tar, directory-only transport, and CBOR/protobuf remain future alternatives.
- Consequences: Byte-for-byte archive reproducibility is not promised; archive-level security tests are required before implementation completion.
- Date/phase: 2026-10-09 / Phase 1
- Status: accepted for v0.1, pending independent review

## D-0006 — Strict ordinary fields and namespaced extensions

- Decision: Unknown ordinary manifest members are invalid in v0.1; optional extension data belongs under URI-keyed `extensions`.
- Reasoning: Strict core fields catch misspellings while URI namespaces permit forward-compatible, non-semantic additions.
- Consequences: Future revisions must define migration rules rather than silently changing required meaning.
- Date/phase: 2026-10-09 / Phase 1
- Status: accepted for v0.1, pending independent review

## D-0002 — Proposed ZIP container plus canonical JSON manifest

- Decision: Evaluate ZIP with a required UTF-8 JSON manifest and content-addressed evidence for v0.1.
- Context: Common inspection tools, mature libraries, portability, and bounded extraction matter more than streaming.
- Reasoning: ZIP is widely available, while a simple manifest keeps semantics language-neutral. The design must reject unsafe paths and bounded resources.
- Alternatives: tar, directory-only format, CBOR/protobuf. These remain possible future transports but are not v0.1 interchange formats.
- Consequences: Archive metadata and compression determinism need explicit rules; ZIP is not a trust boundary.
- Date/phase: 2026-10-09 / Phase 0
- Status: superseded by D-0005

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
