# Review — s1-chaine-release

[Story](../stories.md) · [Plan](../plans/s1-chaine-release.md) · [Verification](../verif/s1-chaine-release.md)

Diff reviewed: `git diff main...story/s1-chaine-release`, in a context that did not write it. Findings rated critical, major or minor.

Review status: complete
Fix loop: 0
Reviewed tree: 02d168d7375471fb15b517b9fce9e017a858aca2

(The tree was taken again just before this answer and had not changed. The worktree is clean.)

## Ownership
I listed the diff with `git diff --name-only main...story/s1-chaine-release` and checked each path against `Owns: Cargo.toml, Cargo.lock, rust-toolchain.toml, crates/, plugin/, .github/, scripts/, .gitignore, LICENSE` (`docs/stories.md:13`).
- [x] Every code path is inside `Owns`. Inside by construction: the story's own `docs/research|plans|verif|designs|qa` files, and `docs/decisions/0007-provenance-depot-prive.md`, an ADR the diff adds (status `A`) and the plan names.
- [x] Outside `Owns`, one path, graded major: `AGENTS.local.md`, which no `Owns` entry covers. It changed in commit d3389ef, written by a human and not the implementer: `Dev: — → scripts/dev.sh`.
- [x] Inside another running story's `Owns`: none. `git worktree list` shows only this story's worktree.
- [x] The story has an `Owns` line.

## Evidence reused
- [x] `po-gate verif-current s1-chaine-release` exited 0, so I reused the record. `Tree:` 2c7b7ad… and the run trees 311f7ef… and 2ad9acd… match HEAD outside `docs/` (empty `git diff --stat` for each). Unit suite and type check both exited 0. I read the record's "Not proven here": criteria 1 and 2 only at the first tag, CI never run, no TypeScript formatter.
- [x] `po-gate qa-current s1-chaine-release` exited 0, so I carried the two verified QA findings below. The QA `Tree:` 7a75e59… matches HEAD outside `docs/`.

## Plan
- [x] Tasks 1–10 are all present. I found nothing outside the plan, apart from the `AGENTS.local.md` commit.
- [x] I checked each run interdict:
  - Empty diff on `AGENTS.md`, `.polype/`, `.claude/`, `docs/stories.md`, `docs/architecture.md`, `docs/design-system.md` and ADRs 0001–0006: held.
  - Empty diff on `AGENTS.local.md`: **broken** (finding below).
  - Nothing outside s1's `Owns` apart from the listed docs: broken by the same file.
  - No tracked `.wasm`, `main.js`, `wasm-pkg/` or base64 blob: held (`node scripts/check-no-blobs.mjs` exits 0).
  - No `lophelia-bench`, `web/`, Worker, `plugin/src/vault/`, `saveData`, `vault.create` or `innerHTML`: held.
  - Dependencies: Rust is `wasm-bindgen =0.2.129`. npm has exactly `esbuild`, `typescript`, `obsidian` and `binaryen`, all exact. External actions are only checkout, setup-node and attest-build-provenance. Held.
  - No tag or release: held (no PR or run exists either).
  - No write to `~/.config/obsidian/`: `dev.sh` uses an isolated `--user-data-dir`. Held.

## References
- [x] Generated glue `plugin/wasm-pkg/lophelia_wasm.js`: it exports `initSync` (line 112, exported at 158) and `engine_version` (line 6). Only those two are imported; the async `__wbg_init` (fetch, `import.meta.url`) is dropped from the bundle. The local `plugin/main.js` passes `check-network`, and `import.meta`/`new URL` appear nowhere in it.
- [x] Pinned actions resolved with `gh api`:
  - checkout v7.0.1 = 3d3c42e5…
  - setup-node v7.1.0 = 949feb24…
  - attest-build-provenance v4.2.2 = 4d101475…
- [x] Action inputs exist at those SHAs: `node-version-file` and `package-manager-cache` in setup-node, a multi-line `subject-path` in attest. `artifact-metadata: write` is only needed with `push-to-registry`, which is not used.
- [x] `cargo-audit 0.22.2` and `wasm-bindgen-cli 0.2.129` exist on crates.io.
- [x] The repository is now public (`visibility: public`) and has the `main` and `release-tags` rulesets, consistent with ADR-0007.
- [x] Values:
  - Versions agree at 0.1.0 across the workspace, `package.json` and `manifest.json`.
  - `minAppVersion` 1.14.4 equals the pinned `obsidian` types, as ADR-0005 and the plan require.
  - `.nvmrc` 24.21.0 and toolchain 1.99.0 are exact.

## Rules
- [x] `AGENTS.md` conventions:
  - `lophelia-core` has no dependencies.
  - The plugin has no network API.
  - Text is set with `setText` only.
  - No colour is hard-coded.
  - `.lophelia-view` scopes the corrected tokens, which match the "→" values of `docs/design-system.md`.
- [x] No accepted ADR is contradicted.

## Tests
- [x] Floor: `po-gate floor-guard s1-chaine-release main` exited 0, so nothing was lowered.
- [x] None of the six empty test shapes. Fixtures for the blob checks are built at run time. The Rust test is close to tautological, but the plan accepts that it only guards the binding.
- [x] 12 tests, within the `Test budget` of 25.
- [x] Bite, three mutations, each undone and confirmed clean with `git diff --exit-code`:
  - `fetch` rule of `scripts/check-network.mjs` replaced by `/(?!)/`: 1 of 7 tests failed.
  - `from "<http|https|net>"` branch of the module rule disabled: 1 of 7 failed.
  - base64-WASM check of `scripts/check-no-blobs.mjs` disabled: 1 of 4 failed.
- [x] No tests were made useless; the repository had no code before this story.

## Neighbours
- [x] The repository had no prior code (ADR-0002), so the diff has no other callers. `engineVersion` is called only from `lophelia-view.ts`.

## Lenses
Lenses run: dependencies — `Cargo.lock`, `plugin/package-lock.json`, `Cargo.toml` and `plugin/package.json` add dependencies.
- [x] The lens findings are graded below. Security did not run: the diff has no auth, route, upload or webhook path or content. Performance did not run: there is no server-side code.

## Findings
- major — `AGENTS.local.md:80` — `Dev: scripts/dev.sh` is outside s1's `Owns` and breaks the plan's run interdict "Diff vide sur AGENTS.local.md". The commit d3389ef is by the person, not the implementer, with a stated reason: screen QA needed `Dev` before landing. The plan had expected the person to set it after landing, as ADR-0006 does. Someone needs to choose: keep it in the s1 landing and widen the scope on purpose, or land it separately.
- major — `scripts/check-no-blobs.mjs:14-15,22-23` — criterion 3 ("ni blob WASM ou worker encodé en base64") is only partly enforced. The check catches a base64 WASM header and a `data:` URL with an explicit `;base64,` marker. It does not catch a worker kept as a bare base64 string decoded through `atob`/`Blob` (no `data:` prefix), nor a `data:;base64,` or `data:text/plain;base64,` URL. This matches the rules the plan fixed, so the gap is in the plan's rules more than the code. The CI would not fail on that kind of commit.
- minor — `plugin/src/main.ts:20-22` — QA finding 1, verified: when the view is already open, `revealLeaf(existing)` brings the tab to the front but does not make it the active leaf (`activeLeaf` stays on the note). The design note says the command "révèle la vue".
- minor — `plugin/src/view/lophelia-view.ts:24` — QA finding 2, verified: `createDiv({ cls: "lophelia-engine-version" })` runs before `engineVersion()` throws, so in the error state an empty `.lophelia-engine-version` div stays in front of `Moteur non chargé`.
- minor — `plugin/src/view/lophelia-view.ts:18-20` — the body reuses Obsidian's undocumented `empty-state` / `empty-state-container` CSS classes. The lead recorded this decision in `docs/designs/s1-chaine-release/design.md`, but `docs/design-system.md` does not list it. Those classes are not public API (ADR-0005 spirit), so an Obsidian update can move the layout. It should go into the design system through `/po-design-system`.
- minor — `.github/workflows/release.yml:3-5` — any `v*` tag on any commit triggers an attested release. Nothing checks that the tagged commit is on `main`, so a tag on an unreviewed branch commit would publish attested code. A `git merge-base --is-ancestor "$GITHUB_SHA" origin/main` step would close this.
- minor — `plugin/package-lock.json:891` (dependencies lens) — `esbuild` carries `"hasInstallScript": true` (`postinstall: node install.js`). Neither the plan nor the record says the install script was read.
- minor — `scripts/dev.sh:27-28` — the vault path is put into `obsidian.json` with `printf '%s'` and no JSON escaping. A `XDG_CACHE_HOME` containing `"` or `\` produces invalid JSON. This only affects the development script.

## Not verified
- CI has never run: no PR and no Actions run exists for `story/s1-chaine-release`. So none of these ran on a GitHub runner: the composite action (`rustup toolchain install`, the `cargo install` steps, `npm ci`, clippy, fmt, `cargo audit`, `npm audit`, build, network check, size summary). A person should open the PR and confirm it is green.
- Criteria 1 and 2 (tag → release, `gh attestation verify`) can only be proven at the first `v0.1.0` tag after landing. A person should push the tag on `main` and run `gh attestation verify <file> --repo atoll-labs-io/lophelia` on each of the three files.
- `cargo audit` fails on any advisory, not only high or critical. That is stricter than criterion 7; I did not check whether it would block on a low advisory. The research flagged it.
- `build-wasm.sh` refusing a mismatched `wasm-bindgen` is claimed by the record. I did not reproduce it.
- I did not open the screen myself; I relied on the QA captures. The QA report's `## Not seen`, with its reasons:
  - C1, C2: no screen effect; only visible after a release.
  - C3, C4, C6, C7: repository and CI checks with no screen effect.
  - C5 installed from a GitHub release: no release exists yet.
  - C5 WASM provenance of the text: the screen cannot tell it apart from a constant; only the simulated failure shows it indirectly.
  - Right side panel: the narrow width was only seen in a split screen.
  - System-forced themes outside Obsidian: not reachable from the origin.
- Synchronous `WebAssembly.Module` compilation on Electron's main thread works at today's module size (seen in QA). I did not assess it for the future engine size.

Max severity: major
Ship allowed: yes

---

# Review — s1-chaine-release

[Story](../stories.md) · [Plan](../plans/s1-chaine-release.md) · [Verification](../verif/s1-chaine-release.md)

Diff reviewed: `git diff origin/main...story/s1-chaine-release`, in a context that did not write it. Findings rated critical, major or minor.

Review status: complete
Fix loop: 1
Reviewed tree: b9b2fa466563f62fe2d9616ff22317acbd730b51

(I took the tree again just before answering and it had not changed. The worktree is clean.)

## Ownership
I listed the diff with `git diff --name-only origin/main...story/s1-chaine-release` and checked each path against `Owns: Cargo.toml, Cargo.lock, rust-toolchain.toml, crates/, plugin/, .github/, scripts/, .gitignore, LICENSE` (`docs/stories.md:13`).
- [x] Every code path is inside `Owns`. Inside by construction:
  - the story's own `docs/research|designs|plans|verif|qa|reviews` files;
  - `docs/decisions/0007-provenance-depot-prive.md`, which the diff adds (status `A`) and the plan names.
- [x] Outside `Owns`: none. Against `origin/main` (d3389ef), `AGENTS.local.md` is no longer in the diff. The previous review's major finding on that file came from the stale local `main` (0b05bb4), so it is withdrawn.
- [x] Inside another running story's `Owns`: none. `git worktree list` shows only this story's worktree.
- [x] The story has an `Owns` line.

## Evidence reused
- [x] `po-gate verif-current s1-chaine-release` exited 0, so I reused the record.
  - The record's `Tree:` 455b3fd… and the new run trees 51e9d75… (unit suite) and d76c400… (type check) all match HEAD outside `docs/` (empty diff for each).
  - Both checks exited 0. The unit suite command is now `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'`.
  - I read "Not proven here". The recorded suite ran on Node 26 locally. The record says the new form was also replayed by hand on Node 24.21.0.
- [x] Additional evidence for the fix: CI run 38586885564 (`pull_request`, PR #47, job `build`) concluded `success`.
  - It ran on head 032fb34. That commit's tree is e0ccf2b…, its parent is d3389ef (= `origin/main`), and e0ccf2b matches HEAD outside `docs/`.
  - So the full composite action ran green on a GitHub runner under Node 24 for this exact code: clippy, fmt, `cargo audit`, `npm audit`, build, network check.
- [x] `po-gate qa-current s1-chaine-release` exited 0. The QA `Tree:` e0ccf2b… matches, and the report is committed (525bc8a). Its three verified findings are carried below.

## Plan
- [x] Fix loop scope: commit 1422a47 changes only `.github/actions/build/action.yml:47`, from `node --test scripts/` to `node --test 'scripts/*.test.mjs'`, plus the plan and the record.
  - The quoted glob is expanded by Node's own test runner, not by the shell.
  - The plan was updated to match, and nothing outside the plan was added.
- [x] Tasks 1–10 are still present; the code was unchanged by this loop except that line.
- [x] Run interdicts, checked one by one against `origin/main`:
  - `AGENTS.md`, `AGENTS.local.md`, `.polype/`, `.claude/`, `docs/stories.md`, `docs/architecture.md`, `docs/design-system.md` and ADRs 0001–0006: not in the diff. Held.
  - No tracked `.wasm` or blob: held.
  - No `web/`, Worker, `plugin/src/vault/`, `saveData`, `vault.create` or `innerHTML`: held.
  - Exact dependency pins: held.
  - No tag or release: held.

## References
- [x] Reconfirmed from the previous review, on unchanged code: the `initSync`/`engine_version` glue, the pinned action SHAs and the versions (0.1.0 everywhere; `minAppVersion` 1.14.4; `.nvmrc` 24.21.0; toolchain 1.99.0).
- [x] The new test command matches all script test files: 11 tests (7 in `check-network.test.mjs`, 4 in `check-no-blobs.test.mjs`). The CI run confirms the form works under Node 24.

## Rules
- [x] `AGENTS.md` conventions are respected:
  - `lophelia-core` has no dependencies.
  - The plugin has no network API.
  - Text is set with `setText` only.
  - No `.wasm` file is committed.
- [x] No accepted ADR is contradicted.

## Tests
- [x] Floor: `po-gate floor-guard s1-chaine-release origin/main` exited 0, so nothing was lowered. The test command change widens nothing away; it still runs every `*.test.mjs` file.
- [x] None of the six empty test shapes. Fixtures for the blob checks are built at run time.
- [x] 12 tests (1 Rust, 11 Node), within the `Test budget` of 25.
- [x] Bite, two new mutations, different from the previous loop's three, each undone and confirmed clean with `git diff --exit-code`:
  - `.wasm` path rule of `scripts/check-no-blobs.mjs:21` replaced by `if (false)`: 1 of 4 tests failed ("a tracked .wasm file is refused").
  - `WebSocket` rule of `scripts/check-network.mjs:12` replaced by `/(?!)/`: 1 of 7 tests failed ("WebSocket is refused").
- [x] No tests were made useless; the repository had no code before this story.

## Neighbours
- [x] The repository had no prior code. `engineVersion` is called only from `plugin/src/view/lophelia-view.ts`. The composite action is used by `ci.yml` and `release.yml`, and both get the same test step.

## Lenses
Lenses run: dependencies — `Cargo.lock`, `plugin/package-lock.json`, `Cargo.toml` and `plugin/package.json` add dependencies.
- [x] The lens finding (esbuild install script) is graded below. Security did not run: no auth, route, upload or webhook path or content. Performance did not run: no server-side code.

## Findings
- major — `scripts/check-no-blobs.mjs:14-15,22-23` — criterion 3 ("ni blob WASM ou worker encodé en base64") is only partly enforced. The check does not catch:
  - a worker kept as a bare base64 string decoded through `atob`/`Blob`;
  - a `data:;base64,` or `data:text/plain;base64,` URL.

  The CI would not fail on such a commit. It is still open, filed as #42.
- minor — `plugin/src/main.ts:20-22` — QA finding 1, verified: the second "Ouvrir Lophelia" brings the tab to the front through `revealLeaf(existing)` but does not make it the active leaf. Filed as #40.
- minor — `plugin/src/view/lophelia-view.ts:23-24` — QA finding 2, verified: in the error state, an empty `div.lophelia-engine-version` stays before `Moteur non chargé`. Filed as #41.
- minor — `plugin/src/view/lophelia-view.ts` (view registration; the cause is not proven) — QA finding 3, verified and **new in this QA walk**: Escape in the active Lophelia view switches to the note tab and moves focus into the editor. The native graph view and an empty tab stay active in the same conditions, against the design's "Échap … restent ceux d'Obsidian". The QA notes `navigation: false` as the only difference it saw. **Not yet filed as an issue.**
- minor — `plugin/src/view/lophelia-view.ts:18-20` — the view uses Obsidian's undocumented `empty-state`/`empty-state-container` classes, which `docs/design-system.md` does not list. Filed as #44.
- minor — `.github/workflows/release.yml:3-5` — a `v*` tag on any commit, even one not on `main`, publishes an attested release. Filed as #43.
- minor — `plugin/package-lock.json` (dependencies lens) — `esbuild` has `"hasInstallScript": true`, and nothing records that the install script was read. Filed as #45.
- minor — `scripts/dev.sh:27-28` — the vault path is written into `obsidian.json` without JSON escaping. Filed as #46.

## Not verified
- Criteria 1 and 2 (a tag creates an attested release; `gh attestation verify`) can only be proven at the first `v0.1.0` tag after landing. A person should push the tag on `main` and run `gh attestation verify <file> --repo atoll-labs-io/lophelia` on `main.js`, `manifest.json` and `styles.css`. The release workflow itself has never run.
- `cargo audit` fails on any advisory, not only high or critical, which is stricter than criterion 7. I did not check whether a low advisory would block a release. Today's CI run passed.
- I did not reproduce `build-wasm.sh` refusing a mismatched `wasm-bindgen`; that rests on the record.
- I did not open the screen myself; I relied on the QA captures of tree e0ccf2b.
- The QA report's `## Not seen`, with its reasons:
  - C1, C2: no screen effect; only visible after a release.
  - C3, C4, C6, C7: repository and CI checks with no screen effect. They were covered here by the green CI run.
  - C5 installed from a GitHub release: no release exists yet.
  - C5 WASM origin of the text: the screen cannot tell it from a constant; only the simulated failure shows it indirectly.
  - "Plus d'options" menu opened with a real Enter key: the CDP Enter did not open it, so the walk cannot say. This is an Obsidian control.
  - Right side panel: the narrow width was only seen in a split screen.
  - System-forced themes outside Obsidian: not reachable from the origin.
- QA finding 3 has a cause the QA calls unproven (the `navigation` flag). A person should confirm the cause before fixing it.
- Synchronous `WebAssembly.Module` compilation on Electron's main thread was not assessed for the future engine size.

Max severity: major
Ship allowed: yes
