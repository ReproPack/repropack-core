# Maintainer guide

Maintainers keep the specification, fixtures, implementations, and roadmap synchronized. Every merge should identify its phase and acceptance criterion, preserve the no-execution boundary, and leave the repository reproducible.

Before merging, check that the change has scoped review and tests, format changes have a decision-log entry and implementation impact notes, security changes have threat coverage, CI is appropriate, and roadmap status is truthful.

Core owns the normative specification, schema, canonical fixtures, Rust reference behavior, and cross-language coordination. TypeScript and Python maintainers own native behavior and must not invoke Rust. Any maintainer may stop a release for security, compatibility, privacy, or reproducibility concerns.

Do not create a v0.1 release from a phase summary alone. Use the release checklist and final audit after contributor readiness.
