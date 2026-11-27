# Terrain d'essai des IA et première IA

> Écrit par Claude, session S3, le 2026-11-27 (mission 2). Code : `moteur/ia/`. Rien d'affiché dans le jeu.

## Ce qui existe

| Idée | Ce qui est fait | Comment le vérifier |
|---|---|---|
| IA-001 terrain d'essai | Tournoi entre IA lancé par une commande, sans écran, en parallèle ; même graine, même tournoi, quel que soit le nombre de fils ; classement Elo ; toutes les parties archivées et rejouables | `cargo run --release -p ia -- tournoi --parties 200 --graine 1 --archive target/t.txt` puis `… -- rejouer target/t.txt` |
| IA-002 première IA | IA à utilité : essaie chaque action permise sur une copie de l'état, note le résultat par des critères pondérés, joue la meilleure, finit son tour quand plus rien ne s'améliore | Tournoi : 180 victoires, 17 égalités, 3 défaites sur 200 contre l'IA au hasard |
| IA-004 la nouvelle doit battre l'ancienne | `galerie.txt` (IA admises, la dernière en service) et `candidats.txt` ; une candidate n'entre que si elle marque au moins 55 % des points contre l'IA en service et au moins 50 % contre chaque version plus ancienne | `cargo run --release -p ia -- valider <nom> [--inscrire]` ; code de sortie 0 si admise, 1 si refusée |

Commandes à lancer depuis `moteur/`. `… -- galerie` liste la galerie et les candidates.

## Premier tournoi (2026-11-27)

Rapport complet : `tournois/2026-11-27-premier-tournoi.md` (1 200 parties, 0 forfait, rejeu 1 200 sur 1 200 conformes).

| IA | Elo | Ce qu'elle fait |
|---|---|---|
| utilite-v1 (en service) | 1495 | avance ses unités vers la moitié adverse, en contournant l'eau et les montagnes |
| utilite-v2 (candidate) | 1495 | la même, en plus attentive à l'avancée adverse |
| hasard-v1 | 1000 | joue au hasard |
| utilite-recul (candidate) | 794 | exprès absurde : recule |

Validations du même jour :
- `utilite-v1` : **admise** (95,8 % des points contre le hasard sur 400 parties), inscrite dans la galerie.
- `utilite-v2` : **refusée**, 50,0 % contre `utilite-v1` (98-204-98). Les règles actuelles n'ont aucune
  interaction entre joueurs : regarder l'adversaire ne change aucun coup. La règle fait son travail.
- `utilite-recul` : **refusée**, perd même contre le hasard.

Vitesse : environ 3 000 parties par seconde sur 4 fils avec les IA à utilité (la mesure varie selon la machine).

## Ce que la première IA a appris de ses erreurs

Une première version mesurait l'avancée en rangées. Elle restait bloquée derrière l'eau,
car un pas de côté ne rapprochait d'aucune rangée : 84 % de victoires contre le hasard,
avec 6 % de défaites. La version actuelle mesure la distance réelle à parcourir selon le
terrain. Elle gagne 90 % des parties contre le hasard, avec 1,5 % de défaites. Les égalités
viennent des parties où l'IA au hasard finit, elle aussi, avec toutes ses unités en face.

## Suivre l'arrivée des règles de la première version

Le jeu d'aujourd'hui (plateau, terrains, déplacements, score provisoire) est trop simple pour
une IA forte : l'IA à utilité y est presque parfaite. Quand S1 codera les règles V1 (villes,
combats sans chance, monnaie, arbre, victoire par capitale ou par points, D-23, D-28) :

1. `moteur/ia/src/criteres.rs` reçoit les nouveaux critères (menace, villes, revenu…), et
   `VERSION_CRITERES` monte : les anciens profils sont refusés à la lecture.
2. Les règles montent `VERSION_REGLES` : la galerie écarte les versions faites pour les
   anciennes règles et les signale « à requalifier ».
3. Le terrain d'essai, l'archive, le classement et la validation ne changent pas : ils ne
   passent que par l'interface publique des règles (actions permises, appliquer, copier,
   scores, gagnant, replay).
4. Le brouillard (D-23) : l'IA devra lire la vue de son joueur dès que les règles la fourniront ;
   aujourd'hui elle lit l'état complet, car il n'y a pas encore de brouillard.
5. Plus de deux joueurs : le tournoi joue aujourd'hui des duels (`NB_JOUEURS = 2` dans les règles).

## Limites connues

- Aucune IA ne réfléchit plus d'un coup à l'avance : c'est l'étape suivante (planification du tour, IA-005).
- Les poids de `utilite-v1` sont réglés à la main ; leur évolution automatique (IA-003) viendra avec les règles V1, quand il y aura de quoi apprendre.
- L'archive d'un tournoi pèse environ 600 Ko pour 1 200 parties : elle n'est pas versionnée ; elle se refait à l'identique avec la même graine.

## Règles V1 complètes (règles 3), 2026-11-27

- Adversaire à brancher : `ia::adversaire::jouer_tour(&Partie, Niveau)` (facile = hasard, normal = l'IA en service).
- En service : `utilite-r3-v1` (onze critères lus sur la vue du joueur). Contre l'adversaire provisoire de S1 (`simple-r3`) : 119-0-81 sur 200 parties ; contre le hasard : 200-0-0. Rapport : `tournois/2026-11-27-regles-3.md`.
- Temps de réflexion (PC, `terrain chrono`) : 1 à 2 ms par tour en moyenne, 7,5 ms au pire.
- Évolution courte des poids (IA-003, `terrain evoluer`) : deux essais de 1 et 4 minutes donnent `utilite-r3-v2` et `utilite-r3-v3`. Elles battent mieux `simple-r3` (61,5 % et 66 % des points), mais pas assez `utilite-r3-v1` : 52,7 % et 54 % pour 55 % exigés. Elles sont **refusées** et restent candidates ; la règle IA-004 a joué son rôle.
