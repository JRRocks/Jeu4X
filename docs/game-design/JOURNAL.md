# Journal de la session 2 : game design

> Entrée la plus récente en haut. On ne modifie ni ne supprime une entrée passée.

## 2026-11-27 · Mission 3 : réglage de la durée des parties

- **Auteur** : Claude, session S2
- **Branche et commit** : `s2-game-design`
- **Objectif** : ramener les parties entre IA d'environ 10 tours vers 25 à 35 tours, en ne touchant que les données (demande de la session maître).
- **Réalisé** : diagnostic sur une copie de travail du moteur de `s3-ia` (ab33186), hors dépôt, avec l'exemple `parties_ia` et la commande `chrono`. Les villes grandissaient trop vite pour trop peu, l'économie s'emballait et toutes les parties finissaient au seuil. Trois essais de S3, à 200 parties par format avec les clans en rotation. Valeurs retenues envoyées à S1, session maître en copie. `reglage-duree.md` écrit ; § 17 ajouté à `regles-v1.md`.
- **Décisions** : aucune décision de l'utilisateur. Réglage de données ; une valeur nouvelle, le bonus de défense des capitales, autorisée par la consigne (« défense des capitales »).
- **Vérifications** : essai 3 de S3 (b4c36fd) : 29,6 / 33,7 / 30,1 tours en moyenne à 2 / 3 / 4 joueurs.
- **Non vérifié sur téléphone** : sans objet.
- **Problèmes et dette** : mes premières mesures avec `chrono` étaient trop optimistes, car il oppose toujours les deux mêmes clans. Il reste 10 à 12 % de nulles par palier de score, et les Vents-Errants sont faibles.
- **Prochaine étape** : attendre ; reprendre les nulles et les Vents-Errants si la session maître le demande.

## 2026-11-27 · Mission 2 : règles précises de la première version

- **Auteur** : Claude, session S2
- **Branche et commit** : `s2-game-design`, après fusion de `main` (tri, D-22 à D-29)
- **Objectif** : écrire les règles de la V1 (D-23, D-28) assez précises pour que S1 les code et que S3 les apprenne à l'IA.
- **Réalisé** : `regles-v1.md` : plateau, création de la carte, départs équitables, brouillard qui se souvient, villes, économie, combat sans hasard avec sa formule entière, quatre clans, arbre précisé, victoires, accord avec D-28, autres idées gardées, tables de données en identifiants anglais, actions, 14 cas limites, questions GD-Q9 à GD-Q14. `regles-v1-resume.md` : une page pour l'utilisateur.
- **Décisions** : aucune. Correction de l'esquisse de l'arbre A (`arbre-evolution.md` annonçait 16 nœuds mais en dessinait 19) : 16 nœuds = départ + 3 branches × (rang 1, rang 2, deux au rang 3, merveille au rang 4).
- **Vérifications** : exemples de combat recalculés à la main avec la formule ; contrôle « dépôt public ».
- **Non vérifié sur téléphone** : sans objet (aucun code).
- **Problèmes et dette** : tous les chiffres sont des valeurs de départ, non simulés ; le seuil de points (45/40) et la durée de 25 à 35 tours sont des estimations à mesurer par S3. Le moteur d'essai de S1 se déplace dans 4 directions et ne prend que 2 joueurs : à aligner après validation.
- **Prochaine étape** : réponses aux questions GD-Q9 à GD-Q14 et « oui » de l'utilisateur, puis reprise des chiffres d'après les parties simulées.

## 2026-11-27 · Fichier de tri pour la page à cocher

- **Auteur** : Claude, session S2
- **Branche et commit** : `s2-game-design`
- **Objectif** : demande de la session maître : un `catalogue.json` pour la page de tri sur téléphone.
- **Réalisé** : `catalogue.json` : les 54 idées avec groupe, titre, résumé, intérêt, coût, recommandation (27 garder, 25 plus tard, 2 jeter : GD-054 Masques, GD-057 Glaneurs) et lien vers la fiche ; 8 questions (les cinq du rapport, plus taille de carte, brouillard et hasard). Produit par un script de travail hors dépôt qui vérifie que toutes les idées du catalogue y sont et les longueurs demandées.
- **Décisions** : aucune.
- **Vérifications** : `python3 -m json.tool` passe ; contrôle « dépôt public ».
- **Non vérifié sur téléphone** : l'affichage de la page de tri (construite par la session maître).
- **Problèmes et dette** : les liens vers les fiches suivent la règle d'ancre de GitHub, non vérifiés dans un navigateur.
- **Prochaine étape** : attendre le tri.

## 2026-11-27 · Trois fiches pour l'IA : taille de carte, brouillard, hasard

- **Auteur** : Claude, session S2
- **Branche et commit** : `s2-game-design`
- **Objectif** : demande de la session maître pour la session 3 : rendre explicites trois choix qui changent l'architecture de l'IA.
- **Réalisé** : `catalogue.md` : fiches GD-015 (carte maximale et nombre de joueurs), GD-016 (brouillard), GD-017 (hasard des combats), chacune avec ses options, leurs conséquences pour le jeu et pour l'IA, et la recommandation en premier ; le socle GD-090 y renvoie. Le catalogue compte maintenant 54 idées.
- **Décisions** : aucune. Recommandations : 14×14 et 2 à 4 joueurs au plus ; brume avec mémoire (repli : terrain caché seulement) ; combats sans hasard.
- **Vérifications** : contrôle « dépôt public » avant push.
- **Non vérifié sur téléphone** : sans objet.
- **Problèmes et dette** : aucun.
- **Prochaine étape** : attendre le tri.

## 2026-11-27 · Premier catalogue de game design : boucle, arbre, clans

- **Auteur** : Claude, session S2
- **Branche et commit** : `s2-game-design`
- **Objectif** : exécuter la mission (`MISSION.md`) : boucle de jeu, arbre d'évolution, clans, différenciation, socle minimal.
- **Réalisé** : `boucle-de-jeu.md` (quatre X, rythme, victoires, tailles de carte, lisibilité, contraintes pour l'IA) ; `arbre-evolution.md` (formes, coûts, trois arbres esquissés A, B, C et une variante) ; `clans.md` (univers de travail « Terres Levées », 20 clans, asymétrie, triangles, équilibrage, bouquets) ; `catalogue.md` (51 idées GD-001 à GD-090, tableau puis fiches) ; `sources.md` (5 sources).
- **Décisions** : aucune (recherche ; l'utilisateur trie). Recommandations : socle GD-090 (4 clans, arbre A, 11×11, score à 30 tours).
- **Vérifications** : relecture des noms : aucun nom repris d'un jeu existant ; contrôle « dépôt public » avant push.
- **Non vérifié sur téléphone** : sans objet (aucun code). Les tailles de carte (lisibilité des cases) restent à confirmer sur le téléphone par la session 1.
- **Problèmes et dette** : sources consultées par extraits de recherche, sans lecture complète des pages.
- **Prochaine étape** : attendre le tri de l'utilisateur transmis par la session maître ; détailler ensuite les idées retenues (chiffres des unités, arbre A complet).
