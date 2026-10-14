# Decisions log

What the project lead decided alone, one row per decision, appended and never rewritten: `Date` is `YYYY-MM-DD`, `Story` is a story id or `—` when the decision concerns no single story.
A decision the person took is not logged here: it is an ADR or a story.

| Date | Story | Decision | Reason |
| --- | --- | --- | --- |
| 2026-10-14 | s1-chaine-release | Vague 1 limitée à s1 | Seule story prête : toutes les autres dépendent d'elle, et ses Owns couvrent le squelette. |
| 2026-10-14 | s1-chaine-release | Plan validé sans relecture humaine, une fois le choix technique de l'ADR-0007 tranché par Mathieu | Plan validation: autonomous ; seul le choix technique (dépôt public) est allé à Mathieu. |
| 2026-10-14 | — | Suppression automatique des branches laissée désactivée à l'ouverture du dépôt | /po-ship et le landing de cadrage suppriment eux-mêmes leur branche ; une suppression automatique ferait échouer ce nettoyage. |
| 2026-10-14 | s1-chaine-release | Corps de la vue sur la structure `.empty-state` native d'Obsidian, sans maquette | Mathieu a demandé un rendu fidèle à Obsidian, style revu à l'œil plus tard ; une ligne de texte n'appelle pas de maquette, et le skill de design n'est pas installé (polype#87). |
| 2026-10-14 | s1-chaine-release | Rapports de QA enregistrés mot pour mot par le pilote, puis commit `QA evidence` | Le harnais refuse qu'un sous-agent écrive `report.md` ; même traitement que la revue (polype#88). |
| 2026-10-14 | s1-chaine-release | Correctif lancé (tour 1 sur 2) après l'échec de la CI sur `node --test scripts/` | Le Node 24 de la CI refuse un dossier en argument ; le ship s'arrête et repart en correction. |
| 2026-10-14 | s1-chaine-release | Commit d'atterrissage poussé sur la PR avant la revue du correctif | Obtenir le verdict de la CI pendant la QA et la revue ; le ship a reconstruit et repoussé le commit ensuite. |
| 2026-10-14 | s1-chaine-release | Revue du tour 1 bornée à `origin/main...story/s1-chaine-release` | La `main` locale était en retard ; le premier reviewer avait signalé à tort `AGENTS.local.md`, déjà sur `origin/main`. |
| 2026-10-14 | s1-chaine-release | Étape « file the open findings » de /po-ship non jouée | Tous les constats étaient déjà ouverts par octo (#40 à #48), canal imposé par Mathieu ; la jouer aurait créé des doublons. |
| 2026-10-14 | s2-contrat-moteur | Vague 2 limitée à s2 | Seule story prête une fois s1 livrée : s3 et les suivantes dépendent de s2. |
| 2026-10-14 | s2-contrat-moteur | Réponse « le plus optimisé sur du gros volume de nœuds » lue comme l'option 2 de l'ADR-0008 | serde_json pour le fichier seul, tableaux typés à la frontière WASM : c'est l'option qui évite la sérialisation à chaque calcul. |
| 2026-10-14 | s2-contrat-moteur | Plan validé sans relecture humaine après ce choix | Plan validation: autonomous. |
| 2026-10-14 | s2-contrat-moteur | Commit d'atterrissage poussé et PR ouverte pendant la QA | Obtenir le verdict de la CI en parallèle ; reconstruit et repoussé après la revue. |
| 2026-10-14 | s2-contrat-moteur | Build de production lancé au ship | Build stage: ship-if-route ; le manifeste `crates/lophelia-core/Cargo.toml` a changé. |
| 2026-10-14 | s2-contrat-moteur | Verrou `index.lock` du worktree supprimé | Vide et sans processus git actif ; il bloquait l'avance rapide du couloir. |
| 2026-10-14 | s2-contrat-moteur | Étape « file the open findings » de /po-ship non jouée | Constats déjà ouverts par octo (#54 à #58). |
| 2026-10-14 | s3-banc-evaluation | Vague 3 limitée à s3 | Seule story prête une fois s2 livrée. |
| 2026-10-14 | s3-banc-evaluation | Plan validé sans relecture humaine ; Louvain, RNG et analyse des arguments écrits sans dépendance | Plan validation: autonomous ; rien n'entre dans le lockfile, donc aucun choix technique à soumettre. |
| 2026-10-14 | s3-banc-evaluation | Critère 8 reformulé (PR #60) : seuls les scores, écarts et fichiers doivent se répéter | Le temps de calcul, mesuré à l'horloge, ne peut pas se répéter ; une seule lecture possible. |
| 2026-10-14 | s3-banc-evaluation | QA gardée `complete` malgré le refus de `git check-ignore` par po-bounds | Le refus coupe une vérification, pas le parcours ; le pilote a vérifié lui-même que `bench/local/` est ignoré. |
| 2026-10-14 | s3-banc-evaluation | Build de production lancé au ship | Le `Cargo.toml` racine a changé (nouveau membre du workspace). |
| 2026-10-14 | s3-banc-evaluation | Étape « file the open findings » de /po-ship non jouée | Constats déjà ouverts par octo (#64 à #71). |
