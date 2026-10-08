# Testing strategy

Tests will be layered: manifest unit tests; archive round trips; invalid-input tests; property tests for path and limit handling; redaction warning tests; and cross-language fixture tests. Security tests must include traversal, duplicate names, symlinks, special files, malformed UTF-8/text, oversized entries, decompression bombs, altered bytes, wrong hashes, unsupported versions, and malicious metadata. Temporary directories must be bounded and cleaned up.
