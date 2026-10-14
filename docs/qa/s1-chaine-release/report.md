# QA — s1-chaine-release

[Story](../../stories.md) · [Plan](../../plans/s1-chaine-release.md)

Scope: the story's surface, what `git diff main...story/s1-chaine-release` touches on it, and the acceptance criteria its user can observe through it; nothing else was checked.
Driver: `browser.md` — surface `screen`, platform `web, lib, cli` (seule la ligne `screen` / `web` s'applique)
Evidence: captures
Tool: Chrome DevTools Protocol de la fenêtre Obsidian ouverte par `Dev` (`Page.captureScreenshot`, `Input.dispatchKeyEvent`, `Input.insertText`, `Runtime.evaluate`), piloté par un script Node jetable placé dans un dossier `mktemp -d`, supprimé à la fin ; les outils Playwright de la session (Firefox) ne savent pas se brancher sur Obsidian
Origin: `http://127.0.0.1:9222` (adresse CDP imprimée par `scripts/dev.sh`, seule visitée ; page `app://obsidian.md/index.html`, Obsidian 1.14.4, vault `~/.cache/lophelia/vault-test`, fenêtre de 626 × 1042 px)

Contexte : nouveau passage après le commit `1422a47` (« address review findings »). Depuis la QA précédente (tree `7a75e59…`), le seul changement hors de `docs/` est `.github/actions/build/action.yml` (`node --test scripts/` devient `node --test 'scripts/*.test.mjs'`) : rien ne change à l'écran, le walk a tout de même été refait en entier sur le code actuel.

Préparation : l'espace de travail restauré du vault de test avait déjà une feuille Lophelia ouverte. Elle a été fermée par script (`detachLeavesOfType`) pour partir d'un état sans vue. Les bascules de thème (`app.changeTheme`), la largeur étroite (`workspace:split-vertical`), le choix de la feuille active et la panne simulée sont faits par script ; le reste se fait au clavier, par la palette de commandes. En fin de walk, `WebAssembly.Module`, le thème sombre et le plugin ont été remis en état avant l'arrêt de `Dev`.

## Coverage
| Criterion or item | Reached | Result |
| --- | --- | --- |
| C5 — la commande « Ouvrir Lophelia » existe (palette, Ctrl+P) | yes | as expected — « Lophelia: Ouvrir Lophelia » (`c5-palette-commande.png`) |
| C5 — la commande ouvre une vue dans un onglet de l'espace principal | yes | as expected — une feuille `lophelia` dans `rootSplit`, devenue active (`c5-vue-ouverte-sombre.png`) |
| C5 — la vue affiche la version du moteur renvoyée par le WASM | yes | as expected — `Moteur 0.1.0` = `Cargo.toml` (workspace) et `plugin/manifest.json` ; `div.empty-state > div.empty-state-container`, centré (décision du lead, 14/10/2026) |
| Design — seconde exécution révèle la vue existante | yes | as expected à l'affichage (une seule feuille, onglet au premier plan) ; finding 1 |
| Design — thème sombre | yes | as expected — `rgb(179,179,179)` sur `rgb(28,28,28)` |
| Design — thème clair | yes | as expected — `rgb(92,92,92)` sur `rgb(255,255,255)` (`c5-vue-ouverte-clair.png`) |
| Design — largeur étroite (écran scindé, 141 px) | yes | as expected — `scrollWidth` = `clientWidth` = 141, pas de défilement horizontal (`c5-largeur-etroite.png`) |
| Design — état d'échec | yes (panne simulée) | as expected à l'écran : `Moteur non chargé` en `rgb(251,70,76)` sombre / `rgb(231,32,56)` clair, Notice « Lophelia : le moteur n'a pas pu être chargé. module invalide (QA simulé) » (`c5-echec-sombre.png`, `c5-echec-clair.png`) ; finding 2 |
| Clavier seul (Tab, Shift+Tab, Entrée, Échap, Ctrl+P) | yes | Tab et Shift+Tab circulent dans l'en-tête de la vue (Naviguer en arrière, Naviguer vers l'avant, Plus d'options), focus visible (anneau `box-shadow` 3 px) et `aria-label` sur chacun (`clavier-tab-01.png`) ; aucun contrôle dans le corps (conforme au design) ; Ctrl+P et Entrée ouvrent la vue ; finding 3 pour Échap |

## Findings
Status rule: **verified** means the steps were replayed and reproduced the defect, and an evidence file holds it; **probable** means it was seen once, or no evidence file could be kept; **unverified** means it is suspected from what the surface showed, not seen happening.

### Finding 1 — La seconde commande montre l'onglet Lophelia sans le rendre actif
Severity: minor
Status: verified
Steps to reproduce:
1. Lancer `scripts/dev.sh` ; Ctrl+P, « Ouvrir Lophelia », Entrée : la vue s'ouvre.
2. Donner le focus à l'onglet de la note « Bienvenue », dans le même groupe d'onglets.
3. Ctrl+P, « Ouvrir Lophelia », Entrée.
Capture: `docs/qa/s1-chaine-release/c5-seconde-commande-revele.png`, rejeu `docs/qa/s1-chaine-release/c5-seconde-commande-rejouee.png`
Expected: la commande « révèle la vue si elle est déjà ouverte » (design, `## Screen`), et la vue révélée devient la feuille active.
Observed: l'onglet Lophelia passe au premier plan, mais `app.workspace.activeLeaf` reste la note markdown (`lophActive: false`, barre d'état « 6 mots 40 caractères ») et le focus clavier reste sur `body`. La première ouverture rend la vue active ; la branche `revealLeaf(existing)` de `plugin/src/main.ts` ne le fait pas. Constaté deux fois dans ce walk.

### Finding 2 — En cas d'échec, un `div.lophelia-engine-version` vide reste dans la vue
Severity: minor
Status: verified
Steps to reproduce:
1. Dans la fenêtre ouverte par `Dev`, faire échouer le chargement du WASM (`WebAssembly.Module` remplacé par une fonction qui lève), puis désactiver et réactiver le plugin.
2. Ctrl+P, « Ouvrir Lophelia », Entrée.
3. Lire le DOM du conteneur de la vue ; recommencer en thème clair.
Capture: `docs/qa/s1-chaine-release/c5-echec-sombre.png`, rejeu `docs/qa/s1-chaine-release/c5-echec-clair.png` (élément invisible, hauteur 0) ; DOM relevé deux fois : `<div class="empty-state-container"><div class="lophelia-engine-version"></div><div class="lophelia-engine-error">Moteur non chargé</div></div>`
Expected: dans l'état d'erreur, une seule ligne `Moteur non chargé` (design, `## States`).
Observed: dans `plugin/src/view/lophelia-view.ts`, `container.createDiv({ cls: "lophelia-engine-version" })` s'exécute avant l'évaluation de `engineVersion()`, qui lève. Le div vide reste donc devant la ligne d'erreur. Aucun effet visible aujourd'hui, mais un futur style sur `.lophelia-engine-version` apparaîtrait dans l'état d'erreur.

### Finding 3 — Échap dans la vue Lophelia bascule sur l'onglet de la note
Severity: minor
Status: verified
Steps to reproduce:
1. Lancer `scripts/dev.sh` ; ouvrir la vue (Ctrl+P, « Ouvrir Lophelia », Entrée), avec la note « Bienvenue » ouverte dans un autre onglet du même groupe.
2. Rendre la vue Lophelia active (onglet cliqué ou `setActiveLeaf`), le focus sur `body` ou sur un bouton de l'en-tête.
3. Appuyer sur Échap.
Capture: `docs/qa/s1-chaine-release/clavier-echap-quitte-vue.png`
Expected: Échap garde le comportement d'Obsidian pour une vue (design, `## States`, « Échap, Ctrl+P et la navigation entre onglets restent ceux d'Obsidian »). Dans les mêmes conditions, la vue graphe native et un onglet vide restent actifs après Échap.
Observed: la feuille active redevient la note markdown, l'onglet Lophelia passe à l'arrière-plan (`lophShown: false`) et le focus va dans l'éditeur (`div.cm-content`). Constaté trois fois : en fin de parcours Tab, focus sur `body`, puis au rejeu capturé. Seule différence relevée entre les vues : `navigation` vaut `false` pour Lophelia et `true` pour le graphe. Le lien entre ce drapeau et la bascule n'est pas prouvé.

## Not seen
- C1 — tag `v*`, workflow, release : aucun effet à l'écran ; visible au premier tag après l'atterrissage.
- C2 — attestation, `gh attestation verify` : aucun effet à l'écran ; n'existe qu'après une release.
- C3 — aucun `.wasm` ni blob base64 versionné : contrôle de dépôt et de CI, sans effet à l'écran.
- C4 — la CI échoue si le bundle contient une API réseau : contrôle de CI, sans effet à l'écran.
- C6 — dépendances verrouillées, outils à version fixe : sans effet à l'écran.
- C7 — `cargo audit` et `npm audit` : sans effet à l'écran. Le seul changement depuis la QA précédente (la commande de test de `.github/actions/build/action.yml`) relève aussi de la CI.
- C5 — installation depuis une release GitHub : seul le plugin construit localement et copié par `scripts/dev.sh` a été vu ; aucune release n'existe.
- C5 — origine WASM du texte : l'écran ne la distingue pas d'une constante ; seule la panne simulée l'indique indirectement.
- Menu « Plus d'options » activé au clavier : Entrée envoyé par CDP (`rawKeyDown` sans événement `char`) n'a pas ouvert le menu. Le walk ne dit donc pas si une vraie touche Entrée l'ouvre. C'est un contrôle d'Obsidian, pas de Lophelia.
- Panneau latéral droit : la largeur étroite n'a été vue qu'en écran scindé.
- Thèmes forcés par le système hors d'Obsidian : non atteints depuis l'origine.

Tree: e0ccf2b03445ce6e2f78506883e1055791c41abf
QA status: complete
