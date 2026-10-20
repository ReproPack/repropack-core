import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const workflow = readFileSync(fileURLToPath(new URL("../.github/workflows/ci.yml", import.meta.url)), "utf8");
const releaseWorkflow = readFileSync(fileURLToPath(new URL("../.github/workflows/release-conformance.yml", import.meta.url)), "utf8");

if (workflow.includes("github.head_ref") || workflow.includes("github.ref_name")) {
  throw new Error("workflow must not derive sibling revisions from the triggering branch");
}
for (const input of ["typescript_ref", "python_ref"]) {
  if (!workflow.includes(`      ${input}:`)) throw new Error(`missing explicit input: ${input}`);
  if (!workflow.includes(`inputs.${input} || 'main'`)) throw new Error(`missing safe default for ${input}`);
}
if (!workflow.includes("workflow_dispatch:")) throw new Error("coordinated workflow_dispatch trigger is missing");
for (const marker of ["tags:", '"v*"', "repository: ReproPack/repropack-typescript", "repository: ReproPack/repropack-python", "actions/upload-artifact@v4"]) {
  if (!releaseWorkflow.includes(marker)) throw new Error(`release conformance workflow is missing: ${marker}`);
}
if (releaseWorkflow.includes("|| 'main'") || releaseWorkflow.includes("github.head_ref")) {
  throw new Error("release conformance workflow must use explicit revisions");
}
console.log("workflow revision selection: PASS");
