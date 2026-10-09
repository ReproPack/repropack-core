# ReproPack 0.1 tutorial

This tutorial creates, inspects, verifies, and extracts a small evidence bundle from a clean checkout. ReproPack stores explicitly selected bytes; it does not run, replay, or reconstruct them.

## Prerequisites

Install Rust and clone this repository. The Rust reference CLI is the quickest first workflow. The native TypeScript and Python SDKs are independent implementations; see their repository links in the main README.

## Create, inspect, verify, and extract

From `repropack-core`:

```text
cargo run -- capture examples/minimal/manifest.json target/minimal.rpk evidence/message.txt=examples/minimal/evidence/message.txt
cargo run -- inspect target/minimal.rpk
cargo run -- validate target/minimal.rpk
cargo run -- verify target/minimal.rpk
cargo run -- extract target/minimal.rpk target/minimal-extracted
```

The mapping is explicit: the left side is the archive path and the right side is the source file. Extraction stays below the destination; bundle contents are never executed.

For automation, use `cargo run -- --json verify target/minimal.rpk` and check the process exit code. Exit code 0 means success, 2 means usage error, 3 means input/validation/integrity error, and 4 means an unexpected internal error. See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for failures.
