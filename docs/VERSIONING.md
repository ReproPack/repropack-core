# Versioning

The specification version describes bundle semantics. Phase 1 freezes the v0.1 draft contract as specification version `0.1`; implementation conformance is still pending. Each implementation has its own package/CLI version. Repository tags identify coordinated snapshots but do not make package patch versions identical. Additive optional metadata is compatible; required-field or interpretation changes require a new specification major version. Implementations MUST reject unsupported future major versions and MAY preserve unknown extension fields under `extensions`.
