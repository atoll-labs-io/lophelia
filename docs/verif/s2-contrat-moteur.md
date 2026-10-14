# Verification — s2-contrat-moteur

[Story](../stories.md) · [Plan](../plans/s2-contrat-moteur.md)

The link line under the title is part of this template: it is copied with it, never edited after the copy, and is not one of the implementer's lines.

The implementer writes this file before its final runs, then edits only `Tree:`, the `## Checks` table, "Not proven here" and the status line. `po-run` writes the `## Runs` section, one row per launch; no one else touches it. The gate compares the tree of each run with `Tree:` outside `docs/`: a declared check counts only through its runs on that code, and the last of them must have exited 0. Commands are copied exactly from `AGENTS.local.md`.

Order that produces `Tree:` — `git add -A`, then `git write-tree`. The hash therefore names the exact code the checks ran on. This file and the plan's ticks live inside `docs/`, which the comparison leaves out.

Tree: 8d49a5b6ab034a5f6da53840bdbc91aae46950e9

## Checks
| Check | Command | Note |
| --- | --- | --- |
| Unit suite | `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'` | |
| Type check | `npm --prefix plugin run typecheck` | |
| End to end | — | no such command in this project |
| Build | — | deferred to ship (`Build stage: ship-if-route`) |

A row with a command is a check you launch through `po-run`. A check not run at this stage, or one the project does not have, gets `—` as its command and says why in Note ("no such command in this project", "deferred to ship").

## Not proven here
- The JS error path (`JsError` raised as `code: détail`) cannot run natively; it is for the QA stage (`Surface: endpoint`), through the built `plugin/wasm-pkg/lophelia_wasm_bg.wasm`.
- The new CI step "Engine core has no web dependency" was exercised locally only (its `cargo tree` + `grep` body: exit 0 on `lophelia-core`, exit 1 naming `wasm-bindgen` on `lophelia-wasm`); its run in GitHub Actions is seen on the pull request.
- `cargo fmt --check` and `cargo clippy --workspace --all-targets --locked -- -D warnings` were run by hand (both clean), not through `po-run`; CI reruns them.
- Production build and bundle size: at ship. `scripts/build-wasm.sh` was run once during task 4 to check the generated typings.

Verification status: complete

## Runs
Written by `po-run`, one row per launch. Never edit, reorder or delete a row.

| Check | Command | Exit | Tree | Time (UTC) |
| --- | --- | --- | --- | --- |
| Unit suite | `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'` | 0 | 81848380b7e8dee7bfd5e931d03214130235e37d | 2026-10-14T20:45:02Z |
| Type check | `npm --prefix plugin run typecheck` | 0 | 23d21c4cc68eae305f212f7b4f14dc5efceb0939 | 2026-10-14T20:45:05Z |
