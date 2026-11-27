# CLAUDE.md : Jeu4X

Jeu de stratégie 4X original au tour par tour pour téléphone Android, avec une IA qui progresse au fil des parties. Nom provisoire.

Les règles communes à tous les projets viennent de la Library (`JRRocks/Library`, `SOCLE.md`).
Ce fichier ne contient que ce qui est propre à ce projet.

## Mémoire du projet

| Fichier | Contenu |
|---|---|
| `docs/ORGANISATION.md` | Qui fait quoi : la session maître, les six sessions de travail, leurs branches et leurs dossiers |
| `docs/PLAN.md` | Tâches numérotées, statut, critères, liste « à vérifier sur le téléphone » |
| `docs/JOURNAL.md` | Journal général, tenu par la session maître, entrée la plus récente en haut |
| `docs/DECISIONS.md` | Décisions numérotées et datées |
| `docs/idees-futur.md` | Idées pour plus tard : à ne lire que sur demande |
| `docs/<dossier>/MISSION.md` | La mission de chaque session de travail |
| `docs/<dossier>/JOURNAL.md` | Le journal propre à chaque session de travail |

## Organisation en sessions

Une session maître pilote six sessions de travail (détail et règles : `docs/ORGANISATION.md`).

- L'utilisateur ne parle qu'à la session maître. Une session de travail ne s'adresse jamais à lui : questions, choix à valider, aperçus et résultats partent vers la session maître, qui les lui présente et rapporte ses réponses.
- Une session de travail ne pousse que sur sa branche et n'écrit que dans son dossier (et dans le code pour la session 1). Elle peut lire les branches des autres (`git fetch origin <branche>`).
- `main` est tenue par la session maître : c'est elle qui y fait entrer le travail validé.
- Pas de pull request.

## Commandes

Le code est pour l'instant sur la branche `s1-environnement` ; détail dans `docs/environnement/commandes.md`.

| Dossier | Langage | Contenu |
|---|---|---|
| `moteur/regles` | Rust | Règles du jeu : sans écran, sans horloge, sans virgule, reproductibles par graine ; pour le téléphone et pour l'entraînement sur le PC |
| `moteur/android` | Rust | Pont JNI vers l'application (`libjeu4x_android.so`) |
| `app` | Kotlin + Compose | Application Android ; `fr.jeu4x.app.moteur.Moteur` appelle le pont |
| `.github/workflows/apk.yml` | GitHub Actions | Tests, puis APK signé en pré-version |

```bash
cd moteur && cargo test                     # règles (Rust)
./gradlew :app:testDebugUnitTest            # application, pont Kotlin ↔ Rust, captures d'écran
./gradlew :app:assembleDebug                # APK de debug (jamais livré)
```

- `:app:testDebugUnitTest` compile d'abord le moteur pour le PC et le charge dans les tests ; `assemble*` le compile pour le téléphone par `cargo ndk` (`arm64-v8a`) ; captures dans `app/build/captures/*.png`.
- En session cloud : SDK Android 35 et NDK 27.2.12479018 (Library, `docs/methodes/android-en-session-cloud.md`, § 1 et 2, miroir Maven compris), `rustup target add aarch64-linux-android`, `cargo install cargo-ndk --version 4.1.2 --locked`, et le pot Robolectric `android-all-instrumented:15-robolectric-12650502-i7` préchargé depuis le miroir de Google.

## Livraison

- Version d'essai (D-17) : `Jeu4X-essai.<version>.apk`, identifiant d'application à part, clé d'essai fixe, pré-version dans les Releases du dépôt ; elle s'installe à côté du vrai jeu.
- Vrai jeu : `Jeu4X.0.1.<n>.apk` (`fr.jeu4x.app`, D-18), pré-version `apercu-0.1.<n>` dès que la clé de signature est dans les secrets du dépôt (`docs/environnement/signature.md`). Il s'installe par-dessus la version précédente, données gardées.
- Dépôt : **public** (D-01).

## Dépôt public : rien de privé

Avant chaque commit et chaque push : skill `depot-public` et son script de contrôle.
Ni adresse e-mail, ni prénom d'un tiers, ni nom d'utilisateur ou chemin du poste, ni position réelle, ni lien d'invitation, ni journal de test brut, ni secret. Les commits portent une identité `noreply`.
Aucune clé de signature ne se versionne ici.

## Conventions

- Interface du jeu : français, avec les accents, dans les fichiers de ressources.
- Documents, journaux et messages de commit : français.
- Code : identifiants en anglais, commentaires en français (D-06).
- Idées numérotées avec le préfixe de la session qui les propose, pour que l'utilisateur trie d'un mot (« garde GD-004 ») : voir `docs/ORGANISATION.md`.

## Ce qui compte vraiment dans ce projet

1. **Un jeu original** (D-03). The Battle of Polytopia est la référence de genre : petites cartes, parties courtes, règles simples, lisibles sur un téléphone. On peut en reprendre des principes de jeu, comme de tout jeu du genre, mais jamais son nom, ses tribus, ses personnages, ses noms d'unités, ses dessins, ses icônes, sa musique, ses cartes ni son allure. Clans, noms, univers et graphismes s'inventent ici. Même règle pour tout autre jeu.
2. **Les règles du jeu à part de l'affichage.** Elles doivent jouer des parties entières sans écran, très vite et de façon reproductible (même graine, même partie) : l'IA s'entraînera dessus (voir `docs/ia/`).
3. **Rien de visible sans l'accord de l'utilisateur.** Écran, couleur, style : aperçu avec variantes d'abord, transmis par la session maître, puis son « oui ».
4. **La recherche propose, l'utilisateur trie** (D-07, D-11). Les sessions de recherche livrent des catalogues d'idées numérotées ; seul ce que l'utilisateur retient entre dans le plan.

## Ce que seul le téléphone peut vérifier

Gestes (toucher, glisser, pincer pour zoomer), lisibilité des cases sur un petit écran, fluidité, autonomie, installation par-dessus la version précédente.
