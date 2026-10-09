# Conformance fixture contribution guide

Fixtures are shared behavioral contracts, not data dumps. Keep them minimal, synthetic, and free of credentials or personal information.

## Valid fixture

1. Create `conformance/fixtures/<id>/manifest.json` and `evidence/` files.
2. Use a lowercase UUID, UTC timestamp, sorted paths, exact byte sizes, and lowercase SHA-256 digests.
3. Add the case to `conformance/catalog.json` and `conformance/expected/<id>.json` with meaningful assertions.
4. Run the Rust fixture runner and all implementation tests.

## Invalid fixture

Make one failure mode intentional, add its expected stable error category, and ensure no test reads an unsafe path or executes content to discover the failure.

Explain the protected semantic rule and why the bytes are safe to publish. Never use real logs, tokens, private keys, customer data, or generated archives when synthetic bytes suffice.
