---
adr: 0001
title: Moteur de disposition Rust/WASM écrit de zéro, plugin Obsidian en TypeScript
status: accepted
date: 2026-10-14
scope: framing
supersedes: []
superseded-by: []
---

# ADR-0001 — Moteur de disposition Rust/WASM écrit de zéro, plugin Obsidian en TypeScript

## Context and Problem Statement
Lophelia doit afficher le vault `~/dev` en 3D avec des groupes de liens lisibles et une forme qui ne bouge que là où le vault change (`docs/brief.md`). Aucun outil existant ne tient cette promesse : chez Graph+ et New 3D Graph, la simulation reprend sur tout le graphe à chaque modification, et leurs builds ne sont pas vérifiables. Il faut choisir comment construire le moteur et son premier hôte, avec un wedge utilisable au plus tard le 31/12/2026.

## Considered Options
- **A. Plugin Obsidian (TypeScript) + moteur de disposition Rust/WASM écrit de zéro, rendu three.js** — ~8-10 semaines ; deux langages et une chaîne WASM à sécuriser en CI.
- **B. Tout TypeScript, moteur maison sans Obsidian ni DOM** — ~5-7 semaines ; itérations plus rapides sur l'algorithme, mais performance au-delà de 2 500 nœuds et portage ultérieur si Rust devient nécessaire. Recommandée pendant l'interview.
- **C. Disposition déterministe tirée des dossiers (arbre 3D figé)** — ~2-3 semaines, sans risque technique, mais perd les groupes formés par les liens : le graphe n'est plus vivant.
- **D. Rester en 2D, positions persistées (reprise de Persistent Graph)** — très faible effort, mais perd la 3D et dépend d'API internes fragiles d'Obsidian.
- **E. Étendre Graph+ ou New 3D Graph** — instabilité inscrite dans leur architecture, blobs WASM commités ou build local sans CI, aucun mainteneur n'ayant jamais accepté de contribution extérieure.

## Decision Outcome
Chosen option: **A**, because Mathieu veut un moteur qui tienne la montée en charge bien au-delà de 2 500 notes (issue #3) et qui se partage avec d'autres hôtes, y compris natifs (Nautile, sites web, mobile). Rien n'est repris de New 3D Graph, qui ne sert que de référence de lecture.

### Consequences
- Good, because le moteur ne connaît ni Obsidian ni le DOM et compile vers WASM comme vers du natif : un second hôte (la page web de démonstration prévue juste après le wedge) le réutilise sans portage.
- Bad, because l'effort (~8-10 semaines) tient juste dans le budget au 31/12/2026, et la chaîne Rust/WASM (build en CI avec attestation de provenance, aucun blob commité) doit être prouvée d'abord : le spike 6 du brief est prioritaire.
- Neutral: si le spike 6 échoue ou si le point de contrôle du 15/11/2026 n'est pas tenu, l'option B est le repli, et une nouvelle ADR remplacera celle-ci.
