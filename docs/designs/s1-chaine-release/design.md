# Design — s1-chaine-release

Written by `/po-design` (sous-agent `designer`, couloir de `/po-drive`) into `docs/designs/s1-chaine-release/design.md`. The files in that folder win over any external tool or conversation; change them by running `/po-design` again.

Aucun humain n'a approuvé ce design (couloir sans approbation, ADR-0019).

## Screen
La vue « Lophelia », encore vide de graphe : elle prouve que le module WASM est chargé en affichant la version du moteur qu'il renvoie. Pour Mathieu, qui l'ouvre dans Obsidian par la palette de commandes, commande « Ouvrir Lophelia » ; la commande révèle la vue si elle est déjà ouverte, sinon en ouvre une dans un onglet de l'espace principal.

## Kind
`new`. Régions :

| Région | Ce que le système tient | Verdict |
| --- | --- | --- |
| Onglet et cadre de la vue | composant `Vue` (`ItemView`), variante « une vue Lophelia » | couvert |
| Commande d'ouverture | composant `Commande`, variante « Ouvrir Lophelia » | couvert |
| Corps de la vue : une ligne de texte (version) | tokens de texte et de surface ; aucune disposition du contenu d'une vue (placement, alignement, marges) | **non couvert** |
| Échec de chargement | motif Error : `Notice` avec message en mots | couvert |

Le corps de la vue n'a aucun motif de disposition dans `docs/design-system.md` : le motif Loading fixe un texte et ses tokens, pas sa place. L'écran est donc nouveau ; le doute et sa conséquence sont notés sous `## Gaps`.

## Deltas
Sans objet : écran nouveau, toutes les sections sont remplies.

## Components and tokens

| Region | Component | Variant | Tokens |
| --- | --- | --- | --- |
| Onglet (titre « Lophelia ») | Vue | une vue « Lophelia » | — (chrome d'Obsidian, non stylé par Lophelia) |
| Fond du corps de la vue | Vue | une vue « Lophelia » | `--background-primary` |
| Ligne de version | — (texte posé dans le conteneur de la vue, par `setText`) | — | `--text-muted` sur `--background-primary` |
| Ligne d'échec dans la vue | — (texte posé dans le conteneur de la vue) | — | `--text-error` sur `--background-primary` (jamais sur `--background-secondary`) |
| Notification d'échec | Notice | message simple | — (composant natif) |
| Commande | Commande | « Ouvrir Lophelia » | — |

Le conteneur porte la classe `lophelia-view` ; `plugin/styles.css` redéfinit sous `.lophelia-view` seulement les valeurs corrigées utiles à cet écran : `--text-error` (#e72038 en clair) et, par cohérence avec la règle du système, les autres corrections listées dans `docs/design-system.md` (`## Tokens`, valeurs « → »). Aucune couleur en dur.

## States
- **Ouverte, moteur chargé (succès)** : une ligne, `Moteur 0.1.0` — le mot « Moteur », une espace, la chaîne exacte renvoyée par `engine_version()`. Aucune notification (motif Success : silencieux).
- **Échec de chargement du WASM (erreur)** : la vue affiche `Moteur non chargé`, et une `Notice` dit `Lophelia : le moteur n'a pas pu être chargé.` suivi du message d'erreur en texte. Le signal est en mots, la couleur `--text-error` ne fait que l'accompagner.
- **Chargement** : non atteignable — `initSync` est synchrone et s'exécute à la première demande de version, à l'ouverture de la vue ; aucun texte intermédiaire.
- **Vide (vault sans note)** : sans objet pour s1, la vue n'affiche pas encore de graphe.
- **Désactivé, focus** : la vue ne contient aucun contrôle. Le focus clavier reste sur l'onglet, géré par Obsidian ; Échap, Ctrl+P et la navigation entre onglets restent ceux d'Obsidian.

## Themes and widths
- **Clair / sombre** : la vue suit le thème d'Obsidian par ses variables ; rien ne change dans la disposition. Valeurs en clair : texte #5c5c5c sur #ffffff (6,69:1), erreur #e72038 (corrigée, 4,50:1) ; en sombre : #b3b3b3 sur #1c1c1c (8,13:1), erreur #fb464c (4,94:1). Ratios repris de `docs/design-system.md`, non remesurés.
- **Largeur** : le plugin est `isDesktopOnly` ; il n'y a pas d'écran mobile. La largeur étroite est celle d'un panneau latéral ou d'un écran scindé : la ligne passe à la ligne si le texte d'erreur dépasse, sans défilement horizontal.
- **Disposition du texte** : non fixée par le système (voir `## Gaps`). Elle devait être tranchée par la maquette, qui n'a pas pu être dessinée.

## Artefact
`none` pour l'instant — `mockup.html` est dû (écran nouveau, `Design source: internal`) mais n'a pas été dessiné : le skill `Design skill: frontend-design` n'a pas de `SKILL.md` à `.claude/skills/frontend-design/SKILL.md` dans le worktree. Ce design ne se dessine pas au goût.

## Seen and not seen
- Vu : rien dans un navigateur — aucune maquette n'existe à servir.
- Non vu : les deux états (succès, erreur), en clair et en sombre, en pleine largeur et en panneau étroit ; raison : pas de maquette, faute du skill de dessin.

## Gaps
- **Skill de dessin absent** : `Design skill: frontend-design` est réglé, mais aucun `.claude/skills/frontend-design/SKILL.md` n'existe dans le worktree (ni sous `~/.claude/skills/`). La maquette attend qu'il soit installé ; `/po-design s1-chaine-release` la produira alors.
- **Disposition du contenu d'une vue** : le système ne dit ni où se place un texte dans le corps d'une `ItemView` (en haut à gauche avec le retrait d'Obsidian, ou centré dans la vue), ni sa taille de police. Ce même choix servira au texte de chargement de s5. À trancher par une personne ; sans maquette, l'implémenteur n'a pas de disposition fixée.
- **Doute dérivé / nouveau** : régions qui ont fait hésiter — la ligne de version, qu'on pourrait voir comme une reprise du motif Loading (texte `--text-muted` sur `--background-primary`). Tranché `new`, parce que ce motif ne porte aucune disposition.
- **Texte de l'écran** : `Moteur 0.1.0`, `Moteur non chargé` et le texte de la `Notice` sont proposés ici ; aucun texte d'interface n'est fixé ailleurs (brief, stories, système).
- **Taille de police** : aucun token de typographie dans le système (`--font-*`, tailles) ; la ligne hérite donc de la taille du texte d'Obsidian.

## Décision du lead (14/10/2026)
Mathieu a demandé un rendu fidèle à Obsidian, le style étant revu une fois l'écran visible. En conséquence :
- **Disposition** : le corps de la vue reprend la structure de l'état vide natif d'Obsidian, `div.empty-state > div.empty-state-container` (centré horizontalement et verticalement, `text-align: center`, défini dans `app.css:4083-4102` d'Obsidian 1.14.4). Aucun CSS de disposition n'est propre à Lophelia. La ligne de version reste en `--text-muted`, à la taille du texte d'Obsidian, sans titre `.empty-state-title`. Le texte de chargement de s5 reprendra la même structure.
- **Maquette** : aucune. L'écran est une ligne de texte dans une structure native, et l'implementer la construit telle que décrite ci-dessus ; la QA la regarde dans Obsidian.
- **Ligne d'erreur** `Moteur non chargé` et textes proposés : retenus tels quels.
