---
adr: 0007
title: Provenance des releases alors que le dépôt est privé
status: accepted
date: 2026-10-14
scope: story s1-chaine-release
supersedes: []
superseded-by: []
---

# ADR-0007 — Provenance des releases alors que le dépôt est privé

## Context and Problem Statement
L'ADR-0004 retient `actions/attest-build-provenance` et une vérification par `gh attestation verify`. Or `atoll-labs-io/lophelia` est privé, sur une organisation au plan GitHub Free (constat du 14/10/2026, `docs/research/s1-chaine-release.md`) : les attestations d'artefact n'y sont disponibles qu'avec GitHub Enterprise Cloud. Le workflow de release de s1 échouerait à l'étape d'attestation, et le critère 2 de s1 ne peut pas être tenu. Une release d'un dépôt privé ne s'installe pas non plus par BRAT sans jeton.

## Considered Options
Classées selon les critères du propriétaire (ouvert ou auto-hébergeable, frugal, scalable, facile pour une IA), puis le reste.
1. **Rendre le dépôt public avant le premier tag `v*`** — conforme au brief (open source, MIT), gratuit, l'ADR-0004 et la story restent tels quels ; coût : le code devient visible avant le wedge, et une publication ne se reprend pas vraiment (forks, clones). Le dépôt ne contient aucune donnée personnelle (le banc local est ignoré par git, s3).
2. **Signature Sigstore sans clé (cosign ou générateur SLSA) dans le dépôt privé** — outils ouverts et gratuits, mais le nom du dépôt et l'identité du workflow partent dans le journal public Rekor, la vérification passe par `cosign`/`slsa-verifier` et non `gh attestation verify` : l'ADR-0004 et le critère 2 sont à réécrire.
3. **Passer l'organisation à GitHub Enterprise Cloud** — garde le dépôt privé et tout le reste intact, mais coût récurrent par siège et service propriétaire : contraire au critère frugal et au premier critère.
4. **Garder le dépôt privé et publier sans attestation jusqu'à son ouverture** — abandonne la promesse de chaîne d'approvisionnement du brief pour la v1 ; la story serait livrée sans son critère 2.

## Decision Outcome
Option 1 retenue par Mathieu le 14/10/2026 : le dépôt est public depuis ce jour, avec des protections posées au même moment par des rulesets GitHub.
- `main` (ruleset `main`) : suppression et force-push refusés, historique linéaire, passage obligatoire par une pull request fusionnée en rebase, sans approbation requise. Le dépôt reste solo au sens de `/po-ship`, et `Ship confirmation: automatic (chosen)` tient.
- Tags `v*` (ruleset `release-tags`) : création permise, suppression, déplacement et réécriture refusés, pour qu'une release attestée ne change jamais de commit.
- Fusions : rebase seul (commit de fusion et squash désactivés) ; alertes Dependabot, signalement privé de vulnérabilités, secret scanning et protection des pushes activés.
L'adresse e-mail des commits était déjà publique par `atoll-labs-io/polype` ; le contenu du dépôt a été relu avant l'ouverture, sans donnée personnelle ni secret.

### Consequences
- Good, because l'option 1 tient toutes les décisions acceptées sans dépense ni outil de plus.
- Bad, because l'option 1 rend le code public dès s1, avant que le produit soit prouvé.
- Neutral: avec l'option 1, la vérification des critères 1 et 2 se fait au premier tag `v*` poussé sur `main` après l'atterrissage de s1, une fois le dépôt public.
