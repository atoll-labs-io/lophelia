# Review — s3-banc-evaluation

[Story](../stories.md) · [Plan](../plans/s3-banc-evaluation.md) · [Verification](../verif/s3-banc-evaluation.md)

Diff reviewed: `git diff origin/main...story/s3-banc-evaluation`, in a context that did not write it. The driver said the local `main` is stale, so the diff is taken against `origin/main`. Findings rated critical, major or minor.

Review status: complete
Fix loop: 0
Reviewed tree: dba9e437c38372d84e1ba0ebff37a5ad0f1a0773

## Ownership
Diff listed with `git diff --name-only origin/main...story/s3-banc-evaluation` (49 paths). Each path was checked against the story's `Owns` line (`Cargo.toml`, `Cargo.lock`, `crates/lophelia-bench/`, `bench/`, `.gitignore`, `.github/`).
- [x] Every path is inside `Owns`. The paths under `docs/` are this story's own evidence (research, plan, verif, qa), which is in scope by construction.
- [x] Nothing is outside `Owns`.
- [x] Nothing is inside another running story's `Owns`. `git worktree list` shows only the root and this worktree.
- [x] The story has an `Owns` line.

## Evidence reused
- [x] `po-gate verif-current s3-banc-evaluation` exited 0, so the record is reused (`Verification mode: record`). It shows the unit suite (`cargo test --workspace --locked && node --test 'scripts/*.test.mjs'`) and the type check (`npm --prefix plugin run typecheck`) passing, the type check on its second run. I did not run them again. From its "Not proven here" section:
  - fmt and clippy were run by hand.
  - The CI step was not run locally.
  - The `main.rs` dispatch was checked by hand.
- [x] `po-gate qa-current s3-banc-evaluation` exited 0. The report is committed (dc37bba), `QA status: complete`, with 4 verified minor findings. Those four are carried to Findings and its `## Not seen` is carried to Not verified.

## Plan
- [x] Tasks 1–10 are all present. One addition the plan does not mention: `check_id` / `UnsafeNodeId` in `vault.rs`. It is a defensive path check and is harmless.
- [x] Run interdicts, one by one:
  - The diff does not touch `crates/lophelia-core`, `crates/lophelia-wasm`, `plugin`, `scripts` or `docs/contracts`.
  - `Cargo.lock` only gains the `lophelia-bench` package.
  - `[profile.release]` is unchanged.
  - `git ls-files bench/local bench/references` is empty.
  - No `~/dev` or `/home/` path is in the bench files.
  - There is no `HashMap`/`HashSet` and no `unwrap()`/`expect(`/`panic!` in `crates/lophelia-bench/src`.
  - No new dependency: `serde`/`serde_json` use the same exact versions and features as `lophelia-core`, so feature unification is unchanged.

## References
- [x] Every core API used exists with the signature used:
  - `Graph::new`, `Graph::layout`, `LayoutInput`, `Settings::default` (`crates/lophelia-core/src/contract/mod.rs:93-143`)
  - `positions::read/write`, `Known`, `DEFAULT_MAX_BYTES` (`positions/mod.rs:12-142`)
  - `ContractError::code` and `PositionsError::code`
- [x] Values and formulas checked:
  - Silhouette (a singleton scores 0; `b` is the minimum mean distance to the other clusters) matches the hand case.
  - The Louvain gain `w − Σtot·k_i/2m`, the degrees with self-loops and the modularity formula are correct.
  - 3b uses the radius from the barycentre of `before`, the centre in `before` (or in `after` for a created node), and the max over the farthest half.
  - The SBM `p_in`/`p_out` follow the plan.
  - Criterion 8 matches PR #60's wording on `origin/main` (`docs/stories.md:63`).

## Rules
- [x] `AGENTS.md` conventions hold:
  - Code, comments and messages are in English; the README is in French.
  - `lophelia-core` gets no new dependency.
  - No blob is committed.
  - Dependency versions are exact.
- [x] No accepted ADR is contradicted.

## Tests
- [x] Floor: `po-gate floor-guard s3-banc-evaluation origin/main` exited 0, so nothing was lowered.
- [ ] Assertions pin down the criteria, with one exception: test (j) cannot fail on the 3a rule (see Findings).
- [x] There are 18 tests, within the `Test budget` of 25.
- [x] Bite:
  1. In `stability.rs:134` I reversed the sort so it picks the *nearest* half. 1 test failed (`far_half_displacement_counts_only_the_farthest_half`, value 25). Undone; `git diff --exit-code` is clean.
  2. In `stability.rs:248` I replaced the `positions.json` round-trip before the 3a layout with `Known::default()`. **0 tests failed** (18/18 pass). Undone; clean.
  3. In `snapshot.rs:65,70` I disabled both `userIgnoreFilters` checks. 2 tests failed (`excluded_and_hidden_notes…`, `a_written_vault_snapshots_back…`). Undone; clean.
- [x] The story makes no existing test useless.

## Neighbours
- [x] The diff changes no existing code outside the new crate. The workspace members and the CI step were read; the CI composite action is used by `ci.yml` and `release.yml`.

## Lenses
Lenses run:
- dependencies: triggered by `Cargo.lock` and `Cargo.toml`. It found no new external crate and no version move.
- security: triggered by the file writing in `vault.rs` and `snapshot.rs`. Ids are refused when they are absolute, contain `..` or a drive prefix. A target inside the repository is refused using the canonical parent. Nothing exploitable came up for a local CLI.
- [x] No lens finding of its own.

## Findings
- major — `crates/lophelia-bench/tests/bench/stability.rs:43` (test j) together with `crates/lophelia-bench/src/stability.rs:248-251`
  - The test named "the engine rereads exactly" cannot fail on the 3a rule.
  - The provisional s2 engine places every node from a hash of its id (`contract/mod.rs` `created_position`). So 3a is 0 whether or not the bench feeds back the positions it read from `positions.json`; bite 2 above showed this.
  - The same holds for a drift computed against the wrong layout. As a result, the 3a measurement that s8 will rely on has no test that bites.
  - Fix in a later cycle: a test that hands `run_stability`'s 3a path a known layout differing from the hash placement, or that pins the round-trip directly.
- minor — `crates/lophelia-bench/src/main.rs:73`
  - `--record` always writes to `bench/references/<stem>.json`, whatever the graph's location.
  - `<graph> --record` on a local snapshot (`bench/local/dev.json`) therefore writes `bench/references/dev.json`. That file is neither ignored nor covered by the CI guard, and it carries the vault folder name and a score derived from `~/dev` (brief pre-mortem, cause 6).
  - Possible fixes: refuse `--record` for graphs under `bench/local/`, or write their references under `bench/local/`.
- minor — `crates/lophelia-bench/src/snapshot.rs:46`
  - Any unreadable subdirectory aborts the whole snapshot with an `io` error, where Obsidian would just skip it.
  - This is possible on a large `~/dev` (for example root-owned volumes). A warning-and-skip would match how non-UTF-8 names are handled.
- minor — `crates/lophelia-bench/src/louvain.rs:152` together with `:90`
  - The doc says "gain ties go to the smallest community". In fact a node keeps its own community on a tie, and only ties between other candidates go to the smallest.
  - The behaviour is sound and deterministic; only the wording is inexact (the plan's task 7 has the same wording).
- minor — `crates/lophelia-bench/src/lib.rs` (`InvalidJson` Display): `invalid_json` errors never name the file at fault (reference, scenario, `app.json`). QA finding 1, verified.
- minor — `crates/lophelia-bench/src/vault.rs` (`InvalidTarget`): when the parent folder is missing, the message is vague ("not a usable target folder"). QA finding 2, verified.
- minor — `crates/lophelia-bench/src/lib.rs` (Display impl): error details are in English inside output that is otherwise French. QA finding 3, verified.
- minor — `crates/lophelia-bench/src/main.rs:31-34`: an unknown option (for example `--recrod`) prints the bare usage without naming the option. QA finding 4, verified.

## Not verified
- **Criterion 5, "ignored by git".**
  - I tried `git check-ignore -v bench/local/x.json`; po-bounds refused it ("git check-ignore is forbidden in mode reviewer"), so I did not observe it myself. I only read the `bench/local/` line in `.gitignore`.
  - The driver reports running it and getting `.gitignore:9:bench/local/	bench/local/x.json`; that is the driver's observation, not mine.
  - QA could not observe it either (qa-mode refusal, from its `## Not seen`).
  - A person can confirm it with that same command in the worktree.
- **Criterion 5, "CI fails if a file under `bench/local/` is tracked".** The step at `.github/actions/build/action.yml:33-41` was read, not run. Its failure path was never seen, by QA (`## Not seen`) or by the driver (CI on PR #63 only shows the passing path). A person can check it with a throwaway PR that force-adds `bench/local/x.json`, or a local `git add -f` in a scratch clone, followed by the step's script.
- **Criterion 4, a non-zero 3b through the CLI.** With the s2 engine every step reads 3a = 0 and 3b = 0 (QA `## Not seen`). 3b on a real displacement is only proven by the hand-built unit test (h). A person should re-check `stability` output once s4 or s8 changes the engine.
- **CI results on PR #63.** I took the driver's word that they passed (landing commit 0769fd4). I did not open the run.
- **Unit suite, type check, fmt and clippy.** I relied on the verification record and its "Not proven here" (fmt and clippy were run by hand, not through `po-run`). I did not re-run them. I only ran `cargo test -p lophelia-bench --test bench` around the bite mutations.
- **Running `snapshot` on a real vault such as `~/dev`.** Not done, because the run interdict forbids it. Link resolution is only an approximation of Obsidian's `resolvedLinks`. Mathieu should look at the first real snapshot's node, edge and community counts and its warnings.
- **Hand-written Louvain beyond the single test (k).** Its modularity on a `vault`→`snapshot` round trip of sbm-1000 (11 communities found against 10 planted, Q 0.742) is in the record, not observed by me.
- **E2E and build.** The project has no E2E command (`E2E: —`). The production build is deferred to ship (`Build stage: ship-if-route`), and the diff changes no routing or manifest the build depends on beyond adding a workspace member.

Max severity: major
Ship allowed: yes
