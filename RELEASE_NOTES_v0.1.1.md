# ReproPack v0.1.1 — Draft Release Notes

Status: local draft; not released or published.

ReproPack v0.1.1 is the coordinated corrective release candidate for the language-neutral evidence-bundle format and its native Rust, TypeScript, and Python implementations.

## Changes

- Corrected canonical fixture path handling across host operating systems.
- Preserved canonical fixture evidence bytes across Windows checkouts.
- Corrected portable Python runtime discovery and Phase 8 interoperability path handling.
- Corrected TypeScript CI fixture-root configuration.
- Corrected Python CI installation for the src-layout package.
- Updated coordinated release documentation and validation evidence.

## Validation

The following hosted runs validate the corrected base commits that preceded the local version-only v0.1.1 candidate commits:

- TypeScript hosted CI for the synchronized pre-audit candidate: [run 37925435775](https://github.com/ReproPack/repropack-typescript/actions/runs/37925435775)
- Python hosted CI for the synchronized pre-audit candidate: [run 37925450600](https://github.com/ReproPack/repropack-python/actions/runs/37925450600)
- Core hosted CI, including Ubuntu/Windows validation and 12 interoperability paths, for the synchronized pre-audit candidate: [run 37925467982](https://github.com/ReproPack/repropack-core/actions/runs/37925467982)

The audit branch `audit/full-repropack-2026-10-09` adds security and schema-validation corrections after those runs. Hosted CI for the audit branch remains pending; these links must not be read as validation of the audit fixes.

Hosted CI for the final v0.1.1 candidate commits remains pending and is required before publication.

## Compatibility and safety

The ReproPack specification remains version 0.1. Readers reject unsupported versions and unsafe paths. ReproPack does not execute evidence, reconstruct environments, or guarantee secret removal. Redaction detection is advisory and incomplete; review bundles before sharing.

## Publication status

This is a local release draft. No v0.1.1 tags, GitHub Releases, crates.io publication, PyPI publication, or npm publication have been performed. npm publication remains blocked while the package is private and its public entry-point configuration is undecided.
