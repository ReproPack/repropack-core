# Phase 8 review: cross-language interoperability

Status: complete on 2026-10-09.

`node scripts/phase8_interop.mjs` produces `minimal-valid` and `redacted-valid` bundles independently with Rust, TypeScript, and Python. Each produced bundle is consumed and verified by the other two implementations. TypeScript and Python consumers emit normalized metadata; the harness compares bundle ID, evidence paths, sizes, SHA-256 digests, and redaction state.

| Fixture | Rust -> TypeScript | Rust -> Python | TypeScript -> Rust | TypeScript -> Python | Python -> Rust | Python -> TypeScript |
|---|---|---|---|---|---|---|
| `minimal-valid` | PASS | PASS | PASS | PASS | PASS | PASS |
| `redacted-valid` | PASS | PASS | PASS | PASS | PASS | PASS |

Reproduction from `repropack-core`:

```text
npm.cmd run build                  # from repropack-typescript
python -m unittest discover -s tests -v  # from repropack-python
node scripts/phase8_interop.mjs
```

Recorded environment: Rust `1.98.1`, Node `24.19.0`, Python `3.12.8`. Malformed-archive fuzzing, resource exhaustion, symlink/special-file adversarial cases, and broader threat-model coverage remain Phase 9.
