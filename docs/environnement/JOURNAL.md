# Journal de la session 1 : environnement de jeu

> Entrée la plus récente en haut. On ne modifie ni ne supprime une entrée passée.

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
