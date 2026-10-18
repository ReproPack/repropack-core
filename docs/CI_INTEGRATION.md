# Optional CI failure-evidence integration

This example shows a deliberately narrow GitHub Actions integration. It captures only files selected by the workflow author, validates the bundle, verifies its hashes, and uploads the result as an ordinary artifact. It does not collect the whole workspace, execute bundle contents, reconstruct an environment, or guarantee that selected files contain no secrets.

## Example workflow

The following is illustrative but executable after the `repropack-core` CLI is available. Replace the install step with the approved release installation method after publication.

```yaml
name: failure-evidence

on:
  workflow_run:
    workflows: [CI]
    types: [completed]

permissions:
  contents: read

jobs:
  capture:
    if: ${{ github.event.workflow_run.conclusion == 'failure' }}
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          ref: ${{ github.event.workflow_run.head_sha }}
          persist-credentials: false
      - name: Install ReproPack CLI
        run: cargo install repropack-core --version 0.1.1
      - name: Select safe evidence
        run: |
          mkdir -p repropack-evidence
          printf 'workflow=%s\ncommit=%s\n' \
            '${{ github.event.workflow_run.name }}' \
            '${{ github.event.workflow_run.head_sha }}' \
            > repropack-evidence/run.txt
          git diff --check > repropack-evidence/diff-check.txt || true
      - name: Build a manifest for the selected files
        run: |
          python - <<'PY'
          import hashlib, json
          from pathlib import Path
          path = Path("repropack-evidence/run.txt")
          data = path.read_bytes()
          manifest = {
              "bundle_id": "11111111-1111-4111-8111-111111111111",
              "capture": {"mode": "explicit", "tool": {"name": "github-actions-example", "version": "0.1.1"}},
              "created_at": "2026-01-01T00:00:00Z",
              "evidence": [{"kind": "text", "media_type": "text/plain", "path": "evidence/run.txt", "redaction": {"status": "none"}, "selection": "explicit", "sha256": hashlib.sha256(data).hexdigest(), "size": len(data)}],
              "format": "repropack",
              "spec_version": "0.1",
          }
          Path("manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
          PY
      - name: Create, validate, and verify bundle
        run: |
          repropack-core capture manifest.json failure.rpk \
            evidence/run.txt=repropack-evidence/run.txt
          repropack-core validate failure.rpk
          repropack-core verify failure.rpk
      - name: Upload validated evidence
        uses: actions/upload-artifact@v4
        with:
          name: repropack-${{ github.event.workflow_run.head_sha }}
          path: failure.rpk
          if-no-files-found: error
          retention-days: 7
```

The example uses synthetic metadata and a short retention period. Real deployments must review the selected files before upload, avoid credentials and private customer data, and apply the repository's artifact-retention policy. The `workflow_run` trigger can expose sensitive context, so this example intentionally uses read-only permissions and does not execute untrusted pull-request code.

## Local demonstration

The repository's checked-in minimal example is the canonical smoke test:

```bash
cargo run -- capture examples/minimal/manifest.json target/minimal.rpk \
  evidence/message.txt=examples/minimal/evidence/message.txt
cargo run -- validate target/minimal.rpk
cargo run -- verify target/minimal.rpk
```

For unsafe, corrupted, wrong-hash, and oversized inputs, use the canonical cases in `conformance/fixtures` and the fixture runner described in [CONFORMANCE.md](CONFORMANCE.md).

The static safety contract for this example is checked with:

```bash
node scripts/validate_ci_integration.mjs
```

This validates the documented read-only permission, failure-only trigger,
credentialless checkout, seven-day retention, upload failure behavior, and
single-bundle artifact scope. It does not prove that a repository's selected
files are free of secrets; human review remains required before enabling the
integration.
