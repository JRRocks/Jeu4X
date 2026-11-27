# Équilibre : durée des parties et façon dont elles se gagnent

> Mesures de la session 3 pour la session 2, le 2026-11-27, règles 3 (V1 de S1, 0fa9c01). Les mêmes graines donnent les mêmes chiffres.

En bref : les parties durent 9 à 10 tours (visé : 25 à 35), et 92 à 98 % se gagnent au seuil de points, presque jamais par la capitale. Le premier joueur gagne nettement plus souvent à deux (64 % avec l'IA en service). Les Vents-Errants gagnent moins que les autres clans.

## Durée des parties (règles 3)

IA à toutes les places : utilite-r3-v1 (utilité : issue 100000, mon score 100, meilleur score adverse -60, vivres 2, revenu 15, mon armée 8, armée adverse vue -8, mes villes 150, approche 6, exploration 3, prêtes à rallier 120). Commande : `cargo run --release -p ia -- duree --ia utilite-r3-v1 --parties 200 --graine 1`.

### 2 joueurs (11×11, seuil 45)

200 parties. Durée : 9.8 tours en moyenne, médiane 10, de 6 à 14.

| Fin | Part |
|---|---|
| capitale prise (ou dernier debout) | 2 % |
| seuil de points | 98 % |
| nulle au tour 80 | 0 % |

| Tours | Parties |
|---|---|
| 5 à 9 | 75 |
| 10 à 14 | 125 |

| Clan | Parties jouées | Victoires |
|---|---|---|
| Crocs-Rouges | 100 | 52 % |
| Murailliers | 100 | 60 % |
| Veilleurs d'Astres | 100 | 55 % |
| Vents-Errants | 100 | 33 % |

| Place | Victoires |
|---|---|
| 1 | 64 % |
| 2 | 36 % |

Le gagnant finit en moyenne avec 48.4 points et 5.3 villes.

### 3 joueurs (12×12, seuil 45)

200 parties. Durée : 9.6 tours en moyenne, médiane 10, de 8 à 12.

| Fin | Part |
|---|---|
| capitale prise (ou dernier debout) | 8 % |
| seuil de points | 92 % |
| nulle au tour 80 | 0 % |

| Tours | Parties |
|---|---|
| 5 à 9 | 89 |
| 10 à 14 | 111 |

| Clan | Parties jouées | Victoires |
|---|---|---|
| Crocs-Rouges | 150 | 38 % |
| Murailliers | 150 | 40 % |
| Veilleurs d'Astres | 150 | 37 % |
| Vents-Errants | 150 | 18 % |

| Place | Victoires |
|---|---|
| 1 | 44 % |
| 2 | 28 % |
| 3 | 28 % |

Le gagnant finit en moyenne avec 48.4 points et 5.0 villes.

### 4 joueurs (14×14, seuil 40)

200 parties. Durée : 8.6 tours en moyenne, médiane 9, de 7 à 10.

| Fin | Part |
|---|---|
| capitale prise (ou dernier debout) | 2 % |
| seuil de points | 98 % |
| nulle au tour 80 | 0 % |

| Tours | Parties |
|---|---|
| 5 à 9 | 190 |
| 10 à 14 | 10 |

| Clan | Parties jouées | Victoires |
|---|---|---|
| Crocs-Rouges | 200 | 26 % |
| Murailliers | 200 | 24 % |
| Veilleurs d'Astres | 200 | 30 % |
| Vents-Errants | 200 | 20 % |

| Place | Victoires |
|---|---|
| 1 | 31 % |
| 2 | 33 % |
| 3 | 18 % |
| 4 | 18 % |

Le gagnant finit en moyenne avec 42.8 points et 5.3 villes.


## Durée des parties (règles 3)

IA à toutes les places : simple-r3 (adversaire provisoire des règles (S1)). Commande : `cargo run --release -p ia -- duree --ia simple-r3 --parties 200 --graine 1`.

### 2 joueurs (11×11, seuil 45)

200 parties. Durée : 9.1 tours en moyenne, médiane 9, de 4 à 12.

| Fin | Part |
|---|---|
| capitale prise (ou dernier debout) | 6 % |
| seuil de points | 94 % |
| nulle au tour 80 | 0 % |

| Tours | Parties |
|---|---|
| 0 à 4 | 2 |
| 5 à 9 | 145 |
| 10 à 14 | 53 |

| Clan | Parties jouées | Victoires |
|---|---|---|
| Crocs-Rouges | 100 | 52 % |
| Murailliers | 100 | 46 % |
| Veilleurs d'Astres | 100 | 54 % |
| Vents-Errants | 100 | 48 % |

| Place | Victoires |
|---|---|
| 1 | 58 % |
| 2 | 42 % |

Le gagnant finit en moyenne avec 48.9 points et 5.0 villes.

### 3 joueurs (12×12, seuil 45)

200 parties. Durée : 9.1 tours en moyenne, médiane 9, de 8 à 11.

| Fin | Part |
|---|---|
| capitale prise (ou dernier debout) | 14 % |
| seuil de points | 86 % |
| nulle au tour 80 | 0 % |

| Tours | Parties |
|---|---|
| 5 à 9 | 148 |
| 10 à 14 | 52 |

| Clan | Parties jouées | Victoires |
|---|---|---|
| Crocs-Rouges | 150 | 31 % |
| Murailliers | 150 | 32 % |
| Veilleurs d'Astres | 150 | 38 % |
| Vents-Errants | 150 | 33 % |

| Place | Victoires |
|---|---|
| 1 | 43 % |
| 2 | 34 % |
| 3 | 22 % |

Le gagnant finit en moyenne avec 49.8 points et 4.9 villes.

### 4 joueurs (14×14, seuil 40)

200 parties. Durée : 8.3 tours en moyenne, médiane 8, de 7 à 9.

| Fin | Part |
|---|---|
| capitale prise (ou dernier debout) | 10 % |
| seuil de points | 90 % |
| nulle au tour 80 | 0 % |

| Tours | Parties |
|---|---|
| 5 à 9 | 200 |

| Clan | Parties jouées | Victoires |
|---|---|---|
| Crocs-Rouges | 200 | 31 % |
| Murailliers | 200 | 22 % |
| Veilleurs d'Astres | 200 | 21 % |
| Vents-Errants | 200 | 26 % |

| Place | Victoires |
|---|---|
| 1 | 33 % |
| 2 | 30 % |
| 3 | 17 % |
| 4 | 20 % |

Le gagnant finit en moyenne avec 43.9 points et 5.0 villes.

