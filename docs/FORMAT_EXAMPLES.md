# ReproPack format examples

The checked-in [minimal example](../examples/minimal/manifest.json) is a complete valid v0.1 manifest. Its evidence bytes and digest are checked by the conformance suite.

## Redacted text entry

```json
{
  "path": "evidence/config.txt",
  "kind": "text",
  "media_type": "text/plain",
  "size": 19,
  "sha256": "45ab0b25a2a6b7e4b46a4ef4e3f446a3dc596529dabeb17a96103e9e6e275428",
  "selection": "explicit",
  "redaction": { "status": "redacted", "reason": "secret" }
}
```

The digest and size describe replacement bytes, never the removed value. Redaction detection is advisory and incomplete; users must review evidence before sharing.

See [`complete-valid`](../conformance/fixtures/complete-valid/manifest.json) for incident metadata, generated capture provenance, multiple evidence kinds, and a URI-keyed extension.
