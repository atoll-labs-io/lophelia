---
adr: 0002
title: Base — dépôt vide assumé, s1 pose le squelette
status: accepted
date: 2026-10-14
scope: framing
supersedes: []
superseded-by: []
---

# ADR-0002 — Base : dépôt vide assumé, s1 pose le squelette

## Context and Problem Statement
`/po-architect` ne trouve aucune base dans le dépôt : ni manifeste, ni code source. Il faut choisir d'où partent les stories. Aucun profil du catalogue Polype (next-forge, rust-cli, expo…) ne couvre un plugin Obsidian adossé à un moteur Rust/WASM.

## Considered Options
- **Dépôt vide assumé** — aucun boilerplate ni générateur ; la story s1 pose le squelette (workspace Cargo, crates, plugin, CI attestée).
- **Défaut généré** (`obsidian-sample-plugin` + `cargo new --lib`) — ouvert et léger aussi, mais l'exemple officiel apporte un bundle et des conventions que s1 refera de toute façon, et ajoute un aller-retour avant s1.
- **Boilerplate du catalogue** (next-forge, rust-cli) — hors sujet pour un plugin Obsidian.

## Decision Outcome
Chosen option: **dépôt vide assumé**, because le squelette lui-même est la preuve que s1 doit apporter (ADR-0001, spike 6 du brief : chaîne Rust/WASM → plugin → release attestée).

### Consequences
- Good, because aucune convention étrangère n'est héritée ; chaque fichier du squelette est écrit pour cette chaîne.
- Bad, because rien ne peut tourner en parallèle avant s1, et les commandes du projet (`Install`, `Test`, `Build`…) restent à `—` jusqu'à ce que s1 les crée.
- Neutral: l'avertissement de Polype sur le parallélisme (authentification, multi-tenant, facturation en stories de fondation) est sans objet ici ; la chaîne s1 → s10 est déjà linéaire.
