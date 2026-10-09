# Security policy

ReproPack bundles may contain source, logs, paths, environment details, and other sensitive evidence. Do not assume a bundle is safe to share merely because a tool created it. Report suspected vulnerabilities privately through the repository's GitHub security channel when available; otherwise contact maintainers through a private administration channel before public disclosure. Never include vulnerable bundles, credentials, tokens, or personal data in an issue.

The v0.1 security boundary is documented in `docs/SECURITY.md`. In particular, consumers MUST treat metadata and evidence as untrusted, must not execute bundle content, and must enforce path and resource limits during archive handling.
