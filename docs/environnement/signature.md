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
