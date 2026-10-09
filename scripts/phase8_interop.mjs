import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

const core = resolve(fileURLToPath(new URL("..", import.meta.url)));
const workspace = resolve(core, "..");
const typescript = join(workspace, "repropack-typescript");
const python = join(workspace, "repropack-python");
const pythonRuntime = process.env.REPROPACK_PYTHON ?? join(workspace, "tools", "python312", "python.exe");
const fixtures = join(core, "conformance", "fixtures");
const outputRoot = join(core, "target", "phase8-interop");
const cases = ["minimal-valid", "redacted-valid"];

function run(command, args, cwd, env = {}) {
  const result = spawnSync(command, args, { cwd, encoding: "utf8", env: { ...process.env, ...env }, stdio: ["ignore", "pipe", "pipe"] });
  if (result.status !== 0) throw new Error(`${command} ${args.join(" ")} failed\n${result.stdout}\n${result.stderr}`);
  return result.stdout.trim();
}

function rustProduce(caseId, output) {
  const root = join(fixtures, caseId);
  const manifest = join(root, "manifest.json");
  const evidence = JSON.parse(readFileSync(manifest, "utf8")).evidence;
  const mappings = evidence.map((entry) => `${entry.path}=${join(root, entry.path.replaceAll("/", "\\"))}`);
  run("cargo", ["run", "--quiet", "--", "capture", manifest, output, ...mappings], core);
}

function typescriptProduce(caseId, output) {
  const moduleUrl = pathToFileURL(join(typescript, "dist", "src", "bundle.js")).href;
  const code = `import { readFileSync, writeFileSync } from "node:fs"; import { join } from "node:path"; import { createBundle } from ${JSON.stringify(moduleUrl)}; const [manifestPath, fixtureRoot, destination] = process.argv.slice(1); const manifest = JSON.parse(readFileSync(manifestPath, "utf8")); const evidence = new Map(manifest.evidence.map((entry) => [entry.path, readFileSync(join(fixtureRoot, entry.path.replaceAll("/", "\\\\")))])); writeFileSync(destination, createBundle(manifest, evidence));`;
  run("node", ["--input-type=module", "-e", code, join(fixtures, caseId, "manifest.json"), join(fixtures, caseId), output], typescript);
}

function pythonProduce(caseId, output) {
  const code = "from pathlib import Path; import json, sys; from repropack import Manifest, create_bundle; root=Path(sys.argv[1]); destination=Path(sys.argv[2]); manifest=Manifest.from_dict(json.loads((root/'manifest.json').read_text(encoding='utf-8'))); evidence={entry.path:(root/entry.path).read_bytes() for entry in manifest.evidence}; destination.write_bytes(create_bundle(manifest,evidence))";
  run(pythonRuntime, ["-c", code, join(fixtures, caseId), output], python, { PYTHONPATH: join(python, "src") });
}

function typescriptConsume(bundle) {
  const moduleUrl = pathToFileURL(join(typescript, "dist", "src", "bundle.js")).href;
  const code = `import { readFileSync } from "node:fs"; import { readBundle, verifyBundle } from ${JSON.stringify(moduleUrl)}; const bundle = readBundle(readFileSync(process.argv[1])); verifyBundle(bundle); console.log(JSON.stringify({ bundleId: bundle.manifest.bundle_id, entries: bundle.manifest.evidence.map((entry) => ({ path: entry.path, size: entry.size, sha256: entry.sha256, redaction: entry.redaction })) }));`;
  return JSON.parse(run("node", ["--input-type=module", "-e", code, bundle], typescript));
}

function pythonConsume(bundle) {
  const code = "from pathlib import Path; import json, sys; from repropack import read_bundle, verify_bundle; bundle=read_bundle(Path(sys.argv[1]).read_bytes()); verify_bundle(bundle); print(json.dumps({'bundleId': bundle.manifest.bundle_id, 'entries': [{'path': entry.path, 'size': entry.size, 'sha256': entry.sha256, 'redaction': entry.redaction} for entry in bundle.manifest.evidence]}, sort_keys=True))";
  return JSON.parse(run(pythonRuntime, ["-c", code, bundle], python, { PYTHONPATH: join(python, "src") }));
}

function rustConsume(bundle) {
  run("cargo", ["run", "--quiet", "--", "verify", bundle], core);
}

if (!existsSync(join(typescript, "dist", "src", "bundle.js"))) throw new Error("TypeScript build missing; run npm.cmd run build first");
if (!existsSync(pythonRuntime)) throw new Error(`Python runtime missing: ${pythonRuntime}`);
rmSync(outputRoot, { recursive: true, force: true }); mkdirSync(outputRoot, { recursive: true });
const matrix = [];
for (const caseId of cases) {
  const paths = { rust: join(outputRoot, `${caseId}-rust.rpk`), typescript: join(outputRoot, `${caseId}-typescript.rpk`), python: join(outputRoot, `${caseId}-python.rpk`) };
  rustProduce(caseId, paths.rust); typescriptProduce(caseId, paths.typescript); pythonProduce(caseId, paths.python);
  const summaries = new Map([["typescript", typescriptConsume(paths.rust)], ["python", pythonConsume(paths.rust)]]);
  for (const [producer, bundle] of Object.entries(paths)) {
    const consumers = { rust: rustConsume, typescript: typescriptConsume, python: pythonConsume };
    for (const [consumer, consume] of Object.entries(consumers)) {
      if (producer === consumer) continue;
      const summary = consume(bundle);
      if (consumer !== "rust") {
        const baseline = summaries.get(consumer);
        if (baseline && JSON.stringify(summary) !== JSON.stringify(baseline)) throw new Error(`${caseId}: normalized metadata mismatch for ${producer}->${consumer}`);
      }
      matrix.push({ case: caseId, producer, consumer, result: "pass" });
    }
  }
}
console.log(JSON.stringify({ fixtures: cases, matrix }, null, 2));
