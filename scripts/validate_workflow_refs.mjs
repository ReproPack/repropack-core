import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const workflow = readFileSync(fileURLToPath(new URL("../.github/workflows/ci.yml", import.meta.url)), "utf8");

if (workflow.includes("github.head_ref") || workflow.includes("github.ref_name")) {
  throw new Error("workflow must not derive sibling revisions from the triggering branch");
}
for (const input of ["typescript_ref", "python_ref"]) {
  if (!workflow.includes(`      ${input}:`)) throw new Error(`missing explicit input: ${input}`);
  if (!workflow.includes(`inputs.${input} || 'main'`)) throw new Error(`missing safe default for ${input}`);
}
if (!workflow.includes("workflow_dispatch:")) throw new Error("coordinated workflow_dispatch trigger is missing");
console.log("workflow revision selection: PASS");
