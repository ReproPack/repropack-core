# Phase 5 review: shared conformance

Status: complete on 2026-10-09.

## Delivered

- The canonical fixture catalog is consumed by a Rust integration test rather than by hand-picked tests.
- Every catalog case is loaded from its checked-in manifest and evidence bytes.
- Valid cases are packaged through the real ZIP writer, read through the real ZIP reader, and verified against size and SHA-256.
- Invalid cases assert the expected stable category: invalid manifest, hash mismatch, unsupported version, unsafe path, or limit exceeded.
- The redacted fixture asserts the replacement marker is present and the original secret-like value is absent.
- Expected assertion metadata is required to be non-empty for each case.

## Evidence

```text
cargo test --test conformance
cargo test --all-targets -j 1
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
node scripts/validate_phase1.mjs
```

All passed. The runner covers 12 catalog cases: five valid and seven invalid.

## Scope boundary

The runner establishes shared semantic fixtures and the Rust reference behavior. TypeScript and Python consumers are not claimed yet. Cross-language interoperability and broader malformed-archive/security fixtures remain later phases.
