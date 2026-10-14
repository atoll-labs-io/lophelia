---
validated: yes
---
# Plan — s3-banc-evaluation

[Story](../stories.md) · [Research](../research/s3-banc-evaluation.md)

Branch `story/s3-banc-evaluation`. Read `docs/research/s3-banc-evaluation.md` first; this plan builds on it without repeating it.

Validé sans relecture humaine (`Plan validation: autonomous`), après contrôle critère par critère (fin du fichier). Aucune dépendance nouvelle, aucun ADR.

## Story
Un crate CLI `lophelia-bench` qui note une disposition (silhouette des communautés en 3D, stabilité 3a/3b, temps de calcul) sur des graphes figés, plus deux outils : `snapshot` (vault → graphe local) et `vault` (graphe → vault de test). Critères `docs/stories.md:56-63`, repris dans la recherche. **Critère 8 lu ainsi** : deux exécutions identiques donnent les mêmes notes et les mêmes fichiers ; seul le temps de calcul varie (correction de `docs/stories.md:63` signalée au pilote).

## Reuse
| New code | Rung that holds | Why each rung above fails |
| --- | --- | --- |
| Appel du moteur, aller-retour par `positions.json` | dépôt : `Graph::new`, `Graph::layout`, `positions::write`/`read` (`crates/lophelia-core/src/contract/mod.rs:108`, `:143` ; `positions/mod.rs:102`, `:142`) | — |
| Lecture/écriture JSON (graphes, scénarios, références, `app.json`) | dépendance installée : `serde` + `serde_json` (`Cargo.lock:76`, `:106`), mêmes versions exactes que le cœur | rien dans le dépôt pour ces formats ; `std` ne lit pas le JSON |
| Écriture « une ligne par entrée » | reprise du motif de `positions::write` (`positions/mod.rs:142-`) : `serde_json::to_string` par valeur, lignes assemblées à la main | `to_string_pretty` met un champ par ligne |
| Arguments de la CLI | `std::env::args` | aucun crate CLI installé ; cinq sous-commandes ne justifient pas une dépendance |
| Générateur pseudo-aléatoire | nouveau code : `splitmix64` (≈8 lignes) | celui du cœur est privé (`contract/mod.rs:214`) et `contract/` hors `Owns` ; `std` n'en a pas ; `rand` serait une dépendance |
| Silhouette, disposition naïve, 3a/3b | nouveau code | aucune bibliothèque installée ne les fournit |
| Détection de communautés (Louvain) | nouveau code, `std` seul | aucun crate installé ; un crate serait une dépendance nouvelle, donc un ADR (recherche, Open questions) |
| Parcours du vault, liens `[[…]]`, filtres d'exclusion | `std::fs` + nouveau code (pas de regex) | aucun crate installé ; `regex` serait une dépendance |
| Contrôle « rien de versionné sous `bench/local/` » | outil installé : `git ls-files` dans une étape de l'action composite | `scripts/` hors `Owns` |

## Tasks
Tous les fichiers Rust nouveaux sont sous `crates/lophelia-bench/` ; erreurs : un `enum BenchError` avec `Display`, `std::error::Error` et `code() -> &'static str` stable, comme `ContractError` (`contract/mod.rs:16`). Aucun `unwrap`/`expect`/`panic!` sur une donnée lue d'un fichier. Tout parcours qui produit une sortie passe par des `Vec` triés ou des `BTreeMap`, jamais par l'ordre d'itération d'un `HashMap`/`HashSet`. Tests : binaire d'intégration `crates/lophelia-bench/tests/bench/main.rs` qui déclare un module par thème (habitude de `crates/lophelia-core/tests/contract/main.rs:1-2`) ; les tests écrivent seulement sous `std::env::temp_dir()` (dossier unique par test, supprimé à la fin).

1. [x] **Crate et branchements.**
   - `Cargo.toml` : `members` += `"crates/lophelia-bench"` ; `[profile.dev.package.lophelia-bench] opt-level = 3` (le critère 2 lance `cargo run` en debug ; la disposition naïve est en O(n²)). `[profile.release]` inchangé.
   - `crates/lophelia-bench/Cargo.toml` : champs `.workspace = true` comme le cœur, `publish = false` ; dépendances `lophelia-core = { path = "../lophelia-core" }`, `serde = { version = "=1.0.229", features = ["derive"] }`, `serde_json = { version = "=1.0.154", features = ["float_roundtrip"] }`. `Cargo.lock` mis à jour une fois sans `--locked` : seule l'entrée `lophelia-bench` apparaît.
   - `src/lib.rs` (modules publics), `src/main.rs` (aiguillage : `generate`, `stability`, `snapshot`, `vault`, sinon `<graphe> [--record]` ; usage sur stderr et code 2 si les arguments ne collent pas ; erreur → `erreur : <code>: <détail>` sur stderr, code 1). `repo_root()` = `env!("CARGO_MANIFEST_DIR")/../..` canonicalisé.
   - `.gitignore` += `bench/local/`. `.github/actions/build/action.yml` : étape « Nothing tracked under bench/local », juste après le contrôle des blobs : `tracked="$(git ls-files bench/local)"`, liste et `exit 1` si non vide.
   Vérification : `cargo build -p lophelia-bench --locked`, clippy `--all-targets -D warnings` propre ; `git ls-files bench/local` vide.

2. [x] **Format de graphe, `src/graph.rs`.** Fichier JSON, version 1 :
   ```
   {"version":1,"nodes":[
   {"id":"c00/n00-0000.md","community":0},
   …
   ],"edges":[
   [0,5],
   …
   ]}
   ```
   Une ligne par nœud puis une ligne par arête, nœuds triés par `id` (octets UTF-8), arêtes non orientées `[a,b]` avec `a < b`, triées, sans doublon. `struct BenchGraph { ids: Vec<String>, communities: Vec<u32>, edges: Vec<(u32, u32)> }` ; `read_graph(text) -> Result<BenchGraph, BenchError>` (`deny_unknown_fields`, version ≠ 1 refusée, index hors bornes, boucle, doublon d'arête ou d'identifiant refusés par une erreur nommée) et `write_graph(&BenchGraph) -> String` ; `BenchGraph::flat_edges() -> Vec<u32>` pour `LayoutInput`.
   Tests (`tests/bench/graph.rs`) : (a) écrire puis relire un petit graphe redonne la même valeur ; (b) arête `[0,9]` sur 3 nœuds → erreur `edge_out_of_range`, arête `[1,1]` → `self_loop`.

3. [x] **Graphes synthétiques, `src/rng.rs` + `src/generate.rs`, `bench/graphs/`.** Modèle à blocs stochastiques, paramètres en `const` : `sbm-100` (4 communautés), `sbm-1000` (10), `sbm-2500` (20) ; tailles inégales déterministes (poids `1 + c/(k−1)`, reste à la dernière) ; degré attendu ≈ 6 dans la communauté (`p_in = 6/(taille−1)`), ≈ 1 hors (`p_out = 1/(n−taille)`) ; graine fixe par graphe. Paires `i < j` parcourues dans l'ordre, un tirage `splitmix64` chacune. Identifiants `cKK/nKK-JJJJ.md` (`JJJJ` = rang dans la communauté) : noms de note uniques dans tout le vault, et `c00/n00-0000.md`…`c03/n03-0009.md` existent dans les trois graphes. Sous-commande `generate` : écrit les trois fichiers dans `bench/graphs/` ; les trois fichiers générés sont commités.
   Test (`tests/bench/generate.rs`) : (c) pour chaque taille, `write_graph(generate(spec))` est égal octet pour octet au fichier commité (`include_str!`) — critère 1, et preuve que la génération est déterministe.

4. [x] **Mesures, `src/metrics.rs` + `src/naive.rs`.**
   - `silhouette(xyz: &[f64], communities: &[u32]) -> Option<f64>` : silhouette moyenne, distance euclidienne en 3D, `s(i) = 0` pour un nœud seul dans sa communauté ; `None` si moins de 2 communautés ou de 2 nœuds. O(n²), suffisant à 2 500.
   - `naive_layout(n, edges) -> Vec<f64>` : positions initiales `splitmix64` à graine fixe dans un cube de côté ∝ ∛n ; 300 itérations ; ressort linéaire sur chaque arête vers une longueur de repos, répulsion uniforme en `1/d²` entre toutes les paires, pas borné et refroidi linéairement. Toutes les constantes en `const`, nommées, dans ce fichier (critère 3).
   Tests (`tests/bench/metrics.rs`) : (d) quatre points en deux paires éloignées, valeur calculée à la main, à 1e-12 près ; (e) une seule communauté → `None` ; (f) deux cliques de 6 reliées par une arête : `silhouette(naive_layout(..))` > 0,5 (tombe si les ressorts ou la répulsion manquent).

5. [x] **Notation, `src/evaluate.rs` — `cargo run -p lophelia-bench -- <graphe> [--record]`.** `evaluate(graph, references_dir, stem) -> Result<Evaluation, BenchError>` :
   - moteur : `Graph::new(ids)`, `layout` sans position connue, `changed` vide, `Settings::default()` (graine 0), chronométré par `Instant` autour du seul `layout` ; sortie non finie → erreur `non_finite_layout` (issue #54) ;
   - `Evaluation { nodes, edges, communities, engine_silhouette, naive_silhouette, reference_gap: Option<f64>, elapsed: Duration }` ;
   - référence : `<references_dir>/<stem>.json` = `{"silhouette": <f64>}` ; présente → `reference_gap = engine − référence` ; absente → `None`. `--record` écrit ce fichier (dossier créé au besoin) avec la silhouette du moteur — c'est par là que s4, qui ne possède pas ce crate, enregistre la sienne. `main.rs` passe `repo_root()/bench/references` et le nom du fichier graphe sans `.json`.
   - affichage (une ligne chacun, silhouettes et écart en `{:.6}`) : `graphe : <stem> (<n> nœuds, <m> arêtes, <k> communautés)`, `silhouette moteur`, `silhouette naïve`, `temps de calcul du moteur : <ms> ms`, puis `écart à la référence : <±x>` ou `référence : aucune`.
   Test (`tests/bench/evaluate.rs`) : (g) sur `sbm-100` dans un dossier de références temporaire : sans fichier → `reference_gap == None` ; après `--record` (fonction d'écriture) → `Some(0.0)` ; référence réécrite à `0.25` → `Some(engine − 0.25)`.

6. [x] **Stabilité, `src/stability.rs` — `stability <graphe> <scénario>`, `bench/scenarios/base.json`.** Scénario JSON `{"version":1,"steps":[…]}`, chaque étape exactement une de : `{"link":["a.md","b.md"]}`, `{"unlink":["a.md","b.md"]}`, `{"create":"x.md","links":["a.md",…]}` ; la note modifiée est la première du couple, ou la note créée. Refus nommés : identifiant inconnu, lien déjà présent, lien absent à retirer, note créée déjà existante.
   Déroulé, par le chemin réel d'un hôte : disposition initiale sans position connue ; `text = positions::write`. Pour chaque étape :
   - **3a** : `known = positions::read(text)`, `layout` avec tout connu et `changed` vide ; écart = distance euclidienne maximale entre ancienne et nouvelle position ;
   - modification appliquée (nouveau `Graph` si création), `known = positions::read(text, &nouveau_graphe)`, `changed` = note modifiée + autres extrémités des liens touchés, `layout` ;
   - **3b** : `half_farthest_displacement(before, after, modified) -> Result<f64, BenchError>` : rayon `R` = distance maximale au barycentre de `before` (`R = 0` → erreur `zero_radius`) ; centre = position de la note modifiée (dans `before`, ou `after` si créée) ; nœuds présents des deux côtés hors note modifiée, triés par distance au centre décroissante (égalité : index croissant), première moitié (`⌊m/2⌋`, au moins 1) ; valeur = déplacement maximal de cette moitié / `R` × 100 ;
   - `text = positions::write(after)`.
   Affichage : une ligne par étape `étape <i> (<type> <notes>) : 3a = <écart, Display de f64> ; 3b = <x.xxxxxx> % du rayon`, puis `3a max` et `3b max`. `base.json` : lier `c00/n00-0000.md`–`c01/n01-0000.md`, lier `c02/n02-0001.md`–`c03/n03-0001.md`, retirer le premier lien, créer `c00/n00-new.md` lié à `c00/n00-0002.md` et `c00/n00-0003.md` ; si un de ces liens existe déjà dans un graphe généré, choisir un autre couple parmi les identifiants communs aux trois.
   Tests (`tests/bench/stability.rs`) : (h) `half_farthest_displacement` sur 5 points posés à la main (le plus lointain bouge, le plus proche aussi) → valeur exacte attendue, et le déplacement du nœud proche n'y entre pas ; (i) `unlink` d'un lien absent → `link_missing`, identifiant inconnu → `unknown_node` ; (j) `base.json` sur les trois graphes : une ligne par étape, 3a = 0 à chaque étape (le moteur relit ses positions exactement).

7. [x] **Communautés, `src/louvain.rs`.** `louvain(n: usize, edges: &[(u32, u32)]) -> Vec<u32>` : Louvain classique (déplacements locaux jusqu'à gain nul, puis agrégation, niveaux répétés tant que la modularité monte), nœuds visités par index croissant, égalité de gain tranchée par la plus petite communauté, étiquettes finales renumérotées par ordre de première apparition. Aucun aléa.
   Test (`tests/bench/louvain.rs`) : (k) deux cliques de 5 reliées par une arête, plus un nœud isolé → trois communautés, exactement les deux cliques et le nœud seul.

8. [x] **Instantané, `src/snapshot.rs` — `snapshot <dossier-vault>`.** `snapshot(vault: &Path) -> Result<(BenchGraph, Vec<String>), BenchError>` (graphe, avertissements) :
   - filtres : `.obsidian/app.json` absent → aucun ; `userIgnoreFilters` lu comme liste de chaînes ; une chaîne `/…/` → avertissement « filtre regex non appliqué » ; les autres excluent tout chemin relatif qui commence par elles ;
   - parcours récursif trié, sans suivre les liens symboliques, en sautant tout nom commençant par `.` ; fichiers `.md` seulement ; chemin relatif en `/` ; nom non UTF-8 → avertissement et ignoré ; contenu lu avec `from_utf8_lossy` ;
   - liens : chaque `[[…]]` ou `![[…]]` d'une seule ligne ; cible = avant `|`, puis avant `#`, espaces retirés, `.md` final retiré ; cible vide ignorée ;
   - résolution, sans tenir compte de la casse : cible avec `/` → notes dont le chemin sans `.md` vaut la cible ou finit par `/<cible>` ; sinon notes dont le nom sans `.md` vaut la cible ; plusieurs candidates → chemin le plus court, puis ordre des octets ; aucune → lien ignoré ; une note exclue n'est ni nœud ni cible ;
   - arêtes non orientées, sans boucle ni doublon ; communautés par `louvain`.
   `main.rs` écrit `write_graph` dans `repo_root()/bench/local/<nom du dossier du vault>.json` (dossier créé), affiche nœuds, arêtes, communautés et les avertissements.
   Tests (`tests/bench/snapshot.rs`, vaults écrits à la main dans un dossier temporaire) : (l) `[[B]]`, `[[b|alias]]`, `![[B#titre]]`, `[[dir/C.md]]`, `[[absente]]` → exactement les arêtes A–B et A–C ; (m) `app.json` avec `"priv/"` et `"/x/"` : `priv/n.md` et `.cache/n.md` absents, le filtre regex donne un avertissement ; (n) deux `X.md` (`X.md` et `d/X.md`) : `[[X]]` va vers `X.md`.

9. [x] **Vault de test, `src/vault.rs` — `vault <graphe> <dossier>`.** `write_vault(graph, dir, repo_root) -> Result<(), BenchError>` : le parent de `dir` doit exister ; `canonicalize(parent)/nom` dans `canonicalize(repo_root)` → `inside_repository` ; `dir` existant et non vide → `target_not_empty`. Une note par nœud au chemin de son identifiant (sous-dossiers créés), contenu `# <nom>` puis une ligne `[[<nom sans .md>]]` par arête, écrite dans la note de l'extrémité de plus petit index ; `node_modules/exclue.md` liée au premier nœud ; `.obsidian/app.json` = `{"userIgnoreFilters":["node_modules/"]}`.
   Tests (`tests/bench/vault.rs`) : (o) `dir` = `repo_root/bench/<nom unique>` → `inside_repository`, et rien n'est créé ; (p) dossier temporaire contenant un fichier → `target_not_empty` ; (q) `write_vault(sbm-100)` puis `snapshot` → mêmes identifiants et mêmes arêtes que le graphe, aucun identifiant sous `node_modules/`.

10. [x] **Reproductibilité et documentation.**
    - `bench/README.md` (français, court) : les cinq commandes, les formats (graphe, scénario, référence), « deux exécutions → mêmes notes, seul le temps varie », le profil release `opt-level = "z"` qui s'applique au moteur mesuré, l'interdiction de versionner `bench/local/`.
    - Test (`tests/bench/replay.rs`) : (r) critère 8 — `evaluate` deux fois sur `sbm-100` → `Evaluation` égales hors `elapsed` (bit à bit) ; `stability` deux fois → rapports égaux ; `snapshot` deux fois du même vault généré → `write_graph` identique octet pour octet (attrape un ordre de `HashMap`).
    - Lancer à la main `cargo run -p lophelia-bench -- bench/graphs/sbm-2500.json` puis `stability … bench/scenarios/base.json` et noter dans `docs/verif/s3-banc-evaluation.md` les silhouettes moteur/naïve des trois graphes et la durée du run (repère pour les seuils de s4).

## Scene viewpoints
—

## Run interdicts
- `git diff main...story/s3-banc-evaluation -- crates/lophelia-core crates/lophelia-wasm plugin scripts docs/contracts` est vide.
- `Cargo.lock` ne gagne que le paquet `lophelia-bench` ; aucun autre `[[package]]` ajouté ou changé.
- `[profile.release]` de `Cargo.toml` inchangé.
- `git ls-files bench/local bench/references` est vide (les références sont à s4).
- Aucune donnée de `~/dev` (chemin, nom de note, instantané) dans un fichier versionné ni lue par un test ; ne pas lancer `snapshot` sur `~/dev` pendant la story.
- Pas de `HashMap`/`HashSet` dont l'ordre d'itération atteint une sortie (grep `HashMap`, `HashSet` dans `crates/lophelia-bench/src`).
- Pas de nouvelle dépendance (ni `rand`, `clap`, `regex`, crate de graphes) ; si une s'avérait nécessaire, s'arrêter et le rapporter : c'est un ADR.

## The pivot
Tout repose sur **les définitions des mesures** : s4 juge sa disposition sur la silhouette contre la naïve, s8 sur 3b. Trois façons d'avoir tort :
1. **3b mal défini** — distance à la note modifiée prise avant la modification, rayon au barycentre, maximum sur la moitié la plus éloignée : à comparer au brief (`docs/brief.md:61`, « la moitié des nœuds les plus éloignés … bougent de moins de 1 % du rayon ») et à `docs/stories.md:150-154` (s8 ajoutera ses quarts de distance sur la même base).
2. **Disposition naïve trop faible ou trop forte** — le seuil de s4 (« dépasse d'au moins 0,1 la naïve et vaut au moins 0,3 ») devient trivial ou impossible ; les silhouettes naïves des trois graphes notées à la tâche 10 permettent de le voir avant s4.
3. **Louvain fait main faux** — un instantané réel noté sur de mauvaises communautés ; le test (k) n'en couvre qu'un cas, la modularité finale peut être affichée à titre de contrôle.

## Files touched
`Cargo.toml`, `Cargo.lock`, `.gitignore`, `.github/actions/build/action.yml`, `crates/lophelia-bench/{Cargo.toml, src/{main,lib,graph,rng,generate,metrics,naive,evaluate,stability,louvain,snapshot,vault}.rs, tests/bench/{main,graph,generate,metrics,evaluate,stability,louvain,snapshot,vault,replay}.rs}`, `bench/graphs/sbm-{100,1000,2500}.json`, `bench/scenarios/base.json`, `bench/README.md`, plus `docs/research/`, `docs/plans/`, `docs/verif/` de la story.

## Test strategy
Couche « service » seulement : fonctions pures de `lophelia-bench`, appelées directement ; `main.rs` n'est qu'un adaptateur et n'a pas de test propre (son aiguillage est vérifié à la main, tâche 10). 18 tests, sous le budget de 25 : (a)–(b) format, (c) génération, (d)–(f) mesures, (g) référence, (h)–(j) stabilité, (k) Louvain, (l)–(n) instantané, (o)–(q) vault, (r) rejeu. Chaque test nomme une règle qui, retirée, le fait tomber (ex. (f) sans ressorts, (n) sans tri par longueur, (r) avec un `HashMap` itéré). Pas de test de l'étape CI `bench/local` : le relecteur la vérifie en lisant l'action et `git ls-files`.
Contrôles manuels : `cargo run -p lophelia-bench -- <graphe>` sur les trois graphes, `stability`, `vault` puis `snapshot` sur ce vault, sortie lisible. Tests d'une tâche : `cargo test -p lophelia-bench --locked --test bench <filtre>` ; suite complète et typecheck via `po-run` en fin d'exécution.

## Done when
- les dix tâches cochées, `cargo fmt --check` et clippy `--workspace --all-targets -D warnings` propres, `Test` vert ;
- les trois graphes, le scénario et le README commités, rien sous `bench/local/` ni `bench/references/` ;
- la CI passe, y compris la nouvelle étape ;
- un commit `s3-banc-evaluation: …` avec la recherche, le plan et le relevé de vérification.

### Contrôle des critères (validation autonome)
1 → tâche 3 (c) · 2 → tâche 5 (g) + affichage · 3 → tâches 4 (f) et 5 · 4 → tâche 6 (h)–(j) · 5 → tâche 1 (`.gitignore`, étape CI) et 8 (écriture dans `bench/local/`) · 6 → tâche 8 (l)–(n) · 7 → tâche 9 (o)–(q) · 8 → tâche 10 (r), dans la lecture corrigée (temps exclu).
