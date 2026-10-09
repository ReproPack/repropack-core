# Phase 10 review: usability

Status: complete on 2026-10-09.

## CLI contract

The Rust CLI supports `capture`, `inspect`, `validate`, `verify`, and `extract`. Every command accepts a leading `--json` flag. Human output remains concise; JSON mode returns structured success objects or `{ "error": { "code", "message" } }` objects suitable for automation.

Exit codes are stable:

| Code | Meaning |
|---:|---|
| 0 | Success |
| 2 | Usage or missing-argument error |
| 3 | Input, validation, archive, or integrity error |
| 4 | Unexpected internal error |

Error categories include `usage-error`, `input-error`, `invalid-manifest`, `unsupported-version`, `malformed-archive`, `unsafe-path`, `limit-exceeded`, `missing-entry`, `unexpected-entry`, `hash-mismatch`, and `size-mismatch`. Messages contain paths and safe diagnostics only; evidence bytes are never printed.

## Workflow evidence

The documented capture, inspect, validate, verify, and extract workflow passed against `minimal-valid`. JSON inspect and verify passed, extraction produced the expected evidence file, and a missing-input verification returned JSON `input-error` with exit code 3.

The TypeScript and Python native SDK entry points are documented through their exported `createBundle`/`readBundle`/`verifyBundle`/`extractBundle` and `create_bundle`/`read_bundle`/`verify_bundle`/`extract_bundle` APIs. CLI duplication is intentionally deferred until a language-specific packaging phase.
