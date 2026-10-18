# Contributor work packages

These packages are intentionally documented without creating GitHub issues. They are preparation material for future maintainers or contributors and do not imply that work has been assigned or funded.

## Package A — Large-bundle streaming verification

**Goal:** Add a consumer API that verifies evidence in bounded chunks without materializing every evidence byte.

**Scope:** Preserve the v0.1 wire format, canonical paths, digest semantics, and extraction safety. Cover Rust first, then document equivalent TypeScript/Python behavior.

**Acceptance:** API design reviewed; hashes and length limits match existing readers; large synthetic inputs pass; memory behavior is measured; malformed, encrypted, and oversized entries remain rejected.

**Evidence required:** API documentation, unit tests, interoperability note, benchmark or bounded-memory measurement, and security review.

## Package B — Independent archive robustness corpus

**Goal:** Expand malformed ZIP, encrypted-entry, duplicate-name, and expansion-limit coverage with independently generated synthetic inputs.

**Acceptance:** Inputs contain no secrets; each expected error category is documented; at least two implementations consume the corpus; CI remains bounded.

**Evidence required:** Fixture provenance, hashes, expected results, Rust/TypeScript/Python test output, and a resource-limit explanation.

## Package C — Safe GitHub Actions example

**Goal:** Provide a minimal workflow that captures only explicitly selected failure evidence.

**Acceptance:** The workflow creates, validates, verifies, and uploads a bundle using synthetic files; it does not capture the workspace or secrets; retention and permissions are documented.

**Evidence required:** Local dry run, workflow review, security limitations, and a clean-environment walkthrough.

## Package D — Fixture contribution validator

**Goal:** Make it easy to add a canonical fixture without private maintainer knowledge.

**Acceptance:** A documented command checks paths, hashes, byte lengths, expected categories, deterministic ordering, and synthetic-content rules.

**Evidence required:** Positive and negative examples, CI invocation, and contributor documentation.

## Package E — Additional implementation proposal

**Goal:** Evaluate a new language implementation without weakening conformance.

**Acceptance:** Proposal names a maintainer, supported runtime, native dependency policy, fixture plan, bidirectional interoperability path, and support horizon before code is accepted.

**Evidence required:** Approved design note and shared-suite results.
