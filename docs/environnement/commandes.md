# Commandes du projet

> Session 1, 2026-11-27. À reporter dans `CLAUDE.md`, section « Commandes » (fichier commun : la session maître le fait).

## Organisation du code

| Dossier | Langage | Contenu |
|---|---|---|
| `moteur/regles` | Rust | Les règles du jeu : sans écran, sans horloge, sans virgule, reproductibles par graine. Servent au téléphone et à l'entraînement sur le PC |
| `moteur/android` | Rust | Le pont JNI vers l'application (bibliothèque `libjeu4x_android.so`) |
| `app` | Kotlin + Compose | L'application Android ; `fr.jeu4x.app.moteur.Moteur` appelle le pont |
| `.github/workflows/apk.yml` | GitHub Actions | Tests, puis APK signé en pré-version |

## Vérifier

```bash
cd moteur && cargo test                     # règles (Rust)
./gradlew :app:testDebugUnitTest            # application, pont Kotlin ↔ Rust, captures d'écran
./gradlew :app:assembleDebug                # APK de debug (jamais livré)
```

- `:app:testDebugUnitTest` compile d'abord le moteur pour le PC (`cargo build --release`) et le charge dans les tests.
- `assemble*` compile le moteur pour le téléphone par `cargo ndk` (`arm64-v8a`).
- Captures d'écran : `app/build/captures/*.png`.

## Outils nécessaires (session cloud)

- SDK Android (plateforme 35, build-tools 35.0.0) et NDK 27.2.12479018 : méthode de la Library `android-en-session-cloud.md`, § 1, plus `"ndk;27.2.12479018"` ; `local.properties` pointe le SDK.
- `rustup target add aarch64-linux-android` et `cargo install cargo-ndk --version 4.1.2 --locked`.
- Miroir Maven contre les 429 (`~/.gradle/init.d/`, même méthode, § 2).
- Robolectric télécharge lui-même `org.robolectric:android-all-instrumented:15-robolectric-12650502-i7` (≈ 200 Mo) : en session, le précharger dans `~/.m2/repository` depuis le miroir de Google, sinon 429.

## Livrer

Push sur `s1-environnement` ou `main` qui touche le code → workflow `APK` → pré-version `apercu-0.1.<n>` avec `Jeu4X.0.1.<n>.apk`, si la clé de signature est dans les secrets (`signature.md`).
