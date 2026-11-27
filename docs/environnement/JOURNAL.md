# Journal de la session 1 : environnement de jeu

> Entrée la plus récente en haut. On ne modifie ni ne supprime une entrée passée.

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
