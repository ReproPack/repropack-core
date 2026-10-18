# Generated security fixtures

The implementation test suites generate these ZIP cases in memory so the tests
remain deterministic and do not depend on an external ZIP utility:

- an archive whose local and central directory entries carry the ZIP encrypted
  flag; readers must reject it before attempting to consume content;
- a highly compressible entry whose declared uncompressed size exceeds the
  configured per-entry limit; readers must reject it before inflation;
- traversal and unsafe-path entries; readers must reject them without writing
  outside the extraction destination.

These cases are security fixtures, not user evidence. They must not contain
real credentials, private data, or generated environment state.
