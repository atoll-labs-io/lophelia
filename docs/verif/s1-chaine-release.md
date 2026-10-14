# Verification — s1-chaine-release

[Story](../stories.md) · [Plan](../plans/s1-chaine-release.md)

The link line under the title is part of this template: it is copied with it, never edited after the copy, and is not one of the implementer's lines.

The implementer writes this file before its final runs, then edits only `Tree:`, the `## Checks` table, "Not proven here" and the status line. `po-run` writes the `## Runs` section, one row per launch; no one else touches it. The gate compares the tree of each run with `Tree:` outside `docs/`: a declared check counts only through its runs on that code, and the last of them must have exited 0. Commands are copied exactly from `AGENTS.local.md`.

Order that produces `Tree:` — `git add -A`, then `git write-tree`. The hash therefore names the exact code the checks ran on. This file and the plan's ticks live inside `docs/`, which the comparison leaves out.

Tree: 455b3fd8af4033ccf1d2d8d208efa2f662dc018a

## Checks
| Check | Command | Note |
| --- | --- | --- |
| Unit suite | `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'` | `Test` est `—` dans `AGENTS.local.md` ; commande créée par s1 et nommée par le plan (« Test strategy »), que Mathieu reportera dans `AGENTS.local.md` |
| Type check | `npm --prefix plugin run typecheck` | `Typecheck` est `—` dans `AGENTS.local.md` ; commande nommée par le plan ; lit les `.d.ts` de `plugin/wasm-pkg/`, produits par `scripts/build-wasm.sh` lancé avant |
| End to end | — | `E2E` est `—` : pas de commande dans ce projet |
| Build | — | `Build` est `—` ; le build (`npm --prefix plugin run build`) tourne en CI sur la PR |

## Spike de l'ADR-0006 (tâche 10)
- Port de débogage : **accepté** par Obsidian 1.14.4 (Electron 43.7.9) lancé avec `--user-data-dir` et `--remote-debugging-port=9222` ; `curl -s http://127.0.0.1:9222/json/version` répond (`obsidian/1.14.4`, `Protocol-Version 1.3`). Le repli de l'ADR-0006 n'est pas nécessaire.
- Vault non enregistré : ouvert au démarrage en l'inscrivant (`open: true`) dans `obsidian.json` du profil isolé seulement.
- Mode restreint : **bloque** l'activation du plugin dans un profil neuf (`app.plugins.isEnabled()` faux, plugin listé dans `community-plugins.json` mais non chargé). `scripts/dev.sh` le lève par CDP dans ce profil seul (`app.plugins.setEnable(true)`, puis `enablePluginAndSave("lophelia")`) ; vérifié sur un profil effacé : plugin chargé, commande présente.
- `~/.config/obsidian/` inchangé (empreinte de `obsidian.json` et listing identiques avant et après).

## Vérifications visuelles (tâche 5)
Dans le vault de test, par CDP : la commande « Ouvrir Lophelia » ouvre la vue, une seconde exécution révèle la même (une seule feuille `lophelia`). Le corps est `div.empty-state > div.empty-state-container`, centré ; texte `Moteur 0.1.0` (= `Cargo.toml`), couleur `rgb(179,179,179)` sur `rgb(28,28,28)` en sombre, `rgb(92,92,92)` sur blanc en clair. Échec simulé (`WebAssembly.Module` qui lève, plugin rechargé) : `Moteur non chargé` en `rgb(231,32,56)` (#e72038 corrigé) et une `Notice` « Lophelia : le moteur n'a pas pu être chargé. module invalide (simulé) ».

## Not proven here
- Critères 1 et 2 (release publiée, `gh attestation verify`) : seulement au premier tag `v*` après l'atterrissage (plan, « Done when »).
- CI verte sur la PR (critères 3, 4, 6, 7 en CI) : sur la PR. Localement : `node scripts/check-network.mjs plugin/main.js` à 0, `node scripts/check-no-blobs.mjs` à 0 sur l'index, `cargo fmt --check`, `cargo clippy … -D warnings`, `cargo audit` et `npm audit --audit-level=high` à 0, contrôle tag/version du workflow de release exercé à la main (v0.1.0 accepté, v0.2.0 refusé).
- Largeur étroite (panneau latéral) et thème clair natif d'Obsidian (le clair a été vu en basculant la classe `theme-light`) : à la QA.
- Formatage TypeScript : aucun outil dans le projet.
- Correctif après l'échec CI de la PR #47 (Node 24 refuse `node --test scripts/`) : la forme `node --test 'scripts/*.test.mjs'` a été lancée localement sous le binaire officiel Node 24.21.0 (= `plugin/.nvmrc`, empreinte SHA-256 vérifiée), 11 tests passent ; l'ancienne forme y échoue comme en CI. La suite enregistrée ci-dessous tourne sous le Node 26 local ; la preuve sous 24.21.0 sur le runner reste la CI de la PR.

Verification status: complete

## Runs
Written by `po-run`, one row per launch. Never edit, reorder or delete a row.

| Check | Command | Exit | Tree | Time (UTC) |
| --- | --- | --- | --- | --- |
| Unit suite | `cargo test --workspace --locked && node --test scripts/` | 0 | 311f7effc9633797c38aae3fa7d9850c6b44955f | 2026-10-14T14:49:15Z |
| Type check | `npm --prefix plugin run typecheck` | 0 | 2ad9acd6768f472f9568b5970f3d3fb42519419a | 2026-10-14T14:49:19Z |
| Unit suite | `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'` | 0 | 51e9d75d26e333c172de5d970c31528d357cf80c | 2026-10-14T15:08:23Z |
| Type check | `npm --prefix plugin run typecheck` | 0 | d76c400fe945978c2dd836376564fce5822c50a5 | 2026-10-14T15:08:27Z |
