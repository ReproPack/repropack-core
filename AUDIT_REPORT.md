# ReproPack full organization audit

**Audit branch:** `audit/full-repropack-2026-10-09`  
**Date:** 2026-10-09  
**Scope:** Core/specification/Rust, TypeScript, Python, GitHub organization, security leads, conformance, packaging, release readiness, and funding readiness.

## Executive result

The audit found and corrected confirmed P0/P1 implementation defects on dedicated audit branches. Local tests and the complete hosted audit-branch CI now pass. Review remains open in three PRs; no PR has been merged.

The specification is genuinely language-neutral at the contract level, and the three current implementations agree on the supported semantic fixtures and the recorded six-path interoperability matrix. That evidence does not certify every malformed ZIP input, every future implementation, or deferred integrations.

## Verified repository and organization state

GitHub identity was verified with `gh auth status` and `gh api user`: authenticated account `Marvelg256`. The organization currently contains exactly these public repositories, all defaulting to `main`:

| Repository | Base before audit | Audit branch | Working tree at audit start |
|---|---|---|---|
| `ReproPack/repropack-core` | `b3e5a27418d33009467ca601a5c3c38b70ed14da` | `feb941121e007411307cce7b37ceb49d144fa822` | `audit/full-repropack-2026-10-09` |
| `ReproPack/repropack-typescript` | `9b146ecf3c07b0b9266bc5040d755953369ac413` | `95e61fef87fdd913c3b8e5f77784a90c6b0c0905` | `audit/full-repropack-2026-10-09` |
| `ReproPack/repropack-python` | `26acc6d3d025e68f3545f9c51d1ed9420ba37752` | `ab70d2b2ca11373f49148a223759990a04c53cdb` | `audit/full-repropack-2026-10-09` |

The existing local `v0.1.0` tags were inspected and not moved, recreated, or pushed. No GitHub Releases were listed. All three repositories have empty issue and pull-request lists at audit time. The repositories are pinned, but the organization has no description, website, profile README, or repository topics/homepages. Branch-protection API checks returned 404 and are therefore unverified; no settings were changed.

## Confirmed fixes

### TypeScript

- `src/bundle.ts`: ancestor validation now uses the platform separator, validates/creates each parent component before writing, rejects symlink/special-file ancestors, checks declared total limits before expansion, bounds DEFLATE output with `maxOutputLength`, and decodes manifest UTF-8 with fatal errors.
- `src/model.ts`: path validation rejects `.` and `..` components without rejecting valid names such as `foo..txt`; strict calendar timestamps, absolute URI extension names, nested `incident` validation, capture metadata lengths, media-type lengths, and path-byte limits now match the schema.
- `src/redaction.ts`: CRLF private-key delimiters are recognized and newline style is preserved.
- `test/repropack.test.ts`: added CRLF, symlink ancestor, DEFLATE output bound, consecutive-dot filename, and nested-schema/timestamp/URI regressions.
- `.github/workflows/ci.yml`: ordinary pushes and pull requests use the canonical `main` revisions of sibling repositories; deliberate coordinated interoperability runs use explicit `workflow_dispatch` revision inputs and print the resolved SHAs.

### Python

- `src/repropack/model.py`: added nested `incident`, capture metadata lengths, media-type length, absolute URI extension, and strict reported-timestamp validation.
- `src/repropack/redaction.py`: CRLF private-key delimiters are recognized and newline style is preserved.
- `tests/test_repropack.py`: added CRLF and nested-schema/timestamp/URI regressions.
- `.github/workflows/ci.yml`: ordinary pushes and pull requests use Core `main`; deliberate coordinated runs can provide an explicit Core branch, tag, or SHA.

### Core/Rust

- `src/redaction.rs`: CRLF private-key delimiters are recognized and newline style is preserved.
- `src/lib.rs`: corrected the stale Phase-2-only crate documentation and aligned model-level metadata/path/URI length checks with the schema.
- `.github/workflows/ci.yml`: ordinary interoperability runs use TypeScript/Python `main`; deliberate coordinated runs can provide explicit revisions for both implementations.
- `docs/SPECIFICATION.md`, `docs/CONFORMANCE.md`, and implementation mapping documents now describe the current evidence without claiming universal conformance.

## Validation evidence

All commands below ran locally on Windows 11-style PowerShell paths on the audit branches:

- TypeScript: `npm.cmd test` — 12 passed; includes build/type compilation.
- Python: `C:\Users\user\Projects\ReproPack\tools\python312\python.exe -m unittest discover -s tests -v` — 8 passed.
- Core: `cargo fmt --all`, `cargo test --offline` — 22 Rust unit tests, 1 conformance test, and doctests passed.
- Core: `cargo clippy --offline --all-targets -- -D warnings` — passed.
- Core: `cargo package --offline --locked --allow-dirty --no-verify` — passed.
- Core Phase 8: `node scripts/phase8_interop.mjs` with explicit local TypeScript, Python project, and Python-runtime overrides — all 12 directed paths passed for `minimal-valid` and `redacted-valid`.

The audit-branch hosted evidence is verified: [TypeScript run 37932693014](https://github.com/ReproPack/repropack-typescript/actions/runs/37932693014), [Python run 37932699454](https://github.com/ReproPack/repropack-python/actions/runs/37932699454), and [Core run 37932685340](https://github.com/ReproPack/repropack-core/actions/runs/37932685340) all passed. Core’s run passed Ubuntu and Windows documentation/Rust jobs plus Ubuntu and Windows interoperability. The workflows have since been corrected so ordinary PRs do not require same-named sibling branches; a new coordinated dispatch is required to revalidate all three audit branch SHAs together.

## Remaining security and conformance scope

The following were not fully covered by the current canonical matrix and remain release gates or residual risks:

- central/local ZIP header disagreement, overlapping regions, invalid UTF-8 filenames, ZIP64, trailing data, comments, and unsupported flags need implementation-specific regression cases;
- streaming/memory-bounded handling of very large compressed input remains a residual concern even with output limits;
- extraction has an honest residual time-of-check/time-of-use race between link checks and writes on ordinary filesystems;
- redaction is heuristic and incomplete; users must review evidence before sharing;
- the matrix covers two valid fixtures and expected semantic fixture errors, not all possible malformed archives.

No implementation executes bundle content. The current six directed paths agree on manifest meaning, evidence bytes, sizes, hashes, redaction state, and supported version behavior for the tested fixtures.

## Packaging and release blockers

- Core package metadata and offline package validation pass; crates.io name/owner availability and publication require a separate registry check and owner approval.
- Python wheel/sdist and installed-package checks must be rerun on the audit branch; PyPI name ownership/availability is not inferred from a 404 and publication requires owner approval.
- TypeScript remains `private: true`; npm publication and entry-point/export decisions remain blocked on explicit owner approval.
- Existing local `v0.1.0` tags remain historical pre-correction tags. Do not move them without explicit confirmation that they were never externally consumed. A corrected coordinated release should use an owner-approved new version/tag strategy.
- Audit branches have passed hosted CI and require independent maintainer review before merging. No direct `main` push, tag, GitHub Release, or package publication is authorized by this report.

## Funding readiness

### Drips Wave / RetroPGF

Official Drips documentation says maintainers apply repositories to a Wave, selected repositories carry structured issues, and contributors work through the Wave flow. RetroPGF applications require a claimed Drips Project; claiming requires a repository/default-branch `FUNDING.json` and a connected wallet. ReproPack currently has zero open issues and zero open PRs across the three repositories, so it does not yet present a credible active contributor backlog for a Wave application. Onboarding, app installation, repository claim state, and any specific round/deadline were not verified without entering the Drips app or connecting a wallet.

### Stellar Community Fund

The official Community Fund awards page currently lists SCF #46 as accepting interest submissions, with a November 8, 2026 submission deadline and tracks for Open, Integration, and RFP. Eligibility requires demonstrated Stellar/Soroban use, product-market fit, submission quality, integration planning, and a ready-to-build project. ReproPack is currently a language-neutral evidence format and has no verified Stellar/Soroban integration or adoption evidence, so fit is unproven; do not submit by analogy alone.

### GrantFox

GrantFox is verified as an open-source contribution ecosystem/platform with issues and bounties, not as a guaranteed grant program. The official site provides contributor/opportunity discovery and links to its platform; no definitive ReproPack-approved campaign or maintainer application route was verified in this audit. Treat a GrantFox campaign application as a separate, campaign-specific decision after confirming an active campaign, organizer requirements, issue acceptance, reward terms, and the official application URL.

## Required manual GitHub actions

1. Review the open [Core PR #1](https://github.com/ReproPack/repropack-core/pull/1), [TypeScript PR #1](https://github.com/ReproPack/repropack-typescript/pull/1), and [Python PR #1](https://github.com/ReproPack/repropack-python/pull/1).
2. Confirm branch protection/rulesets through the GitHub UI or organization-admin API access.
3. Add an organization description, profile README, repository topics, and homepages if desired; these are presentation decisions, not code fixes.
4. Merge only after maintainer review confirms the hosted CI evidence and, if coordinated candidate revisions are required, a manually dispatched run with all explicit revision inputs.
5. Decide the corrected release version/tag strategy and npm visibility/entry-point policy before creating releases or publishing packages.
6. Build a real contributor backlog before pursuing Drips Wave.
7. Establish verified Stellar/Soroban utility and adoption evidence before considering SCF.

## Explicit answers

1. **Is the specification language-neutral?** Yes at the normative contract level: ZIP/JSON/schema/fixture semantics are independent of Rust, and the implementations do not invoke one another at runtime.
2. **Do all three implementations satisfy the same supported contract?** For the current canonical fixtures and two valid interoperability fixtures, yes; local audit-branch tests and all 12 matrix paths pass. This is not a universal-input certification.
3. **What remains untested or potentially failing?** The malformed ZIP/parser cases listed above, a post-correction coordinated dispatch using explicit audit-branch revisions, registry ownership, Drips app state, and organization rulesets.
4. **What prevents a clean v0.1.1 release?** Audit-branch maintainer review/merge, the old-tag decision, registry approvals and package-specific gates, and the TypeScript npm-publication decision.
5. **What must be fixed manually in GitHub?** Verify or configure rulesets, optionally improve organization/repository metadata, publish audit branches/open PRs, review/merge them, then perform separately approved release/tag/registry actions.
