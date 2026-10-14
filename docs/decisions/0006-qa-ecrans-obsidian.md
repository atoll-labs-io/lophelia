---
adr: 0006
title: QA des écrans dans Obsidian — vault de test piloté par Playwright en CDP
status: accepted
date: 2026-10-14
scope: framing
supersedes: []
superseded-by: []
---

# ADR-0006 — QA des écrans dans Obsidian : vault de test piloté par Playwright en CDP

## Context and Problem Statement
`QA stage: surfaces` envoie en QA chaque story dont la surface est `screen`, mais les écrans de Lophelia vivent dans Obsidian (Electron), pas dans une page servie, et `Dev` vaut `—`. Sans décision, la QA de s1 et de s5 à s10 s'arrêterait.

## Considered Options
- **Obsidian lancé sur un vault de test, piloté par Playwright connecté en CDP** (retenu).
- **`QA stage: —`** — écarté : perd toute vérification des écrans.
- **`_electron.launch` de Playwright** — écarté : Obsidian est un binaire empaqueté, le lancer ainsi est fragile.
- **Vérification manuelle seule** — écarté : non rejouable.

## Decision Outcome
Chosen option: **Obsidian piloté par Playwright en CDP**, because c'est la seule option qui vérifie l'écran réel, de façon rejouable :
- la QA d'une story `screen` lance Obsidian avec `--remote-debugging-port` sur un vault de test (minimal en s1, généré par le banc dès s3) où le plugin construit est installé, puis s'y connecte par `connectOverCDP` ;
- la commande `Dev` sera un script créé par s1 (`scripts/dev.sh`) : il construit le plugin, prépare le vault de test, lance Obsidian avec le port de débogage et affiche son adresse ; Mathieu écrit la ligne `Dev:` dans `AGENTS.local.md` une fois s1 atterrie, car `/po-architect` ne change pas les réglages ;
- repli, si la version d'Electron d'Obsidian refuse le port de débogage : la QA vérifie le rendu (`web/render/`) dans une page de test servie par `Dev`, et Mathieu vérifie l'intégration dans Obsidian à la main. Le spike se fait dans s1.

### Consequences
- Good, because les écrans sont vérifiés tels que Mathieu les voit, dans le vrai Obsidian.
- Bad, because la QA dépend d'Obsidian installé sur la machine qui la lance, et d'un port de débogage que les futures versions d'Electron pourraient fermer.
- Neutral: la page de démo de s11 sert sa propre commande (`npm --prefix demo run dev`) et n'a pas besoin d'Obsidian.
