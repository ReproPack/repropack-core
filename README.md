# ReproPack Core

ReproPack is a language-neutral, inspectable evidence bundle for software failures and development problems. This repository owns the canonical specification, shared conformance material, and the Rust reference implementation planned for later phases.

## Current status

Phases 0–14 are complete as a local release-candidate foundation. The v0.1 contract, three native implementations, interoperability, security hardening, stable CLI, CI/package validation, contributor readiness, and fresh-user documentation are complete. The corrected v0.1.1 candidate is in local release preparation; publication, tags, and registry decisions remain separate.

## Repository map

- `docs/SPECIFICATION.md` — canonical language-neutral format proposal.
- `docs/ARCHITECTURE.md` — boundaries between the three independent implementations.
- `docs/CONFORMANCE.md` — shared semantic test model.
- `ROADMAP.md` — executable phase plan.
- `PROJECT_STATE.md` — current verified state.

The TypeScript and Python implementations do not depend on Rust at runtime.

Start with the [tutorial](docs/TUTORIAL.md). See [format examples](docs/FORMAT_EXAMPLES.md), [troubleshooting](docs/TROUBLESHOOTING.md), and [migration guidance](docs/MIGRATION.md) for common workflows.

Native implementations: [TypeScript](../repropack-typescript/README.md) and [Python](../repropack-python/README.md).

## Safety boundary

ReproPack v0.1 is an evidence container. Tools must inspect, validate, verify, and extract data without executing bundle contents. It is not a replay engine, sandbox, hosted service, or guarantee that a bundle contains no secrets.

## License

MIT. See [LICENSE](LICENSE).
