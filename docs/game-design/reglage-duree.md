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

## 5. Confirmation par S3

En attente : tournoi complet (durées, façons de gagner, taux de victoire par clan).
