# Release checklist

The v0.1 release candidate is supported by [FINAL_AUDIT.md](FINAL_AUDIT.md). Before external publication, a maintainer must re-run or inspect:

- [x] Frozen specification and schema.
- [x] Native Rust, TypeScript, and Python implementations.
- [x] Shared conformance and required interoperability paths.
- [x] Malformed archive, traversal, limit, integrity, and redaction tests.
- [x] Independent CI workflows and package/build validation.
- [x] Documentation examples and clean-environment workflow.
- [x] Version consistency at `0.1.0`.
- [x] Completed final audit and release notes.
- [x] Local Windows validator and interoperability regression checks pass.
- [ ] Linux validator and interoperability checks pass on the corrected commit in hosted CI.
- [ ] Windows fixture bytes remain unchanged and Windows validator/interoperability checks pass on the corrected commit in hosted CI.
- [ ] Hosted CI passes for the corrected release snapshot, including Rust formatting, lint, tests, metadata, and package validation.
- [ ] Explicit publication approval, tags, package upload, and hosted release.
