# Release checklist

The v0.1.1 local release candidate is supported by [FINAL_AUDIT.md](FINAL_AUDIT.md). Before external publication, a maintainer must re-run or inspect:

- [x] Frozen specification and schema.
- [x] Native Rust, TypeScript, and Python implementations.
- [x] Shared conformance and required interoperability paths.
- [x] Malformed archive, traversal, limit, integrity, and redaction tests.
- [x] Independent CI workflows and package/build validation.
- [x] Documentation examples and clean-environment workflow.
- [x] Version consistency at `0.1.1` in authoritative declarations, lock metadata, and rebuilt local artifacts.
- [ ] Draft coordinated `v0.1.1` release notes reviewed.
- [x] Local Windows validator and interoperability regression checks pass.
- [x] Linux validator and interoperability checks pass on the corrected commit in hosted CI: [run 37919776037](https://github.com/ReproPack/repropack-core/actions/runs/37919776037).
- [x] Windows fixture bytes remain unchanged and Windows validator/interoperability checks pass on the corrected commit in hosted CI: [run 37919776037](https://github.com/ReproPack/repropack-core/actions/runs/37919776037).
- [x] Hosted CI passes for the corrected release snapshot, including Rust formatting, lint, tests, metadata, and package validation.
- [ ] Owner chooses whether to preserve the local pre-correction v0.1.0 tags and release 0.1.1, or explicitly approve recreating v0.1.0 after confirming no external consumption.
- [ ] Approved tags are created on the exact release commits; no current remote tags exist.
- [ ] Release notes, artifacts, SHA-256 checksums, and GitHub Release contents are reviewed and approved.
- [ ] crates.io and PyPI publication are separately approved; npm remains blocked by its private: true setting and entry-point decision.
