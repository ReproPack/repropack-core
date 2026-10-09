# Conformance architecture

The canonical repository owns fixtures organized under `conformance/fixtures` and `conformance/expected`. Phase 1 fixtures are directory representations containing `manifest.json` and evidence bytes; archive-level malformed, duplicate, symlink, and decompression-bomb fixtures are deferred to Phase 3 when archive readers exist. Each case records purpose, expected result, and semantic assertions.

The current fixture set covers minimal and complete bundles, redaction, selected files, missing fields, invalid metadata, corrupted content, wrong hashes, future versions, unknown optional extensions, unsafe paths, and oversized input. Valid manifest files are checked against `schema/manifest.schema.json`; valid evidence hashes and sizes are calculated from the checked-in bytes.

The matrix eventually tests Rust↔TypeScript, Rust↔Python, and TypeScript↔Python in both directions. Passing means equivalent manifest meaning, evidence bytes, hashes, redaction state, and expected failure category—not merely that a file opened.
