# Journal de la session 1 : environnement de jeu

> Entrée la plus récente en haut. On ne modifie ni ne supprime une entrée passée.

## 2026-11-28 · Couleurs du guide de S6 et cadrage (consigne de la maître, essai 0.1.20)

Claude, session S1.

- Table sous le plateau : kraft #E2C9A0 de jour, #0C1018 de nuit (guide S6 § 2).
- Barre du haut et panneau du bas : blancs avec texte #1C1E24 de jour ; #1C212A avec texte #EEF1F5 de nuit. Boutons du panneau en jeu sombre Material de nuit. Encre de « Fin du tour » : brun #2B2118 sur Murailliers et Vents-Errants, blanc ailleurs.
- Cadrage (D-20, D-21) : on part du centre des unités, puis la vue glisse d'au plus un sixième de l'écran pour que le plateau couvre mieux la vue. Les unités restent dans le tiers central. Tests de `CadrageTest` réécrits en conséquence.
- Disposition « Pouce » (D-27) non touchée : elle attend les retours de jeu de l'utilisateur.
- Tests Kotlin verts ; captures de jour et de nuit regardées.

## 2026-11-28 · Essai 0.1.19 publié (sprites 3D, merveilles, anneau)

Claude, session S1.

- CI n° 19 verte (commit 700c507) ; pré-version `essai-0.1.19`.
- Passage à l'émulateur API 34 tout vert : installation, partie, fin du tour, mode sombre (sprites de Nuit), rapport de plantage.
- Captures de l'émulateur regardées, de jour et de nuit : plateau 3D, anneaux sous les unités, brouillard en nuages.
- Poids de l'APK : 9 670 059 octets (9,7 Mo).
- Non vérifié : merveilles à l'écran (aucune construite au tour 1-2), toucher sur un vrai téléphone.

## 2026-11-28 · Merveilles par clan et anneau 3D (livraison S6 a0d53a4)

Claude, session S1.

- Sprites 192 px jour et nuit repris de `s6-direction-artistique` (a0d53a4) : 80 par jeu, 2,4 Mo d'assets.
- Chaque merveille (arène, verger, tour) a son sprite propre, aux couleurs du clan qui tient la ville.
- L'anneau 3D `anneau_<clan>` se dessine juste avant l'unité ; l'ovale 2D est retiré.
- Tests Kotlin verts ; captures de jour et de nuit regardées (anneaux visibles sous les unités). Aucune merveille n'apparaît encore dans les captures.

## 2026-11-28 · Sprites 3D low poly de S6 intégrés (D-37)

*Claude, session S1.*
- Sprites de S6 (`s6-direction-artistique`, 808edf9 et 846c60a), variante retenue « dalles épaisses, facettes, trapus » seulement, en 192 px, Jour et Nuit : `app/src/main/assets/sprites/{jour,nuit}/`, 2,0 Mo. Le 192 px couvre le zoom de départ (case d'environ 190 px sur un écran de 1080 px). 96 et 288 px ne sont pas embarqués.
- `Sprites.kt` charge à la demande. Chaque sprite est posé par son ancre (96 ; 163,2) sur `Perspective.centre()`, à la largeur d'une case. Ordre par case, de l'arrière vers l'avant : terrain ou nuage, exploitation, remparts du fond, lieu, surcouches, voile « vue autrefois », puis l'unité.
- Repères en 2D par-dessus : voile de territoire, pastilles, socle de couleur, barre de vie, anneaux de cible, niveau de ville.
- Le toucher vise d'abord la silhouette de l'unité, celles de devant en premier, puis la case.
- Mode sombre du téléphone : sprites de Nuit et fond plus sombre (D-33).
- Cas tranché ici : les sprites portent la couleur du clan (rouge, jaune, indigo, turquoise). Chaque joueur prend donc la couleur de son clan, et l'écran nomme les joueurs par leur clan au lieu de « Bleu » ou « Rouge ».
- Une seule merveille a un sprite (`merveille_veil`), il sert pour les trois.
- Tests Kotlin verts ; captures jour et nuit regardées. L'émulateur passe aussi en mode sombre et en fait une capture.

## 2026-11-27 · Essai jouable 0.1.17 vert à l'émulateur

*Claude, session S1.* CI n° 17 (b0fa065) verte, pré-version `essai-0.1.17` publiée. Résumé de l'émulateur : installation, « Nouvelle partie » puis « Commencer », « Votre tour », « Fin du tour » (l'ordinateur joue, retour au tour 2), plantage provoqué, rapport et relance, tout vert. Les captures, regardées : le Rouge (le joueur humain) au tour 1, puis au tour 2, avec 7 puis 11 vivres. Contenu : règles V1 complètes, IA `utilite-r3-v3` de S3, réglage de durée de S2. Non vérifié : une partie entière sur un téléphone, les gestes, la lisibilité, l'installation par-dessus.

## 2026-11-27 · CI n° 16 rouge : profil pixel_6 trop lent, retiré

*Claude, session S1.* Avec le profil `pixel_6`, plus rien ne répond à temps à l'émulateur, même l'écran de plantage, qui passait avant. Le profil est retiré. Le bouton « Commencer » fixé en bas reste, et doit suffire sur le petit écran par défaut. Le script affiche désormais dans le journal de la CI les textes visibles à l'écran quand une étape échoue, pour voir au lieu de deviner.

## 2026-11-27 · CI n° 12 à 15 rouges : « Commencer » hors de l'écran de l'émulateur

*Claude, session S1.* Les quatre exécutions depuis la version jouable échouent au même endroit : l'émulateur (écran 320 × 640, mdpi) ne montre pas le bouton « Commencer », placé au bas d'une colonne qui défile, et uiautomator ne voit que l'écran visible. Les tests, la compilation et le plantage provoqué sont verts. Réparation durable : le bouton reste fixé en bas de l'écran « Nouvelle partie » ; l'émulateur prend le profil `pixel_6` (1080 × 2400), plus proche d'un vrai téléphone. Les essais 0.1.12 à 0.1.15 n'ont pas été publiés.

## 2026-11-27 · Réglage de la durée des parties (S2)

*Claude, session S1.* Valeurs de S2 (`reglage-duree.md` sur `s2-game-design`) reportées dans `donnees.rs` : coût de croissance [8, 14, 20, 26], revenu par niveau [2, 2, 3, 3, 4], points tenus [1, 2, 3, 4, 5], seuil 58 / 56 / 56, +2 vivres de départ par rang, +2 de défense dans sa propre capitale. Mesure avec l'adversaire simple (`examples/parties_ia.rs`) : 19 à 21 tours en moyenne (S3 mesure 30 à 34 avec son IA). Tests verts.

## 2026-11-27 · IA de S3 fusionnée et branchée

*Claude, session S1.* `s3-ia` ab33186 fusionnée (avance rapide, elle contenait 0fa9c01). Le pont `tourIa` appelle `ia::adversaire::jouer_tour_numeros(…, Niveau normal)` au lieu de `ia_simple` (gardé pour les tests et les mesures). `cargo test --workspace` vert (ia compris, 30 s), tests Kotlin verts.

## 2026-11-27 · Version jouable de bout en bout (D-34) : règles V1 complètes, adversaire simple

*Claude, session S1.* Consigne de la maître (D-34) : une version d'essai jouable de la nouvelle partie à la fin, sans aperçu à valider, graphismes provisoires, tests légers.

- Moteur (VERSION_REGLES 3, codage 3, replay 3 avec les clans) :
  - `donnees.rs` : tables du § 12 (9 unités, 4 clans, 15 avancées, exploitations, bâtiments, niveaux de ville, trésors).
  - Carte : villages (deux par capitale à distance 2 à 4, plus proches d'elle), ruines à trésor tiré à la création.
  - Villes et villages, rallier/prendre, territoire, revenu, entretien, croissance avec cadeau a/b, exploitations, remparts, marché, trois merveilles (une seule de chaque, détruite à l'élimination).
  - Combat sans hasard (§ 6.3) avec annonce (`annonce_combat`), triangle, siège, Forge, Porte-pavois, riposte, Razzia ; repos et soins.
  - Arbre en trois branches avec choix exclusifs ; prix des Veilleurs et des Écoles ; ruines (réserve, savoir, recrue, carte) ; lunette de l'Astronome.
  - Victoire : capitale prise (élimination ; dernier debout), seuil annoncé à la fin du tour et gagné au début du suivant, nulle après le tour 80.
  - Espace d'actions fixe en blocs (`B_DEPLACER` … `B_RECHERCHER`, 103 k numéros).
  - `ia_simple.rs` : adversaire provisoire (prendre, attaquer si rentable, grandir, produire, exploiter, rechercher, avancer vers le but connu le plus proche). L'IA de S3 (`s3-ia`, 977f5f2) attend l'adaptation à ces règles : pas encore fusionnée.
  - 30 tests Rust verts.
- Cas non tranchés par les règles, décidés ici (à revoir avec S2) :
  - pas de revenu au début du tour 1 (les vivres de départ suffisent) ;
  - défense de ville seulement pour une unité dans une ville de son camp ;
  - le Savoir d'une ruine donne la moins chère des avancées ouvertes ;
  - la ruine et les exploitations se voient sur toute case connue, même vue autrefois ;
  - une ville prise garde ses remparts gratuits (c'est un bâtiment) mais perd les autres cadeaux ;
  - la place du joueur humain est tirée de la graine ; les autres clans vont à l'ordinateur dans l'ordre.
- Mesure (`examples/parties_ia.rs`, 40 parties par nombre de joueurs, adversaires simples) : toutes gagnées au seuil, en 8 à 10 tours en moyenne, loin des 25 à 35 visés par S2. À signaler : l'économie et le seuil sont à régler.
- Application : écran « Nouvelle partie » (2 à 4 joueurs, clan), partie contre l'ordinateur qui joue action par action, barre (tour, vivres et revenu, unités, scores et seuil, annonce), pastilles blanches et rouges, panneau d'annonce du combat (D-31), panneaux de ville et d'exploitation, arbre d'évolution plein écran, écran de fin (victoire, défaite, nulle, n° de carte). Toucher : la silhouette de l'unité d'abord, puis le losange (guide S6). Montagne grise froide, colline ocre (S6).
- Pont JNI : `nouvellePartie(graine, nbJoueurs, clans)`, `etat(h, joueur)`, `permises`, `appliquer(numéro)`, `annonce`, `tourIa`.
- Émulateur (`outils/essai-emulateur.sh`) : « Nouvelle partie » → « Commencer » → « Votre tour » → « Fin du tour » → retour à notre tour au tour suivant, puis plantage provoqué.
- Tests Kotlin verts ; captures en session : réglage, plateau, unité choisie, ville, arbre, combat, fin.
- Non fait : annuler un coup, recommencer son tour (coûteux), code de carte saisissable (seul le n° de carte s'affiche).

## 2026-11-27 · Règles V1, étape a : 8 directions, 2 à 4 joueurs, collines, brouillard

*Claude, session S1.* Consigne de la maître : coder les règles validées (D-30), étape a d'abord.

- Moteur (`moteur/regles`, VERSION_REGLES 2, codage 2, format de replay 2) :
  - `Reglages::pour(n)` : 2 joueurs 11×11, 3 joueurs 12×12, 4 joueurs 14×14 ; distance des capitales 6, 6, 7 ; limite 80 tours. `Default` = 2 joueurs.
  - Terrain `Colline` (coût 2, vision +1 ; montagne vision +1 aussi). Voisines dans 8 directions, distance = plus grand écart.
  - Carte (`carte.rs`) : deux champs lissés (altitude, humidité), classés pour des parts exactes (15 % eau, 8 % montagne, 12 % colline, 20 % forêt). Capitales tirées puis abords retouchés par échanges avec des cases lointaines (parts gardées) ; conditions du § 2.3 vérifiées (voisines, plaines à 2 près, chemin) ; sinon graine dérivée `graine + k·0x9E3779B97F4A7C15` (pas `graine + 1`, qui faisait retomber 42 sur 43). Mesure (`examples/essais_carte.rs`) : 1,0 essai en moyenne, 2 au plus, sur 2 000 graines à 2, 3 et 4 joueurs.
  - Départ : capitale (vision 2), un Lancier (1 point, vision 2) et un Éclaireur (3 points, vision 3) sur deux voisines.
  - Brouillard par joueur (inconnue, vue autrefois, visible) avec souvenir du dernier propriétaire des villes. Déplacements d'après ce que sait le joueur : cases connues seulement, zone de contrôle des adverses visibles, premier pas toujours permis, arrêt sur surprise case par case.
  - `Partie::vue(joueur)` pour S3 ; `coder` lit la vue (14 canaux, retourné pour mettre sa capitale en bas à gauche). Scores et gagnant provisoires (cases découvertes) jusqu'à l'étape e.
  - 32 tests verts. Vitesse : 973 parties/s à 2 joueurs (80 tours), 412 à 4 joueurs.
- Pont JNI : `nouvellePartie(graine, nbJoueurs)`, `etat` rend la vue du joueur courant (terrain -1 si inconnu).
- Application : brouillard du joueur dont c'est le tour, cases vues autrefois assombries, collines, capitale (maison), signe de l'Éclaireur. Graphismes provisoires. L'application reste à 2 joueurs : un choix du nombre de joueurs demanderait un aperçu.
- Captures en session : `plateau`, `plateau-choix`, `plateau-souvenir` (nouvelle), `plateau-tour-rouge`. 33 tests Kotlin verts.

## 2026-11-27 · Essai 0.1.10 vert à l'émulateur, glissement de début de tour (D-21)

*Claude, session S1.*

- CI n° 10 (commit 49bce61) verte ; pré-version `essai-0.1.10` publiée avec l'APK, le résumé et les captures.
- Résumé de l'émulateur (API 34, x86_64) : installation, plateau, passage au Rouge, plantage provoqué, rapport et relance, tout vert.
- Capture `emulateur-tour-rouge` regardée : 2 s après « Fin du tour », les trois unités rouges sont au centre de la vue, zoom ×2 gardé.
- Non vérifié : douceur réelle de l'animation (une capture fixe ne la montre pas), gestes, téléphone ARM, installation par-dessus.

## 2026-11-27 · Début de tour : la vue glisse vers les unités du joueur (D-21)

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : réponse a) de l'utilisateur (D-21).
- **Réalisé** : `Perspective.decalagePour` (centre moyen des unités au milieu de la vue, à l'échelle donnée) ; `EcranPartie` : au changement de tour ou de joueur, animation de 500 ms du décalage seul, zoom gardé. Script d'émulateur : capture après « Fin du tour » (`capture-tour-rouge.png`), jointe à la pré-version. Capture haute définition : `docs/environnement/captures/plateau-tour-rouge.png` (les trois unités rouges au centre).
- **Décisions** : aucune nouvelle.
- **Vérifications** : `:app:testDebugUnitTest` 31 verts (dont le centrage au changement de tour) ; capture regardée.
- **Non vérifié sur téléphone** : douceur de l'animation, confort.
- **Problèmes et dette** : aucun.
- **Prochaine étape** : essai à l'émulateur, rapport.

## 2026-11-27 · Essai 0.1.9 vert à l'émulateur, cadrage zoomé

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`, `a44f0b0`, exécution CI n° 9
- **Objectif** : livrer D-20 en essai.
- **Réalisé** : pré-version `essai-0.1.9` (APK 7,1 Mo, capture du plateau au lancement, capture du rapport, résumé). Le miroir de Google a tenu.
- **Décisions** : aucune.
- **Vérifications** : Android 14 x86_64 : installation, plateau (« Tour 1 sur 30 », « Joueur Bleu »), « Fin du tour » → « Joueur Rouge », plantage provoqué et rapport, relance : tout vert. Capture retéléchargée et regardée : les unités bleues au centre, cases deux fois plus grandes.
- **Non vérifié sur téléphone** : pincer pour dézoomer, confort du zoom, ARM réel.
- **Problèmes et dette** : tour du Rouge, unités hors de l'écran (voir l'entrée du cadrage).
- **Prochaine étape** : avis de l'utilisateur sur le changement de tour.

## 2026-11-27 · CI n° 8 rouge : Maven Central 429, miroir de Google ajouté

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : réparer l'exécution n° 8 (commit `8e56a05`), rouge avant tout test.
- **Réalisé** : cause lue dans le journal de la CI : « Could not GET …kotlin-reflect-2.0.21.pom … status code 429 Too Many Requests » sur `repo.maven.apache.org`, à la configuration de Gradle ; le code n'y est pour rien. Parade durable plutôt qu'une relance : `settings.gradle.kts` interroge d'abord le miroir de Maven Central chez Google (`maven-central.storage-download.googleapis.com`), Maven Central en repli.
- **Décisions** (développeur) : miroir de Google en premier pour les deux listes de dépôts.
- **Vérifications** : `./gradlew --refresh-dependencies help` en session : OK.
- **Non vérifié sur téléphone** : sans objet.
- **Problèmes et dette** : aucun essai 0.1.8 publié (l'exécution a échoué avant la compilation) ; le numéro 0.1.8 est sauté, le prochain essai sera 0.1.9.
- **Prochaine étape** : CI verte, essai 0.1.9 à l'émulateur, rapport.

## 2026-11-27 · Cadrage au lancement zoomé sur ses unités (D-20)

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : réponse de l'utilisateur sur le cadrage : b), zoomé sur ses unités (D-20).
- **Réalisé** : `Perspective.cadrageSur` (échelle = 2 × la vue d'ensemble, centre = moyenne des unités du joueur qui commence) ; appliqué au lancement et à chaque « Nouvelle partie » (`Affichage.numeroPartie`). Changement de tour inchangé, comme demandé. Captures : `docs/environnement/captures/plateau.png` (lancement), `plateau-choix.png`, `plateau-tour-rouge.png`.
- **Décisions** : aucune nouvelle.
- **Vérifications** : `:app:testDebugUnitTest` 30 verts (dont `CadrageTest`) ; captures regardées : au lancement les trois unités bleues sont au centre, cases deux fois plus grandes.
- **Non vérifié sur téléphone** : confort du zoom de départ, pincer pour dézoomer.
- **Problèmes et dette** : au tour du Rouge, la vue reste sur les unités bleues ; les unités rouges sont hors de l'écran (en haut à droite) : il doit faire glisser la vue pour les trouver. Solution proposée, non codée : au début de chaque tour, faire glisser la vue en douceur vers les unités du joueur dont c'est le tour, sans changer le zoom.
- **Prochaine étape** : essai à l'émulateur, rapport.

## 2026-11-27 · Essai 0.1.7 : plateau vert à l'émulateur

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`, `3f15a9e`, exécution CI n° 7 (7 min 46)
- **Objectif** : vérifier le plateau à l'émulateur avant tout envoi à l'utilisateur (D-19).
- **Réalisé** : pré-version `essai-0.1.7` (APK, capture du plateau, capture du rapport, résumé).
- **Décisions** : aucune.
- **Vérifications** : Android 14 (API 34) x86_64 : installation, plateau affiché depuis le moteur (« Tour 1 sur 30 », « Joueur Bleu »), « Fin du tour » touché donne « Joueur Rouge », plantage provoqué et rapport, relance : tout vert. Capture retéléchargée et regardée.
- **Non vérifié sur téléphone** : toucher une case précise, glisser, pincer ; lisibilité (plateau petit au premier cadrage) ; ARM réel.
- **Problèmes et dette** : cadrage de départ trop petit sur un écran de téléphone.
- **Prochaine étape** : avis de l'utilisateur sur l'essai, puis zoom de départ.

## 2026-11-27 · Plateau en perspective jouable à deux sur le même téléphone

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : étape C, affichage du plateau choisi par l'utilisateur (D-16 : variante C, perspective, aucune adresse à l'écran).
- **Réalisé** : pont JNI étendu (partie tenue côté Rust par une poignée ; état lu en un seul bloc ; cases atteignables ; déplacer ; fin du tour). Kotlin : `moteur/Partie.kt` (lecture du bloc), `plateau/Perspective.kt` (projection en losanges, case sous le doigt, ordre de dessin), `plateau/EcranPartie.kt` (barre « Tour n sur 30 · Joueur Bleu/Rouge », plateau dessiné dans un `Canvas`, glisser pour déplacer la vue, pincer pour zoomer, toucher une unité puis une case jaune, bouton « Fin du tour », « Nouvelle partie » à la fin), `plateau/PartieViewModel.kt`. Couleurs et dessins provisoires repris de l'aperçu C. L'écran de contrôle du moteur ne s'affiche plus que si la bibliothèque Rust est en défaut. Captures : `docs/environnement/captures/plateau.png` et `plateau-choix.png`.
- **Décisions** (développeur) : deux joueurs humains sur le même téléphone (pas encore d'IA) ; graine de la carte tirée de l'horloge du téléphone (hors des règles, qui restent reproductibles) ; plateau 11 × 11 par défaut.
- **Vérifications** : `cargo test` 21 verts ; `:app:testDebugUnitTest` 29 verts (dont 4 de géométrie, 4 de l'écran de partie branché sur le vrai moteur, 2 captures) ; captures regardées. Script d'émulateur mis à jour : plateau affiché (« Tour 1 sur 30 », « Joueur Bleu »), toucher « Fin du tour » donne « Joueur Rouge », plantage provoqué, relance.
- **Non vérifié sur téléphone** : gestes (glisser, pincer, toucher une case précise), lisibilité des cases sur un petit écran, fluidité.
- **Problèmes et dette** : au premier cadrage le plateau tient dans la largeur et paraît petit ; à revoir avec l'utilisateur (zoom de départ). Rotation de l'écran : l'état est gardé par le ViewModel, cadrage recalculé.
- **Prochaine étape** : CI verte avec capture du plateau à l'émulateur, rapport à la session maître.

## 2026-11-27 · Essai 0.1.6 vert à l'émulateur

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`, `1449b5d`, exécution CI n° 6 (7 min 20)
- **Objectif** : premier passage de l'essai à l'émulateur (D-19).
- **Réalisé** : pré-version `essai-0.1.6` publiée avec l'APK, deux captures et le résumé.
- **Décisions** : aucune.
- **Vérifications** : résumé de la CI, Android 14 (API 34) x86_64 : installation OK ; ouverture OK, « Moteur de règles 1 : contrôle réussi » à l'écran ; plantage provoqué : écran « Jeu4X s'est arrêté » avec la cause « Plantage de test demandé », commit et appareil dans le rapport ; relance normale OK. Captures retéléchargées depuis la pré-version et regardées.
- **Non vérifié sur téléphone** : processeur ARM réel (bibliothèque `arm64-v8a`), gestes, lisibilité, installation par-dessus une version précédente, boutons Copier et Partager du rapport.
- **Problèmes et dette** : captures de l'émulateur en 320 × 640 (définition par défaut de l'appareil virtuel).
- **Prochaine étape** : affichage du plateau en perspective, sans adresses (D-16), avec le même essai à l'émulateur à chaque version.

## 2026-11-27 · Essai vérifié à l'émulateur dans la CI (D-19)

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : consigne de l'utilisateur, « utilise l'émulateur Android Studio pour ce genre de test » : vérifications de base à l'émulateur avant de lui demander quoi que ce soit.
- **Réalisé** : l'essai embarque aussi la bibliothèque Rust `x86_64` (le vrai jeu reste `arm64-v8a`, D-18) ; plantage provoqué possible dans l'essai seulement (`BuildConfig.PLANTAGE_DE_TEST`, extra `fr.jeu4x.app.PLANTAGE_DE_TEST`) ; script `outils/essai-emulateur.sh` (installation, ouverture, texte « Moteur de règles 1 : contrôle réussi » lu par `uiautomator`, plantage provoqué puis écran du rapport et cause, relance normale, deux captures, résumé) ; CI : KVM, `reactivecircus/android-emulator-runner@v2` (API 34, x86_64, google_apis), puis pré-version `essai-0.1.<n>` avec l'APK, les deux captures et le résumé. Durée bornée à 20 min (D-15). Une pré-version n'est publiée que si l'essai à l'émulateur est vert.
- **Décisions** (développeur) : le logcat de l'appli reste dans le journal de la CI, pas dans la pré-version (dépôt public : pas de journal de test brut publié).
- **Vérifications** : en session : `assembleEssai` contient `lib/arm64-v8a` et `lib/x86_64/libjeu4x_android.so`, l'APK de debug seulement `arm64-v8a` ; tests unitaires verts ; script vérifié par `bash -n`. L'émulateur lui-même ne tourne qu'en CI (pas de KVM en session).
- **Non vérifié sur téléphone** : tout ; l'émulateur ne remplace ni les gestes, ni la lisibilité, ni un vrai processeur ARM.
- **Problèmes et dette** : aucun pour l'instant.
- **Prochaine étape** : CI verte avec captures, rapport à la session maître, puis affichage du plateau.

## 2026-11-27 · Moteur Rust : plateau, terrains, déplacements, tours, interface d'actions

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : avancer sur le moteur seul, testé sans appareil, en attendant le retour du téléphone sur l'essai (consigne de la session maître).
- **Réalisé** : `moteur/regles` : `plateau` (carré de 6 à 16 cases de côté, adresses internes A1…, jamais à l'écran, D-16 ; plaine coût 1, forêt coût 2, montagne et eau infranchissables), `partie` (réglages en paramètre : taille 11 par défaut, 3 unités par joueur, 3 points de mouvement, 30 tours ; deux joueurs au tour par tour ; actions numérotées dans un espace fixe de 1 + 256 × 256 numéros ; masque ; copie par `Clone` ; scores provisoires ; empreinte ; codage en 8 plans 16 × 16 du point de vue du joueur ; actions en texte « B2>C3 » et « FIN »), `replay` (format texte versionné, empreintes de contrôle toutes les 50 actions), `aleatoire` (joueur au hasard), exemple `vitesse`.
- **Décisions** (développeur, provisoires jusqu'au tri de S2) : un déplacement = une action (départ, arrivée) dans la portée restante ; les points non dépensés restent à l'unité jusqu'à la fin du tour ; score provisoire = unités dans la moitié adverse ; pas de brouillard ni de combat.
- **Vérifications** : `cargo test` : 21 tests verts (adresses, coûts, portée, refus sans effet, tours et fin, numéros aller-retour pour 6, 11, 14, 16, masque, codage des deux points de vue, replays rejoués et relus, replay altéré détecté). Vitesse (`cargo run --release --example vitesse`, un cœur du conteneur) : 11 × 11 : 2 622 parties/s de 196 actions, 1,95 µs par action (liste des actions permises comprise) ; 14 × 14 : 2 053 parties/s, 2,49 µs.
- **Non vérifié sur téléphone** : sans objet (moteur seul).
- **Problèmes et dette** : une partie de 1 000 actions reviendrait à environ 500 parties/s : au-dessus du minimum de S3 (200), sous sa cible (1 000) ; la liste des actions alloue à chaque appel, à optimiser si besoin. Pont JNI pas encore étendu à cette interface (attendre l'affichage).
- **Prochaine étape** : retour du téléphone sur l'essai, puis affichage en perspective sans adresses.

## 2026-11-27 · Version d'essai « Jeu4X essai » (D-16, D-17)

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : livrer tout de suite une version d'essai installable, en attendant la clé de l'utilisateur (D-17) ; noter le choix du plateau (D-16).
- **Réalisé** : type de compilation `essai` : `fr.jeu4x.app.essai`, nom « Jeu4X essai » (`app/src/essai/res`), clé d'essai publique versionnée `app/essai/cle-essai.keystore` (mot de passe `essai-public`, expliqué dans `app/essai/LISEZMOI.md`). La CI publie à chaque push de code `Jeu4X-essai.0.1.<n>.apk` en pré-version `essai-0.1.<n>`. Le vrai `fr.jeu4x.app` ne change pas.
- **Décisions** : D-16 (utilisateur) : plateau en perspective (variante C), aucune adresse à l'écran ; les adresses restent dans le moteur. D-17 (utilisateur) : essai maintenant, clé plus tard.
- **Vérifications** : `assembleEssai` en session : `aapt2` lit `fr.jeu4x.app.essai`, « Jeu4X essai », `arm64-v8a` ; `apksigner` : certificat `CN=Jeu4X essai`. Tests `:app:testDebugUnitTest` verts. Contrôle `depot-public` : deux alertes attendues (mot de passe et clé d'essai publics), admises par D-17.
- **Non vérifié sur téléphone** : installation, écran, mise à jour par-dessus.
- **Problèmes et dette** : aucun.
- **Prochaine étape** : lien de l'essai à la session maître, puis moteur Rust (plateau, terrains, déplacements, tours, interface d'actions).

## 2026-11-27 · CI verte, compilation Rust pour Android vérifiée sur GitHub

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`, `aa332ab` puis `4810263`
- **Objectif** : vérifier en CI ce qui ne l'était qu'en session : la compilation du moteur Rust pour le téléphone.
- **Réalisé** : sans clé, la CI compile maintenant un APK de debug (jamais livré) et vérifie qu'il contient `lib/arm64-v8a/libjeu4x_android.so`. Panne révélée : « NDK is not installed » (le NDK 27.2.12479018 n'est pas sur le runner et AGP ne l'installe pas pour une tâche Exec). Parade : étape `sdkmanager --install "ndk;27.2.12479018"` avant Gradle.
- **Décisions** : aucune.
- **Vérifications** : exécution 1 (`19daecf`) verte, 3 min 24 ; exécution 2 (`aa332ab`) rouge sur la panne NDK ; exécution 3 (`4810263`) verte, 4 min, tests Rust + Kotlin + APK de debug avec la bibliothèque native.
- **Non vérifié sur téléphone** : tout.
- **Problèmes et dette** : actions GitHub en Node 20 dépréciées (avertissement), à monter de version plus tard.
- **Prochaine étape** : clé de signature par l'utilisateur, choix de l'aperçu du plateau.

## 2026-11-27 · Vérification des développeurs Android

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : vérifier à la source l'information de la veille (S5) transmise par la session maître.
- **Réalisé** : section ajoutée à `signature.md`, avec les sources. Obligatoire depuis le 30/09/2026 dans quatre pays (boutiques participantes), partout en 2027 ; APK installé directement pas encore concerné ; compte à distribution limitée (20 appareils, sans pièce d'identité) suffisant pour un usage privé ; il faudra déclarer `fr.jeu4x.app` et l'empreinte SHA-256 de la clé release.
- **Décisions** : aucune ; identifiant et clé déjà compatibles. Aucun compte créé.
- **Vérifications** : pages officielles Google (blog, guide, FAQ, aide), consultées le 2026-11-27.
- **Non vérifié sur téléphone** : sans objet.
- **Problèmes et dette** : versions d'Android concernées (7 ou 8 et plus) différentes selon les pages.
- **Prochaine étape** : résultat de la CI, rapport d'étape B.

## 2026-11-27 · Étape B : application vide, pont Rust, CI, aperçus du plateau

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : étape B de la mission (application qui s'installe, CI, version, signature, rapport de plantage, aperçus du plateau).
- **Réalisé** : moteur Rust (`moteur/regles` : version des règles, hasard PCG32 dans l'état, empreinte FNV-1a ; `moteur/android` : pont JNI) ; application Kotlin + Compose `fr.jeu4x.app` (écran provisoire : nom, version, contrôle du moteur) ; rapport de plantage repris de la Library (`rapport-plantage`) ; workflow `.github/workflows/apk.yml` (tests Rust, tests Kotlin, APK release signé en pré-version `apercu-0.1.<n>`) ; `signature.md`, `commandes.md` ; trois aperçus du plateau (`apercus/plateau-a|b|c.png`, maquettes HTML rendues par Playwright, rien dans l'application).
- **Décisions** (développeur) : identifiant d'application `fr.jeu4x.app`, définitif ; version `0.1.<numéro d'exécution de la CI>` ; seule architecture `arm64-v8a` ; minSdk 26 ; release non réduite (R8 plus tard) ; pas de mise à jour intégrée à cette étape (elle demande une interface, donc un aperçu).
- **Vérifications** : `cargo test` 4/4 ; `./gradlew :app:testDebugUnitTest` 20/20 (dont 3 tests du pont Kotlin ↔ Rust sur la bibliothèque compilée pour le PC, 9 tests du rapport de plantage sous Robolectric, 1 capture d'écran) ; `assembleDebug` : APK avec `lib/arm64-v8a/libjeu4x_android.so` ; `assembleRelease` signé avec une clé jetable dont le mot de passe contient `\` et une espace : `apksigner verify` OK, `versionCode 7` / `0.1.7` lus depuis l'environnement ; clé jetable effacée.
- **Non vérifié sur téléphone** : installation, chargement de la bibliothèque native (« contrôle réussi » à l'écran), rapport de plantage, installation par-dessus.
- **Problèmes et dette** : Robolectric a reçu des 429 de Maven Central en session (pot préchargé depuis le miroir de Google) ; `captureToImage()` bloque sous Robolectric (remplacé par un dessin de la vue, comme la Library). Aucun APK tant que les secrets de signature manquent.
- **Prochaine étape** : CI verte, clé de signature par l'utilisateur, choix de l'aperçu du plateau, puis étape C.

## 2026-11-27 · Décision : règles et IA en Rust, affichage en Kotlin + Compose

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : revoir le choix avec le facteur de l'utilisateur (entraînement sur PC en Rust, IA pré-entraînée sur le téléphone), et l'écrire : l'utilisateur m'a autorisé à suivre ma recommandation.
- **Réalisé** : section « Décision retenue » en tête du comparatif (mélange par domaine).
- **Décisions** : règles, IA du téléphone et entraînement en Rust ; affichage Kotlin + Compose ; pont JNI à la main ; CI cargo test → cargo ndk → Gradle. À reporter par la session maître dans `docs/DECISIONS.md`.
- **Vérifications** : SDK 35, NDK 27.2 et `cargo-ndk` 4.1.2 installés dans le scratchpad ; bibliothèque Rust + crate `jni` compilée pour `arm64-v8a` (8 s).
- **Non vérifié sur téléphone** : chargement de la bibliothèque native.
- **Problèmes et dette** : InnovAdapt-CRM (antécédent) non relu, dépôt hors de la session.
- **Prochaine étape** : étape B.

## 2026-11-27 · Comparatif revu avec les besoins de la session 3

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : intégrer `docs/ia/besoins-moteur.md` (branche `s3-ia`, 825a0a4) au comparatif, à la demande de la session maître.
- **Réalisé** : § 2 bis ajouté (mesure des règles appelées depuis Python, tableau besoins E1 à E11) ; avis contraire de la session 3 (Rust) écrit dans « En bref ». Recommandation inchangée : Kotlin + Compose.
- **Décisions** : aucune.
- **Vérifications** : essai jetable dans le scratchpad, même règles en Rust (PyO3, `maturin`) et en Kotlin (JPype), 180 actions par partie : pas à pas 0,7 µs/action (Rust) contre 3,8 µs (Kotlin) ; par lots 13 500 et 14 100 parties/s ; mêmes actions permises à chaque pas et même empreinte pour la graine 42.
- **Non vérifié sur téléphone** : tout.
- **Problèmes et dette** : JPype non essayé en plusieurs processus.
- **Prochaine étape** : attendre le choix de l'utilisateur.

## 2026-11-27 · Comparatif des technologies

- **Auteur** : Claude, session S1
- **Branche et commit** : `s1-environnement`
- **Objectif** : étape A de la mission, comparer les technologies avant le choix de l'utilisateur (D-04).
- **Réalisé** : `docs/environnement/comparatif-technologies.md` (cinq options, quatre écartées, mesure de vitesse). Recommandation : Kotlin + Compose, règles dans un module Kotlin pur.
- **Décisions** : aucune ; le choix revient à l'utilisateur.
- **Vérifications** : essai jetable dans le scratchpad (non versionné), même partie de 30 tours sur 12 × 12 : Rust 15 200 parties/s, Kotlin JVM 12 800, Node 7 400, Python 590 ; même résultat (891) pour la graine 42 dans les quatre langages. Godot 4.3 téléchargé, mais son exécution a été refusée par la session : GDScript non mesuré.
- **Non vérifié sur téléphone** : tout (pas d'appareil).
- **Problèmes et dette** : besoins de la session 3 (`docs/ia/besoins-moteur.md`) pas encore reçus.
- **Prochaine étape** : attendre le choix de l'utilisateur transmis par la session maître, puis étape B.
