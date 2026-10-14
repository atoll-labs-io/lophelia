# Research — s3-banc-evaluation

[Story](../stories.md)

## Structuring facts
- **Critère 8 inexact tel qu'écrit** : « rejouer deux fois la même commande donne les mêmes chiffres » ne peut pas valoir pour le temps de calcul (critère 2), mesuré à l'horloge murale. Seules les notes (silhouettes, écart à la référence, 3a, 3b) et les fichiers écrits (`snapshot`, `vault`, graphes générés) sont reproductibles. À corriger dans `docs/stories.md:63` : « … donne les mêmes notes ; seul le temps de calcul varie ». Le plan l'applique ainsi.
- **s4 ne peut pas écrire sa référence sans s3** : s4 possède `bench/references/` mais pas `crates/lophelia-bench/` (`docs/stories.md:74`, `:77`). Le format de référence et son écriture (`--record`) reviennent donc à s3 ; sans eux, le critère 2 (« écart à cette référence ») n'a rien à lire.
- Le moteur actuel ne fait qu'un placement pseudo-aléatoire par identifiant (`crates/lophelia-core/src/contract/mod.rs:181-192`, `:198-205`) et garde toute position connue même si le nœud est dans `changed` (`:181-182`) : aujourd'hui, silhouette moteur ≈ 0, 3a = 0 et 3b = 0 sur une modification de liens. Le banc doit mesurer, pas supposer ce comportement.
- `lophelia-bench` n'existe pas : workspace à deux membres (`Cargo.toml:3`), aucun dossier `bench/`. ADR-0003 l'a déjà prévu comme crate CLI (`docs/decisions/0003-pile-technique.md`, « Decision Outcome »), à l'emplacement de `docs/architecture.md:19`.
- Aucune dépendance nouvelle n'est nécessaire : `serde`/`serde_json` sont déjà au verrou (`Cargo.lock:76`, `:106`) ; arguments, générateur pseudo-aléatoire, Louvain et filtres d'exclusion s'écrivent avec `std` (voir Open questions).

## Story
Une commande `lophelia-bench` qui note une disposition sur des graphes figés, pour juger chaque changement du moteur sur des chiffres. Critères (`docs/stories.md:56-63`) :
1. graphes synthétiques à communautés connues, 100 / 1 000 / 2 500 nœuds, générés à graine fixe, dans le dépôt ;
2. `cargo run -p lophelia-bench -- <graphe>` : silhouette des communautés en 3D, temps de calcul, écart à la référence si elle existe ;
3. disposition naïve de référence (ressorts sur les liens + répulsion uniforme, paramètres en dur), silhouette affichée à côté de celle du moteur ;
4. sous-commande de stabilité : série scriptée, critère 3a (écart maximal sans changement) et 3b (déplacement de la moitié la plus éloignée de la note modifiée, en % du rayon) ;
5. instantané local `bench/local/` ignoré par git, CI en échec si un fichier y est versionné ;
6. `snapshot <dossier-vault>` : lit un vault sans Obsidian (`.md`, `[[…]]` résolus par nom, exclusions de `.obsidian/app.json`) et écrit nœuds + arêtes dans `bench/local/` ;
7. `vault <graphe> <dossier>` : vault de test hors du dépôt, une note par nœud, un `[[…]]` par arête, `node_modules/` déclaré dans les « Fichiers exclus » ;
8. deux exécutions identiques → mêmes chiffres (lu « mêmes notes », voir plus haut).

## Current code
- `crates/lophelia-core/src/contract/mod.rs` — `Graph::new(ids)` (`:108`) indexe les identifiants, refuse un doublon (`DuplicateNode`, `:114-116`) ; `Graph::layout(&LayoutInput)` (`:143`) rend `3n` flottants, valide arêtes paires, index, finitude (`:144-179`), ignore encore les arêtes pour placer. `Settings::default()` : graine 0, forces provisoires (`:51-61`).
- `crates/lophelia-core/src/positions/mod.rs` — `read(text, &graph, max_bytes) -> Known` (`:102`), `write(&graph, xyz) -> String` (`:142`) ; chemins validés (absolu, `..`, vide refusés) et triés par octets. C'est le chemin réel d'un hôte entre deux ouvertures.
- `crates/lophelia-core/tests/contract/main.rs:1-2` — habitude des tests : un binaire d'intégration `tests/<nom>/main.rs` qui déclare ses modules ; aides `graph()` / `input()` dans `layout.rs:3`, `:10`.
- `.github/actions/build/action.yml` — action composite partagée par `ci.yml:18` et `release.yml:36` : `cargo fmt --check` (`:33-35`), clippy `--workspace --all-targets -D warnings` (`:48-50`), `cargo test --workspace --locked` (`:52-54`), contrôle « cœur sans dépendance web » (`:37-46`).
- `.gitignore:1-8` — pas de `bench/local/`.
- `scripts/check-no-blobs.mjs:17-27` — refuse `.wasm` et blobs base64 ; des fichiers JSON de graphes ne le déclenchent pas.

## Attachment points
- `Cargo.toml:3` (`members`) — ajouter `crates/lophelia-bench`.
- `crates/lophelia-bench/` — nouveau crate : `src/lib.rs` (logique testable) + `src/main.rs` (aiguillage des sous-commandes), dépendance de chemin vers `lophelia-core`.
- `bench/graphs/`, `bench/scenarios/` (prévu par `docs/architecture.md:19`), `bench/references/` (écrit via `--record`, rempli par s4), `bench/local/` (ignoré).
- `.gitignore` — `bench/local/`.
- `.github/actions/build/action.yml` — une étape `git ls-files bench/local` vide (le dossier `scripts/` n'est pas dans les `Owns` de s3 : le contrôle vit dans l'action).

## Verified interfaces
- `lophelia_core::contract::Graph::new(ids: Vec<String>) -> Result<Graph, ContractError>` (`contract/mod.rs:108`).
- `Graph::layout(&self, input: &LayoutInput) -> Result<Vec<f64>, ContractError>` (`:143`) ; `LayoutInput { edges: &[u32], known_nodes: &[u32], known_xyz: &[f64], changed: &[u32], settings: Settings }` (`:93-99`).
- `Graph::index_of(&str) -> Option<u32>` (`:129`), `ids() -> &[String]` (`:125`), `node_count()` (`:121`).
- `lophelia_core::positions::{read, write, Known, DEFAULT_MAX_BYTES}` (`positions/mod.rs:12`, `:16-19`, `:102`, `:142`).
- `ContractError::code()` / `PositionsError::code()` (`contract/mod.rs:16`, `positions/mod.rs:34`) — codes stables à reprendre dans les messages du banc.
- `splitmix64` et `fnv1a64` sont privés (`contract/mod.rs:208`, `:214`) et `contract/` n'est pas dans les `Owns` : le banc écrit son propre générateur.
- Toolchain Rust 1.99.0 (`rust-toolchain.toml:2`), édition 2024 (`Cargo.toml:7`) ; `serde = "=1.0.229"` derive, `serde_json = "=1.0.154"` `float_roundtrip` (`crates/lophelia-core/Cargo.toml:8-10`).
- Vault réel de Mathieu (lu localement, rien copié) : `~/dev/.obsidian/app.json` porte 8 `userIgnoreFilters`, tous des préfixes de dossier terminés par `/`, aucune expression régulière.

## Traps
- **Durée en debug** : le critère 2 dit `cargo run` sans `--release`. La disposition naïve est en O(n²) par itération : 2 500 nœuds × quelques centaines d'itérations en debug se comptent en minutes. Parade sans toucher au moteur : `[profile.dev.package.lophelia-bench] opt-level = 3` dans `Cargo.toml`. Le temps du moteur, lui, reste mesuré dans le profil du moteur.
- **Profil release** : `opt-level = "z"` + LTO (`Cargo.toml:11-13`) s'applique aussi au moteur natif mesuré par le banc ; s4 mesure « en build release » (`docs/stories.md:78`). Ne pas changer ce profil (il sert au WASM livré) ; le noter dans `bench/README.md`.
- **Déterminisme** : `HashMap`/`HashSet` de `std` itèrent dans un ordre aléatoire par instance ; tout parcours qui produit une sortie (Louvain, résolution des liens, instantané) passe par des `Vec` triés ou `BTreeMap`. Graine du moteur fixe (`Settings::default()`, graine 0).
- **Sortie non finie** : issue #54 (forces finies énormes → coordonnées infinies). Le banc refuse une sortie non finie du moteur par une erreur nommée au lieu d'afficher `NaN`.
- **Chemins** : les identifiants passent par `positions::write`, qui refuse chemin absolu, `..`, vide (`docs/contracts/moteur.md:68`) ; les identifiants synthétiques et ceux de l'instantané (relatifs au vault, séparateur `/`) doivent les respecter.
- **Tests en CI** : clippy `--all-targets -D warnings` et `cargo test --workspace` incluent le nouveau crate, dans `ci.yml` **et** `release.yml` ; les tests doivent rester rapides (graphe de 100 nœuds sauf la comparaison des graphes générés).
- **Données personnelles** : aucun test ni fichier versionné ne lit `~/dev` (`docs/stories.md:66`, pré-mortem 6). Les tests travaillent sur des vaults générés dans `std::env::temp_dir()`.
- **Liens entre stories** : s8 possède ensuite `crates/lophelia-bench/` et `bench/scenarios/` et a besoin d'ajout **et** de retrait de lien dans la série (`docs/stories.md:153`) ; ADR-0006 annonce un vault de test « généré par le banc dès s3 » (`docs/decisions/0006-qa-ecrans-obsidian.md:24`), mais brancher `scripts/dev.sh` dessus est hors des `Owns` de s3.
- Pas de migration ; `Cargo.lock` ne gagne que l'entrée `lophelia-bench`.

## Open questions
- **Louvain écrit à la main ou bibliothèque ?** Les notes de la story demandent « Louvain ou équivalent » pour les communautés d'un instantané réel. Écrit à la main (~150 lignes, `std`, déterministe) plutôt qu'un crate : le brief veut des dépendances minimales et justifiées, et aucun crate Louvain établi n'existe dans l'écosystème. Le plan tranche ainsi, sans ADR, puisque rien n'est ajouté au verrou ; si Mathieu préfère un crate, c'est un ADR.
- **Filtres d'exclusion en expression régulière** (`/…/`) : non appliqués, signalés sur stderr (le vault réel n'en a aucun, voir plus haut). Une regex demanderait le crate `regex`, donc un ADR.
- **Sémantique exacte d'un filtre simple d'Obsidian** : le plan le traite comme préfixe de chemin relatif ; c'est une approximation, comme la résolution des liens (`docs/stories.md:66`).
- **Liens Markdown `[texte](note.md)`** et liens dans les blocs de code : non lus (le critère 6 ne cite que `[[…]]`).
- **Liens symboliques** dans le vault : non suivis (évite les cycles) ; Obsidian peut les suivre.
- **Rayon du graphe** (3b) : le plan prend la distance maximale au barycentre de la disposition avant modification.

## Complexity
Story : 3 → après lecture : 3. Logique métier réelle (silhouette, disposition naïve, 3a/3b, Louvain, lecture d'un vault) mais aucun état persistant, aucune intégration tierce ; le volume (huit critères, cinq sous-commandes) pousse vers une dizaine de tâches, sans justifier un 4.

## Split
Sans objet (score 3).
