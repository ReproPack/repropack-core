# Phase 13 review: contributor readiness

Status: complete on 2026-10-09.

## Delivered

- GitHub bug and feature issue templates.
- Pull-request template covering phase scope, compatibility, privacy, security, tests, and truthful status.
- Maintainer responsibilities and ownership boundaries.
- Design-review requirements for format, API, security, and interoperability changes.
- v0.1 compatibility policy and migration expectations.
- Private vulnerability-reporting guidance.
- Synthetic conformance-fixture contribution instructions.

## Contributor smoke check

An external contributor can begin from `CONTRIBUTING.md`, identify the phase in `ROADMAP.md`, read the linked review policies, add a fixture using `docs/FIXTURE_CONTRIBUTION.md`, and run:

```text
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked --all-targets
node scripts/validate_phase1.mjs
```

Language-specific changes use the commands in the TypeScript and Python CI workflows. No private context or generated artifact is required.

## Boundary

Release freeze, final audit, release notes, and tags remain Phase 14.
