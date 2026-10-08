# Requirements

R-01: A conforming bundle is readable by an independent implementation from the specification alone.

R-02: Required metadata, evidence paths, lengths, and SHA-256 hashes are validated before trusted use.

R-03: Inspection and verification do not execute any bundle content.

R-04: Extraction rejects traversal, absolute paths, duplicate paths, symlinks/special files where unsupported, and configured resource limits.

R-05: Capture defaults to explicit selection and warns about possible secrets; it never promises secret absence.

R-06: Unknown optional fields are forward-tolerant only when they do not change required semantics; unknown future versions are rejected safely.

R-07: Rust, TypeScript, and Python implementations remain operationally independent.

R-08: Conformance tests assert semantic equivalence, expected errors, hashes, metadata, and redaction behavior.
