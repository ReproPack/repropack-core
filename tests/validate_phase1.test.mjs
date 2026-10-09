import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

test("Phase 1 validator resolves canonical fixture paths on the host OS", () => {
  const output = execFileSync(process.execPath, ["scripts/validate_phase1.mjs"], {
    cwd: root,
    encoding: "utf8",
  });

  assert.match(output, /minimal-valid: valid fixture PASS/);
  assert.match(output, /redacted-valid: valid fixture PASS/);
  assert.match(output, /unsafe-path: invalid fixture PASS \(unsafe-path\)/);
  assert.match(output, /Phase 1 audit: PASS/);
});
