# ReproPack v0.1.0

ReproPack v0.1.0 is an inspectable, language-neutral evidence-bundle format and native reference implementation set for software failures and development problems.

## Included

- Frozen v0.1 manifest and ZIP format specification.
- Rust reference model, bundle operations, CLI, verification, safe extraction, and JSON errors.
- Native TypeScript and Python implementations.
- Shared semantic fixtures and a six-direction interoperability harness.
- Bounded archive handling, traversal/special-file rejection, SHA-256 verification, and conservative redaction warnings.
- CI, package validation, tutorial, examples, troubleshooting, migration, and contributor guidance.

## Safety boundary

ReproPack does not execute evidence, replay environments, or guarantee secret removal. Review bundles before sharing. Redaction detection is advisory and incomplete, and binary input is not scanned by the conservative helpers.

## Compatibility

Readers accept specification version `0.1`. Unknown versions and unknown ordinary manifest fields are rejected. Extension data belongs under URI-keyed `extensions` members.

## Known follow-up

Streaming decompression, broader fuzzing, external security review, language-specific packaging policy, and optional integrations remain post-v0.1 work.
