---
validated: yes
---
# Plan — s2-contrat-moteur

[Story](../stories.md) · [Research](../research/s2-contrat-moteur.md)

Branch `story/s2-contrat-moteur`. Read `docs/research/s2-contrat-moteur.md` first; this plan builds on it without repeating it.

Choix tranché par Mathieu le 14/10/2026, [ADR-0008](../decisions/0008-json-contrat-moteur.md) (`accepted`) : option 2, « le plus optimisé sur du gros volume de nœuds ». `serde`/`serde_json` dans `lophelia-core` pour `positions.json` seulement ; tableaux typés à la frontière WASM (positions en `Float64Array`, arêtes en `Uint32Array` d'index) ; identifiants passés une fois, l'hôte travaille ensuite par index. Validé sans relecture humaine du plan (`Plan validation: autonomous`).

## Story
Une API unique du moteur (nœuds, arêtes, positions connues, nœuds modifiés, réglages → une position (x, y, z) par nœud) et un format `positions.json` versionné, lu et validé par le moteur, appelés tels quels depuis le module WASM. Critères `docs/stories.md:36-42`, repris dans la recherche. Le critère 5 est lu dans la formulation corrigée par le pilote : « le lecteur aiguille sur le numéro de version ; une version inconnue est refusée ».

## Reuse
| New code | Rung that holds | Why each rung above fails |
| --- | --- | --- |
| Lecture et écriture de `positions.json` | dépendances nouvelles `serde` (derive) et `serde_json` (ADR-0008) | rien dans le dépôt (`crates/lophelia-core/src/lib.rs:1`) ; la bibliothèque standard ne lit pas le JSON ; aucune dépendance dans le cœur (`crates/lophelia-core/Cargo.toml:8`) |
| Écriture « une ligne par nœud » | `serde_json::to_string` par valeur (chemin, nombres), lignes assemblées à la main | `to_string_pretty` met un champ par ligne, pas une entrée par ligne |
| Index des identifiants, validation (index, longueurs, chemins, doublons, nombres finis, taille) | bibliothèque standard (`HashMap`, `f64::is_finite`) | aucune bibliothèque installée ne connaît ces règles ; c'est la frontière de confiance |
| Placement trivial déterministe | nouveau code : FNV-1a 64 et splitmix64 (une dizaine de lignes) | `DefaultHasher` n'est pas stable entre versions de Rust ; aucun générateur dans `std` ; `rand` serait une dépendance pour dix lignes |
| Exports WASM par tableaux typés | dépendance installée `wasm-bindgen` (`crates/lophelia-wasm/Cargo.toml:13`) : `Vec<String>`, `&[u32]`, `&[f64]`, `Vec<f64>`, struct à getters, vérifiés par sonde (recherche) | — |
| Contrôle « cœur sans dépendance web » | outil installé : `cargo tree` dans une étape de l'action composite | un script dans `scripts/` serait hors `Owns` |

## Tasks
1. [x] **Dépendances du cœur.** `crates/lophelia-core/Cargo.toml` : `serde = { version = "=1.0.229", features = ["derive"] }`, `serde_json = { version = "=1.0.154", features = ["float_roundtrip"] }` (présentes dans le cache cargo local). `Cargo.lock` mis à jour par un `cargo build` (sans `--locked`, une seule fois) ; entrées nouvelles attendues : `serde`, `serde_core`, `serde_derive`, `serde_json`, `itoa`, `memchr`, `zmij`. Rien dans `Cargo.toml` racine ni dans `crates/lophelia-wasm/Cargo.toml`.
   Vérification : `cargo tree --locked -p lophelia-core -e normal,build --target all --prefix none --format '{p}'` ne liste ni `wasm-bindgen`, ni `js-sys`, ni `web-sys`.

2. [x] **`src/contract/` — API de disposition par index.** `crates/lophelia-core/src/lib.rs` déclare `pub mod contract; pub mod positions;` et garde `VERSION`. Aucun `serde` dans ce module.
   - `pub struct Graph` : `Graph::new(ids: Vec<String>) -> Result<Graph, ContractError>` construit l'index identifiant → `u32` (identifiant deux fois → `DuplicateNode(id)`) ; `node_count()`, `ids() -> &[String]`, `index_of(&str) -> Option<u32>`. Les identifiants restent opaques.
   - `pub struct Settings { seed: u32, center: f64, repulsion: f64, link_strength: f64, link_distance: f64 }`, `Default` (graine 0, forces provisoires écrites comme telles dans le contrat, s4 les règle) ; `Settings::from_parts(seed: u32, forces: &[f64])` : exactement 4 valeurs dans l'ordre centre, répulsion, force des liens, distance des liens, sinon `MalformedInput` ; `Settings::forces() -> [f64; 4]`. Graine `u32` pour survivre au passage par un nombre JS.
   - `pub struct LayoutInput<'a> { edges: &'a [u32], known_nodes: &'a [u32], known_xyz: &'a [f64], changed: &'a [u32], settings: Settings }` : `edges` à plat par paires (source, cible) ; `known_xyz` à trois valeurs par entrée de `known_nodes`.
   - `Graph::layout(&self, input: &LayoutInput) -> Result<Vec<f64>, ContractError>` : rend `3 × node_count` valeurs, x, y, z du nœud d'index i aux positions `3i..3i+3`. Refus nommés : `MalformedInput(what)` (`edges` de longueur impaire, `known_xyz.len() != 3 × known_nodes.len()`), `IndexOutOfRange(index)` (dans `edges`, `known_nodes` ou `changed`), `DuplicateNode` (un index deux fois dans `known_nodes`), `NonFinite(what)` (coordonnée connue ou réglage non fini).
   - Un nœud avec position connue la garde exactement, qu'il soit dans `changed` ou non (provisoire de s2 : s8 ajustera sans changer la signature). Un nœud sans position connue est créé : `splitmix64` initialisé par `seed as u64 ^ fnv1a64(id)`, trois tirages ramenés dans `[-h, h)`, `h = link_distance × max(1, ∛n)` ; la position ne dépend que de la graine, de l'identifiant, de `n` et de `link_distance`, pas de l'ordre des index.
   - `ContractError` : `Display`, `std::error::Error`, `code() -> &'static str` stable (`duplicate_node`, `index_out_of_range`, `malformed_input`, `non_finite`). Aucun `unwrap`, `expect`, `panic!` ni indexation non contrôlée sur une donnée d'entrée.
   Tests, `crates/lophelia-core/tests/contract/layout.rs` (module de `tests/contract/main.rs`) : (a) un nœud créé et deux connus → `3n` valeurs, connus exacts, créé fini ; (b) critère 2, `changed` vide et tout connu (`0.1`, `-0.0`, `1e300`, `-5e-324`) → sortie égale bit à bit à `known_xyz` réordonné par index ; (c) critère 3, deux appels identiques avec nœuds créés → égalité bit à bit ; (d) deux graines → au moins un créé ailleurs ; (e) `Graph::new` avec un identifiant en double → `DuplicateNode` ; (f) `IndexOutOfRange` sur une arête **et** sur `changed` ; (g) `NonFinite` sur `f64::NAN` connu ; (h) `MalformedInput` sur `edges` impair **et** sur `known_xyz` de mauvaise longueur.

3. [x] **`src/positions/` — format version 1, lu contre un `Graph`.** Constantes `FORMAT_VERSION: u32 = 1`, `DEFAULT_MAX_BYTES: usize = 10 * 1024 * 1024` (10 Mio, écrit dans le contrat).
   `pub struct Known { pub nodes: Vec<u32>, pub xyz: Vec<f64> }` — exactement la forme que `LayoutInput` attend.
   `pub fn read(text: &str, graph: &Graph, max_bytes: usize) -> Result<Known, PositionsError>`, dans cet ordre :
   1. `text.len() > max_bytes` → `TooLarge { size, max }`, **avant** toute analyse ;
   2. en-tête `{ version }` seul ; JSON cassé → `InvalidJson(détail)` ; champ absent ou non entier → `MissingVersion` ; puis `match` sur le numéro : `1` → lecteur v1, tout autre → `UnsupportedVersion(n)` (critère 5 corrigé ; s9 ajoute sa branche) ;
   3. lecture complète v1, `deny_unknown_fields` : `{"version":1,"positions":[{"path":…,"x":…,"y":…,"z":…}, …]}` ;
   4. erreur `serde_json` dont le message commence par `number out of range` → `NonFiniteNumber(détail)` (pas de code public, recherche) ; finitude recontrôlée après lecture ;
   5. chaque chemin validé **avant** le filtrage : vide, commençant par `/` ou `\`, de la forme `X:`, ou dont un segment (découpe sur `/` et `\`) vaut exactement `..` → `MalformedPath { path, reason }` ; chemin deux fois → `DuplicatePath(path)` ;
   6. chemin absent du `Graph` → ignoré ; les autres deviennent `(index, x, y, z)`.
   `pub fn write(graph: &Graph, xyz: &[f64]) -> Result<String, PositionsError>` : `xyz.len() != 3 × node_count` → `LengthMismatch` ; mêmes contrôles de chemin et de finitude (un fichier écrit est toujours relisible) ; tri par `str::cmp` (octets UTF-8) ; sortie exacte : `{"version":1,"positions":[`, une ligne par nœud `{"path":<serde_json::to_string>,"x":…,"y":…,"z":…}` suivie de `,` sauf la dernière, `]}`, saut de ligne final. Seuls les nœuds du `Graph` sont écrits : un chemin périmé disparaît. `PositionsError` : `Display`, `Error`, `code()` (`too_large`, `invalid_json`, `missing_version`, `unsupported_version`, `non_finite_number`, `malformed_path`, `duplicate_path`, `length_mismatch`).
   Tests, `tests/contract/positions.rs` : (a) critère 4, `write` puis `read` de 200 positions tirées sur tout l'espace des `f64` finis, chemins à accents, guillemets et `\` → égalité bit à bit ; (b) format : identifiants non triés → première ligne avec `"version":1`, une ligne par nœud, chemins dans l'ordre des octets ; (c) `1e999` → `NonFiniteNumber` ; (d) `max_bytes` + 1 → `TooLarge` ; (e) JSON tronqué → `InvalidJson` ; (f) `"version":2` → `UnsupportedVersion(2)` ; (g) sans `version` → `MissingVersion` ; (h) `/abs.md`, `a/../b.md`, `""` → trois `MalformedPath` ; (i) `a..b.md` et `d/..x/n.md` acceptés ; (j) chemin absent du `Graph` → absent de `Known`, sans erreur ; (k) `DuplicatePath` ; (l) `write` d'un identifiant `/abs.md` → `MalformedPath`, et `xyz` trop court → `LengthMismatch`.
   Test de chaîne, `tests/contract/main.rs` : (m) **critère 6, seconde moitié** — un fichier avec un chemin périmé → `read` → `layout` → `write` : le chemin périmé n'est plus dans le texte, les autres positions sont identiques bit à bit.

4. [x] **Exports WASM, `crates/lophelia-wasm/src/lib.rs`.** Aucune dépendance ajoutée ; si `wasm-bindgen` refusait une de ces signatures, s'arrêter et le rapporter (la sonde de la recherche les a compilées).
   - `#[wasm_bindgen] pub struct Engine { graph: Graph }` ; `#[wasm_bindgen(constructor)] new(ids: Vec<String>) -> Result<Engine, JsError>` → `Graph::new` ; `node_count() -> u32`.
   - `layout(&self, edges: &[u32], known_nodes: &[u32], known_xyz: &[f64], changed: &[u32], seed: u32, forces: &[f64]) -> Result<Vec<f64>, JsError>` → `Settings::from_parts` puis `Graph::layout`.
   - `read_positions(&self, file: &str) -> Result<KnownPositions, JsError>` → `positions::read(file, &graph, DEFAULT_MAX_BYTES)` ; `#[wasm_bindgen] pub struct KnownPositions` à getters `nodes: Vec<u32>`, `xyz: Vec<f64>`, passables tels quels à `layout`.
   - `write_positions(&self, xyz: &[f64]) -> Result<String, JsError>` → `positions::write`.
   - fonction libre `default_forces() -> Vec<f64>` → `Settings::default().forces()`.
   Chaque corps est une conversion directe vers le cœur ; l'erreur devient `JsError::new(&format!("{}: {}", e.code(), e))`. Le plugin n'est pas touché (s5/s6).
   Tests natifs, chemin `Ok` seulement (un `JsError` panique hors WASM) : (a) `Engine::new` puis `layout` rend la même sortie que `Graph::layout` ; (b) `write_positions` puis `read_positions` rend les index et positions écrits.
   Vérification : `scripts/build-wasm.sh` construit ; `plugin/wasm-pkg/lophelia_wasm.d.ts` type `layout(...)` en `Uint32Array`/`Float64Array` et `constructor(ids: string[])` ; `npm --prefix plugin run typecheck` reste vert.

5. [x] **Contrôle CI « cœur sans dépendance web », `.github/actions/build/action.yml`.** Étape après « Rust formatting » : `cargo tree --locked -p lophelia-core -e normal,build --target all --prefix none --format '{p}'` ; échec, en nommant le paquet, si une ligne commence par `wasm-bindgen`, `js-sys`, `web-sys`, `gloo` ou `obsidian`. Rien d'autre ne change dans `.github/` (issues #42, #43 hors story).
   Vérification : sort en 0 sur le dépôt ; avec `-p lophelia-wasm` à la place, sort en 1 en nommant `wasm-bindgen`.

6. [x] **`docs/contracts/moteur.md`**, en français : l'objet `Engine` et ses méthodes, avec leurs équivalents Rust ; la correspondance identifiant ↔ index (ordre de `ids` au constructeur), la mise à plat de `edges` et de `xyz`, l'ordre des quatre forces et leurs valeurs par défaut provisoires ; les règles de la tâche 2 (créé, connu, modifié provisoire, refus) ; la libération par `free()` ; le format `positions.json` v1 octet par octet, l'ordre de tri (octets UTF-8), la borne de 10 Mio, l'aiguillage par version ; la table des codes d'erreur et le format `code: détail` du message levé ; la règle de compatibilité du brief (`docs/brief.md:87`) ; la reproductibilité sur une même plateforme seulement (`docs/brief.md:102`).
   Vérification : chaque code de `ContractError` et `PositionsError` figure dans la table (relecture).

## Scene viewpoints
—

## Run interdicts
- Diff vide sur `Cargo.toml` (racine), `crates/lophelia-wasm/Cargo.toml`, `rust-toolchain.toml`, `plugin/`, `scripts/`, `.github/workflows/`, `AGENTS.md`, `AGENTS.local.md`, `docs/architecture.md`, `docs/decisions/0001` à `0007` ; `docs/stories.md` ne change que par la correction du critère 5 faite par le pilote.
- Aucun fichier hors des `Owns` (`docs/stories.md:33`), sauf `docs/research/`, `docs/plans/`, `docs/verif/`, `docs/qa/`, `docs/reviews/` de s2 et `docs/decisions/0008-json-contrat-moteur.md`.
- Dépendances ajoutées limitées à `serde` et `serde_json`, versions exactes, dans `crates/lophelia-core/Cargo.toml` seulement, `float_roundtrip` présent ; `serde` n'apparaît que dans `src/positions/`.
- Aucun JSON à la frontière WASM : pas d'export qui prend ou rend du JSON, hormis `read_positions`/`write_positions` qui manipulent le texte du fichier.
- Aucun algorithme de forces, de disposition ni d'ajustement (s4, s8).
- Ni `unsafe`, ni `DefaultHasher`, ni `rand`, ni horloge, ni accès fichier ou réseau dans `lophelia-core`.
- `grep -rn 'unwrap\|expect(\|panic!' crates/lophelia-core/src` vide.

## The pivot
La frontière WASM ne transporte que des tableaux plats indexés, et toute conversion (index, réglages, fichier) vit dans `lophelia-core`. Comment cela peut casser :
- **Index et identifiants désaccordés** : l'hôte doit garder l'ordre des `ids` du constructeur ; un index hors bornes est refusé, mais un index valide mal attribué ne l'est pas. Comparer le test 3(m) : la chaîne lecture → disposition → écriture redonne chaque chemin à sa position.
- **Relecture inexacte des flottants** : sans `float_roundtrip`, un `f64` sur trois change d'un ULP (sonde). Le test 3(a) doit échouer sans la fonction.
- **« Nombre non fini » reconnu au message** : une montée de `serde_json` qui reformule `number out of range` fait tomber `1e999` dans `InvalidJson` ; la version exacte et le test 3(c) le gardent.
- **Valeurs par défaut des forces dans `src/contract/`** : s4 ne possède pas ce dossier ; à signaler au cadrage de s4.

## Files touched
`Cargo.lock`, `crates/lophelia-core/Cargo.toml`, `crates/lophelia-core/src/lib.rs`, `crates/lophelia-core/src/contract/mod.rs`, `crates/lophelia-core/src/positions/mod.rs`, `crates/lophelia-core/tests/contract/{main.rs,layout.rs,positions.rs}`, `crates/lophelia-wasm/src/lib.rs`, `.github/actions/build/action.yml`, `docs/contracts/moteur.md`, plus `docs/research/s2-contrat-moteur.md`, `docs/plans/s2-contrat-moteur.md`, `docs/decisions/0008-json-contrat-moteur.md`.

## Test strategy
Automatisés, 23 (sous le `Test budget` de 25), sur l'API publique, chacun lié à une règle qui, retirée, le fait échouer :
- contrat de disposition, 8 : tâche 2 (a)–(h) ; (f) et (h) portent deux assertions, une par point de contrôle ;
- format `positions.json`, 12 : tâche 3 (a)–(l) ; (h) porte les trois formes malformées ;
- chaîne lecture → disposition → écriture, 1 : tâche 3 (m) ;
- liaisons WASM, 2 : tâche 4 (a), (b) ; le chemin d'erreur (`JsError`) n'est pas testable en natif, il est vu en QA ;
- l'existant `engine_version_is_the_core_version` reste.
Pas de test pour `default_forces` (constante) ni pour la tâche 5, vérifiée par la manipulation inverse et par la revue.
Tâche par tâche : `cargo test --locked -p lophelia-core --test contract`, `cargo test --locked -p lophelia-wasm`. En fin : `Test` et `Typecheck` par `po-run`, plus `cargo fmt --check` et `cargo clippy --workspace --all-targets --locked -- -D warnings`.
QA (`Surface: endpoint`, `Platform` comprend `lib`) : un script Node qui charge `plugin/wasm-pkg/lophelia_wasm_bg.wasm` par `initSync`, construit un `Engine`, appelle `layout`, `write_positions`, `read_positions` avec des `Float64Array`/`Uint32Array`, et provoque les erreurs nommées (`1e999`, chemin `../x.md`, version 9, index hors bornes).

## Done when
- Un commit `s2-contrat-moteur: Le contrat du moteur et le format de positions.json` atterri sur `main` par pull request, CI verte, nouvelle étape de contrôle comprise.
- Les sept critères tenus, le critère 5 dans sa formulation corrigée.
- Revue sans constat critique ouvert.
