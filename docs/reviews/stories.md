# Revue des stories — Lophelia

J'ai relu `docs/stories.md` contre `docs/brief.md` (Entry mode `new`, Platform `web, lib, cli`), dans un contexte qui n'a pas écrit les stories. Chaque constat est classé critical, major ou minor. J'ai aussi lu `AGENTS.md`, `AGENTS.local.md` et `docs/decisions/0001-moteur-rust-wasm.md`, la seule ADR, qui est acceptée.

Review status: complete

## Coverage
| Élément du périmètre | Section source | Couvert par | Couvert |
| --- | --- | --- | --- |
| Wedge : le vault `~/dev` en 3D, exclusions d'Obsidian respectées | the wedge | s5 (critères 1-2 : `userIgnoreFilters`, `node_modules/`, `resolvedLinks`) | yes |
| Wedge 1 : des groupes de liens lisibles, pas une boule uniforme | the wedge | s3 (silhouette, référence naïve), s4 (silhouette ≥ 0,3 et ≥ référence naïve + 0,1, composantes disjointes) | yes |
| Wedge 2 : une disposition immobile d'une ouverture à l'autre, sans tremblement | the wedge | s4 (critère 3, aucun tremblement), s6 (critère 2, écart 0 à la réouverture) | yes |
| Wedge 3 : une note ajoutée ou reliée autrement rejoint ses voisines, avec un ajustement amorti par la distance | the wedge | s8 (critères 1, 3-5 : décroissance par quarts, 3b < 1 % du rayon, ajout et retrait de lien), s9 (renommage, suppression) | yes |
| Wedge 4 : navigation fluide sur ~2 500 notes | the wedge | s5 (critère 5 : calcul hors du fil principal ; critère 6 : 60 im/s, jamais sous 30, sur un vault de 2 500 nœuds) | yes (voir le constat minor sur le zoom) |
| Wedge 5 : un clic sur un nœud ouvre la note | the wedge | s5 (critère 4) | yes |
| Build en CI depuis les sources, avec attestation, aucun binaire commité | the wedge | s1 (critères 1-3, 6-7) | yes |
| Prémisse 1 : la mémoire spatiale, une forme qui ne bouge que là où le vault change | ## Premises | s4, s6, s8, s9, s10 (« Appliquer » est le seul déclencheur d'une réorganisation globale) | yes |
| Prémisse 2 : compléter l'IA, voir l'ensemble et aller vérifier une note | ## Premises | s5 (vue d'ensemble, clic qui ouvre la note) | yes |
| Prémisse 3 : aucun outil existant ne se répare, le moteur est écrit de zéro | ## Premises | s1 (notes : rien n'est repris de New 3D Graph), s2, s4, s8 | yes |
| Prémisse 4 : le moteur est séparable de son hôte | ## Premises | s2 (`lophelia-core` sans dépendance web, vérifié en CI), s5 (`web/render/` sans Obsidian), s11 (second hôte réel) | yes |
| Prémisse 5 : un premier utilisateur suffit (critère de réussite 1) | ## Premises | s7 (compteur local de jours ouvrés) | yes |
| Prémisse 6 : la discipline (tests, revue, build vérifiable), un vault réel, un second client | ## Premises | s1 (attestation, audit, contrôle réseau), s3 (banc, instantané local de `~/dev`), s11 | yes |

## Exclusions
| Élément exclu | Section source | Ravivé par | Ravivé |
| --- | --- | --- | --- |
| Aucun adaptateur web, mobile ou d'un autre vault en v1 ; la page de démo minimale est permise juste après le wedge | ## Wedge & non-goals | none. s11 est la page de démo permise : locale, sans publication, et placée après le wedge. La sous-commande `snapshot` de s3 lit un vault pour le banc mais n'affiche rien : ce n'est pas un hôte | no |
| Pas d'édition de notes ni de liens depuis le graphe | ## Wedge & non-goals | none. Le `vault` de s3 génère un vault de test hors du dépôt, ce n'est pas une édition depuis le graphe | no |
| Pas de recherche ni de réponse par IA, pas de source Nautile | ## Wedge & non-goals | none | no |
| Pas de fichiers autres que les notes Markdown | ## Wedge & non-goals | none (s5 critère 1 et s3 `snapshot` : Markdown seulement) | no |
| Surbrillance de dossier et couleurs de `graph.json` | ## Wedge & non-goals | none (les notes de s5 l'écartent explicitement) | no |
| Pas de synchronisation ni de partage du vault entre machines ou personnes | ## Wedge & non-goals | none. Le compteur de s7 et les réglages de s10 suivent le vault par son propre transport, et aucune story ne construit de synchronisation | no |

## Checks
- [x] Exclusions : aucune story ne ravive un non-objectif du brief, sous aucun nom.
- [x] Aucune couche technique déguisée en story. s2 est une story de contrat qui remplit ses conditions : elle est observable par ses tests et par l'appel depuis le module WASM, et s3, s4, s6, s8 et s11 la listent dans `Depends on` (s9 l'atteint par sa chaîne). s4 n'a pas d'écran, mais elle est vérifiée par la commande du banc (`command`, admise par `cli`).
- [x] Chaque critère peut être vérifié par un test. Voir les constats minor sur s3 (nombre de critères) et s7 (moyenne hebdomadaire).
- [x] Ordre des dépendances : chaque `Depends on` désigne un id qui existe et qui vient plus tôt dans le fichier.
- [x] Complexité : aucun 5. Chaque 4 (s1, s4, s8) nomme son risque principal dans ses notes.
- [x] Ids : `s<number>-<slug>`, uniques, de s1 à s11 dans l'ordre du fichier.
- [x] Owns : chaque story en déclare. La chaîne de dépendances est linéaire de s1 à s10, et s11 part de s9. La seule paire sans dépendance entre elles est s10/s11, et leurs `Owns` sont disjoints (`plugin/src/...` d'un côté, `demo/` de l'autre). Tous les autres recouvrements (`Cargo.lock`, `.github/`, `plugin/src/main.ts`, `plugin/src/persistence/`, `crates/lophelia-core/...`) concernent des stories reliées par leur chaîne.
- [x] Surface : chaque story a sa ligne. `screen` (web), `endpoint` (lib) et `command` (cli) sont toutes admises par `web, lib, cli`.
- [x] Enregistrements possédés : aucune ligne `Fixes`/`Corrige`, et aucune ADR qui en remplace une autre.

## Findings
- minor — s3-banc-evaluation — La story a 8 critères d'acceptation, au-delà de la limite indicative de sept. Les notes l'assument (« accepté (revue 5) ») et le plan doit découper `snapshot` et `vault` en tâches distinctes. Je le consigne sans exiger de changement.
- minor — s5-vue-vault-3d — Le brief (Quality attributes) demande 60 im/s « en rotation et zoom », jamais sous 30. Le critère 6 ne mesure que la rotation. Il faudrait ajouter le zoom (molette) à la mesure du diagnostic.
- minor — s7-compteur-ouvertures — Au critère 3, la « moyenne par semaine » n'a pas de dénominateur défini pour un mois en cours ou pour les semaines incomplètes (semaines écoulées ? jours ouvrés écoulés / 5 ?). Deux implémentations différentes passeraient chacune leur propre test. Le critère de réussite 1 est « 3 jours ouvrés sur 5 en moyenne sur le mois » : il faut dire comment le chiffre affiché se calcule.
- minor — s9-renommage-suppression — Les notes disent « s9 ne modifie pas le moteur », alors que la story possède `crates/lophelia-core/src/positions/` et fait passer `positions.json` en version 2, avec une empreinte de contenu par note. Deux points ne sont pas dits. D'abord, d'où vient l'empreinte : calculée par le plugin, ou passée au moteur par l'API. Ensuite, si elle entre par l'API de s2, `crates/lophelia-core/src/contract/` et `crates/lophelia-core/src/lib.rs` ne sont pas dans `Owns` ; une bibliothèque de hachage côté Rust demanderait aussi `Cargo.lock` et `crates/lophelia-core/Cargo.toml`. Il faut préciser dans les notes où l'empreinte est calculée et comment elle traverse le contrat, ou bien ajouter ces chemins.

## Not verified
- **Les seuils de performance de s5 (60/30 im/s, fil principal bloqué au plus 50 ms), s6 (< 2 s) et s8 (< 1 s).** Ils sont mesurés sur la machine de Mathieu (Arch, Hyprland), pas en CI. Leur faisabilité ne se juge pas sur le texte : il faut les confirmer en QA sur cette machine.
- **La conduite de la QA `screen` pour un plugin Obsidian.** `QA stage: surfaces` envoie s1 et s5 à s10 en QA avec le pilote d'écran web, mais ces écrans vivent dans Obsidian. De plus, `Dev: —` dans `AGENTS.local.md`, et seule s11 fournit sa propre commande de serveur (`npm --prefix demo run dev`). Ce n'est pas une règle que vérifie cette revue. Une personne doit décider comment la QA ouvre Obsidian avant la première story à écran.
- **La taille de s1 et de s8 en tâches de plan.** s1 réunit la release, l'attestation, l'interdiction des blobs, le contrôle réseau, la vue, le verrouillage et l'audit. s8 réunit le moteur, le banc et la synchronisation du plugin, avec 7 critères. Je n'ai pas pu dire si l'une dépasse la dizaine de tâches qui impose un découpage. Le planificateur doit le confirmer.
- **L'existence d'une API Obsidian fiable pour `userIgnoreFilters` (s5).** C'est un spike du brief que la story laisse ouvert. S'il échoue, le critère 1 de s5 change de nature.
- **Le repli de l'ADR-0001.** Les stories suivent l'option A (Rust/WASM). Le repli prévu (option B si le spike 6 échoue) ferait réécrire s1, s2, s4 et s8, et ce cas n'est pas évalué ici.
- **La complétude des `Owns` au-delà du constat sur s9.** Exemples : s5 appelle le module WASM depuis un worker sans posséder `crates/lophelia-wasm/src/`, et s11 pourrait avoir besoin d'un fichier hors de `demo/` (un workspace npm, la CI). J'ai vérifié les recouvrements, pas l'insuffisance des chemins : la recherche de chaque story doit le confirmer.
- **Les prémisses 2, 3 et 6.** Ce sont des positions de principe plutôt que des fonctions. Je les ai reliées aux stories qui portent leurs contraintes, mais ce rapprochement relève du jugement.
- **Le critère de réussite 2** (désigner de mémoire la zone de 5 projets) est un test humain en fin de mois. Aucune story ne le porte, et ce n'est pas un élément de périmètre au sens du contrôle de couverture.

Max severity: minor
Stories ready: yes
