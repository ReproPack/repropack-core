# Competitive landscape

Research accessed 2026-10-09.

| Source | Solves | Relevant gap / overlap |
|---|---|---|
| [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) | Standardizes static-analysis results and locations | Useful structured diagnostics, but not a general selected-evidence bundle or safe archive |
| [Sentry event/envelope concepts](https://develop.sentry.dev/sdk/envelopes/) | Transports telemetry and diagnostic event items | Operational event ingestion, not a portable offline evidence package with extraction semantics |
| [CycloneDX](https://cyclonedx.org/specification/overview/) | Software/supply-chain inventory with schemas and extensions | Valuable provenance vocabulary, but broader inventory and not failure-evidence packaging |
| [Reproducible Builds definition](https://reproducible-builds.org/docs/definition/) | Defines bit-for-bit artifact reproducibility | Provides determinism vocabulary; ReproPack records evidence and context rather than claiming a reproducible build |

The gap is meaningful only if ReproPack stays narrow: an inspectable, hash-verifiable, cross-language evidence container with conservative capture and safe extraction. It must reference or embed compatible standards rather than recreate their domain semantics.

## Format evaluation

ZIP offers broad tooling and inspection; tar is simpler but less uniformly native on all target platforms; a directory is easiest to inspect but is not a portable single artifact; CBOR/protobuf improve typed encoding but reduce casual inspection. Phase 1 must measure library behavior and finalize the candidate. The current ZIP/JSON proposal is therefore explicitly provisional.
