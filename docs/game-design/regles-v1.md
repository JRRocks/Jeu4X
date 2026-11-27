# Règles de la première version (V1)

> Écrit par Claude, session S2, le 2026-11-27. **Proposition à valider par l'utilisateur** (D-29) : la session 1 ne code qu'après son « oui ».
> Lecteurs : la session 1 (moteur de règles en Rust, D-14) et la session 3 (IA). Résumé pour l'utilisateur : `regles-v1-resume.md`.
> Sources de ces règles : `docs/TRI.md`, décisions D-23 et D-28, idées gardées du catalogue (`catalogue.md`, `docs/idees-futur.md` pour le reste).
> Tous les chiffres sont des **valeurs de départ**, à régler par parties simulées (GD-073). Ils sont regroupés au § 12 en tables de données que le moteur lira (GD-077, VE-061). Les identifiants de données sont en anglais (D-06).
> Questions encore ouvertes : § 14. Là où une question est ouverte, la règle écrite suit la recommandation.

## 0. Principes

1. **Rien au hasard pendant la partie.** Le hasard ne sert qu'à créer la carte, à partir de la graine : terrain, villages, ruines et contenu des ruines, ordre des joueurs. Après, tout se calcule (D-23, VE-003).
2. **Que des nombres entiers.** Aucun calcul à virgule ; les divisions arrondissent vers le bas (D-14).
3. **Une seule monnaie : les vivres** (nom provisoire, D-23). Elle paie tout.
4. **Même information pour tous.** L'IA ne voit que ce que voit son joueur, brouillard compris (VE-049).
5. **Une action à la fois**, chacune avec son numéro (besoins de S3, E4). Finir son tour est une action.
6. **Toute partie se termine** (§ 9.4).

## 1. Le plateau

### 1.1 Cases et distances
- Plateau carré de `taille × taille` cases. Les adresses A1, B2… existent dans le moteur et ne s'affichent jamais (D-16).
- **Voisines : les 8 cases autour**, diagonales comprises (question GD-Q11). La **distance** entre deux cases est le plus grand des deux écarts, en colonnes ou en rangées. Avec cette distance, une portée ou une vision de 2 couvre un carré de 5 × 5.
- Une case porte au plus **une unité**.

### 1.2 Tailles (D-23)

| Joueurs | Côté du plateau | Villages neutres | Ruines | Seuil de points (§ 9.2) |
|---|---|---|---|---|
| 2 | 11 | 8 | 4 | 45 |
| 3 | 12 | 10 | 5 | 45 |
| 4 | 14 | 14 | 7 | 40 |

Le côté le plus grand permis est **14** (D-23). Le moteur peut garder une grille interne plus grande, mais aucune partie ne la dépasse.

### 1.3 Terrains

| Terrain | Coût pour y entrer | Défense de qui s'y trouve | Vision depuis la case | Exploitation possible | Part de la carte |
|---|---|---|---|---|---|
| Plaine | 1 | +0 | normale | Champ | 45 % |
| Forêt | 2 | +1 | normale | Cueillette | 20 % |
| Colline | 2 | +2 | +1 | Carrière | 12 % |
| Montagne | infranchissable (sauf le Guetteur-planeur, § 7) | +2 | +1 | aucune | 8 % |
| Eau | infranchissable en V1 (pas de bateaux) | — | — | aucune | 15 % |

Montagnes et eau ne bloquent pas la vue.

### 1.4 Ce qu'il y a sur les cases
- **Villes** : la capitale de chaque joueur au départ, puis les villages ralliés et les villes prises.
- **Villages neutres** : à rallier (§ 4.2).
- **Ruines** : un trésor caché, fixé à la création de la carte (§ 3.3).
- **Exploitations** : champ, cueillette, carrière (§ 5.3).

## 2. La création de la carte

Le moteur ne tire au sort qu'ici, avec le générateur rangé dans l'état et initialisé par la graine (D-14).

1. **Terrain** : tiré selon les parts du § 1.3 (l'algorithme de regroupement en forêts, chaînes et lacs est laissé à S1). Le bord du plateau n'est pas forcément de l'eau.
2. **Capitales** : sur une plaine, à une distance de 6 au moins l'une de l'autre (7 au moins sur 14 × 14), et à 2 au moins du bord.
3. **Départs équitables (VE-024)** : le générateur vérifie pour chaque capitale :
   - au moins **2 villages** plus proches d'elle que de toute autre capitale, à une distance de 2 à 4 ;
   - au moins **4 cases franchissables** parmi ses 8 voisines ;
   - le nombre de plaines à distance 2 ou moins, à 2 près de la moyenne des joueurs ;
   - un chemin praticable vers chaque autre capitale.
   Si une condition échoue, le moteur recommence avec une graine dérivée de la première (par exemple `graine + 1`, puis `+ 2`…). Le résultat reste reproductible : même graine, même carte.
4. **Villages** : sur plaine, forêt ou colline, à une distance de 2 au moins de toute ville ou de tout autre village.
5. **Ruines** : sur une case franchissable, à une distance de 3 au moins des capitales. Le contenu de chaque ruine est tiré à ce moment-là (§ 3.3).
6. **Ordre des joueurs** : tiré par la graine. Compensation de qui joue après les autres : +1 vivre de départ par rang (le 2ᵉ joueur reçoit +1, le 3ᵉ +2, le 4ᵉ +3).
7. **Code de carte (VE-040)** : la graine, le nombre de joueurs et les clans forment un code court. Il suffit à recréer exactement la même partie.

## 3. Ce que chacun voit : le brouillard qui se souvient (D-23)

### 3.1 Trois états par case et par joueur

| État | Ce que le joueur en sait |
|---|---|
| **Inconnue** | Rien. |
| **Vue autrefois** | Le terrain, les villes et les villages (avec leur dernier propriétaire connu), les exploitations et les ruines, **tels qu'ils étaient la dernière fois qu'il les a vus**. Aucune unité. |
| **Visible** | Tout, unités comprises, à jour. |

### 3.2 Vision
- Est visible toute case à une distance au plus égale à la **vision** d'une de ses unités ou d'une de ses villes (vision 2 pour une ville ; vision des unités au § 12.3 ; +1 pour qui se tient sur une colline ou une montagne).
- La vision se recalcule après chaque action. Une case qui sort de la vision passe à « vue autrefois ».
- Au départ, les cases autour de la capitale et des unités de départ sont visibles. Tout le reste est inconnu.
- **Informations publiques** : le score de chaque joueur et sa progression vers le seuil (§ 9.2), son clan et ses avancées. **Privées** : son stock de vivres et la position des unités hors de vue.

### 3.3 Ruines
- Une unité qui **entre** dans une ruine la fouille aussitôt. La ruine disparaît, et le joueur gagne **1 point de découverte** (§ 9.1) et le trésor :

| Trésor | Part des ruines | Effet |
|---|---|---|
| Réserve | 40 % | +6 vivres |
| Savoir | 20 % | La moins chère des avancées que le joueur peut prendre, gratuite (à égalité de prix : la première dans l'ordre du § 12.6) |
| Recrue | 20 % | Un Lancier sur la case libre la plus proche de la ruine, même au-delà de la limite d'unités. Il peut agir au tour suivant |
| Carte | 20 % | Toutes les cases à distance 4 ou moins de la ruine deviennent « vues autrefois », ou visibles si elles sont dans la vision |

## 4. Les villes

### 4.1 Niveaux

| Niveau | Coût pour y monter | Revenu de la ville par tour | Unités entretenues | Points tenus |
|---|---|---|---|---|
| 1 | — | 2 | 2 | 2 |
| 2 | 5 | 3 | 3 | 4 |
| 3 | 8 | 4 | 4 | 6 |
| 4 | 11 | 5 | 5 | 8 |
| 5 (maximum) | 14 | 6 | 6 | 10 |

- La **capitale** rapporte **+2 vivres** de plus par tour et entretient **1 unité** de plus.
- **Agrandir une ville** est une action : on paie le coût du niveau suivant, et la ville monte d'un niveau aussitôt.
- **Croissance à choix (GD-013)** : chaque montée de niveau offre deux récompenses. On en choisit une dans la même action, qui existe donc en deux numéros, « agrandir + récompense a » et « agrandir + récompense b ».

| Niveau atteint | Récompense a | Récompense b |
|---|---|---|
| 2 | +1 revenu dans cette ville, pour toujours | Un Lancier gratuit sur la ville (si la case est libre, sinon sur la case libre voisine la plus proche) |
| 3 | Territoire étendu : rayon 2 au lieu de 1 | Remparts gratuits dans cette ville (même sans l'avancée) |
| 4 | +2 revenu dans cette ville | +2 unités entretenues par cette ville |
| 5 | +2 points tenus, en plus | Un Cavalier gratuit (même sans l'avancée) |

### 4.2 Rallier un village, prendre une ville (GD-005)
- **Rallier** : une unité qui se trouve sur un village neutre **depuis le début du tour de son joueur**, et qui n'a encore rien fait, peut rallier ce village. Il devient une ville de niveau 1 de ce joueur. L'unité a fini son tour.
- **Prendre** : même règle sur une ville adverse. La ville change de propriétaire. Elle garde son niveau, ses bâtiments, ses exploitations et son territoire, mais perd ses récompenses de croissance déjà acquises. Les unités de l'ancien propriétaire restent à lui.
- Une ville dont la case est occupée par une unité ennemie ne produit plus d'unités : c'est un siège.

### 4.3 Territoire
- Une ville possède les cases à une distance de 1 autour d'elle (rayon 1), ou de 2 avec la récompense « territoire étendu ».
- Une case déjà à une autre ville le reste. Le territoire ne change de mains qu'avec sa ville.
- Le territoire sert à poser des exploitations (§ 5.3). Il compte aussi pour certaines capacités de clan.

### 4.4 Limite d'unités (GD-011)
- Un joueur entretient au plus la **somme des « unités entretenues »** de ses villes. Au-delà, on ne peut plus produire d'unités.
- Les unités gratuites (récompenses, ruines) s'ajoutent même au-delà de la limite.
- Si la limite baisse, parce qu'on perd une ville, aucune unité ne disparaît : on ne peut simplement plus produire.

## 5. L'économie

### 5.1 Le revenu
Au **début de son tour**, le joueur reçoit la somme des revenus de ses villes : niveau, bonus de capitale, récompenses, exploitations, bâtiments et merveilles. Le revenu du tour suivant s'affiche en permanence (GD-004). Le stock n'a pas de plafond.

### 5.2 Départ

| | Valeur |
|---|---|
| Vivres au départ | 5 (+1 par rang, § 2.6) |
| Ville | la capitale, niveau 1 |
| Unités | un Lancier et une unité d'éclaireur, sur des cases voisines de la capitale (unité d'éclaireur selon le clan : § 7) |
| Avancée | celle de départ de son clan (§ 7) |

### 5.3 Exploitations
- Action « exploiter » : sur une case de son territoire, sans exploitation, sans unité ennemie dessus. Pas besoin d'unité.

| Exploitation | Terrain | Avancée requise | Coût | Revenu par tour, pour la ville du territoire |
|---|---|---|---|---|
| Cueillette | Forêt | aucune | 2 | +1 |
| Champ | Plaine | Champs (Terre 1) | 3 | +1 |
| Carrière | Colline | Carrières (Terre 2) | 5 | +2 |

Une ville prise garde ses exploitations, qui rapportent alors à son nouveau propriétaire.

### 5.4 Bâtiments (un exemplaire de chaque par ville)

| Bâtiment | Avancée requise | Coût | Effet |
|---|---|---|---|
| Remparts | Remparts (Fer 3a) | 6 | La défense de ville passe de +2 à +4 (§ 6.3) |
| Marché | Marchés (Terre 3a) | 6 | +2 revenu |
| Merveille | Avancée de rang 4 de la branche | 15, ville de niveau 3 au moins | § 8.3 |

## 6. Les unités et le combat (sans hasard : D-23)

### 6.1 Le tour d'une unité
- Chaque unité a des **points de mouvement**. Ils se rechargent au début du tour de son joueur.
- **Déplacer** : choisir une case d'arrivée. Le moteur prend le chemin le moins cher. Le chemin et l'arrivée doivent être des cases **connues** (visibles ou vues autrefois), franchissables et libres.
- Entrer dans une case coûte le coût du terrain. Une unité qui n'a pas encore bougé de son tour peut toujours entrer dans une case voisine franchissable, même si cela coûte plus que ses points.
- **Zone de contrôle** : entrer sur une case voisine d'une unité ennemie visible arrête le déplacement.
- **Surprise** : le déplacement se fait case par case. Si une unité ennemie devient visible sur le chemin ou à côté, l'unité s'arrête sur la dernière case atteinte et perd le reste de son mouvement.
- **Attaquer** : une fois par tour, une cible visible à portée (§ 6.2). On peut se déplacer puis attaquer. Attaquer termine le tour de l'unité (sauf le Coureur de crête, § 7).
- Une unité **produite** ce tour ne peut pas agir avant le tour suivant.
- **Repos** : une unité qui n'a ni bougé ni attaqué pendant le tour de son joueur regagne au début du tour suivant **2 points de vie**, ou **4** si elle est dans une de ses villes, sans dépasser son maximum.

### 6.2 Portée
- Une attaque vise une unité à une distance au plus égale à la **portée** de l'attaquant.
- La Catapulte a une **portée minimale de 2** : elle ne peut pas attaquer une case voisine.

### 6.3 Les dégâts
Toutes les valeurs sont entières ; une division arrondit vers le bas.

```
Attaque de l'attaquant   = Attaque de base + bonus du triangle (+2) + bonus de siège + bonus de clan
Défense du défenseur     = Défense de base + bonus du triangle (+2) + terrain de sa case + défense de ville + bonus de clan
Dégâts infligés          = max(1, 4 + Attaque de l'attaquant − Défense du défenseur)

Riposte, si le défenseur survit ET que l'attaquant est à sa portée (distance ≤ portée du défenseur) :
Attaque du défenseur     = Attaque de base du défenseur + bonus du triangle
Défense de l'attaquant   = Défense de base de l'attaquant + bonus du triangle + terrain de la case de l'attaquant
Riposte                  = max(0, (4 + Attaque du défenseur − Défense de l'attaquant) / 2)
```

- **Défense de ville** : +2 pour une unité dans une ville, **+4 avec remparts** (pas de cumul avec le +2). Elle s'ajoute au terrain de la case.
- **Triangle des unités (GD-072)** : le Lancier a +2 face au Cavalier, le Cavalier +2 face à l'Archer, l'Archer +2 face au Lancier. Ce +2 s'applique à l'attaque et à la défense de celui qui a l'avantage, qu'il attaque ou qu'il se défende.
- **Siège** : la Catapulte a +4 d'attaque contre une unité dans une ville.
- Une unité à 0 point de vie ou moins disparaît. L'attaquant **n'avance pas** dans la case libérée. Une riposte peut détruire l'attaquant.
- **Combat annoncé (GD-007)** : avant de confirmer, le joueur voit les dégâts infligés, la riposte, et qui serait détruit. Le moteur fournit ce calcul sans rien changer à l'état.

Exemples (unités du § 12.3, en plaine sauf mention) :

| Attaquant → défenseur | Dégâts | Riposte |
|---|---|---|
| Lancier → Lancier | 4 + 3 − 3 = 4 | (4 + 3 − 3) / 2 = 2 |
| Cavalier → Archer | 4 + 6 − 1 = 9 (l'Archer, 8 PV, est détruit) | — |
| Archer à distance 2 → Lancier | 4 + 5 − 3 = 6 | aucune (hors de portée du Lancier) |
| Lancier → Cavalier | 4 + 5 − 2 = 7 | (4 + 4 − 5) / 2 = 1 |
| Lancier → Lancier en ville avec remparts | max(1, 4 + 3 − 7) = 1 | (4 + 3 − 3) / 2 = 2 |
| Catapulte à distance 2 → Lancier en ville avec remparts | 4 + 9 − 7 = 6 | aucune |

## 7. Les clans (D-23 : avancée de départ et unité propre)

Quatre clans, niveaux d'asymétrie 1 et 2 (GD-070). Chaque clan est une fiche de données (§ 12.7).

| Clan | Avancée de départ (niveau 1) | Unité propre (niveau 2) | Unité d'éclaireur au départ |
|---|---|---|---|
| **Crocs-Rouges** (attaque) | **Razzia** : +2 vivres chaque fois qu'une de leurs unités détruit une unité ennemie | **Coureur de crête** : après avoir attaqué, peut encore dépenser les points de mouvement qui lui restent | Coureur de crête |
| **Murailliers** (défense) | **Pierres levées** : la capitale commence avec des remparts ; ils peuvent bâtir des remparts sans l'avancée Remparts, pour 3 vivres au lieu de 6 | **Porte-pavois** : les unités amies voisines ont +1 en défense (sans cumul entre plusieurs porte-pavois) | Éclaireur |
| **Veilleurs d'Astres** (savoir) | **Lecture du ciel** : leurs avancées coûtent 4/5 du prix, arrondi vers le bas (5 → 4, 8 → 6, 11 → 8, 14 → 11) | **Astronome** : ne peut pas attaquer. Action « lunette », une fois par tour : toutes les cases à distance 1 ou moins d'une case choisie à distance 4 ou moins deviennent visibles jusqu'à la fin du tour | Éclaireur |
| **Vents-Errants** (exploration) | **Pistes du vent** : +1 vision pour toutes leurs unités ; le trésor « Réserve » d'une ruine leur rapporte 12 vivres au lieu de 6 | **Guetteur-planeur** : peut entrer sur les montagnes et s'y arrêter (coût 2) | Guetteur-planeur |

Leur place dans le triangle des styles (GD-071) : Crocs-Rouges = attaque ; Murailliers = défense ; Veilleurs d'Astres et Vents-Errants = économie et expansion, par deux chemins différents.

## 8. L'arbre d'évolution (D-23 : trois branches sur un écran)

### 8.1 Forme
L'esquisse de `arbre-evolution.md` est précisée ici. Il y a **16 avancées à l'écran** : l'avancée de départ du clan, plus trois branches de 5. Chaque branche suit le même schéma : rang 1, puis rang 2, puis au rang 3 un **choix exclusif** entre deux avancées (en prendre une ferme l'autre pour la partie, GD-025), puis au rang 4 la **merveille** (GD-027). Un joueur possède au plus 13 avancées : son départ, plus 4 par branche.

```
                    [ Avancée de départ du clan ]
    FER (force)              TERRE (croissance)        SOUFFLE (savoir et mouvement)
 1  Arcs                     Champs                    Sentiers
 2  Dressage                 Carrières                 Mécanique
 3  Remparts ⟷ Forge         Marchés ⟷ Greniers        Cartographes ⟷ Écoles
 4  L'Arène des Crêtes       Le Grand Verger           La Tour des Brumes
```

### 8.2 Règles
- **Rechercher** est une action : payer le prix, et l'avancée est acquise aussitôt.
- Il faut le rang précédent de la même branche ; le rang 4 demande l'une des deux avancées du rang 3.
- Prix par rang : **5, 8, 11, 14**. Un prix fixe, qui ne dépend ni du nombre de villes ni des avancées déjà prises (la montée des coûts, GD-024, est en « plus tard »).
- Sans avancée, chacun a déjà : Éclaireur, Lancier, son unité propre, la cueillette et l'agrandissement des villes.

### 8.3 Les avancées

| Branche | Rang | Avancée | Débloque ou donne |
|---|---|---|---|
| Fer | 1 | Arcs | Unité Archer |
| Fer | 2 | Dressage | Unité Cavalier |
| Fer | 3a | Remparts | Bâtiment Remparts |
| Fer | 3b | Forge | +1 attaque pour toutes ses unités qui attaquent au contact (portée 1) |
| Fer | 4 | L'Arène des Crêtes | Droit de bâtir la merveille : +2 points de vie maximum pour toutes ses unités |
| Terre | 1 | Champs | Exploitation Champ |
| Terre | 2 | Carrières | Exploitation Carrière |
| Terre | 3a | Marchés | Bâtiment Marché |
| Terre | 3b | Greniers | Agrandir une ville coûte 2 vivres de moins |
| Terre | 4 | Le Grand Verger | Droit de bâtir la merveille : +4 revenu dans sa ville |
| Souffle | 1 | Sentiers | Entrer en forêt ne coûte plus que 1 |
| Souffle | 2 | Mécanique | Unité Catapulte |
| Souffle | 3a | Cartographes | +1 vision pour toutes ses unités et ses villes |
| Souffle | 3b | Écoles | Les avancées suivantes coûtent 2 vivres de moins |
| Souffle | 4 | La Tour des Brumes | Droit de bâtir la merveille : toute la carte devient au moins « vue autrefois » au moment où elle est bâtie ; +2 vision pour sa ville |

- **Merveilles** : une seule de chaque dans la partie. Le premier qui la bâtit l'a. Les autres gardent l'avancée et son point de score, mais ne peuvent plus bâtir cette merveille. Une merveille rapporte **6 points tenus** à qui possède sa ville : elle change de mains avec la ville.

## 9. Gagner (D-28)

### 9.1 Les points
Le score est **public** et se recalcule après chaque action.

| Source | Points | Type |
|---|---|---|
| Chaque ville | ses points tenus (§ 4.1 : 2 par niveau) | **tenus** : perdus si la ville est perdue |
| Chaque merveille possédée | 6 | tenus |
| Chaque avancée acquise (sauf celle de départ) | 1 | **acquis** : gardés pour toujours |
| Chaque unité ennemie détruite | 1 | acquis |
| Chaque ruine fouillée | 1 | acquis |

### 9.2 Les deux victoires
1. **La capitale** : prendre la capitale adverse (§ 4.2). À deux joueurs, c'est une **victoire immédiate**. À trois ou quatre, la règle recommandée est la suivante (question GD-Q10) : le joueur qui perd sa capitale est **éliminé**. Ses autres villes redeviennent des villages neutres et ses unités disparaissent. Le dernier joueur restant gagne.
2. **Le seuil de points** (§ 1.2 : 45 à deux et trois joueurs, 40 à quatre) :
   - à la **fin** de son tour, si un joueur a au moins le seuil, il est **annoncé** à tous (« Rouge a atteint le seuil ») ;
   - au **début** de son tour suivant, s'il a toujours au moins le seuil, il **gagne**. Les autres ont donc un tour pour le faire redescendre, en lui prenant des villes (question GD-Q12).
   - Si plusieurs joueurs sont annoncés, le premier dont le tour commence gagne.

### 9.3 Élimination
Un joueur qui n'a plus aucune ville est éliminé (cela vaut aussi en cas de règle GD-Q10 b ou c).

### 9.4 Fin de sécurité
Pour que toute partie se termine (CLAUDE.md, point 2 ; besoins de S3), une **limite de sécurité** est réglée par défaut au **tour 80**. Si personne n'a gagné à ce tour-là, la partie est **nulle**, sans vainqueur. Ce n'est pas une fin au score : D-28 est respecté. Le compte des tours restants ne s'affiche qu'à partir du tour 70 (question GD-Q9). Un tour est un passage complet de tous les joueurs.

## 10. Ce qui est accordé avec D-28

| Idée gardée | Ce qu'elle disait | Comment elle s'accorde |
|---|---|---|
| **VE-015** Parties à nombre de tours fixe | Meilleur score en un nombre de tours connu | **Pas de mode à tours fixes en V1** : D-28 l'emporte. La partie se joue jusqu'à la capitale ou au seuil. Le tour 80 n'est qu'une sécurité (§ 9.4). Le défi du jour reprend l'idée autrement (ci-dessous). |
| **GD-010** Tours restants et objectifs affichés | Compte à rebours des tours | Il devient un **compte à rebours de points** : chaque joueur voit le score de tous et ce qu'il leur manque pour le seuil, plus l'annonce d'un joueur qui l'a atteint (§ 9.2). Les tours restants ne s'affichent qu'à l'approche de la limite de sécurité. |
| **GD-090** Socle V1 | Victoire au score au trentième tour ou par conquête | Remplacé sur ce point : capitale ou seuil. Le reste du socle tient : carte 11 × 11, quatre clans, arbre en trois branches, une monnaie. Les seuils sont réglés pour qu'une partie dure **25 à 35 tours** environ, à vérifier par simulation. |
| **VE-021** Fin de partie plus courte | Limite de tours, victoire aux points ou abandon automatique | Victoire aux points : oui (le seuil). Limite : la sécurité seulement. Abandon : un joueur sans ville est éliminé (§ 9.3). Un abandon automatique d'un camp très en retard est laissé pour plus tard. |
| **VE-009** Défi du jour ou de la semaine | Même partie pour tous, comparer son score | Même carte (même code) pour tous. Le résultat à comparer est **le nombre de tours pour gagner** : moins, c'est mieux. À égalité, le score au moment de la victoire départage (question GD-Q13). |

## 11. Les autres idées gardées, en règles

- **Annuler (VE-004)** : un joueur peut annuler sa dernière action tant qu'elle **n'a rien révélé** : pas de case nouvellement vue, pas de ruine fouillée, pas d'unité ennemie découverte. Le moteur marque chaque action « annulable » ou non, et ne garde l'historique que depuis la dernière action non annulable. Une fois par partie, le joueur peut aussi **remettre son tour à zéro**, c'est-à-dire revenir au début de son tour. Ce qu'il a vu entre-temps reste connu de lui. Les deux sont refusés à l'IA.
- **Plusieurs joueurs sur un téléphone (VE-011)** : chacun a sa propre vue (§ 3). L'écran de passage de main relève de l'affichage, pas des règles. Les places vides sont tenues par l'IA.
- **Replays et code de carte (VE-040, VE-041)** : la graine, les réglages et la suite des actions suffisent à rejouer la partie (format de S3, E9). L'annulation n'entre pas dans le replay : seules les actions gardées y figurent.
- **IA à égalité (VE-046, VE-049)** : aucun niveau d'IA ne reçoit de bonus. Si un jour un niveau en reçoit, il sera affiché (VE-047).

## 12. Les tables de données

Ce sont les valeurs que le moteur lit. S1 choisit le format de fichier (D-14). Les identifiants sont en anglais (D-06) et les noms affichés vont dans les ressources de traduction.

### 12.1 Réglages de partie (`game_settings`)

| Clé | 2 joueurs | 3 joueurs | 4 joueurs |
|---|---|---|---|
| `board_size` | 11 | 12 | 14 |
| `village_count` | 8 | 10 | 14 |
| `ruin_count` | 4 | 5 | 7 |
| `points_threshold` | 45 | 45 | 40 |
| `capital_min_distance` | 6 | 6 | 7 |
| `safety_turn_limit` | 80 | 80 | 80 |
| `safety_countdown_from` | 70 | 70 | 70 |
| `start_food` | 5 | 5 | 5 |
| `start_food_per_rank` | 1 | 1 | 1 |

### 12.2 Terrains (`terrains`)

| `id` | `move_cost` (0 = infranchissable) | `defense_bonus` | `vision_bonus` | `improvement` | `share_percent` |
|---|---|---|---|---|---|
| `plain` | 1 | 0 | 0 | `field` | 45 |
| `forest` | 2 | 1 | 0 | `gathering` | 20 |
| `hill` | 2 | 2 | 1 | `quarry` | 12 |
| `mountain` | 0 | 2 | 1 | — | 8 |
| `water` | 0 | — | — | — | 15 |

### 12.3 Unités (`units`)

| `id` | Nom affiché | `cost` | `hp` | `attack` | `defense` | `move` | `range` | `min_range` | `vision` | `requires` | `class` | `abilities` |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `scout` | Éclaireur | 3 | 6 | 1 | 1 | 3 | 1 | 1 | 3 | — | `none` | — |
| `spearman` | Lancier | 4 | 10 | 3 | 3 | 1 | 1 | 1 | 2 | — | `spear` | — |
| `archer` | Archer | 5 | 8 | 3 | 1 | 1 | 2 | 1 | 2 | `arcs` | `bow` | — |
| `rider` | Cavalier | 6 | 10 | 4 | 2 | 2 | 1 | 1 | 2 | `dressage` | `horse` | — |
| `catapult` | Catapulte | 8 | 6 | 5 | 0 | 1 | 3 | 2 | 1 | `mechanics` | `siege` | `siege_bonus:4` |
| `ridge_runner` | Coureur de crête | 4 | 8 | 3 | 1 | 2 | 1 | 1 | 2 | clan `red_fangs` | `none` | `move_after_attack` |
| `shield_bearer` | Porte-pavois | 5 | 12 | 1 | 4 | 1 | 1 | 1 | 2 | clan `wallers` | `none` | `guard_adjacent:1` |
| `astronomer` | Astronome | 4 | 6 | 0 | 1 | 2 | 0 | 0 | 3 | clan `star_watchers` | `none` | `no_attack`, `spyglass:4:1` |
| `glider_scout` | Guetteur-planeur | 5 | 6 | 2 | 1 | 3 | 1 | 1 | 3 | clan `wind_wanderers` | `none` | `enter_mountain:2` |

Triangle (`class_bonus`, +2 à l'attaque et à la défense) : `spear` sur `horse`, `horse` sur `bow`, `bow` sur `spear`.

### 12.4 Niveaux de ville (`city_levels`)

| `level` | `grow_cost` | `income` | `unit_capacity` | `held_points` | Récompense a | Récompense b |
|---|---|---|---|---|---|---|
| 1 | — | 2 | 2 | 2 | — | — |
| 2 | 5 | 3 | 3 | 4 | `income:+1` | `free_unit:spearman` |
| 3 | 8 | 4 | 4 | 6 | `territory_radius:2` | `free_building:walls` |
| 4 | 11 | 5 | 5 | 8 | `income:+2` | `unit_capacity:+2` |
| 5 | 14 | 6 | 6 | 10 | `held_points:+2` | `free_unit:rider` |

Bonus de capitale : `capital_income_bonus = 2`, `capital_capacity_bonus = 1`. Rayon de territoire de base : `territory_radius = 1`. Vision de ville : `city_vision = 2`. Défense de ville : `city_defense = 2`, `walls_city_defense = 4`. Soins : `rest_heal = 2`, `rest_heal_in_city = 4`.

### 12.5 Exploitations et bâtiments (`improvements`, `buildings`)

| `id` | Type | Terrain ou condition | `requires` | `cost` | Effet |
|---|---|---|---|---|---|
| `gathering` | exploitation | `forest` | — | 2 | `income:+1` |
| `field` | exploitation | `plain` | `fields` | 3 | `income:+1` |
| `quarry` | exploitation | `hill` | `quarries` | 5 | `income:+2` |
| `walls` | bâtiment | ville | `walls_tech` | 6 | `city_defense:4` |
| `market` | bâtiment | ville | `markets` | 6 | `income:+2` |
| `wonder_arena` | merveille | ville de niveau 3 ou plus | `arena` | 15 | `unit_max_hp:+2`, `held_points:+6` |
| `wonder_orchard` | merveille | ville de niveau 3 ou plus | `great_orchard` | 15 | `income:+4`, `held_points:+6` |
| `wonder_tower` | merveille | ville de niveau 3 ou plus | `mist_tower` | 15 | `reveal_map`, `city_vision:+2`, `held_points:+6` |

### 12.6 Avancées (`techs`), dans l'ordre de référence

| `id` | Nom affiché | `branch` | `tier` | `cost` | `requires` | `excludes` | Effet |
|---|---|---|---|---|---|---|---|
| `arcs` | Arcs | `iron` | 1 | 5 | — | — | `unlock_unit:archer` |
| `dressage` | Dressage | `iron` | 2 | 8 | `arcs` | — | `unlock_unit:rider` |
| `walls_tech` | Remparts | `iron` | 3 | 11 | `dressage` | `forge` | `unlock_building:walls` |
| `forge` | Forge | `iron` | 3 | 11 | `dressage` | `walls_tech` | `melee_attack:+1` |
| `arena` | L'Arène des Crêtes | `iron` | 4 | 14 | `walls_tech` ou `forge` | — | `unlock_wonder:wonder_arena` |
| `fields` | Champs | `earth` | 1 | 5 | — | — | `unlock_improvement:field` |
| `quarries` | Carrières | `earth` | 2 | 8 | `fields` | — | `unlock_improvement:quarry` |
| `markets` | Marchés | `earth` | 3 | 11 | `quarries` | `granaries` | `unlock_building:market` |
| `granaries` | Greniers | `earth` | 3 | 11 | `quarries` | `markets` | `grow_cost:-2` |
| `great_orchard` | Le Grand Verger | `earth` | 4 | 14 | `markets` ou `granaries` | — | `unlock_wonder:wonder_orchard` |
| `trails` | Sentiers | `breath` | 1 | 5 | — | — | `forest_move_cost:1` |
| `mechanics` | Mécanique | `breath` | 2 | 8 | `trails` | — | `unlock_unit:catapult` |
| `cartographers` | Cartographes | `breath` | 3 | 11 | `mechanics` | `schools` | `vision:+1` (unités et villes) |
| `schools` | Écoles | `breath` | 3 | 11 | `mechanics` | `cartographers` | `tech_cost:-2` |
| `mist_tower` | La Tour des Brumes | `breath` | 4 | 14 | `cartographers` ou `schools` | — | `unlock_wonder:wonder_tower` |

Prix minimal d'une avancée après réductions : 1.

### 12.7 Clans (`clans`)

| `id` | Nom affiché | `start_tech` | Effets de `start_tech` | `unique_unit` | `start_units` |
|---|---|---|---|---|---|
| `red_fangs` | Crocs-Rouges | `raid` | `food_per_kill:2` | `ridge_runner` | `spearman`, `ridge_runner` |
| `wallers` | Murailliers | `standing_stones` | `capital_starts_with:walls`, `walls_without_tech`, `walls_cost:3` | `shield_bearer` | `spearman`, `scout` |
| `star_watchers` | Veilleurs d'Astres | `sky_reading` | `tech_cost_ratio:4/5` | `astronomer` | `spearman`, `scout` |
| `wind_wanderers` | Vents-Errants | `wind_trails` | `vision:+1` (unités), `ruin_food:12` | `glider_scout` | `spearman`, `glider_scout` |

### 12.8 Points (`score`) et ruines (`ruins`)

| Clé | Valeur |
|---|---|
| `points_per_city_level` | 2 (déjà dans `held_points`) |
| `points_per_wonder` | 6 |
| `points_per_tech` | 1 (avancée de départ non comptée) |
| `points_per_kill` | 1 |
| `points_per_ruin` | 1 |

| Trésor (`ruin_reward`) | `weight` | Effet |
|---|---|---|
| `food` | 40 | `food:+6` |
| `tech` | 20 | `free_cheapest_tech` |
| `recruit` | 20 | `free_unit:spearman` |
| `map` | 20 | `reveal_radius:4` |

## 13. Les actions (pour l'espace d'actions de S3)

| Action | Paramètres | Conditions principales |
|---|---|---|
| Finir le tour | — | toujours |
| Déplacer | unité (case de départ), case d'arrivée | § 6.1 |
| Attaquer | unité, case cible | cible visible, à portée, l'unité n'a pas encore attaqué |
| Rallier ou prendre | unité | § 4.2 |
| Lunette (Astronome) | unité, case visée | § 7 |
| Renvoyer une unité | unité | elle disparaît, sans remboursement |
| Produire une unité | ville, type d'unité | case de la ville libre, limite d'unités, une production par ville et par tour, vivres suffisants |
| Agrandir une ville | ville, récompense a ou b | niveau inférieur à 5, vivres suffisants |
| Exploiter une case | case | § 5.3 |
| Bâtir | ville, bâtiment ou merveille | § 5.4 |
| Rechercher | avancée | § 8.2 |
| Annuler, remettre le tour à zéro | — | joueurs humains seulement, hors replay (§ 11) |

Avec les déplacements vers n'importe quelle case d'arrivée, l'espace « case de départ × case d'arrivée » déjà prévu par S1 couvre déplacer, attaquer et lunette, à condition de distinguer le type d'action. Les autres actions s'ajoutent sur des numéros fixes : ville × unité, ville × récompense, case × exploitation, ville × bâtiment, avancée.

## 14. Cas limites

1. **Deux unités sur une case** : jamais. Une unité produite ou gratuite va sur la ville, sinon sur la case libre franchissable la plus proche, la plus basse dans l'ordre des cases à égalité ; s'il n'y en a aucune, rien n'est créé (la récompense est perdue).
2. **Ville assiégée** (unité ennemie sur sa case) : pas de production. Elle rapporte toujours son revenu à son propriétaire jusqu'à sa prise.
3. **Capitale prise à deux joueurs** : victoire immédiate, même si le perdant avait atteint le seuil.
4. **Seuil atteint, puis perdu** : l'annonce tombe ; aucune victoire.
5. **Merveille commencée par deux joueurs** : le bâtiment se paie et s'achève dans la même action, il n'y a donc pas de « commencée ». Le premier à agir l'a.
6. **Ville prise qui contient une merveille** : la merveille et ses 6 points passent au preneur. L'effet « carte révélée » de la Tour ne se rejoue pas ; seule la vision +2 suit la ville.
7. **Remparts et récompense « remparts gratuits »** : si la ville a déjà des remparts, la récompense b du niveau 3 ne peut pas être choisie.
8. **Avancée gratuite d'une ruine** quand tout est pris : le trésor devient « Réserve ».
9. **Unité entrant sur une ruine pendant un déplacement plus long** : elle s'y arrête.
10. **Coureur de crête** : après son attaque, il peut se déplacer, mais pas attaquer de nouveau. La zone de contrôle s'applique.
11. **Astronome attaqué** (ou toute unité marquée `no_attack`) : il se défend normalement mais ne riposte jamais.
12. **Joueur éliminé à trois ou quatre** : ses villes redeviennent des villages neutres, que l'on peut rallier. Les exploitations y restent. Les bâtiments et merveilles sont détruits ; une merveille détruite ne peut plus être bâtie.
13. **Limite de sécurité atteinte** pendant qu'un joueur est annoncé au seuil : la partie est nulle, sauf si le tour de ce joueur commence avant la fin du tour 80.
14. **Annulation après un combat** : un combat ne révèle rien de caché, puisque la cible était visible et le résultat annoncé. Il reste annulable, sauf s'il a détruit une unité dont la disparition fait voir de nouvelles cases.

## 15. Questions pour l'utilisateur (ma recommandation en premier)

- **GD-Q9 · Une partie que personne ne gagne doit-elle s'arrêter un jour ?**
  a) Oui, partie nulle au tour 80, avec le compte à rebours affiché à partir du tour 70 (recommandé : toute partie finit, ce que l'entraînement de l'IA exige, sans fin au score) ;
  b) au tour 80, le meilleur score gagne (c'est une fin au score, contraire à D-28) ;
  c) aucune limite (une partie bloquée pourrait ne jamais finir).
- **GD-Q10 · À trois ou quatre, que se passe-t-il quand on prend une capitale ?**
  a) Son propriétaire est éliminé, ses villes redeviennent des villages, le dernier debout gagne (recommandé) ;
  b) celui qui prend une capitale gagne aussitôt la partie ;
  c) on n'est éliminé que sans aucune ville, la capitale n'étant qu'une ville comme une autre.
- **GD-Q11 · Comment les unités se déplacent-elles ?**
  a) Dans les 8 directions, diagonales comprises (recommandé : distances simples, portées et vision en carrés lisibles en perspective) ;
  b) dans 4 directions seulement, comme le moteur d'essai actuel.
- **GD-Q12 · Quand gagne-t-on par les points ?**
  a) Au début de son tour suivant, si l'on a toujours le seuil : les autres ont un tour pour réagir (recommandé : fin tendue, qui remplace le compte à rebours de tours) ;
  b) dès que l'on atteint le seuil.
- **GD-Q13 · Le défi du jour, sans nombre de tours fixe, se compare comment ?**
  a) Au nombre de tours qu'il a fallu pour gagner, le score départageant les égalités (recommandé) ;
  b) le défi du jour seul se joue en tours fixes, exception à D-28 ;
  c) le défi attend une version suivante.
- **GD-Q14 · Comment une ville grandit-elle ?**
  a) On paie des vivres pour l'agrandir, et chaque niveau offre un choix entre deux récompenses (recommandé : une décision claire, une seule monnaie) ;
  b) elle grandit seule, selon les exploitations de son territoire.

## 16. À faire après validation

- S1 : ajuster le moteur d'essai (4 joueurs, plateau de 14 au plus, 8 directions si GD-Q11 a, terrains du § 1.3), puis coder ces règles et les tables du § 12.
- S3 : premières parties simulées pour régler les seuils (durée visée : 25 à 35 tours) et vérifier l'équilibre des quatre clans (aucun au-delà de 55 % de victoires en moyenne, GD-073).
- S2 : reprendre les chiffres d'après ces mesures.
