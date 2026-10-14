---
validated: yes
---
# Plan — s1-chaine-release

[Story](../stories.md) · [Research](../research/s1-chaine-release.md)

Branch `story/s1-chaine-release`. Read `docs/research/s1-chaine-release.md` first; this plan builds on it without repeating it.


Choix tranché par Mathieu le 14/10/2026, [ADR-0007](../decisions/0007-provenance-depot-prive.md) (`accepted`) : option 1, le dépôt est public et protégé. Le constat de départ : le dépôt est privé sur un plan Free, où l'attestation de provenance n'existe pas. Options classées : 1. rendre le dépôt public avant le premier tag `v*` ; 2. signature Sigstore dans le dépôt privé (réécrit l'ADR-0004 et le critère 2) ; 3. GitHub Enterprise Cloud ; 4. publier sans attestation tant que le dépôt est privé. Les tâches ci-dessous sont écrites pour l'option 1 ; elles valent aussi pour 3 et 4, seule l'option 2 change la tâche 9.

## Prérequis machine (avant `/po-execute`)
La cible `wasm32-unknown-unknown` manque sur le poste et `rustup` est absent (recherche, faits 2). Sans eux, l'implementer ne compile ni le WASM ni le bundle, et le typecheck du plugin (qui lit les `.d.ts` générés) échoue. Mathieu installe `rustup` (paquet `rustup` à la place de `rust`, ou `rustup-init` en espace utilisateur) ; le reste (`wasm-bindgen-cli`, `cargo-audit`) s'installe par `cargo install --locked` en espace utilisateur.

## Story
Prouver la chaîne Rust/WASM → plugin Obsidian → release attestée avant le moteur. Critères (`docs/stories.md:15-22`) : (1) tag `v*` → workflow qui construit le WASM et le plugin depuis les sources et publie `main.js`, `manifest.json`, `styles.css` ; (2) chaque artefact attesté, `gh attestation verify` réussit ; (3) aucun `.wasm` ni blob base64 versionné, CI en échec sinon ; (4) CI en échec si le bundle contient une API réseau ; (5) commande « Ouvrir Lophelia » → vue qui affiche la version du moteur renvoyée par le WASM ; (6) dépendances verrouillées, installation stricte, outils à version fixe ; (7) `cargo audit` et `npm audit` à chaque build, échec sur haut ou critique. Taille du bundle notée dans la PR, licence MIT.

## Reuse
Le dépôt n'a aucun code (ADR-0002) : le premier échelon, « déjà dans ce dépôt », ne tient jamais.

| New code | Rung that holds | Why each rung above fails |
| --- | --- | --- |
| Version du moteur | bibliothèque standard : `env!("CARGO_PKG_VERSION")` | dépôt vide |
| Liaison WASM | dépendance retenue par l'ADR-0003 : `wasm-bindgen` | dépôt vide ; la bibliothèque standard n'exporte rien vers JS |
| Chargement du WASM dans le plugin | plateforme : loader `binary` d'esbuild + `initSync` généré par wasm-bindgen | dépôt vide ; rien à écrire au-delà d'un import |
| Vue et commande | plateforme : `ItemView`, `addCommand` d'Obsidian (`docs/design-system.md:76-77`) | dépôt vide |
| Contrôle réseau, contrôle anti-blob | nouveau code, Node sans dépendance (`node:fs`, `node:child_process`, tests `node:test`) | dépôt vide ; aucun outil installé ne connaît ces motifs (un `grep` en YAML ne se teste pas et se trouverait lui-même) |
| Build WASM, vault de test | nouveau code, scripts shell | dépôt vide ; aucun outil n'enchaîne cargo, wasm-bindgen, wasm-opt et Obsidian |
| `wasm-opt` épinglé | dépendance npm `binaryen` (expose `wasm-opt`, recherche) | un binaire téléchargé à part échapperait au lockfile et à `npm ci` |
| CI, release | plateforme : GitHub Actions, `gh` du runner, actions de GitHub épinglées par SHA (ADR-0004) | une action tierce de release n'apporte rien que `gh release create` ne fasse |

## Tasks
1. [x] **Workspace Rust, licence, `.gitignore`.** `Cargo.toml` (`[workspace]` avec `crates/lophelia-core` et `crates/lophelia-wasm`, `[workspace.package] version = "0.1.0"`, `license = "MIT"`, profil release `opt-level = "z"`, `lto = true`) ; `rust-toolchain.toml` (canal exact `1.99.0`, la version du poste ; cible `wasm32-unknown-unknown` ; composants `rustfmt`, `clippy`) ; `crates/lophelia-core/src/lib.rs` : `pub const VERSION: &str = env!("CARGO_PKG_VERSION");`, aucune dépendance ; `crates/lophelia-wasm` (`crate-type = ["cdylib", "rlib"]`, `wasm-bindgen = "=<version exacte>"`) avec `#[wasm_bindgen] pub fn engine_version() -> String` qui rend `lophelia_core::VERSION`. `Cargo.lock` commité. `LICENSE` MIT. `.gitignore` ajoute `target/`, `plugin/node_modules/`, `plugin/main.js`, `plugin/wasm-pkg/`, `*.wasm`.
   Test : `crates/lophelia-wasm/src/lib.rs`, `engine_version()` égale `lophelia_core::VERSION` — échoue si la liaison rend autre chose que la version du moteur. `cargo test -p lophelia-wasm --locked`.
2. [x] **`scripts/build-wasm.sh`.** `set -euo pipefail` ; lit la version de `wasm-bindgen` dans `Cargo.lock` et refuse un `wasm-bindgen --version` différent (ADR-0003 `:28`) ; `cargo build --release --locked --target wasm32-unknown-unknown -p lophelia-wasm` ; `wasm-bindgen --target web --out-dir plugin/wasm-pkg` ; `wasm-opt -Oz` via `npm --prefix plugin exec -- wasm-opt`. Rien n'est écrit hors de `target/` et `plugin/wasm-pkg/` (ignorés).
   Vérification : le script produit `plugin/wasm-pkg/` ; avec un `wasm-bindgen` d'une autre version dans le `PATH`, il s'arrête en le nommant.
3. [x] **Outillage du plugin.** `plugin/package.json` (`private`, devDependencies **exactes** : `esbuild`, `typescript`, `obsidian`, `binaryen` ; scripts `build` = `../scripts/build-wasm.sh && tsc --noEmit && node esbuild.config.mjs production`, `typecheck` = `tsc --noEmit`) ; `plugin/package-lock.json` ; `plugin/.nvmrc` à une version exacte de Node LTS ; `plugin/tsconfig.json` en `strict` ; `plugin/esbuild.config.mjs` (entrée `src/main.ts`, `bundle`, `format: "cjs"`, `external` : `obsidian`, `electron`, `@codemirror/*`, `@lezer/*` et les modules Node intégrés, `loader: { ".wasm": "binary" }`, `minify` en production, pas de sourcemap en production, sortie `plugin/main.js`) ; `plugin/manifest.json` (`id: "lophelia"`, `version: "0.1.0"`, `minAppVersion` = version épinglée du paquet `obsidian`, `isDesktopOnly: true`).
   Vérification : `npm --prefix plugin ci` puis `npm --prefix plugin run build` produit `plugin/main.js` ; `git status --porcelain` ne liste ni `main.js`, ni `wasm-pkg/`, ni `target/`.
4. [x] **Chargement du moteur, `plugin/src/engine/wasm.ts`.** Importe les octets du `.wasm` généré (déclaration `*.wasm` → `Uint8Array` dans `plugin/src/engine/wasm.d.ts`) et **seulement** `initSync` et `engine_version` de la glue générée — jamais l'export par défaut, dont l'init asynchrone appelle `fetch` (recherche, Traps). `engineVersion(): string` initialise une fois, à la première demande, pas dans `onload`.
   Vérification : le contrôle réseau (tâche 6) passe sur le `main.js` construit ; `npm --prefix plugin run typecheck`.
5. [x] **Vue et commande.** `plugin/src/view/` : une `ItemView` (type `lophelia`, titre « Lophelia ») dont le conteneur porte la classe `lophelia-view` et qui affiche la version de `engineVersion()` par `setText`/`textContent`, jamais `innerHTML` ; un échec de chargement du WASM s'affiche en mots par une `Notice` (`docs/design-system.md:85`). `plugin/src/main.ts` : `registerView` et `addCommand` « Ouvrir Lophelia » qui révèle la vue existante ou en ouvre une. `plugin/styles.css` : uniquement des règles sous `.lophelia-view`, variables d'Obsidian et corrections du design system, aucune couleur en dur. Construit ce que `docs/designs/s1-chaine-release/design.md` fixera, sans rien ajouter.
   Vérification visuelle : dans le vault de test (tâche 10), la commande ouvre la vue, la version affichée égale celle de `Cargo.toml` ; thèmes clair et sombre. Plus `npm --prefix plugin run typecheck`.
6. [x] **Contrôle réseau, `scripts/check-network.mjs`.** Exporte `findNetworkApis(source)` → liste des motifs trouvés ; en CLI, `node scripts/check-network.mjs plugin/main.js` sort en 1 en les nommant. Motifs, par identifiant entier : `fetch`, `XMLHttpRequest`, `WebSocket`, `requestUrl`, et `require`/`import` de `http`, `https`, `net` (avec ou sans `node:`).
   Tests `scripts/check-network.test.mjs` (`node:test`) : un cas par famille (5), un bundle propre passe, un identifiant qui contient `fetch` sans l'être (`prefetch`) passe. `node --test scripts/check-network.test.mjs`.
7. [x] **Contrôle anti-blob, `scripts/check-no-blobs.mjs`.** Exporte `findBlobs(files)` sur une liste `{ path, content }` ; en CLI, lit `git ls-files -z` et les fichiers suivis, sort en 1 en nommant chaque fichier fautif. Règles : chemin en `.wasm` ; contenu qui porte la signature base64 du WASM (`AGFzbQ`, construite à l'exécution depuis les octets `\0asm`, jamais écrite en clair) ; URL `data:` en `;base64,` de type JavaScript, WASM ou octet-stream (worker ou module embarqué).
   Tests `scripts/check-no-blobs.test.mjs` : `.wasm` suivi refusé ; signature base64 refusée ; worker en `data:…;base64,` refusé ; un contenu de lockfile avec ses empreintes `sha512-…` passe. Fixtures construites à l'exécution. Et `node scripts/check-no-blobs.mjs` sort en 0 sur le dépôt.
8. [x] **CI, `.github/actions/build/action.yml` + `.github/workflows/ci.yml`.** Action composite locale, seule source des étapes de build et de contrôle : `actions/setup-node` (SHA, `node-version-file: plugin/.nvmrc`) ; `rustup show` (installe la chaîne de `rust-toolchain.toml`) ; `cargo install --locked` de `wasm-bindgen-cli` à la version de `Cargo.lock` et de `cargo-audit` à une version exacte ; `npm --prefix plugin ci` ; `node scripts/check-no-blobs.mjs` ; `cargo fmt --check` ; `cargo clippy --workspace --all-targets --locked -- -D warnings` ; `cargo test --workspace --locked` ; `node --test 'scripts/*.test.mjs'` ; `cargo audit` ; `npm --prefix plugin audit --audit-level=high` ; `npm --prefix plugin run build` ; `node scripts/check-network.mjs plugin/main.js` ; taille de `main.js` en octets dans `$GITHUB_STEP_SUMMARY`. `ci.yml` : sur `pull_request` et `push` vers `main`, `permissions: contents: read`, `actions/checkout` épinglé, puis l'action composite.
   Vérification : chaque `uses:` externe porte un SHA de 40 caractères (`grep -rn 'uses:' .github | grep -v '\./' | grep -vE '@[0-9a-f]{40}'` est vide) ; la CI de la PR est verte.
9. [x] **Release, `.github/workflows/release.yml`.** Sur `push` de tags `v*` ; `permissions` : `contents: write`, `id-token: write`, `attestations: write` ; checkout épinglé ; l'action composite (mêmes contrôles qu'en CI, critères 3, 4, 6, 7) ; une étape refuse un tag dont la version (sans `v`) diffère de `plugin/manifest.json`, `plugin/package.json` ou `[workspace.package]` ; `actions/attest-build-provenance` (SHA) sur les trois fichiers ; `gh release create "$GITHUB_REF_NAME" plugin/main.js plugin/manifest.json plugin/styles.css --verify-tag`, l'attestation avant la publication.
   Vérification : relecture du workflow ; preuve réelle au premier tag après l'atterrissage (« Done when »).
10. [x] **`scripts/dev.sh` (spike de l'ADR-0006).** Construit le plugin ; prépare un vault de test minimal hors du dépôt (`${XDG_CACHE_HOME:-$HOME/.cache}/lophelia/vault-test`) avec `.obsidian/plugins/lophelia/` (les trois fichiers) et `community-plugins.json` ; lance Obsidian avec un profil isolé (`--user-data-dir` sous le même dossier, vault enregistré dans ce profil seulement) et `--remote-debugging-port` (`LOPHELIA_CDP_PORT`, défaut 9222) ; affiche l'adresse CDP.
    Vérification : `curl -s http://127.0.0.1:9222/json/version` répond ; `~/.config/obsidian/` inchangé. Résultat du spike (port accepté ou repli de l'ADR-0006 `:26`, mode restreint) noté dans `docs/verif/s1-chaine-release.md`.

## Scene viewpoints
—

## Run interdicts
- Diff vide sur `AGENTS.md`, `AGENTS.local.md`, `.polype/`, `.claude/`, `docs/stories.md`, `docs/architecture.md`, `docs/design-system.md`, `docs/decisions/0001` à `0006`.
- Aucun fichier hors des `Owns` de s1 (`docs/stories.md:13`), sauf `docs/research/`, `docs/plans/`, `docs/verif/`, `docs/designs/`, `docs/qa/`, `docs/reviews/` de s1 et l'ADR-0007.
- Aucun `.wasm`, `plugin/main.js`, `plugin/wasm-pkg/` ni blob base64 commité (`node scripts/check-no-blobs.mjs` à 0).
- Ni `crates/lophelia-bench/` (s3), ni `web/` (s5), ni Web Worker (s5), ni `plugin/src/vault/` ; rien n'est écrit dans le vault, aucun `saveData`, aucun `vault.create`, aucun `innerHTML`.
- Dépendances limitées à `wasm-bindgen` (Rust) et `esbuild`, `typescript`, `obsidian`, `binaryen` (npm, dev), en versions exactes ; actions externes limitées à `actions/checkout`, `actions/setup-node`, `actions/attest-build-provenance`, épinglées par SHA.
- Aucun tag poussé, aucune release créée : c'est la personne, après l'atterrissage.
- Aucune écriture dans `~/.config/obsidian/` ; aucun code repris de New 3D Graph.

## The pivot
Le WASM est embarqué par le loader `binary` d'esbuild et initialisé par `initSync` : pas de blob versionné, pas de `fetch` dans le bundle. Comment cela peut casser :
- esbuild ne retire pas l'init asynchrone de la glue (format `cjs`, effets de bord supposés) : le contrôle réseau échoue sur le `main.js` construit. Comparer `node scripts/check-network.mjs plugin/main.js` ; s'il échoue, s'arrêter et le rapporter — jamais affaiblir les motifs ni exclure la glue.
- `import.meta.url` de la glue, vide en `cjs` : sans effet si la fonction qui le lit est élaguée ; lire les avertissements du build.
- L'attestation repose sur le dépôt public (ADR-0007) : rien dans le code ne la contourne.

## Files touched
`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `crates/lophelia-core/{Cargo.toml,src/lib.rs}`, `crates/lophelia-wasm/{Cargo.toml,src/lib.rs}`, `LICENSE`, `.gitignore`, `scripts/{build-wasm.sh,dev.sh,check-network.mjs,check-network.test.mjs,check-no-blobs.mjs,check-no-blobs.test.mjs}`, `plugin/{package.json,package-lock.json,.nvmrc,tsconfig.json,esbuild.config.mjs,manifest.json,styles.css}`, `plugin/src/{main.ts,engine/wasm.ts,engine/wasm.d.ts,view/…}`, `.github/actions/build/action.yml`, `.github/workflows/{ci.yml,release.yml}`, plus `docs/research/`, `docs/plans/`, `docs/decisions/0007-provenance-depot-prive.md`.

## Test strategy
Automatisés (12, sous le `Test budget` de 25) :
- Rust, 1 : `engine_version()` rend `lophelia_core::VERSION`.
- Contrôle réseau, 7 : une famille de motifs par test, un bundle propre, un faux positif évité. Chaque règle a son test ; retirer un motif fait échouer le sien.
- Contrôle anti-blob, 4 : `.wasm`, signature base64, `data:` base64, lockfile propre.
- La vue n'a pas de test de composant : afficher une donnée n'est pas une règle (doctrine).
Commandes créées par s1, que Mathieu reportera dans `AGENTS.local.md` : suite complète `cargo test --workspace --locked && node --test 'scripts/*.test.mjs'` ; typecheck `npm --prefix plugin run typecheck` (après `scripts/build-wasm.sh`). Lancées par `po-run --story s1-chaine-release`, une fois chacune, à la fin.
Vérifications non automatisées : build et contrôles en CI sur la PR ; écran dans Obsidian (tâches 5 et 10) ; release et attestation au premier tag.

## Done when
- Un commit `s1-chaine-release: …` atterri par PR, avec la taille de `main.js` dans sa description ; CI verte sur la PR.
- Après l'atterrissage et la décision de l'ADR-0007 : un tag `v0.1.0` sur `main` publie les trois fichiers, et `gh attestation verify <fichier> --repo atoll-labs-io/lophelia` réussit sur chacun.
- Revue sans critique ouverte ; Mathieu reporte `Install`, `Test`, `Typecheck`, `Build`, `Dev` dans `AGENTS.local.md`.
