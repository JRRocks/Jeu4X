# Schéma des données (proposition)

> Session 4, 2026-11-27. Étude : rien n'est codé (D-11). Les besoins de la session 3
> (`docs/ia/besoins-donnees.md`, branche `s3-ia`) n'existaient pas encore à cette date :
> tout ce qui suit repose sur les hypothèses H1 à H8 ci-dessous, à reprendre quand ils arrivent.

## Hypothèses de départ

| N° | Hypothèse | Si elle tombe |
|---|---|---|
| H1 | Le moteur de règles est **déterministe** : même graine + mêmes actions = même partie, à l'octet près (`CLAUDE.md`, point 2). | Il faudrait stocker l'état de chaque tour : volume ×50 à ×200. |
| H2 | Petites cartes, 2 à 4 joueurs, partie de 20 à 40 tours, 5 à 15 actions par joueur et par tour. | Volume à recalculer (il reste proportionnel au nombre d'actions). Cohérent avec S3 (`docs/ia/besoins-moteur.md`, branche `s3-ia` : ≈ 1 000 actions par partie, 30 à 50 tours). |
| H3 | Une action tient en 3 à 5 octets (type, unité ou ville, case cible). | Idem. |
| H4 | L'IA s'entraîne surtout **contre elle-même** sur un PC ou en session cloud (S3) ; ces parties-là ne passent jamais par les téléphones. | Aucune : c'est l'hypothèse la plus sûre. |
| H5 | Les parties des joueurs humains servent à **mesurer** l'IA et, plus tard, à la **pré-entraîner par imitation**. | Si S3 n'en veut que des agrégats, on ne remonte que le résultat (≈ 200 octets). |
| H6 | Un modèle d'IA pèse de 100 Ko à 5 Mo. | Au-delà de 50 Mo, revoir le canal de retour (§ 5 de `etude.md`). |
| H7 | Aucun compte joueur : un identifiant d'installation aléatoire suffit. | Un compte ajoute e-mail, mot de passe, lien de suppression obligatoire au Play Store. |
| H8 | Un joueur actif joue 1 à 3 parties par jour ; en moyenne sur l'année, 300 parties par joueur. | Volumes linéaires : diviser ou multiplier. |

## 1. La partie (replay compact)

Unité de base, écrite sur le téléphone à la fin de chaque partie (et à chaque abandon).

| Champ | Type | Exemple | Pourquoi |
|---|---|---|---|
| `format` | entier | `1` | Version de ce schéma, pour relire les vieux replays |
| `game_id` | UUID v4 | `3f2c…` | Dédoublonner les envois répétés |
| `install_id` | UUID v4 aléatoire | `a91e…` | Regrouper les parties d'un même joueur sans savoir qui il est ; réinitialisable (§ 6) |
| `app_version` | texte | `0.4.2` | Retrouver le bon APK |
| `engine_version` | texte | `rules-7` | Rejouer avec les bonnes règles ; un replay ne se rejoue qu'avec son moteur |
| `ruleset_hash` | 8 octets | `9b1d…` | Détecter un moteur modifié (triche, version maison) |
| `seed` | entier 64 bits | | Recrée la carte et tout le hasard |
| `map` | objet | `{size: 11, kind: "iles"}` | Paramètres choisis par le joueur |
| `players[]` | liste | voir dessous | Qui jouait quoi |
| `actions` | octets compressés | | Le cœur : la suite des actions, tour par tour |
| `result` | objet | `{winner: 0, ranks: [1,3,2], reason: "conquete", turns: 27}` | Lu sans rejouer, pour les tableaux de bord |
| `duration_s` | entier | `912` | Durée réelle de jeu |
| `played_on` | date (jour seulement) | `2026-12-03` | Suivre dans le temps sans heure précise (minimisation) |
| `abandoned` | booléen | | Un abandon dit aussi quelque chose de l'IA (trop forte, ennuyeuse) |

`players[]` : `{slot, kind: "humain" | "ia", clan, ai_model: "ia-2026.12-a" | null, difficulty, start_tile}`.

Ce qu'on **ne** met **pas** : nom, e-mail, position, modèle de téléphone, adresse IP (le serveur
ne la garde pas), heure précise, texte libre.

### Encodage des actions (à arrêter avec S1 et S3)

Proposition : flux d'entiers variables (*varint*), un marqueur de fin de tour, puis
compression `deflate` (disponible partout, sans bibliothèque). Une action = type (1 octet) +
acteur (1 octet) + case cible (1 à 2 octets) + paramètre éventuel.

### Volume d'une partie

| Poste | Calcul (H2, H3) | Octets |
|---|---|---|
| En-tête et résultat | | ≈ 250 |
| Actions brutes | 3 joueurs × 30 tours × 10 actions × 4 octets | ≈ 3 600 |
| Actions compressées | facteur 2 à 3 sur des données répétitives | ≈ 1 500 |
| **Total par partie** | | **≈ 2 Ko** (fourchette 1 à 5 Ko) |

## 2. Le lot d'envoi

Le téléphone garde ses parties et les envoie par lots quand le réseau est là (formule mixte).

`{batch_id, install_id, sent_on, parties: [partie…], integrity_token}` — `integrity_token` :
jeton Play Integrity (DO-011), vérifié côté serveur, jamais stocké.

## 3. Les évaluations IA contre IA (produites sur le PC ou en CI, jamais sur le téléphone)

`{eval_id, engine_version, model_a, model_b, clans, map_kind, seeds: "1000-1999", games: 1000, wins_a, wins_b, draws, avg_turns, run_on}`

C'est la source principale des courbes de progrès : des milliers de parties par version, sans
aucun joueur, donc sans aucune donnée personnelle (publiable dans le dépôt public).

## 4. Le manifeste des modèles (retour vers les téléphones)

`{model_id: "ia-2026.12-a", engine_version: "rules-7", file: "ia-2026.12-a.bin", size, sha256, min_app_version, rollout: 0.2, notes}`

## 5. Les agrégats (tableau de bord)

Recalculés à partir des parties et des évaluations, jamais saisis à la main :
taux de victoire par version d'IA et par niveau, classement des versions (OpenSkill),
matrice clan contre clan, durée moyenne, taux d'abandon, répartition des actions par grande
famille (expansion, guerre, recherche, économie).

## 6. Identité et effacement

- `install_id` : tiré au hasard au premier lancement, jamais dérivé du téléphone ni du compte Google.
- Bouton « Effacer mes données » : envoie une demande d'effacement pour cet `install_id`, puis en tire un nouveau.
- Au sens du RGPD, cet identifiant reste une donnée **pseudonyme**, donc personnelle (voir `etude.md`, § 3).

## 7. Volumes par échelle

| Échelle | Parties par an (H8) | Replays bruts | Remarque |
|---|---|---|---|
| 1 joueur | 300 à 1 000 | 0,6 à 5 Mo | Tient sur le téléphone sans y penser |
| 100 joueurs | 30 000 | ≈ 60 Mo (30 à 150 Mo) | Tient dans toute offre gratuite |
| 10 000 joueurs | 3 millions | ≈ 6 Go (3 à 15 Go) | Offre gratuite de stockage objet ou premier palier payant |
| Évaluations IA contre IA | 100 000 à 10 millions | on ne garde que les résumés : < 10 Mo | Les replays d'entraînement restent sur la machine d'entraînement (S3) |

Débit d'envoi à 10 000 joueurs : ≈ 8 000 parties par jour, soit, en lots quotidiens, environ
10 000 requêtes par jour : bien sous les plafonds gratuits des services étudiés.
