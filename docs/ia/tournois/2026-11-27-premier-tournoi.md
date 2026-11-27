# Tournoi des IA

Commande : `cargo run --release -p ia -- tournoi --ia hasard-v1,utilite-v1,utilite-v2,utilite-recul --parties 200 --graine 1 --taille 11`

Règles version 1 ; plateau 11×11, 3 unités par joueur, 30 tours. 200 parties par paire, chaque carte jouée des deux côtés ; 1200 parties en tout, 0 forfait(s).

## Classement

Elo ajusté sur toutes les parties (Bradley-Terry), ancré à 1000 pour la première IA de la liste.

| Rang | IA | Elo | Points | Description |
|---|---|---|---|---|
| 1 | utilite-v1 | 1495 | 81.1 % | utilité : mon score 100, score adverse -100, mon avancée 10, avancée adverse 0 |
| 2 | utilite-v2 | 1495 | 81.1 % | utilité : mon score 100, score adverse -100, mon avancée 10, avancée adverse -4 |
| 3 | hasard-v1 | 1000 | 29.1 % | au hasard |
| 4 | utilite-recul | 794 | 8.8 % | utilité : mon score 100, score adverse -100, mon avancée -10, avancée adverse 0 |

## Face à face

Victoires-égalités-défaites de l'IA de la ligne contre celle de la colonne.

|  | utilite-v1 | utilite-v2 | hasard-v1 | utilite-recul |
|---|---|---|---|---|
| utilite-v1 | — | 47-106-47 | 180-17-3 | 196-4-0 |
| utilite-v2 | 47-106-47 | — | 179-19-2 | 196-4-0 |
| hasard-v1 | 3-17-180 | 2-19-179 | — | 103-97-0 |
| utilite-recul | 0-4-196 | 0-4-196 | 0-97-103 | — |

## Vitesse (seule partie qui varie d'une machine à l'autre)

1200 parties, 126107 actions, en 0.38 s sur 4 fils : 3138 parties/s.

Toutes les parties sont rejouables : `cargo run --release -p ia -- rejouer target/premier-tournoi.replays.txt`.
