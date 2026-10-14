# lophelia — Polype settings

This file is the project's. Polype's installer never rewrites it; `/po-setup` writes it once and you edit it afterwards. Rules live in `AGENTS.md`, which the installer regenerates — do not edit that one.

Format: one `Name: value` per line inside the blocks below, with no comment after the value (anything after the colon is the value, except a final `(forge)` or `(chosen)` on `Ship confirmation`, which records where the value came from). After a change, rerun the installer so the conventions at the bottom reach `AGENTS.md`.

## Pipeline

```
Merge mode:        pr
Target branch:     main
Plan validation:   autonomous
Ship confirmation: automatic (chosen)
Test budget:       25
Verification mode: record
Full suite:        execute-end
E2E stage:         ship
E2E scope:         nominal
E2E browsers:      —
Build stage:       ship-if-route
Platform:          web, lib, cli
QA stage:          surfaces
Issue tracker:     github
Worktree root:     .worktrees/
ADR directory:     docs/decisions/
Entry mode:        new
Design source:     internal
Design skill:      frontend-design
Design tool:       —
Story track:       auto
Flow threshold:    2
Parallel:          3
Max attempts:      2
Tracker:           none
Model strong:      opus
Model light:       sonnet
Light up to:       2
```

| Setting | Values |
| --- | --- |
| Merge mode | `pr` — a pull request to the target branch, landed as one commit · `local` — only for a repository without a remote: the commit lands on this machine and nothing is pushed |
| Target branch | where finished stories land |
| Plan validation | `human` — you approve each plan · `autonomous` — the planner approves its own and says so; the default, since a human is asked only for the reasons ADR-0077 names |
| Ship confirmation | `human` — you approve each merge · `automatic` — the review gate is enough. A final `(forge)` means the value follows the repository and `/po-ship` keeps it in line at each ship; `(chosen)` is the person's choice on a solo repository, never rewritten by an agent; no suffix reads `(forge)`. A collaborative repository gives `human (forge)` whatever the line says |
| Test budget | maximum tests per story before the plan must justify more |
| Verification mode | `record` — a review reuses a current verification record · `rerun` — a review repeats the checks |
| Full suite | where the complete unit suite runs: `execute-end` · `ship` · `both` |
| E2E stage | where end-to-end tests run: `ship` · `ci` · `—` |
| E2E scope | how much end-to-end runs; `nominal` means one happy path |
| E2E browsers | browsers used before ship, e.g. `chromium`; `—` leaves the project default. Ship always uses all of them |
| Build stage | where the production build runs: `ship-if-route` (only when routing or a manifest changed) · `ship` · `ci` · `—` |
| Platform | what the project builds, one or more of `web` · `mobile` · `cli` · `api` · `lib` · `xr` · `robot`, separated by commas; it decides which boilerplate profiles `/po-setup` offers and which `Surface` a story may declare (ADR-0054, ADR-0055) |
| QA stage | `screens` — /po-qa walks the screens a story touches before its review · `surfaces` — /po-qa checks every story whose `Surface` is not `—`, with the driver of that surface (ADR-0054) · `—` — no QA stage |
| Issue tracker | where defects are filed: `github` · `taskwarrior` · `—` |
| Worktree root | folder that holds story worktrees, ending with `/`; kept out of git by `.gitignore` |
| ADR directory | where the project's ADRs live, with its own numbering; ends with `/` |
| Entry mode | where the project starts: `replace` — build an internal replacement of a paid SaaS · `adapt` — a competing product with its own angle · `new` — an idea from scratch · `—` not chosen yet |
| Design source | who draws the screens: `internal` — screens drawn in the repository by an agent skill · `external` — a tool outside the repository · `—` means design commands stop and ask |
| Design skill | skill that draws the screens when `Design source` is `internal`; `—` otherwise |
| Design tool | tool the screens come from when `Design source` is `external`; `—` otherwise |
| Story track | `full` — every story walks every stage · `flow` — every story takes the short track · `auto` — the short track up to `Flow threshold` |
| Flow threshold | highest story complexity allowed on the short track when `Story track` is `auto` |
| Parallel | how many story lanes the fleet driver runs at once |
| Max attempts | fix runs a story gets before the fleet driver hands it to a human |
| Tracker | where the fleet publishes story state: `none` · `github` when the repository has a remote |
| Model strong | the model every deciding subagent (planner, designer, reviewer, qa, stories-reviewer) runs on: a name the Agent tool accepts, or `—` for the session's model |
| Model light | the model the implementer runs on for a story whose complexity is at most `Light up to`: a name the Agent tool accepts, or `—` for the session's model |
| Light up to | highest story complexity whose implementer runs on `Model light`; `—` means never |

## Commands

```
Package manager:   npm
Install:           npm --prefix plugin ci && cargo fetch --locked
Test:              cargo test --workspace --locked && node --test 'scripts/*.test.mjs'
Typecheck:         npm --prefix plugin run typecheck
E2E:               —
Build:             npm --prefix plugin run build
Dev:               scripts/dev.sh
```

Agents run these exactly as written. `—` means the project has no such command, and no agent will make one up. `Test` has to accept paths to test files, so a single task's tests can run alone. `Dev` starts the development server, or whatever serves what a person looks at (the web render of a mobile app), and prints the address it serves; `—` when the project has nothing to serve. QA of a `screen` surface needs it; the other drivers start what they check themselves.

## Conventions

The repository had no base when these rules were written (ADR-0002): each rule cites the ADR or brief section that backs it instead of a `path:line`.

- Code, identifiers, comments and commit messages are in English; documents under `docs/` are in French (atoll `AGENTS.md`).
- Commit messages follow Conventional Commits (`type: subject`), as the history does.
- `crates/lophelia-core` depends on no web or Obsidian crate (ADR-0003).
- The plugin never computes a position: every layout goes through the engine contract (s2, premise 4 of `docs/brief.md`).
- `web/render/` imports nothing from `obsidian` (ADR-0003, s11).
- Never commit a `.wasm` file or a generated blob (base64 WASM or worker) (ADR-0004).
- No network API in `plugin/`, `web/` or `demo/` (ADR-0004).
- Only Obsidian's public API is used; the single undocumented access, if any, lives in one function under `plugin/src/vault/` (ADR-0005).
- Everything Lophelia writes goes under `.lophelia/` through `vault.adapter`, atomically (temporary file, then rename); never `saveData` nor `vault.create` (ADR-0005).
- Note names and paths are rendered as text (`textContent`), never through `innerHTML` (`docs/brief.md`, Security & abuse).
- Rust passes `cargo fmt --check` and `cargo clippy -- -D warnings`; TypeScript compiles in strict mode.
- Dependency versions are exact; every new dependency is justified in its pull request (`docs/brief.md`, Security & abuse).
