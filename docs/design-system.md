# Design system — Lophelia

Written by `/po-design-system`; change it by running that command again, not by hand.

## Sources
- Visual direction: le thème d'Obsidian. Lophelia hérite de ses variables CSS et se fond dans le thème de l'utilisateur, clair ou sombre (choix de Mathieu, 14/10/2026). Le vault `~/dev` n'utilise ni thème communautaire ni couleur d'accent personnalisée : les valeurs ci-dessous sont celles du thème par défaut.
- System in the base: none — le dépôt était vide (ADR-0002). Les tokens ont été lus dans l'installation locale d'Obsidian 1.14.4 (`/usr/lib/obsidian/obsidian.asar`, fichier `app.css`) ; « Defined at » cite ce fichier et sa ligne.
- Profile: none

## Tokens
Une valeur « → » est une correction approuvée : Lophelia la redéfinit **dans le périmètre de sa propre vue** (`.lophelia-view`), sans jamais toucher au thème d'Obsidian ailleurs.

| Token | Role | Light | Dark | Defined at |
| --- | --- | --- | --- | --- |
| `--background-primary` | surface (fond de la vue et de la scène 3D) | #ffffff | #1c1c1c | `app.css:2943` (base-00 : 2994 / 3044) |
| `--background-secondary` | surface (panneau des réglages) | #f6f6f6 | #282828 | `app.css:2945` (base-20 : 2997 / 3047) |
| `--text-normal` | text | #222222 | #dadada | `app.css:2961` (base-100 : 3005 / 3055) |
| `--text-muted` | text | #5c5c5c | #b3b3b3 | `app.css:2962` (base-70 : 3004 / 3054) |
| `--text-faint` | text | #ababab → #717171 | #666666 → #8e8e8e | `app.css:2963` (base-50 : 3002 / 3052) |
| `--text-accent` | text | #8a5cf5 → #8656f4 | #a68af9 | `app.css:2970` ; dark `app.css:3069` |
| `--text-error` | text | #e93147 → #e72038 | #fb464c | `app.css:2966` |
| `--text-on-accent` | text (sur bouton d'accent) | #ffffff | #ffffff | `app.css:2964` |
| `--interactive-accent` | accent (boutons, curseurs actifs) | #9873f7 → #8457f5 | #8a5cf5 → #8656f4 | `app.css:2974` ; dark `app.css:3070` ; accent hsl(258, 88 %, 66 %) `app.css:2939-2941` |
| `--background-modifier-border` | border | #e4e4e4 | #333333 | `app.css:2948` (base-30 : 2999 / 3049) |
| `--background-modifier-border-focus` | focus | #bdbdbd → #959595 | #555555 → #676767 | `app.css:2950` (base-40 : 3001 / 3051) |
| `--graph-node` | nœud du graphe (= text-muted) | #5c5c5c | #b3b3b3 | `app.css:2312` |
| `--graph-node-focused` | nœud survolé ou focalisé (= text-accent) | #8656f4 | #a68af9 | `app.css:2314` |
| `--graph-text` | étiquette du graphe (= text-normal) | #222222 | #dadada | `app.css:2310` |
| `--graph-line` | arête du graphe | #dadada | #3f3f3f | `app.css:2311` (base-35 : 3000 / 3050) |

## Contrast
Mesuré par `python3` selon WCAG 2, le 14/10/2026. Seuils : 4,5:1 pour le texte, 3:1 pour l'interface non textuelle (bordures, focus, nœuds, arêtes).

| Text | Surface | Theme | Ratio | Needed | Result | Fix |
| --- | --- | --- | --- | --- | --- | --- |
| text-normal | background-primary | light | 15.91:1 | 4.5:1 | pass | — |
| text-normal | background-secondary | light | 14.72:1 | 4.5:1 | pass | — |
| text-muted | background-primary | light | 6.69:1 | 4.5:1 | pass | — |
| text-muted | background-secondary | light | 6.19:1 | 4.5:1 | pass | — |
| text-faint | background-primary | light | 2.30:1 → 4.88:1 | 4.5:1 | pass after fix | #ababab → #717171 |
| text-faint | background-secondary | light | 2.12:1 → 4.51:1 | 4.5:1 | pass after fix | #ababab → #717171 |
| text-accent | background-primary | light | 4.27:1 → 4.51:1 | 4.5:1 | pass after fix | #8a5cf5 → #8656f4 |
| text-accent | background-secondary | light | 4.18:1 (after fix) | 4.5:1 | fail | — (voir Don't) |
| text-error | background-primary | light | 4.20:1 → 4.50:1 | 4.5:1 | pass after fix | #e93147 → #e72038 |
| text-error | background-secondary | light | 4.17:1 (after fix) | 4.5:1 | fail | — (voir Don't) |
| text-on-accent | interactive-accent | light | 3.42:1 → 4.52:1 | 4.5:1 | pass after fix | interactive-accent #9873f7 → #8457f5 |
| interactive-accent | background-primary | light | 4.50:1 (after fix) | 3:1 | pass | — |
| background-modifier-border | background-primary | light | 1.27:1 | 3:1 | fail | — (fix set aside : fidèle à Obsidian, à revoir à l'œil) |
| background-modifier-border-focus | background-primary | light | 1.88:1 → 3.01:1 | 3:1 | pass after fix | #bdbdbd → #959595 |
| graph-node (text-muted) | background-primary | light | 6.69:1 | 3:1 | pass | — |
| graph-node-focused (text-accent) | background-primary | light | 4.51:1 (after fix) | 3:1 | pass | — |
| graph-line | background-primary | light | 1.40:1 | 3:1 | fail | — (fix set aside → Gaps) |
| text-normal | background-primary | dark | 12.19:1 | 4.5:1 | pass | — |
| text-normal | background-secondary | dark | 10.55:1 | 4.5:1 | pass | — |
| text-muted | background-primary | dark | 8.13:1 | 4.5:1 | pass | — |
| text-muted | background-secondary | dark | 7.03:1 | 4.5:1 | pass | — |
| text-faint | background-primary | dark | 2.97:1 → 5.20:1 | 4.5:1 | pass after fix | #666666 → #8e8e8e |
| text-faint | background-secondary | dark | 2.57:1 → 4.50:1 | 4.5:1 | pass after fix | #666666 → #8e8e8e |
| text-accent | background-primary | dark | 6.20:1 | 4.5:1 | pass | — |
| text-error | background-primary | dark | 4.94:1 | 4.5:1 | pass | — |
| text-on-accent | interactive-accent | dark | 4.27:1 → 4.51:1 | 4.5:1 | pass after fix | interactive-accent #8a5cf5 → #8656f4 |
| interactive-accent | background-primary | dark | 3.77:1 (after fix) | 3:1 | pass | — |
| background-modifier-border | background-primary | dark | 1.35:1 | 3:1 | fail | — (fix set aside : fidèle à Obsidian, à revoir à l'œil) |
| background-modifier-border-focus | background-primary | dark | 2.29:1 → 3.01:1 | 3:1 | pass after fix | #555555 → #676767 |
| graph-node (text-muted) | background-primary | dark | 8.13:1 | 3:1 | pass | — |
| graph-node-focused (text-accent) | background-primary | dark | 6.20:1 | 3:1 | pass | — |
| graph-line | background-primary | dark | 1.62:1 | 3:1 | fail | — (fix set aside → Gaps) |

Les corrections ne sont dans aucun code : le dépôt n'a pas encore de vue. La story qui crée la feuille de style de la vue (s1 pour la vue vide, s5 pour la scène) les applique en redéfinissant ces variables sous `.lophelia-view`.

## Components
Lophelia n'a pas de composants à elle : elle utilise ceux de l'API publique d'Obsidian (ADR-0005).

| Component | Path | Variants | States |
| --- | --- | --- | --- |
| Vue | `obsidian.d.ts` : `ItemView` | une vue « Lophelia » | ouverte, fermée |
| Commande | `obsidian.d.ts` : `Plugin.addCommand` | « Ouvrir Lophelia » | — |
| Notice | `obsidian.d.ts` : `Notice` | message simple | affichée, expirée |
| Ligne de réglage | `obsidian.d.ts` : `Setting` | avec curseur (`addSlider`), avec bouton (`addButton`, `setCta` pour l'accent) | normal, survol, focus, désactivé |
| Onglet de réglages | `obsidian.d.ts` : `PluginSettingTab` | — | — |

## State patterns
- Empty: pas de motif dans le système → Gaps.
- Loading: texte « Calcul de la disposition : N % » en `--text-muted` sur `--background-primary`, mis à jour au moins une fois par seconde (s5).
- Error: `Notice` d'Obsidian, avec un message en mots (jamais la couleur seule), par exemple pour un `positions.json` invalide (s6).
- Success: silencieux — la forme s'affiche ; pas de notification.
- Disabled: état désactivé natif des `Setting` d'Obsidian.
- Focus: contour `--background-modifier-border-focus` (corrigé à 3:1) sur les contrôles ; dans la scène, le nœud focalisé prend `--graph-node-focused`.

## Do / Don't

Do:
- Utiliser uniquement les variables CSS d'Obsidian listées ici, pour que la vue suive le thème de l'utilisateur.
- Redéfinir les valeurs corrigées sous `.lophelia-view` seulement.
- Construire l'interface avec les composants de l'API d'Obsidian (`Setting`, `Notice`, `ItemView`).

Don't:
- Coder une couleur en dur, dans le CSS ou dans la scène three.js (lire les variables à l'exécution).
- Styler l'interface d'Obsidian hors de la vue Lophelia.
- Poser du texte d'accent ou d'erreur sur `--background-secondary` (4,18:1 et 4,17:1 en clair) : réservés à `--background-primary`.
- Délimiter un contrôle par `--background-modifier-border` seule (1,27:1).

## Gaps
- **Arêtes du graphe** (`--graph-line`, 1,40:1 / 1,62:1) : la valeur d'Obsidian est volontairement pâle, et une valeur à 3:1 (#959595 / #676767) risque de noyer les groupes. À trancher à l'œil pendant s5 sur le banc, la valeur retenue revenant par `/po-design-system`.
- **Composants de la scène 3D** : forme et taille d'un nœud, épaisseur d'une arête, étiquette de survol. Le système d'Obsidian n'en a pas ; seules leurs couleurs sont fixées (`--graph-*`).
- **Navigation clavier dans la scène** : comment le focus se déplace de nœud en nœud et se voit (doctrine « focus visible »). Aucun motif existant.
- **État vide** : vault sans aucune note visible (toutes exclues). Aucun motif dans Obsidian.
- **Revue visuelle du style** : Mathieu veut juger le style une fois l'écran sous les yeux ; les bordures restent fidèles à Obsidian d'ici là (1,27:1 / 1,35:1 notées en échec).
