# ReproPack v0.1 final audit

Audit date: 2026-10-09  
Specification: `0.1`  
Implementation version: `0.1.0`

## Release gate

| Criterion | Evidence | Result |
|---|---|---|
| Frozen specification and schema | `docs/SPECIFICATION.md`, `schema/manifest.schema.json`, Phase 1 audit | PASS |
| Rust implementation | 21 tests, Clippy, formatting, locked Cargo package | PASS |
| TypeScript implementation | Typecheck, build, 7 tests, npm audit, package dry-run | PASS |
| Python implementation | Compileall, 6 unittest tests, wheel/sdist, pip check | PASS |
| Shared conformance | Catalog runner and 12 canonical cases | PASS |
| Interoperability | Phase 8 harness: 12 directed paths across minimal/redacted fixtures | PASS |
| Security | Phase 9 threat matrix, malformed/limit/traversal/hash/redaction tests | PASS |
| Usability | Phase 10 CLI workflow, JSON errors, exit-code smoke test | PASS |
| CI and packaging | Real workflows in all repositories and local artifact checks | PASS |
| Documentation | Tutorial, example, format, troubleshooting, migration, contributor guides | PASS |
| Contributor readiness | Templates, compatibility/design/fixture/maintainer guidance | PASS |

## Commands and recorded results

```text
cargo fmt -- --check                          PASS
cargo clippy --all-targets -- -D warnings     PASS
cargo test --locked --all-targets              PASS (21 tests)
cargo package --locked --allow-dirty --no-verify PASS
node scripts/validate_phase1.mjs               PASS (12 fixtures)
node scripts/phase8_interop.mjs                PASS (12 paths)
npm.cmd run typecheck                           PASS
npm.cmd test                                    PASS (7 tests)
npm.cmd audit --audit-level=high                PASS (0 vulnerabilities)
npm.cmd pack --dry-run                          PASS
python -m compileall -q src tests               PASS
python -m unittest discover -s tests -v         PASS (6 tests)
python -m build --no-isolation --wheel --sdist  PASS
python -m pip check                             PASS
```

## Known limitations

The v0.1 format does not promise byte-identical ZIP archives. Redaction detection is heuristic and incomplete. Readers use bounded in-memory buffers; streaming decompression and an external security review remain follow-up recommendations. TypeScript is currently a private package artifact pending a later packaging decision.

## Release decision

The repository evidence supports a v0.1.0 release candidate. No code path executes bundle content. Publishing tags, package artifacts, or hosted releases requires a separate explicit publication action.

## Release-readiness correction addendum

The original local release snapshot contained a portability defect in `scripts/validate_phase1.mjs`: canonical `/` evidence paths were converted to `\` before `path.join`, causing the Ubuntu run `37865758652` to fail while resolving `minimal-valid/evidence/message.txt`. The correction keeps canonical paths unchanged and adds a host-OS regression test covering valid fixtures and the unsafe-path case.

The Phase 8 interoperability helper now uses platform-neutral path components, supports explicit `REPROPACK_PYTHON`, `REPROPACK_PYTHON_PROJECT`, and `REPROPACK_TYPESCRIPT` overrides, and performs platform-aware Python discovery with an actionable missing-runtime error. Hosted Ubuntu and Windows CI coverage passed in run [37919776037](https://github.com/ReproPack/repropack-core/actions/runs/37919776037).

This addendum does not change the existing `v0.1.0` tag. The tag still points to `5abad374be8bce57d60978b923b8f9dec4fbfef5`; the corrected synchronized `main` is `08807da720abeca2d685993ee14978b36e499192`. No remote tags, package publication, or hosted release exists.

## Post-CI failure correction

Hosted run `37918806077` established that the Windows checkout normalized canonical fixture evidence line endings, changing the byte length expected by the manifest. The repository now marks only `/conformance/fixtures/**/evidence/**` as `-text`; normal source-file line-ending behavior is unchanged. Existing Phase 1 validator tests cover fixture byte lengths, hashes, valid cases, and unsafe paths. The correction was validated by hosted Linux/Windows run [37919776037](https://github.com/ReproPack/repropack-core/actions/runs/37919776037); no publication is claimed here.

## Final release preparation audit

Verified facts:

- Core, TypeScript, and Python main branches are clean and synchronized at the reviewed corrected commits.
- All three hosted CI workflows passed, including Core's Ubuntu/Windows validator, Rust, and 12-path interoperability matrix.
- Existing local v0.1.0 tags target pre-correction commits. GitHub reports no remote tags and no Releases for these repositories.
- Core cargo publish dry-run passes. Python artifacts build and install in hosted CI. TypeScript package validation passes, but private: true remains set.

Recommendations:

- Preserve the old local tags and release corrected code as 0.1.1 if external consumption of the old tags cannot be ruled out.
- Recreate v0.1.0 only if the owner explicitly confirms the old tags were never externally consumed and accepts the tag replacement risk.
- Prepare coordinated release notes, exact artifact checksums, and GitHub Releases after the tag/version decision.

Owner decisions required:

- Coordinated version and tag strategy.
- crates.io and PyPI publication approval and publisher/name confirmation.
- Whether TypeScript becomes public; if yes, approval to change private: true and finalize package entry points/files.
- GitHub Release creation and final artifact contents.
