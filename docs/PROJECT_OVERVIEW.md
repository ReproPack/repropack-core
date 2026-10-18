# ReproPack project overview

ReproPack is a language-neutral format and toolchain for packaging selected software-failure evidence into portable, inspectable bundles. It records a manifest, evidence bytes, provenance, limits, redaction metadata, and SHA-256 digests without executing the captured material or reconstructing an environment.

## Start here

| Goal | Document |
| --- | --- |
| Understand the problem and boundaries | [Project charter](PROJECT_CHARTER.md) |
| Implement the format | [Specification](SPECIFICATION.md) and [schema](../schema/manifest.schema.json) |
| Use the Rust CLI | [Tutorial](TUTORIAL.md) and [demo](DEMO.md) |
| Use another language | [TypeScript repository](https://github.com/ReproPack/repropack-typescript) or [Python repository](https://github.com/ReproPack/repropack-python) |
| Check compatibility | [Conformance guide](CONFORMANCE.md) |
| Review security limits | [Security model](SECURITY.md) |
| Contribute without private context | [Contributor guide](CONTRIBUTOR_GUIDE.md) and [work packages](CONTRIBUTOR_WORK_PACKAGES.md) |
| Verify releases | [Release process](RELEASE_PROCESS.md) and [GitHub Releases](https://github.com/ReproPack/repropack-core/releases) |

## Repository roles

- **Core** owns the specification, schema, canonical fixtures, Rust reference implementation, security model, and release coordination.
- **TypeScript** is an independent native Node.js implementation.
- **Python** is an independent native standard-library implementation.

The implementations do not invoke Rust. Shared fixtures and the interoperability workflow are the evidence that their behavior agrees with the format contract.

## Evidence boundaries

Verified evidence includes the v0.1.1 GitHub Releases, hosted Linux/Windows validation, canonical fixtures, cross-language interoperability, package builds, and documented security tests. It does not establish external adoption, production deployment, perfect secret detection, a sandbox, environment reconstruction, or registry publication.
