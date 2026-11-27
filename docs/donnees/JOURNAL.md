# Journal de la session 4 : regroupement des données

> Entrée la plus récente en haut. On ne réécrit pas le passé : une erreur se corrige par une nouvelle entrée.

---

## 2026-11-27 · Fichier de tri pour la page à cocher

- **Auteur** : Claude, session S4
- **Branche et commit** : `s4-donnees` (voir le dernier commit de la branche)
- **Objectif** : demande de la session maître : `catalogue.json` pour la page de tri sur téléphone.
- **Réalisé** : `docs/donnees/catalogue.json` : 21 idées en 5 groupes, avec recommandation (7 à garder, 13 pour plus tard, 1 à jeter : DO-017) et les 4 questions DO-Q1 à DO-Q4 (DO-Q1 couvre aussi l'envoi des parties demandé par S3).
- **Décisions** : aucune.
- **Vérifications** : `python3 -m json.tool` sans erreur ; longueurs des titres, résumés et libellés contrôlées par script.
- **Non vérifié sur téléphone** : rendu de la page (construite par la session maître).
- **Problèmes et dette** : aucun.
- **Prochaine étape** : attendre le tri groupé.

## 2026-11-27 · Étude revue avec les besoins de l'IA, règles de GitHub Actions

- **Auteur** : Claude, session S4
- **Branche et commit** : `s4-donnees` (voir le dernier commit de la branche)
- **Objectif** : demandes de la session maître : confronter les hypothèses à `docs/ia/besoins-donnees.md` (S3, commits a3e1385 et 9c5ecd0) ; vérifier ce que GitHub permet pour les tournois IA contre IA.
- **Réalisé** : `schema-donnees.md` reçoit une section « Revue S3 » : champs `replay_hash`, `checks`, `encoding_version`, `think_time` ajoutés ; renommages `player_id` et `rules_version` ; rythme de jeu relevé jusqu'à 10 parties par jour. Volumes et coûts recalculés dans le schéma, l'étude et la recommandation : 0 € jusqu'à environ 1 000 joueurs, jusqu'à 8 $ par mois à 10 000 joueurs très actifs. IA fantôme ajoutée au volet RGPD. Nouvelle section 7 de l'étude sur GitHub Actions, avec la limite à ne pas franchir.
- **Décisions** : aucune (propositions).
- **Vérifications** : conditions et limites GitHub relues sur les pages officielles le 2026-11-27 (`sources.md`).
- **Non vérifié sur téléphone** : rien (étude).
- **Problèmes et dette** : les seuils anti-abus de GitHub ne sont pas publiés.
- **Prochaine étape** : attendre le tri groupé de l'utilisateur.

## 2026-11-27 · Première étude : où regrouper les parties pour suivre l'IA

- **Auteur** : Claude, session S4
- **Branche et commit** : `s4-donnees` (voir le dernier commit de la branche)
- **Objectif** : mission `docs/donnees/MISSION.md` : quoi regrouper, où, à quel coût, comment voir l'évolution de l'IA, comment les modèles reviennent aux téléphones, recommandation par étapes.
- **Réalisé** : `etude.md` (comparatif de 8 formules), `recommandation.md` (3 étapes), `schema-donnees.md` (replay compact ≈ 2 Ko, volumes), `catalogue.md` (DO-001 à DO-021), `sources.md`.
- **Décisions** : aucune ; propositions seulement (D-11). Hypothèses H1 à H8 écrites dans `schema-donnees.md`.
- **Vérifications** : tarifs relevés sur les pages officielles le 2026-11-27 (`sources.md`) ; chiffres non vérifiés signalés comme tels. Aucun code, aucun compte, aucune dépense.
- **Non vérifié sur téléphone** : rien (étude).
- **Problèmes et dette** : `docs/ia/besoins-donnees.md` (S3) n'existait pas encore (la branche `s3-ia` ne porte encore que `besoins-moteur.md`, dont l'hypothèse de ≈ 1 000 actions par partie confirme H2) : à reprendre dès qu'il arrive.
- **Prochaine étape** : attendre le tri de l'utilisateur et les besoins de S3.
