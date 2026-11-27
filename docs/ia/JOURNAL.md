# Journal de la session 3 : IA évolutive

> Entrée la plus récente en haut. Gabarit de `docs/JOURNAL.md`.

## 2026-11-27 · Les cas ouverts dans l'architecture (brouillard, hasard, taille)

- **Auteur** : Claude, session S3
- **Branche et commit** : `s3-ia`
- **Objectif** : consigne de la session maître : couvrir les deux cas là où ils changent l'architecture.
- **Réalisé** : `architecture.md`, § 7 : information complète ou brouillard, combats avec ou sans hasard, taille de carte et nombre de joueurs.
- **Décisions** : aucune.
- **Vérifications** : contrôle `depot-public`. Les multiplicateurs de coût sont des estimations.
- **Non vérifié sur téléphone** : sans objet.
- **Problèmes et dette** : aucun nouveau.
- **Prochaine étape** : attendre le tri de l'utilisateur.

## 2026-11-27 · Étude de l'IA : méthodes, architecture, catalogue

- **Auteur** : Claude, session S3
- **Branche et commit** : `s3-ia`
- **Objectif** : terminer l'étape A de la mission.
- **Réalisé** : `etude.md` (14 méthodes notées, lieux d'apprentissage, contraintes du téléphone, garde-fous, feuille de route R0 à R6) ; `architecture.md` ; `besoins-donnees.md` pour S4 ; `catalogue.md` (IA-001 à IA-019) ; `sources.md` complété.
- **Décisions** : aucune (propositions seulement).
- **Vérifications** : contrôle `depot-public` sans signalement. Aucun essai de calcul : temps d'inférence et coûts d'entraînement sont des estimations.
- **Non vérifié sur téléphone** : temps d'évaluation d'un réseau sur un téléphone moyen (estimé 5 à 20 ms).
- **Problèmes et dette** : sources citées de mémoire, liens à revérifier.
- **Prochaine étape** : attendre le tri de l'utilisateur et le choix de technologie (S1).

## 2026-11-27 · Ce que les règles du jeu doivent offrir à l'IA

- **Auteur** : Claude, session S3
- **Branche et commit** : `s3-ia`
- **Objectif** : livrer en priorité `besoins-moteur.md` pour la comparaison des technologies (session 1).
- **Réalisé** : `besoins-moteur.md` (11 exigences, interface minimale, tableau des langages, recommandation : règles dans un module à part, de préférence en Rust) ; `sources.md` amorcé.
- **Décisions** : aucune (propositions seulement).
- **Vérifications** : aucune mesure ; les vitesses sont des cibles à vérifier sur le premier prototype.
- **Non vérifié sur téléphone** : sans objet.
- **Problèmes et dette** : sources citées de mémoire, liens à revérifier pendant l'étude.
- **Prochaine étape** : rapport à la session maître, puis la suite de l'étude (`etude.md`, `architecture.md`, `besoins-donnees.md`, `catalogue.md`) si elle le confirme.
