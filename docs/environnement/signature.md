# Clé de signature de l'APK

> Session 1, 2026-11-27. Le dépôt est public : aucune clé ne s'y versionne.

## Pourquoi

Android n'installe une version par-dessus la précédente, données gardées, que si les deux portent **la même signature**. La clé est donc créée une seule fois, gardée en lieu sûr, et ne change jamais. Elle vit seulement dans les secrets GitHub du dépôt, que la CI lit pour signer.

Sans ces secrets, la CI lance tous les tests mais ne produit aucun APK : un APK signé d'une autre clé obligerait plus tard à désinstaller.

## Ce que l'utilisateur fait, une seule fois (sur le PC, environ 5 minutes)

1. Dans un terminal où Java est installé (Android Studio en fournit un), créer la clé :

   ```
   keytool -genkeypair -v -keystore jeu4x-release.jks -alias jeu4x -keyalg RSA -keysize 4096 -validity 36500 -storetype PKCS12
   ```

   Un seul mot de passe est demandé : le choisir long, et le noter dans le gestionnaire de mots de passe.

2. Convertir la clé en texte (PowerShell) et la copier :

   ```
   [Convert]::ToBase64String([IO.File]::ReadAllBytes("jeu4x-release.jks")) | Set-Clipboard
   ```

3. Sur GitHub, dépôt Jeu4X : *Settings* → *Secrets and variables* → *Actions* → *New repository secret*, quatre fois :

   | Nom | Valeur |
   |---|---|
   | `SIGNATURE_KEYSTORE_BASE64` | le texte copié à l'étape 2 |
   | `SIGNATURE_STORE_PASSWORD` | le mot de passe |
   | `SIGNATURE_KEY_PASSWORD` | le même mot de passe |
   | `SIGNATURE_KEY_ALIAS` | `jeu4x` |

4. Ranger `jeu4x-release.jks` hors du dépôt, avec une sauvegarde (Drive privé par exemple). Perdue, plus aucune mise à jour ne s'installe par-dessus.

Ne jamais coller la clé ni le mot de passe dans une conversation.

## Ce qui se passe ensuite

À chaque push qui touche le code, la workflow `APK` (`.github/workflows/apk.yml`) teste, signe, nomme le fichier `Jeu4X.0.1.<n>.apk` et l'attache à une pré-version `apercu-0.1.<n>` des *Releases* du dépôt. `n` est le numéro d'exécution de la workflow : il monte seul et ne revient jamais deux fois.

## Vérification des développeurs Android (consulté le 2026-11-27)

Google impose peu à peu que chaque application installée sur un téléphone Android certifié soit enregistrée par un développeur vérifié : obligatoire depuis le 30 septembre 2026 au Brésil, en Indonésie, à Singapour et en Thaïlande pour les boutiques participantes, prévu partout en 2027. Un APK installé directement n'y est pas encore soumis.

- L'enregistrement se fait dans l'*Android Developer Console* : vérification d'identité, puis déclaration de chaque **identifiant d'application** (`fr.jeu4x.app`) avec l'**empreinte SHA-256 du certificat de signature** ; on prouve qu'on possède l'application en envoyant un APK signé de la clé.
- Un compte à **distribution limitée**, sans pièce d'identité, suffit pour partager une application avec 20 appareils au plus : adapté à ce projet tant qu'il reste privé. Au-delà : distribution complète, avec vérification d'identité.
- Sans enregistrement, l'installation reste possible par un parcours avancé de l'utilisateur ou par ADB.

Ce que cela impose ici, et que la chaîne respecte déjà : un identifiant d'application fixe et une seule clé de signature, gardée hors du dépôt. L'empreinte à déclarer le moment venu se lit sur la clé : `keytool -list -v -keystore jeu4x-release.jks -alias jeu4x` (ligne SHA256). Aucun compte n'est créé pour l'instant.

Sources : [blog Android Developers, juin 2026](https://android-developers.googleblog.com/2026/06/android-developer-verification.html) · [guide de la vérification](https://developer.android.com/developer-verification/guides) · [FAQ](https://developer.android.com/developer-verification/guides/faq?hl=en) · [aide Android Developer Console](https://support.google.com/android-developer-console/answer/16561738?hl=en). Point non tranché : la FAQ parle d'Android 7 et plus, l'aide Android d'Android 8 et plus.
