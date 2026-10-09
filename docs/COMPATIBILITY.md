# Compatibility policy

ReproPack v0.1 readers accept exactly `spec_version` `0.1`. Unknown versions are rejected before evidence use. Unknown ordinary fields are rejected; URI-keyed data under `extensions` may be preserved or ignored without changing core semantics.

Changes to required fields, path rules, hash meaning, redaction semantics, archive safety, or stable error categories are format changes. They require a decision-log entry, updated fixtures, all implementation updates, and explicit compatibility review.

Bug fixes may tighten rejection of malformed or unsafe input when valid v0.1 bundles remain valid. Public SDK and CLI changes should preserve names and error codes or document migration.

Compatibility means equivalent manifest meaning, evidence bytes, sizes, hashes, redaction state, and expected error categories; it does not promise byte-identical ZIP archives.
