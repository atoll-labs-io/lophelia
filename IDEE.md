# Lophelia

Une vue graphe 3D du vault Obsidian `~/dev` dont la disposition ne bouge plus d'une ouverture à l'autre. Décidé le 14/10/2026, après la revue de Filamental 3D Graph (voir plus bas). Le format (plugin Obsidian ou application à part) se tranche dans `/po-brief`.

## Nom

*Lophelia pertusa* (aujourd'hui *Desmophyllum pertusum*) : corail d'eau froide qui bâtit de vastes récifs en trois dimensions dans l'obscurité des grands fonds, branche après branche, et dont la structure reste en place pendant des siècles.

## Le besoin

- Voir le vault en 3D, en paquets par thème : domaine → projet → ADR / milestones → story → ses artefacts.
- **Disposition persistante** : chaque nœud garde sa place d'une session à l'autre ; une nouvelle note s'insère près de ses voisines sans tout réorganiser.
- Fonctionne sous **Linux** (Arch, Hyprland).
- **Open source, sans dépendance à un éditeur** : pas d'application propriétaire ni d'offre « gratuit pour un seul vault ».

## Points de départ pour le brief

- **Taille réelle** : environ 2 500 notes `.md` indexées par Obsidian, dont ~1 300 visibles une fois le code tiers retiré, et 280 notes « carte » générées (`~/dev/_cartes/`, par `_cartes/.outils/generer.py`).
- **Exclusions** : le graphe doit respecter les exclusions d'Obsidian (`userIgnoreFilters` dans `.obsidian/app.json` : `node_modules`, `tools/flutter`, `tools/context7`…). Le graphe global natif ne le fait pas, et Filamental non plus (il charge tout via `getMarkdownFiles()`).
- **Liens** : les arêtes viennent de `metadataCache.resolvedLinks`. Les liens d'un artefact Polype à l'autre arrivent avec la story Polype s128 (`../brouillons/polype-s128-artefacts-relies.md`).
- **Couleurs** : reprendre les groupes de couleur par domaine de `.obsidian/graph.json`.
- **Où stocker les positions** : un dossier caché (`.lophelia/`) n'est pas indexé par Obsidian. L'écrire avec l'API du vault (`vault.create`/`getAbstractFileByPath`) ne se relit pas ; il faut passer par `vault.adapter` ou par le stockage du plugin. C'est très probablement le bug de Filamental.
- **Leçons de la revue de Filamental (14/10/2026)** : le `main.js` publié était construit hors du dépôt public, donc invérifiable. Lophelia construit sa release dans la CI, depuis les sources du dépôt, avec une attestation de provenance sur cet artefact-là.
- **Existant à regarder** : 3D Graph (alexw00, apoo711), GraphFrontier, Graph Save et Persistent Graph (positions en 2D), Filamental (three.js + positions en JSON).
- **Plus tard** : ouvert à d'autres vaults que `~/dev` ; peut-être une source de données Nautile (la mémoire des agents vue comme un graphe).
