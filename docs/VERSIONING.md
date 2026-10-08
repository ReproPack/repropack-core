# Versioning

The specification version describes bundle semantics. Each implementation has its own package/CLI version. Repository tags identify coordinated snapshots but do not make package patch versions identical. Additive optional metadata is compatible; required-field or interpretation changes require a new specification major version. Implementations MUST reject unsupported future major versions and MAY preserve unknown optional fields.
