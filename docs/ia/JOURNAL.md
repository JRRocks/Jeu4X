# Journal de la session 3 : IA évolutive

> Entrée la plus récente en haut. Gabarit de `docs/JOURNAL.md`.

## 2026-11-27 · Fichier de tri pour la page à cocher

- **Auteur** : Claude, session S3
- **Branche et commit** : `s3-ia`
- **Objectif** : consigne de la session maître : `catalogue.json` pour la page de tri sur téléphone.
- **Réalisé** : 19 idées en 4 groupes. Recommandations : 8 à garder (IA-001 à IA-008), 9 pour plus tard, 2 à jeter (IA-018, IA-019). Une question sur la location de puissance de calcul. La question sur l'envoi des parties est fusionnée avec celle de S4.
- **Vérifications** : `python3 -m json.tool` valide ; longueurs des titres et résumés contrôlées ; contrôle `depot-public`.
- **Prochaine étape** : attendre le tri.

## 2026-11-27 · Sources vérifiées une à une

- **Auteur** : Claude, session S3
- **Branche et commit** : `s3-ia`
- **Objectif** : consigne de la session maître : sources réellement consultées (règle 8).
- **Réalisé** : 24 sources ouvertes (par un sous-agent, pages lues le 2026-11-27). 19 tenaient telles quelles. 3 ont été corrigées : le titre de l'article de Justesen et al. ; le résultat de Tribes, où RHEA est première et MCTS troisième ; LiteRT, qui succède à TensorFlow Lite et n'en est pas un renommage. Une source n'a pas pu être vérifiée : le quota GPU de Kaggle, pour lequel aucun chiffre officiel n'a été trouvé. Une affirmation sans source a été retirée de l'étude et du catalogue : la mention d'AlphaGo. Ajouts : EvoTorch, ainsi que les conditions d'usage de GitHub Actions dans les garde-fous.
- **Décisions** : aucune.
- **Vérifications** : contrôle `depot-public`.
- **Non vérifié sur téléphone** : sans objet.
- **Problèmes et dette** : OpenReview, Science, MIT Press et Springer refusent la lecture automatique ; ces notices ont été lues par Crossref ou par une page miroir.
- **Prochaine étape** : attendre le tri de l'utilisateur.

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
