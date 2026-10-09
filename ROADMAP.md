# ReproPack canonical roadmap

This roadmap is the execution source of truth. Status is evidence-based and must be updated when work changes.

## Status legend

- **Complete** — every acceptance and exit criterion has evidence in the repository or linked CI result.
- **In progress** — work has started, but at least one exit criterion remains open.
- **Not started** — dependencies are not yet satisfied.
- **Blocked** — progress requires a specific external decision, permission, or dependency recorded in `PROJECT_STATE.md`.
- **Deferred** — intentionally postponed; the reason and revisit condition are documented.

Current overall status: **Phase 0 complete; Phase 1 in progress**.

## Phase register

| Phase | Status | Owner repositories | Dependencies | Exit gate |
|---|---|---|---|---|
| 0 — Foundation | **Complete** | All | None | Independent repos, truthful documentation, audit, and pushed commits |
| 1 — Specification | **In progress** | Core, reviewed by all | Phase 0 | Frozen normative specification and canonical schema |
| 2 — Rust model | **Not started** | Core | Phase 1 | Tested typed model and validation behavior |
| 3 — Rust bundle operations | **Not started** | Core | Phase 2 | Safe create/read/inspect/validate/extract operations |
| 4 — Integrity and redaction | **Not started** | Core | Phase 3 | Hash verification and conservative redaction behavior |
| 5 — Shared conformance | **Not started** | Core plus all implementations | Phase 4 | Canonical fixtures and Rust fixture runner |
| 6 — TypeScript implementation | **Not started** | TypeScript | Phases 1 and 5 | Native TypeScript passes shared semantics |
| 7 — Python implementation | **Not started** | Python | Phases 1 and 5 | Native Python passes shared semantics |
| 8 — Interoperability | **Not started** | All | Phases 6 and 7 | Required cross-language matrix passes |
| 9 — Security hardening | **Not started** | All | Phase 8 | Malformed-input and resource-limit audit passes |
| 10 — Usability | **Not started** | All | Phases 3, 6, 7 | Working examples and stable user-facing errors |
| 11 — CI and packaging | **Not started** | All | Phase 9 | Real CI, package builds, and validation succeed |
| 12 — Documentation and examples | **Not started** | All | Phase 10 | Fresh-user workflow works from clean environments |
| 13 — Contributor readiness | **Not started** | All | Phase 11 | External contribution path is usable and tested |
| 14 — v0.1 release | **Not started** | All | Phases 8–13 | Final audit and release checklist pass |
| 15 — Reproduction integrations | **Deferred** | All | v0.1 | Revisit after user feedback and a separate integration threat model |
| 16 — CI/GitHub integrations | **Deferred** | All | v0.1 | Revisit after permissions, retention, and secret handling design |
| 17 — Additional implementations | **Deferred** | Core plus ecosystem | v0.1 interoperability | New language proposal has maintainer and conformance plan |
| 18 — Specification evolution | **Deferred** | Core plus ecosystem | v0.1 feedback | Compatibility-reviewed next specification version |

## Detailed phase plans

### Phase 0 — Multi-repository foundation and documentation

**Status:** Complete. **Evidence:** Three independent repositories, required documents, clean trees, verified remotes, and commits `a9670fb`, `d67b55a`, and `734b2e1`.

**Purpose:** Establish repository boundaries and durable project memory before implementation. **Work:** Create repositories; add policies, context, state, decisions, requirements, architecture, security, conformance, versioning, research, CI scaffolding, and release checklists. **Exit:** Documentation agrees, the specification is explicitly draft, and no implementation is falsely claimed.

### Phase 1 — Language-neutral specification

**Status:** In progress. **Depends on:** Phase 0. **Completed in this execution:** normative draft, schema, format decision, and semantic fixtures.

**Purpose:** Freeze semantics implementers can follow without reading Rust. **Tasks:** Compare ZIP, tar, and directory transport; define the manifest schema, evidence kinds, paths, timestamps, provenance, redaction markers, SHA-256 encoding, deterministic ordering, limits, errors, extensions, unknown fields, and compatibility.

**Artifacts:** `docs/SPECIFICATION.md`, `schema/manifest.schema.json`, format decision, examples, and initial valid/invalid fixtures. **Remaining tests:** Independent review by two implementers; examples validated by two JSON implementations; unsafe paths, duplicate entries, future versions, malformed metadata, and limits have expected outcomes. **Exit:** Another-language developer can implement the reader/writer from the specification alone and the independent review is recorded.

### Phase 2 — Rust core data model

**Status:** Not started. **Depends on:** Phase 1.

**Tasks:** Create stable Rust package structure; define typed bundle, evidence, provenance, redaction, hash, limit, and error types; implement JSON serialization/deserialization and validation; document public APIs.

**Tests:** Required/optional fields, canonical paths, timestamps, hashes, unknown fields, unsupported versions, malformed JSON, and round trips. **Exit:** Formatting, Clippy, unit tests, and integration tests pass, with behavior mapped to specification clauses.

### Phase 3 — Rust bundle operations

**Status:** Not started. **Depends on:** Phase 2.

**Tasks:** Implement the proposed container writer/reader and commands for explicitly selected capture, inspect, validate, verify, and safe extract; normalize paths; enforce entry and total-size limits; provide actionable errors.

**Tests/security:** Minimal and complete round trips; malformed archives; absolute/traversal paths; duplicate names; symlink/special-file handling; bounded extraction; no execution of bundle content. **Exit:** Every CLI command has tested behavior and documented examples use real commands.

### Phase 4 — Integrity and conservative redaction

**Status:** Not started. **Depends on:** Phase 3.

**Tasks:** Recompute SHA-256 digests and lengths; define verification failures; implement explicit redaction and markers; add advisory secret-pattern warnings without claiming perfect detection; hash only post-redaction bytes.

**Tests:** Altered/missing content, wrong hashes, redaction, secret-like values, Unicode, binary data, and warning behavior. **Exit:** Integrity and redaction semantics match the specification and do not echo sensitive data.

### Phase 5 — Shared conformance suite

**Status:** Not started. **Depends on:** Phase 4.

**Tasks:** Establish `conformance/fixtures`, `expected`, `invalid`, and `manifests`; add minimal, complete, redacted, selected-evidence, missing-field, corrupt, wrong-hash, future-version, extension, malformed-archive, unsafe-path, and oversized cases.

**Tests:** Rust runner consumes every fixture and records machine-readable results. **Exit:** Fixtures are real and reviewed, Rust passes all applicable cases, and TypeScript/Python integration instructions are complete.

### Phase 6 — Native TypeScript implementation

**Status:** Not started. **Depends on:** Phases 1 and 5.

**Tasks:** Implement native types, validation, archive I/O, hashing, safe extraction, errors, SDK APIs, and optional CLI without invoking Rust. **Tests:** Type checks, unit/integration tests, shared fixtures, malformed archive/path/limit tests, and package builds. **Exit:** TypeScript passes shared semantics and exchanges bundles with Rust.

### Phase 7 — Native Python implementation

**Status:** Not started. **Depends on:** Phases 1 and 5.

**Tasks:** Implement native models, archive I/O, hashing, validation, safe extraction, exceptions, SDK APIs, and optional CLI without invoking Rust. **Tests:** Unit/integration tests, shared fixtures, malformed archive/path/limit tests, and package builds. **Exit:** Python passes shared semantics and exchanges bundles with Rust.

### Phase 8 — Cross-language interoperability

**Status:** Not started. **Depends on:** Phases 6 and 7.

**Tasks:** Run all six producer/consumer paths and compare normalized metadata, evidence bytes, hashes, redactions, and expected errors. **Exit:** The matrix passes in CI or a reproducible harness with tool versions and fixture IDs recorded.

### Phase 9 — Security hardening

**Status:** Not started. **Depends on:** Phase 8.

**Tasks:** Threat-model review and bounded tests for malformed archives, traversal, symlinks, special files, decompression bombs, oversized metadata/files, corrupted text, malicious metadata, permissions, and secret-capture risks. **Exit:** Each threat has mitigation and test evidence; no implementation executes content.

### Phase 10 — CLI and SDK usability

**Status:** Not started. **Depends on:** Phases 3, 6, and 7.

**Tasks:** Stabilize commands and SDK names; provide inspect/validate/verify/extract examples; define exit codes and machine-readable errors; improve diagnostics without leaking evidence. **Exit:** Fresh-user workflows and failure behavior are tested.

### Phase 11 — CI and packaging

**Status:** Not started. **Depends on:** Phase 9.

**Tasks:** Add real Rust, TypeScript, and Python CI for formatting, linting, tests, conformance, builds, package validation, and dependency/security checks. **Exit:** CI runs real checks and package artifacts satisfy documented guarantees.

### Phase 12 — Documentation and examples

**Status:** Not started. **Depends on:** Phase 10.

**Tasks:** Add real demo bundles, tutorials, troubleshooting, format examples, migration guidance, and cross-repository links. **Exit:** A clean-environment user can create, inspect, verify, and safely extract a sample.

### Phase 13 — Contributor readiness

**Status:** Not started. **Depends on:** Phase 11.

**Tasks:** Add issue/PR templates, maintainer responsibilities, design review, compatibility policy, security reporting, and fixture contribution guidance. **Exit:** An external contributor can run checks and add a fixture without private context.

### Phase 14 — v0.1 release

**Status:** Not started. **Depends on:** Phases 8–13.

**Tasks:** Freeze versions; complete `FINAL_AUDIT.md`; audit repositories, examples, CI, security, conformance, package builds, and documentation; prepare release notes and tags. **Exit:** Every P0 criterion has evidence and no unfinished feature is presented as complete.

### Phase 15 — Reproduction integrations

**Status:** Deferred until after v0.1. **Tasks when resumed:** Opt-in test/build integrations with bounded inputs and selection controls. **Exit:** Consent, threat model, tests, and no implicit execution or secret capture.

### Phase 16 — CI and GitHub integrations

**Status:** Deferred until after v0.1. **Tasks when resumed:** Design artifact upload/download, retention, permissions, naming, and secret handling. **Exit:** Least-privilege and privacy behavior are tested.

### Phase 17 — Additional language implementations

**Status:** Deferred until after v0.1 interoperability. **Tasks when resumed:** Accept Go, Java, C#, or other proposals with native dependency, maintainer, fixture, and support plans. **Exit:** New implementation passes the shared suite and a bidirectional path.

### Phase 18 — Specification evolution

**Status:** Deferred until post-release feedback. **Tasks when resumed:** Review usage, compatibility issues, security findings, and ecosystem standards; propose versioned changes with migration plans. **Exit:** Updated fixtures and compatibility analysis are approved.

## Status-change rules

Changing a phase status requires updating `PROJECT_STATE.md`, linking command or CI evidence, and recording a decision when scope or dependencies change. Code existing or compilation succeeding alone never completes a phase.

## Self-directing workflow

Inspect actual state and Git status, read this phase's criteria, examine implementation and tests, run checks, compare every criterion with evidence, update state/changelog, and report gaps, risks, and the next action.
