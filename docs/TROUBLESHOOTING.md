# Troubleshooting

## `invalid-manifest`

Check required fields, `spec_version` `0.1`, sorted evidence paths, byte-count sizes, and 64-character lowercase SHA-256 values. Unknown ordinary fields are rejected.

## `missing-entry` or `unexpected-entry`

Every manifest evidence path must have exactly one archive entry, and every `evidence/` archive entry must be indexed. Check capture mappings and spelling.

## `hash-mismatch` or `size-mismatch`

The bytes changed after the manifest was generated. Recreate metadata from the final replacement bytes; do not edit the digest by hand.

## `unsafe-path` or extraction failure

Use relative paths below `evidence/` with ASCII letters, digits, `.`, `_`, `-`, and `~`. Do not use `..`, absolute paths, links, or special files. Extract into a new destination when diagnosing a conflict.

## `limit-exceeded`

The archive or manifest exceeds configured reader limits. Reduce selected evidence, split the capture, or intentionally provide larger limits to a trusted caller.

## Secret concerns

ReproPack is not a guarantee of secret removal. Select evidence explicitly, review it before sharing, and treat redaction warnings as advisory. Binary input is not scanned by the conservative helpers.
