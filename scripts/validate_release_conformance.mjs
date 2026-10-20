import { readFile } from "node:fs/promises";

const workflow = await readFile(new URL("../.github/workflows/release-conformance.yml", import.meta.url), "utf8");
const required = [
  'tags:',
  '"v*"',
  'workflow_dispatch:',
  'required: true',
  'repository: ReproPack/repropack-typescript',
  'repository: ReproPack/repropack-python',
  'ref: ${{ steps.refs.outputs.typescript_ref }}',
  'ref: ${{ steps.refs.outputs.python_ref }}',
  'node scripts/phase8_interop.mjs',
  'actions/upload-artifact@v4',
];
for (const marker of required) {
  if (!workflow.includes(marker)) throw new Error(`release conformance workflow is missing: ${marker}`);
}
if (workflow.includes("|| 'main'") || workflow.includes('github.head_ref')) {
  throw new Error('release conformance must not silently fall back to main or triggering branch names');
}
console.log('release conformance workflow: PASS');
