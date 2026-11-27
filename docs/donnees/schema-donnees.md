# Schéma des données (proposition)

> Session 4, 2026-11-27. Étude : rien n'est codé (D-11). Première version écrite sur les
> hypothèses H1 à H8 ; **revue le même jour** avec `docs/ia/besoins-donnees.md` de la session 3
> (branche `s3-ia`, commits a3e1385 et 9c5ecd0). Ce qui a changé est dans la section « Revue S3 ».

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
| H8 | Un joueur actif joue 1 à 3 parties par jour ; en moyenne sur l'année, 300 parties par joueur. | Volumes linéaires : diviser ou multiplier. **Remplacée** (revue S3) : S3 compte ≈ 10 parties par jour et par joueur actif ; on garde les deux, en fourchette basse et haute. |

## 1. La partie (replay compact)

Unité de base, écrite sur le téléphone à la fin de chaque partie (et à chaque abandon).

| Champ | Type | Exemple | Pourquoi |
|---|---|---|---|
| `format` | entier | `1` | Version de ce schéma, pour relire les vieux replays |
| `replay_hash` | empreinte SHA-256 du replay | `3f2c…` | Clé unique : dédoublonne les envois répétés (choix de S3, remplace un `game_id` tiré au hasard) |
| `player_id` | UUID v4 aléatoire | `a91e…` | Regrouper les parties d'un même joueur sans savoir qui il est ; réinitialisable (§ 6). Nom de S3 ; s'appelait `install_id` dans la première version |
| `app_version` | texte | `0.4.2` | Retrouver le bon APK |
| `rules_version` | texte | `rules-7` | Rejouer avec les bonnes règles ; un replay ne se rejoue qu'avec son moteur (nom de S3, ex-`engine_version`) |
| `encoding_version` | entier | `3` | Savoir quels modèles d'IA peuvent apprendre sur cette partie (ajout S3) |
| `seed` | entier 64 bits | | Recrée la carte et tout le hasard |
| `map` | objet | `{size: 11, kind: "iles"}` | Paramètres choisis par le joueur |
| `players[]` | liste | voir dessous | Qui jouait quoi |
| `actions` | octets compressés | | Le cœur : la suite des numéros d'actions, avec le tour et la place |
| `checks` | empreintes de l'état tous les N tours et à la fin | | Vérifier le rejeu et repérer une partie truquée ; remplace l'empreinte des règles de la première version (ajout S3) |
| `result` | objet | `{winner: 0, ranks: [1,3,2], reason: "conquete", turns: 27}` | Lu sans rejouer, pour les tableaux de bord |
| `duration_s` | entier | `912` | Durée réelle de jeu |
| `played_on` | date (jour seulement) | `2026-12-03` | Suivre dans le temps sans heure précise (minimisation) |
| `abandoned` | booléen | | Un abandon dit aussi quelque chose de l'IA (trop forte, ennuyeuse) |
| `think_time` | facultatif : secondes par tour, arrondies | | Repérer les joueurs automatiques (ajout S3). Seule donnée de comportement fin : à n'envoyer qu'avec le consentement général, jamais d'horodatage |

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

`{batch_id, player_id, sent_on, parties: [partie…], integrity_token}` — `integrity_token` :
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

- `player_id` : tiré au hasard au premier lancement, jamais dérivé du téléphone ni du compte Google.
- Bouton « Effacer mes données » : envoie une demande d'effacement pour ce `player_id`, puis en tire un nouveau.
- Au sens du RGPD, cet identifiant reste une donnée **pseudonyme**, donc personnelle (voir `etude.md`, § 3).

## 7. Volumes par échelle

Fourchette basse : 300 parties par an et par joueur (H8 d'origine). Fourchette haute : 10 parties
par jour et par joueur actif, 3 Ko par partie (S3 : 20 à 50 Mo par jour pour 1 000 joueurs actifs).

| Échelle | Parties par jour (basse – haute) | Replays bruts par an (basse – haute) | Remarque |
|---|---|---|---|
| 1 joueur | 1 – 10 | 0,6 – 11 Mo | Tient sur le téléphone sans y penser |
| 100 joueurs | 80 – 1 000 | 60 Mo – 1 Go | Tient dans toute offre gratuite |
| 1 000 joueurs | 800 – 10 000 | 0,6 – 11 Go | Limite de la gratuité du stockage objet (10 Go) |
| 10 000 joueurs | 8 000 – 100 000 | 6 – 110 Go | Premier palier payant : quelques dollars par mois (`etude.md`, § 2) |
| Évaluations IA contre IA | — | on ne garde que les résumés : < 10 Mo | Les replays d'entraînement restent sur la machine d'entraînement (S3) |

Débit d'envoi : en lots quotidiens, une requête par joueur et par jour, soit 10 000 requêtes par
jour à 10 000 joueurs, sous les plafonds gratuits. Ce qui coince en haut de fourchette, c'est le
nombre de lignes écrites (une par partie) et le stockage cumulé, pas les requêtes.

## Revue S3 (2026-11-27)

Confrontation avec `docs/ia/besoins-donnees.md` (branche `s3-ia`) :

| Point | Avant | Après |
|---|---|---|
| H1 déterminisme, H4 entraînement hors téléphones, H5 usage des parties humaines, H7 pas de compte | hypothèses | **confirmées** par S3 |
| H2, H3 taille d'une partie | ≈ 2 Ko | confirmée (S3 : « quelques kilooctets, moins après compression ») |
| H8 rythme de jeu | 300 parties par an | **haussé** : jusqu'à 10 par jour et par joueur actif ; volumes × 10 environ en haut de fourchette |
| Clé de dédoublonnage | `game_id` aléatoire | `replay_hash` (empreinte du replay) |
| Contrôle anti-triche | empreinte des règles | `checks` : empreintes de l'état tous les N tours |
| Champs nouveaux | — | `encoding_version`, `think_time` (facultatif) |
| Noms | `install_id`, `engine_version` | `player_id`, `rules_version` (vocabulaire de S3) |
| Consentement | question ouverte à l'utilisateur | S3 propose aussi « désactivé par défaut » : même avis que l'option recommandée |
| Usage nouveau | — | **IA fantôme** d'un joueur (une IA qui joue comme lui), avec un accord à part : demande de garder le lien entre les parties d'un même `player_id` |
| Export | — | par jour ou par version : un fichier par jour dans le stockage objet convient tel quel |
