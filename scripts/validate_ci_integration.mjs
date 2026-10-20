import { readFile } from "node:fs/promises";

const path = new URL("../docs/CI_INTEGRATION.md", import.meta.url);
const source = (await readFile(path, "utf8")).replaceAll("\r\n", "\n");

const required = [
  "workflow_run:",
  "permissions:\n  contents: read",
  "persist-credentials: false",
  "actions/upload-artifact@v4",
  "if-no-files-found: error",
  "retention-days: 7",
  "does not execute untrusted pull-request code",
];
for (const marker of required) {
  if (!source.includes(marker)) throw new Error(`CI integration is missing required safety marker: ${marker}`);
}

const workflow = source.match(/```yaml\n([\s\S]*?)\n```/u)?.[1] ?? "";
const forbidden = ["permissions: write-all", "permissions: {}", "secrets.", "pull_request_target:"];
for (const marker of forbidden) {
  if (workflow.includes(marker)) throw new Error(`CI integration contains forbidden marker: ${marker}`);
}
if (!workflow.includes("github.event.workflow_run.conclusion == 'failure'")) {
  throw new Error("CI integration must run only for failed workflow_run events");
}
if (!workflow.includes("path: failure.rpk")) {
  throw new Error("CI integration must upload only the validated bundle");
}
console.log("CI integration privacy and retention contract: PASS");
