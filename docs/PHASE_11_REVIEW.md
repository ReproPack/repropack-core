# Phase 11 review: CI and packaging

Status: complete on 2026-10-09.

## Delivered

- Core CI runs documentation checks, Phase 1 fixture audit, Rust formatting, Clippy, locked tests, locked dependency metadata, and Cargo package validation.
- TypeScript CI checks out canonical fixtures, installs from `package-lock.json`, runs typecheck/build/tests, performs `npm audit`, validates the npm tarball with `npm pack --dry-run`, and checks documentation.
- Python CI checks out canonical fixtures, installs Python build tooling, runs compile and unittest checks, builds wheel/sdist artifacts, installs the wheel without dependencies, runs `pip check`, and checks documentation.
- Python packaging metadata now declares src-layout package discovery.
- Generated build/cache artifacts are ignored by the TypeScript and Python repositories.

## Local evidence

- Rust: format, Clippy, 21 tests, conformance, and `cargo package --locked --allow-dirty --no-verify` pass.
- TypeScript: typecheck, build, 7 tests, npm audit with 0 vulnerabilities, and npm package dry-run pass.
- Python: compileall and 6 unittest tests pass with Python 3.12.8; wheel/sdist packaging is exercised by CI because the isolated local runtime has no pip/build module.

## Boundary

This phase adds repeatable validation and package checks. Release versioning, final audit, contributor process, and release artifacts remain later phases.
