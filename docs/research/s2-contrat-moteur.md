# Research — s2-contrat-moteur

[Story](../stories.md)

## Structuring facts
- **Hypothèse fausse de la story** : « une version antérieure connue est lue » (`docs/stories.md:40`). Aucune version antérieure n'existe : s2 crée la version 1 ; s9 porte la lecture d'une version 1 par un lecteur v2 (`docs/stories.md:174`). Le pilote corrige le critère en « le lecteur aiguille sur le numéro de version ; une version inconnue est refusée » (14/10/2026) ; le plan vise cette formulation.
- Le moteur n'a ni API ni dépendance : `crates/lophelia-core/src/lib.rs:1` ne contient que `VERSION`, `crates/lophelia-core/Cargo.toml:8` a un `[dependencies]` vide. Lire et écrire `positions.json` demande une bibliothèque : **ADR-0008, acceptée par Mathieu le 14/10/2026** — `serde`/`serde_json` dans le cœur pour le fichier seul, tableaux typés à la frontière WASM.
- `crates/lophelia-wasm/Cargo.toml` n'est **pas** dans les `Owns` (`docs/stories.md:33`) : le crate WASM ne reçoit aucune dépendance. `wasm-bindgen 0.2.129` suffit pour les tableaux typés (sonde, Verified interfaces) ; toute conversion (index, réglages, fichier) vit dans `lophelia-core`, et `crates/lophelia-wasm/src/lib.rs` ne fait que relayer (« appelé tel quel », `docs/stories.md:42`).
- `serde_json` 1.0.154 sans la fonction `float_roundtrip` ne relit pas exactement ce qu'il écrit : 59 533 écarts sur 200 000 `f64` tirés au hasard ; avec `float_roundtrip`, 0 écart (sonde hors dépôt, `/tmp/s2probe`, 14/10/2026). Sans elle, le critère 4 (`docs/stories.md:39`) échoue.
- Le workspace racine `Cargo.toml` n'est pas dans les `Owns` : toute dépendance se déclare dans `crates/lophelia-core/Cargo.toml`, versions exactes (`AGENTS.local.md`, conventions).

## Story
Une API unique du moteur et un format `positions.json` versionné, lu et validé par le moteur (`docs/stories.md:27-45`). Critères :
1. Entrées : nœuds (identifiants opaques), arêtes, positions connues, nœuds modifiés, réglages ; sortie : (x, y, z) par nœud ; sans position connue = créé ; ni Obsidian ni DOM, vérifié en CI (le crate compile sans dépendance web).
2. Modifiés vide et tout connu → chaque nœud à sa position exacte d'entrée.
3. Graine fixe, entrées identiques, même plateforme → mêmes positions.
4. `positions.json` : champ de version, une ligne par nœud, trié par chemin ; écrire puis relire redonne exactement les positions.
5. Fichier invalide (nombre non fini, taille > borne, JSON cassé, version future) refusé avec une erreur nommée, sans panique ; version antérieure connue lue (voir faits).
6. Chemin malformé (absolu, `..`, vide) → fichier refusé, erreur nommée ; chemin sans nœud de l'hôte → ignoré à la lecture, retiré à l'écriture suivante.
7. Contrat décrit dans `docs/contracts/moteur.md`, appelé tel quel depuis le module WASM.
Notes : placement trivial des nœuds créés ; s4 et s8 remplacent le calcul sans changer l'API ; borne par défaut 10 Mo ; chemins opaques pour le moteur.

## Current code
- `crates/lophelia-core/src/lib.rs:1` — `pub const VERSION: &str = env!("CARGO_PKG_VERSION");`, rien d'autre ; pas de `src/contract/`, `src/positions/`, `tests/`.
- `crates/lophelia-wasm/src/lib.rs:3-6` — seul export `#[wasm_bindgen] pub fn engine_version() -> String` ; test natif `:12-15` qui compare à `lophelia_core::VERSION`.
- `crates/lophelia-wasm/Cargo.toml:9,12-13` — `crate-type = ["cdylib", "rlib"]`, dépend de `lophelia-core` (chemin) et `wasm-bindgen = "=0.2.129"`.
- `plugin/src/engine/wasm.ts:3,15-18` — n'importe que `engine_version` et `initSync` de la glue ; aucun appel de disposition côté plugin (s5/s6).
- `scripts/build-wasm.sh:24,28` — `cargo build --release --target wasm32-unknown-unknown -p lophelia-wasm` puis `wasm-bindgen --target web` : tout nouvel export `#[wasm_bindgen]` du crate apparaît dans `plugin/wasm-pkg/` sans autre changement.
- `.github/actions/build/action.yml:33-47` — `cargo fmt --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, tests des scripts ; `:49-51` `cargo audit`. Aucun contrôle « pas de dépendance web » aujourd'hui.
- `Cargo.lock:17-19` — `lophelia-core` sans dépendance ; `syn 3.0.9`, `proc-macro2`, `quote`, `unicode-ident` déjà verrouillés (`:35-74`).

## Attachment points
- `crates/lophelia-core/src/lib.rs` — déclarer `pub mod contract;` et `pub mod positions;` (`docs/architecture.md:55`).
- `crates/lophelia-core/src/contract/` et `src/positions/` — nouveaux modules (`docs/architecture.md:17`, partagés `:49`).
- `crates/lophelia-core/tests/contract/` — Cargo découvre un test d'intégration `tests/<nom>/main.rs` ; un fichier `tests/contract.rs` serait hors `Owns`. Ces tests ne voient que l'API publique.
- `crates/lophelia-wasm/src/lib.rs` — nouveaux exports `#[wasm_bindgen]`.
- `.github/actions/build/action.yml` — nouvelle étape de contrôle (`docs/architecture.md:57` cite `.github/workflows/`, mais toutes les étapes vivent dans l'action composite, `.github/workflows/ci.yml:18`).
- `docs/contracts/moteur.md` — à créer (`docs/architecture.md:23`).

## Verified interfaces
- `cargo tree --locked -p lophelia-core -e normal,build --target all --prefix none --format '{p}'` rend aujourd'hui la seule ligne `lophelia-core v0.1.0 (…)` ; sur `lophelia-wasm`, il liste `wasm-bindgen`, `wasm-bindgen-macro`, `wasm-bindgen-shared` (exécuté le 14/10/2026). `--target all` est nécessaire : sans lui, une dépendance sous `cfg(target_arch = "wasm32")` échapperait au contrôle.
- Cache cargo local : `serde-1.0.229`, `serde_derive-1.0.229`, `serde_core-1.0.229`, `serde_json-1.0.154`, `itoa-1.0.18`, `memchr-2.8.3` présents ; `serde_json 1.0.154` tire aussi `zmij 1.0.23` (arbre de la sonde). Construction `--offline` réussie.
- `serde_json 1.0.154` (sonde) : `1e999` et `-1e400` → erreur `Syntax`, message `number out of range at line … column …` ; `NaN` → `Syntax`, `expected value` ; `1e-400` → `0.0` accepté ; JSON tronqué → `Eof`. `serde_json::Error` n'expose pas de code public : distinguer « nombre non fini » de « JSON cassé » passe par le message ou par une lecture propre des nombres.
- `wasm-bindgen 0.2.129` : un export rendant `Result<String, JsError>` lève une exception JS. `JsError::new` appelle JS : sur cible native (tests `cargo test`), l'appeler panique. Les tests du crate WASM ne doivent donc toucher qu'une fonction interne pure, comme le fait `engine_version_is_the_core_version` (`crates/lophelia-wasm/src/lib.rs:13`).
- `rust-toolchain.toml:2` — Rust `1.99.0`, édition `2024` (`Cargo.toml:7`).
- `wasm-bindgen 0.2.129`, sonde hors dépôt (14/10/2026, supprimée) : sans autre crate, un `#[wasm_bindgen] struct` avec `constructor(ids: Vec<String>) -> Result<_, JsError>`, une méthode `(&[u32], &[u32], &[f64], &[u32], u32, &[f64]) -> Result<Vec<f64>, JsError>` et un struct de retour à getters `Vec<u32>`/`Vec<f64>` compilent pour `wasm32` ; la glue type `string[]`, `Uint32Array`, `Float64Array`. Le chemin `Ok` se teste en natif (`cargo test`). Un objet exporté se libère côté JS par `free()`.

## Traps
- **Précision des nombres JS** : une graine `u64` passée en JSON depuis JS perd ses bits au-delà de 2^53. Graine en `u32`.
- **`NaN` depuis JS** : `JSON.stringify(NaN)` donne `null` ; côté JSON le moteur reçoit `null`, pas `NaN`. Le contrôle « nombre fini » des positions connues vaut pour l'API typée ; en JSON, `null` est un type invalide.
- **Hachage stable** : `std::collections::hash_map::DefaultHasher` n'est pas garanti stable entre versions de Rust ; un placement dérivé de l'identifiant doit utiliser une fonction écrite dans le code (FNV-1a, splitmix64).
- **Tri** : « trié par chemin » doit être défini (ordre des octets UTF-8, celui de `str::cmp`), jamais une collation locale ; le documenter dans le contrat, sinon le plugin et la démo trieront autrement.
- **`..`** : « contenant `..` » vise un segment de chemin ; une note `a..b.md` est légitime. Segment égal à `..` après découpe sur `/` (et `\`).
- **Borne de taille** : contrôlée sur la longueur en octets **avant** l'analyse, sinon 10 Mo sont alloués avant le refus. « 10 Mo » : à fixer (10 × 1024 × 1024 ou 10 000 000) et écrire dans le contrat.
- **Doublons** : un même chemin deux fois dans le fichier ; un tableau d'entrées permet de le refuser, un objet JSON l'écraserait en silence.
- **Taille du bundle** : les nouveaux exports restent dans le `.wasm` même si le plugin ne les appelle pas encore ; `main.js` grossit (taille notée par la CI, `.github/actions/build/action.yml:65-68`).
- **Couplage futur** : les valeurs par défaut des forces vivront avec `Settings` dans `src/contract/` ; s4 ne possède pas ce dossier (`docs/stories.md:74`) et ne pourra pas les régler sans repasser par une story de contrat.
- Tests lancés : `cargo test --workspace --locked` et `node --test 'scripts/*.test.mjs'` (`AGENTS.local.md`, `Test`). Clippy `-D warnings` sur `--all-targets` : les tests aussi doivent être propres.
- Issues #42 et #43 touchent `.github/` et `scripts/` : hors de cette story.
- Aucune migration ; `.lophelia/positions.json` n'existe nulle part encore (le plugin n'écrit rien, s6).

## Open questions
- Forme exacte de la frontière par tableaux typés (ordre des arguments, marquage des positions connues) : fixée par le plan, ADR-0008.
- Un nœud modifié qui a une position connue : s2 le laisse à sa place (le calcul arrive avec s8) — à écrire dans le contrat comme comportement provisoire.
- Un identifiant de `changed` inconnu de `nodes`, une arête vers un nœud inconnu : refus nommé (choix du plan) plutôt qu'ignorés.
- Champs inconnus dans une entrée v1 : refusés (lecture stricte), au prix d'une version 2 pour tout ajout — c'est ce que s9 fait déjà.
- La borne de taille est-elle un paramètre de l'hôte ? Le plan la garde en constante exposée, passée explicitement à la lecture.

## Complexity
3 → 3. Peu de code, mais trois pièges qui cassent un critère sans bruit (relecture exacte des flottants, classement des erreurs de nombre, absence de dépendance web), et un choix de bibliothèque soumis à une personne.

## Split
—
