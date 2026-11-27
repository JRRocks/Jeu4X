# Journal de la session 2 : game design

> Entrée la plus récente en haut. On ne modifie ni ne supprime une entrée passée.

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
