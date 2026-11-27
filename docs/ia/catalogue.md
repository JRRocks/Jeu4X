# Catalogue des idées de la session 3 (IA)

> Écrit par Claude, session S3, le 2026-11-27. L'utilisateur trie (« garde IA-003 ») ; seul le
> retenu entre dans le plan (D-07, D-11). Détail des méthodes : `etude.md`. Sources : `sources.md`.
> Coût : S = quelques jours, M = quelques semaines, L = plus d'un mois ou du GPU loué.

| N° | Titre | L'idée en une ligne | Intérêt | Coût |
|---|---|---|---|---|
| IA-001 | Banc d'essai | Moteur sans écran, tournoi automatique, classement Elo des IA, rejeu des parties | Indispensable à tout le reste | S |
| IA-002 | IA à utilité | Une note pondérée par action, l'IA joue la meilleure | Première IA jouable, minuscule, explicable | S |
| IA-003 | Pré-entraînement par évolution | CMA-ES ou algorithme génétique règle les poids de IA-002 sur des millions de parties | « Ne part pas de zéro » à coût quasi nul | S |
| IA-004 | Ligue de versions | Chaque nouvelle IA doit battre l'ancienne et toute la galerie | Progrès réel, pas de cycles ; donne les niveaux | S |
| IA-005 | Planification du tour par évolution | À chaque tour, fait évoluer des séquences d'actions du tour entier | Adaptée aux tours à plusieurs actions | M |
| IA-006 | Niveaux par budget de réflexion | Même IA, plus ou moins de temps de réflexion selon le niveau | Niveaux sans entraîner plusieurs IA | S |
| IA-007 | Difficulté qui s'ajuste | Propose l'IA qui laisse environ une chance sur deux au joueur | Jamais imbattable, jamais ridicule | S |
| IA-008 | Personnalités par clan | Poids et préférences propres à chaque clan | Variété, lisibilité ; lien avec S2 | S |
| IA-009 | Lecture du style du joueur | En cours de partie, l'IA devine si le joueur attaque, bâtit ou explore et répond | IA qui « s'adapte » sans réseau | M |
| IA-010 | Collecte des parties | Replays anonymes envoyés avec accord, rejoués avant acceptation | Base de « l'IA progresse avec les joueurs » | M |
| IA-011 | Imitation des joueurs | Un réseau apprend à jouer comme les humains à partir des parties collectées | IA au jeu humain ; amorce du réseau | M |
| IA-012 | Réseau et jeu contre soi-même | Réseau valeur + politique, recherche courte (Gumbel AlphaZero) | IA la plus forte | L |
| IA-013 | Ligue vivante | Nouvelle version régulière à partir des parties de tous, publiée si elle gagne | La progression collective visible | L |
| IA-014 | IA fantôme | Une IA qui imite le style d'un joueur, à affronter ou à prêter | Original, motivant | M |
| IA-015 | Modèles téléchargés | Les nouvelles IA arrivent comme fichiers séparés, sans nouvel APK | IA à jour sans réinstaller | S |
| IA-016 | Coup expliqué | L'IA à utilité peut dire pourquoi elle a joué (« menace sur ta ville ») | Aide à apprendre, tutoriel | S |
| IA-017 | Exploiteurs | IA entraînées seulement à trouver les failles du champion | Champion robuste face aux humains | M |
| IA-018 | Neuroévolution NEAT | Fait évoluer forme et poids d'un petit réseau | Pédagogique ; peu utile ici | M |
| IA-019 | Apprentissage fédéré | Les téléphones entraînent et ne renvoient que des mises à jour | Confidentialité ; trop lourd pour l'instant | L |

---

## IA-001 · Banc d'essai
- **Description** : le moteur de règles tourne sans écran ; des IA jouent des tournois
  automatiques ; chaque version a un classement (Elo ou TrueSkill) ; tout replay se rejoue et se
  vérifie par empreinte.
- **Pourquoi** : sans lui, impossible de savoir si une IA progresse. Il sert à toutes les autres idées.
- **Coût** : S, une fois le moteur prototype disponible.
- **Dépendances** : `besoins-moteur.md` respecté par la session 1.
- **Sources** : OpenSpiel (interface), Tribes et Stratega (cadres de recherche pour jeux de stratégie au tour par tour).

## IA-002 · IA à utilité
- **Description** : pour chaque action permise, une note = somme de critères pondérés (valeur de
  la case, danger, distance aux villes, économie…). L'IA joue la meilleure, avec un peu de
  hasard pour ne pas être prévisible.
- **Pourquoi** : jouable tout de suite, ne pèse rien, tourne sur tous les téléphones, explicable.
- **Coût** : S. **Dépendances** : IA-001.

## IA-003 · Pré-entraînement par évolution
- **Description** : CMA-ES (ou un algorithme génétique) fait évoluer les poids de IA-002 en
  faisant jouer des milliers de parties contre la ligue (IA-004).
- **Pourquoi** : répond à D-10 (pré-entraînée) sans GPU, sur le PC.
- **Coût** : S ; quelques heures de calcul si le moteur atteint sa vitesse cible.
- **Dépendances** : IA-001, IA-002. **Sources** : CMA-ES (Hansen).

## IA-004 · Ligue de versions
- **Description** : toutes les versions sont gardées ; une candidate doit battre la version en
  service (par exemple 55 % sur 400 parties) sans reculer contre les autres.
- **Pourquoi** : progrès mesuré, pas de cycles ; la galerie fournit les niveaux.
- **Coût** : S. **Dépendances** : IA-001. **Sources** : AlphaStar (ligue).

## IA-005 · Planification du tour par évolution
- **Description** : pendant un temps fixé, l'IA fait évoluer des séquences complètes d'actions de
  son tour et garde la meilleure selon l'évaluation (IA-002 puis réseau).
- **Pourquoi** : conçue pour les jeux où l'on bouge plusieurs unités par tour.
- **Coût** : M. **Dépendances** : IA-001, IA-002. **Sources** : « Online Evolution for
  Multi-Action Adversarial Games » (Justesen et al., 2016), Tribes (RHEA meilleure IA).

## IA-006 · Niveaux par budget de réflexion
- **Description** : le niveau règle le temps ou le nombre d'essais de la planification.
- **Pourquoi** : plusieurs niveaux avec une seule IA ; les niveaux bas restent cohérents.
- **Coût** : S. **Dépendances** : IA-005 ou une recherche.

## IA-007 · Difficulté qui s'ajuste
- **Description** : un classement du joueur, tenu sur le téléphone, choisit l'adversaire de la
  galerie qui lui laisse environ une chance sur deux ; le joueur peut toujours choisir lui-même.
- **Pourquoi** : garde-fou « jamais imbattable » ; garde le joueur motivé.
- **Coût** : S. **Dépendances** : IA-004.

## IA-008 · Personnalités par clan
- **Description** : chaque clan a son profil (poids, préférences : expansion, guerre, science),
  obtenu par évolution avec une récompense légèrement orientée.
- **Pourquoi** : variété et lisibilité ; colle aux clans de la session 2.
- **Coût** : S. **Dépendances** : IA-003, clans de S2.

## IA-009 · Lecture du style du joueur
- **Description** : quelques indicateurs (unités militaires produites, villes fondées, cases
  explorées) classent le joueur en cours de partie ; l'IA choisit le profil de réponse prévu.
- **Pourquoi** : impression d'une IA qui s'adapte, sans entraînement sur le téléphone.
- **Coût** : M. **Dépendances** : IA-002, IA-008.

## IA-010 · Collecte des parties
- **Description** : replays anonymes (`besoins-donnees.md`), envoyés seulement avec accord,
  rejoués et contrôlés avant d'être acceptés.
- **Pourquoi** : sans données de joueurs, pas d'IA qui « progresse avec les joueurs ».
- **Coût** : M. **Dépendances** : session 4, jeu distribué.

## IA-011 · Imitation des joueurs
- **Description** : un réseau apprend à prédire le coup joué par les bons joueurs.
- **Pourquoi** : IA au style humain ; point de départ du réseau d'IA-012.
- **Coût** : M. **Dépendances** : IA-010, codage de l'état (E7).

## IA-012 · Réseau et jeu contre soi-même
- **Description** : réseau valeur + politique, entraîné par jeu contre soi-même avec une
  recherche guidée ; variante Gumbel pour rester efficace avec peu de simulations sur le téléphone.
- **Pourquoi** : la méthode la plus forte connue pour ce type de jeu.
- **Coût** : L (GPU, à chiffrer sur un prototype réduit). **Dépendances** : IA-001, moteur rapide,
  codage de l'état. **Sources** : AlphaZero, Gumbel AlphaZero, Expert Iteration.

## IA-013 · Ligue vivante
- **Description** : à intervalles réguliers, entraînement sur les nouvelles parties, tournoi,
  publication si la candidate gagne (IA-004, IA-015).
- **Pourquoi** : la progression collective rendue visible (classement de l'IA au fil des mois).
- **Coût** : L. **Dépendances** : IA-010, IA-011 ou IA-012, IA-015.

## IA-014 · IA fantôme
- **Description** : à partir des parties d'un joueur, un profil d'utilité (ou un petit réseau
  ajusté) imite son style ; on peut l'affronter ou le prêter à un ami.
- **Pourquoi** : idée originale et motivante.
- **Coût** : M. **Dépendances** : IA-002 ou IA-011 ; accord du joueur.

## IA-015 · Modèles téléchargés
- **Description** : les profils et réseaux sont des fichiers à part, listés dans un index publié
  avec les versions du jeu ; le téléphone les télécharge s'ils sont compatibles et vérifiés.
- **Pourquoi** : l'IA s'améliore sans réinstaller le jeu.
- **Coût** : S. **Dépendances** : brique `maj-auto-github` de la Library (à adapter).

## IA-016 · Coup expliqué
- **Description** : l'IA à utilité connaît le critère qui a décidé son coup et peut l'afficher.
- **Pourquoi** : aide le joueur à comprendre ; utile au tutoriel et au débogage.
- **Coût** : S. **Dépendances** : IA-002 ; tout affichage passe par un aperçu (S6).

## IA-017 · Exploiteurs
- **Description** : des IA dont le seul but est de battre le champion ; le champion apprend
  ensuite contre elles.
- **Pourquoi** : bouche les failles qu'un humain finirait par trouver.
- **Coût** : M. **Dépendances** : IA-004. **Sources** : AlphaStar.

## IA-018 · Neuroévolution NEAT
- **Description** : un petit réseau dont la structure évolue.
- **Pourquoi** : peu utile ici face aux réseaux convolutifs ; listée pour complétude.
- **Coût** : M. **Dépendances** : IA-001. **Sources** : NEAT (Stanley et Miikkulainen).

## IA-019 · Apprentissage fédéré
- **Description** : les téléphones entraînent localement et n'envoient que des mises à jour.
- **Pourquoi** : protège mieux les données, mais lourd, coûteux en batterie, et inutile tant
  qu'un replay anonyme suffit. Non recommandé pour l'instant.
- **Coût** : L. **Dépendances** : IA-010, IA-012.
