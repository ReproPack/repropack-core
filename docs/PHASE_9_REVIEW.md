# Phase 9 review: security hardening

Status: complete on 2026-10-09.

## Threat and mitigation evidence

| Threat | Mitigation | Evidence |
|---|---|---|
| Malformed/truncated ZIP | Native readers reject bad signatures, central-directory bounds, decompression failures, and CRC mismatches | Rust malformed-archive test; TypeScript and Python malformed-archive tests |
| Traversal, absolute, backslash, and unsafe components | Canonical `evidence/` path allowlist; extraction resolves below a destination root | Rust, TypeScript, and Python fixture/tests |
| Duplicate entries | Reader tracks archive names and rejects duplicates | Rust reader and Phase 3 archive tests; TypeScript/Python readers |
| Symlinks and special files | Archive metadata and existing extraction path components are rejected | Rust extraction checks; TypeScript/Python extraction checks; directory/special-entry tests |
| Decompression/resource exhaustion | Configurable entry count, manifest, per-entry, total uncompressed, and path limits are checked before content use | All three readers; limit tests in each implementation |
| Corrupted content or hash substitution | Exact size and SHA-256 are recomputed after reading | Cross-language fixture tests and altered-content tests |
| Malicious metadata/unknown fields | Strict manifest field and enum validation, unsupported-version rejection | Shared invalid fixtures and all implementation suites |
| Malformed text and secrets | UTF-8 redaction is conservative; binary input warns; warnings never contain values | Redaction tests in Rust, TypeScript, and Python |
| Content execution or hooks | APIs only read, verify, write, and extract bytes; no subprocess or handler execution exists | Code review and full test path audit |

## Validation evidence

- Rust: 21 unit/integration tests, Clippy, formatting, and Phase 1 audit pass.
- TypeScript: typecheck, build, and 7 tests pass.
- Python: 6 tests, compileall, and all 12 canonical fixtures pass.
- Phase 8 six-direction interoperability matrix remains passing for minimal and redacted fixtures.

## Residual risks

The implementations use bounded in-memory buffers rather than streaming decompression, so configured limits reduce but do not eliminate memory pressure. Secret detection remains heuristic and incomplete. Broader fuzzing and an external security review remain recommended before release.
