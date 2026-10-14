# Stories review — Lophelia

Stories reviewed: `docs/stories.md`, against `docs/brief.md`, in a context that did not write them. Findings rated critical, major or minor.

Review status: complete

## Coverage
| Scope item | Source section | Covered by | Covered |
| --- | --- | --- | --- |
| Le vault `~/dev` en 3D, exclusions d'Obsidian respectées | the wedge | s5-vue-vault-3d | yes |
| 1. Groupes de liens lisibles, pas une boule | the wedge | s3-banc-evaluation (mesure), s4-disposition-lisible, s5-vue-vault-3d, s10-reglages-forces | yes |
| 2. Disposition immobile d'une ouverture à l'autre, sans tremblement | the wedge | s4-disposition-lisible (crit. 3), s6-forme-persistante | yes |
| 3. Note ajoutée/reliée rejoint ses voisines, ajustement amorti avec la distance | the wedge | s8-ajustement-amorti, s9-renommage-suppression | yes (partiel : la propagation aux voisins proches n'a aucun critère, voir les findings) |
| 4. Navigation fluide sur ~2 500 notes | the wedge | s5-vue-vault-3d (crit. 6), s6-forme-persistante (crit. 5) | yes |
| 5. Un clic sur un nœud ouvre la note | the wedge | s5-vue-vault-3d | yes |
| Build en CI depuis les sources, attestation, aucun binaire commité | the wedge | s1-chaine-release | yes |
| P1. Valeur = mémoire spatiale, forme qui ne bouge que là où le vault change | ## Premises | s4, s6, s8, s9 | yes |
| P2. Complète l'IA : voir l'ensemble, vérifier/sourcer une réponse | ## Premises | s5-vue-vault-3d (clic ouvre la note) | yes |
| P3. Aucun outil existant ne se répare : moteur écrit de zéro | ## Premises | s1-chaine-release, s2-contrat-moteur, s4 (rien repris de New 3D Graph) | yes |
| P4. Moteur séparable de son hôte | ## Premises | s2-contrat-moteur, s11-demo-web | yes |
| P5. Un premier utilisateur suffit : Mathieu sur `~/dev` | ## Premises | s5–s9 (sur son vault), s7-compteur-ouvertures (mesure) | yes |
| P6. Discipline : tests, revue, build vérifiable, vault réel, second client | ## Premises | s1, s3-banc-evaluation, s11-demo-web | yes |

## Exclusions
| Excluded item | Source section | Revived by | Revived |
| --- | --- | --- | --- |
| Aucun adaptateur web, mobile ou autre vault en v1 (page de démo minimale juste après le wedge) | ## Wedge & non-goals | none (s11 est la démo explicitement prévue, en local, sans publication) | no |
| Pas d'édition de notes ni de liens depuis le graphe | ## Wedge & non-goals | none | no |
| Pas de recherche ni de réponse par IA, pas de source Nautile | ## Wedge & non-goals | none | no |
| Pas de fichiers autres que les notes Markdown | ## Wedge & non-goals | none (s5 crit. 1 : notes Markdown seules) | no |
| Surbrillance de dossier et couleurs de `graph.json` | ## Wedge & non-goals | none (s5 les écarte dans ses notes) | no |
| Pas de synchronisation ni de partage entre machines ou personnes | ## Wedge & non-goals | none | no |

## Checks
- [x] Exclusions : aucune story ne fait revenir un non-objectif
- [x] Pas de couche technique déguisée ; s2-contrat-moteur remplit les conditions de contrat (testée, appelée depuis le module WASM, au moins s3, s4, s6, s8 et s11 en dépendent par la chaîne)
- [ ] Chaque critère vérifiable par un test : trois critères à reprendre (voir les findings)
- [x] Ordre des dépendances : toutes les dépendances pointent vers une story antérieure existante
- [x] Complexité : aucun 5 ; les trois 4 (s1, s4, s8) nomment leur risque
- [x] Ids : `s<number>-<slug>`, uniques, numérotés dans l'ordre du fichier
- [x] Owns : chaque story en déclare un ; aucun chevauchement entre stories sans dépendance. s1 à s10 forment une chaîne linéaire ; s11 (`demo/`) n'est reliée ni à s7, ni à s8, ni à s9, ni à s10, et son `demo/` est disjoint de tous leurs chemins
- [x] Surface : toutes déclarées ; `screen` (web), `endpoint` (lib) et `command` (cli) sont admis par `Platform: web, lib, cli`
- [x] Enregistrements possédés : aucune ligne `Corrige`, aucune ADR remplacée

## Findings
- major — s8-ajustement-amorti — L'élément 3 du wedge demande que « ses voisins proches s'ajustent et [que] l'ajustement se propage […] en s'amortissant avec la distance ». Aucun critère ne le vérifie. Le crit. 3 borne seulement le déplacement de la moitié la plus lointaine. Une implémentation qui ne déplace que la note modifiée et fige tout le reste passerait les cinq critères. Il faut un critère qui montre, sur la série scriptée, que les voisins proches bougent et que le déplacement décroît avec la distance.
- major — s8-ajustement-amorti — Crit. 1 : « apparaît […] près des notes qu'elle relie, sans figer l'interface ». Ni « près » ni « sans figer » n'ont de seuil, donc aucun test ne peut trancher. Il faut une mesure, par exemple une distance aux notes liées inférieure à la distance médiane du graphe, et un fil principal jamais bloqué plus de N ms.
- major — s6-forme-persistante — Crit. 5 : « sur le vault de 2 500 notes ». Aucune story ne produit un tel vault : s3 fournit des graphes synthétiques (nœuds et arêtes), pas un vault Obsidian, et `~/dev` compte environ 1 300 notes visibles. Le test n'a pas d'entrée. Il faut nommer la fixture (un vault généré depuis le graphe synthétique de 2 500 nœuds) et dire quelle story la crée.
- major — s9-renommage-suppression — L'empreinte de contenu « stockée avec la position » modifie le format `positions.json`, qui est le contrat de s2, lu par s6 et s11. s9 possède `crates/lophelia-core/src/positions/`, mais ni `docs/contracts/` (où `docs/contracts/moteur.md` décrit le format), ni `crates/lophelia-core/tests/contract/`, ni `crates/lophelia-wasm/src/`. Cela va contre « Owns and contracts first », et la voie s'arrêtera sur des fichiers qu'elle n'a pas le droit de modifier. Deux corrections possibles : prévoir le champ d'empreinte dans s2, ou ajouter ces chemins aux `Owns` de s9 avec une montée de version du format.
- minor — s11-demo-web — Le brief place la démo « juste après le wedge ». Or s11 ne dépend que de s6, donc la flotte peut la construire en parallèle de s7 à s10, avant que le wedge (ajustement amorti, renommage) soit livré. C'est à l'auteur de choisir : soit dépendre de s9 (ou de s10), soit assumer l'ordre et l'écrire dans les notes.
- minor — s7-compteur-ouvertures — La story se rattache au critère de réussite 1, qui n'est ni le wedge ni une prémisse. On peut la rattacher à la prémisse 5, mais la note devrait l'écrire, ou la question devrait être posée à Mathieu. Par ailleurs, « jours ouvrés » n'est pas défini (lundi à vendredi ? jours fériés ?) : il faut le préciser pour que le crit. 2 soit testable.
- minor — s5-vue-vault-3d — Crit. 5 : « l'interface d'Obsidian reste utilisable » n'ajoute rien de vérifiable à « hors du fil principal ». Il faut le reformuler en mesure ou le retirer.
- minor — s5-vue-vault-3d — Crit. 6 (60 images/s) ne nomme pas la machine. Le brief fixe « Arch, Hyprland » et « jamais sous 30 », la story ne reprend ni l'un ni l'autre.
- minor — s4-disposition-lisible — La story possède `Cargo.lock` mais pas `crates/lophelia-core/Cargo.toml`. Si elle ajoute une dépendance au moteur, elle ne pourra pas l'écrire. Il faut ajouter ce fichier ou retirer `Cargo.lock`.
- minor — s1-chaine-release — Le brief (Security & abuse, pré-mortem 7) demande « un audit des avis de sécurité à chaque build ». Aucun critère de s1 ne le porte, ni aucune autre story. Ce n'est pas un élément de couverture au sens strict, mais c'est une promesse du brief laissée sans story.

## Not verified
- Les prémisses 2, 3, 5 et 6 relèvent du positionnement ou de la méthode, plus que de fonctions à livrer. Je les ai rattachées aux stories qui les servent le mieux, mais c'est un jugement : aucune story n'a de critère qui les porte mot pour mot. Une personne devrait confirmer que ce rattachement lui suffit.
- Le critère de réussite 2 (désigner de mémoire 5 zones en 2 minutes) est un test humain qui juge la propagation amortie. Aucune story ne peut le porter, et cette revue ne peut pas dire si s4 et s8 le tiendront.
- L'étendue réelle des `Owns` de s5, s8 et s11 : s5 aura peut-être besoin de la configuration de bundle (esbuild ou équivalent) sous `plugin/`, ou de `crates/lophelia-wasm/src/` pour le Worker ; s8 aura peut-être besoin des liaisons WASM pour l'ajustement ; s11 aura peut-être besoin d'un `package.json` hors de `demo/`, ou de `.github/`. Sans code ni plan, je ne peux pas dire si ces chemins manqueront. La recherche de chaque story doit le vérifier.
- J'ai jugé qu'une vue de plugin Obsidian relève de `screen` (page d'une app web). La table des surfaces ne dit pas explicitement que c'est le cas.
- Hors du périmètre des stories, mais cela bloquera la QA : `AGENTS.local.md` déclare `QA stage: surfaces` et `Dev: —`, alors que huit stories ont la surface `screen` et que, selon les règles, la QA d'une surface `screen` a besoin de `Dev`. Cela se règle dans les réglages, pas dans `docs/stories.md`.
- La revue précédente (`docs/reviews/stories.md`) et le code n'étaient pas dans le périmètre : je n'ai pas vérifié si ses findings ont été traités.

Max severity: major
Stories ready: yes
