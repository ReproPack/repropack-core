# ReproPack format specification (draft)

Status: proposed Phase 0 design; not yet frozen or implemented. Normative language: MUST is required for conformance, SHOULD is a strong recommendation that may be waived with documented reason, and MAY is optional.

## Identity and manifest

A bundle MUST identify itself as ReproPack and carry `spec_version`, a stable bundle identifier, creation timestamp, capture provenance, and an evidence index. Timestamps MUST be RFC 3339 UTC strings. Machine-specific absolute paths SHOULD be normalized or marked as sensitive. The manifest is UTF-8 JSON and its object members are semantically keyed; implementations MUST NOT rely on member order.

Each evidence entry MUST include a relative archive path, media type or declared kind, byte length, and a lowercase SHA-256 digest encoded as 64 hexadecimal characters. Evidence paths MUST use `/`, MUST be relative, MUST not contain `..`, empty components, drive letters, or NUL, and MUST be unique after normalization.

## Proposed container

The current candidate is a ZIP container with one required `manifest.json` at the root and evidence under `evidence/`. ZIP is selected for evaluation because common desktop and server tooling can inspect it, mature libraries exist in all three target ecosystems, and it supports bounded per-entry access. The proposal is not frozen until Phase 1 compares ZIP with tar and directory transport. A reader MUST reject malformed archives, duplicate logical paths, unsafe paths, unsupported entry types, and configured size/count limits.

## Evidence and redaction

Evidence kinds include logs, text, structured data, source excerpts, environment summaries, test output, and arbitrary user-selected files. A producer MUST record whether an item was selected explicitly, generated, or redacted. Redaction MUST remove original bytes before hashing and MUST emit a marker describing the redaction class without reproducing the secret. Secret detection is advisory and MUST never be described as complete. Credentials, private keys, access tokens, and unrelated private data are excluded by default.

## Integrity and validation

Validation checks schema, required fields, limits, path safety, and supported version. Verification recomputes each evidence digest and length. A changed manifest, missing entry, hash mismatch, or duplicate path is an error. A future major specification version is unsupported; unknown optional fields in a supported version are ignored or preserved according to the schema, while unknown required semantics are rejected. Extensions MUST use a namespaced key and cannot weaken safety rules.

## Determinism and compatibility

Equivalent normalized inputs SHOULD produce stable manifest ordering, stable path ordering, and stable content hashes. Producers MUST NOT include uncontrolled host paths, random IDs, or current timestamps in deterministic fields. Byte-for-byte archive identity is not promised until archive metadata and compression settings are specified. Specification version, package version, and Git tag are distinct. Additive optional fields are compatible; changed required meaning requires a new major specification version.

## Limits and errors

Implementations MUST expose bounded limits for entries, manifest bytes, entry bytes, total uncompressed bytes, nesting, and extraction path length. Errors MUST identify category and safe context without echoing secrets. Extraction MUST create files only below a caller-selected destination and SHOULD refuse symlinks and special files.

## Security boundary

No operation in v0.1 executes scripts, binaries, hooks, format handlers, or commands from a bundle. Replay is explicitly out of scope and requires a future threat model, sandbox, privilege policy, network policy, and security review.
