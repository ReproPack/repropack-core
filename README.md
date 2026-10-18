# ReproPack Core

<p align="center">
  <img src="docs/architecture.svg" alt="ReproPack architecture: one language-neutral format with Rust, TypeScript, and Python implementations" width="900">
</p>

<p align="center"><strong>Portable, inspectable evidence bundles for software failures and development problems.</strong></p>

<p align="center">
  <a href="https://github.com/ReproPack/repropack-core/actions/workflows/ci.yml"><img src="https://github.com/ReproPack/repropack-core/actions/workflows/ci.yml/badge.svg" alt="Core CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-2f6f9f.svg" alt="MIT license"></a>
  <a href="docs/SPECIFICATION.md"><img src="https://img.shields.io/badge/format-ReproPack%200.1-6b4fbb.svg" alt="ReproPack format 0.1"></a>
</p>

ReproPack gives developers and maintainers a small, deterministic format for handing a software failure from one tool or environment to another. It packages selected logs, test output, metadata, and files into an inspectable bundle whose paths, sizes, and SHA-256 digests can be checked without running the bundle.

It is for developers debugging CI failures, maintainers reviewing bug reports, and tool authors that need a common evidence format across Rust, Node.js, Python, and future implementations. It is not a hosted debugging service: the bundle is a portable evidence record, not an execution recipe.

## What this repository owns

This is the canonical project repository. It contains:

- the language-neutral [format specification](docs/SPECIFICATION.md) and JSON schema;
- the shared [conformance fixtures](docs/CONFORMANCE.md);
- the Rust reference model, bundle operations, and CLI;
- the security, compatibility, release, and contributor documentation.

The native implementations are independent repositories:

| Implementation | Best for | Repository |
| --- | --- | --- |
| Rust | CLI, validation, and the reference implementation | [repropack-core](https://github.com/ReproPack/repropack-core) |
| TypeScript | Node.js applications and tooling | [repropack-typescript](https://github.com/ReproPack/repropack-typescript) |
| Python | Python automation and analysis | [repropack-python](https://github.com/ReproPack/repropack-python) |

Use this Core repository when you need the specification, schema, canonical fixtures, Rust CLI, compatibility rules, or release process. Use the TypeScript or Python repository when you are embedding ReproPack in that language. Each implementation reads the shared contract and fixtures; neither implementation imports or invokes Rust.

## Why ReproPack?

Reproduction artifacts are often scattered across terminals, CI logs, temporary directories, and issue comments. ReproPack makes the handoff explicit and auditable:

1. select the evidence intentionally;
2. record canonical metadata and SHA-256 digests;
3. redact before hashing when redaction is required;
4. validate, verify, and extract safely in another environment.

The format is transport, not execution. ReproPack does not replay commands, reconstruct environments, guarantee that evidence contains no secrets, or create a sandbox.

Unlike ordinary logs or CI artifacts, a bundle carries an explicit manifest and post-redaction integrity metadata. Unlike a bug report, it is machine-readable and can be consumed by independent implementations. Unlike a crash dump or replay system, it does not capture a complete process state and cannot reproduce a program by itself.

## Quick start

Install Rust, then run the real minimal workflow from this repository:

```bash
cargo run -- capture examples/minimal/manifest.json target/minimal.rpk \
  evidence/message.txt=examples/minimal/evidence/message.txt
cargo run -- inspect target/minimal.rpk
cargo run -- validate target/minimal.rpk
cargo run -- verify target/minimal.rpk
cargo run -- extract target/minimal.rpk target/minimal-extracted
```

The full walkthrough is in the [tutorial](docs/TUTORIAL.md). For automation, use `--json` and check the documented exit codes in [troubleshooting](docs/TROUBLESHOOTING.md).

## Navigate the project

- [Specification](docs/SPECIFICATION.md) — normative format and semantics.
- [Architecture](docs/ARCHITECTURE.md) — repository boundaries and implementation roles.
- [Project overview](docs/PROJECT_OVERVIEW.md) — one-page map of the project and its evidence boundaries.
- [Impact evidence](docs/IMPACT_EVIDENCE.md) — verified adoption evidence and clearly marked gaps.
- [Format examples](docs/FORMAT_EXAMPLES.md) — valid manifest and bundle shapes.
- [Conformance](docs/CONFORMANCE.md) — shared fixtures and expected behavior.
- [Security](SECURITY.md) — threat model, limits, and reporting guidance.
- [Development and testing](docs/DEVELOPMENT_MODEL.md) — contributor workflow.
- [Contributor work packages](docs/CONTRIBUTOR_WORK_PACKAGES.md) — self-contained future work without private context.
- [Roadmap](ROADMAP.md) — evidence-based project phases.
- [Release checklist](RELEASE_CHECKLIST.md) — preparation and publication gates.
- [CI integration example](docs/CI_INTEGRATION.md) — an optional, safe failure-evidence workflow.
- [Funding and contributor dossier](docs/FUNDING_READINESS.md) — verified evidence and submission boundaries.

## Current status

Phases 0–14 are complete. ReproPack v0.1.1 is released on GitHub for [Core](https://github.com/ReproPack/repropack-core/releases/tag/v0.1.1), [TypeScript](https://github.com/ReproPack/repropack-typescript/releases/tag/v0.1.1), and [Python](https://github.com/ReproPack/repropack-python/releases/tag/v0.1.1), with checksum-verified release assets. crates.io and PyPI publication remain owner-controlled, and the TypeScript package remains intentionally private for npm.

## License

MIT. See [LICENSE](LICENSE).
