# Security model

Threats include malicious archives, path traversal, duplicate names, decompression bombs, oversized metadata, symlinks, malformed Unicode, hash substitution, secret leakage, and misleading provenance. Mitigations are canonical path validation, bounded reads, duplicate rejection, digest verification, explicit selection, redaction markers, safe temporary directories, and no execution.

Users remain responsible for reviewing contents before sharing. Redaction detection is heuristic and incomplete. Future replay is a separate product surface and cannot reuse v0.1 trust assumptions.
