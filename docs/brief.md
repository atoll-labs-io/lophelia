---
product: Lophelia
status: approved
date: 2026-10-14
---
# Brief — Lophelia

`/po-brief` fills this file theme by theme, following the `interrogation` skill. `status` moves from `interviewing` to `approved` once, at `/po-brief`'s commit, when every exit criterion holds and every theme was approved on its own question; the brief is never shown again for reading.

**One-liner.** Lophelia affiche les notes liées d'un vault Obsidian en 3D, en groupes lisibles, avec une forme qui ne bouge que là où le vault change, pour qu'on la mémorise, s'y repère d'un coup d'œil et vérifie ce que répond une IA ; son moteur de disposition stable est fait pour servir ensuite d'autres hôtes que Obsidian.

## Problem & evidence
Le graphe d'Obsidian se recompose à chaque ouverture : impossible de mémoriser où se trouve quoi, donc la promesse « naviguer dans sa pensée » n'est pas tenue.

- **Preuve.** Mathieu le constate sur son vault `~/dev` (revue de Filamental 3D Graph, 14/10/2026). Des amis font le même reproche ; certains ont quitté Obsidian pour HackMD, où le club de robotique ne relit plus ce qu'il produit. Preuve mince : des témoignages, aucun chiffre.
- **Statu quo.** Demander à une IA, qui répond vite mais qu'on croit sur parole.
- **Ce qui manque.** Une forme stable du vault, connue à force de la voir, donne la vue d'ensemble d'un coup d'œil et permet d'aller droit à la note pour vérifier et sourcer la réponse de l'IA. Lophelia complète l'IA ; il ne la remplace pas pour retrouver un fait.

## Consumer & job
- **Premier utilisateur.** Mathieu, seul, sur son vault Obsidian `~/dev` (les notes Markdown, ~1 300 visibles), à n'importe quel moment de la journée.
- **Usage.** Il cherche une information, sait à peu près dans quelle zone elle se trouve, et la forme stable du graphe l'y mène ; ou une IA vient de lui répondre et il va voir la note et ses voisines pour vérifier.
- **Plus tard.** Les amis utilisateurs d'Obsidian, le club de robotique et les visiteurs de ses sites web, par d'autres adaptateurs, une fois le moteur prouvé sur ce premier usage.
- **Hors périmètre.** Les fichiers qui ne sont pas des notes (PDF, documents).

## Status quo & incumbents
Aujourd'hui : demander à une IA, ou le graphe natif d'Obsidian (2D, recomposé à chaque ouverture, n'applique pas les exclusions).

Arborescence et recherche rapide (Ctrl+O) d'Obsidian : stables par construction, elles mènent à une note dont on connaît le rangement. Elles ne montrent ni les liens entre projets, ni les zones denses ou isolées du vault : c'est ce que Lophelia ajoute, et rien de ce qu'elles font déjà n'est refait.

Existant lu dans le code le 14/10/2026 :
- **Graph+** (woodamsclark, MPL-2.0, v2.0.4) : 3D dessinée en Canvas2D ; positions de tous les nœuds enregistrées via `vault.adapter`, mais toute modification relance la simulation entière ; un renommage perd la position ; pas d'exclusions ni de dossiers ; cible 1 500 nœuds ; build local sans CI ; auteur unique.
- **New 3D Graph** (apoo711, MIT, ~24 k téléchargements, v2.8.2) : WebGL et physique Rust/WASM dans un Web Worker ; aucune persistance, les positions sont remises à zéro à chaque modification ; pas d'exclusions ; blobs WASM commités, invérifiables ; aucune PR externe jamais mergée.
- **Persistent Graph, Graph Save, GraphPro** : 2D, positions fragiles ou plugins peu utilisés. **Filamental** : build invérifiable, positions qui ne se relisent pas.

**Pourquoi étendre perd.** Chez tous, la stabilité casse par architecture (la simulation reprend sur tout le graphe), pas par bug. Aucun ne respecte les exclusions, ne connaît les dossiers ni ne publie un build vérifiable, et aucun mainteneur n'a jamais accepté de contribution extérieure. Rien n'en est repris (pré-mortem, cause 5) : le moteur de New 3D Graph ne sert que de référence de lecture, et celui de Lophelia est écrit de zéro.

## Wedge & non-goals
Le vault `~/dev` en 3D, exclusions d'Obsidian respectées, avec :
1. des groupes de liens lisibles, pas une boule uniforme (le défaut constaté sur New 3D Graph le 14/10/2026) ;
2. tant que le vault ne change pas, une disposition parfaitement immobile d'une ouverture à l'autre, sans aucun tremblement ;
3. une note ajoutée ou dont les liens changent rejoint ses voisines ; ses voisins proches s'ajustent et l'ajustement se propage au reste du graphe en s'amortissant avec la distance, de sorte que les zones éloignées gardent une forme reconnaissable ;
4. une navigation fluide sur ~2 500 notes ;
5. un clic sur un nœud qui ouvre la note.

Le build est fait en CI depuis les sources, avec attestation de provenance ; aucun binaire n'est commité.

Non-goals, at least three:
- Aucun adaptateur web, mobile ou autre vault dans la première version : le moteur reste séparable, mais un seul hôte est livré. Juste après le wedge, une page web de démonstration minimale prouve la séparation (pré-mortem, cause 8).
- Pas d'édition de notes ni de liens depuis le graphe.
- Pas de recherche ni de réponse par IA, pas de source Nautile.
- Pas de fichiers autres que les notes Markdown.
- Surbrillance de dossier et couleurs de `graph.json` : juste après le wedge, pas dedans.
- Pas de synchronisation ni de partage du vault entre machines ou personnes : la forme voyage avec le vault, par le transport de l'utilisateur (éventuelle fonction SaaS plus tard).

## Success metric
Après un mois d'usage sur `~/dev`, Lophelia marche si :
1. Mathieu l'ouvre au moins 3 jours ouvrés sur 5, en moyenne sur le mois — compteur local d'ouvertures, rien n'est envoyé ailleurs ;
2. il désigne de mémoire la zone de 5 projets tirés au hasard, sans chercher — test chronométré de 2 minutes, à la fin du mois ;
3. deux mesures, par un test automatique :
   - a. vault inchangé entre deux ouvertures → écart de position exactement 0 pour tous les nœuds ;
   - b. après une modification de liens, la moitié des nœuds les plus éloignés dans l'espace de la note modifiée bougent de moins de 1 % du rayon du graphe (seuil par défaut, réglable et affiné à l'usage).

Le critère 2 reste le juge final : il vérifie que la propagation amortie n'a pas déformé la forme.

**Échec** : au bout de deux semaines, il est revenu au graphe natif ou ne l'ouvre plus.

## Premises
1. La valeur vient de la mémoire spatiale : une forme qu'on retient parce qu'elle ne bouge que lorsque le vault change, et alors surtout là où il a changé. Le graphe reste vivant (les groupes suivent les liens), mais une disposition qui s'agite seule ou une boule sans groupes ne vaut rien.
2. Lophelia complète l'IA, il ne la remplace pas : il sert à voir l'ensemble et à vérifier ou sourcer une réponse, pas à retrouver un fait.
3. Aucun outil existant ne se répare à moindre coût : chez tous, l'instabilité vient de l'architecture (essai et lecture du code du 14/10/2026).
4. Le moteur de disposition stable est séparable de son hôte et c'est lui qui garde sa valeur à deux ans (Nautile, sites web) ; le plugin Obsidian n'est que le premier hôte.
5. Un premier utilisateur suffit pour prouver le produit : Mathieu, sur `~/dev`.
6. L'avantage de Lophelia n'est pas d'être construit par des agents, mais la discipline qui les encadre (tests, revue, build vérifiable), un vault réel de ~2 500 notes et un second client prévu (Nautile).

## Gap map
Status is one of `answered`, `defaulted`, `open risk`, `N/A`. A dimension marked ★ in the skill never stays blank.

| Dimension | Status | Evidence, default or risk |
| --- | --- | --- |
| ★ Problem & evidence | answered | Témoignages de Mathieu et d'amis (abandon d'Obsidian pour HackMD) ; preuve mince, sans chiffre |
| ★ Consumer & job | answered | Mathieu seul sur `~/dev`, plusieurs fois par jour : aller à une zone connue, vérifier une réponse d'IA |
| ★ Incumbents & alternatives | answered | Graph+ et New 3D Graph lus dans le code : la stabilité y casse par architecture ; rien n'en est repris, New 3D Graph sert de référence de lecture ; essai par Mathieu le 14/10/2026 : Graph+ ne démarre pas, New 3D Graph rame, tremble et forme une boule sans groupes |
| ★ Wedge & non-goals | answered | Vault `~/dev` en 3D lisible, immobile si rien ne change, ajustement amorti, fluide, clic ouvre la note ; cinq non-objectifs |
| ★ Success metrics | answered | Ouverture 3 jours ouvrés sur 5 (compteur local), 5 zones désignées de mémoire (test de 2 min), écart nul si le vault est inchangé, moins de 1 % du rayon pour la moitié la plus éloignée des nœuds après une modification (test automatique) |
| Product & distribution | defaulted | Gratuit, open source, licence MIT (réutilisation du moteur maximale) ; v1 installée depuis les releases GitHub attestées ; store Obsidian après le signal de réussite. Réversible tant que personne d'autre n'a contribué |
| ★ Domain model | answered | Nœud = note Markdown non exclue, identifiée par son chemin relatif au vault ; arête = lien résolu (`resolvedLinks`) ; position = (x, y, z) par nœud. Stockage dans le vault, `.lophelia/positions.json` lisible (une ligne par note, trié par chemin), via `vault.adapter` : la forme voyage avec le vault. Création → placée près de ses voisines ; modification des liens → la note rejoint ses nouvelles voisines, ses voisins proches s'ajustent et l'ajustement se propage en s'amortissant avec la distance ; renommage → la position suit ; suppression → retirée ; vault inchangé → rien ne bouge. Forces réglables comme le 2D d'Obsidian (centre, répulsion, force et distance des liens). Renommage Obsidian fermé → risque ouvert. 2 500 notes max pour le wedge, au-delà issue #3 |
| ★ Interface contract | defaulted | Le moteur ne connaît ni Obsidian ni le DOM : il reçoit nœuds (identifiant opaque), arêtes, positions connues et réglages, et rend des positions ; reproductible à graine fixe sur une même plateforme (pour les tests) ; le critère 3a repose sur la relecture des positions, pas sur un recalcul. Un hôte = un adaptateur (fournit nœuds et arêtes, lit et écrit les positions, dessine) ; v1 : le plugin Obsidian seul. Rendu 3D séparable du moteur. `positions.json` versionné : une version récente lit les anciennes, jamais l'inverse en silence ; API du moteur en semver dès un second hôte. Langage du moteur tranché par l'ADR |
| Quality attributes | defaulted | 60 images/s en rotation et zoom sur 2 500 notes (Arch, Hyprland), jamais sous 30 ; graphe affiché en moins de 2 s quand les positions existent (premier calcul plus long, progression visible) ; ajustement visible en moins de 1 s, hors du fil principal ; `positions.json` écrit de façon atomique, jamais corrompu, jamais écrasé en silence s'il est illisible ; disponibilité et coût par opération sans objet (local, hors ligne) |
| ★ Security & abuse | answered | Authentification, autorisation, multi-utilisateur sans objet en v1 (local, un utilisateur, droits d'Obsidian). Le vault contient des données personnelles (emploi, mariage) : aucun appel réseau, pas de télémétrie, compteur d'ouvertures local, rien ne quitte la machine ; la CI échoue si le bundle publié contient une API réseau (`fetch`, `XMLHttpRequest`, `WebSocket`, modules `http`/`https`/`net`, `requestUrl` d'Obsidian), les dépendances sont minimales et chaque ajout est justifié dans sa PR, avec un audit des avis de sécurité à chaque build. Noms et chemins affichés comme texte, jamais interprétés en HTML ; `positions.json` validé à la lecture (nombres finis, taille bornée, chemins connus), un fichier invalide est signalé et mis de côté, jamais exécuté ni écrasé. Chaîne d'approvisionnement : release construite en CI depuis le dépôt, attestation de provenance sur l'artefact publié, dépendances verrouillées et installation stricte, aucun binaire ni blob généré commité, outils de build à version fixe. Supprimer `.lophelia/` efface tout ce que Lophelia a écrit |
| Operations | defaulted | Aucun serveur ; code sur GitHub (`atoll-labs-io/lophelia`), releases GitHub attestées. v1 installée à la main ou via BRAT, store Obsidian après le signal de réussite. Sauvegarde : celle du vault, dont `positions.json` fait partie ; copie de l'ancien fichier gardée avant toute migration de format. Pas de télémétrie ; erreurs dans une notice Obsidian et la console. Une nouvelle version lit les positions des anciennes ; jamais de recalcul global imposé par une mise à jour |
| ★ Evaluation | answered | Banc noté, rejoué à l'identique à chaque changement du moteur. Graphes figés : synthétiques à communautés connues (100, 1 000, 2 500 nœuds), seuls présents dans le dépôt et en CI ; instantané de la structure de `~/dev` généré sur la machine de Mathieu, ignoré par git et rejoué en local uniquement (aucune donnée personnelle, même hachée, dans le dépôt). Lisibilité : score de séparation des communautés en 3D (silhouette), comparé à une référence ; toute dégradation est justifiée. Stabilité : critères 3a et 3b sur une série scriptée de modifications. Performance : images/s et temps d'ouverture sur 2 500 nœuds. Jugement humain : critère 2 |
| Technical unknowns | open risk | Six spikes (point de contrôle au 15/11/2026), listés sous « Open risks & spikes », levés avant ou pendant les premières stories |

## Pre-mortem findings
Outcome is one of `accepted`, `refuted`, `spike`.

| Finding | Outcome | Reasoning |
| --- | --- | --- |
| 1. Le critère « ouvert chaque jour ouvré » ne colle pas à un outil qui sert quand on doute ; seul signal : l'assiduité de l'auteur | accepted | Critère 1 assoupli à 3 jours ouvrés sur 5 en moyenne ; l'échec reste « plus ouvert depuis deux semaines » |
| 2. Sur un graphe petit-monde (cartes pivots), « plus de 3 sauts » est presque vide : 3b ne teste rien ; le spike propagation amortie (1 jour) est sous-dimensionné | accepted | 3b mesuré sur la moitié des nœuds les plus éloignés dans l'espace ; spike porté à 3 jours, ouvert par la mesure du diamètre de `~/dev` avec et sans `_cartes/` |
| 3. `positions.json` réécrit massivement à chaque modification (propagation à tout le graphe) : diffs illisibles, conflits entre machines | spike | Mesurer sur le banc le nombre de lignes changées par modification avant de choisir la parade |
| 4. Le déterminisme au bit près entre machines est inutile (3a se réduit à relire un fichier) et contraint le choix du langage | accepted | Contrat ramené à « reproductible à graine fixe sur une même plateforme » ; spike déterminisme supprimé |
| 5. Reprendre le moteur de New 3D Graph hérite de blobs invérifiables et d'un fork à maintenir seul | accepted | Rien n'est repris : moteur écrit de zéro, New 3D Graph ne sert que de référence de lecture |
| 6. L'instantané de `~/dev` haché sans sel se retourne par dictionnaire, et la forme du graphe identifie des sous-arbres personnels dans un dépôt public | accepted | Banc public limité aux graphes synthétiques ; instantané de `~/dev` local, ignoré par git |
| 7. « Aucun appel réseau » n'est vérifié par rien : une mise à jour de dépendance peut rompre la promesse sans bruit | accepted | La CI refuse un bundle contenant une API réseau ; dépendances minimales justifiées ; audit des avis de sécurité à chaque build |
| 8. La valeur à deux ans (prémisse 4) repose sur un second hôte qui n'existe pas : Nautile hors production et exclu de la v1 | accepted | Second hôte tôt : une page web de démonstration minimale, livrée juste après le wedge, prouve la séparation moteur/hôte |

## Alternatives considered

| Approach | Effort | Risk | What it gives up |
| --- | --- | --- | --- |
| **Retenue — A.** Plugin Obsidian (TypeScript) + moteur de disposition Rust/WASM écrit de zéro, rendu three.js. Choisie pour la montée en charge (issue #3) et le partage du moteur avec d'autres hôtes, y compris natifs | Élevé : ~8-10 semaines, juste dans le budget au 31/12/2026 | Deux langages ; chaîne WASM en CI (spike 6, prioritaire) ; itérations plus lentes sur l'algorithme | Du temps d'expérimentation pendant les spikes |
| B. Tout TypeScript, moteur maison sans Obsidian ni DOM. Recommandée par l'interview, écartée par Mathieu au profit de Rust ; reste le repli si le spike 6 échoue | Moyen : ~5-7 semaines | Performance au-delà de 2 500 nœuds ; portage ultérieur si Rust devient nécessaire | La vitesse native, le partage avec un hôte natif |
| C. Disposition déterministe tirée des dossiers (arbre 3D figé) | Faible : ~2-3 semaines | Aucun risque technique | Les groupes par liens : le graphe n'est plus vivant |
| D. Rester en 2D, positions persistées (reprise de Persistent Graph) | Très faible | API internes d'Obsidian fragiles | La 3D |
| Étendre Graph+ ou New 3D Graph | Faible en apparence | Instabilité par architecture, provenance douteuse (pré-mortem, cause 5) | La maîtrise du moteur et de la chaîne de build |

## Open risks & spikes
- **Budget** : le wedge est utilisable au plus tard le 31/12/2026, sur le temps hors mission de conseil.
- **Point de contrôle** : les spikes « propagation amortie » et « groupes lisibles en 3D » sont bouclés avant le 15/11/2026.
- **Seuil d'arrêt** : si à cette date aucune approche ne donne à la fois des groupes lisibles et une forme stable sur l'instantané de `~/dev`, on arrête ou on réduit l'ambition (p. ex. disposition 2D stable) ; si le wedge n'est pas utilisable au 31/12/2026, on fait le point avant d'y remettre du temps.
- **Spike — propagation amortie** : un algorithme qui garde les groupes lisibles et tient les critères 3a/3b (relaxation locale dont l'énergie décroît avec la distance). Commence par mesurer le diamètre de `~/dev` avec et sans `_cartes/`. 3 jours sur le banc d'évaluation.
- **Spike — bruit de `positions.json`** : mesurer sur le banc combien de lignes changent par modification de liens, puis décider (quantification des positions, écriture des seules lignes changées, détection des conflits de synchro). ½ jour, après le spike propagation amortie.
- **Spike — groupes lisibles en 3D** : pourquoi New 3D Graph forme une boule (forces, répulsion en 3D, échelle) et quel moteur sépare le mieux les communautés sur l'instantané de `~/dev`. 1 jour.
- **Spike — exclusions** : l'API Obsidian fiable pour appliquer `userIgnoreFilters`. ½ jour.
- **Spike — renommage Obsidian fermé** : le détecter (empreinte de contenu, git) pour que la note garde sa place. ½ jour.
- **Spike — Rust/WASM dans un plugin Obsidian** : build en CI avec attestation, sans blob commité, et taille du bundle. ½ jour.
- **Risque — montée en charge** au-delà de 2 500 notes : [issue #3](https://github.com/atoll-labs-io/lophelia/issues/3).
- **Risque — preuve du problème mince** (témoignages, aucun chiffre) : la retirer en interrogeant deux des amis qui ont quitté Obsidian.
