# Comparatif des technologies

> Session 1, étape A (D-04). Écrit le 2026-11-27 par Claude, session S1. Skill `revue-langages`.
> Rien n'est codé : ce document prépare le choix de l'utilisateur.

## En bref

| | Option | Verdict |
|---|---|---|
| **1** | **Kotlin + Jetpack Compose, règles dans un module Kotlin pur** | **Recommandée** |
| 2 | Règles en Rust + interface Kotlin/Compose | Bonne, mais trop tôt : à garder comme évolution possible de l'option 1 |
| 3 | Godot 4 (GDScript ou C#), règles éventuellement en Rust (GDExtension) | Sérieuse si l'on veut un vrai moteur de jeu ; plus lourde pour ce projet |
| 4 | libGDX (Kotlin) | Repli si Compose montrait ses limites pour le plateau |
| 5 | Web + Capacitor (TypeScript) | Intéressante pour essayer dans le navigateur ; écartée pour l'APK |
| — | Flutter + Flame, Unity, Bevy, Defold | Écartées (voir § 4) |

La recommandation tient en trois raisons :

1. **Les règles en Kotlin pur tiennent les besoins de la session 3** (`docs/ia/besoins-moteur.md`, branche `s3-ia`) : appelées depuis Python action par action, elles coûtent 3,8 µs par action, soit environ 260 parties de 1 000 actions par seconde (minimum demandé : 200) ; appelées par lots, elles vont aussi vite que Rust (§ 2 bis). Rust fait mieux action par action (0,7 µs, environ 1 300 parties par seconde, au-dessus de la cible de 1 000) : c'est le seul vrai écart.
2. **Tout ce que la Library sait faire est en Kotlin/Compose** : mise à jour intégrée (`maj-auto-github`), rapport de plantage (`rapport-plantage`), compilation en session cloud, captures d'écran sans émulateur (`captures-compose-robolectric`). Avec Godot ou le web, il faudrait tout refaire.
3. **Les règles vont changer souvent au début** (game design à trier, D-07) : un second langage et une frontière à traverser coûteraient à chaque changement. Le module de règles restera étanche (aucune dépendance Android, interface de la session 3) : si l'entraînement mesure un frein réel, le portage en Rust est un pas isolé, vérifié par les mêmes tests et les mêmes empreintes (EN-002).

**Avis contraire, à présenter à l'utilisateur** : la session 3 recommande un cœur de règles en Rust dès le départ, seul choix qui coche vitesse, Android et Python d'un seul code. L'essai lui donne raison sur la vitesse action par action ; il montre aussi que Kotlin reste au-dessus du minimum et que les règles restent écrites une seule fois (Python appelle le même code Kotlin), ce qui respecte E11.

## 1. Ce que le projet demande, domaine par domaine

| Domaine | Besoin | Charge |
|---|---|---|
| Règles du jeu | Parties entières sans écran, vite, reproductibles (graine + liste d'actions), liste des actions permises, copie d'un état, replay | Calcul : c'est le seul domaine où la vitesse du langage compte |
| Affichage et animations | Plateau de cases adressées (A1, B2…), petites images, déplacement de la vue, zoom, quelques animations courtes | Faible : jeu au tour par tour, on redessine peu |
| IA sur le téléphone | Réfléchir un tour en moins d'une seconde ; petit modèle ou recherche arborescente | Calcul modéré, appelle le moteur de règles des milliers de fois |
| Entraînement de l'IA hors du téléphone | Des millions de parties ; jeu contre soi-même, évolution ; sans doute PyTorch | Calcul lourd : vitesse du moteur et passage vers Python |
| Réseau et données (plus tard) | Envoyer des parties, recevoir des modèles (session 4) | Attente réseau : le langage ne compte pas |
| Chaîne de compilation | APK produit par GitHub Actions depuis un dépôt public, version qui monte seule, clé de signature hors du dépôt | Outils et temps de compilation |
| Vérification des écrans sans émulateur | Images PNG d'un écran produites en session ou en CI | Outils |
| Briques de la Library | `maj-auto-github`, `rapport-plantage`, `android-en-session-cloud`, `captures-compose-robolectric` | Toutes écrites pour Kotlin/Android natif |

## 2. Mesure : la vitesse d'un moteur de règles

Un essai jetable (non versionné) joue la même petite partie dans plusieurs langages : plateau 12 × 12 tiré au hasard (plaine coût 1, forêt coût 2, montagne et eau bloquantes), deux joueurs de six unités, trois points de mouvement, 30 tours ; à chaque unité, calcul des cases atteignables (la « liste des actions permises »), choix au hasard par un générateur à graine, puis copie de l'état à chaque tour. Conteneur Linux de la session, un seul cœur.

| Langage | Parties par seconde | Même résultat pour la graine 42 |
|---|---|---|
| Rust (`rustc -O`) | 15 200 | oui (891) |
| **Kotlin sur la JVM** | **12 800** | oui (891) |
| JavaScript (Node 22) | 7 400 (ralenti par un générateur en `BigInt` ; sans doute plus avec des entiers 32 bits) | oui (891) |
| Python 3 pur | 590 | oui (891) |
| GDScript (Godot 4.3) | non mesuré : l'exécution du binaire Godot téléchargé a été refusée dans cette session | — |

Ce qu'on en tire :

- **Kotlin et Rust sont du même ordre** (écart de 20 %) pour ce genre de règles : petits tableaux, entiers, peu d'objets.
- **Python pur est 20 à 25 fois plus lent** : l'entraînement ne doit pas réécrire les règles en Python. Il doit appeler le moteur du jeu (même code que le téléphone), par lots.
- **La reproductibilité tient d'un langage à l'autre** quand on n'utilise que des entiers et un générateur à graine écrit par nous : les quatre versions donnent le même résultat. Règle à garder : pas de nombres à virgule dans les règles, pas de `Random` du système, pas d'ordre de parcours dépendant d'une table de hachage.
- GDScript : il est connu pour être nettement plus lent que C# ou C++ dans Godot (langage interprété à typage dynamique) ; chiffre non vérifié ici. Dans Godot, les règles iraient donc en C# ou en Rust.
- Sur téléphone, la JVM Android (ART) est plus lente que la JVM du PC ; pas de mesure sur appareil. Avec des centaines de parties par seconde, une IA qui réfléchit un tour garde une large marge.

## 2 bis. Mesure : appeler les règles depuis Python

Ajoutée après réception des besoins de la session 3. Deuxième essai jetable, non versionné : les mêmes règles, écrites une fois en Rust (liaison PyO3, construite par `maturin`) et une fois en Kotlin (appelé depuis Python par JPype, qui démarre une JVM dans le processus Python). Interface de la session 3 réduite à l'essentiel : `legal()`, `apply(action)`, `copy()`, `over()`, `hash()`. Une partie : 30 tours, 180 actions. Python choisit chaque action au hasard, comme le ferait une recherche arborescente écrite en Python.

| Moteur, appelé depuis Python | Pas à pas | Pas à pas, avec copie de l'état à chaque action | Par lots (la boucle tourne dans le moteur) |
|---|---|---|---|
| Rust + PyO3 | 7 800 parties/s · **0,7 µs par action** | 6 300 parties/s | 13 500 parties/s |
| Kotlin + JPype | 1 460 parties/s · **3,8 µs par action** | 740 parties/s | 14 100 parties/s |

- Les deux moteurs donnent, à chaque pas, la même liste d'actions permises et la même empreinte finale pour la graine 42.
- Rapporté à la partie de 1 000 actions que la session 3 suppose : Rust pas à pas ≈ 1 300 parties/s (cible : 1 000), Kotlin pas à pas ≈ 260 parties/s (minimum : 200), les deux par lots ≈ 2 500 parties/s.
- Le coût du pont Kotlin ↔ Python se paie à chaque appel ; il disparaît quand la boucle tourne dans le moteur (algorithmes génétiques, jeu contre soi-même écrivant des replays). Pour un entraînement à réseau de neurones, le temps d'évaluation du réseau (des dizaines à des centaines de µs) écrase les 3,8 µs du pont.
- Les chiffres valent pour un cœur de PC (conteneur de la session) et des règles très simples : de vraies règles (combat, villes) coûteront plus, dans les deux langages.

### Comment Python appellera les règles Kotlin (option 1)

Trois chemins, tous sur le même code `:rules` (E11) :

1. **Pas à pas, par JPype** (essayé ici) : Python démarre une JVM dans son propre processus et appelle `legal()`, `apply()`, `copy()` directement ; 3,8 µs par appel. Pour une recherche arborescente ou un entraînement à réseau pilotés depuis Python. En parallèle : un processus Python par cœur, une JVM chacun (non essayé).
2. **Par lots, par JPype** : `:rules` offre des fonctions qui font tourner la boucle dans la JVM (jouer N parties au hasard, évaluer une population de poids, jouer une partie entière avec une IA à poids) et rendent des tableaux ; même vitesse que Rust (§ 2 bis). Pour les algorithmes génétiques et l'évaluation.
3. **Par fichiers de replay** : un petit programme JVM en ligne de commande joue des parties et écrit des replays (E9) que Python lit ensuite. Sans aucun pont ; pour le pré-entraînement par imitation et les données des téléphones (session 4).

Pour les échanges en masse (masques d'actions, plans d'état), le pont passe des tableaux d'entiers ou d'octets d'un seul bloc, jamais élément par élément. Si le chemin 1 freine un jour l'entraînement de façon mesurée, `:rules` se porte en Rust (EN-002) sans toucher à l'interface.

### Les besoins de la session 3 et l'option 1

| Besoin | Réponse avec un module `:rules` en Kotlin pur |
|---|---|
| E1 sans écran, aucun état global | Module JVM sans Android, une partie par objet, testé en ligne de commande |
| E2 vitesse | Atteinte par lots ; minimum atteint pas à pas depuis Python (§ 2 bis) |
| E3 reproductibilité | Générateur écrit dans le projet et rangé dans l'état, entiers seulement, empreinte de 64 bits, test « même graine, même empreinte » en CI |
| E4 actions numérotées, masque | Espace fixe `type × case × case` calculé pour la plus grande carte, masque en tableau d'octets |
| E5 copie | État en tableaux d'entiers, copie d'un bloc (`copyOf`) |
| E6 vue par joueur | Prévue dans l'interface ; dépend du brouillard (session 2) |
| E7 codage en plans | Tableau plat `canaux × hauteur × largeur`, versionné |
| E8 à E10 fin de partie, replay, version | Dans l'interface dès le premier prototype |
| E11 un seul code | Le même module sert au jeu et, par JPype ou par lots, à Python |

## 3. Les options en détail

### Option 1 : Kotlin + Compose, règles dans un module Kotlin pur (recommandée)

- **Règles** : module Gradle `:rules` en Kotlin/JVM sans Android. Testé par JUnit en quelques secondes, en session et en CI. Interface prévue pour la session 3 : `legalActions(state)`, `apply(state, action)`, `copy()`, état et actions codés en entiers, partie rejouée depuis `graine + liste d'actions`.
- **Affichage** : Compose, plateau dessiné dans un `Canvas` (cases, petites images, adresses), gestes de déplacement et de zoom (`transformable`), animations courtes (`Animatable`). Largement suffisant pour un jeu au tour par tour de 10 × 10 à 20 × 20 cases.
- **IA sur le téléphone** : même module `:rules`, appelé directement, sans frontière. Un petit réseau de neurones peut tourner avec TensorFlow Lite ou ONNX Runtime pour Android, ou en Kotlin pur s'il est petit.
- **Entraînement** : en Python avec PyTorch, le moteur appelé par JPype (même code, 3,8 µs par action pas à pas) ou par lots ; ou directement sur la JVM pour les méthodes sans réseau (§ 2 bis).
- **Chaîne** : Gradle et GitHub Actions, déjà éprouvés sur quatre projets de l'utilisateur ; SDK installable en session (≈ 20 s) ; miroir Maven contre les erreurs 429 (Library).
- **Écrans sans émulateur** : captures Compose par Robolectric en PNG (méthode de la Library, déjà utilisée).
- **Briques reprises telles quelles** : `maj-auto-github`, `rapport-plantage`, `android-en-session-cloud`, `captures-compose-robolectric`.
- **Taille de l'APK** : quelques Mo en version réduite.
- **Limites** : Compose n'est pas un moteur de jeu (pas d'éditeur de cartes, pas de système de particules) ; pour des effets riches plus tard, il faudrait les écrire. Le jeu ne tourne que sur Android (pas de version PC ou web sans travail en plus ; Compose Multiplatform existe si le besoin naît).

### Option 2 : règles en Rust + interface Kotlin/Compose

- **Gain** : le même moteur pour le téléphone et pour Python (PyO3), sans JVM dans le processus Python ; vitesse un peu meilleure seul (+20 %), cinq fois meilleure appelé pas à pas depuis Python (0,7 µs contre 3,8 µs par action, § 2 bis) ; mémoire maîtrisée. C'est l'option que recommande la session 3.
- **Coût** : NDK et `cargo-ndk` dans la CI et dans chaque session ; une bibliothèque native par architecture ; une frontière Kotlin ↔ Rust (JNI ou UniFFI) à traverser pour chaque action jouée par l'humain et pour chaque calcul des cases atteignables ; plantages natifs moins lisibles (la brique `rapport-plantage` ne les couvre qu'à moitié, 🟡) ; tests des deux côtés plus un test de la frontière.
- **Verdict** (grille `revue-langages`) : un moteur Rust se justifie pour un calcul lourd ou un moteur partagé entre plateformes. Ici le calcul reste léger, Kotlin tient le minimum de la session 3 et les règles vont changer chaque semaine au début : **pas maintenant, sauf si l'utilisateur fait passer l'entraînement de l'IA avant tout**. L'option 1 garde la porte ouverte : le module `:rules` étanche pourra être porté en Rust en un pas isolé, si la session 3 mesure que le passage Kotlin → Python freine l'entraînement.

### Option 3 : Godot 4

- **Gain** : un vrai moteur de jeu 2D : cartes en cases (`TileMap`), caméra avec zoom, animations, particules, éditeur visuel ; export Android, Windows, web ; mode sans écran (`--headless`).
- **Règles** : en GDScript, trop lentes pour l'entraînement (§ 2) ; en C# (export Android encore récent dans Godot 4) ou en Rust par GDExtension (`godot-rust`) : on retrouve alors le coût de l'option 2, plus celui de Godot.
- **Chaîne** : export en CI possible (images Docker communautaires, modèles d'export d'environ 1 Go à télécharger), mais aucune expérience dans la Library ; signature et version à refaire.
- **Écrans sans émulateur** : possible en lançant Godot avec un rendu logiciel et en enregistrant une image, à mettre au point.
- **Briques de la Library** : aucune ne se reprend telle quelle (mise à jour intégrée, rapport de plantage à réécrire).
- **APK** : 25 à 35 Mo au départ.
- **Verdict** : à préférer si l'utilisateur veut à terme un jeu très animé ou multiplateforme. Pour un jeu au tour par tour sur téléphone, avec une IA qui s'entraîne hors écran, son atout principal (le rendu) compte moins que la reprise de la Library.

### Option 4 : libGDX (Kotlin)

- Cadre de jeu sur la JVM : rendu par OpenGL, cartes en cases, caméra ; les règles Kotlin de l'option 1 s'y branchent sans changement.
- Moins à jour que Compose, interface (menus, boutons) plus pénible ; captures sans émulateur plus difficiles ; briques de la Library reprises en partie seulement.
- **Repli** : si le plateau en Compose devenait trop lent ou trop limité, on passerait l'affichage en libGDX en gardant `:rules`.

### Option 5 : web + Capacitor (TypeScript)

- Atout réel : le jeu s'ouvrirait aussi dans le navigateur du téléphone par un simple lien, et les captures d'écran se font très bien (Playwright est installé dans les sessions).
- Mais l'APK passe par une coquille Android (Capacitor) qui demande Gradle quand même, les briques de la Library ne s'appliquent pas, et l'entraînement se ferait en Node ou par un pont. Écartée pour l'APK ; idée notée EN-001.

## 4. Écartées

| Technologie | Pourquoi |
|---|---|
| Flutter + Flame (Dart) | Un langage et une chaîne de plus, sans gain pour l'IA ; rien dans la Library |
| Unity (C#) | Lourd, licence et activation à gérer en CI, APK volumineux ; démesuré pour ce jeu |
| Bevy (Rust) | Prise en charge d'Android encore jeune |
| Defold (Lua) | Bon moteur 2D léger, mais Lua lent pour l'entraînement et aucune brique reprise |

## 5. Ce qui n'a pas pu être vérifié

- Vitesse de GDScript (exécution de Godot refusée dans la session) et export Godot en CI (non essayé).
- Toute mesure sur téléphone (pas d'appareil, pas d'émulateur).
- Les besoins de la session 3 sont des cibles, pas des mesures ; la taille réelle des parties dépend de la session 2. La vitesse sera remesurée sur le premier prototype de `:rules` (« parties aléatoires par seconde », demandé par la session 3).
- JPype n'a été essayé que dans un seul processus ; plusieurs processus d'entraînement en parallèle (une JVM chacun) n'ont pas été essayés.

## 6. Idées pour plus tard

| N° | Titre | L'idée | Intérêt | Coût |
|---|---|---|---|---|
| EN-001 | Version navigateur | Une version web du plateau pour essayer une règle par un simple lien, sans installer d'APK | Essais rapides | M |
| EN-002 | Moteur Rust partagé | Porter `:rules` en Rust si l'entraînement en Python le demande, mêmes tests et même graine | Entraînement plus direct | L |

## 7. Si l'option 1 est choisie : premiers pas (étape B)

1. Projet Gradle : modules `:rules` (Kotlin pur) et `:app` (Compose), identifiant d'application fixé une fois pour toutes.
2. CI GitHub Actions : tests de `:rules` puis de `:app`, puis APK `Jeu4X.<version>.apk`, version qui monte seule ; clé de signature dans les secrets GitHub (`maj-auto-github`), jamais dans le dépôt.
3. Rapport de plantage (`rapport-plantage`).
4. Aperçus du plateau en PNG (deux ou trois variantes) envoyés à la session maître avant de coder l'affichage.
