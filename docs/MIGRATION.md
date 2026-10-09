# Migration guidance

## From ad hoc log archives

1. Keep original files outside the bundle staging directory.
2. Create a v0.1 manifest with explicit evidence paths and provenance.
3. Review selected files for credentials, private data, and unrelated source.
4. Apply deliberate replacement bytes before calculating `size` and `sha256`.
5. Create the ZIP bundle, run `validate`, then run `verify`.
6. Share only after reviewing contents and metadata.

Do not copy a filesystem tree wholesale, rely on filename extensions for safety, or treat a successful ZIP open as verification.

## From a future ReproPack revision

Check `spec_version` before consuming evidence. Unknown versions must be rejected rather than interpreted optimistically. Preserve URI-keyed extension data where possible, but never let an extension override core semantics.

## From the Rust CLI to native SDKs

The TypeScript package exposes `createBundle`, `readBundle`, `verifyBundle`, and `extractBundle`. The Python package exposes `create_bundle`, `read_bundle`, `verify_bundle`, and `extract_bundle`. Both consume the same canonical fixtures and exchange bundles with Rust; neither invokes Rust at runtime.
