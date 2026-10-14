# Revue des stories — Lophelia

Stories relues : `docs/stories.md`, contre `docs/brief.md` (Entry mode `new`, Platform `web, lib, cli`). La relecture a été faite dans un contexte qui n'a pas écrit les stories. Chaque constat est classé critical, major ou minor.

Review status: complete

## Coverage
| Élément du périmètre | Section source | Couvert par | Couvert |
| --- | --- | --- | --- |
| Wedge : le vault `~/dev` en 3D, exclusions d'Obsidian respectées | the wedge | s5 (critères 1-2 : `userIgnoreFilters`, `node_modules/`, `resolvedLinks`) | yes |
| Wedge 1 : des groupes de liens lisibles, pas une boule | the wedge | s3 (silhouette, référence naïve), s4 (silhouette ≥ 0,3 et ≥ naïve + 0,1) | yes |
| Wedge 2 : disposition immobile d'une ouverture à l'autre, sans tremblement | the wedge | s4 (critère 3, pas de tremblement), s6 (critère 3a, écart 0 à la réouverture) | yes |
| Wedge 3 : une note ajoutée ou reliée autrement rejoint ses voisines, ajustement amorti avec la distance | the wedge | s8 (décroissance par quarts, 3b < 1 % du rayon, ajout ou retrait de lien), s9 (renommage et suppression) | yes |
| Wedge 4 : navigation fluide sur ~2 500 notes | the wedge | s5 (critère 6 : 60 im/s, jamais sous 30, sur le vault de 2 500 nœuds ; critère 5 : calcul hors du fil principal) | yes |
| Wedge 5 : un clic sur un nœud ouvre la note | the wedge | s5 (critère 4) | yes |
| Build en CI depuis les sources, avec attestation, aucun binaire commité | the wedge | s1 (critères 1-3, 6-7) | yes |
| Prémisse 1 : la mémoire spatiale, une forme qui ne bouge que là où le vault change | ## Premises | s4, s6, s8, s9, s10 (« Appliquer » est le seul déclencheur d'une réorganisation globale) | yes |
| Prémisse 2 : compléter l'IA, voir l'ensemble et aller vérifier une note | ## Premises | s5 (clic qui ouvre la note, vue d'ensemble) | yes |
| Prémisse 3 : aucun outil existant ne se répare ; moteur écrit de zéro | ## Premises | s1 (notes : rien n'est repris de New 3D Graph), s2, s4, s8 | yes |
| Prémisse 4 : le moteur est séparable de son hôte | ## Premises | s2 (le crate core compile sans dépendance web), s5 (`web/render/` sans Obsidian), s11 (second hôte réel) | yes |
| Prémisse 5 : un premier utilisateur suffit (mesuré par le critère de réussite 1) | ## Premises | s7 (compteur local de jours ouvrés) | yes |
| Prémisse 6 : la discipline (tests, revue, build vérifiable), un vault réel, un second client | ## Premises | s1 (build attesté, audit, contrôle réseau), s3 (banc, instantané local de `~/dev`), s11 | yes |

## Exclusions
| Élément exclu | Section source | Ravivé par | Ravivé |
| --- | --- | --- | --- |
| Aucun adaptateur web, mobile ou autre vault en v1 (page de démo minimale permise juste après le wedge) | ## Wedge & non-goals | none : s11 est la page de démo permise, locale, sans publication. La sous-commande `snapshot` de s3 lit un vault pour le banc mais n'affiche rien ; ce n'est pas un hôte | no |
| Pas d'édition de notes ni de liens depuis le graphe | ## Wedge & non-goals | none | no |
| Pas de recherche ni de réponse par IA, pas de source Nautile | ## Wedge & non-goals | none | no |
| Pas de fichiers autres que les notes Markdown | ## Wedge & non-goals | none (s5 critère 1 et s3 `snapshot` : Markdown seulement) | no |
| Surbrillance de dossier et couleurs de `graph.json` | ## Wedge & non-goals | none (les notes de s5 l'écartent explicitement) | no |
| Pas de synchronisation ni de partage du vault entre machines ou personnes | ## Wedge & non-goals | none (les notes de s7 disent seulement que le compteur suit le vault ; aucune story ne construit de synchronisation) | no |

## Checks
- [x] Exclusions : aucune story ne ravive un élément exclu, sous aucun nom (un non-objectif du brief)
- [x] Aucune couche technique déguisée en story. s2 est une story de contrat qui remplit ses conditions : elle est observable par ses tests et l'appel WASM, et s3, s4, s6, s8 et s11 en dépendent.
- [x] Chaque critère peut être vérifié par un test (voir les constats minor sur le nombre de critères)
- [x] Ordre des dépendances : aucune dépendance vers un id ultérieur ou inconnu
- [x] Complexité : aucun 5 ; chaque 4 (s1, s4, s8) nomme son risque principal
- [x] Ids : `s<number>-<slug>`, uniques, s1 à s11 dans l'ordre du fichier
- [x] Owns : chaque story en déclare. La chaîne de dépendances est linéaire de s1 à s10, et s11 en part depuis s9. La seule paire sans dépendance entre elles est s10/s11, et leurs `Owns` sont disjoints (`plugin/src/...` contre `demo/`).
- [x] Surface : chaque story a une ligne Surface. `screen` (web), `endpoint` (lib) et `command` (cli) sont toutes admises par `web, lib, cli`.
- [x] Enregistrements possédés : aucune ligne `Fixes`/`Corrige`, et aucune ADR qui en remplace une autre

## Findings
- minor — s3-banc-evaluation — 8 critères d'acceptation, au-delà de la limite d'environ sept. Les sous-commandes `snapshot` et `vault` (critères 6-7) forment un outil distinct de la notation du banc. Elles pourraient faire une story à part, qui dépendrait de s3.
- minor — s9-renommage-suppression — Le critère 2 exige un déplacement amorti après une suppression (la moitié la plus éloignée bouge de moins de 1 % du rayon). Or `Owns` ne contient ni `crates/lophelia-core/src/adjust/`, ni `crates/lophelia-core/src/lib.rs`, ni `crates/lophelia-bench/`. Le critère ne tient que si le plugin peut exprimer une suppression avec l'API de s2/s8 telle qu'elle est (voisins de la note supprimée passés comme « modifiés »). Si le moteur doit changer, la story sort de ses `Owns`. Il faut le dire dans les notes, ou ajouter les chemins.
- minor — s10-reglages-forces — Critère 2 : « voyagent avec le vault » n'est pas une observation en soi. La partie testable est le fichier `.lophelia/settings.json`. Il faudrait formuler ce qui est vu, par exemple : les réglages sont relus après la réouverture d'Obsidian.

## Not verified
- **Les seuils de performance de s5 (60/30 im/s, blocage du fil principal ≤ 50 ms), s6 (< 2 s) et s8 (< 1 s).** Ils sont mesurés sur la machine de Mathieu (Arch, Hyprland), pas en CI. Leur faisabilité n'est pas vérifiable à partir du texte : à confirmer en QA sur cette machine.
- **La taille de s1 et de s8 en tâches de plan.** s1 réunit la release, l'attestation, l'interdiction des blobs, le contrôle réseau, la vue, le verrouillage et l'audit. s8 réunit le moteur, le banc et la synchronisation du plugin. Je n'ai pas pu dire si l'une ou l'autre dépasse la dizaine de tâches qui impose un découpage. Le planificateur doit le confirmer.
- **L'existence d'une API Obsidian fiable pour `userIgnoreFilters` (s5).** C'est un spike du brief que les stories laissent ouvert. Si elle n'existe pas, le critère 1 de s5 change de nature.
- **Le respect de la chaîne Rust/WASM fixée par ADR-0001.** Les stories le respectent sur le papier. Le repli prévu par l'ADR (option B si le spike 6 échoue) ferait réécrire s1, s2, s4 et s8 ; ce cas n'est pas évalué ici.
- **La complétude des `Owns` au-delà du constat sur s9.** Exemples : s5 appelle le module WASM sans posséder `crates/lophelia-wasm/src/`, et s11 pourrait avoir besoin de fichiers à la racine. Je n'ai vérifié que le recouvrement, pas l'insuffisance des chemins : la recherche de chaque story doit le confirmer.
- **Les prémisses 3 et 6.** Ce sont des positions de principe plutôt que des fonctions. Je les ai reliées aux stories qui portent leurs contraintes (CI, banc, écriture de zéro), mais ce rapprochement relève du jugement.
- **Le critère de réussite 2** (désigner 5 zones de mémoire) est un test humain en fin de mois. Aucune story ne le porte, et ce n'est pas un élément de périmètre au sens du contrôle de couverture.

Max severity: minor
Stories ready: yes
