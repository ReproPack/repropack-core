# Competitive landscape

Research accessed 2026-10-09.

| Source | Solves | Relevant gap / overlap |
|---|---|---|
| [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) | Standardizes static-analysis results and locations | Useful structured diagnostics, but not a general selected-evidence bundle or safe archive |
| [Sentry event/envelope concepts](https://develop.sentry.dev/sdk/envelopes/) | Transports telemetry and diagnostic event items | Operational event ingestion, not a portable offline evidence package with extraction semantics |
| [CycloneDX](https://cyclonedx.org/specification/overview/) | Software/supply-chain inventory with schemas and extensions | Valuable provenance vocabulary, but broader inventory and not failure-evidence packaging |
| [Reproducible Builds definition](https://reproducible-builds.org/docs/definition/) | Defines bit-for-bit artifact reproducibility | Provides determinism vocabulary; ReproPack records evidence and context rather than claiming a reproducible build |

The gap is meaningful only if ReproPack stays narrow: an inspectable, hash-verifiable, cross-language evidence container with conservative capture and safe extraction. It must reference or embed compatible standards rather than recreate their domain semantics.

Phase 1 also evaluated the [JSON Schema Draft 2020-12 specification](https://json-schema.org/draft/2020-12) for manifest validation. It provides a language-neutral schema vocabulary with broad implementations; ReproPack uses it for structural validation while keeping path safety, archive limits, and digest verification as additional semantic checks. Research access date: 2026-10-09.

## Format evaluation

ZIP offers broad tooling and inspection; tar is simpler but less uniformly native on all target platforms; a directory is easiest to inspect but is not a portable single artifact; CBOR/protobuf improve typed encoding but reduce casual inspection. Phase 1 freezes ZIP plus JSON for v0.1, with ZIP entries restricted to regular files and bounded reads. Tar remains a possible future transport; the directory representation is used only for source fixtures.
