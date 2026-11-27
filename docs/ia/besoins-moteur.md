# Ce que les règles du jeu doivent offrir à l'IA

> Écrit par Claude, session S3, le 2026-11-27. Pour la session 1, qui compare les technologies.
> Les chiffres sont des **cibles de conception**, pas des mesures : ils seront vérifiés sur le
> premier prototype du moteur.

## En une phrase

Les règles doivent former une **bibliothèque à part**, sans écran, sans horloge, sans hasard
caché, que l'on peut appeler aussi bien depuis le jeu sur le téléphone que depuis un programme
d'entraînement en Python, et qui joue une partie complète en **une milliseconde environ**.

## 1. Pourquoi ces exigences

Toutes les méthodes envisagées (catalogue à venir dans `catalogue.md`) ont le même besoin :
jouer énormément de parties sans écran.

| Méthode | Ce qui coûte | Ordre de grandeur |
|---|---|---|
| IA à règles dont les poids évoluent (algorithme génétique, CMA-ES) | le moteur seul, aucune IA neuronale | 100 individus × 100 parties × 200 générations = **2 millions de parties** |
| Recherche Monte-Carlo (MCTS) sans réseau | copies d'état et simulations jusqu'à la fin | 1 000 simulations par coup, chacune une fin de partie |
| Jeu contre soi-même avec réseau (type AlphaZero) | le réseau domine, le moteur ne doit pas freiner | 100 000 parties × ~1 000 coups × 100 simulations |

Exemple : 2 millions de parties à 1 000 parties par seconde et par cœur, sur 8 cœurs, prennent
environ **4 minutes**. À 10 parties par seconde (moteur écrit dans un langage de script de
moteur de jeu), **plus de 3 jours**. La vitesse du moteur décide de ce qui est faisable
gratuitement sur un PC.

## 2. Les exigences

### E1. Parties sans écran
- Les règles ne dépendent d'aucune bibliothèque d'affichage, de son, de saisie ni du système
  Android. Elles se compilent et se testent seules, sur un PC Linux, en ligne de commande.
- L'affichage lit l'état et envoie des actions ; il ne modifie jamais l'état lui-même.
- Aucun état global : plusieurs parties tournent en même temps, une par fil d'exécution.

### E2. Vitesse visée (un cœur de PC ordinaire)

Hypothèse de taille de partie, à confirmer avec la session 2 : carte de 11×11 à 16×16 cases,
2 à 4 joueurs, 30 à 50 tours, environ 1 000 actions par partie.

| Opération | Minimum acceptable | Cible |
|---|---|---|
| Partie complète jouée au hasard | 200 parties/s | **1 000 parties/s ou plus** |
| Appliquer une action | 10 µs | 1 µs |
| Lister les actions permises | 20 µs | 5 µs |
| Copier un état | 10 µs | 2 µs (état de quelques kilooctets, d'un seul bloc) |
| Coder l'état pour le réseau | 50 µs | 10 µs |

Une limite de tours fixe garantit que toute partie se termine.

### E3. Reproductibilité : même graine, même partie
- Le générateur de hasard est **dans l'état** (par exemple PCG32 ou xoshiro, écrit dans le
  projet), initialisé par la graine. Jamais le hasard du système, jamais l'heure.
- **Pas de nombres à virgule dans les règles** : combats, dégâts et scores en entiers (ou en
  virgule fixe). Les virgules donnent des résultats différents selon le processeur et le
  compilateur.
- Pas de dépendance à l'ordre de parcours d'une table de hachage ni à des adresses mémoire.
- Une **empreinte** de l'état (somme de contrôle de 64 bits) pour comparer deux exécutions.
- Test automatique : la même graine et les mêmes actions donnent la même empreinte, sur PC et
  sur téléphone.

### E4. Liste des actions permises
- Une action à la fois ; « finir son tour » est une action comme les autres. Pas d'action
  composée (déplacer *puis* attaquer = deux actions).
- Chaque action a un **numéro fixe** dans un espace de taille connue d'avance, par exemple
  `type × case de départ × case d'arrivée` (ou `type × case × direction`), pour la carte la plus
  grande permise. Le moteur fournit le **masque** des actions permises dans cet espace.
- Les choix hors carte (recherche d'une technologie, construction dans une ville, production
  d'unité) ont aussi leur numéro.
- Il faut pouvoir passer d'un numéro à l'action lisible et inversement (pour les replays, le
  débogage et l'affichage des coups de l'IA).

### E5. Copie d'un état
- Copie complète, rapide, indépendante de l'original (E2). C'est le cœur de la recherche
  arborescente.
- Utile mais facultatif : annuler la dernière action.

### E6. Information cachée (brouillard de guerre)
- Si le jeu a un brouillard, le moteur fournit la **vue d'un joueur** : ce qu'il sait, rien de
  plus. L'IA du jeu ne lit que cette vue, sinon elle triche.
- Pour la recherche arborescente, il faut pouvoir **tirer au sort un état complet compatible**
  avec ce qu'un joueur voit (déterminisation). À prévoir dès la conception de l'état.

### E7. Codage de l'état pour un réseau de neurones
- L'état d'un joueur se code en **plans** de taille fixe : `canaux × hauteur × largeur` pour la
  carte (terrain, unités par type et par propriétaire, villes, brouillard…), plus un petit
  **vecteur** (tour, ressources, technologies connues…).
- Taille fixe = la carte la plus grande ; une carte plus petite est complétée par des cases
  « hors carte ».
- Toujours **du point de vue du joueur qui joue** (lui = joueur 0), pour qu'un seul modèle
  joue toutes les places.
- Le codage est versionné : changer le codage oblige à réentraîner.

### E8. Fin de partie et récompenses
- `est_finie`, le gagnant, et un **score par joueur** à tout moment (taille de l'empire,
  villes, unités, points), qui sert de récompense intermédiaire et de départage à la limite de
  tours.

### E9. Format de replay
Un replay, c'est la recette pour rejouer la partie, pas un film :

```
en-tête : version des règles, version du codage, graine, réglages de la carte,
          joueurs (humain ou IA, avec sa version), date au jour près
corps   : la suite des numéros d'actions, avec le joueur et le tour
contrôle: une empreinte d'état tous les N tours, et l'empreinte finale
```

- Rejouer un replay = refaire la partie depuis la graine en appliquant les actions ; les
  empreintes vérifient qu'on retombe juste.
- Compact (quelques kilooctets par partie), pour être envoyé depuis les téléphones (détails pour
  la session 4 dans `besoins-donnees.md`, à venir).
- Lisible par le moteur ET par le programme d'entraînement.

### E10. Versions des règles
- Un numéro de version des règles, inscrit dans chaque replay et chaque modèle d'IA. Un
  changement de règles rend les anciens replays rejouables seulement avec l'ancienne version :
  le moteur garde la version qu'il sait jouer, et l'entraînement trie par version.

### E11. Une seule écriture des règles
- Les règles existent en **un seul exemplaire de code**, utilisé à la fois dans le jeu et pour
  l'entraînement. Deux exemplaires (par exemple un pour le jeu, un recopié en Python) finissent
  toujours par diverger, et l'IA apprend alors un autre jeu que celui du téléphone.

## 3. L'interface minimale

Inspirée de l'interface de jeux d'OpenSpiel (voir `sources.md`), dont les outils d'entraînement
pourront peut-être être réutilisés :

```
nouvelle_partie(réglages, graine)      -> état
joueur_courant(état)                   -> numéro
actions_permises(état)                 -> masque ou liste de numéros
appliquer(état, action)                -> (état modifié)
copier(état)                           -> état
est_finie(état), scores(état)
vue(état, joueur)                      -> vue d'un joueur
coder(vue)                             -> plans + vecteur (nombres)
action_vers_texte(numéro), texte_vers_action(texte)
empreinte(état)
sauver(état) / charger(octets)
```

## 4. Ce que cela implique pour la technologie

L'entraînement se fera en **Python** (PyTorch ou JAX : c'est là que sont les outils, les
exemples et les machines louées). Sur le téléphone, le modèle entraîné tourne avec un moteur
d'inférence mobile (ONNX Runtime ou LiteRT, successeur de TensorFlow Lite), qui existe pour Kotlin, C++ et
la plupart des moteurs de jeu. La question est donc : **en quoi écrire les règles pour qu'elles
tournent vite, à la fois sur Android et sous Python ?**

| Règles écrites en… | Vitesse | Sur Android | Depuis Python | Avis pour l'IA |
|---|---|---|---|---|
| **Rust** | très rapide | oui (bibliothèque native, `cargo-ndk`, liaison par `uniffi` ou JNI) | oui, directement (`PyO3` + `maturin`) | **le meilleur choix** : un seul code, rapide partout, déterministe |
| C++ | très rapide | oui (NDK) | oui (`pybind11`) | aussi bon pour l'IA, plus de risques de plantage |
| Kotlin (JVM) | rapide | natif | indirect : le jeu contre soi-même tourne sur la JVM et écrit des replays que Python lit, ou passerelle réseau | **acceptable** si l'écran est en Kotlin ; plus lourd à brancher |
| C# (.NET, Unity ou Godot) | rapide | oui | indirect (pythonnet, ou replays) | acceptable, même remarque que Kotlin |
| TypeScript / JavaScript | moyen | via une vue web | indirect | possible, déconseillé pour l'entraînement |
| GDScript, Lua, langage de script d'un moteur | lent (10 à 100 fois) | oui | non | **à éviter pour les règles** ; bien pour l'écran |
| Python | lent | non (pas raisonnablement) | natif | à éviter : ne tourne pas sur le téléphone, contredit E11 |

Recommandations pour la session 1, quel que soit le choix de l'écran :

1. **Les règles dans un module à part**, sans dépendance à l'écran, compilable et testable sur
   PC (E1, E11).
2. De préférence **un cœur de règles en Rust** sous l'écran choisi (Kotlin, Godot, Unity…) :
   c'est le seul choix qui coche vitesse, Android et Python d'un seul code. Si la session 1
   préfère un seul langage, Kotlin ou C# restent acceptables, à condition d'entraîner en
   produisant des replays sur la JVM ou .NET.
3. Pas de virgule dans les règles, hasard dans l'état, limite de tours (E3, E2).
4. Une mesure de vitesse « parties aléatoires par seconde » dès le premier prototype, pour
   vérifier E2.

## 5. Ce qui reste à trancher avec les autres sessions

- Taille maximale de la carte, nombre de joueurs et longueur d'une partie (session 2) : elles
  fixent la taille de l'espace d'actions et des plans.
- Brouillard de guerre ou non (session 2) : il change beaucoup l'IA (E6).
- Hasard dans les combats ou combats déterministes (session 2) : des combats sans hasard
  simplifient la recherche arborescente.
