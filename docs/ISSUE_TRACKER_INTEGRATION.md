# Issue-tracker integration contract

This document defines the safe boundary for attaching a ReproPack bundle to an existing issue. The repository now includes a local preview generator for GitHub Issues, Jira, and Linear. It never calls a tracker API, creates an issue, modifies an issue, or uses a tracker token.

## Common flow

1. A user explicitly selects a failed workflow and a bounded set of evidence files.
2. GitHub Actions creates, validates, and verifies the bundle.
3. The integration produces an attachment record containing the bundle URL, SHA-256 digest, repository, commit, and retention date.
4. A user or approved automation attaches that record to an existing issue.
5. The tracker adapter records the public issue URL without copying secrets or unrestricted logs.

The bundle itself remains an ordinary artifact. No tracker integration may execute bundle contents, reconstruct an environment, or silently capture an entire workspace.

## Adapter profiles

| Tracker | Initial safe operation | Required configuration | Not included |
| --- | --- | --- | --- |
| GitHub Issues | Add a release/CI artifact link to an existing issue | Repository, issue number, token with least privilege | Creating issues automatically |
| Jira | Add a comment or attachment link to an existing issue | Base URL, project/key, scoped token, retention policy | Searching or bulk-updating issues |
| Linear | Add a comment/link to an existing issue | Workspace/team, issue ID, scoped token | Creating or assigning issues |

## Security requirements

- Require explicit opt-in and an existing issue identifier.
- Use read-only repository permissions for capture workflows.
- Never pass repository secrets into bundle contents or issue text.
- Never upload pull-request code through a privileged `pull_request_target` workflow.
- Preserve the bundle digest and source commit in the attachment record.
- Make retention and deletion behavior visible to the user.
- Treat all issue text and attachments as potentially public.

## Local preview generator

Generate a reviewable payload for an existing issue without network access:

```bash
node scripts/issue_tracker_preview.mjs input.json preview.json
```

The input must identify an existing issue and include the repository, workflow, exact commit, run URL, bundle URL, SHA-256 digest, and retention date. The output is explicitly marked `mode: "preview"`, `network_request: false`, and `creates_issue: false`. The safety and validation contract is tested with:

```bash
node scripts/test_issue_tracker_preview.mjs
```

The preview is the supported integration boundary for now. A maintainer may review and manually attach its contents to an existing issue; no automated tracker mutation is performed.

## Implementation order

The GitHub Actions capture workflow and the offline preview generator are the reference integrations. Live Jira and Linear API adapters remain intentionally deferred until a maintainer selects an API client, token scope, retention policy, and test workspace. No live tracker credentials are used.
