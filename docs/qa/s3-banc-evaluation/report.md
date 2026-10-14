# QA — s3-banc-evaluation

[Story](../../stories.md) · [Plan](../../plans/s3-banc-evaluation.md)

Scope: the story's surface, what `git diff main...story/s3-banc-evaluation` touches on it, and the acceptance criteria its user can observe through it; nothing else was checked.
Driver: scenarios.md — surface command, platform cli (Platform: web, lib, cli)
Evidence: transcripts
Tool: Bash (cargo run -p lophelia-bench, on a copy of the tracked files in a mktemp -d directory, CARGO_TARGET_DIR in that directory)
Origin: —

## Coverage
| Criterion or item | Reached | Result |
| --- | --- | --- |
| 1. Synthetic graphs 100 / 1 000 / 2 500 nodes, known communities, fixed seed | yes | as expected — `generate` rewrites the three files byte for byte (same sha256), 4 / 10 / 20 communities (`c1-graphes.txt`, `c1-graphes-error.txt`) |
| 2. `cargo run -p lophelia-bench -- <graphe>`: silhouette, time, gap to reference | yes | as expected; error messages: findings 1 and 4 (`c2-notation.txt`, `c2-notation-error.txt`) |
| 3. Naive layout, its silhouette next to the engine's, fixed parameters in code | yes | as expected — naive 0.460019 / 0.399012 / 0.303299, constants in `naive.rs` (`c3-naive.txt`) |
| 4. `stability` subcommand shows 3a and 3b per step and their maxima | yes | as expected on sbm-100 and sbm-2500; error message: finding 1 (`c4-stabilite.txt`, `c4-stabilite-error.txt`); a non-zero 3b not seen |
| 5. Local snapshot under `bench/local/` accepted; ignored by git; CI fails if tracked | partly | the bench scores `bench/local/*.json` as expected; ignoring by git and CI failure not observed (Not seen) (`c5-instantane-local.txt`) |
| 6. `snapshot <dossier-vault>`: Markdown notes, `[[…]]` by note name, `app.json` exclusions, written to `bench/local/` | yes | as expected — generated vault gives back exactly the ids and the 347 edges of sbm-100, none under `node_modules/`; the hand-written vault resolves aliases, headings, embeds, paths and duplicate names, excludes `priv/` and `.cache/`, warns on the regex filter; error message: finding 1 (`c6-snapshot.txt`, `c6-snapshot-error.txt`) |
| 7. `vault <graphe> <dossier>`: one note per node, one link per edge, `node_modules/` declared excluded, outside the repository | yes | as expected — 100 notes + `node_modules/exclue.md`, 348 links = 347 edges + 1, refuses a target inside the repository or not empty; error message: finding 2 (`c7-vault.txt`, `c7-vault-error.txt`) |
| 8. Replaying the same command gives the same figures (reading of PR #60: only the time varies) | yes | as expected — evaluation differs only on the time line; stability, snapshot and vault outputs identical (`c8-rejeu.txt`) |

## Findings
Status rule: **verified** means the steps were replayed and reproduced the defect, and an evidence file holds it; **probable** means it was seen once, or no evidence file could be kept; **unverified** means it is suspected from what the surface showed, not seen happening.

### Finding 1 — `invalid_json` errors never name the file at fault
Severity: minor
Status: verified
Steps to reproduce:
1. In a copy of the repository, write `{"silhouette":"x"}` to `bench/references/sbm-100.json`, then run `cargo run -q -p lophelia-bench -- bench/graphs/sbm-100.json`.
2. Run `cargo run -q -p lophelia-bench -- stability bench/graphs/sbm-100.json <file holding "not json">`.
3. Run `cargo run -q -p lophelia-bench -- snapshot <vault whose .obsidian/app.json holds {"userIgnoreFilters": "priv/"}>`.
Capture: docs/qa/s3-banc-evaluation/c2-notation-error.txt, c4-stabilite-error.txt, c6-snapshot-error.txt
Expected: the message says which file is wrong: the reference, the scenario rather than the graph, or `.obsidian/app.json`.
Observed: `erreur : invalid_json: invalid JSON: <serde detail> at line L column C`, with no file name. With two inputs (graph + scenario), or with a hidden `app.json` or reference file, the user cannot tell which one failed.

### Finding 2 — `vault` into a folder whose parent is missing gives a vague message
Severity: minor
Status: verified
Steps to reproduce:
1. Run `cargo run -q -p lophelia-bench -- vault bench/graphs/sbm-100.json <tmp>/nope/v`, where `<tmp>/nope` does not exist.
Capture: docs/qa/s3-banc-evaluation/c7-vault-error.txt
Expected: a message saying the parent folder does not exist.
Observed: `erreur : invalid_target: <tmp>/nope/v: not a usable target folder` (exit 1).

### Finding 3 — Error details in English inside French output
Severity: minor
Status: verified
Steps to reproduce:
1. Run any failing command, for example `cargo run -q -p lophelia-bench -- stability bench/graphs/sbm-100.json <scenario linking to "zz.md">`.
Capture: docs/qa/s3-banc-evaluation/c4-stabilite-error.txt, c7-vault-error.txt
Expected: output in one language. The results, the usage text and the `erreur :` / `avertissement :` prefixes are in French, like the README.
Observed: the detail after the code is in English, for example `erreur : unknown_node: unknown node "zz.md"` or `erreur : target_not_empty: … is not empty`.

### Finding 4 — An unknown option prints the usage without naming the option
Severity: minor
Status: verified
Steps to reproduce:
1. Run `cargo run -q -p lophelia-bench -- bench/graphs/sbm-100.json --recrod`.
Capture: docs/qa/s3-banc-evaluation/c2-notation-error.txt
Expected: a message naming `--recrod` as unknown, or at least saying the arguments did not match.
Observed: the usage block alone on stderr, exit 2. The same block answers a missing argument and an extra argument (`generate sbm-100`, `c1-graphes-error.txt`).

## Not seen
- Criterion 5, "ignored by git": `git check-ignore` was refused by `po-bounds` (mode qa runs reading verbs only). It was not reached by another route. Only the `bench/local/` line of `.gitignore` exists in the diff, and that was read, not observed working.
- Criterion 5, "the CI fails if a file of `bench/local/` is tracked": showing it needs a git repository to stage a file in, which mode qa of `po-bounds` forbids, and the CI does not run locally. The step in `.github/actions/build/action.yml:33-41` was read, not run.
- Criterion 4, a non-zero 3b: with the current engine (s2), every step of `base.json` reads 3a = 0 and 3b = 0.000000 % on sbm-100 and sbm-2500. The command line cannot bring the engine into a state where far nodes move, so the 3b computation on a real displacement was not seen through the surface.

Tree: 46a3767d844eab05c2f91b020005168bdf19d371
QA status: complete
