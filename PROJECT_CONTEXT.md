# Project context

## Mission

ReproPack packages portable evidence needed to understand and reproduce a software failure, test failure, build failure, or development problem.

## Users and boundaries

Developers, maintainers, support engineers, and CI systems are intended users. v0.1 captures explicitly selected evidence, validates and verifies it, and extracts it safely. It does not execute evidence, reconstruct environments, replay failures, host bundles, or promise secret-free output.

## Architecture

`repropack-core` owns the language-neutral specification, canonical fixtures, project coordination, and Rust reference implementation. `repropack-typescript` and `repropack-python` are independent native implementations. The specification, not Rust source, is authoritative.

## Technology and security

Rust uses stable toolchains. TypeScript and Python use their native ecosystems and must not shell out to Rust. All implementations must treat bundles as hostile input, use bounded resource handling, reject unsafe paths, and never execute contents.

## Conformance and versioning

The shared suite tests semantic equivalence: metadata, evidence, hashes, redactions, and expected failures. Specification version, implementation version, and repository tags remain distinct. Current specification version is `0.1` in Phase 1 implementation review; no implementation is conformant yet.

## Future-agent rules

Inspect all three repositories and Git status before work. Read the relevant roadmap phase and acceptance criteria. Preserve independent histories. Do not mark work complete from compilation alone. Update `PROJECT_STATE.md`, decisions, changelogs, and conformance material when behavior changes. The next task is determined by the first incomplete phase whose dependencies and acceptance criteria can be satisfied.
