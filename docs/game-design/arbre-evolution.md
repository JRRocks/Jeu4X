# L'arbre d'évolution

> Session 2, recherche de game design. Rien n'est décidé : ce document propose, l'utilisateur trie (D-07).
> Noms de travail, tous inventés ici (D-03). Fiches dans `catalogue.md`.

## 1. Les formes possibles

| Forme | Principe | Pour | Contre | Sur téléphone |
|---|---|---|---|---|
| **Arbre classique** | Des avancées reliées par des prérequis | Connu, lisible | Chemin optimal vite trouvé, parties qui se ressemblent | Bien si 15-25 nœuds |
| **Réseau** | Une avancée a plusieurs chemins d'accès | Plus de liberté | Plus dur à dessiner | Moyen |
| **Branches par thème** | 3 à 5 branches courtes (guerre, terre, mer, savoir…) | Un style de jeu = une branche, très lisible | Peu de croisements | Très bien : une branche = une colonne |
| **Ères** | On passe d'une ère à l'autre ; chaque ère ouvre de nouvelles avancées | Rythme fort, la partie change de visage | Risque de « mur » entre ères | Bien |
| **Cartes tirées** | À chaque palier, on tire 3 avancées et on en garde une | Chaque partie diffère, choix forcé | Hasard (à fixer par graine pour l'IA) | Très bien : 3 cartes à l'écran |
| **Arbre propre à chaque clan** | Chaque clan a ses avancées | Asymétrie maximale | Beaucoup à concevoir et équilibrer | Bien, mais coûteux |
| **Déblocage par hauts faits** | Une avancée s'obtient en accomplissant un acte (gagner 3 combats, pêcher 5 fois…) | Récompense le style de jeu, peu de menus | Moins de contrôle, plus dur à prévoir | Très bien : aucune dépense |
| **Savoirs partagés** | Les avancées découvertes par un joueur deviennent moins chères pour les autres | Rattrapage naturel | Moins de récompense pour le pionnier | Invisible, rien à afficher |

## 2. Taille, monnaie, coûts, rythme

- **Taille** : 15 à 25 avancées en tout. Au-delà, l'écran ne suffit plus et le joueur ne retient plus ce qu'il a. Un joueur doit pouvoir en prendre **la moitié à peu près** dans une partie de 30 tours : il choisit, il ne collectionne pas tout.
- **Monnaie** : deux options.
  - *Une seule monnaie* (provisoirement appelée **« vivres »**, nom à trouver) qui paie tout : unités, bâtiments, avancées. Simple, et l'arbitrage « une avancée ou une unité ? » est une décision intéressante. Recommandé pour la V1.
  - *Deux monnaies* : vivres pour produire, **savoir** pour les avancées. Plus de profondeur, permet un clan de recherche plus net, mais un compteur de plus.
- **Coûts** : trois façons de monter les prix, à comparer par parties simulées [GD-024].
  - par **palier** (palier 1 = 5, palier 2 = 8, palier 3 = 12) : simple et prévisible ;
  - par **nombre de villes** : freine celui qui s'étend vite (c'est le choix de la référence de genre) ;
  - par **nombre d'avancées déjà prises** : chaque avancée coûte un peu plus que la précédente, ce qui pousse à choisir.
- **Ce qu'une avancée débloque** : une seule chose nette, parmi : une unité, un bâtiment, une action sur une case (récolter, défricher, pêcher), un bonus permanent simple, une capacité d'unité. Jamais « +5 % ».
- **Rythme visé** : une avancée tous les 2 tours au début, tous les 3 à 4 tours à la fin.

## 3. Trois arbres originaux, en esquisse

### Arbre A : « Les Trois Veines » (branches par thème) [GD-020]

Trois branches de 5 avancées, plus un nœud de départ propre au clan. 16 nœuds. Une branche correspond à un style de jeu ; chaque branche a, au palier 3, **un choix exclusif** (on prend l'une des deux, l'autre se ferme) [GD-025].

```
                    [Départ du clan]  (offert, différent pour chaque clan)
            ┌──────────────┼──────────────┐
         FER (force)    TERRE (croissance)   SOUFFLE (savoir et mouvement)
   P1   Taille-pierre     Cueillette          Pistes
   P2   Lances            Champs              Radeaux
   P3   Remparts  ⟷  Cavaliers   Greniers ⟷ Pâtures    Voiles ⟷ Astres
   P4   Forges            Marchés             Cartographes
   P5   Grand ouvrage de guerre  Grand ouvrage de terre  Grand ouvrage de savoir
```

- Coûts : palier 1 = 4, palier 2 = 7, palier 3 = 10, palier 4 = 14, palier 5 = 20 (à simuler).
- Le palier 5 de chaque branche est un **grand ouvrage** (merveille) qui rapporte des points de score et un effet fort ; un seul joueur peut bâtir chacun.
- Lisible : trois colonnes à l'écran, on voit tout l'arbre sans défiler.
- C'est l'arbre recommandé pour la première version.

### Arbre B : « Les Quatre Lunes » (ères et cartes tirées) [GD-021]

La partie traverse quatre ères (première lune, deuxième lune…). À chaque ère, on tire **5 avancées** dans le paquet de cette ère et on les pose face visible ; on en prend autant qu'on peut payer. On passe à l'ère suivante quand on en a pris **3** (ou au tour 8, 16 et 24 au plus tard, pour personne en retard de deux ères).

```
Ère 1 (paquet de 8) : outils, chasse, pêche, sentiers, palissade, cueillette, poterie, dressage
Ère 2 (paquet de 8) : forge, voile, champs, marché, archers, routes, chants, tours de guet
Ère 3 (paquet de 8) : cavaliers, catapultes, ports, bibliothèque, temples, canaux, mines, espions
Ère 4 (paquet de 6) : grands ouvrages et unités d'élite
```

- Chaque partie a une couleur différente selon les cartes tirées (graine fixée : même graine, même tirage, pour l'IA).
- Le passage d'ère est un événement visible : il relance la partie à mi-chemin et répond au défaut de fin de partie répétitive.
- Variante : les cartes non prises par un joueur passent au voisin (comme un tour de table) ; trop complexe pour la V1.

### Arbre C : « L'Arbre Vivant » (hauts faits, propre au clan) [GD-022]

Pas de dépense : l'arbre **pousse tout seul** quand le joueur accomplit des actes. Chaque clan a son arbre de 9 nœuds en losange, dont les conditions reflètent son style.

```
                  [Racine du clan]
            ┌────────┴────────┐
      [Fait 1 : 2 villages]  [Fait 2 : 1 ruine fouillée]
        ┌───┴───┐               ┌───┴───┐
    [Fait 3]  [Fait 4]       [Fait 5]  [Fait 6]
        └───┬───┘               └───┬───┘
       [Fait 7]                 [Fait 8]
            └────────┬────────┘
               [Fruit du clan] (avancée ultime, propre au clan)
```

Exemples de hauts faits : « Gagner 3 combats » (Crocs-Rouges), « Tenir une ville assiégée 2 tours » (Murailliers), « Voir 60 % de la carte » (Vents-Errants), « Avoir 4 routes commerciales » (Guilde des Gués).

- Récompense le joueur qui joue le style de son clan, sans menu de recherche.
- Combinable avec l'arbre A : l'arbre A est commun, l'arbre C est un petit bonus propre à chaque clan.
- Coût de conception élevé : un arbre par clan. Plutôt pour une version ultérieure.

### Variante transverse : « Le Puits des Savoirs » [GD-023]

S'ajoute à n'importe quel arbre : chaque avancée déjà connue d'un autre joueur que l'on **a rencontré** coûte 25 % de moins. Les idées circulent entre voisins, le retardataire rattrape, l'isolé (archipel) avance seul.

## 4. Recommandation

Pour la première version : **arbre A** (16 nœuds, une monnaie, coûts par palier), avec un **nœud de départ propre à chaque clan** [GD-026]. Les arbres B et C se gardent pour plus tard ; les parties simulées de la session 3 pourront dire lequel donne les parties les plus variées.
