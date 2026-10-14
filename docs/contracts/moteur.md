# Contrat du moteur

Ce contrat est la seule façon dont un hôte (le plugin Obsidian, puis la page de démo) parle au moteur. Il est défini dans `crates/lophelia-core` (`src/contract/`, `src/positions/`) et exposé tel quel par le module WASM (`crates/lophelia-wasm`). Choix de la frontière : [ADR-0008](../decisions/0008-json-contrat-moteur.md).

Le moteur ne connaît ni Obsidian ni le DOM ; la CI échoue si `lophelia-core` dépend d'un crate web (`wasm-bindgen`, `js-sys`, `web-sys`, `gloo`, `obsidian`).

## L'objet `Engine`

| JavaScript (module WASM) | Rust (`lophelia-core`) | Rôle |
| --- | --- | --- |
| `new Engine(ids: string[])` | `Graph::new(ids: Vec<String>)` | enregistre les identifiants des nœuds, une seule fois |
| `engine.node_count(): number` | `Graph::node_count()` | nombre de nœuds |
| `engine.layout(edges, known_nodes, known_xyz, changed, seed, forces): Float64Array` | `Settings::from_parts(seed, forces)` puis `Graph::layout(&LayoutInput)` | une position (x, y, z) par nœud |
| `engine.read_positions(file: string): KnownPositions` | `positions::read(text, &graph, DEFAULT_MAX_BYTES)` | lit `positions.json` |
| `engine.write_positions(xyz: Float64Array): string` | `positions::write(&graph, xyz)` | écrit `positions.json` |
| `known.nodes: Uint32Array`, `known.xyz: Float64Array` | `positions::Known { nodes, xyz }` | positions connues, à passer telles quelles à `layout` |
| `default_forces(): Float64Array` | `Settings::default().forces()` | les quatre forces par défaut |
| `engine.free()`, `known.free()` | — | libère la mémoire WASM de l'objet |

Un `Engine` et un `KnownPositions` vivent dans la mémoire du module WASM : l'hôte appelle `free()` dès qu'il n'en a plus besoin (par exemple quand la liste des notes change et qu'il construit un nouvel `Engine`).

## Identifiants et index

- Les identifiants sont opaques pour le moteur ; pour le plugin, c'est le chemin de la note relatif au vault.
- Le nœud d'index `i` est `ids[i]`, dans l'ordre donné au constructeur. Un identifiant donné deux fois est refusé (`duplicate_node`). Après la construction, tout passe par index : l'hôte garde l'ordre de `ids`. Un index hors bornes est refusé ; un index valide mal attribué ne peut pas l'être.
- `edges` (`Uint32Array`) : paires (source, cible) mises à plat, `[s0, t0, s1, t1, …]` ; longueur paire.
- `known_nodes` (`Uint32Array`) et `known_xyz` (`Float64Array`) : `known_xyz` porte trois valeurs par entrée de `known_nodes`, x, y, z de `known_nodes[k]` aux places `3k..3k+3`. Un index au plus une fois.
- `changed` (`Uint32Array`) : les nœuds modifiés (créés ou dont les liens ont changé).
- Sortie de `layout` (`Float64Array`) : `3 × node_count` valeurs, x, y, z du nœud `i` aux places `3i..3i+3`. `write_positions` prend ce même tableau.

## Réglages

- `seed` : entier `u32` (un nombre JS entier de 0 à 2³² − 1, sans perte).
- `forces` : exactement quatre nombres, dans cet ordre : **centre, répulsion, force des liens, distance des liens**. Toute autre longueur est refusée (`malformed_input`), un nombre non fini aussi (`non_finite`).
- Valeurs par défaut **provisoires** (s4 les règle) : centre `0.05`, répulsion `30`, force des liens `1`, distance des liens `30`.

## Règles de `layout`

- Un nœud avec une position connue la garde exactement, au bit près. C'est vrai qu'il soit dans `changed` ou non : **comportement provisoire** de s2, s8 ajustera les nœuds modifiés sans changer cette signature.
- En particulier, `changed` vide et tous les nœuds connus : chaque nœud ressort à sa position d'entrée.
- Un nœud sans position connue est un nœud **créé**. Placement trivial (s4 le remplace) : un générateur `splitmix64` initialisé par `seed ⊕ fnv1a64(identifiant)` tire trois valeurs dans `[−h, h)`, avec `h = distance des liens × max(1, ∛n)` et `n` le nombre de nœuds. La position ne dépend que de la graine, de l'identifiant, de `n` et de la distance des liens, pas de l'ordre des index.
- Reproductibilité : à graine fixe et entrées identiques, deux appels **sur une même plateforme** rendent les mêmes positions (`docs/brief.md`, Interface contract) ; rien n'est promis d'une plateforme à l'autre.
- Refus : voir la table des erreurs. Aucune entrée ne fait paniquer le moteur.

## Format `positions.json`, version 1

Écrit par `write_positions`, octet par octet :

```
{"version":1,"positions":[
{"path":"a.md","x":0.1,"y":-0.0,"z":1e+300},
{"path":"b/c.md","x":-5e-324,"y":2.5,"z":3.0}
]}
```

- Première ligne `{"version":1,"positions":[`, puis **une ligne par nœud** `{"path":…,"x":…,"y":…,"z":…}` suivie de `,` sauf la dernière, puis `]}` et un saut de ligne final. Sans nœud : `{"version":1,"positions":[`, `]}`.
- Chemins encodés en chaîne JSON (échappements de `serde_json`), nombres dans leur écriture la plus courte qui se relit exactement (exposant écrit `e+300`, `e-324`).
- Lignes **triées par chemin, dans l'ordre des octets UTF-8** (`str::cmp` de Rust), jamais selon une collation locale : `Z.md` avant `a.md` avant `é.md`.
- Seuls les nœuds de l'`Engine` sont écrits : un chemin qui n'a plus de nœud disparaît à l'écriture suivante.
- Écrire puis relire redonne exactement les mêmes `f64`, au bit près.

Lecture par `read_positions`, dans cet ordre :

1. Taille en octets au-delà de la borne → `too_large`, **avant** toute analyse. Borne : `DEFAULT_MAX_BYTES` = 10 Mio = 10 × 1024 × 1024 = 10 485 760 octets.
2. Lecture de l'en-tête seul : JSON cassé → `invalid_json` ; champ `version` absent ou non entier → `missing_version`.
3. **Aiguillage sur le numéro de version** : `1` → lecteur v1 ; tout autre numéro → `unsupported_version`. La lecture des versions antérieures par une version plus récente arrive avec s9, qui ajoute sa branche.
4. Lecteur v1 strict : objet `{"version":1,"positions":[…]}`, chaque entrée exactement `path`, `x`, `y`, `z` ; un champ inconnu est refusé (`invalid_json`). Un nombre hors des `f64` finis (`1e999`) → `non_finite_number`.
5. Chaque chemin est validé avant tout filtrage. Malformé → `malformed_path` : vide, commençant par `/` ou `\`, de la forme `X:` (lettre de lecteur), ou dont un segment (découpe sur `/` et `\`) vaut exactement `..`. `a..b.md` et `d/..x/n.md` sont légitimes. Un chemin deux fois → `duplicate_path`.
6. Un chemin qui ne correspond à aucun nœud de l'`Engine` est ignoré ; les autres deviennent `known.nodes` (index) et `known.xyz`, dans l'ordre du fichier.

Un fichier refusé n'est jamais réécrit par le moteur : c'est à l'hôte de le signaler et de le mettre de côté (`docs/brief.md`, Security & abuse).

## Erreurs

Côté JavaScript, chaque refus lève une exception `Error` dont le message est `code: détail`, par exemple `unsupported_version: unsupported format version 2`. Le **code** est stable ; le détail est pour un humain et peut changer. Côté Rust, `ContractError::code()` et `PositionsError::code()` rendent le même code.

| Code | Erreur Rust | Quand |
| --- | --- | --- |
| `duplicate_node` | `ContractError::DuplicateNode(id)` | identifiant deux fois au constructeur, ou index deux fois dans `known_nodes` |
| `index_out_of_range` | `ContractError::IndexOutOfRange(index)` | index ≥ `node_count` dans `edges`, `known_nodes` ou `changed` |
| `malformed_input` | `ContractError::MalformedInput(quoi)` | `edges` de longueur impaire, `known_xyz` qui n'a pas 3 valeurs par nœud connu, `forces` qui n'a pas 4 valeurs |
| `non_finite` | `ContractError::NonFinite(quoi)` | coordonnée connue ou réglage `NaN` ou infini |
| `too_large` | `PositionsError::TooLarge { size, max }` | fichier au-delà de la borne |
| `invalid_json` | `PositionsError::InvalidJson(détail)` | JSON cassé ou tronqué, forme v1 non respectée, champ inconnu |
| `missing_version` | `PositionsError::MissingVersion` | pas de champ `version` entier |
| `unsupported_version` | `PositionsError::UnsupportedVersion(n)` | numéro de version inconnu de ce lecteur |
| `non_finite_number` | `PositionsError::NonFiniteNumber(détail)` | nombre hors des `f64` finis, à la lecture ou dans le tableau donné à `write_positions` |
| `malformed_path` | `PositionsError::MalformedPath { path, reason }` | chemin vide, absolu, à lettre de lecteur, ou avec un segment `..`, à la lecture comme à l'écriture |
| `duplicate_path` | `PositionsError::DuplicatePath(path)` | même chemin deux fois dans le fichier |
| `length_mismatch` | `PositionsError::LengthMismatch { expected, actual }` | `xyz` de `write_positions` qui n'a pas `3 × node_count` valeurs |

## Compatibilité

- `positions.json` est versionné : une version récente du moteur lit les anciennes, jamais l'inverse en silence — une version inconnue est refusée, pas devinée (`docs/brief.md`, Interface contract).
- Tout ajout au format v1 (un champ de plus) passe par une version 2, puisque le lecteur v1 refuse les champs inconnus.
- L'API du moteur passe en semver dès qu'un second hôte existe.
