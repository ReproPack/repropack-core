#!/usr/bin/env node
import { readFile, writeFile } from "node:fs/promises";

const TRACKERS = new Set(["github", "jira", "linear"]);
const SHA256 = /^[0-9a-f]{64}$/u;
const SHA = /^[0-9a-f]{40}$/u;

function requiredString(value, name) {
  if (typeof value !== "string" || value.trim() === "") throw new Error(`${name} is required`);
  return value.trim();
}

function url(value, name) {
  const result = requiredString(value, name);
  try { return new URL(result).toString(); } catch { throw new Error(`${name} must be an absolute URL`); }
}

export function createIssueTrackerPreview(input) {
  const tracker = requiredString(input.tracker, "tracker").toLowerCase();
  if (!TRACKERS.has(tracker)) throw new Error(`unsupported tracker: ${tracker}`);
  const issue = url(input.issue_url, "issue_url");
  const bundle = url(input.bundle_url, "bundle_url");
  const run = url(input.run_url, "run_url");
  const repository = requiredString(input.repository, "repository");
  const workflow = requiredString(input.workflow, "workflow");
  const commit = requiredString(input.commit, "commit");
  if (!SHA.test(commit)) throw new Error("commit must be a full 40-character SHA-1");
  const sha256 = requiredString(input.sha256, "sha256").toLowerCase();
  if (!SHA256.test(sha256)) throw new Error("sha256 must be a 64-character lowercase hex digest");
  const retentionUntil = requiredString(input.retention_until, "retention_until");
  if (Number.isNaN(Date.parse(retentionUntil))) throw new Error("retention_until must be an ISO date");

  const summary = [
    "ReproPack failure evidence (preview; no tracker request sent)",
    `Repository: ${repository}`,
    `Workflow: ${workflow}`,
    `Commit: ${commit}`,
    `Run: ${run}`,
    `Bundle: ${bundle}`,
    `SHA-256: ${sha256}`,
    `Retention until: ${retentionUntil}`,
  ].join("\n");

  return {
    schema: "repropack.issue-tracker-preview/1",
    mode: "preview",
    network_request: false,
    tracker,
    existing_issue_url: issue,
    summary,
    payload: tracker === "github"
      ? { body: summary, comment_on_existing_issue: true }
      : tracker === "jira"
        ? { body: { type: "doc", version: 1, content: [{ type: "paragraph", content: [{ type: "text", text: summary }] }] }, add_comment_to_existing_issue: true }
        : { body: summary, comment_on_existing_issue: true },
    safety: {
      creates_issue: false,
      modifies_tracker: false,
      captures_secrets: false,
      requires_explicit_existing_issue: true,
    },
  };
}

async function main() {
  const [inputPath, outputPath] = process.argv.slice(2);
  if (!inputPath || !outputPath) throw new Error("usage: node scripts/issue_tracker_preview.mjs input.json output.json");
  const input = JSON.parse(await readFile(inputPath, "utf8"));
  const preview = createIssueTrackerPreview(input);
  await writeFile(outputPath, `${JSON.stringify(preview, null, 2)}\n`, "utf8");
  console.log(`Issue-tracker preview written for ${preview.tracker}; no network request was made.`);
}

if (import.meta.url === `file://${process.argv[1].replaceAll("\\", "/")}`) await main();
