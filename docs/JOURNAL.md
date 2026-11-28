# Journal de bord

> Journal général, tenu par la session maître. Entrée la plus récente **en haut**. On ne modifie
> ni ne supprime une entrée passée : une erreur se corrige par une nouvelle entrée.
> Chaque session de travail tient aussi le sien dans `docs/<dossier>/JOURNAL.md`.

Gabarit :

```markdown
## AAAA-MM-JJ · <titre écrit pour l'utilisateur>

- **Auteur** : Claude, session <maître ou Sx> (ou Antigravity)
- **Branche et commit** :
- **Objectif** :
- **Réalisé** :
- **Décisions** :
- **Vérifications** : <commandes lancées et résultats>
- **Non vérifié sur téléphone** : <points précis, reportés dans le plan>
- **Problèmes et dette** :
- **Prochaine étape** :
```

---

## 2026-11-27 · Première version jouable de bout en bout (essai 0.1.17)

- **Auteur** : Claude, session maître
- **Branche et commit** : `s1-environnement` (c60aa38), pré-version `essai-0.1.17`
- **Objectif** : une version jouable complète pour essayer les mécaniques (D-34).
- **Réalisé** : S1 a codé les règles de la première version (villes, villages, vivres, croissance avec cadeau, exploitations, bâtiments, merveilles, 9 unités, combat sans chance annoncé, brouillard par joueur, arbre en trois branches, 4 clans, ruines, victoire par la capitale ou le seuil, élimination, nulle au tour 80) et l'écran « Nouvelle partie » (2 à 4 joueurs, choix du clan) ; l'ordinateur joue avec l'IA `utilite-r3-v3` de S3. S2 et S3 ont réglé la durée des parties (D-35).
- **Décisions** : D-34, D-35.
- **Vérifications** : lien de l'APK vérifié (7,4 Mo) ; capture de l'émulateur relue : seuil de 58 points à deux joueurs, réglage appliqué ; test à l'émulateur vert (installation, « Commencer », fin de tour, tour de l'ordinateur, plantage provoqué, relance). Les exécutions 14 à 16 étaient rouges (bouton « Commencer » hors de l'écran de l'émulateur), corrigé ; pas d'essais 0.1.12 à 0.1.16.
- **Non vérifié sur téléphone** : une partie entière, les gestes, la lisibilité des panneaux, l'installation par-dessus l'essai précédent (R8).
- **Problèmes et dette** : graphismes provisoires ; ni « annuler un coup » ni « recommencer son tour » ; les captures haute définition du dépôt montrent encore l'ancien seuil de 45 (images de test, pas l'application) ; 10 à 12 % de nulles et Vents-Errants un peu faibles (D-35).
- **Prochaine étape** : l'utilisateur joue et donne ses retours ; ensuite les questions de style en attente (S6) et la nuit (D-32).

## 2026-11-27 · Page de tri : boutons qui bloquaient, corrigé

- **Auteur** : Claude, session maître
- **Branche et commit** : hors dépôt (page de tri privée, version 3)
- **Objectif** : l'utilisateur signale que les boutons de la page de tri ne répondent pas toujours et que la page bloque pendant ses choix.
- **Réalisé** : défaut reproduit sur un banc d'essai (téléphone émulé, processeur ralenti, base simulée qui renvoie parfois un état en retard) : chaque toucher reconstruisait toute la liste deux à trois fois, et un état en retard venu de la base pouvait effacer les derniers choix, puis enregistrer cet effacement (2 à 3 choix perdus sur 6). Correction : un toucher ne redessine que sa carte ; la base ne remplace l'écran que par un état plus récent ; rien n'est écrit avant d'avoir relu les choix déjà enregistrés ; nouvel essai automatique en cas d'échec ; état de l'enregistrement affiché en haut ; brouillon gardé sur l'appareil.
- **Décisions** : aucune.
- **Vérifications** : même banc, avant et après : 6 choix sur 6 affichés et enregistrés, choix d'une visite précédente conservés, 0 ou 1 reconstruction de la liste au lieu de 21 ou 22, réponse au toucher en 80 à 115 ms au lieu de 130 à 330 ms. Non vérifié : la base réelle sur le téléphone de l'utilisateur.
- **Non vérifié sur téléphone** : la page corrigée.
- **Problèmes et dette** : les validations déjà faites (game design avec un seul changement, IA sans changement) ont pu perdre des choix à cause du défaut ; l'utilisateur est prévenu.
- **Prochaine étape** : l'utilisateur reprend le tri.

## 2026-11-27 · Point de contrôle : plateau choisi, version d'essai demandée

- **Auteur** : Claude, session maître
- **Branche et commit** : `main`
- **Objectif** : faire le point sur les six sessions et transmettre les réponses de l'utilisateur.
- **Réalisé** : S1 a livré l'étape B sauf l'APK (moteur Rust et pont JNI, application Kotlin + Compose vide, rapport de plantage repris de la Library, CI verte à l'exécution 3, vérifiée par l'outil GitHub) ; l'APK attend la clé de signature de l'utilisateur. L'utilisateur a choisi le plateau en perspective sans adresse visible (D-16) et une version d'essai installable tout de suite (D-17) ; consignes transmises à S1, information transmise à S6. Sessions 2 à 6 en attente du tri ; aucun choix encore enregistré sur la page de tri.
- **Décisions** : D-16, D-17, D-18 ; section « Commandes » de `CLAUDE.md` remplie d'après S1.
- **Vérifications** : exécutions de la CI lues (1 et 3 vertes, 2 en échec réparée) ; collection `tri` de la page lue, vide.
- **Non vérifié sur téléphone** : tout ; la version d'essai n'est pas encore livrée.
- **Problèmes et dette** : retours de S1 pour la Library (rapport de plantage repris sans retouche ; blocage de `captureToImage()` sous Robolectric 4.14 ; NDK à installer par `sdkmanager` sur la machine GitHub) à reporter dans les fiches.
- **Prochaine étape** : relayer le lien de la version d'essai ; lire les choix du tri.

## 2026-11-27 · Tous les catalogues livrés, page de tri publiée

- **Auteur** : Claude, session maître
- **Branche et commit** : `main`
- **Objectif** : réunir les cinq catalogues de recherche sur une seule page de tri (D-13).
- **Réalisé** : S2 (54 idées, 8 questions), S3 (19 idées, 1 question), S4 (21 idées, 4 questions), S5 (80 idées, 5 questions) et S6 (12 propositions en images, 4 questions) ont livré un `catalogue.json` chacun. Page de tri privée publiée pour l'utilisateur : recommandations pré-cochées, choix enregistrés dans la base de la page, une collection `tri` avec un document par session. S1 a retenu les technologies (D-14) et prépare la première application installable.
- **Décisions** : D-14 (technologies), D-15 (usage limité de GitHub Actions).
- **Vérifications** : les cinq JSON passent le contrôle de fusion (champs, recommandations, identifiants, images présentes) ; page rendue à 390 px de large sans erreur ni défilement de côté ; collection `tri` lue une fois, vide comme attendu. Non vérifié : l'enregistrement d'un choix depuis le téléphone de l'utilisateur.
- **Non vérifié sur téléphone** : rien à installer encore.
- **Problèmes et dette** : les liens vers les fiches s'appuient sur des ancres GitHub que S2 n'a pas vérifiées dans un navigateur ; au pire, le lien ouvre le haut du fichier.
- **Prochaine étape** : lire les choix de l'utilisateur, les reporter dans les décisions et le plan, et donner la suite à chaque session.

## 2026-11-27 · Premiers livrables, choix de la technologie et de la façon de trier

- **Auteur** : Claude, session maître
- **Branche et commit** : `main`
- **Objectif** : relayer les premiers livrables et les décisions de l'utilisateur.
- **Réalisé** : S3 a livré les besoins du moteur pour l'entraînement (vitesse, reproductibilité, actions numérotées), transmis à S1. S1 a livré un comparatif mesuré par essais jetables : 15 200 parties par seconde en Rust, 12 800 en Kotlin, 7 400 en JavaScript, 590 en Python ; quand Python joue coup par coup, Rust est environ 5 fois plus rapide que Kotlin, à égalité quand le moteur joue par lots. S2 (54 idées `GD-`), S3 (19 idées `IA-`) et S4 (21 idées `DO-`) ont livré leurs catalogues.
- **Décisions** : D-12 (technologie : un mélange par points forts, entraînement sur ordinateur idéalement en Rust, IA déjà entraînée sur le téléphone), D-13 (tri sur une page à cocher).
- **Vérifications** : commits des branches relus (présence des fichiers, identité `noreply`). Sources de S2 et S4 consultées en ligne ; celles de S3, d'abord citées de mémoire, revérifiées à ma demande : 24 sources, 19 tiennent, 3 corrigées, 1 non vérifiable, 1 affirmation retirée.
- **Non vérifié sur téléphone** : rien à installer encore.
- **Problèmes et dette** : les vitesses de S1 viennent d'essais jetables non versionnés ; à refaire sur le vrai moteur.
- **Prochaine étape** : S1 revoit le mélange de technologies puis enchaîne sur l'étape B ; page de tri à publier ; livrables de S5 et S6.

## 2026-11-27 · Création du projet et lancement des six sessions

- **Auteur** : Claude, session maître
- **Branche et commit** : `main`, commit de départ
- **Objectif** : poser le dépôt, sa mémoire et l'organisation, puis lancer les six sessions de travail.
- **Réalisé** : dépôt public créé par l'utilisateur ; mémoire posée (`CLAUDE.md`, plan, décisions, organisation, une mission par session) ; six sessions de travail lancées, chacune sur sa branche (`docs/ORGANISATION.md`).
- **Décisions** : D-01 à D-11.
- **Vérifications** : contrôle `depot-public` avant le push. Le commit initial créé par GitHub portait une identité personnelle : historique remplacé par ce commit à identité `noreply`, avec l'accord de l'utilisateur ; il doit activer la protection de son adresse dans les réglages GitHub.
- **Non vérifié sur téléphone** : rien à installer encore.
- **Problèmes et dette** : aucun.
- **Prochaine étape** : comparatif des technologies (S1) à présenter à l'utilisateur ; premiers rapports des sessions 2 à 6.
