# Security policy

ReproPack bundles may contain source, logs, paths, environment details, and other sensitive evidence. Do not assume a bundle is safe to share merely because a tool created it. Report suspected vulnerabilities privately through the repository's GitHub security channel when available; otherwise open a minimal issue without secrets and identify that a private disclosure is needed.

The v0.1 security boundary is documented in `docs/SECURITY.md`. In particular, consumers MUST treat metadata and evidence as untrusted, must not execute bundle content, and must enforce path and resource limits during archive handling.
