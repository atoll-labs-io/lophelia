# Architecture — Lophelia

Written by `/po-architect`, committed on the target branch. Research and planning read it before touching the code. Change it through `/po-architect` again: a decision that changes gets a new ADR superseding the old one.

## Base
- **Name**: empty repository
- **Origin**: none — the skeleton is laid by story s1
- **Licence**: MIT (ADR-0001, brief)
- **Profile**: analysis only — no catalogue profile covers an Obsidian plugin with a Rust/WASM engine
- **Decision**: [ADR-0002](decisions/0002-depot-vide.md)

## Map
Le dépôt était vide quand ce fichier a été écrit : la carte décrit la disposition **prévue** par les stories et les ADR, aucun chemin n'existe encore. La recherche de chaque story la confirme ou la corrige ; une disposition qui change repasse par `/po-architect`.

| Kind | Path | Example |
| --- | --- | --- |
| Moteur (contrat, format, disposition, ajustement) | `crates/lophelia-core/` | `src/contract/`, `src/positions/`, `src/layout/`, `src/adjust/` (s2, s4, s8) |
| Liaisons WASM | `crates/lophelia-wasm/` | `src/` (s1, s2) |
| Banc d'évaluation (notation, `snapshot`, `vault`) | `crates/lophelia-bench/`, `bench/` | `bench/references/`, `bench/scenarios/`, `bench/local/` ignoré par git (s3) |
| Plugin Obsidian | `plugin/src/` | `main.ts`, `view/`, `vault/`, `persistence/`, `sync/`, `stats/`, `settings/` (s1, s5–s10) |
| Rendu 3D sans Obsidian | `web/render/` | three.js, `InstancedMesh` (s5) |
| Page de démo | `demo/` | (s11) |
| Contrat documenté | `docs/contracts/` | `moteur.md` (s2, s9) |
| CI et release | `.github/`, `scripts/` | workflow de release attestée, `scripts/dev.sh` (s1) |

## Capabilities
| Capability | Brick | Alternatives | ADR |
| --- | --- | --- | --- |
| Stack | Workspace Rust/WASM (`wasm-bindgen-cli` épinglé), plugin TypeScript bundlé par esbuild, rendu three.js, npm | 3d-force-graph, Babylon.js, WebGL brut ; Vite/Rollup ; pnpm | [ADR-0003](decisions/0003-pile-technique.md) |
| Stockage | Fichiers JSON dans `.lophelia/` du vault, via `vault.adapter` | `saveData` (écarté : écrit hors de `.lophelia/`) | [ADR-0005](decisions/0005-integration-obsidian.md) |
| Intégration Obsidian | API publique d'Obsidian, desktop seulement | API internes ; application hors Obsidian | [ADR-0005](decisions/0005-integration-obsidian.md) |
| Deployment | Release GitHub construite et attestée en CI | Forgejo/Woodpecker auto-hébergés ; build local | [ADR-0004](decisions/0004-chaine-release.md) |
| QA des écrans | Obsidian sur un vault de test, piloté par Playwright en CDP | `QA stage: —` ; `_electron.launch` ; manuel seul | [ADR-0006](decisions/0006-qa-ecrans-obsidian.md) |
| Database | Sans objet en v1 : les données sont des fichiers du vault | — | — |
| Auth | Sans objet en v1 : plugin local, un seul utilisateur | — | — |
| Authorization | Sans objet en v1 : droits d'Obsidian et du système de fichiers | — | — |
| Multi-tenancy | Sans objet en v1 : un vault, une personne | — | — |
| Files | Sans objet en v1 : pas de stockage objet | — | — |
| Jobs | Sans objet en v1 : le calcul tourne dans un Web Worker du plugin | — | — |
| Email | Sans objet en v1 | — | — |
| Payment | Sans objet en v1 : gratuit et open source | — | — |
| Cache and rate limiting | Sans objet en v1 | — | — |
| Observability | Sans objet en v1 : aucune télémétrie, par principe (brief, Security & abuse) | — | — |
| i18n | Sans objet en v1 : interface en français seulement | — | — |

## Ownership
- `Cargo.toml`, `Cargo.lock` — shared: only a story that reaches the previous owner through its `Depends on` chain changes them
- `crates/lophelia-core/src/lib.rs` — shared: module registration, same rule
- `crates/lophelia-core/src/contract/`, `docs/contracts/` — shared: contract story s2, then only a story that changes the contract on purpose (s9 for the format version 2)
- `plugin/src/main.ts` — shared: plugin registration, same chain rule
- Every other folder of the map — owned by one story at a time, as its `Owns` line says

## Attachment points
- `plugin/src/main.ts` (`onload`) — views, commands, vault event handlers
- `crates/lophelia-core/src/lib.rs` — engine modules and their public API
- `Cargo.toml` (`[workspace] members`) — new crates
- `.github/workflows/` — new CI checks

## Commands
Quoted from `AGENTS.local.md`, never decided here. Every command is `—` until s1 creates them; Mathieu then writes them into `AGENTS.local.md` (expected: `Install` = `npm ci` + `cargo fetch`, `Test` = `cargo test` + the TypeScript tests, `Build`, `Dev` = `scripts/dev.sh`, ADR-0006).

| Setting | Value |
| --- | --- |
| Package manager | — |
| Install | — |
| Test | — |
| Typecheck | — |
| E2E | — |
| Build | — |

## Conventions
The rules live in the « Conventions » section of `AGENTS.local.md`, which every agent loads through `AGENTS.md`. This file does not repeat them.
