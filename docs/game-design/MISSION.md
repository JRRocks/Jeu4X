# Mission de la session 2 : recherche de game design

> Écrite par la session maître le 2026-11-27. Branche : `s2-game-design`. Dossier : `docs/game-design/`. Préfixe des idées : `GD-`.
> Lire d'abord `CLAUDE.md`, `docs/ORGANISATION.md` et `docs/DECISIONS.md`.

## Ce que l'utilisateur a demandé

Travailler en toute indépendance sur les idées de game design : l'arbre d'évolution, les différents clans, la différence entre les clans, les méthodes de gameplay selon les clans — des clans agressifs, défensifs, basés sur l'évolution, sur la recherche, sur l'exploration, et tout ce qui peut se trouver dans le monde du jeu vidéo. Rien n'est implémenté tout de suite : c'est de la recherche. L'utilisateur fera le tri, et seul ce qu'il retient sera implémenté (D-07).

Tu ne codes rien.

## Ce qu'il faut couvrir

1. **La boucle de jeu 4X sur téléphone** : explorer, s'étendre, exploiter, combattre, en parties courtes ; conditions de victoire ; tailles de carte ; limite de tours ; ce qui rend une partie lisible et tendue en 10 à 30 minutes.
2. **L'arbre d'évolution** : formes possibles (arbre, réseau, branches par thème, ères, cartes tirées au sort, arbre propre à chaque clan…), taille adaptée au téléphone, coûts et monnaie, ce que chaque avancée débloque, rythme. Propose deux ou trois arbres originaux, en esquisse.
3. **Les clans**, le plus largement possible : agressif, défensif, évolution (unités qui évoluent ou mutent), recherche, exploration, et tout le reste (économie, commerce, diplomatie, culture, nomades, mer, nature, bâtisseurs, essaim ou élite, ruse, mystique…). Pour chaque clan : l'idée, son style de jeu, ses forces et ses faiblesses, comment il gagne, comment on le contre, son rythme en début, milieu et fin de partie.
4. **Ce qui différencie les clans** : niveaux d'asymétrie (simple bonus de départ, unités propres, mécaniques propres, victoire propre) et équilibrage (contres, pierre-feuille-ciseaux, parties simulées : l'IA de la session 3 pourra servir à équilibrer).
5. **Un socle minimal recommandé** pour une première version jouable : combien de clans, quel arbre, quelles règles.

Clans, noms et univers s'inventent (D-03) : des noms de travail originaux, jamais ceux d'un jeu existant.

## Livrables

- `docs/game-design/catalogue.md` : tableau résumé, puis une fiche par idée `GD-NNN` (format de `docs/ORGANISATION.md`, règle 9).
- `docs/game-design/clans.md`, `arbre-evolution.md`, `boucle-de-jeu.md`, `sources.md`, `JOURNAL.md`.
- Un rapport à la session maître : où lire, les cinq idées que tu recommandes le plus, les questions pour le tri.

## Voisins

La session 5 fait une veille large du monde du jeu vidéo (modes de jeu, social, modèle économique…) ; toi, tu conçois nos systèmes (arbre, clans, styles de jeu). La session 6 s'occupe des visuels, la session 3 de l'IA.
