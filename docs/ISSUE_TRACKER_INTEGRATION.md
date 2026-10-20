# Issue-tracker integration contract

This document defines the safe boundary for attaching a ReproPack bundle to an issue. It is an integration contract, not an active connector, and it does not create or modify issues.

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

## Implementation order

The GitHub Actions capture workflow is the first reference integration. Jira and Linear adapters should be added only after a maintainer selects an API client, token scope, retention policy, and test workspace. Until then, this contract is the complete supported boundary and no live tracker credentials are used.
