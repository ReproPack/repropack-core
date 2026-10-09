# ReproPack 0.1 format specification

**Status:** Phase 1 draft frozen for implementation review. The semantic contract below is the proposed v0.1 specification revision `0.1`; implementation conformance is not yet claimed. Changes after this point require a decision-log entry and fixture updates.

## 1. Normative language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** are to be interpreted as requirements. MUST describes conformance; SHOULD describes a recommendation that may be waived only with a documented reason; MAY describes an option.

## 2. Bundle identity and container

A ReproPack bundle is a ZIP archive with these rules:

1. The archive MUST contain exactly one root entry named `manifest.json` and zero or more entries below `evidence/`.
2. `manifest.json` MUST be UTF-8 JSON without a byte-order mark. It MUST validate against [`schema/manifest.schema.json`](../schema/manifest.schema.json).
3. Archive entry names MUST use `/`, MUST be relative, and MUST match the manifest path rules. Names containing `..`, empty components, NUL, a drive prefix, or a leading `/` MUST be rejected.
4. Archive entries MUST NOT be encrypted, comments are ignored, and duplicate names after UTF-8/path normalization MUST be rejected.
5. Symlink, hard-link, device, FIFO, and other special-file entries MUST be rejected. A regular file is the only supported entry type.
6. Readers MUST enforce caller-configurable maximum manifest bytes, evidence-entry count, per-entry uncompressed bytes, total uncompressed bytes, and path bytes before extraction. Suggested defaults are 1 MiB, 10,000 entries, 256 MiB per entry, 1 GiB total, and 4 KiB paths.
7. ZIP compression MAY be STORE or DEFLATE. Producers SHOULD use stable entry order and stable compression settings, but v0.1 does not promise byte-identical archives.

ZIP was chosen over tar, directory-only transport, and CBOR/protobuf after considering inspection tools, cross-platform library availability, compression, streaming, determinism, and implementation complexity. ZIP provides a single portable artifact and mature libraries in the three target ecosystems. Tar remains a possible future transport; a directory is a staging representation, not the v0.1 interchange artifact. JSON is used for the manifest because it is inspectable and supported natively across target languages. The schema dialect is JSON Schema Draft 2020-12.

## 3. Manifest

The root object MUST contain `format`, `spec_version`, `bundle_id`, `created_at`, `capture`, and `evidence`. `format` MUST be `repropack`; `spec_version` MUST be `0.1` for this revision. `bundle_id` MUST be a lowercase RFC 4122 UUID string. `created_at` MUST be an RFC 3339 timestamp in UTC with a `Z` suffix.

`capture` records provenance. `mode` MUST be `explicit` for user-selected evidence or `generated` for tool-produced evidence. `tool.name` and `tool.version` MUST identify the producer. Producers MAY include a human-readable `actor` and `source` but MUST avoid secrets and uncontrolled absolute paths.

`incident` is optional and may contain a short `title`, `summary`, `category`, and `reported_at`. It is descriptive, not executable. `extensions` is optional and maps absolute URI namespace keys to JSON values. Unknown ordinary members MUST be rejected in v0.1 to catch misspellings; extension data MUST be placed under `extensions` and MUST NOT override required semantics.

## 4. Evidence index

`evidence` MUST contain at least one entry. Entries MUST be ordered lexicographically by `path` in canonical manifests. Each entry requires:

- `path`: archive path beginning `evidence/`; allowed components are ASCII letters, digits, `.`, `_`, `-`, and `~`; `.` and `..` are forbidden.
- `kind`: one of `log`, `text`, `structured`, `source`, `environment`, `test-output`, or `file`.
- `media_type`: lowercase ASCII media type string.
- `size`: non-negative uncompressed byte count.
- `sha256`: lowercase hexadecimal SHA-256 of the exact archive bytes for the entry after any redaction replacement.
- `selection`: `explicit`, `generated`, or `derived`.
- `redaction`: an object with `status` `none` or `redacted`. A redacted entry MUST include `reason` from `secret`, `personal-data`, `user-requested`, or `policy`, and MUST use replacement bytes such as `[REDACTED]` rather than retain the removed value.

The digest and size always describe the bytes present in the archive, never the removed original. The format does not contain a reversible redaction mechanism.

## 5. Serialization and determinism

JSON object member order is semantically irrelevant. Canonical producers MUST emit UTF-8, no BOM, no insignificant whitespace, lexicographically sorted object keys, and the evidence array sorted by `path`. Numbers MUST be integers where the schema requires integers; implementations MUST NOT serialize NaN or Infinity. Timestamps, UUIDs, host paths, and tool versions are intentionally informative and can prevent byte-for-byte determinism. Equivalent normalized evidence SHOULD produce equal content digests and equivalent manifest meaning.

## 6. Validation, verification, and extraction

Validation MUST check JSON syntax, schema, supported `spec_version`, paths, duplicates, archive entry types, limits, required manifest-to-archive correspondence, and absence of unindexed evidence. Verification MUST recompute each entry's size and SHA-256. Extraction MUST create files only beneath a caller-selected destination after validation and verification; it MUST not follow or create links and MUST NOT execute any file, hook, handler, or command.

Errors MUST expose a stable category such as `malformed-archive`, `invalid-manifest`, `unsupported-version`, `unsafe-path`, `duplicate-entry`, `limit-exceeded`, `missing-entry`, `unexpected-entry`, `hash-mismatch`, or `extraction-failed`. Error details MUST be safe to log and MUST NOT include secret values.

## 7. Compatibility and unknown versions

An implementation MUST accept exactly `0.1` unless it explicitly supports another version. A future major or unknown version MUST produce `unsupported-version` before using evidence. Unknown ordinary fields are invalid in v0.1. Unknown extension namespaces under `extensions` MUST be preserved or ignored without changing core semantics. New optional fields in a future compatible revision MUST NOT be required by older readers.

## 8. Security and capture boundary

Capture is explicit or tool-generated evidence selection, not environment reconstruction. Producers MUST exclude credentials, private keys, access tokens, secret environment variables, credential files, and unrelated private source by default. Secret detection is advisory and incomplete; no producer or consumer may promise that a bundle contains no secrets. Users MUST review a bundle before sharing it.

No v0.1 operation executes bundle-contained code. Replay, sandboxing, network access, privilege changes, and environment reconstruction require a separate future specification and security review.

## 9. Conformance

The canonical fixtures in `conformance/` define semantic expectations. A conforming implementation must agree on manifest meaning, evidence bytes, sizes, hashes, redaction state, supported versions, and expected error categories. Opening a ZIP successfully is not conformance.
