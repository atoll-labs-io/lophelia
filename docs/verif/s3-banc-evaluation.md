# Verification — s3-banc-evaluation

[Story](../stories.md) · [Plan](../plans/s3-banc-evaluation.md)

The link line under the title is part of this template: it is copied with it, never edited after the copy, and is not one of the implementer's lines.

The implementer writes this file before its final runs, then edits only `Tree:`, the `## Checks` table, "Not proven here" and the status line. `po-run` writes the `## Runs` section, one row per launch; no one else touches it. The gate compares the tree of each run with `Tree:` outside `docs/`: a declared check counts only through its runs on that code, and the last of them must have exited 0. Commands are copied exactly from `AGENTS.local.md`.

Order that produces `Tree:` — `git add -A`, then `git write-tree`. The hash therefore names the exact code the checks ran on. This file and the plan's ticks live inside `docs/`, which the comparison leaves out.

Tree: 0d4ce8033e47954404fad55aac3190fbe0e85295

## Checks
| Check | Command | Note |
| --- | --- | --- |
| Unit suite | `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'` | |
| Type check | `npm --prefix plugin run typecheck` | |
| End to end | — | no such command in this project |
| Build | — | deferred to ship (`Build stage: ship-if-route`) |

A row with a command is a check you launch through `po-run`. A check not run at this stage, or one the project does not have, gets `—` as its command and says why in Note ("no such command in this project", "deferred to ship").

## Not proven here
- The new CI step "Nothing tracked under bench/local" was not run in GitHub Actions; locally `git ls-files bench/local bench/references` prints nothing and `git status --ignored` shows `bench/local/` as ignored after a `snapshot` run. Its run in CI is seen on the pull request.
- `cargo fmt --check` and `cargo clippy --workspace --all-targets --locked -- -D warnings` were run by hand (both clean), not through `po-run`; CI reruns them.
- The CLI dispatch (`main.rs`) has no test of its own; it was checked by hand: the three graphs, `stability` on `sbm-2500`, `vault` into a `/tmp` folder then `snapshot` of it, `vault` into `bench/x` (`erreur : inside_repository: …`, exit 1) and a malformed call (usage, exit 2). The QA stage (`Surface: command`) walks the commands on the committed tree.
- No snapshot of `~/dev` was taken during the story (run interdict).
- The first type check run failed (exit 1) because the ignored `plugin/wasm-pkg/` had never been generated in this worktree; after `scripts/build-wasm.sh`, the rerun on the same code exited 0.
- Figures from the manual runs (debug profile, `opt-level = 3` for the bench crate only), a landmark for the thresholds of s4:

| Graph | Nodes | Edges | Communities | Engine silhouette | Naive silhouette | Engine time |
| --- | --- | --- | --- | --- | --- | --- |
| sbm-100 | 100 | 347 | 4 | -0.070507 | 0.460019 | 0.109 ms |
| sbm-1000 | 1000 | 3620 | 10 | -0.057257 | 0.399012 | 27.035 ms |
| sbm-2500 | 2500 | 8622 | 20 | -0.063995 | 0.303299 | 1.279 ms |

  The whole `sbm-2500` run (naive layout and two O(n²) silhouettes included) took about 15 s wall clock. `stability bench/graphs/sbm-2500.json bench/scenarios/base.json`: 3a = 0 and 3b = 0.000000 % at all four steps, as expected from the provisional s2 engine, which keeps every known position. `snapshot` of a vault written from `sbm-1000` found 11 communities (10 planted), modularity 0.741985.

Verification status: complete

## Runs
Written by `po-run`, one row per launch. Never edit, reorder or delete a row.

| Check | Command | Exit | Tree | Time (UTC) |
| --- | --- | --- | --- | --- |
| Unit suite | `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'` | 0 | 6a9eac0acb4f0ff60dcc0437fe6258881fdce661 | 2026-10-14T21:25:13Z |
| Type check | `npm --prefix plugin run typecheck` | 1 | 80d868fd1d11558de28c15d53eb29ee8b3c8badd | 2026-10-14T21:25:19Z |
| Type check | `npm --prefix plugin run typecheck` | 0 | 26e94699bf85331f837c68b618a77f64432eb049 | 2026-10-14T21:27:22Z |
