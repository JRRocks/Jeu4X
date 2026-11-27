# Étude : quelles méthodes pour une IA qui progresse

> Écrit par Claude, session S3, le 2026-11-27. Étape A de la mission : étude seulement, rien dans
> le code du jeu (D-11). Les idées retenables sont numérotées dans `catalogue.md` (`IA-NNN`).
> Les sources sont dans `sources.md`. Les chiffres de coût sont des estimations, pas des mesures.

## 0. Ce qui rend ce jeu particulier pour une IA

Un 4X au tour par tour sur petite carte ressemble aux jeux de plateau (tour par tour, parties
courtes), mais avec trois difficultés que les échecs ou le go n'ont pas :

1. **Beaucoup d'actions par tour.** Un joueur bouge plusieurs unités, attaque, produit,
   recherche, puis finit son tour. Le nombre de combinaisons d'un tour est énorme. Les travaux
   sur des jeux de ce genre (Hero Academy, le cadre de recherche *Tribes*, *Stratega*) montrent
   que la recherche arborescente classique s'y noie ; on décompose le tour en actions simples
   (une par décision, voir `besoins-moteur.md`, E4) ou on planifie le tour entier par évolution
   (IA-005).
2. **Plusieurs adversaires** (2 à 4 joueurs) : pas de simple « gagner/perdre » à deux. Les
   méthodes pensées pour deux joueurs doivent être adaptées (récompense par rang ou par score).
3. **Information cachée probable** (brouillard de guerre) : il faut une vue par joueur et, pour
   chercher, des tirages d'états compatibles avec ce qu'on voit.

À cela s'ajoutent les contraintes du téléphone (§ 3) et la demande de l'utilisateur : une IA
qui **progresse avec le nombre de joueurs et de parties** (D-10).

## 1. Les méthodes

Notes : **apport** (pour ce jeu), **coût** (calcul pour l'entraîner), **difficulté** (travail de
développement). Échelle : faible, moyen, élevé.

### 1.1 IA à fonctions d'utilité, poids évolués
Pour chaque action permise, l'IA calcule une note à partir de critères pondérés (menace sur
l'unité, valeur de la case, distance à une ville ennemie, économie…) et joue la meilleure.
Les poids ne sont pas réglés à la main : un algorithme génétique ou CMA-ES les fait évoluer en
faisant jouer des milliers de parties.
- **Apport** : IA jouable dès le premier prototype, compréhensible (on sait pourquoi elle joue un
  coup), minuscule sur le téléphone (quelques dizaines de nombres), réflexion instantanée. Les
  personnalités par clan se font en changeant les poids (IA-011).
- **Coût** : faible, quelques heures de PC (il faut un moteur rapide, `besoins-moteur.md`).
- **Difficulté** : faible.
- **Limite** : plafonne ; ne découvre pas de stratégie que ses critères ne savent pas voir.

### 1.2 Algorithmes génétiques, CMA-ES, stratégies d'évolution
Familles d'optimisation sans dérivée : on garde les meilleurs individus d'une population, on les
croise et on les mute. CMA-ES apprend en plus la forme de la recherche ; il excelle jusqu'à
quelques centaines de paramètres. Les stratégies d'évolution « à la OpenAI » estiment un
gradient par perturbations et passent à l'échelle de réseaux entiers, en parallèle parfait.
- **Apport** : le moteur d'amélioration des IA à utilité (1.1) et des petits réseaux ; tolère
  des récompenses rares (victoire en fin de partie) ; facile à paralléliser sur des machines
  gratuites.
- **Coût** : faible à moyen. **Difficulté** : faible (bibliothèques prêtes : `cma`, `evotorch`).

### 1.3 Neuroévolution type NEAT
Fait évoluer la structure et les poids d'un petit réseau à la fois.
- **Apport** : historiquement important, mais pour une carte en grille il reste en dessous des
  réseaux convolutifs entraînés par gradient. Intérêt surtout pédagogique ou pour un petit
  « cerveau » de décision stratégique (quand attaquer, quand s'étendre).
- **Coût** : moyen. **Difficulté** : moyenne. **Avis** : non prioritaire.

### 1.4 Planification en ligne par évolution du tour (Online Evolutionary Planning, RHEA)
À chaque tour, l'IA fait évoluer pendant une fraction de seconde des *séquences d'actions du
tour entier*, notées par une fonction d'évaluation de l'état final du tour.
- **Apport** : conçu précisément pour les jeux à plusieurs actions par tour ; meilleur que la
  recherche arborescente classique dans ces jeux d'après la littérature. Fonctionne sans
  réseau, avec l'évaluation de 1.1 ; s'améliore si l'évaluation est un réseau appris.
- **Coût** : nul à l'entraînement ; du calcul sur le téléphone, réglable (budget de temps).
- **Difficulté** : moyenne.

### 1.5 Recherche arborescente Monte-Carlo (MCTS)
Explore l'arbre des coups par simulations. Sans réseau, les simulations vont jusqu'à la fin au
hasard ou avec une politique simple.
- **Apport** : niveau réglable par le nombre de simulations (niveaux de difficulté gratuits).
  Faible sur les tours à nombreuses actions s'il est employé naïvement ; à combiner avec des
  macro-actions ou des politiques par unité.
- **Coût** : nul à l'entraînement ; calcul sur le téléphone. **Difficulté** : moyenne.

### 1.6 Jeu contre soi-même avec réseau et recherche (AlphaZero, Gumbel AlphaZero, Expert Iteration)
Un réseau prédit pour chaque état la valeur (qui va gagner) et une politique (quels coups sont
bons) ; la recherche guidée par ce réseau joue mieux que le réseau seul ; le réseau apprend à
imiter la recherche ; on recommence. La variante « Gumbel » (2022) reste efficace avec très peu
de simulations, ce qui compte pour un téléphone.
- **Apport** : la méthode qui a donné les IA les plus fortes dans les jeux de plateau ; découvre
  des stratégies seule, sans parties humaines. Le réseau sert ensuite seul (sans recherche) ou
  avec une recherche courte sur le téléphone.
- **Coût** : élevé : des jours de GPU pour un jeu de cette taille (estimation, à confirmer sur un
  prototype réduit). **Difficulté** : élevée.
- **Adaptations** : plusieurs joueurs (valeur par joueur), information cachée (tirages d'états),
  actions nombreuses (une décision à la fois, masque d'actions).

### 1.7 MuZero
Comme AlphaZero, mais le réseau apprend aussi un modèle du jeu, sans connaître les règles.
- **Apport ici** : faible. On possède les règles exactes et rapides ; MuZero coûte plus pour
  rien. **Avis** : écarté.

### 1.8 Apprentissage par renforcement sans recherche (PPO)
Le réseau apprend directement en jouant, récompensé par la victoire ou le score.
- **Apport** : plus simple à brancher qu'AlphaZero, IA directement utilisable sur le téléphone
  (une évaluation du réseau par décision). Moins efficace que la recherche dans les jeux de
  stratégie à long terme ; sensible aux réglages.
- **Coût** : moyen à élevé. **Difficulté** : moyenne.

### 1.9 Ligues et entraînement par population (AlphaStar, PBT)
Au lieu d'un seul champion, on entretient une **ligue** : versions passées, « exploiteurs »
chargés de trouver les failles du champion, styles variés. Le champion doit battre tout le
monde, ce qui évite qu'il tourne en rond (A bat B, B bat C, C bat A). L'entraînement par
population règle en plus les réglages d'apprentissage au fil de l'eau.
- **Apport** : rend l'IA robuste face aux humains, qui jouent de façons imprévues ; donne
  naturellement une **galerie de niveaux et de styles** à proposer aux joueurs.
- **Coût** : multiplie le coût de 1.6 ou 1.8 (à garder petit : 4 à 8 membres).
- **Difficulté** : moyenne, par-dessus 1.6.

### 1.10 Apprentissage par imitation des parties humaines
Le réseau apprend à prédire le coup qu'un humain a joué (comme la première version d'AlphaGo).
- **Apport** : c'est le cœur de « l'IA qui progresse avec les joueurs » : chaque partie
  envoyée enrichit le jeu de données. Donne une IA au jeu « humain » ; sert d'amorce au jeu
  contre soi-même. Les mauvais coups humains se filtrent par niveau du joueur.
- **Coût** : faible à moyen (entraînement supervisé). **Difficulté** : moyenne.
- **Dépend** : du regroupement des parties (session 4, `besoins-donnees.md`) et du consentement
  des joueurs.

### 1.11 Modélisation de l'adversaire
L'IA estime en cours de partie le style du joueur (agressif, bâtisseur, explorateur) et choisit
la réponse prévue pour ce style.
- **Apport** : sentiment d'une IA qui « s'adapte » sans aucun entraînement sur le téléphone.
- **Coût** : faible. **Difficulté** : moyenne.

### 1.12 Difficulté qui s'ajuste
Mesurer le niveau du joueur (classement, résultats récents) et lui opposer l'IA de la galerie
qui lui donne une chance sur deux environ, ou réduire le budget de recherche.
- **Apport** : l'IA n'est jamais imbattable ni ridicule (garde-fou de la mission).
- **Coût** : nul. **Difficulté** : faible.

### 1.13 IA « fantôme » d'un joueur
Un petit modèle imite le style d'un joueur précis (à partir de ses parties), pour l'affronter
ou le prêter à d'autres (« jouer contre le fantôme d'un ami »).
- **Apport** : original et motivant ; possible par réglage fin d'un modèle d'imitation (1.10)
  ou, plus simplement, par les poids d'une IA à utilité ajustés aux choix du joueur.
- **Coût** : faible par joueur si fait à partir de la version à utilité.
- **Difficulté** : moyenne. **Attention** : données personnelles (le style d'un joueur) :
  consentement.

### 1.14 Personnalités par clan
Même moteur d'IA, préférences différentes par clan : poids d'utilité, récompenses légèrement
différentes en entraînement (bonus d'expansion, de guerre, de science…), ou membres différents
de la ligue.
- **Apport** : variété et lisibilité (« ce clan attaque tôt »). Point de rencontre avec la
  session 2. **Coût** : faible. **Difficulté** : faible.

## 2. Où l'IA apprend

| Lieu | Ce qui s'y fait | Coût | Avis |
|---|---|---|---|
| **Hors ligne, PC ou machine louée** | pré-entraînement : évolution des IA à utilité, jeu contre soi-même | gratuit sur le PC ; GPU gratuits limités (Kaggle, Colab) ; GPU loué quelques euros l'heure | **indispensable** (D-10 : ne pas partir de zéro) |
| **Central, à partir des parties de tous** | les parties remontent ; l'imitation et la ligue s'entraînent dessus ; une nouvelle version sort régulièrement | un lieu de stockage (session 4) + des sessions d'entraînement périodiques | **le moteur de la progression collective** |
| **Sur le téléphone** | adaptation légère : estimation du style du joueur, choix du niveau, petits ajustements de poids d'une IA à utilité | batterie négligeable si on n'entraîne pas de réseau | **oui, mais léger** |
| **Apprentissage fédéré** | chaque téléphone entraîne et ne renvoie que des mises à jour du modèle | infrastructure lourde, gain de confidentialité réel mais complexe, batterie | **non pour l'instant** : un replay de quelques kilooctets, anonyme, suffit |

**Comment les nouveaux modèles arrivent sur les téléphones** : le premier modèle est dans l'APK
(jeu hors ligne dès l'installation). Les suivants sont des fichiers de quelques mégaoctets
publiés à part, par exemple dans les Releases GitHub du dépôt public, à côté de l'APK (la brique
`maj-auto-github` de la Library sait déjà le faire pour l'APK). Le jeu vérifie l'empreinte et la
version des règles du modèle avant de l'utiliser ; s'il est incompatible, il garde l'ancien.
Détail dans `architecture.md`.

## 3. Les contraintes du téléphone

| Contrainte | Cible | Conséquence |
|---|---|---|
| Taille du modèle | **moins de 5 Mo** par modèle, quelques modèles dans l'APK | petit réseau convolutif (de l'ordre d'un million de paramètres), quantifié en entiers 8 bits |
| Temps de réflexion | **1 à 2 secondes au plus** pour tout le tour de l'IA, qui compte souvent 5 à 20 décisions | une évaluation de réseau coûte de l'ordre de 5 à 20 ms sur le processeur d'un téléphone moyen (estimation pour un réseau de cette taille sur 16×16) : réseau seul ou recherche très courte (Gumbel), pas des centaines de simulations par décision |
| Batterie | l'IA ne doit pas chauffer le téléphone | budget de temps fixe par tour ; pas d'entraînement de réseau sur le téléphone |
| Jeu hors ligne | **toujours jouable sans réseau** | modèle embarqué ; parties gardées puis envoyées plus tard, si le joueur l'accepte |
| Diversité des téléphones | anciens téléphones compris | niveaux faibles = IA à utilité (aucun réseau), toujours disponibles |

## 4. Les garde-fous

1. **Versions numérotées** : chaque IA porte un numéro, la version des règles et du codage pour
   lesquelles elle est faite, et l'empreinte de son fichier.
2. **Classement Elo** (ou TrueSkill, mieux adapté à plusieurs joueurs) de toutes les versions,
   tenu par des tournois automatiques.
3. **Une nouvelle version doit battre l'ancienne** : par exemple au moins 55 % de victoires sur
   400 parties ou plus contre la version en service, ET pas de recul contre les versions plus
   anciennes de la galerie (pas de cycle). Sinon, elle n'est pas publiée.
4. **Jamais imbattable** : le joueur choisit un niveau ; chaque niveau est une IA de la galerie
   ou un budget de recherche ; la difficulté qui s'ajuste (1.12) le guide. Les niveaux faibles
   ne doivent pas être idiots : on les affaiblit en limitant la réflexion, pas en jouant au
   hasard.
5. **Parties truquées ou empoisonnées** : un tricheur peut envoyer des parties fausses ou
   volontairement absurdes pour fausser l'IA. Parades : le serveur **rejoue chaque replay** avec
   le moteur (une partie impossible est rejetée, l'empreinte finale doit correspondre) ; on
   limite le poids de chaque joueur dans les données ; on filtre par niveau ; une nouvelle
   version passe toujours le test 3 sur des parties entre IA, pas seulement sur les données
   humaines.
6. **Reproductibilité** : graine dans chaque partie, entraînement noté (version du code, graine,
   données utilisées), tout modèle publié peut être reconstruit ou au moins rejoué.
7. **Coût de calcul et options gratuites** : PC de l'utilisateur ; GPU gratuits de Kaggle et
   Colab (quotas hebdomadaires, à vérifier au moment de s'en servir) ; machines de GitHub
   Actions, gratuites pour un dépôt public mais limitées en durée par tâche et à réserver à du
   calcul léger (tournois, évolution d'IA à utilité) : leurs conditions d'usage sont à relire
   avant. Les GPU loués seulement pour l'étape AlphaZero, si l'utilisateur l'accepte.

## 5. Feuille de route proposée

Chaque étape donne une IA jouable ; on ne passe à la suivante que si elle apporte un gain
mesuré (classement).

| Étape | Ce qu'on obtient | Prérequis | Coût |
|---|---|---|---|
| **R0. Le banc d'essai** | moteur sans écran, IA au hasard, mesure de vitesse, replays, tournoi automatique et classement | premier prototype du moteur (S1) | faible |
| **R1. IA à utilité** | une IA honnête, réglée à la main, niveaux par budget | R0 | faible |
| **R2. Pré-entraînement par évolution** | poids évolués par CMA-ES contre une ligue de versions ; personnalités par clan | R1 | faible (PC) |
| **R3. Planification du tour** | OEP ou recherche courte avec l'évaluation de R2 ; niveaux supérieurs | R2 | faible |
| **R4. Les parties des joueurs** | collecte des replays (avec accord), rejeu de contrôle ; IA ajustée au style du joueur ; difficulté qui s'ajuste | S4 (stockage), jeu publié | moyen |
| **R5. Le réseau** | réseau valeur + politique appris par imitation des parties de R2 à R4, puis par jeu contre soi-même (Gumbel AlphaZero) ; IA forte | R0 rapide, GPU | élevé |
| **R6. La ligue vivante** | nouvelle version régulière à partir des parties de tous, publiée seulement si elle bat l'ancienne ; IA fantômes | R4, R5 | moyen à élevé |

R0 à R3 suffisent pour sortir un jeu avec une IA crédible et variée, sans réseau ni GPU. La
partie « l'IA progresse avec les joueurs » commence à R4 et prend toute sa force à R5 et R6.
