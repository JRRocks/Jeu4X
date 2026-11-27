# La boucle de jeu 4X sur téléphone

> Session 2, recherche de game design. Rien n'est décidé : ce document propose, l'utilisateur trie (D-07).
> Les idées citées entre crochets (`GD-NNN`) ont leur fiche dans `catalogue.md`.

## 1. Ce que doit donner une partie

Une partie de Jeu4X doit tenir **entre 10 et 30 minutes**, se jouer **d'une main**, en tours courts, et rester **lisible d'un coup d'œil** sur un écran de 6 pouces. La référence de genre (petites cartes, une seule monnaie, peu de règles) montre que cela fonctionne ; les critiques qu'on lui adresse le plus souvent indiquent aussi ce qu'il faut éviter : une fin de partie répétitive, des ressources qui s'accumulent sans choix à faire, et deux camps qui finissent avec les mêmes unités (voir `sources.md`).

Principe directeur, repris de la conférence de Sid Meier sur les « décisions intéressantes » : à chaque tour, le joueur doit avoir **au moins un vrai choix** (un arbitrage entre court et long terme, ou entre deux bonnes options), et **jamais une corvée** (déplacer vingt unités qui n'ont rien à faire).

## 2. Les quatre X, en version courte

| Phase | Ce que fait le joueur | Ce qui la rend courte | Tours (partie de 30 tours) |
|---|---|---|---|
| **Explorer** | Lever le brouillard, trouver villages, ruines, bêtes, ressources | Carte petite ; récompenses dès les premiers tours [GD-006] | 1 à 8 |
| **S'étendre** | Rallier ou prendre des villages neutres, fonder des postes | Pas de colons à escorter : on prend un village en s'y tenant un tour [GD-005] | 3 à 15 |
| **Exploiter** | Récolter, bâtir, faire grandir les villes, choisir des avancées | Une seule monnaie [GD-004] ; chaque ville qui grandit offre un choix [GD-013] | tout du long |
| **Combattre** | Prendre les villes ennemies, défendre les siennes | Combat sans hasard, résultat affiché avant d'attaquer [GD-007] | 8 à 30 |

Les phases se chevauchent : c'est voulu. La tension naît du moment où il faut **arrêter de s'étendre pour commencer à se battre**, et ce moment n'est pas le même selon le clan (voir `clans.md`).

## 3. Rythme d'une partie

```
Tours  1-5   : ouverture   → explorer, 1er village, 1re avancée. Peu de risque.
Tours  6-12  : expansion   → course aux villages neutres, premiers contacts, premières escarmouches.
Tours 13-20  : affrontement → frontières fixées, l'économie se convertit en armée ou en projet de victoire.
Tours 21-30  : dénouement  → compte à rebours visible, objectifs de fin, coups décisifs.
```

Leviers pour garder la fin de partie tendue (le défaut le plus cité du genre) :

- **Limite de tours avec score** [GD-001] : la partie finit, même sans vainqueur par conquête. Le score est affiché en permanence, on sait qui mène.
- **Compte à rebours et objectifs visibles** [GD-010] : « 5 tours restants », « la Guilde est à 2 comptoirs de sa victoire ».
- **Un événement de mi-partie** [GD-009] qui rebat les cartes : la mer monte et engloutit les côtes, une migration de bêtes traverse la carte, etc.
- **Rattrapage doux** [GD-008] : celui qui est en retard rattrape plus vite les avancées déjà connues des autres.
- **Peu d'unités** : un plafond d'unités par ville (par exemple autant que sa taille) évite la fin de partie à quarante pions.

## 4. Conditions de victoire

Trois familles, à combiner selon le mode de partie [GD-002] :

| Victoire | Règle (esquisse) | Durée typique | Bon pour |
|---|---|---|---|
| **Domination** | Prendre toutes les capitales adverses | 15 à 40 tours | Parties libres, clans agressifs |
| **Score** | Au tour limite (20, 30 ou 40), le plus haut score gagne. Score = villes + population + avancées + territoire + merveilles + bêtes vaincues | Fixe | Parties courtes, mode par défaut |
| **Victoire de clan** | Chaque clan a un objectif propre, annoncé à tous dès le début (voir `clans.md` et [GD-076]) | 20 à 30 tours | Asymétrie forte, plus tard |

Recommandation : **score à 30 tours + domination** pour la première version ; la victoire de clan n'arrive qu'avec l'asymétrie forte.

## 5. Tailles de carte

Le plateau est fait de cases adressées (A1, B2…), carrées, comme le prévoit la session 1. Estimations, à confirmer sur le téléphone (lisibilité des cases) :

| Format | Taille | Joueurs | Villages neutres | Durée visée | Tour limite |
|---|---|---|---|---|---|
| Duel | 9 × 9 (81 cases) | 2 | 4 à 6 | 10 min | 20 |
| Escarmouche | 11 × 11 (121 cases) | 2 à 3 | 8 à 10 | 15-20 min | 30 |
| Bataille | 14 × 14 (196 cases) | 3 à 4 | 12 à 16 | 25-30 min | 30 |
| Grand monde | 18 × 18 (324 cases) | 4 à 6 | 20 à 26 | 40 min et plus | 40 ou sans limite |

Repères : environ **25 à 35 cases par joueur** pour que le premier contact ait lieu vers le tour 6-8. Sur un écran de téléphone en portrait, une carte de 9 à 11 cases de large tient sans zoom ; au-delà, il faudra faire défiler ou pincer.

[GD-003] propose aussi des **formes de carte** : continent, archipel, deux îles, anneau autour d'un lac, carte qui rétrécit.

## 6. Ce qui rend la partie lisible

1. **Une seule monnaie** visible en haut de l'écran, avec le revenu du prochain tour [GD-004].
2. **Combat déterministe** : avant d'attaquer, on voit les dégâts infligés et reçus [GD-007]. Pas de dés : c'est plus lisible, et l'IA (session 3) apprend mieux sur des règles reproductibles.
3. **Peu de statistiques par unité** : attaque, défense, points de vie, mouvement, portée. Une capacité spéciale au plus.
4. **Peu de types de terrain** : plaine, forêt, colline, montagne, eau peu profonde, mer. Chaque terrain a un seul effet facile à retenir.
5. **Peu d'avancées** : 15 à 25 au total, chacune débloquant une chose nette (une unité, un bâtiment, une action) [GD-020].
6. **Un tour = peu de gestes** : sélectionner, voir les cases possibles, toucher, fin de tour. Le bouton « fin de tour » prévient s'il reste des unités inactives.

## 7. Ce qui rend la partie tendue

- **La rareté** : peu de villages neutres, à se disputer.
- **Le choix exclusif** : chaque ville qui grandit propose deux récompenses, on n'en prend qu'une [GD-013] ; certaines avancées s'excluent [GD-025].
- **La menace visible** : on voit l'armée adverse approcher ; le brouillard cache ce qui est loin, pas ce qui est à la frontière.
- **Le temps compté** : tour limite, compte à rebours, événement de mi-partie.
- **Les contres** : chaque style de jeu a une réponse (triangle des styles, [GD-071]) ; aucune stratégie unique ne gagne toujours.

## 8. Contraintes venues de l'IA (session 3)

Les règles doivent jouer une partie entière sans écran, vite et de façon reproductible (`CLAUDE.md`, point 2). Côté game design, cela pousse vers :

- un combat sans hasard, ou un hasard tiré d'une graine connue ;
- des règles en petit nombre, sans exception cachée ;
- des actions dénombrables à chaque tour (déplacer, attaquer, récolter, bâtir, rechercher, fin de tour) ;
- une partie qui finit toujours (tour limite), pour que l'entraînement ne tourne pas sans fin ;
- des clans décrits par des données (chiffres, listes de capacités) plutôt que par du code à part, pour que l'IA puisse aussi servir à les équilibrer [GD-073].
