# Demonstration

The Phase 3 CLI can create and inspect a bundle from explicitly selected evidence. From the core repository:

```text
cargo run -- capture conformance/fixtures/minimal-valid/manifest.json target/example.rpk evidence/message.txt=conformance/fixtures/minimal-valid/evidence/message.txt
cargo run -- inspect target/example.rpk
cargo run -- validate target/example.rpk
cargo run -- verify target/example.rpk
cargo run -- extract target/example.rpk target/extracted
```

The commands validate the archive without executing its contents. Traversal, directory, and oversized-entry cases are rejected by the reader tests. Secret detection and redaction warnings are Phase 4 work and are not claimed here.
