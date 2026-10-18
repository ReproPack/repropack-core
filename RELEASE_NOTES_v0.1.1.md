# ReproPack v0.1.1 — Release Notes

Status: released on GitHub; registry publication is separate and not complete.

ReproPack v0.1.1 is the coordinated corrective release candidate for the language-neutral evidence-bundle format and its native Rust, TypeScript, and Python implementations.

## Changes

- Corrected canonical fixture path handling across host operating systems.
- Preserved canonical fixture evidence bytes across Windows checkouts.
- Corrected portable Python runtime discovery and Phase 8 interoperability path handling.
- Corrected TypeScript CI fixture-root configuration.
- Corrected Python CI installation for the src-layout package.
- Added encrypted-entry, compression-bomb-sized-entry, and deterministic malformed-input regression coverage.
- Added bounded chunked ZIP reads in Rust and Python and a CI failure-evidence privacy/retention contract.
- Updated coordinated release documentation and validation evidence.

## Validation

The following hosted runs validate the corrected base commits and the final merged v0.1.1 release snapshot:

- TypeScript hosted CI for the synchronized pre-audit candidate: [run 37925435775](https://github.com/ReproPack/repropack-typescript/actions/runs/37925435775)
- Python hosted CI for the synchronized pre-audit candidate: [run 37925450600](https://github.com/ReproPack/repropack-python/actions/runs/37925450600)
- Core hosted CI, including Ubuntu/Windows validation and 12 interoperability paths, for the synchronized pre-audit candidate: [run 37925467982](https://github.com/ReproPack/repropack-core/actions/runs/37925467982)

The hardened audit PRs were merged into `main`. Final merged-main validation passed in:

- TypeScript [run 39157586079](https://github.com/ReproPack/repropack-typescript/actions/runs/39157586079)
- Python [run 39157599596](https://github.com/ReproPack/repropack-python/actions/runs/39157599596)
- Core [run 39157616081](https://github.com/ReproPack/repropack-core/actions/runs/39157616081)

The Core run includes Ubuntu/Windows documentation and Rust checks plus all 12 directed interoperability paths.

## Compatibility and safety

The ReproPack specification remains version 0.1. Readers reject unsupported versions and unsafe paths. ReproPack does not execute evidence, reconstruct environments, or guarantee secret removal. Redaction detection is advisory and incomplete; review bundles before sharing.

## Publication status

This coordinated release is tagged `v0.1.1` in all three repositories and published as GitHub Releases with checksum-verified assets. crates.io and PyPI publication remain pending publisher credentials and owner approval. npm publication remains blocked while the package is private and its public entry-point configuration is undecided.
