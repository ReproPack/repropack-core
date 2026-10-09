# Release process

Releases follow the checklist, not calendar pressure. Freeze the specification, run all independent CI, build packages, execute the conformance matrix, perform the security audit, review examples from clean environments, update changelogs, and create `FINAL_AUDIT.md` with command evidence. Phase 0 commits and research are not a v0.1 release.

## Cross-platform release gate

Canonical manifest evidence paths use `/` regardless of the host operating system. Validation and interoperability tooling must pass those paths to platform path APIs without hard-coding `\` separators. The core CI release gate runs the Phase 1 validator and Phase 8 interoperability matrix on both Ubuntu and Windows. The interoperability script accepts `REPROPACK_PYTHON` for an explicit Python executable, otherwise it discovers a suitable platform runtime; `REPROPACK_PYTHON_PROJECT` and `REPROPACK_TYPESCRIPT` may override sibling repository locations.

The existing local `v0.1.0` tag is not moved when release-readiness fixes create later commits. If the corrected snapshot is approved for release, the maintainer must explicitly choose between creating a new release commit/tag or treating the existing tag as a superseded local candidate. No tag decision is implicit.
