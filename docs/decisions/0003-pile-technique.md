---
adr: 0003
title: Pile technique — workspace Rust/WASM, plugin TypeScript esbuild, rendu three.js, npm
status: accepted
date: 2026-10-14
scope: framing
supersedes: []
superseded-by: []
---

# ADR-0003 — Pile technique : workspace Rust/WASM, plugin TypeScript esbuild, rendu three.js, npm

## Context and Problem Statement
L'ADR-0001 retient un moteur de disposition Rust/WASM écrit de zéro et un plugin Obsidian en TypeScript. Il reste à fixer les outils précis : organisation du code Rust, liaison WASM, bundle du plugin, bibliothèque de rendu 3D et gestionnaire de paquets. Les options sont classées selon les critères de Polype : ouvert ou auto-hébergeable, frugal en matériel, scalable, facile à manipuler pour une IA.

## Considered Options
- **Rendu three.js** (retenu) — ouvert, léger, `InstancedMesh` pour des milliers de nœuds, très connu des IA.
- **3d-force-graph** — ouvert, mais embarque sa propre physique d3, inutile puisque la disposition vient du moteur Rust.
- **Babylon.js** — ouvert, mais plus lourd pour un besoin simple.
- **WebGL brut ou regl** — le plus frugal, mais coûteux à écrire et à relire.
- **Liaison wasm-bindgen-cli épinglé** (retenu) ou **wasm-pack** — `wasm-pack` est sorti de l'organisation `rustwasm` fermée en 2025, avec un statut incertain ; `wasm-bindgen` continue dans sa nouvelle organisation.
- **Bundle esbuild** (retenu) ou **Vite/Rollup** — esbuild est le standard des plugins Obsidian et produit le `main.js` unique attendu.
- **Paquets npm** (retenu) ou **pnpm** — pnpm est plus frugal, mais l'outillage Obsidian et la CI le supposent moins.

## Decision Outcome
Chosen option:
- moteur : workspace Cargo avec `crates/lophelia-core` (Rust stable fixé par `rust-toolchain.toml`, aucune dépendance web), `crates/lophelia-wasm` (liaisons `wasm-bindgen`) et `crates/lophelia-bench` (CLI) ;
- liaison WASM : `wasm-bindgen-cli` à la version exacte du crate `wasm-bindgen`, puis `wasm-opt` ;
- plugin : TypeScript bundlé par esbuild en un seul `main.js`, WASM embarqué comme octets au build, calcul de disposition dans un Web Worker ;
- rendu : three.js, nœuds en `InstancedMesh` ;
- paquets : npm, lockfile commité, `npm ci` en CI,

because cette pile respecte le format de plugin d'Obsidian, garde le moteur sans dépendance web (prémisse 4) et reste la plus courante pour chaque rôle.

### Consequences
- Good, because chaque outil est ouvert, épinglé et largement documenté ; la page de démo (s11) réutilise le même rendu et le même module WASM.
- Bad, because deux langages et deux chaînes d'outils sont à maintenir, et la version de `wasm-bindgen-cli` doit suivre celle du crate à chaque mise à jour.
- Neutral: passer de npm à pnpm ne touche pas au code ; à reconsidérer si la taille de `node_modules` devient gênante.
