# Contributor backlog

These are real, bounded work items. A contributor should be able to start from the issue text and repository documentation without private maintainer context.

## High-value implementation work

### Streaming verification for large bundles

**Problem:** Current readers use bounded in-memory buffers; limits reduce risk but do not provide streaming semantics.

**Likely areas:** `repropack-core/src/bundle.rs`, the TypeScript and Python readers, `docs/SECURITY.md`.

**Acceptance:** Add a streaming or chunked verification API without changing the v0.1 wire format; preserve hash and limit behavior; add large-input tests and memory-behavior documentation.

**Required tests/docs:** Rust, TypeScript, and Python coverage where applicable; conformance note; benchmark or bounded-memory evidence.

**Difficulty:** Large.

### Archive-bomb and encrypted-archive fixtures

**Problem:** The security model documents archive expansion and encrypted-entry boundaries, but the shared suite lacks representative fixtures.

**Likely areas:** `conformance/fixtures`, `conformance/expected`, implementation archive readers.

**Acceptance:** Add synthetic, non-secret fixtures for encrypted entries, expansion limits, duplicate names, and malformed headers; every implementation returns the documented error category.

**Required tests/docs:** Cross-language fixture runner and security documentation update.

**Difficulty:** Medium.

### GitHub Actions failure-bundle example

**Problem:** Users need a concrete way to capture selected failure evidence without collecting credentials or the whole workspace.

**Likely areas:** `docs/CI_INTEGRATION.md`, `examples/github-actions/`.

**Acceptance:** A safe workflow example captures only declared files, validates and verifies the resulting bundle, and uploads it as an ordinary CI artifact.

**Required tests/docs:** Dry-run the command locally with synthetic files and document retention/privacy boundaries.

**Difficulty:** Small.

### Deterministic release checksums

**Problem:** The release process describes checksums but does not yet automate a coordinated checksum manifest.

**Likely areas:** `scripts/`, `docs/RELEASE_PROCESS.md`, release checklist.

**Acceptance:** Generate a deterministic SHA-256 file for named Rust, Python, and TypeScript artifacts without publishing them; add verification instructions and a test fixture.

**Difficulty:** Medium.

## SDK and ecosystem work

### Python wheel installation test

**Acceptance:** Build a wheel, install it into a fresh virtual environment, run the public API smoke test, and run `pip check` in CI.

**Difficulty:** Small.

### Stable TypeScript package exports

**Acceptance:** If npm publication is approved, define `exports`, `types`, `files`, and a public entry point; otherwise keep `private: true` and document the reference-implementation status.

**Difficulty:** Medium.

### Conformance fixture contribution tooling

**Acceptance:** Provide a command or documented validator that checks fixture paths, hashes, expected categories, and no-secret synthetic content before a pull request.

**Difficulty:** Medium.

### Additional language implementation proposal

**Acceptance:** A proposal names a maintainer, native dependencies, supported runtime, fixture plan, bidirectional interoperability path, and support horizon before code is accepted.

**Difficulty:** Medium.

## Scope boundary

Replay, environment reconstruction, implicit command execution, credential capture, hosted debugging, and ecosystem-specific adapters are not v0.1 backlog items. They require separate design and threat-model approval.
