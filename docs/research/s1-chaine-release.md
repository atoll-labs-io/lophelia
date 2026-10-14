# Research — s1-chaine-release

[Story](../stories.md)

## Structuring facts
- **Hypothèse fausse (critère 2, et par ricochet critère 1)** : le dépôt `atoll-labs-io/lophelia` est **privé**, sur une organisation au plan GitHub **Free** (`gh api repos/atoll-labs-io/lophelia` → `"private":true` ; `gh api orgs/atoll-labs-io` → `plan.name: "free"`, lus le 14/10/2026). Les attestations d'artefact (`actions/attest-build-provenance`) n'existent, sur un dépôt privé, qu'avec GitHub Enterprise Cloud : en l'état, l'étape d'attestation du workflow de release échoue, donc la release attestée de `docs/stories.md:16-17` et `docs/decisions/0004-chaine-release.md:22-24` ne peut pas être publiée. Choix soumis à la personne : ADR-0007 (`proposed`).
- **Hypothèse fausse (machine locale)** : le poste n'a ni `rustup` ni la cible `wasm32-unknown-unknown` (`rustc 1.99.0` du paquet Arch `rust 1:1.99.0-1`, `/usr/lib/rustlib/` ne contient que `x86_64-unknown-linux-gnu`), ni `wasm-bindgen`, ni `wasm-opt`, ni `cargo-audit`. `rust-toolchain.toml` y sera ignoré : sans `rustup`, l'implementer ne peut ni compiler le WASM ni bundler le plugin en local.
- Le dépôt n'a aucun code (ADR-0002, `docs/decisions/0002-depot-vide.md:22`) : tout le squelette est à créer ; les chemins de `docs/architecture.md:15-24` sont une disposition prévue, pas un existant.
- La pile est fixée : `wasm-bindgen-cli` à la version exacte du crate, puis `wasm-opt`, esbuild en un seul `main.js` avec le WASM embarqué en octets, npm + `npm ci` (`docs/decisions/0003-pile-technique.md:27-31`) ; actions épinglées par SHA (`docs/decisions/0004-chaine-release.md:24`).
- La QA `screen` de s1 s'arrêtera : `Dev: —` (`AGENTS.local.md:80`) et `/po-qa` stoppe une surface `screen` sans `Dev` (`~/.claude/commands/po-qa.md:26`) ; `AGENTS.local.md` n'est pas dans les `Owns` de s1 (`docs/stories.md:13`) et l'ADR-0006 réserve cette ligne à Mathieu après l'atterrissage (`docs/decisions/0006-qa-ecrans-obsidian.md:25`).

## Story
Prouver toute la chaîne Rust/WASM → plugin Obsidian → release attestée avant d'écrire le moteur (`docs/stories.md:7-25`). Critères :
1. un tag `v*` déclenche un workflow qui compile le crate WASM et le plugin depuis les sources et publie `main.js`, `manifest.json`, `styles.css` en release GitHub ;
2. chaque artefact publié est attesté ; `gh attestation verify <fichier> --repo atoll-labs-io/lophelia` réussit ;
3. aucun `.wasm`, blob WASM ou worker base64 versionné ; la CI échoue si `git ls-files` en liste un ;
4. la CI échoue si le bundle publié contient `fetch`, `XMLHttpRequest`, `WebSocket`, modules `http`/`https`/`net`, `requestUrl` ;
5. la commande « Ouvrir Lophelia » ouvre une vue qui affiche la version du moteur, renvoyée par une fonction du module WASM ;
6. dépendances verrouillées (`Cargo.lock`, lockfile npm), installation stricte en CI ; Rust, wasm-bindgen et Node à version fixe ;
7. `cargo audit` et `npm audit` à chaque build CI, échec sur haut ou critique.
Notes : taille du bundle mesurée et notée dans la PR ; licence MIT ; rien repris de New 3D Graph (`docs/stories.md:25`).

## Current code
- Fichiers suivis : `.claude/settings.json`, `.polype/**`, `AGENTS*.md`, `IDEE.md`, `docs/**` (`git ls-files`). Aucun manifeste (`Cargo.toml`, `package.json`), aucun `.github/`, aucun `scripts/`, aucun `LICENSE`.
- `.gitignore:1-3` : `.worktrees/`, `.env*`, `.claude/worktrees/` seulement — ni `target/`, ni `node_modules/`, ni sorties de build.
- Remote : `origin git@github.com:atoll-labs-io/lophelia.git` ; aucune release (`gh api .../releases` → 0). Actions activées, toutes actions permises, épinglage SHA non imposé par le dépôt (`gh api .../actions/permissions`) : l'épinglage est une règle de l'ADR-0004, pas de GitHub.
- Commandes du projet toutes à `—` (`AGENTS.local.md:74-80`) ; l'architecture attend que s1 les crée et que Mathieu les écrive ensuite (`docs/architecture.md:60`).

## Attachment points
- `Cargo.toml` `[workspace] members` (`docs/architecture.md:56`) : `crates/lophelia-core`, `crates/lophelia-wasm` ; `lophelia-bench` appartient à s3 (`docs/stories.md:53`).
- `plugin/src/main.ts` (`onload`) : vue et commande (`docs/architecture.md:54`) ; vue sous `plugin/src/view/` (`docs/architecture.md:20`).
- `.github/workflows/` : contrôles de CI (`docs/architecture.md:57`) ; `scripts/` : `scripts/dev.sh` (`docs/architecture.md:24`, ADR-0006 `:25`).
- Styles : variables corrigées redéfinies sous `.lophelia-view` seulement (`docs/design-system.md:11`, `:69`, `:94`).

## Verified interfaces
- Obsidian installé : `/usr/bin/obsidian`, `/usr/lib/obsidian/obsidian.asar`, version 1.14.4 (`docs/design-system.md:7`) ; paquet npm `obsidian` (types de l'API) en 1.14.4 (`npm view obsidian version`).
- Composants imposés : `ItemView`, `Plugin.addCommand`, `Notice` de `obsidian.d.ts` (`docs/design-system.md:76-78`).
- `isDesktopOnly` vrai et `minAppVersion` fixé par s1 (`docs/decisions/0005-integration-obsidian.md:26`).
- Paquet npm `binaryen` 132.0.0 : expose le binaire `wasm-opt` (`npm view binaryen bin`) — `wasm-opt` peut donc être épinglé par le lockfile npm, sans téléchargement à part.
- Versions npm courantes relevées : `esbuild` 0.28.2, `typescript` 7.0.2 (`npm view`).
- Outils locaux : `node v26.11.1`, `npm 12.2.0`, `gh` présent (`which`, `--version`).

## Traps
- **Glue wasm-bindgen et contrôle réseau** : la sortie `--target web` contient une fonction d'init asynchrone qui appelle `fetch` sur une URL du `.wasm` ; si elle reste dans le bundle, le contrôle du critère 4 échoue (à raison). Le plugin doit n'importer que l'init synchrone (`initSync` avec les octets embarqués) pour que l'élagage d'esbuild retire l'autre ; c'est le contrôle réseau lui-même qui le prouve sur le vrai bundle. Vérifié par connaissance de wasm-bindgen, pas sur une sortie générée ici (pas de cible wasm locale).
- **Auto-détection du contrôle anti-blob** : un script qui cherche la signature base64 du WASM (`AGFzbQ`, base64 de `\0asm`) se trouve lui-même s'il l'écrit en clair ; de même pour un test qui la mettrait en fixture. Construire le motif à l'exécution.
- **Lockfiles** : `package-lock.json` porte des empreintes `sha512-…` en base64 (~88 caractères) ; une règle « longue suite base64 » doit avoir un seuil bien au-dessus, ou viser les marqueurs explicites (`AGFzbQ`, `;base64,`).
- **`cargo audit` n'a pas de seuil de gravité** (à vérifier) : il échoue sur tout avis de vulnérabilité, plus strict que « haut ou critique » ; `npm audit --audit-level=high` tient le seuil exact.
- **Tag `v*` et magasin Obsidian** : le magasin attend un tag égal à la version de `manifest.json`, sans `v` ; sans effet pour la v1 (installation manuelle ou BRAT, ADR-0004 `:25`), à revoir avec l'issue #23.
- **BRAT et dépôt privé** : une release d'un dépôt privé ne s'installe pas sans jeton ; autre argument pour l'ADR-0007.
- **Obsidian déjà ouvert** : lancer `obsidian` une seconde fois avec le même profil passe la main à l'instance de Mathieu ; `scripts/dev.sh` doit isoler son profil (`--user-data-dir`) et enregistrer le vault de test dans ce profil, jamais dans `~/.config/obsidian/`.
- **Commandes `Test`/`Typecheck` à `—`** : `po-run` exige une commande ; le plan nomme les commandes que s1 crée, que Mathieu reportera dans `AGENTS.local.md` (hors `Owns`).
- Pas de migration ; aucune autre story n'a touché le code (le dépôt est vide).

## Open questions
- Provenance pendant que le dépôt est privé : rendre le dépôt public avant le premier tag, signer autrement, payer Enterprise ou reporter le critère 2 ? → ADR-0007, à trancher par la personne.
- Qui installe `rustup` et la cible `wasm32-unknown-unknown` sur le poste (paquet `rustup` qui remplace `rust`, donc `sudo`, ou `rustup-init` en espace utilisateur) avant `/po-execute` ?
- Le port de débogage d'Obsidian 1.14.4 et l'ouverture d'un vault non enregistré via `--user-data-dir` marchent-ils ? Spike de `scripts/dev.sh` (ADR-0006 `:26` donne le repli).
- Le mode restreint d'Obsidian bloque-t-il l'activation automatique du plugin dans un profil neuf ? À constater pendant le spike.
- Titulaire du copyright de la licence MIT : ATOLL LABS (organisation GitHub) ou Mathieu ?
- La QA de s1 a besoin de `Dev` : Mathieu l'écrit-il avant la QA de s1 (contrairement au « après atterrissage » de l'ADR-0006) ?

## Complexity
Story : 4. Après lecture : **4**. Intégration tierce (Obsidian, GitHub Actions, attestations) et deux chaînes d'outils, sans logique métier ; le risque principal reste le chargement du WASM sans blob ni `fetch`, et un risque nouveau s'ajoute, hors code : la visibilité du dépôt.

## Split
Sans objet (complexité 4). Le plan tient en dix tâches.
