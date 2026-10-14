---
adr: 0004
title: Chaîne de release — GitHub Actions, release attestée, contrôles de CI
status: accepted
date: 2026-10-14
scope: framing
supersedes: []
superseded-by: []
---

# ADR-0004 — Chaîne de release : GitHub Actions, release attestée, contrôles de CI

## Context and Problem Statement
Le brief exige une release construite en CI depuis les sources, avec une attestation de provenance sur l'artefact publié et aucun binaire commité : c'est la leçon de Filamental, dont le `main.js` publié était invérifiable. La brique « déploiement » du catalogue Polype (image conteneur) ne s'applique pas : un plugin Obsidian se distribue sous forme de fichiers de release.

## Considered Options
- **GitHub Actions avec `actions/attest-build-provenance`** (retenu) — attestation vérifiable par `gh attestation verify`, au même endroit que le dépôt.
- **Forgejo + Woodpecker auto-hébergés sur crabelab** — plus ouvert, mais sans attestation de provenance reconnue par `gh` ni par l'écosystème Obsidian, et le dépôt vit sur GitHub.
- **Build local publié à la main** — écarté : c'est exactement ce qui rendait Filamental invérifiable.

## Decision Outcome
Chosen option: **GitHub Actions avec release attestée**, because c'est la seule option qui rend la provenance de l'artefact publié vérifiable par n'importe qui :
- un tag `v*` construit le WASM et le plugin depuis les sources et publie `main.js`, `manifest.json` et `styles.css` en release GitHub, chaque artefact attesté ;
- chaque CI refuse tout `.wasm` ou blob base64 versionné, refuse un bundle contenant une API réseau, lance `cargo audit` et `npm audit` (échec sur haut ou critique), installe en mode strict, et épingle les actions par SHA ;
- la v1 s'installe depuis la release (à la main ou via BRAT) ; le store Obsidian vient plus tard (issue #23).

### Consequences
- Good, because la promesse « aucun appel réseau » et la provenance du build sont vérifiées par la machine à chaque changement, pas déclarées.
- Bad, because la chaîne dépend de GitHub Actions ; un passage à une forge auto-hébergée perdrait l'attestation tant qu'elle n'y existe pas.
- Neutral: les SHA des actions épinglées sont à faire avancer à la main.
