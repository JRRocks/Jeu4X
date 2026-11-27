# Mission de la session 1 : environnement de jeu

> Écrite par la session maître le 2026-11-27. Branche : `s1-environnement`. Dossier : `docs/environnement/` et le code. Préfixe des idées : `EN-`.
> Lire d'abord `CLAUDE.md`, `docs/ORGANISATION.md` et `docs/DECISIONS.md`.

## Ce que l'utilisateur a demandé

« L'objectif primaire, c'est de créer l'environnement : des cases avec des adresses, comme sur un jeu d'échecs, le mouvement de tour à tour et des petits graphiques. » Des concepts de design basiques, juste pour avoir un aperçu : le vrai travail de style revient à la session 6.

## Étape A : comparer les technologies (maintenant)

L'utilisateur a choisi que tu compares avant de coder (D-04). Skill `revue-langages`.

- Domaines : règles du jeu ; affichage et animations ; IA sur le téléphone ; entraînement de l'IA hors du téléphone ; réseau et données (plus tard) ; chaîne de compilation qui produit l'APK depuis un dépôt public ; vérification des écrans sans émulateur ; reprise des briques de la Library (`maj-auto-github`, `rapport-plantage`, `docs/methodes/android-en-session-cloud.md`, `docs/methodes/captures-compose-robolectric.md`).
- Candidats au minimum : Kotlin + Compose avec les règles dans un module Kotlin pur ; Godot 4 ; règles en Rust + interface Kotlin. Ajoute ce qui mérite de l'être (libGDX, Flutter + Flame, web + Capacitor, Unity…) en disant pourquoi tu le gardes ou l'écartes.
- Besoin de la session 3 (IA) à prendre en compte : les règles doivent jouer des milliers de parties sans écran, vite et de façon reproductible, avec la liste des actions permises, la copie d'un état et le replay d'une partie (graine + liste d'actions). La session maître te transmettra ses besoins précis dès qu'elle les aura.
- Un petit essai jetable pour vérifier une affirmation est permis (par exemple un export Godot en CI) ; rien de ces essais n'est poussé.
- Livrable : `docs/environnement/comparatif-technologies.md`, puis un rapport à la session maître avec une question numérotée, des choix et ta recommandation en premier. **Attends le choix de l'utilisateur.**

## Étape B : livrer dès le premier jour (après le choix)

Skills `nouveau-projet` (§ 4 et 5) et `livraison` :

- une application vide qui s'installe sur le téléphone, compilée par la CI à chaque push, tests avant la compilation ;
- une version qui monte seule, un fichier `Jeu4X.<version>.apk`, l'installation par-dessus la précédente (même identifiant d'application et même clé de signature, toujours) ;
- signature : le dépôt est public, aucune clé ne s'y versionne ; propose la solution et dis à la session maître ce que l'utilisateur doit faire, s'il doit faire quelque chose ;
- un rapport de plantage lisible sur le téléphone (`rapport-plantage`) ;
- en même temps, des aperçus du plateau en images (deux ou trois variantes : par exemple vue de dessus ou en perspective, adresses toujours visibles ou au toucher). **Attends le choix** avant de coder l'affichage.

## Étape C : l'environnement (après le choix de l'aperçu)

- Plateau carré de cases, adresses comme aux échecs : colonnes en lettres, rangées en chiffres (A1, B2…).
- Quelques terrains (par exemple plaine, forêt, montagne, eau) qui ralentissent ou bloquent le déplacement.
- Des unités pour deux joueurs, au tour par tour : toucher une unité, voir les cases atteignables, la déplacer, passer le tour ; le tour et le joueur en cours affichés.
- Déplacer la vue et zoomer.
- Les règles dans un module sans affichage, testé, reproductible, avec l'interface d'actions demandée par la session 3.
- Petits graphismes simples et provisoires.
- Livraison : un APK au numéro nouveau, son lien envoyé à la session maître avec ce que l'utilisateur doit vérifier sur son téléphone.

## Hors mission

Clans, arbre d'évolution, combat, villes, IA, serveur, style définitif : d'autres sessions s'en occupent. Si le besoin se présente, demande à la session maître.
