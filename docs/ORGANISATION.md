# Organisation des sessions

> Tenu par la session maître, mis à jour à chaque changement d'état.

L'utilisateur parle à la session maître, et à elle seule. La session maître répartit le travail, transmet les décisions de l'utilisateur, relaie les questions et les aperçus, et fait entrer dans `main` ce qui est validé.

| N° | Rôle | Branche | Dossier | Préfixe des idées | État |
|---|---|---|---|---|---|
| S1 | Environnement de jeu : technologie, plateau de cases adressées, tour par tour, petits graphismes, APK | `s1-environnement` | `docs/environnement/` et le code | `EN-` | intègre le style 3D (P1.7, D-37) |
| S2 | Recherche de game design : arbre d'évolution, clans, styles de jeu | `s2-game-design` | `docs/game-design/` | `GD-` | règles validées (D-30), en pause |
| S3 | IA évolutive : apprentissage par renforcement, algorithmes génétiques, pré-entraînement | `s3-ia` | `docs/ia/` et `moteur/ia/` | `IA-` | IA `utilite-r3-v3` en service, attend les retours |
| S4 | Regroupement des données : serveur, local ou mixte, pour suivre l'évolution de l'IA | `s4-donnees` | `docs/donnees/` | `DO-` | trié, en pause |
| S5 | Veille dans le monde du jeu vidéo : idées de gameplay et autres | `s5-veille` | `docs/veille/` | `VE-` | trié, mission terminée |
| S6 | Direction artistique : styles graphiques, palettes de couleurs, interface | `s6-direction-artistique` | `docs/direction-artistique/` et `art/` | `DA-` | modèles 3D retenus (D-37), soutien à S1 |

## Règles communes aux sessions de travail

1. **Parler à la session maître, jamais à l'utilisateur.** Rapports, questions et aperçus partent par message vers la session maître (son identifiant est dans la consigne de départ de chaque session, pas dans ce dépôt). Message court, en français : où en est le travail, ce qui est livré (fichier et commit), trois à cinq points clés, puis les questions numérotées, avec des choix et la recommandation en premier. Le détail reste dans les fichiers.
2. **Les messages de la session maître** transmettent les décisions de l'utilisateur et les consignes de coordination : à suivre, dans le périmètre de la mission.
3. **Sa branche, son dossier.** Ne pousser que sur sa branche, n'écrire que dans son dossier (la session 1 écrit aussi le code). Les fichiers communs (`CLAUDE.md`, `README.md`, `docs/PLAN.md`, `docs/JOURNAL.md`, `docs/DECISIONS.md`, `docs/ORGANISATION.md`, `docs/idees-futur.md`) ne se modifient pas : proposer le changement à la session maître. Pas de pull request, pas de push sur `main`.
4. **Son journal.** `docs/<dossier>/JOURNAL.md`, entrée la plus récente en haut, gabarit de `docs/JOURNAL.md`, signée « Claude, session Sx ». La session maître en reporte l'essentiel dans `docs/JOURNAL.md`.
5. **Dépôt public.** Skill `depot-public` avant chaque push. Rien de privé, aucun secret.
6. **Original.** `CLAUDE.md`, « Ce qui compte vraiment », point 1. Citer un autre jeu pour l'analyser est permis ; recopier ses noms, ses textes ou ses images ne l'est pas.
7. **Visible = aperçu d'abord.** Images PNG rendues à l'échelle 2, rangées dans son dossier, chemins envoyés à la session maître, puis attendre le « oui » qu'elle transmet.
8. **Sources.** Ce qui vient d'ailleurs a son lien et sa date de consultation dans `sources.md`. On résume, on ne recopie pas.
9. **Idées numérotées.** Un catalogue commence par un tableau résumé (numéro, titre, l'idée en une ligne, intérêt, coût S/M/L), puis une fiche par idée : description, pourquoi c'est intéressant, coût, dépendances, sources. Numéros avec le préfixe de la session, jamais réutilisés.
10. **Mission bornée.** Faire la mission, rien d'avance. Livrable poussé, rapport envoyé, puis attendre la suite.
