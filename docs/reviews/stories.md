# Stories review — Lophelia

Stories reviewed: `docs/stories.md`, against `docs/brief.md`, in a context that did not write them. Findings rated critical, major or minor.

Review status: complete

## Coverage
| Scope item | Source section | Covered by | Covered |
| --- | --- | --- | --- |
| Capacité d'ouverture : le vault `~/dev` en 3D, exclusions d'Obsidian respectées | the wedge | s5-vue-vault-3d | yes |
| Wedge 1 — groupes de liens lisibles, pas une boule | the wedge | s3-banc-evaluation, s4-disposition-lisible | yes |
| Wedge 2 — disposition immobile d'une ouverture à l'autre, sans tremblement | the wedge | s4-disposition-lisible (aucun tremblement), s6-forme-persistante (écart 0) | yes |
| Wedge 3 — note ajoutée ou reliée autrement rejoint ses voisines, ajustement amorti | the wedge | s8-ajustement-amorti, s9-renommage-suppression | yes |
| Wedge 4 — navigation fluide sur ~2 500 notes | the wedge | s5-vue-vault-3d (60 img/s, jamais < 30) | yes |
| Wedge 5 — un clic ouvre la note | the wedge | s5-vue-vault-3d | yes |
| Build en CI depuis les sources, attestation de provenance, aucun binaire commité | the wedge | s1-chaine-release | yes |
| Prémisse 1 — mémoire spatiale : forme qui ne bouge que là où le vault change, groupes vivants | ## Premises | s4, s6, s8, s9, s10 (« Appliquer » seul déclencheur global) | yes |
| Prémisse 2 — complète l'IA, ne la remplace pas | ## Premises | s5 (aller droit à la note) ; aucune story d'IA | yes |
| Prémisse 3 — aucun outil existant ne se répare à moindre coût | ## Premises | s1, s4 (moteur écrit de zéro, rien de New 3D Graph) | yes |
| Prémisse 4 — moteur séparable de son hôte | ## Premises | s2-contrat-moteur, s5 (`web/render/` sans Obsidian), s11-demo-web | yes |
| Prémisse 5 — un premier utilisateur suffit | ## Premises | s7-compteur-ouvertures (critère de réussite 1) | yes |
| Prémisse 6 — discipline (tests, revue, build vérifiable), vault réel, second client | ## Premises | s1, s3, s11 | yes |

## Exclusions
| Excluded item | Source section | Revived by | Revived |
| --- | --- | --- | --- |
| Aucun adaptateur web, mobile ou autre vault en v1 (sauf la démo minimale juste après le wedge) | ## Wedge & non-goals | none (s11 est la démo permise, locale, sans publication ; voir finding sur son ordre) | no |
| Pas d'édition de notes ni de liens depuis le graphe | ## Wedge & non-goals | none | no |
| Pas de recherche ni de réponse par IA, pas de source Nautile | ## Wedge & non-goals | none | no |
| Pas de fichiers autres que les notes Markdown | ## Wedge & non-goals | none (s5 : une note Markdown par nœud) | no |
| Surbrillance de dossier et couleurs de `graph.json` | ## Wedge & non-goals | none (s5 les déclare hors wedge) | no |
| Pas de synchronisation ni de partage entre machines ou personnes | ## Wedge & non-goals | none | no |

## Checks
- [x] Exclusions: no story revives an excluded item, under any name — a PRD's graveyard feature or a brief's non-goal
- [x] No technical layer dressed as a story; contract stories meet their conditions (s2 : observable par ses tests et l'appel WASM, plus de deux stories en aval)
- [ ] Every criterion can be checked by one test — un critère de s9 ne l'est pas (major), plusieurs imprécis (minor)
- [x] Dependency order: no dependency on a later or unknown id (s1→…→s10 en chaîne, s11→s6)
- [x] Complexity: no 5 left unsplit, every 4 names its risk (s1, s4, s8)
- [x] Ids: `s<number>-<slug>`, unique, numbered in file order
- [x] Owns: every story declares one; no two stories with no dependency between them hold entries that overlap — s1 à s10 forment une seule chaîne ; s11 (`demo/`) n'a de lien ni avec s7 ni avec s8, s9, s10, et `demo/` ne recoupe aucune de leurs entrées
- [x] Surface : chaque story en déclare une ; `screen` (web), `endpoint` (lib) et `command` (cli) sont admis par `Platform: web, lib, cli`

## Findings
- major — s3-banc-evaluation (touche aussi s4, s8) — les notes disent « L'instantané de `~/dev` sera exporté plus tard par le plugin », mais aucune story ne livre cet export. Or le spike de s4 (« quel moteur sépare le mieux sur l'instantané de `~/dev` »), la mesure du diamètre de `~/dev` avec et sans `_cartes/` en ouverture de s8, et le seuil d'arrêt du 15/11/2026 du brief (« sur l'instantané de `~/dev` ») en dépendent tous. Il faut ajouter un critère à une story existante (par exemple s5) ou écrire une story qui produise l'instantané dans `bench/local/`.
- major — s7-compteur-ouvertures — les notes stockent le compteur via `saveData` et le disent « local à la machine, pas une donnée du vault ». Or l'API `saveData` d'Obsidian écrit `data.json` sous `.obsidian/plugins/<id>/`, à l'intérieur du vault. Cela contredit la promesse de sécurité du brief, « Supprimer `.lophelia/` efface tout ce que Lophelia a écrit », et le compteur voyage avec le vault. Il faut soit placer le stockage sous `.lophelia/`, soit corriger la note et dire comment la promesse du brief tient.
- major — s9-renommage-suppression — aucun test ne peut vérifier le critère « ses voisines ne bougent qu'au titre de l'ajustement amorti ». Il faut le reformuler en seuil mesurable, par exemple le critère 3b après une suppression.
- minor — s2-contrat-moteur — les notes disent que s4, s6, s8 et s11 dépendent du contrat. Pourtant, seul s3 le cite dans `Depends on` ; les autres ne l'atteignent que par la chaîne. Or `agentic-stories` demande que chaque story qui utilise le contrat le liste.
- minor — s2-contrat-moteur / s6-forme-persistante — le brief (Security & abuse) exige de valider « chemins connus » dans `positions.json`. Aucun critère ne dit ce que devient une entrée dont le chemin est inconnu du vault ou malformé.
- minor — s11-demo-web — la story dépend de s6 seulement, donc la flotte peut la construire avant s8 et s9, c'est-à-dire avant la fin du wedge. Or le non-objectif la place « juste après le wedge » (pré-mortem, cause 8). Une dépendance à s9 respecterait cet ordre.
- minor — s11-demo-web — dans « la fait afficher à l'identique », la référence de comparaison n'est pas nommée (chaque nœud aux coordonnées du fichier ?). La commande qui lance le serveur n'est pas nommée non plus.
- minor — s1-chaine-release — les notes disent « Licence MIT à ajouter », mais `LICENSE` ne figure pas dans `Owns`.
- minor — s5-vue-vault-3d — la story a 7 critères, dont deux outils hors de la tranche (le générateur de vault de test du banc et la commande de diagnostic d'images/s), en plus du worker, du WASM, de three.js et du spike exclusions. La complexité 3 semble basse, et il faut vérifier que le plan tient en ~10 tâches. Le générateur pourrait passer dans s3. Il faut aussi vérifier que la configuration du bundler pour le worker tombe bien sous les `Owns` déclarés (seuls des fichiers précis de `plugin/` y sont listés).
- minor — s5-vue-vault-3d — le critère « On fait tourner, zoomer et déplacer la caméra à la souris » nomme l'action mais pas ce qu'on voit.
- minor — s8-ajustement-amorti — dans « Après l'ajout ou le retrait d'un lien, la note concernée se rapproche de ses nouvelles voisines », le cas du retrait n'est pas défini : quelles nouvelles voisines, quel résultat attendu ?
- minor — s10-reglages-forces — le fichier de `.lophelia/` qui garde les réglages n'est pas nommé. La story se rattache à la ligne Domain model de la gap map (forces réglables) plus qu'au wedge ; on l'accepte par le wedge 1 et la prémisse 1.

## Not verified
- Prémisses 2 et 3 : ce sont des positions, pas des capacités. Je les ai comptées couvertes par l'absence de toute fonction d'IA et par un moteur écrit de zéro ; aucune story ne peut les « livrer » au sens strict.
- Je n'ai pas pu dire si s8 peut faire passer l'ajustement incrémental par l'API de s2 sans toucher `crates/lophelia-wasm/src/`, qui ne figure pas dans ses `Owns`. Le planner de s8 doit trancher.
- Les seuils (60 img/s, 2 s, 1 s, 50 ms, silhouette 0,3) ne se vérifient que sur la machine de Mathieu ou par les spikes. Je ne juge que leur forme de critère, pas leur faisabilité.
- J'ai lu la vue du plugin Obsidian (Electron) comme un `screen` de la plateforme `web` ; ADR-0054 et ADR-0055 ne sont pas dans ce dépôt pour le confirmer.
- Tous les `Commands` de `AGENTS.local.md` valent `—`, y compris `Dev`. Avec `QA stage: surfaces`, la QA des stories `screen` en aura besoin. C'est un réglage, pas un défaut des stories.
- La revue précédente (`docs/reviews/stories.md`), le code et l'historique git sont hors du périmètre et n'ont pas été lus.

Max severity: major
Stories ready: yes
