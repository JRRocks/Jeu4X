# Mission de la session 3 : l'IA évolutive

> Écrite par la session maître le 2026-11-27. Branche : `s3-ia`. Dossier : `docs/ia/`. Préfixe des idées : `IA-`.
> Lire d'abord `CLAUDE.md`, `docs/ORGANISATION.md` et `docs/DECISIONS.md`.

## Ce que l'utilisateur a demandé

Une IA capable de se développer en même temps que le joueur : plus il y a de joueurs, plus il y a de parties, plus l'IA se développe et s'adapte. Avec les méthodes d'entraînement connues : apprentissage par renforcement, mutation génétique et autres procédés. Pré-entraînée, pour qu'elle ne commence pas de zéro (D-10).

## Étape A : étude et architecture (maintenant)

Le moteur de jeu n'existe pas encore (la session 1 compare les technologies) : rien dans le code du jeu (D-11). Un petit essai jetable pour vérifier une affirmation est permis ; rien de ces essais n'est poussé.

1. **Les méthodes** : apprentissage par renforcement (jeu contre soi-même, PPO, recherche arborescente Monte-Carlo guidée par réseau façon AlphaZero, MuZero, ligues et entraînement par population), méthodes évolutionnaires (algorithmes génétiques, neuroévolution type NEAT, CMA-ES, stratégies d'évolution), apprentissage par imitation des parties humaines, IA à fonctions d'utilité aux poids évolués, modélisation de l'adversaire, difficulté qui s'ajuste, IA « fantôme » qui imite le style d'un joueur, personnalités par clan. Pour chacune : ce qu'elle apporte ici, son coût, sa difficulté.
2. **Où l'IA apprend** : pré-entraînement hors ligne (PC, machine louée), apprentissage continu central à partir des parties de tous, adaptation sur le téléphone, apprentissage fédéré. Comment les nouveaux modèles arrivent sur les téléphones.
3. **Les contraintes du téléphone** : taille du modèle, temps de réflexion par tour, batterie, jeu hors ligne.
4. **Les garde-fous** : versions numérotées et classement Elo des IA, une nouvelle version doit battre l'ancienne, une IA jamais imbattable (niveaux), parties truquées ou empoisonnées, reproductibilité, coût de calcul et options gratuites.
5. **Une feuille de route par étapes**, de l'IA la plus simple à l'IA qui apprend des joueurs.

## À envoyer tôt à la session maître

`docs/ia/besoins-moteur.md`, poussé et signalé à la session maître dès que possible, avant le reste de l'étude : ce que les règles du jeu doivent offrir pour l'entraînement (parties sans écran, vitesse visée en parties par seconde, reproductibilité par graine, liste des actions permises, copie d'un état, codage de l'état et des actions, format de replay) et ce que cela implique pour le choix de la technologie et du langage d'entraînement. La session maître le transmet à la session 1, qui compare les technologies en ce moment.

## Livrables

- `docs/ia/etude.md`, `architecture.md`, `besoins-moteur.md`, `besoins-donnees.md` (ce que l'IA doit recevoir des parties, pour la session 4), `catalogue.md` (`IA-NNN`), `sources.md`, `JOURNAL.md`.
- Un rapport à la session maître : la recommandation, la feuille de route, les questions pour l'utilisateur.

## Voisins

La session 4 étudie où et comment regrouper les données des parties ; elle lira `besoins-donnees.md`. La session 2 conçoit les clans : les personnalités d'IA par clan sont un point de rencontre.
