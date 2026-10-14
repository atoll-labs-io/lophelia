# QA — s2-contrat-moteur

[Story](../../stories.md) · [Plan](../../plans/s2-contrat-moteur.md)

Scope: the story's surface, what `git diff main...story/s2-contrat-moteur` touches on it, and the acceptance criteria its user can observe through it; nothing else was checked.
Driver: scenarios.md — surface endpoint, platform lib (Platform: web, lib, cli)
Evidence: transcripts
Tool: Bash. Scratch Rust crate depending on `lophelia-core` by path, built with cargo 1.99.0 `--offline` from a copy of the tracked files in a `mktemp -d` folder. WASM module built for wasm32 and bound by wasm-bindgen 0.2.129 `--target nodejs`, driven by node v26.11.1.
Origin: —

## Coverage
| Criterion or item | Reached | Result |
| --- | --- | --- |
| 1. API: nodes, edges, known positions, changed, settings in; one (x, y, z) per node out; node without known position treated as created; no Obsidian/DOM dependency | yes | as expected (c1.txt, c1-error.txt, c1-no-web-dependency.txt); finding 1 |
| 2. `changed` empty and every node known: each node comes out at its exact input position | yes | as expected, bit-identical including -0.0, -5e-324, f64::MAX (c2.txt, c2-error.txt) |
| 3. Fixed seed and identical inputs: two calls give the same positions | yes | as expected; other seed differs; same position per id when ids are reordered (c3.txt); finding 3 for the JS seed |
| 4. `positions.json`: version field, one line per node, sorted by path; write then read gives the same positions | yes | as expected: UTF-8 byte order `Z.md` < `a..b.md` < `a.md` < `b/c.md` < `say "hi".md` < `é.md`, bit-identical round trip, byte-identical rewrite (c4.txt, c4-error.txt) |
| 5. Invalid file (non-finite number, over the size limit, broken JSON, unknown future version) refused with a named error, no panic; reader dispatches on the version | yes | as expected: `non_finite_number`, `too_large` (checked before parsing, exact limit 10 485 760 accepted, +1 refused), `invalid_json`, `unsupported_version`; 100 000-deep nesting refused without a crash (c5.txt, c5-error.txt); finding 2 |
| 6. Malformed path (absolute, `..`, empty) refuses the file with a named error; a path matching no node is ignored on read and removed on the next write | yes | as expected: `malformed_path` for `""`, `/…`, `\\…`, `C:`, `..` segments with `/` or `\`; `a..b.md` and `d/..x/n.md` accepted; `gone.md` ignored on read and absent from the next write (c6.txt, c6-error.txt) |
| 7. Contract described in `docs/contracts/moteur.md` and called as-is from the WASM module | yes | as expected: generated `.d.ts` matches the documented table; same results from JS as from Rust; every refusal throws `Error` with `code: detail` (c7.txt); finding 3 |

## Findings
Status rule: **verified** means the steps were replayed and reproduced the defect, and an evidence file holds it; **probable** means it was seen once, or no evidence file could be kept; **unverified** means it is suspected from what the surface showed, not seen happening.

### Finding 1 — `layout` returns infinite coordinates from finite settings without an error
Severity: minor
Status: verified
Steps to reproduce:
1. Build a `Graph` of 8 nodes (`n0.md` … `n7.md`).
2. Call `Settings::from_parts(0, &[0.05, 30.0, 1.0, 1e308])`: accepted, every value is finite.
3. Call `graph.layout` with no edges, no known positions, no changed nodes.
4. Pass the result to `positions::write`.
Capture: docs/qa/s2-contrat-moteur/c1-overflow-error.txt (script scenarios-overflow.rs)
Expected: the contract gives "une position (x, y, z) par nœud" or a named refusal (`non_finite` covers settings that are not finite). A host should never get back positions it cannot write.
Observed: `layout` returns `Ok` with 24 non-finite values out of 24 (`[-inf, inf, -inf, …]`), because `half = link_distance × ∛n` overflows. `write_positions` then refuses that output with `non_finite_number`. With 2 nodes, or with 1e307, the output stays finite.

### Finding 2 — an integer version outside u64 is named `missing_version`, not `unsupported_version`
Severity: minor
Status: verified
Steps to reproduce:
1. Build a `Graph` of `["a.md"]`.
2. Call `positions::read` on `{"version":-1,"positions":[]}`, then on `{"version":99999999999999999999,"positions":[]}`.
Capture: docs/qa/s2-contrat-moteur/c5-error.txt
Expected: `docs/contracts/moteur.md` says `missing_version` is for a field "absent ou non entier", and "tout autre numéro → `unsupported_version`". Both files hold an integer version this reader does not know.
Observed: both are refused, without a panic, as `missing_version: no integer "version" field`. The file is refused either way, but the code and message name the wrong cause: the field is present and is an integer. `version: 1.0` and `version: "1"` also give `missing_version`, which matches the doc's "non entier".

### Finding 3 — the JS `seed` outside 0..2³²−1, or not an integer, is accepted silently
Severity: minor
Status: verified
Steps to reproduce:
1. In Node, load the module bound with `wasm-bindgen --target nodejs`, then `new Engine(["b.md","a.md","c/d.md"])`.
2. Call `engine.layout(new Uint32Array([]), new Uint32Array([]), new Float64Array([]), new Uint32Array([]), 2**32, default_forces())`, then the same with seed `0`, `-1` and `1.5`.
Capture: docs/qa/s2-contrat-moteur/c7.txt
Expected: the contract says `seed` is "entier `u32` (un nombre JS entier de 0 à 2³² − 1, sans perte)", and an input outside the contract is refused with a named error.
Observed: every value returns positions with no error. Seed `2**32` gives exactly the positions of seed `0`, so two different seeds the host passes collide without any warning. `-1` and `1.5` are also coerced silently.

## Not seen
- Criterion 3, "sur la même plateforme": only x86_64 Linux native and wasm32 under Node on this machine were run. Agreement or difference across other platforms is not promised by the contract and was not observed.
- Criterion 1, "vérifié en CI": the CI step was read in `.github/actions/build/action.yml`, and its `cargo tree` listing was run locally with 0 web crates (c1-no-web-dependency.txt). A run on GitHub Actions is a remote service the driver did not start.
- Criterion 5, the host setting a refused file aside and reporting it: the contract assigns this to the host (plugin), which this story does not change. It has no observable effect through the engine's surface.

Tree: 65c63ca34d71194f48056f647ef47433712de167
QA status: complete
