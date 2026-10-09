# Demonstration

The CLI can create and inspect a bundle from explicitly selected evidence. From the core repository:

```text
cargo run -- capture conformance/fixtures/minimal-valid/manifest.json target/example.rpk evidence/message.txt=conformance/fixtures/minimal-valid/evidence/message.txt
cargo run -- inspect target/example.rpk
cargo run -- validate target/example.rpk
cargo run -- verify target/example.rpk
cargo run -- extract target/example.rpk target/extracted
```

For automation, put `--json` before the command:

```text
cargo run -- --json inspect target/example.rpk
cargo run -- --json verify target/example.rpk
```

Exit code 0 means success, 2 means usage error, 3 means input/validation/integrity error, and 4 means an unexpected internal error. The commands validate the archive without executing its contents. Error JSON never includes evidence bytes.
