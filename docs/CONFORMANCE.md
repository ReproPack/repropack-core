# Conformance architecture

The canonical repository will own fixtures organized as `conformance/fixtures`, `conformance/expected`, `conformance/invalid`, and `conformance/manifests`. Each case records purpose, spec version, expected result, and semantic assertions. Initial cases cover minimal and complete bundles, redaction, selected files, missing fields, invalid metadata, corrupted content, wrong hashes, future versions, unknown optional extensions, malformed archives, unsafe paths, and oversized input.

The matrix eventually tests Rust↔TypeScript, Rust↔Python, and TypeScript↔Python in both directions. Passing means equivalent manifest meaning, evidence bytes, hashes, redaction state, and expected failure category—not merely that a file opened.
