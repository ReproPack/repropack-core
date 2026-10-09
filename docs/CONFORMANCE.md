# Conformance architecture

The canonical repository owns fixtures organized under `conformance/fixtures` and `conformance/expected`. Each case records its purpose, expected result, and semantic assertions. Valid manifest files are checked against `schema/manifest.schema.json`; valid evidence hashes and sizes are calculated from the checked-in bytes.

The current semantic catalog covers minimal and complete bundles, redaction, selected files, missing fields, invalid metadata, corrupted content, wrong hashes, future versions, unknown extension data, unsafe paths, and oversized input. Implementation tests consume all applicable canonical cases.

The recorded Phase 8 matrix covers Rust, TypeScript, and Python producers and consumers in all six directed language paths for the minimal and redacted fixtures, on Ubuntu and Windows. Passing means equivalent manifest meaning, evidence bytes, sizes, hashes, redaction state, and expected failure categories—not merely that a file opened.

Archive-specific malformed-input cases and future implementations remain subject to their own tests; current results do not certify every possible input or integration.
