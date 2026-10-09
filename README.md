# ReproPack Core

ReproPack is a language-neutral, inspectable evidence bundle for software failures and development problems. This repository owns the canonical specification, shared conformance material, and the Rust reference implementation planned for later phases.

## Current status

Phase 4 is next. The v0.1 manifest contract, schema, semantic fixtures, Rust typed model, ZIP operations, safe extraction, and initial CLI are complete; redaction hardening, full shared conformance, and release readiness are not yet implemented.

## Repository map

- `docs/SPECIFICATION.md` — canonical language-neutral format proposal.
- `docs/ARCHITECTURE.md` — boundaries between the three independent implementations.
- `docs/CONFORMANCE.md` — shared semantic test model.
- `ROADMAP.md` — executable phase plan.
- `PROJECT_STATE.md` — current verified state.

The TypeScript and Python implementations do not depend on Rust at runtime.

## Safety boundary

ReproPack v0.1 is an evidence container. Tools must inspect, validate, verify, and extract data without executing bundle contents. It is not a replay engine, sandbox, hosted service, or guarantee that a bundle contains no secrets.

## License

MIT. See [LICENSE](LICENSE).
