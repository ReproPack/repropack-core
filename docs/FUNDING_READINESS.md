# Drips Wave and GrantFox readiness dossier

This document is a preparation dossier, not an application. It must not be read as evidence that ReproPack is approved by Drips, GrantFox, or any funding program.

## Project summary

ReproPack is a portable, inspectable, language-neutral evidence-bundle format and toolchain for safely sharing structured software-failure and development evidence across languages, CI systems, and debugging workflows.

## What is verified

- Core owns the specification, JSON schema, canonical fixtures, conformance rules, Rust reference implementation, and release process.
- TypeScript and Python are independent native implementations.
- The current supported fixtures and 12 directed interoperability paths have passed local and hosted validation.
- Linux and Windows CI coverage exists for Core documentation, Rust checks, and interoperability.
- Security boundaries are documented: no implicit execution, no environment reconstruction, advisory redaction only, and bounded in-memory processing.
- No public package, GitHub release, or registry publication is currently claimed by this dossier.

## Contributor impact to fund

The strongest Wave-shaped work is concrete maintenance and integration:

1. streaming verification for large bundles;
2. archive-bomb and encrypted-archive fixtures;
3. GitHub Actions failure-evidence integration;
4. deterministic release checksum tooling;
5. Python wheel installation coverage;
6. stable TypeScript package exports;
7. conformance-fixture tooling; and
8. additional native implementations with a compatibility plan.

These items are tracked in [`BACKLOG.md`](../BACKLOG.md). They are not claims that the work is complete.

## Repository relationship

Use `repropack-core` as the lead project. The TypeScript and Python repositories should be linked as independent implementations, not submitted as duplicate projects. A contributor can work on the Core specification/fixtures or on one implementation while using the same contract.

## Readiness labels

| Item | State |
| --- | --- |
| Format, implementations, fixtures, and CI | Completed and verified within the documented scope |
| v0.1.1 release | Prepared locally; unpublished and owner-controlled |
| Drips Project claim | Not performed; requires wallet and `FUNDING.json` decision |
| Drips Wave repository approval | Unknown; requires application/appeal status check |
| RetroPGF round eligibility | Unknown until a specific open Registration round is identified |
| GrantFox campaign | Unknown; no specific ReproPack campaign is claimed |
| External adoption | Not established; do not imply users or production deployments |
| Stellar-specific integration | Planned only if a real use case is approved; not part of Core |

## Submission boundaries

Do not claim universal security, production forensic safety, package publication, external adoption, or approval. Do not submit a wallet address or public application field until the project owner reviews the exact public content and funding splits.

For current requirements, consult the [Drips Wave documentation](https://docs.drips.network/wave/), [Drips application guide](https://docs.drips.network/rpgf/apply-to-a-round/), and [repository claim guide](https://docs.drips.network/get-support/claim-your-repository/). GrantFox must be evaluated against a specific active campaign rather than treated as a generic grant program.
