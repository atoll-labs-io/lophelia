# Review: s2-contrat-moteur

[Story](../stories.md) · [Plan](../plans/s2-contrat-moteur.md)

**No critical finding, so the story can ship.** The five findings are all minor: three come from QA and two are new.

Reviewed tree: 6a8918ab6439d32d1f5560b4d7af581079a356af
Fix loop: 0
Scope: `git diff origin/main...story/s2-contrat-moteur`. The local `main` (fb036cd) is behind `origin/main` (71f8b92), as the driver noted.
Lenses run: Dependencies (`Cargo.lock`, and the `[dependencies]` of `crates/lophelia-core/Cargo.toml`: adds `serde`, `serde_json`). Security also ran, because the story parses an untrusted file read from the vault (`positions::read`).

## Scope check (`Owns`)
- Every code path is inside the story's `Owns` line (`docs/stories.md`, s2): `Cargo.lock`, `crates/lophelia-core/{Cargo.toml,src/lib.rs,src/contract/,src/positions/,tests/contract/}`, `crates/lophelia-wasm/src/lib.rs`, `docs/contracts/moteur.md`, and `.github/actions/build/action.yml` under `.github/`.
- `docs/decisions/0008-json-contrat-moteur.md` is a new file (status A) and the plan names it. It is in scope.
- The rest is this story's own pipeline evidence (research, plan, verif, qa).
- Only one other worktree exists (`main`), so no running story's `Owns` is touched.
- The plan's run interdicts hold:
  - Nothing changed in the root `Cargo.toml`, `crates/lophelia-wasm/Cargo.toml`, `plugin/`, `scripts/` or `.github/workflows/`.
  - `serde` appears only in `src/positions/`.
  - There is no `unwrap`, `expect` or `panic!` in `crates/lophelia-core/src`.

## What was relied on
- **`po-gate verif-current s2-contrat-moteur`: exit 0.** Under `Verification mode: record`, I relied on the record for the Unit suite (`cargo test --workspace --locked && node --test 'scripts/*.test.mjs'`) and the Type check (`npm --prefix plugin run typecheck`). Both exited 0 on code equal to `Tree:` outside `docs/`. I did not rerun them.
- **The record's "Not proven here" list:**
  - the JS error path is left to QA;
  - the CI step was only run locally;
  - fmt and clippy were run by hand, not through `po-run`. The driver reports CI green on PR #53 (c6dd380), which covers them.
- **`po-gate qa-current s2-contrat-moteur`: exit 0.** QA tree 65c63ca… is committed in df50495, with `QA status: complete`. Its three verified findings are carried over below.
- **`po-gate floor-guard s2-contrat-moteur origin/main`: exit 0.** Nothing was lowered.
- **End to end:** the project has no `E2E` command.
- **Build:** `npm --prefix plugin run build` is deferred to ship (`Build stage: ship-if-route`). No routing or manifest changed, apart from the Rust crate manifest.

## Steps
- **2. References.** `Graph`, `Settings`, `LayoutInput`, `positions::{read, write, DEFAULT_MAX_BYTES}` and `JsError` are all real.
  - `as_chunks` and `is_multiple_of` are stable on the pinned toolchain 1.99.0 (`rust-toolchain.toml`).
  - `plugin/src/engine/wasm.ts:3` still imports only `engine_version` and `initSync`, and both are unchanged.
- **3. Plan.** Tasks 1–6 are all present and match the plan: dependency versions and features, the error codes, the read order (size → header → version dispatch → strict v1 → paths before filtering), the exact write format, the WASM exports and the CI step.
  - The code follows ADR-0008, option 2 (typed arrays at the boundary, JSON for the file only).
  - It follows ADR-0003: the core has no web crate.
  - The Reuse table holds: FNV-1a and splitmix64 are written out because `DefaultHasher` is not stable across Rust releases, and nothing else does it in-tree.
  - `docs/contracts/moteur.md` lists all 12 error codes.
- **4. Tests.** There are 23 tests (8 layout, 12 positions, 1 chain, 2 wasm), within the `Test budget` of 25. None of them falls in the doctrine's six "prove nothing" kinds: fixtures go through the public API. Bite results, each change undone and confirmed with `git diff --exit-code <file>`, final `git status --porcelain` empty:

| Rule disabled | File | Tests failed |
| --- | --- | --- |
| `..` segment check in `check_path` | `src/positions/mod.rs` | 1 (`malformed_paths_refuse_the_file`) |
| Known node keeps its exact position (replaced by a created position) | `src/contract/mod.rs` | 3 |
| Duplicate index in `known_nodes` → `DuplicateNode` (last one wins instead) | `src/contract/mod.rs` | **0**, see finding 4 |
| `float_roundtrip` feature removed (the plan's pivot) | `crates/lophelia-core/Cargo.toml` | 1 (`write_then_read_gives_back_every_position_bit_for_bit`) |
| Size check before parsing | `src/positions/mod.rs` | 1 (`file_above_the_limit_is_refused_before_parsing`) |

- **5. Values.**
  - The 10 MiB bound matches the story note ("10 Mo") and the contract doc.
  - The `number out of range` message prefix is confirmed by the research probe on the pinned `serde_json` 1.0.154, and test 3(c) guards it.
  - The default forces are marked as provisional in both the code and the doc.
- **6. Neighbours.** The plugin and `scripts/build-wasm.sh` are unaffected; the exports are additive only.
- **Security lens.**
  - The size is checked before any parsing.
  - `serde_json`'s recursion limit holds: QA saw 100 000-deep nesting refused without a crash.
  - A path is validated before the stale-path filter.
  - There is no panic path on input.
  - Nothing exploitable found.

## Findings

### 1. `layout` can return infinite coordinates from finite settings (from QA, verified)
- **Severity:** minor
- **Where:** `crates/lophelia-core/src/contract/mod.rs`, `let half = input.settings.link_distance * (n as f64).cbrt().max(1.0);`
- **What happens:** with `link_distance = 1e308` and 8 nodes, `half` overflows. `layout` returns `Ok` with every value infinite, and `write` then refuses that output with `non_finite_number`.
- **Fix:** check that `half` is finite, or that the output is.
- **Evidence:** `docs/qa/s2-contrat-moteur/c1-overflow-error.txt`

### 2. An integer `version` outside u64 is reported as `missing_version` (from QA, verified)
- **Severity:** minor
- **Where:** `positions/mod.rs`, `header.version.and_then(|v| v.as_u64())`
- **What happens:** `-1` and `99999999999999999999` are refused as "no integer version field". The file is still refused; only the code names the wrong cause.
- **Evidence:** `c5-error.txt`

### 3. A JS `seed` outside 0..2³²−1 or not an integer is silently coerced (from QA, verified)
- **Severity:** minor
- **Where:** `crates/lophelia-wasm/src/lib.rs`, `layout(..., seed: u32, ...)`
- **What happens:** seed `2**32` gives exactly the positions of seed 0. The contract doc says `u32` but does not warn the host that wasm-bindgen does not check this.
- **Evidence:** `c7.txt`

### 4. The duplicate-index refusal in `known_nodes` is tested by no test
- **Severity:** minor
- **Where:** `contract/mod.rs`, the `Some(Some(_)) => return Err(DuplicateNode…)` arm of `Graph::layout`
- **What happens:** disabling that arm leaves all 21 contract tests green. The plan (task 2) and `docs/contracts/moteur.md` ("index deux fois dans `known_nodes`") both list this refusal, but test (e) covers only `Graph::new`. QA checked the behaviour by hand (`c1-error.txt:29-30`), so it works today, but a regression would go unseen.

### 5. Dependencies lens: no record that the build scripts of the new crates were read
- **Severity:** minor
- **What's missing:** `serde`, `serde_core` and `serde_json` ship build scripts. The research, ADR-0008, the verif record and PR #53 justify the dependencies, but none says the build scripts were read.
- **What's fine:** these are new dependencies, not upgrades, so no changelog applies. The 7 lockfile entries are exactly the ones the plan announced, with exact versions.

## Not verified
- **The real GitHub Actions run of the new "Engine core has no web dependency" step.** I read the step and rely on the driver's statement that CI on PR #53 passed. A person should open the PR's checks and confirm that this step ran and is green.
- **The generated `plugin/wasm-pkg/lophelia_wasm.d.ts` typings** (`Uint32Array`/`Float64Array`, `constructor(ids: string[])`). This file is not committed, and I did not rebuild the WASM. QA saw that it matches the doc (`c7.txt`).
- **The JS error path (`code: detail`).** I saw it only through the QA transcripts (`c7.txt`), not by running it myself.
- **From QA `## Not seen`:**
  - Criterion 3 was run only on x86_64 Linux native and on wasm32 under Node, and the contract promises nothing across platforms.
  - The "vérifié en CI" part of criterion 1 is not seen remotely; it is the same point as the first item above.
  - Setting a refused file aside is the host's job and is not part of this story.
- **The production build and end to end** are left to ship; the project has no E2E command.

Review status: complete

Max severity: minor
Ship allowed: yes
