# Banc d'évaluation

Le crate `lophelia-bench` note une disposition du moteur sur des graphes figés, pour juger chaque changement du moteur sur des chiffres.

## Commandes

| Commande | Effet |
| --- | --- |
| `cargo run -p lophelia-bench -- <graphe.json> [--record]` | silhouette des communautés en 3D (moteur et disposition naïve), temps de calcul du moteur, écart à la référence si elle existe ; `--record` enregistre la silhouette du moteur comme référence |
| `cargo run -p lophelia-bench -- generate` | régénère `bench/graphs/sbm-{100,1000,2500}.json` (graine fixe) |
| `cargo run -p lophelia-bench -- stability <graphe.json> <scénario.json>` | rejoue la série et affiche 3a et 3b par étape, puis leurs maxima |
| `cargo run -p lophelia-bench -- snapshot <dossier-vault>` | lit un vault sans Obsidian et écrit `bench/local/<nom du dossier>.json` |
| `cargo run -p lophelia-bench -- vault <graphe.json> <dossier>` | écrit un vault de test hors du dépôt, dans un dossier absent ou vide |

## Formats

- **Graphe** (`bench/graphs/`, `bench/local/`) : `{"version":1,"nodes":[…],"edges":[…]}`, une ligne par nœud `{"id":"c00/n00-0000.md","community":0}`, triés par identifiant, puis une ligne par arête non orientée `[a,b]` (`a < b`, index dans `nodes`).
- **Scénario** (`bench/scenarios/`) : `{"version":1,"steps":[…]}`, chaque étape exactement une de `{"link":["a.md","b.md"]}`, `{"unlink":["a.md","b.md"]}`, `{"create":"x.md","links":["a.md"]}`. La note modifiée est la première du couple, ou la note créée.
- **Référence** (`bench/references/<graphe>.json`) : `{"silhouette":<nombre>}`, écrite par `--record`.

## Mesures

- **Silhouette** : moyenne sur tous les nœuds, distance euclidienne en 3D ; un nœud seul dans sa communauté compte 0.
- **Disposition naïve** : ressorts linéaires sur les liens, répulsion uniforme en `1/d²`, 300 itérations ; paramètres fixes dans `crates/lophelia-bench/src/naive.rs`.
- **3a** : à chaque étape, positions relues depuis `positions.json` et moteur relancé sans changement ; écart maximal d'un nœud.
- **3b** : après la modification, déplacement maximal de la moitié des nœuds les plus éloignés de la note modifiée, en % du rayon (distance maximale au barycentre avant la modification).
- **Communautés d'un instantané** : Louvain déterministe ; la modularité finale est affichée à titre de contrôle.

## Reproductibilité

Deux exécutions de la même commande donnent les mêmes notes et les mêmes fichiers ; seul le temps de calcul varie.

Le temps est mesuré dans le profil où le banc est compilé. En release, le moteur hérite de `opt-level = "z"` et de la LTO du profil `[profile.release]` (choisis pour le WASM livré) : c'est ce moteur-là que mesure `cargo run --release`.

## Instantanés locaux

`bench/local/` est ignoré par git et la CI échoue si un fichier y est versionné : un instantané d'un vault personnel ne quitte jamais la machine.
