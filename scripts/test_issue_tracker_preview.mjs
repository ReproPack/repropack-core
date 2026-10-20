import assert from "node:assert/strict";
import { createIssueTrackerPreview } from "./issue_tracker_preview.mjs";

const base = {
  issue_url: "https://github.com/ReproPack/repropack-core/issues/42",
  bundle_url: "https://github.com/ReproPack/repropack-core/actions/artifacts/123",
  run_url: "https://github.com/ReproPack/repropack-core/actions/runs/123",
  repository: "ReproPack/repropack-core",
  workflow: "CI",
  commit: "0123456789abcdef0123456789abcdef01234567",
  sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
  retention_until: "2026-11-01T00:00:00Z",
};

for (const tracker of ["github", "jira", "linear"]) {
  const preview = createIssueTrackerPreview({ ...base, tracker });
  assert.equal(preview.mode, "preview");
  assert.equal(preview.network_request, false);
  assert.equal(preview.safety.creates_issue, false);
  assert.equal(preview.safety.modifies_tracker, false);
  assert.equal(preview.existing_issue_url, base.issue_url);
}

for (const [field, value] of [["issue_url", ""], ["bundle_url", "not-a-url"], ["sha256", "bad"]]) {
  assert.throws(() => createIssueTrackerPreview({ ...base, tracker: "github", [field]: value }), new RegExp(field));
}
assert.throws(() => createIssueTrackerPreview({ ...base, tracker: "github", commit: "main" }), /commit/);
assert.throws(() => createIssueTrackerPreview({ ...base, tracker: "github", issue_url: "relative/issue" }), /issue_url/);
console.log("Issue-tracker preview safety tests: PASS");
