# ReproPack 0.1 conformance fixtures

These fixtures are canonical semantic test inputs for independent implementations. They are intentionally represented as directories containing `manifest.json` and evidence files during Phase 1; archive packaging tests will wrap the same contents in ZIP containers during Phase 3.

Each case has an expected result in `expected/`. Valid cases must satisfy the schema and manifest/archive correspondence. Invalid cases must fail with the recorded category. Implementations must compare evidence bytes, sizes, SHA-256 values, redaction state, and metadata—not merely parse JSON.

Fixture IDs:

- `minimal-valid`: one selected text file.
- `complete-valid`: incident, provenance, multiple evidence kinds, and an extension.
- `redacted-valid`: replacement bytes and a redaction marker; original secret is absent.
- `selected-evidence-valid`: explicitly selected file evidence.
- `missing-required-field`: no `capture` object.
- `invalid-metadata`: unsupported capture mode.
- `corrupted-content`: manifest digest does not match evidence bytes.
- `incorrect-hash`: digest has the wrong value.
- `unsupported-future-version`: unsupported `spec_version`.
- `unknown-extension-valid`: namespaced optional extension.
- `unsafe-path`: traversal path.
- `oversized-input`: fixture metadata declares a size beyond the configured limit.

Malformed ZIP, duplicate entry, symlink, and decompression-bomb cases require binary/archive construction and are reserved for the Phase 3 security fixture set; this phase defines their expected categories in `expected/`.
