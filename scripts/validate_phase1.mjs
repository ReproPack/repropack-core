import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(fileURLToPath(new URL("..", import.meta.url)));
const fixtureRoot = join(root, "conformance", "fixtures");
const allowedKinds = new Set(["log", "text", "structured", "source", "environment", "test-output", "file"]);
const allowedSelection = new Set(["explicit", "generated", "derived"]);
const pathPattern = /^evidence\/[A-Za-z0-9._~-]+(?:\/[A-Za-z0-9._~-]+)*$/;
const mediaPattern = /^[a-z0-9!#$%&'*+.^_`|~-]+\/[a-z0-9!#$%&'*+.^_`|~-]+$/;
const uuidPattern = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
const timestampPattern = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,9})?Z$/;

function fail(message) { throw new Error(message); }
function object(value, label) { if (!value || typeof value !== "object" || Array.isArray(value)) fail(`${label} must be an object`); }
function exactKeys(value, allowed, label) { for (const key of Object.keys(value)) if (!allowed.has(key)) fail(`${label} has unknown field ${key}`); }
function required(value, keys, label) { for (const key of keys) if (!(key in value)) fail(`${label} missing ${key}`); }
function string(value, label) { if (typeof value !== "string") fail(`${label} must be a string`); }

async function json(file) { return JSON.parse(await readFile(file, "utf8")); }
async function validateManifest(manifest, label, options = {}) {
  object(manifest, label);
  exactKeys(manifest, new Set(["format", "spec_version", "bundle_id", "created_at", "capture", "incident", "evidence", "extensions"]), label);
  required(manifest, ["format", "spec_version", "bundle_id", "created_at", "capture", "evidence"], label);
  if (manifest.format !== "repropack") fail(`${label}.format`);
  if (manifest.spec_version !== "0.1") fail(`${label}.spec_version`);
  if (!uuidPattern.test(manifest.bundle_id)) fail(`${label}.bundle_id`);
  if (!timestampPattern.test(manifest.created_at)) fail(`${label}.created_at`);
  object(manifest.capture, `${label}.capture`);
  exactKeys(manifest.capture, new Set(["mode", "tool", "actor", "source"]), `${label}.capture`);
  required(manifest.capture, ["mode", "tool"], `${label}.capture`);
  if (!["explicit", "generated"].includes(manifest.capture.mode)) fail(`${label}.capture.mode`);
  object(manifest.capture.tool, `${label}.capture.tool`);
  exactKeys(manifest.capture.tool, new Set(["name", "version"]), `${label}.capture.tool`);
  required(manifest.capture.tool, ["name", "version"], `${label}.capture.tool`);
  for (const key of ["name", "version"]) { string(manifest.capture.tool[key], `${label}.capture.tool.${key}`); if (!manifest.capture.tool[key]) fail(`${label}.capture.tool.${key}`); }
  if (manifest.incident !== undefined) object(manifest.incident, `${label}.incident`);
  if (manifest.extensions !== undefined) {
    object(manifest.extensions, `${label}.extensions`);
    for (const key of Object.keys(manifest.extensions)) { try { new URL(key); } catch { fail(`${label}.extensions.${key}`); } }
  }
  if (!Array.isArray(manifest.evidence) || manifest.evidence.length < 1) fail(`${label}.evidence`);
  let previous = "";
  const paths = new Set();
  for (const [index, evidence] of manifest.evidence.entries()) {
    const prefix = `${label}.evidence[${index}]`;
    object(evidence, prefix);
    exactKeys(evidence, new Set(["path", "kind", "media_type", "size", "sha256", "selection", "redaction"]), prefix);
    required(evidence, ["path", "kind", "media_type", "size", "sha256", "selection", "redaction"], prefix);
    if (!pathPattern.test(evidence.path) || evidence.path.includes("..")) fail(`${prefix}.path`);
    if (paths.has(evidence.path)) fail(`${prefix}.duplicate path`);
    if (evidence.path <= previous) fail(`${prefix}.path not sorted`);
    paths.add(evidence.path); previous = evidence.path;
    if (!allowedKinds.has(evidence.kind)) fail(`${prefix}.kind`);
    if (!mediaPattern.test(evidence.media_type)) fail(`${prefix}.media_type`);
    if (!Number.isSafeInteger(evidence.size) || evidence.size < 0) fail(`${prefix}.size`);
    if (!/^[0-9a-f]{64}$/.test(evidence.sha256)) fail(`${prefix}.sha256`);
    if (!allowedSelection.has(evidence.selection)) fail(`${prefix}.selection`);
    object(evidence.redaction, `${prefix}.redaction`);
    exactKeys(evidence.redaction, new Set(["status", "reason"]), `${prefix}.redaction`);
    if (!["none", "redacted"].includes(evidence.redaction.status)) fail(`${prefix}.redaction.status`);
    if (evidence.redaction.status === "redacted" && !["secret", "personal-data", "user-requested", "policy"].includes(evidence.redaction.reason)) fail(`${prefix}.redaction.reason`);
    if (evidence.redaction.status === "none" && "reason" in evidence.redaction) fail(`${prefix}.redaction.reason must be absent`);
  }
  if (options.checkFiles) {
    for (const evidence of manifest.evidence) {
      if (evidence.size > 256 * 1024 * 1024) fail(`${label}: size exceeds configured per-entry limit`);
      // Manifest paths are canonical POSIX-style paths. Node's path.join accepts
      // those separators on Windows, while preserving them as separators on
      // POSIX; converting them to backslashes breaks Linux fixture validation.
      const file = join(options.fixtureDir, evidence.path);
      const bytes = await readFile(file);
      const digest = createHash("sha256").update(bytes).digest("hex");
      if (bytes.length !== evidence.size) fail(`${label}: ${evidence.path} size mismatch`);
      if (digest !== evidence.sha256) fail(`${label}: ${evidence.path} hash mismatch`);
    }
  }
}

const catalog = await json(join(root, "conformance", "catalog.json"));
if (catalog.spec_version !== "0.1" || !Array.isArray(catalog.cases)) fail("invalid fixture catalog");
const expectedSignals = {
  "missing-required-field": "missing capture",
  "invalid-metadata": "capture.mode",
  "corrupted-content": "hash mismatch",
  "incorrect-hash": "hash mismatch",
  "unsupported-future-version": "spec_version",
  "unsafe-path": ".path",
  "oversized-input": "size exceeds configured per-entry limit"
};
for (const entry of catalog.cases) {
  const manifestFile = join(fixtureRoot, entry.id, "manifest.json");
  const expected = await json(join(root, entry.expected));
  if (expected.valid !== entry.valid) fail(`${entry.id}: catalog/expected mismatch`);
  let passed = false;
  try {
    const manifest = await json(manifestFile);
    await validateManifest(manifest, entry.id, { checkFiles: entry.valid, fixtureDir: join(fixtureRoot, entry.id) });
    passed = true;
  } catch (error) {
    if (entry.valid) throw error;
    if (expectedSignals[entry.id] && !error.message.includes(expectedSignals[entry.id])) {
      throw new Error(`${entry.id}: expected ${expectedSignals[entry.id]}, got ${error.message}`);
    }
    passed = true;
  }
  if (!passed) fail(`${entry.id}: no result`);
  console.log(`${entry.id}: ${entry.valid ? "valid fixture PASS" : `invalid fixture PASS (${expected.error})`}`);
}
console.log("Phase 1 audit: PASS");
