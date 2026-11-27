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

1. **Les règles en Kotlin pur sont assez rapides pour l'IA** : 12 800 parties d'essai par seconde sur un seul cœur, contre 15 200 en Rust (mesure du § 2). L'écart ne justifie pas un second langage.
2. **Tout ce que la Library sait faire est en Kotlin/Compose** : mise à jour intégrée (`maj-auto-github`), rapport de plantage (`rapport-plantage`), compilation en session cloud, captures d'écran sans émulateur (`captures-compose-robolectric`). Avec Godot ou le web, il faudrait tout refaire.
3. **Les règles vont changer souvent au début** (game design à trier, D-07) : un second langage et une frontière à traverser coûteraient à chaque changement. Le module de règles restera étanche (aucune dépendance Android, interface étroite) : s'il fallait un jour un moteur Rust (pour l'entraînement en Python, par exemple), le portage serait un pas isolé, vérifié par les mêmes tests et la même graine.

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

## 3. Les options en détail

### Option 1 : Kotlin + Compose, règles dans un module Kotlin pur (recommandée)

- **Règles** : module Gradle `:rules` en Kotlin/JVM sans Android. Testé par JUnit en quelques secondes, en session et en CI. Interface prévue pour la session 3 : `legalActions(state)`, `apply(state, action)`, `copy()`, état et actions codés en entiers, partie rejouée depuis `graine + liste d'actions`.
- **Affichage** : Compose, plateau dessiné dans un `Canvas` (cases, petites images, adresses), gestes de déplacement et de zoom (`transformable`), animations courtes (`Animatable`). Largement suffisant pour un jeu au tour par tour de 10 × 10 à 20 × 20 cases.
- **IA sur le téléphone** : même module `:rules`, appelé directement, sans frontière. Un petit réseau de neurones peut tourner avec TensorFlow Lite ou ONNX Runtime pour Android, ou en Kotlin pur s'il est petit.
- **Entraînement** : directement sur la JVM (12 800 parties par seconde et par cœur dans l'essai, évolution ou jeu contre soi-même sans Python), ou depuis Python en lançant le moteur comme processus qui joue des lots de parties (un appel pour mille parties, jamais mille appels). Le choix revient à la session 3.
- **Chaîne** : Gradle et GitHub Actions, déjà éprouvés sur quatre projets de l'utilisateur ; SDK installable en session (≈ 20 s) ; miroir Maven contre les erreurs 429 (Library).
- **Écrans sans émulateur** : captures Compose par Robolectric en PNG (méthode de la Library, déjà utilisée).
- **Briques reprises telles quelles** : `maj-auto-github`, `rapport-plantage`, `android-en-session-cloud`, `captures-compose-robolectric`.
- **Taille de l'APK** : quelques Mo en version réduite.
- **Limites** : Compose n'est pas un moteur de jeu (pas d'éditeur de cartes, pas de système de particules) ; pour des effets riches plus tard, il faudrait les écrire. Le jeu ne tourne que sur Android (pas de version PC ou web sans travail en plus ; Compose Multiplatform existe si le besoin naît).

### Option 2 : règles en Rust + interface Kotlin/Compose

- **Gain** : le même moteur pour le téléphone et pour Python (PyO3) sans processus intermédiaire ; vitesse un peu meilleure (+20 % dans l'essai) ; mémoire maîtrisée.
- **Coût** : NDK et `cargo-ndk` dans la CI et dans chaque session ; une bibliothèque native par architecture ; une frontière Kotlin ↔ Rust (JNI ou UniFFI) à traverser pour chaque action jouée par l'humain et pour chaque calcul des cases atteignables ; plantages natifs moins lisibles (la brique `rapport-plantage` ne les couvre qu'à moitié, 🟡) ; tests des deux côtés plus un test de la frontière.
- **Verdict** (grille `revue-langages`) : un moteur Rust se justifie pour un calcul lourd ou un moteur partagé entre plateformes. Ici le calcul reste léger et les règles vont changer chaque semaine au début : **pas maintenant**. L'option 1 garde la porte ouverte : le module `:rules` étanche pourra être porté en Rust en un pas isolé, si la session 3 mesure que le passage Kotlin → Python freine l'entraînement.

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
- Les besoins précis de la session 3 (`docs/ia/besoins-moteur.md`) ne sont pas encore arrivés : la recommandation sera revue à leur lecture, surtout s'ils exigent PyTorch avec des appels pas à pas.

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
