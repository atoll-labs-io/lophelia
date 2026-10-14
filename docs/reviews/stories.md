# Stories review — Lophelia

Stories reviewed: `docs/stories.md`, against `docs/brief.md`, in a context that did not write them. Findings rated critical, major or minor.

Review status: complete

## Coverage
| Scope item | Source section | Covered by | Covered |
| --- | --- | --- | --- |
| Le vault `~/dev` en 3D, exclusions d'Obsidian respectées (capacité d'ouverture) | the wedge | s5-vue-vault-3d | yes |
| 1. Groupes de liens lisibles, pas une boule uniforme | the wedge | s3-banc-evaluation, s4-disposition-lisible | yes |
| 2. Disposition parfaitement immobile tant que le vault ne change pas, sans tremblement | the wedge | s4-disposition-lisible (pas sans déplacement), s6-forme-persistante (critère 3a) | yes |
| 3. Note ajoutée ou reliée autrement : rejoint ses voisines, ajustement amorti avec la distance | the wedge | s8-ajustement-amorti, s9-renommage-suppression | yes |
| 4. Navigation fluide sur ~2 500 notes | the wedge | s5-vue-vault-3d (60 images/s, jamais sous 30) | yes |
| 5. Un clic sur un nœud ouvre la note | the wedge | s5-vue-vault-3d | yes |
| Build en CI depuis les sources, attestation de provenance, aucun binaire commité | the wedge | s1-chaine-release | yes |
| P1. Valeur par la mémoire spatiale : la forme ne bouge que quand le vault change, surtout là où il change | ## Premises | s4, s6, s8, s10 (« Appliquer » seul déclenche une réorganisation globale) | yes |
| P2. Lophelia complète l'IA : voir l'ensemble, vérifier ou sourcer une réponse | ## Premises | s5 (clic → note, survol) | yes |
| P3. Aucun outil existant ne se répare : rien n'est repris | ## Premises | s1, s4 (moteur écrit de zéro, rien de New 3D Graph) | yes |
| P4. Moteur séparable de son hôte | ## Premises | s2-contrat-moteur (core sans dépendance web, vérifié en CI), s11-demo-web | yes |
| P5. Un premier utilisateur suffit : Mathieu sur `~/dev` | ## Premises | s7-compteur-ouvertures (mesure du critère de réussite 1), s3 (instantané local de `~/dev`) | yes |
| P6. Discipline (tests, revue, build vérifiable), vault réel ~2 500 notes, second client | ## Premises | s1, s3, s11 | yes |

## Exclusions
| Excluded item | Source section | Revived by | Revived |
| --- | --- | --- | --- |
| Aucun adaptateur web, mobile ou autre vault en v1 (page de démo minimale permise juste après le wedge) | ## Wedge & non-goals | none: s11 est la page de démo minimale que le non-objectif autorise. Elle reste locale, sans publication, et passe après le wedge (s11 dépend de s9) | no |
| Pas d'édition de notes ni de liens depuis le graphe | ## Wedge & non-goals | none | no |
| Pas de recherche ni de réponse par IA, pas de source Nautile | ## Wedge & non-goals | none | no |
| Pas de fichiers autres que les notes Markdown | ## Wedge & non-goals | none (s3 et s5 se limitent aux notes Markdown) | no |
| Surbrillance de dossier et couleurs de `graph.json` | ## Wedge & non-goals | none (s5 les écarte explicitement) | no |
| Pas de synchronisation ni de partage du vault | ## Wedge & non-goals | none (s9 détecte un `git mv` local, rien n'est synchronisé) | no |

## Checks
- [x] Exclusions : aucune story ne fait revenir un non-objectif, sous aucun nom
- [x] Aucune couche technique déguisée en story. s2 remplit les conditions d'une story contrat : elle est observable par `tests/contract/` et l'appel WASM, et s3, s4, s6, s8 et s11 la listent dans `Depends on`
- [ ] Chaque critère est vérifiable par un test (voir s2, s4 et s5 ci-dessous)
- [x] Ordre des dépendances : aucune dépendance vers une story ultérieure ou inconnue
- [x] Complexité : aucun 5. Les trois 4 (s1, s4, s8) nomment leur risque
- [x] Ids : `s<number>-<slug>`, uniques, numérotés dans l'ordre du fichier
- [x] Owns : toutes les stories en déclarent un. La chaîne de dépendances est linéaire (s1 → … → s9, puis s10 et s11), donc la seule paire sans dépendance est s10/s11. Leurs entrées (`plugin/src/...` contre `demo/`) sont disjointes
- [x] Surface : chaque story en déclare une, admise par `web, lib, cli` (`screen` : web, `endpoint` : lib, `command` : cli)
- [x] Enregistrements possédés : aucune ligne `Fixes` / `Corrige`, aucune ADR qui en remplace une autre

## Findings
- major — s2-contrat-moteur — Le critère 2 dit : « Un nœud dont la position est connue et qui n'est touché par aucun changement ressort à la position exacte d'entrée ». Mais le contrat ne reçoit que des nœuds, des arêtes, des positions connues et des réglages. Il n'a ni ensemble d'arêtes précédent ni liste de nœuds modifiés, donc rien dans ces entrées ne définit « touché par un changement ». Aucun test ne peut fixer ce que le critère attend. La lacune s'étend à s8, qui doit savoir quelle note a changé de liens pour amortir l'ajustement autour d'elle. Si le contrat doit porter un delta, s2 doit le dire dès maintenant : s8 ne possède ni `crates/lophelia-core/src/contract/` ni `docs/contracts/`.
- major — s8-ajustement-amorti — Deux critères se mesurent « sur la série scriptée du banc » : le critère 2 (déplacement moyen par quart de distance) et le critère 5 (silhouette après la série). Le banc de s3 n'affiche que 3a et 3b dans sa sous-commande de stabilité. s8 ne possède ni `crates/lophelia-bench/` ni `crates/lophelia-core/Cargo.toml` ni `Cargo.lock`. Il ne peut donc ni ajouter ces mesures au banc, ni faire dépendre ses tests du calcul de silhouette du banc. Le critère ne peut pas être livré dans son `Owns`.
- minor — s6-forme-persistante — Le critère 6 place une note absente de `positions.json` près de ses voisines, mais s6 ne possède que du code du plugin. Ce placement sera donc écrit en TypeScript dans l'hôte, ce qui déplace de la logique de disposition hors du moteur (prémisse 4), ou il exige du code moteur que s6 ne possède pas. Les notes ne disent pas où vit ce placement, ni s'il préfigure l'ajustement de s8.
- minor — s4-disposition-lisible — « Deux composantes non reliées ne se superposent pas » ne nomme pas de mesure : boîtes englobantes disjointes, distance minimale entre nœuds ou enveloppes convexes. Le test devra en inventer une.
- minor — s5-vue-vault-3d — Le critère 5 ne dit pas ce qu'est une « progression visible » (barre, pourcentage, texte). Le reste du critère (50 ms au plus) est mesurable.
- minor — s3-banc-evaluation — 8 critères, au-delà des sept environ que permet la règle. Les sous-commandes `snapshot` et `vault` pourraient former une story à part, qui dépendrait du banc noté.

## Not verified
- **Prémisses P2, P3 et P5.** Elles sont trop générales pour être reliées à une story par ses critères. Je les ai comptées couvertes sur la foi de s5 (clic → note), de s1/s4 (rien n'est repris de New 3D Graph) et de s7 (mesure du critère de réussite 1). Une personne doit confirmer que c'est bien ce que ces prémisses demandent.
- **Taille de s1 et de s8 au regard de la limite d'environ dix tâches de plan.** s1 réunit une chaîne WASM, l'attestation, deux garde-fous de CI, l'audit et une vue Obsidian. s8 réunit l'algorithme moteur, la synchro du plugin, la persistance et des scénarios du banc. Je ne peux pas estimer le nombre de tâches sans plan : le planificateur doit découper s'il dépasse la limite.
- **Mesures sur la machine de Mathieu.** s5 (60 images/s sur Arch, Hyprland) et s6 (moins de 2 s) ne peuvent pas tourner en CI. Leur reproductibilité dépend de la commande de diagnostic, et non de ce que dit le fichier des stories.
- **Construction du module WASM par la démo (s11).** s11 ne possède que `demo/` et interdit de modifier le moteur et le rendu. Je ne peux pas dire si la cible web du module WASM se construit sans toucher `crates/lophelia-wasm/` ou `.github/`.
- **Couverture des appels moteur depuis WASM pour s6, s8 et s9.** Aucune de ces stories ne possède `crates/lophelia-wasm/src/` sauf s9. Je suppose que l'appel unique exposé par s2 suffit ; je ne l'ai pas vérifié.
- **Routage QA, hors du périmètre de cette revue.** `QA stage: surfaces` envoie toutes les stories `screen` (s1, s5 à s11) en QA. Or `Dev:` vaut `—` dans `AGENTS.local.md`, et ces écrans vivent dans Obsidian (Electron), pas dans une page servie. La QA de ces stories devra trouver comment piloter Obsidian. C'est une question de réglages et non de stories ; à porter à `/po-setup` ou `/po-architect`.

Max severity: major
Stories ready: yes
