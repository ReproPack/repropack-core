# Architecture

```mermaid
flowchart TD
  S[Language-neutral specification] --> C[Shared conformance suite]
  C --> R[Rust reference + CLI]
  C --> T[Native TypeScript SDK/CLI]
  C --> P[Native Python SDK/CLI]
  R --> B[Interoperable bundles]
  T --> B
  P --> B
```

The core repository coordinates semantics and fixtures. Implementations may share fixtures and documentation links, but not runtime code. The bundle boundary contains only declared metadata and evidence; no implementation may infer executable behavior from it.
