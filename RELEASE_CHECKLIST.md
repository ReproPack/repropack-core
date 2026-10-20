# Release checklist

The v0.1.1 local release candidate is supported by [FINAL_AUDIT.md](FINAL_AUDIT.md). Before external publication, a maintainer must re-run or inspect:

- [x] Frozen specification and schema.
- [x] Native Rust, TypeScript, and Python implementations.
- [x] Shared conformance and required interoperability paths.
- [x] Malformed archive, traversal, limit, integrity, and redaction tests.
- [x] Encrypted-entry, compression-bomb-sized entry, and deterministic malformed-input regression tests.
- [x] CI failure-evidence privacy/retention contract validated locally; selected files remain subject to human review.
- [x] Independent CI workflows and package/build validation.
- [x] Documentation examples and clean-environment workflow.
- [x] Version consistency at `0.1.1` in authoritative declarations, lock metadata, and rebuilt local artifacts.
- [x] Coordinated `v0.1.1` release notes reviewed and used for the Core GitHub Release.
- [x] Local Windows validator and interoperability regression checks pass.
- [x] Linux validator and interoperability checks pass on the corrected commit in hosted CI: [run 37919776037](https://github.com/ReproPack/repropack-core/actions/runs/37919776037).
- [x] Windows fixture bytes remain unchanged and Windows validator/interoperability checks pass on the corrected commit in hosted CI: [run 37919776037](https://github.com/ReproPack/repropack-core/actions/runs/37919776037).
- [x] Hosted CI passes for the previously synchronized corrected snapshot, including Rust formatting, lint, tests, metadata, package validation, and interoperability.
- [x] Hosted CI reran the new security-hardening changes on Ubuntu and Windows: Core run [39157941057](https://github.com/ReproPack/repropack-core/actions/runs/39157941057).
- [x] Historical local v0.1.0 tags were preserved; corrected code was released as v0.1.1.
- [x] v0.1.1 tags were created on the exact release commits and pushed.
- [x] Release notes, artifacts, SHA-256 checksums, and GitHub Release contents are published and verified.
- [x] PyPI publication is complete and verified for `repropack==0.1.1`; npm publication is complete and verified for `repropack-typescript@0.1.1`.
- [ ] crates.io publication: dry-run passes, but publisher credentials and owner approval are still required.

## Funding-readiness gates

- [x] Submission dossier distinguishes verified, unpublished, planned, and unknown claims: [docs/FUNDING_READINESS.md](docs/FUNDING_READINESS.md).
- [x] Contributor backlog is scoped locally in [BACKLOG.md](BACKLOG.md); no artificial issue count is claimed.
- [ ] Maintainer identifies the exact Drips Wave or RetroPGF round and reviews the previous rejection reason.
- [ ] Owner approves whether and how `repropack-core` is claimed as the single Drips Project, including `FUNDING.json`, wallet, maintainers, and dependency splits.
- [ ] A specific GrantFox campaign and official application route are verified before any submission claim.
