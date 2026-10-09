# Phase 4 review: integrity and conservative redaction

Status: complete on 2026-10-09.

## Delivered

- `Bundle::verify` recomputes the exact byte length and lowercase SHA-256 for every indexed evidence entry.
- Verification rejects changed content with distinct size-mismatch and hash-mismatch errors.
- `redact_text` and `redact_bytes` provide explicit, conservative replacement for high-signal private-key blocks, bearer tokens, and common secret assignments.
- `redact_manifest_entry` updates replacement bytes, size, digest, redaction status, and reason as one operation.
- Invalid UTF-8 is preserved and reported as `binary-input-not-scanned`; it is never presented as safely redacted.
- Warning messages contain no scanned values.

## Evidence

The core test suite has 19 passing tests, including altered content, redaction without secret echo, private-key and bearer-token handling, Unicode preservation, binary input, manifest metadata updates, bundle verification after redaction, and missing-entry errors.

Validation commands:

```text
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets -j 1
node scripts/validate_phase1.mjs
```

All passed for this phase. The Phase 1 fixture audit remains reproducible and passing.

## Explicit limitations

Secret detection is advisory and incomplete. A clean result does not prove that evidence contains no secrets. Binary content is not scanned. The API does not automatically capture, upload, or execute evidence; callers must explicitly choose replacement bytes and then create a bundle.

Streaming decompression hardening and broader malformed-archive fuzzing remain scheduled for Phase 9.
