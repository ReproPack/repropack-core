# ReproPack roadmap

Every phase has an exit gate; code existing is not evidence of completion.

| Phase | Purpose and dependency | Deliverables and tests | Exit criteria |
|---|---|---|---|
| 0 | Establish independent repositories and truthful foundation | Docs, policies, CI scaffolds, audit, local commits | All required docs agree; no misleading capability claims |
| 1 | Freeze language-neutral semantics | Normative spec, schema, format comparison, canonical fixtures | Two independent reviewers can implement manifest rules from docs |
| 2 | Build Rust model | Typed model, JSON serialization, validation tests | Required/optional fields and errors tested |
| 3 | Build Rust bundle operations | Create, inspect, validate, safe extract | Minimal and complete bundles round-trip; traversal rejected |
| 4 | Integrity and redaction | SHA-256 verification, explicit redaction markers, warnings | Altered content and redaction cases fail/pass as specified |
| 5 | Shared conformance | fixtures/, expected/, invalid/, manifest metadata | Rust passes every fixture with recorded semantics |
| 6 | Native TypeScript | SDK, reader/writer, validation, tests | TypeScript passes shared semantics without Rust |
| 7 | Native Python | SDK, reader/writer, validation, tests | Python passes shared semantics without Rust |
| 8 | Interoperability | six implementation pair paths, semantic matrix | Metadata, contents, hashes, redactions, and failures agree |
| 9 | Security hardening | malformed archive, symlink, bomb, limits, secret-risk tests | Security checklist has evidence and no unbounded handling |
| 10 | Usability | CLI/SDK examples, stable errors, inspect output | README commands work against released code |
| 11 | CI and packaging | independent workflows, build/package checks | CI runs real checks on supported toolchains |
| 12 | Documentation and examples | demo bundles, troubleshooting, migration notes | Fresh user can inspect and verify a bundle |
| 13 | Contributor readiness | issue templates, maintainer process, review guidance | External contributor path is documented and tested |
| 14 | v0.1 release | final audit, tags, packages, release notes | Checklist and all P0 evidence pass; no fabricated artifacts |
| 15 | Reproduction integrations | opt-in integrations with test/build systems | Integrations preserve capture boundary and consent |
| 16 | CI/GitHub integrations | upload/download workflows and retention guidance | Permissions and secret handling reviewed |
| 17+ | Ecosystem evolution | additional implementations and spec revisions | New phases have acceptance criteria and compatibility analysis |

## Self-directing workflow

For any phase request, inspect actual status, read the phase criteria, run relevant checks, compare each criterion with evidence, update state, and report gaps. A phase is complete only when its exit criteria are evidenced in the repository.
