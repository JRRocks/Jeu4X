# Tournoi des IA

Commande : `cargo run --release -p ia -- tournoi --ia hasard-r3,simple-r3,utilite-r3-v1,utilite-r3-v3,utilite-r3-v2 --parties 200 --graine 1 --taille 11`

Règles version 3 ; plateau 11×11, 2 joueurs, limite 80 tours. 200 parties par paire, chaque carte jouée des deux côtés ; 2000 parties en tout, 0 forfait(s).

## Classement

Elo ajusté sur toutes les parties (Bradley-Terry), ancré à 1000 pour la première IA de la liste.

| Rang | IA | Elo | Points | Description |
|---|---|---|---|---|
| 1 | utilite-r3-v3 | 1747 | 75.6 % | utilité : issue 57899, mon score 113, meilleur score adverse -55, vivres 4, revenu 19, mon armée 13, armée adverse vue -4, mes villes 185, approche 6, exploration 1, prêtes à rallier 138 |
| 2 | utilite-r3-v1 | 1682 | 66.8 % | utilité : issue 100000, mon score 100, meilleur score adverse -60, vivres 2, revenu 15, mon armée 8, armée adverse vue -8, mes villes 150, approche 6, exploration 3, prêtes à rallier 120 |
| 3 | simple-r3 | 1621 | 58.1 % | adversaire provisoire des règles (S1) |
| 4 | utilite-r3-v2 | 1543 | 47.3 % | utilité : issue 148951, mon score 113, meilleur score adverse -59, vivres 3, revenu 15, mon armée 11, armée adverse vue -2, mes villes 91, approche 7, exploration 3, prêtes à rallier 201 |
| 5 | hasard-r3 | 1000 | 2.3 % | au hasard |

## Face à face

Victoires-égalités-défaites de l'IA de la ligne contre celle de la colonne.

|  | utilite-r3-v3 | utilite-r3-v1 | simple-r3 | utilite-r3-v2 | hasard-r3 |
|---|---|---|---|---|---|
| utilite-r3-v3 | — | 121-15-64 | 133-1-66 | 119-50-31 | 197-3-0 |
| utilite-r3-v1 | 64-15-121 | — | 123-0-77 | 117-47-36 | 198-2-0 |
| simple-r3 | 66-1-133 | 77-0-123 | — | 121-0-79 | 200-0-0 |
| utilite-r3-v2 | 31-50-119 | 36-47-117 | 79-0-121 | — | 168-32-0 |
| hasard-r3 | 0-3-197 | 0-2-198 | 0-0-200 | 0-32-168 | — |

## Vitesse (seule partie qui varie d'une machine à l'autre)

2000 parties, 477911 actions, en 58.34 s sur 4 fils : 34 parties/s.

Toutes les parties sont rejouables : `cargo run --release -p ia -- rejouer target/r3b.txt`.
