---
adr: 0005
title: Intégration à Obsidian — API publique, données dans .lophelia/ via vault.adapter
status: accepted
date: 2026-10-14
scope: framing
supersedes: []
superseded-by: []
---

# ADR-0005 — Intégration à Obsidian : API publique, données dans `.lophelia/` via `vault.adapter`

## Context and Problem Statement
Obsidian est la seule intégration tierce de la v1. Les plugins existants cassent sur deux points : les API internes non documentées (Persistent Graph) et l'écriture dans un dossier caché par `vault.create`, qui ne s'y relit pas (Filamental). Le brief exige aussi que supprimer `.lophelia/` efface tout ce que Lophelia a écrit.

## Considered Options
- **API publique d'Obsidian, données dans `.lophelia/` via `vault.adapter`** (retenu).
- **API internes non documentées en général** — écarté : fragile à chaque mise à jour d'Obsidian.
- **Application hors d'Obsidian qui lit le vault** — écarté : pas de clic qui ouvre la note dans Obsidian, contraire au wedge.

## Decision Outcome
Chosen option: **API publique d'Obsidian, données dans `.lophelia/` via `vault.adapter`**, because c'est la seule combinaison qui survit aux mises à jour et tient la promesse de suppression :
- seule l'API publique (`obsidian.d.ts` : `ItemView`, `vault`, `metadataCache.resolvedLinks`, événements `create`/`modify`/`rename`/`delete`) est utilisée ; pas de `saveData`, qui écrit dans `.obsidian/plugins/` ;
- les données de Lophelia vivent dans `.lophelia/` à la racine du vault, lues et écrites par `vault.adapter` ;
- `userIgnoreFilters` n'a pas d'API publique documentée : le spike de s5 tranche entre l'accès non documenté (`vault.getConfig`), isolé derrière une seule fonction, et la lecture de `.obsidian/app.json` par `vault.adapter` ;
- le plugin est desktop seulement (`isDesktopOnly`), avec un `minAppVersion` fixé par s1.

### Consequences
- Good, because une mise à jour d'Obsidian ne casse au pire qu'une fonction isolée (les exclusions), et `.lophelia/` contient tout ce que Lophelia écrit.
- Bad, because l'absence d'API publique pour les exclusions laisse un point fragile, à surveiller à chaque version d'Obsidian.
- Neutral: le mobile est hors v1 (issue #20) ; `isDesktopOnly` sera revu à ce moment-là.
