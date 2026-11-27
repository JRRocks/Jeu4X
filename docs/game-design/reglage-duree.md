# Réglage de la durée des parties (mission 3)

> Session 2, le 2026-11-27. Seules des valeurs des tables de données changent (`regles-v1.md` § 12). Aucune règle validée ne bouge.

## 1. Le constat

Entre IA, les parties duraient **environ 10 tours** au lieu des 25 à 35 visés.

| Mesure (branche `s3-ia`, commit ab33186) | 2 joueurs | 3 joueurs | 4 joueurs |
|---|---|---|---|
| Adversaire de S3 (`chrono`, niveau Normal, 20 parties) | 10,1 tours | 9,8 tours | 8,3 tours |
| `ia_simple` (exemple `parties_ia`, 40 parties) | 9,7 tours, 40 au seuil | 9,2 tours, 40 au seuil | 8,2 tours, 40 au seuil |

## 2. Pourquoi

- **Toutes les parties finissent au seuil de points.** Aucune ne finit par la prise d'une capitale.
- **Faire grandir ses villes rapporte trop pour trop peu.** Le niveau 2 coûte 5 vivres et rapporte 2 points, +1 de revenu et un cadeau. Une avancée rapporte 1 point pour 5 à 14 vivres. Les IA ne font donc que grandir.
- **L'économie s'emballe.** Le revenu monte de 1 à chaque niveau, et les coûts restent bas. Dans une partie suivie tour par tour, une ville passe du niveau 2 au niveau 5 en trois tours, le seuil de 45 tombe au tour 9, et aucune avancée n'a été prise.

## 3. Le réglage proposé

| Donnée | Avant | Proposé | Pourquoi |
|---|---|---|---|
| Coût pour monter de niveau (`grow_cost`, niveaux 2 à 5) | 5, 8, 11, 14 | **8, 14, 20, 26** | Grandir devient un vrai investissement |
| Revenu d'une ville par niveau (`income`, 1 à 5) | 2, 3, 4, 5, 6 | **2, 2, 3, 3, 4** | Casse l'emballement de l'économie |
| Points tenus par niveau (`held_points`, 1 à 5) | 2, 4, 6, 8, 10 | **1, 2, 3, 4, 5** | Les villes pèsent moins, les avancées et les merveilles comptent davantage |
| Seuil de points (2, 3, 4 joueurs) | 45, 45, 40 | **55, 52, 55** | Ramène la durée vers 30 tours |

## 4. Les essais

Mesures faites par S2 dans une copie de travail hors dépôt, puis confirmées par S3.

| Essai | Coûts | Revenus | Points | Seuils | Adversaire de S3 (2 / 3 / 4 joueurs) | `ia_simple` (2 / 3 / 4 joueurs) |
|---|---|---|---|---|---|---|
| Départ | 5, 8, 11, 14 | 2…6 | 2…10 | 45, 45, 40 | 10,1 / 9,8 / 8,3 | 9,7 / 9,2 / 8,2 |
| 1 | 10, 18, 28, 40 | 2, 2, 3, 3, 4 | 1…5 | 70, 70, 65 | 44,6 / 47,7 / 37,4 | 21,3 / 26,1 / 24,0 |
| 2 | 8, 14, 20, 26 | 2, 2, 3, 3, 4 | 1…5 | 50, 50, 45 | 24,7 / 23,6 / 21,2 | — |
| 3 | 8, 14, 20, 26 | 2, 2, 3, 3, 4 | 1…5 | 55, 55, 50 | 31,8 / 39,0 / 22,8 | — |
| **Retenu** | 8, 14, 20, 26 | 2, 2, 3, 3, 4 | 1…5 | **55, 52, 55** | **31,8 / 29,6 / 29,6** | 18,2 / 20,1 / 19,6 |

Observations :
- La durée réagit fortement au seuil : 5 points de plus donnent 7 à 10 tours de plus avec l'adversaire de S3.
- Avec `ia_simple` à deux joueurs, un tiers des parties finissent par une capitale prise, parfois dès le tour 5. Cette IA laisse sa capitale vide. Ajouter +2 de défense aux capitales n'y change rien : c'est son jeu, pas un chiffre. L'adversaire de S3 sert de référence, puisque c'est lui que l'utilisateur affrontera.

## 5. Tournois de S3

Fait sur 200 parties par format, avec les clans en rotation : chaque clan joue à chaque place. Détail dans `docs/ia/equilibre/` sur `s3-ia`.

| Essai | IA | Durée (min.–max.) 2 / 3 / 4 joueurs | Fins par capitale / au seuil / nulles | Victoires par clan (Crocs / Murailliers / Veilleurs / Vents) | Victoires du premier joueur à 2 |
|---|---|---|---|---|---|
| Référence | IA de S3 | ≈ 9 à 10 tours | 86 à 98 % au seuil | Vents-Errants faibles | 64 % |
| 1 (réglage du § 3) | IA de S3 | 18,7 (6–26) / 19,4 (8–24) / 19,4 (16–23) | 28/72/0, 36/64/0, 53/47/0 | 49/46/56/49 % à 2 ; 24/25/27/24 % à 4 | 57 % |
| 1 | `ia_simple` | 26,1 / 26,4 / 29,1 | 13/82/5, 22/74/4, 28/64/8 | 59/42/59/30 % à 2 ; Vents-Errants faibles | 58 % |

Lecture : `chrono` opposait toujours Crocs-Rouges et Murailliers sur 20 parties, d'où mon 31,8, trop optimiste. Avec l'IA de S3, il manque 6 à 10 tours. Les clans sont équilibrés, mais les victoires par la capitale dominent à 4 joueurs.

Essai 2 demandé :
- **variante A** : seuils 62, 60 et 60 ;
- **variante B** : A, plus +2 de défense dans sa propre capitale et 2 vivres de départ par rang au lieu de 1.

| 2 A (seuils 62/60/60) | IA de S3 | moyenne 37,0 / 43,0 / 39,1 ; médiane 26 / 29 / 26 ; parties gagnées 25,6 / 30,0 / 29,9 | 14/65/21, 22/52/26, 28/54/18 | 47/31/53/27 % à 2 ; 24/21/22/16 % à 4 | 49 % contre 30 % |
| 2 B (A + capitale +2 et 2 vivres par rang) | IA de S3 | moyenne 36,7 / 43,0 / 35,2 ; médiane 26 / 28 / 25 ; parties gagnées 23,8 / 32,3 / 28,0 | 16/60/23, 20/58/22, 28/58/14 | 40/43/52/19 % à 2 ; 20/22/26/18 % à 4 | 43 % contre 34 % |

Lecture de l'essai 2 : la médiane et les parties gagnées tombent dans l'intervalle de 25 à 35 tours. Mais 14 à 26 % des parties finissent nulles au tour 80, sans doute parce que le score plafonne quand toutes les villes sont au maximum, peut-être aussi à cause de l'IA. B équilibre mieux le premier joueur.

Essai 3 demandé (le dernier) : B avec des seuils de 58, 56 et 56, plus le score du meilleur joueur dans les parties nulles.
