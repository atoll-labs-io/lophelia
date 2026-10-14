---
adr: 0008
title: Lecture et écriture JSON du contrat moteur, et forme de la frontière WASM
status: accepted
date: 2026-10-14
scope: story s2-contrat-moteur
supersedes: []
superseded-by: []
---

# ADR-0008 — Lecture et écriture JSON du contrat moteur, et forme de la frontière WASM

## Context and Problem Statement
s2 fait lire et valider `positions.json` par le moteur et expose l'API de disposition au module WASM. `lophelia-core` n'a aucune dépendance (`crates/lophelia-core/Cargo.toml:8`) et la bibliothèque standard de Rust ne lit pas le JSON. Le brief veut des dépendances minimales, justifiées une à une (Security & abuse) ; le critère 4 de s2 exige qu'écrire puis relire redonne exactement les mêmes flottants. Le `Cargo.toml` du crate WASM n'appartient pas à s2 : tout encodage doit vivre dans `lophelia-core` (`docs/research/s2-contrat-moteur.md`).

## Considered Options
Classées selon les critères du propriétaire (ouvert ou auto-hébergeable, frugal en matériel, scalable, facile pour une IA), puis le reste. Toutes sont ouvertes (MIT ou Apache-2.0).
1. **`serde` + `serde_json` (fonction `float_roundtrip`) dans `lophelia-core`, chaînes JSON à travers la frontière WASM** — bibliothèques de référence de l'écosystème, que toute IA manipule sans hésiter ; relecture exacte des `f64` vérifiée (0 écart sur 200 000 tirages, 59 533 sans `float_roundtrip`) ; sept crates de plus au verrou (`serde`, `serde_core`, `serde_derive`, `serde_json`, `itoa`, `memchr`, `zmij`), aucun web ; le crate WASM ne fait que relayer des chaînes. Coût : une sérialisation JSON par appel (quelques millisecondes pour 2 500 nœuds), quelques dizaines de Ko de WASM en plus.
2. **`serde_json` dans le cœur pour le fichier, tableaux typés (`Float64Array`, `Uint32Array`) à travers la frontière** — plus frugal à l'appel pour de très grands graphes ; mais deux formes du même contrat à documenter et à tenir, plus de code de glue côté JS, et le crate WASM cesse d'appeler le contrat « tel quel ». À reconsidérer si la frontière devient le goulot (issue #3, au-delà de 2 500 notes).
3. **Lecteur et écrivain JSON écrits à la main, sans dépendance** — le plus frugal (aucun crate), mais ~300 lignes d'analyse de chaînes (échappements, Unicode, nombres) à écrire, tester et auditer ; l'impression et la relecture exactes des `f64` sont précisément le travail difficile que `serde_json` fait déjà ; un analyseur maison est un risque de sécurité à la lecture d'un fichier venu du vault.
4. **`serde-wasm-bindgen` à la frontière** — objets JS convertis sans passer par du texte ; mais nouvelle dépendance dans le crate WASM (dont le `Cargo.toml` est hors des `Owns` de s2) et `js-sys` en plus ; ne règle pas la lecture du fichier, qui reste à faire.

## Decision Outcome
Chosen option: **2**, retenue par Mathieu le 14/10/2026 (« le plus optimisé sur du gros volume de nœuds »), because le moteur doit tenir bien au-delà de 2 500 notes (issue #3) et que le chemin chaud (disposition, ajustement de s8 en moins de 1 s) ne doit pas payer un encodage texte par appel :
- `serde` et `serde_json` (versions exactes, `float_roundtrip`) dans `lophelia-core`, pour `positions.json` seulement ;
- à la frontière WASM, tableaux typés : positions en `Float64Array` (`&[f64]`, trois valeurs par nœud), arêtes en `Uint32Array` d'index de nœuds ; les identifiants sont passés une fois, à la construction d'un objet moteur, et l'hôte travaille ensuite par index ;
- aucune dépendance nouvelle dans `lophelia-wasm` : `wasm-bindgen` mappe `&[f64]`, `Vec<f64>`, `&[u32]` et `Vec<String>` nativement ; toute conversion vit dans `lophelia-core`.
Option 1 écartée : une sérialisation JSON de tout le graphe à chaque appel. Option 3 : analyseur maison à auditer. Option 4 : dépendance hors `Owns` et sans gain sur le fichier.

### Consequences
- Good, because l'appel de disposition copie des tableaux plats, sans texte ni objet JS par nœud, et le format de fichier reste lisible et relu exactement.
- Bad, because deux formes coexistent : le fichier (chemins) et la frontière (index) ; le contrat documente la correspondance, et l'hôte libère l'objet moteur (`free()`).
- Neutral: `serde_json` n'expose pas de code d'erreur public ; le « nombre non fini » se reconnaît au message `number out of range`, à revérifier à chaque montée de version (un test le garde).
