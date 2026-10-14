# Stories — Lophelia

Each story is a slice a user can actually use, described precisely enough that an agent builds it without asking. Its id, `s<number>-<short-slug>`, names every file the pipeline writes for it and its branch `story/<id>`.

**Complexity scale** — 1: simple create/read/update/delete · 2: a form, storage and a list · 3: business logic or several states · 4: third-party integration, payments or roles · 5: real time, migrations or external systems. A 4 names its main risk. A 5 is split before anyone plans it.

## s1-chaine-release — Un plugin vide, construit et attesté en CI, s'ouvre dans Obsidian
**As** Mathieu **I want** installer une release GitHub de Lophelia et ouvrir sa vue (vide) dans Obsidian **so that** toute la chaîne Rust/WASM → plugin → release attestée soit prouvée avant d'écrire le moteur.

- **Complexity**: 4
- **Depends on**: none
- **Surface**: `screen`
- **Owns**: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `crates/`, `plugin/`, `.github/`, `scripts/`, `.gitignore`, `LICENSE`

### Acceptance criteria
- [ ] Un tag `v*` poussé déclenche un workflow qui compile le crate WASM et le plugin depuis les sources du dépôt et publie `main.js`, `manifest.json` et `styles.css` dans une release GitHub.
- [ ] Chaque artefact publié porte une attestation de provenance GitHub ; `gh attestation verify <fichier> --repo atoll-labs-io/lophelia` réussit.
- [ ] Aucun fichier `.wasm`, ni blob WASM ou worker encodé en base64, n'est versionné : la CI échoue si `git ls-files` en liste un.
- [ ] La CI échoue si le bundle publié contient une API réseau (`fetch`, `XMLHttpRequest`, `WebSocket`, modules `http`/`https`/`net`, `requestUrl`).
- [ ] Installé dans un vault, le plugin ajoute une commande « Ouvrir Lophelia » qui ouvre une vue affichant la version du moteur, renvoyée par une fonction du module WASM.
- [ ] Les dépendances sont verrouillées (`Cargo.lock`, lockfile npm) et installées en mode strict en CI ; les outils de build (Rust, wasm-pack ou wasm-bindgen, Node) ont une version fixe.
- [ ] Chaque build CI lance un audit des avis de sécurité des dépendances (`cargo audit` pour Rust, `npm audit` pour le plugin) et échoue sur un avis de gravité haute ou critique.

### Notes for the agent
Risque principal (spike 6 du brief, prioritaire) : charger du WASM dans un plugin Obsidian sans committer de blob. New 3D Graph y parvient en commitant du base64 généré : c'est précisément ce qui est interdit ici. Le WASM est produit au build et embarqué dans `main.js` par l'outil de bundle. Mesurer la taille du bundle et la noter dans la PR. Disposition proposée : workspace Cargo avec `crates/lophelia-core/` (moteur, sans dépendance web) et `crates/lophelia-wasm/` (liaisons), plugin TypeScript sous `plugin/`. Rien n'est repris de New 3D Graph (ADR-0001, pré-mortem cause 5). Licence MIT à ajouter.

## s2-contrat-moteur — Le contrat du moteur et le format de positions.json
**As** hôte du moteur (le plugin Obsidian, puis la page de démo) **I want** une API unique — nœuds, arêtes, positions connues et réglages en entrée, positions en sortie — et un format `positions.json` versionné, lu et validé par le moteur **so that** tous les hôtes parlent au moteur de la même façon et qu'aucune position ne soit perdue ou corrompue.

- **Complexity**: 3
- **Depends on**: s1-chaine-release
- **Surface**: `endpoint`
- **Owns**: `Cargo.lock`, `crates/lophelia-core/Cargo.toml`, `crates/lophelia-core/src/lib.rs`, `crates/lophelia-core/src/contract/`, `crates/lophelia-core/src/positions/`, `crates/lophelia-core/tests/contract/`, `crates/lophelia-wasm/src/`, `docs/contracts/`, `.github/`

### Acceptance criteria
- [ ] L'API du moteur prend des nœuds (identifiants opaques), des arêtes, des positions connues et des réglages, et rend une position (x, y, z) par nœud ; elle n'importe ni Obsidian ni le DOM (vérifié en CI : le crate `lophelia-core` compile sans aucune dépendance web).
- [ ] Un nœud dont la position est connue et qui n'est touché par aucun changement ressort à la position exacte d'entrée.
- [ ] À graine fixe et entrées identiques, deux appels sur la même plateforme rendent les mêmes positions.
- [ ] `positions.json` porte un champ de version de format, une ligne par nœud, triée par chemin ; sérialiser puis relire redonne exactement les mêmes positions.
- [ ] Un fichier invalide (nombre non fini, taille au-delà de la borne, JSON cassé, version future inconnue) est refusé avec une erreur nommée, sans panique ; une version antérieure connue est lue.
- [ ] Une entrée dont le chemin est malformé (absolu, contenant `..`, vide) fait refuser le fichier avec une erreur nommée ; une entrée dont le chemin ne correspond à aucun nœud fourni par l'hôte est ignorée à la lecture et retirée à l'écriture suivante.
- [ ] Le contrat est décrit dans `docs/contracts/moteur.md` et appelé tel quel depuis le module WASM.

### Notes for the agent
Story contrat : elle ne sert aucun utilisateur final directement, mais s3, s4, s6, s8 et s11 en dépendent et la listent dans leur `Depends on`. Le moteur ne fait encore aucun calcul de disposition intelligent : un placement trivial suffit pour exercer le contrat. Borne de taille par défaut : 10 Mo. Les chemins sont des identifiants opaques pour le moteur ; c'est l'hôte qui sait ce qu'est une note.

## s3-banc-evaluation — Un banc d'évaluation noté, rejoué à l'identique
**As** Mathieu **I want** une commande qui note une disposition (séparation des groupes, stabilité, temps de calcul) sur des graphes figés **so that** chaque changement du moteur se juge sur des chiffres et pas au goût.

- **Complexity**: 3
- **Depends on**: s2-contrat-moteur
- **Surface**: `command`
- **Owns**: `Cargo.toml`, `Cargo.lock`, `crates/lophelia-bench/`, `bench/`, `.gitignore`, `.github/`

### Acceptance criteria
- [ ] Le dépôt contient des graphes synthétiques à communautés connues de 100, 1 000 et 2 500 nœuds, générés à graine fixe.
- [ ] `cargo run -p lophelia-bench -- <graphe>` affiche le score de silhouette des communautés dans l'espace 3D, le temps de calcul et, si une référence existe, l'écart à cette référence.
- [ ] Le banc calcule aussi une disposition de référence naïve (ressorts sur les liens + répulsion uniforme, paramètres fixes écrits dans le code) et affiche sa silhouette à côté de celle du moteur.
- [ ] Une sous-commande de stabilité rejoue une série scriptée de modifications et affiche le critère 3a (écart maximal quand rien ne change) et 3b (déplacement de la moitié des nœuds les plus éloignés de la note modifiée, en % du rayon).
- [ ] Le banc accepte un instantané local (`bench/local/`), ignoré par git ; la CI échoue si un fichier de `bench/local/` est versionné.
- [ ] `cargo run -p lophelia-bench -- snapshot <dossier-vault>` lit un vault sans Obsidian (notes Markdown, liens `[[…]]` résolus par nom de note, exclusions de `.obsidian/app.json`) et écrit l'instantané (nœuds + arêtes) dans `bench/local/`.
- [ ] `cargo run -p lophelia-bench -- vault <graphe> <dossier>` génère, hors du dépôt, un vault de test : une note Markdown par nœud, un lien `[[…]]` par arête, et un dossier `node_modules/` déclaré dans les « Fichiers exclus » de son `.obsidian/app.json`.
- [ ] Rejouer deux fois la même commande donne les mêmes chiffres.

### Notes for the agent
Aucune donnée de `~/dev` dans le dépôt, même hachée (pré-mortem, cause 6). L'instantané de `~/dev` est produit par la sous-commande `snapshot`, lancée par Mathieu sur sa machine ; il sert au spike de s4, à la mesure de diamètre de s8 et au seuil d'arrêt du 15/11/2026. Sa résolution des liens par nom de note approche celle d'Obsidian (`resolvedLinks`) sans l'égaler : c'est assez pour un banc. Les communautés de référence des graphes synthétiques sont celles de leur génération ; pour un instantané réel, une détection de communautés (Louvain ou équivalent) les fournit.

## s4-disposition-lisible — Une disposition 3D dont les groupes de liens se voient
**As** Mathieu **I want** que le moteur calcule, sans positions connues, une disposition 3D où les groupes de notes liées sont séparés **so that** le graphe ne soit pas une boule uniforme et que sa forme puisse se mémoriser.

- **Complexity**: 4
- **Depends on**: s3-banc-evaluation, s2-contrat-moteur
- **Surface**: `command`
- **Owns**: `Cargo.lock`, `crates/lophelia-core/Cargo.toml`, `crates/lophelia-core/src/lib.rs`, `crates/lophelia-core/src/layout/`, `crates/lophelia-core/tests/layout/`, `bench/references/`

### Acceptance criteria
- [ ] Sur les trois graphes synthétiques, la silhouette du moteur dépasse d'au moins 0,1 celle de la disposition naïve du banc (s3) et vaut au moins 0,3 ; la valeur obtenue est enregistrée dans `bench/references/` comme référence pour s8.
- [ ] Sur le graphe de 2 500 nœuds, le calcul complet depuis zéro prend moins de 10 s en build release, mesuré par le banc.
- [ ] Une fois la disposition stabilisée, un pas de calcul supplémentaire sans changement ne déplace aucun nœud (aucun tremblement).
- [ ] Deux composantes non reliées ne se superposent pas.
- [ ] La disposition passe par l'API du contrat (s2) sans la modifier.

### Notes for the agent
Risque principal (spike « groupes lisibles en 3D » du brief) : New 3D Graph produit une boule sans groupes. Comprendre pourquoi (répulsion en 3D, échelle, forces de liens) et comparer au moins deux approches sur le banc avant de choisir ; noter les chiffres dans la recherche. Point de contrôle au 15/11/2026 (seuil d'arrêt du brief). Seuils par défaut (0,1 ; 0,3 ; 10 s), réglables si le spike montre qu'ils sont mal placés. Les forces doivent rester réglables comme dans le graphe 2D d'Obsidian (centre, répulsion, force et distance des liens). Pour l'arrêt des tremblements : geler les nœuds sous un seuil d'énergie plutôt que de laisser la simulation tourner.

## s5-vue-vault-3d — Le vault en 3D dans Obsidian, exclusions respectées
**As** Mathieu **I want** ouvrir Lophelia et voir toutes mes notes non exclues en 3D, naviguer et ouvrir une note d'un clic **so that** j'aie la vue d'ensemble du vault et que j'aille droit à une note pour vérifier une réponse d'IA.

- **Complexity**: 3
- **Depends on**: s4-disposition-lisible
- **Surface**: `screen`
- **Owns**: `plugin/src/main.ts`, `plugin/src/view/`, `plugin/src/vault/`, `plugin/styles.css`, `plugin/package.json`, `plugin/package-lock.json`, `plugin/esbuild.config.mjs`, `web/render/`

### Acceptance criteria
- [ ] La vue montre un nœud par note Markdown du vault, sauf celles exclues par les « Fichiers exclus » d'Obsidian (`userIgnoreFilters`) ; une note sous `node_modules/` n'apparaît pas.
- [ ] Les arêtes sont les liens résolus entre notes (`resolvedLinks`) ; un lien vers une note exclue ou inexistante ne crée pas d'arête.
- [ ] Glisser avec le bouton gauche fait tourner la vue autour du centre du graphe, la molette rapproche ou éloigne la caméra, glisser avec le bouton droit la déplace ; les nœuds gardent leurs coordonnées.
- [ ] Un clic sur un nœud ouvre la note correspondante ; le survol affiche son nom comme du texte, jamais interprété en HTML.
- [ ] Le calcul de disposition tourne hors du fil principal : pendant le calcul, le fil principal n'est jamais bloqué plus de 50 ms (mesuré par le diagnostic), avec une progression visible.
- [ ] Sur le vault de test généré par le banc (s3) depuis le graphe de 2 500 nœuds, sur la machine de Mathieu (Arch, Hyprland), une commande de diagnostic affiche au moins 60 images/s en rotation, et jamais moins de 30.

### Notes for the agent
Spike « exclusions » du brief : trouver l'API Obsidian fiable pour `userIgnoreFilters` (le graphe natif, Graph+ et New 3D Graph ne l'appliquent pas). Le rendu three.js vit dans `web/render/` et n'importe rien d'Obsidian : la page de démo (s11) le réutilise. Positions recalculées à chaque ouverture dans cette story ; la persistance vient avec s6. Surbrillance de dossier et couleurs de `graph.json` sont hors wedge.

## s6-forme-persistante — La forme revient à l'identique à chaque ouverture
**As** Mathieu **I want** que les positions soient enregistrées dans le vault et relues à l'ouverture **so that** la forme ne bouge pas tant que le vault ne change pas et voyage avec lui.

- **Complexity**: 3
- **Depends on**: s5-vue-vault-3d, s2-contrat-moteur
- **Surface**: `screen`
- **Owns**: `plugin/src/main.ts`, `plugin/src/persistence/`

### Acceptance criteria
- [ ] Après le premier calcul, les positions sont écrites dans `.lophelia/positions.json` à la racine du vault, via `vault.adapter`.
- [ ] Fermer puis rouvrir Obsidian sans modifier le vault affiche chaque nœud à la position exacte enregistrée (critère 3a : écart 0).
- [ ] L'écriture est atomique (fichier temporaire puis renommage) : un fichier tronqué n'existe jamais à la place du bon.
- [ ] Un `positions.json` invalide n'est ni écrasé ni utilisé : il est renommé à côté, une notice Obsidian le signale, et une disposition neuve est calculée.
- [ ] Le graphe complet s'affiche en moins de 2 s quand les positions existent, sur le vault de test généré par le banc (s3) depuis le graphe de 2 500 nœuds.
- [ ] Une note présente dans le vault mais absente de `positions.json` est placée à une distance de ses notes liées inférieure à la distance médiane entre nœuds liés du graphe ; les autres ne bougent pas.
- [ ] Supprimer `.lophelia/` fait repartir d'un premier calcul, sans erreur.

### Notes for the agent
Le bug de Filamental : un fichier écrit avec `vault.create` dans un dossier caché ne se relit pas, car Obsidian n'indexe pas les dossiers cachés. Passer par `vault.adapter` (lecture et écriture directes). Lecture et validation par le moteur (contrat s2). Une note présente dans le vault mais absente du fichier est placée près de ses voisines connues.

## s7-compteur-ouvertures — Le compteur local d'ouvertures
**As** Mathieu **I want** voir combien de jours ouvrés j'ai ouvert Lophelia ce mois-ci **so that** je mesure le critère de réussite 1 sans rien envoyer hors de ma machine.

- **Complexity**: 1
- **Depends on**: s6-forme-persistante
- **Surface**: `screen`
- **Owns**: `plugin/src/main.ts`, `plugin/src/stats/`, `plugin/src/persistence/`

### Acceptance criteria
- [ ] Chaque ouverture de la vue enregistre la date du jour dans `.lophelia/usage.json`, au plus une fois par jour.
- [ ] Lophelia n'écrit rien dans `.obsidian/plugins/` en dehors de ses fichiers d'installation.
- [ ] Les réglages du plugin affichent, pour le mois en cours, le nombre de jours ouvrés avec ouverture et la moyenne par semaine ; un jour ouvré va du lundi au vendredi, jours fériés compris.
- [ ] Aucune donnée ne quitte la machine (le contrôle réseau de la CI, s1, reste vert).

### Notes for the agent
Sert la prémisse 5 (un premier utilisateur suffit) en mesurant le critère de réussite 1. Pas de `saveData` : il écrit `data.json` sous `.obsidian/plugins/<id>/`, dans le vault, ce qui casserait la promesse du brief « supprimer `.lophelia/` efface tout ce que Lophelia a écrit ». Le compteur voyage donc avec le vault ; c'est assumé (une seule personne, la mesure suit son vault).

## s8-ajustement-amorti — Une note ajoutée ou reliée autrement trouve sa place sans bouleverser le reste
**As** Mathieu **I want** qu'une note créée ou dont les liens changent rejoigne ses voisines, que ses voisines proches s'ajustent et que l'ajustement s'amortisse avec la distance **so that** le graphe reste vivant sans que je perde la forme mémorisée.

- **Complexity**: 4
- **Depends on**: s7-compteur-ouvertures, s2-contrat-moteur
- **Surface**: `screen`
- **Owns**: `crates/lophelia-core/src/lib.rs`, `crates/lophelia-core/src/adjust/`, `crates/lophelia-core/tests/adjust/`, `bench/scenarios/`, `plugin/src/main.ts`, `plugin/src/sync/`, `plugin/src/persistence/`

### Acceptance criteria
- [ ] Une note créée, la vue ouverte, apparaît en moins de 1 s à une distance de ses notes liées inférieure à la distance médiane entre nœuds liés du graphe ; pendant l'ajustement, le fil principal n'est jamais bloqué plus de 50 ms.
- [ ] Sur la série scriptée du banc, les voisins directs de la note modifiée bougent (déplacement moyen non nul), et le déplacement moyen décroît avec la distance : le quart des nœuds les plus proches bouge plus que le quart suivant, qui bouge plus que la moitié la plus éloignée.
- [ ] Après l'ajout d'un lien entre deux notes, leur distance diminue ; après son retrait, elle augmente.
- [ ] Sur la série scriptée du banc, la moitié des nœuds les plus éloignés de la note modifiée bougent de moins de 1 % du rayon du graphe (critère 3b ; seuil réglable).
- [ ] Le score de silhouette après la série reste au-dessus de la référence de s4.
- [ ] Les nouvelles positions sont enregistrées dans `positions.json` ; rouvrir Obsidian les retrouve à l'identique.

### Notes for the agent
Risque principal (spike « propagation amortie », 3 jours sur le banc, point de contrôle au 15/11/2026) : tenir à la fois des groupes lisibles et 3b. Commencer par mesurer le diamètre de `~/dev` avec et sans `_cartes/` (les cartes générées relient tout et raccourcissent les distances). Spike « bruit de positions.json » : mesurer combien de lignes changent par modification et, si c'est trop, quantifier les positions ou n'écrire que les lignes changées. Mesurer 3b en distance dans l'espace, pas en sauts (pré-mortem, cause 2).

## s9-renommage-suppression — Une note renommée garde sa place, une note supprimée disparaît
**As** Mathieu **I want** qu'une note renommée ou déplacée garde sa position, même renommée Obsidian fermé, et qu'une note supprimée sorte du graphe et du fichier **so that** mes réorganisations et celles des agents ne cassent pas la forme.

- **Complexity**: 3
- **Depends on**: s8-ajustement-amorti
- **Surface**: `screen`
- **Owns**: `plugin/src/sync/`, `plugin/src/persistence/`, `crates/lophelia-core/src/positions/`, `crates/lophelia-core/tests/contract/`, `crates/lophelia-wasm/src/`, `docs/contracts/`

### Acceptance criteria
- [ ] Une note renommée ou déplacée dans Obsidian garde exactement sa position ; `positions.json` porte le nouveau chemin.
- [ ] Une note supprimée disparaît de la vue et de `positions.json` ; après la suppression, la moitié des nœuds les plus éloignés de la note supprimée bouge de moins de 1 % du rayon du graphe (même seuil que le critère 3b de s8).
- [ ] Une note renommée Obsidian fermé (`git mv`) retrouve sa position à l'ouverture suivante, quand son contenu n'a pas changé.
- [ ] Quand le rapprochement est impossible (contenu modifié en même temps), la note est traitée comme nouvelle, sans erreur.
- [ ] `positions.json` passe à la version 2 de son format, avec une empreinte de contenu par note ; un fichier en version 1 est lu, réécrit en version 2, et une copie de l'ancien est gardée à côté ; `docs/contracts/moteur.md` décrit la version 2.

### Notes for the agent
Spike « renommage Obsidian fermé » du brief : une empreinte de contenu stockée avec la position permet de rapprocher une note disparue d'une note apparue. Les agents de Mathieu renomment souvent hors d'Obsidian. Événements Obsidian : `vault.on("rename")`, `vault.on("delete")`.

## s10-reglages-forces — Régler les forces comme dans le graphe 2D
**As** Mathieu **I want** un panneau de réglages des forces (centre, répulsion, force et distance des liens) proche de celui du graphe 2D d'Obsidian **so that** j'ajuste la lisibilité des groupes à l'usage.

- **Complexity**: 2
- **Depends on**: s9-renommage-suppression
- **Surface**: `screen`
- **Owns**: `plugin/src/main.ts`, `plugin/src/settings/`, `plugin/src/view/`, `plugin/src/persistence/`

### Acceptance criteria
- [ ] Un panneau dans la vue propose quatre curseurs : centre, répulsion, force des liens, distance des liens, avec les valeurs par défaut du moteur.
- [ ] Les réglages sont enregistrés dans `.lophelia/settings.json` et voyagent avec le vault.
- [ ] Changer un réglage ne déplace rien tant que l'utilisateur n'a pas cliqué « Appliquer » ; « Appliquer » recalcule la disposition et l'enregistre.
- [ ] Un bouton « Valeurs par défaut » remet les quatre curseurs à leurs valeurs initiales.

### Notes for the agent
Imiter le panneau « Forces » du graphe natif d'Obsidian. « Appliquer » est le seul moyen de déclencher une réorganisation globale : jamais en douce (prémisse 1).

## s11-demo-web — Une page web montre le même moteur hors d'Obsidian
**As** Mathieu **I want** une page web minimale qui charge un graphe et son `positions.json` avec le même moteur et le même rendu **so that** la séparation moteur/hôte soit prouvée avant de compter dessus pour Nautile ou mes sites.

- **Complexity**: 2
- **Depends on**: s9-renommage-suppression, s2-contrat-moteur
- **Surface**: `screen`
- **Owns**: `demo/`

### Acceptance criteria
- [ ] `npm --prefix demo run dev` lance un serveur local qui sert la page de démo et affiche son adresse.
- [ ] La page affiche en 3D le graphe synthétique de 1 000 nœuds du banc, avec le module WASM du moteur et le rendu de `web/render/`.
- [ ] Donner à la page un `positions.json` produit par le plugin affiche chaque nœud aux coordonnées exactes du fichier.
- [ ] La page ne contient aucun code propre à Obsidian, et le build de la démo ne modifie ni le moteur ni le rendu.

### Notes for the agent
Pré-mortem, cause 8 : prouver la prémisse 4 par un second hôte réel. Aucune publication en ligne dans cette story : la page tourne en local. Si le rendu ou le moteur doivent changer pour la démo, c'est le signe d'une fuite d'Obsidian dans `web/render/` ou le moteur, à signaler plutôt qu'à contourner.
