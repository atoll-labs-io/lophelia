# Stories review — Lophelia

I reviewed `docs/stories.md` against `docs/brief.md` without having written either. I also read `AGENTS.md`, `AGENTS.local.md` (Platform: `web, lib, cli`) and `docs/decisions/0001-moteur-rust-wasm.md`. Each finding is rated critical, major or minor.

Review status: complete

## Coverage
| Scope item | Source section | Covered by | Covered |
| --- | --- | --- | --- |
| Wedge: `~/dev` shown in 3D, with Obsidian's excluded files left out | the wedge | s5 | yes |
| Wedge 1: link groups you can read, not a uniform ball | the wedge | s4 (s5 shows it) | yes, but the check is weak (see the s4 finding) |
| Wedge 2: the layout does not move between openings and does not jitter | the wedge | s4 (no jitter), s6 (3a, positions read back) | yes |
| Wedge 3: a new or relinked note joins its neighbours, and the adjustment fades with distance | the wedge | s8, s9 | yes |
| Wedge 4: smooth navigation on ~2,500 notes | the wedge | s5 (60 fps on 2,500), s6 (< 2 s) | yes |
| Wedge 5: clicking a node opens the note | the wedge | s5 | yes |
| Wedge: built in CI with provenance attestation, no binary committed | the wedge | s1 | yes |
| P1: spatial memory; the shape moves only where the vault changes | ## Premises | s4, s6, s8, s10 ("Appliquer" is the only global recompute) | yes |
| P2: complements AI by checking the source | ## Premises | s5 (click opens the note) | yes |
| P3: no existing tool can be fixed cheaply | ## Premises | s1, s2, s4 (engine written from scratch, nothing reused) | yes (a stance, see Not verified) |
| P4: the engine can be separated from its host | ## Premises | s2, s11 | yes |
| P5: one first user, on `~/dev`, is enough | ## Premises | all stories; s7 measures it | yes |
| P6: discipline (tests, review, verifiable build) | ## Premises | s1, s3 | yes |

## Exclusions
| Excluded item | Source section | Revived by | Revived |
| --- | --- | --- | --- |
| No web, mobile or other-vault adapter in v1 (a minimal demo page is allowed just after the wedge) | ## Wedge & non-goals | s11 is the allowed demo; see the minor ordering finding | no |
| No editing notes or links from the graph | ## Wedge & non-goals | none | no |
| No AI search or answers, no Nautile source | ## Wedge & non-goals | none | no |
| No files other than Markdown notes | ## Wedge & non-goals | none (s5 shows Markdown notes only) | no |
| Folder highlighting and `graph.json` colours | ## Wedge & non-goals | none (s5 notes keep them out) | no |
| No syncing or sharing between machines or people | ## Wedge & non-goals | none (s10 stores settings in the vault, which the brief allows) | no |

## Checks
- [x] Exclusions: no story revives a non-goal, under any name.
- [x] No technical layer dressed as a story. s2 is a contract that a test can observe (`tests/contract/`, called from the WASM module), and s4, s6, s8 and s11 rely on it.
- [ ] Every criterion can be checked by one test: two criteria in s4 cannot.
- [x] Dependency order: every `Depends on` names an earlier, existing id.
- [x] Complexity: there is no 5, and each 4 (s1, s4, s8) names its risk.
- [x] Ids: s1 to s11 follow `s<number>-<slug>`, are unique and are numbered in file order.
- [x] Owns: every story declares one. Only s11 is free of a dependency with s7, s8, s9 and s10, and its `demo/` is disjoint from all of their entries. No critical overlap.
- [x] Surface: every story has one. `screen` (web), `endpoint` (lib) and `command` (cli) are all admitted.
- [x] Owned records: there are no `Fixes` lines and no ADR supersession.

## Findings
- major — s4 — Criterion 1 is circular: the silhouette has to reach "la référence enregistrée dans `bench/references/` par cette story", and this same story writes that reference (it owns `bench/references/`). Any layout passes, so wedge item 1 ("pas une boule uniforme") is never actually checked. It needs an outside floor: either a fixed silhouette value, or beating a stated baseline such as a naive force layout on the same graphs.
- major — s4 — Criterion 2 checks against "la limite enregistrée dans la PR", a number the implementer picks for themselves. "Progression visible, pas de blocage" does not fit a `command` surface either. A fixed time limit on the 2,500-node graph is needed.
- major — s2, s3 — The `Owns` entries leave out files these stories must edit. s2 adds dependencies to `lophelia-core` (serde or similar, for `positions.json`), which changes `Cargo.lock`. s3 creates the `crates/lophelia-bench/` crate, which changes `Cargo.lock` and, unless s1's workspace uses `members = ["crates/*"]`, the root `Cargo.toml` too. s1 owns both files, but s1 is not s2 or s3. As written, `po-bounds` would stop the implementer.
- minor — s9 — It stores a content fingerprint in `positions.json`, which changes the s2 format, but it does not own `docs/contracts/` (`moteur.md`). A format-version bump may also touch `crates/lophelia-wasm/src/`, which s9 does not own.
- minor — s5 — The engine runs in a worker. That probably needs the plugin's bundler config (for example `plugin/esbuild.config.*`), which no story after s1 owns.
- minor — s7 — The story traces back to success metric 1, not to the wedge or a premise. It can be tied to P5, but it should be either tied explicitly or raised with Mathieu. Also, "jours ouvrés" is not defined (Monday to Friday? public holidays?).
- minor — s11 — It depends only on s6, so it can ship before s8, s9 and s10 finish the wedge. The non-goal places the demo page "juste après le wedge". If that order matters, it should depend on s10.
- minor — s2 — Its notes say s4, s6, s8 and s11 depend on it, but only s3 lists it in `Depends on`. The others reach it only through the chain. The rule asks every story that uses the contract to list it.
- minor — s2 — The brief's security section asks that `positions.json` be checked against "chemins connus". The criteria check finite numbers, size, broken JSON and version, but not that.
- minor — s1 — The brief's security section and pre-mortem item 7 ask for an audit of security advisories on every build. No criterion covers it.
- minor — s5 — "Une note sous `node_modules/` n'apparaît pas" leaves its precondition unstated: is `node_modules/` in the test vault's `userIgnoreFilters`, or is it always excluded? The brief's floor of "jamais sous 30 images/s" is not a criterion either.
- minor — s6 — In criterion 5, "le vault de 2 500 notes" is not defined (synthetic? which fixture?), and no story owns an Obsidian test vault built from it. "Une note absente du fichier placée près de ses voisines" appears only in the notes, with no criterion.
- minor — s5 — A complexity of 3 looks low for three.js rendering, a worker, a 60 fps target and the exclusions spike. A 4 that names the worker/perf risk would fit better.
- minor — s8 — It covers engine adjustment, bench scenarios, plugin sync, persistence and two spikes (fading propagation, `positions.json` noise). The plan should be checked against the limit of about ten tasks. The noise spike could become its own story that depends on s8.

## Not verified
- Premises P2, P3, P5 and P6 are positions rather than features. I matched each to the stories that act on it, but no criterion can prove a stance. A person should confirm that this matching is acceptable.
- Whether s1's Cargo workspace uses a glob for `members`. That decides how much of the s3 `Owns` finding applies, and it is not on the page.
- Whether `userIgnoreFilters` and `vault.adapter` behave the way s5 and s6 assume. These are the brief's own spikes, outside what this review can check.
- Whether the numbers (60 fps, < 1 s, < 2 s, 1 % of the radius) are achievable. I only checked that the criteria state them.
- A note for the lead, outside the stories: with `QA stage: surfaces` and `Dev: —` (every command in `AGENTS.local.md` is `—`), QA of the `screen` stories has no server to start. Those screens also live inside the Obsidian desktop app, which a browser driver may not reach. Expect QA for s1 and s5 to s11 to stop unless the settings or the QA approach change.
- I read no code or git history, as my limits require.

Max severity: major
Stories ready: yes
