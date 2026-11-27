# Catalogue de game design (GD-)

> Session 2, le 2026-11-27. Recherche seulement : l'utilisateur trie, seul le retenu est implémenté (D-07).
> Trier d'un mot : « garde GD-020 », « jette GD-054 », « plus tard GD-021 ».
> Documents de fond : `boucle-de-jeu.md`, `arbre-evolution.md`, `clans.md`. Sources : `sources.md`.
> Coût : **S** petit (quelques règles), **M** moyen (une mécanique avec son interface), **L** grand (système entier, équilibrage long).
> ⭐ = recommandé par la session 2 pour la première version.

## Tableau résumé

### Boucle de jeu

| N° | Titre | L'idée en une ligne | Intérêt | Coût |
|---|---|---|---|---|
| GD-001 ⭐ | Limite de tours et score | La partie finit au tour 20, 30 ou 40 ; le meilleur score gagne | Parties courtes garanties, IA qui termine toujours | S |
| GD-002 ⭐ | Trois familles de victoire | Domination, score, victoire propre au clan | Plusieurs façons de gagner, styles variés | M |
| GD-003 | Formats et formes de carte | Duel 9×9 à grand monde 18×18 ; continent, archipel, anneau… | Variété, durée réglable | M |
| GD-004 ⭐ | Une seule monnaie | Une ressource qui paie tout, avec le revenu du tour affiché | Lisibilité, arbitrages nets | S |
| GD-005 ⭐ | Villages neutres à rallier | On s'étend en prenant des villages, pas en escortant des colons | Expansion rapide et disputée | S |
| GD-006 ⭐ | Brume et lieux remarquables | Brouillard de guerre, ruines et sommets qui récompensent l'exploration | Le premier X a du sens | S |
| GD-007 ⭐ | Combat sans hasard, prévisualisé | Avant d'attaquer, on voit les dégâts donnés et reçus | Lisible, reproductible, bon pour l'IA | S |
| GD-008 | Rattrapage doux | Le retardataire paie moins les avancées déjà connues | Moins de parties jouées d'avance | S |
| GD-009 | Événement de mi-partie | La mer monte, une migration de bêtes, une éclipse… | Relance la fin de partie | M |
| GD-010 ⭐ | Compte à rebours et objectifs visibles | Tours restants et progrès de chacun affichés | Tension de fin de partie | S |
| GD-011 | Plafond d'unités par ville | Autant d'unités que la taille de la ville | Peu de pions, tours rapides | S |
| GD-012 | Bêtes neutres | Des bêtes gardent les ruines ; les vaincre rapporte | Danger précoce, cibles hors joueurs | M |
| GD-013 ⭐ | Croissance à choix | Chaque niveau de ville offre deux récompenses, on en prend une | Une décision intéressante à chaque croissance | S |
| GD-014 | Tours rapides | Un geste par action, fin de tour qui signale les unités oubliées | Fluidité sur téléphone | S |
| GD-015 ⭐ | Carte maximale et nombre de joueurs | Recommandé : 14×14 au plus, 2 à 4 joueurs (V1 en 11×11) ; options 18×18 à 6, ou duel seul | Fixe la taille de l'IA (S3) | S |
| GD-016 ⭐ | Brouillard de guerre ou non | Recommandé : brume avec mémoire ; options sans brume, terrain caché seulement, au choix | Change l'architecture de l'IA (S3) | M |
| GD-017 ⭐ | Combats avec ou sans hasard | Recommandé : sans hasard ; options petit hasard borné, hasard franc | Change l'architecture de l'IA (S3) | S |

### Arbre d'évolution

| N° | Titre | L'idée en une ligne | Intérêt | Coût |
|---|---|---|---|---|
| GD-020 ⭐ | Arbre A « Les Trois Veines » | 3 branches de 5 (Fer, Terre, Souffle) avec un choix exclusif chacune | Lisible, tient sur un écran | M |
| GD-021 | Arbre B « Les Quatre Lunes » | Quatre ères, 5 avancées tirées par ère, on en prend 3 pour passer | Chaque partie diffère | M |
| GD-022 | Arbre C « L'Arbre Vivant » | Arbre propre au clan qui pousse par hauts faits, sans dépense | Récompense le style du clan | L |
| GD-023 | Le Puits des Savoirs | Une avancée connue d'un voisin rencontré coûte 25 % de moins | Rattrapage, contacts utiles | S |
| GD-024 | Montée des coûts | Comparer coût par palier, par ville et par avancée prise | Rythme réglé par simulation | S |
| GD-025 ⭐ | Avancées exclusives | À certains paliers, en prendre une ferme l'autre | Choix réels, parties différentes | S |
| GD-026 ⭐ | Nœud de départ du clan | Chaque clan commence avec une avancée propre | Asymétrie simple dès le tour 1 | S |
| GD-027 | Grands ouvrages | Un ouvrage unique au bout de chaque branche, un seul bâtisseur | Course visible, points de score | M |

### Clans (fiches complètes dans `clans.md`)

| N° | Titre | L'idée en une ligne | Intérêt | Coût |
|---|---|---|---|---|
| GD-040 ⭐ | Crocs-Rouges (agressif) | Razzieurs qui vivent du butin | Style le plus simple à comprendre | S |
| GD-041 ⭐ | Murailliers (défensif) | Forts et territoire piège | Contre naturel de l'agressif | S |
| GD-042 | Sèves (évolution) | Unités qui mutent selon ce qu'elles vivent | Très original, demandé | L |
| GD-043 ⭐ | Veilleurs d'Astres (recherche) | Avancées moins chères, copie chez les voisins | Style « savant » demandé | S |
| GD-044 ⭐ | Vents-Errants (exploration) | Vitesse, vision, récompenses de découverte | Style demandé, ouverture vive | S |
| GD-045 | Guilde des Gués (commerce) | Comptoirs dans les villes des autres | Gagner sans détruire | M |
| GD-046 | Conseil des Plumes (diplomatie) | Pactes, sceaux, ralliement pacifique | Jeu à plusieurs riche | L |
| GD-047 | Chanteurs du Soir (culture) | Influence qui convertit les villes | Conquête sans armes | M |
| GD-048 | Peuple des Chariots (nomade) | Villes qui se replient et se déplacent | Mécanique rare dans le genre | L |
| GD-049 | Fils de l'Écume (mer) | Marins, pontons, raids depuis l'eau | Donne du sens aux archipels | M |
| GD-050 | Racines (nature) | Forêts qui poussent, unités cachées dedans | Carte qui change | M |
| GD-051 | Tailleurs de Pierre (bâtisseurs) | Monuments, double grand ouvrage | Style « bâtir » paisible | M |
| GD-052 | Ruche Grise (essaim) | Multitude d'unités faibles qui se renforcent | Opposé net de l'élite | M |
| GD-053 | Ordre des Lames (élite) | Quelques champions qui montent en niveau | Attachement aux unités | M |
| GD-054 | Masques (ruse) | Leurres, espions, invisibilité | Bluff ; complique l'IA | L |
| GD-055 | Voyants de la Brume (mystique) | Rituels sur la brume, le terrain, la météo | Magie sans surcharge | L |
| GD-056 | Fouisseurs (souterrain) | Tunnels secrets, mines de montagne | Surprise tactique | M |
| GD-057 | Glaneurs (recyclage) | Débris des batailles récupérés | Profite des guerres des autres | M |
| GD-058 | Bergers des Cimes (bêtes) | Apprivoiser les bêtes neutres | Lien avec GD-012 | M |
| GD-059 | Gardiens du Feu (terre brûlée) | Incendier pour défricher et détruire | Rival naturel des Racines | M |

### Différenciation et équilibrage

| N° | Titre | L'idée en une ligne | Intérêt | Coût |
|---|---|---|---|---|
| GD-070 ⭐ | Échelle d'asymétrie en 4 niveaux | Bonus, unité, mécanique, victoire : commencer aux niveaux 1-2 | Avancer par petits pas | S |
| GD-071 ⭐ | Triangle des styles | Agressif > économie > défensif > agressif | Aucune stratégie dominante | S |
| GD-072 ⭐ | Triangle des unités | Lances > cavaliers > archers > lances, plus siège et navire | Combat tactique lisible | S |
| GD-073 ⭐ | Équilibrage par parties simulées | L'IA joue des milliers de parties, on lit la matrice des victoires | Équilibre mesuré, pas deviné | M |
| GD-074 | Tirage et interdiction de clans | En multijoueur : clans tirés, ou chacun en interdit un | Évite le clan à la mode | S |
| GD-075 | Clans à deux pièces | Un clan = une origine (bonus) + une tradition (mécanique), combinables | Beaucoup de combinaisons pour peu de contenu | M |
| GD-076 | Victoire propre au clan | Chaque clan a un objectif annoncé à tous | Asymétrie forte, parties lisibles | L |
| GD-077 ⭐ | Clans décrits par des données | Un fichier par clan (chiffres, capacités), pas de code à part | Ajout et équilibrage faciles | S |

### Socle

| N° | Titre | L'idée en une ligne | Intérêt | Coût |
|---|---|---|---|---|
| GD-090 ⭐ | Socle minimal V1 | 4 clans, arbre A, carte 11×11, score à 30 tours + domination | Une première version jouable et équilibrable | M |

---

## Fiches

### Boucle de jeu

#### GD-001 · Limite de tours et score
- **Description** : la partie s'arrête à un tour fixé (20 en duel, 30 par défaut, 40 en grand monde). Le score (villes, population, avancées, territoire, grands ouvrages, bêtes vaincues) désigne le vainqueur. Le score de chacun est visible à tout moment.
- **Pourquoi** : garantit des parties de 10 à 30 minutes ; une partie qui finit toujours est indispensable pour entraîner l'IA.
- **Coût** : S. **Dépendances** : aucune. **Sources** : boucle de la référence de genre (`sources.md`, S-01).

#### GD-002 · Trois familles de victoire
- **Description** : domination (prendre toutes les capitales), score (au tour limite), victoire propre au clan (objectif annoncé, voir GD-076). Le mode de partie choisit lesquelles sont actives.
- **Pourquoi** : laisse plusieurs chemins ; les clans pacifiques ont une chance.
- **Coût** : M. **Dépendances** : GD-001 ; GD-076 pour la troisième. **Sources** : S-03, S-05.

#### GD-003 · Formats et formes de carte
- **Description** : formats Duel 9×9, Escarmouche 11×11, Bataille 14×14, Grand monde 18×18 (environ 30 cases par joueur) ; formes continent, archipel, deux îles, anneau autour d'un lac, carte qui rétrécit. Génération à partir d'une graine.
- **Pourquoi** : la durée se règle par la taille ; la forme change la stratégie (mer, goulets).
- **Coût** : M. **Dépendances** : générateur de carte (S1). **Sources** : `boucle-de-jeu.md` § 5.

#### GD-004 · Une seule monnaie
- **Description** : une ressource (nom de travail « vivres ») paie unités, bâtiments et avancées. Affichée en haut avec le revenu du prochain tour.
- **Pourquoi** : lisibilité maximale ; l'arbitrage « avancée ou unité » est un vrai choix.
- **Coût** : S. **Dépendances** : aucune. **Sources** : S-01.

#### GD-005 · Villages neutres à rallier
- **Description** : la carte compte des villages neutres ; une unité qui s'y tient au début de son tour le rallie et en fait une ville. Pas de colons.
- **Pourquoi** : expansion rapide, carte disputée, premier contact tôt.
- **Coût** : S. **Dépendances** : GD-003. **Sources** : S-01.

#### GD-006 · Brume et lieux remarquables
- **Description** : brouillard de guerre (« la brume ») ; lieux remarquables à découvrir (ruines, sources, sommets) qui donnent une récompense tirée par la graine (vivres, avancée, unité, carte révélée).
- **Pourquoi** : l'exploration rapporte dès les premiers tours.
- **Coût** : S. **Dépendances** : GD-003. **Sources** : S-01.

#### GD-007 · Combat sans hasard, prévisualisé
- **Description** : dégâts calculés par une formule fixe (attaque, défense, points de vie, terrain, triangle des unités). Toucher une cible montre les dégâts donnés et reçus avant de confirmer.
- **Pourquoi** : lisible ; même graine, même partie ; l'IA apprend sur des règles stables.
- **Coût** : S. **Dépendances** : GD-072. **Sources** : S-04.

#### GD-008 · Rattrapage doux
- **Description** : le joueur dernier au score reçoit un petit bonus (avancées moins chères, ou un village de plus visible). Variante : GD-023.
- **Pourquoi** : évite les parties jouées d'avance au tour 10.
- **Coût** : S. **Dépendances** : GD-001. **Sources** : S-01 (critique de la fin de partie).

#### GD-009 · Événement de mi-partie
- **Description** : vers le tour 15, un événement annoncé 3 tours à l'avance : la mer monte (les cases côtières basses disparaissent), une migration de bêtes traverse la carte, une éclipse cache tout pendant 2 tours…
- **Pourquoi** : relance la partie là où le genre devient répétitif ; donne du sens à l'univers.
- **Coût** : M. **Dépendances** : GD-003, GD-012. **Sources** : S-03 (ères qui rythment la partie).

#### GD-010 · Compte à rebours et objectifs visibles
- **Description** : nombre de tours restants affiché ; progrès de chacun vers une victoire de clan visible à tous.
- **Pourquoi** : tension de fin de partie ; on sait qui il faut arrêter.
- **Coût** : S. **Dépendances** : GD-001, GD-002. **Sources** : S-05.

#### GD-011 · Plafond d'unités par ville
- **Description** : chaque ville entretient autant d'unités que sa taille ; au-delà, pas de nouvelle production.
- **Pourquoi** : peu de pions, tours rapides, fin de partie lisible.
- **Coût** : S. **Dépendances** : aucune. **Sources** : S-01.

#### GD-012 · Bêtes neutres
- **Description** : des bêtes gardent certains lieux remarquables et attaquent ce qui s'approche. Les vaincre rapporte des points de score et une récompense.
- **Pourquoi** : danger précoce, cibles hors joueurs, terrain pour les clans Bergers et Chariots.
- **Coût** : M. **Dépendances** : GD-006 ; une IA simple pour les bêtes. **Sources** : —.

#### GD-013 · Croissance à choix
- **Description** : quand une ville passe un niveau, deux récompenses au choix (par exemple : une unité gratuite ou +1 revenu ; un rempart ou +1 vision).
- **Pourquoi** : une décision intéressante à chaque croissance, sans menu lourd.
- **Coût** : S. **Dépendances** : GD-004. **Sources** : S-01, S-04.

#### GD-014 · Tours rapides
- **Description** : un toucher sélectionne, les cases possibles s'allument, un second toucher agit. Le bouton de fin de tour signale les unités inactives. Animation courte et passable.
- **Pourquoi** : fluidité d'une main sur téléphone.
- **Coût** : S. **Dépendances** : S1 (plateau). Visible : aperçu avant tout code. **Sources** : S-01.

### Trois choix qui décident de l'architecture de l'IA

Les trois fiches suivantes changent la façon dont la session 3 construira l'IA. Chacune donne les options (la recommandée en premier), leurs conséquences pour le jeu et pour l'IA, et ce qu'elles rendent possible ou non parmi les autres idées. L'utilisateur tranche au tri.

#### GD-015 · Taille maximale de la carte et nombre de joueurs
- **Description** : fixer la plus grande carte et le plus grand nombre de joueurs que le jeu acceptera, pour que l'IA soit construite pour cette limite.

| Option | Carte la plus grande | Joueurs | Pour le jeu | Pour l'IA |
|---|---|---|---|---|
| **A (recommandée)** : moyen | 14 × 14 (196 cases) ; V1 en 11 × 11 | 2 à 4 | Parties de 10 à 30 min ; formats Duel, Escarmouche, Bataille (`boucle-de-jeu.md` § 5) | L'état de la carte a une taille fixe, bornée à 196 cases : on peut tout ramener à une grille 14 × 14 de taille constante. Parties de 30 tours au plus : entraînement rapide. Au-delà de 2 joueurs, l'IA doit gérer plusieurs adversaires (alliances de fait, joueur qui fait gagner un autre) : on entraîne d'abord en duel, puis à 3 ou 4. |
| B : grand | 18 × 18 (324 cases) | 2 à 6 | Ajoute le « Grand monde » (40 min et plus) ; plus de place pour les clans diplomatie, commerce, nomades | État presque deux fois plus gros, parties de 40 tours ou sans limite : entraînement nettement plus long. À 5 ou 6 joueurs, l'apprentissage devient bien plus difficile et plus lent à mesurer. |
| C : duel seulement | 11 × 11 | 2 | Le plus simple ; mais moins « 4X », et les clans diplomatie (GD-046) ou commerce (GD-045) perdent leur sens | Le cas le plus simple : un contre un, gagnant ou perdant, adapté à l'entraînement de l'IA contre elle-même. |

- **Conséquences sur d'autres idées** : B est nécessaire pour GD-046 à plein régime et pour le format Grand monde de GD-003 ; C exclut les victoires de diplomatie.
- **Recommandation** : **A**. On entraîne d'abord l'IA en duel 11 × 11, puis à 3 ou 4 joueurs jusqu'à 14 × 14. L'option B reste possible plus tard si l'IA est conçue pour une taille de carte variable, à demander à S3.
- **Coût** : S (décision) ; le coût réel est côté IA. **Dépendances** : GD-003, GD-001. **Sources** : `boucle-de-jeu.md` § 5 et § 8.

#### GD-016 · Brouillard de guerre (« la brume ») ou non
- **Description** : décider ce que chaque joueur, humain ou IA, voit de la carte.

| Option | Règle | Pour le jeu | Pour l'IA |
|---|---|---|---|
| **A (recommandée)** : brume avec mémoire | Ce qui n'a jamais été vu est caché. Ce qui a été vu garde son terrain et ses villes connus, mais les unités ennemies n'y sont visibles que dans la vision actuelle (unités et villes) | L'exploration, premier des quatre X, a un sens ; surprise et menace à la frontière ; les clans exploration, ruse, souterrain et mystique fonctionnent | Information imparfaite : l'IA ne doit recevoir que ce que voit son joueur (sinon elle triche) et doit deviner le reste. Plus difficile que sans brume : il faut une mémoire de ce qu'elle a vu, et on ne peut plus calculer exactement les coups adverses. L'IA peut s'entraîner d'abord sans brume, puis avec. |
| B : sans brume | Tout est visible tout le temps | Plus simple et très lisible ; mais l'exploration se réduit à une course aux villages ; GD-006, GD-044, GD-054, GD-056 perdent l'essentiel de leur intérêt | Information complète : le cas le plus simple pour une IA de recherche (comme aux échecs). Entraînement plus rapide. |
| C : terrain caché, unités visibles | La carte est cachée au départ et se découvre ; une fois une zone découverte, on y voit tout, unités comprises, pour toujours | Compromis : l'exploration garde son sens (terrain, ruines, villages) ; pas de surprise militaire | L'incertitude ne porte que sur le terrain inconnu, pas sur les unités adverses : beaucoup plus simple que A pour l'IA. |
| D : au choix de la partie | A ou B, réglé au lancement | Les deux publics | Deux IA à entraîner, ou une seule qui sait jouer les deux : coût doublé environ. |

- **Conséquences sur d'autres idées** : A est nécessaire pour les Masques (GD-054), les Fouisseurs (GD-056) et les Voyants (GD-055) ; C les affaiblit ; B les rend inutiles.
- **Recommandation** : **A**, parce que l'exploration est un pilier du genre et que plusieurs clans en dépendent. Si S3 juge A trop coûteux pour la V1, **C** est le repli. B ne garde qu'une partie du jeu.
- **Coût** : M (vision par unité, mémoire de la carte vue, affichage). **Dépendances** : GD-006. **Sources** : S-01.

#### GD-017 · Combats avec ou sans hasard
- **Description** : décider si un même combat donne toujours le même résultat.

| Option | Règle | Pour le jeu | Pour l'IA |
|---|---|---|---|
| **A (recommandée)** : sans hasard | Dégâts calculés par une formule fixe (attaque, défense, points de vie, terrain, triangle des unités). Le hasard n'existe qu'à la création de la partie (carte, contenu des ruines), tiré de la graine | La prévisualisation montre le résultat exact (GD-007) : lisible, aucune frustration, le joueur est seul responsable | Un coup donne toujours le même état : l'IA peut calculer plusieurs coups à l'avance. Même graine, même partie : reproductible. Équilibrage mesurable avec moins de parties. |
| B : petit hasard borné | Dégâts variant d'au plus ± 10 %, ou un coup critique rare, tirés d'un générateur à graine | Un peu de suspense, résultats moins « calculables » ; la prévisualisation montre une fourchette | Reproductible si le générateur à graine fait partie de l'état de la partie. L'IA doit raisonner sur des moyennes ; il faut plus de parties pour mesurer l'équilibre. |
| C : hasard franc | Chaque combat a une chance de victoire (par exemple 70 %) | Tension, coups de chance ; mais frustration fréquente sur des parties courtes, et la prévisualisation n'affiche qu'un pourcentage | Le plus coûteux : beaucoup de variance, apprentissage plus lent, équilibrage bruité. |

- **Règle commune à toutes les options** : tout hasard, même dans la création de la carte, vient d'un seul générateur à graine rangé dans l'état de la partie (`CLAUDE.md`, point 2).
- **Recommandation** : **A**. Le hasard se met là où il crée de la variété sans frustrer : la carte, les ruines, le tirage d'avancées de l'arbre B (GD-021).
- **Coût** : S. **Dépendances** : GD-007, GD-072. **Sources** : S-04 ; `boucle-de-jeu.md` § 6 et § 8.

### Arbre d'évolution

#### GD-020 · Arbre A « Les Trois Veines »
- **Description** : un nœud de départ propre au clan, puis trois branches de 5 paliers (Fer : force ; Terre : croissance ; Souffle : savoir et mouvement). Au palier 3 de chaque branche, choix exclusif entre deux avancées. Au palier 5, un grand ouvrage. 16 nœuds. Esquisse dans `arbre-evolution.md` § 3.
- **Pourquoi** : trois colonnes, tout l'arbre tient sur un écran ; une branche ≈ un style de jeu.
- **Coût** : M. **Dépendances** : GD-004, GD-025, GD-026, GD-027. **Sources** : S-01, S-03.

#### GD-021 · Arbre B « Les Quatre Lunes »
- **Description** : quatre ères ; à chaque ère, 5 avancées tirées (par la graine) dans le paquet de l'ère ; en prendre 3 fait passer à l'ère suivante (au plus tard aux tours 8, 16, 24).
- **Pourquoi** : chaque partie diffère ; le passage d'ère relance le rythme.
- **Coût** : M. **Dépendances** : GD-004. **Sources** : S-03.

#### GD-022 · Arbre C « L'Arbre Vivant »
- **Description** : petit arbre de 9 nœuds propre à chaque clan ; chaque nœud s'obtient par un haut fait (gagner 3 combats, voir 60 % de la carte…), sans dépense.
- **Pourquoi** : récompense le joueur qui joue le style de son clan ; aucun menu.
- **Coût** : L. **Dépendances** : clans définis ; s'ajoute à GD-020. **Sources** : —.

#### GD-023 · Le Puits des Savoirs
- **Description** : chaque avancée déjà connue d'un joueur que l'on a rencontré coûte 25 % de moins.
- **Pourquoi** : rattrapage naturel, intérêt à rencontrer ses voisins, l'isolé paie plein tarif.
- **Coût** : S. **Dépendances** : n'importe quel arbre. **Sources** : —.

#### GD-024 · Montée des coûts
- **Description** : trois règles possibles pour le prix des avancées : par palier, par nombre de villes, par nombre d'avancées déjà prises. À comparer par parties simulées.
- **Pourquoi** : le rythme d'une partie dépend surtout de cette règle.
- **Coût** : S. **Dépendances** : GD-073. **Sources** : S-01.

#### GD-025 · Avancées exclusives
- **Description** : à certains paliers, deux avancées se font face ; en prendre une ferme l'autre pour la partie.
- **Pourquoi** : choix réels, deux joueurs du même clan ne jouent pas pareil.
- **Coût** : S. **Dépendances** : GD-020. **Sources** : S-04.

#### GD-026 · Nœud de départ du clan
- **Description** : chaque clan commence avec une avancée propre, déjà acquise (par exemple Crocs-Rouges : « Lames de silex », Vents-Errants : « Pistes »).
- **Pourquoi** : asymétrie de niveau 1, simple, visible dès le tour 1.
- **Coût** : S. **Dépendances** : GD-020. **Sources** : S-01.

#### GD-027 · Grands ouvrages
- **Description** : au bout de chaque branche, un ouvrage unique dans la partie ; le premier qui le bâtit l'a, les autres non. Effet fort et points de score.
- **Pourquoi** : course visible entre joueurs ; objectif de fin de partie.
- **Coût** : M. **Dépendances** : GD-020. **Sources** : —.

### Clans

Description, style, forces, faiblesses, victoire, contres et rythme : `clans.md`. Ici, l'intérêt, le coût et les dépendances.

#### GD-040 · Crocs-Rouges (agressif)
- **Pourquoi** : le style le plus simple à comprendre ; teste le combat dès la V1.
- **Coût** : S. **Dépendances** : GD-007. **Sources** : S-02.

#### GD-041 · Murailliers (défensif)
- **Pourquoi** : contre naturel des Crocs-Rouges ; teste les remparts et le territoire.
- **Coût** : S. **Dépendances** : territoire, remparts. **Sources** : S-02.

#### GD-042 · Sèves (évolution)
- **Pourquoi** : demandé par l'utilisateur ; très original ; crée de l'attachement aux unités.
- **Coût** : L (arbre de mutations, interface de choix). **Dépendances** : expérience des unités. **Sources** : —.

#### GD-043 · Veilleurs d'Astres (recherche)
- **Pourquoi** : demandé ; simple en niveau 1-2 (avancées moins chères).
- **Coût** : S. **Dépendances** : GD-020. **Sources** : —.

#### GD-044 · Vents-Errants (exploration)
- **Pourquoi** : demandé ; ouverture vive ; met en valeur la brume et les lieux remarquables.
- **Coût** : S. **Dépendances** : GD-006. **Sources** : —.

#### GD-045 · Guilde des Gués (commerce)
- **Pourquoi** : gagner en enrichissant les autres crée des alliances de fait.
- **Coût** : M. **Dépendances** : routes, caravanes. **Sources** : —.

#### GD-046 · Conseil des Plumes (diplomatie)
- **Pourquoi** : rend le jeu à plusieurs riche ; mais demande une diplomatie complète et une IA qui la comprend.
- **Coût** : L. **Dépendances** : système de pactes. **Sources** : —.

#### GD-047 · Chanteurs du Soir (culture)
- **Pourquoi** : conquête sans armes, style rare sur téléphone.
- **Coût** : M. **Dépendances** : jauge d'influence. **Sources** : —.

#### GD-048 · Peuple des Chariots (nomade)
- **Pourquoi** : villes mobiles, mécanique rare dans le genre ; très bon pour les cartes moyennes.
- **Coût** : L (casse l'hypothèse « une ville ne bouge pas »). **Dépendances** : —. **Sources** : —.

#### GD-049 · Fils de l'Écume (mer)
- **Pourquoi** : donne du sens aux archipels et à l'événement « la mer monte ».
- **Coût** : M. **Dépendances** : bateaux, GD-003. **Sources** : —.

#### GD-050 · Racines (nature)
- **Pourquoi** : la carte change pendant la partie ; rivalité naturelle avec GD-059.
- **Coût** : M. **Dépendances** : terrain modifiable. **Sources** : —.

#### GD-051 · Tailleurs de Pierre (bâtisseurs)
- **Pourquoi** : style paisible, bon pour les joueurs qui n'aiment pas le combat.
- **Coût** : M. **Dépendances** : GD-027. **Sources** : —.

#### GD-052 · Ruche Grise (essaim)
- **Pourquoi** : opposé exact de l'élite ; teste le plafond d'unités.
- **Coût** : M. **Dépendances** : GD-011. **Sources** : —.

#### GD-053 · Ordre des Lames (élite)
- **Pourquoi** : attachement à quelques héros ; contraste avec l'essaim.
- **Coût** : M. **Dépendances** : expérience des unités. **Sources** : —.

#### GD-054 · Masques (ruse)
- **Pourquoi** : bluff et information cachée ; amusant entre humains.
- **Coût** : L ; complique l'apprentissage de l'IA (information cachée). **Dépendances** : brume. **Sources** : —.

#### GD-055 · Voyants de la Brume (mystique)
- **Pourquoi** : magie sobre liée à l'univers (la brume) ; rituels visibles donc contrables.
- **Coût** : L. **Dépendances** : GD-009, terrain modifiable. **Sources** : —.

#### GD-056 · Fouisseurs (souterrain)
- **Pourquoi** : surprise tactique, mise en valeur des montagnes.
- **Coût** : M. **Dépendances** : terrain. **Sources** : —.

#### GD-057 · Glaneurs (recyclage)
- **Pourquoi** : profite des guerres des autres ; jeu opportuniste original.
- **Coût** : M. **Dépendances** : débris sur la carte. **Sources** : —.

#### GD-058 · Bergers des Cimes (bêtes)
- **Pourquoi** : réutilise les bêtes neutres ; armée « empruntée ».
- **Coût** : M. **Dépendances** : GD-012. **Sources** : —.

#### GD-059 · Gardiens du Feu (terre brûlée)
- **Pourquoi** : terrain qui change ; rival des Racines ; agressif d'un autre genre.
- **Coût** : M. **Dépendances** : terrain modifiable. **Sources** : —.

### Différenciation et équilibrage

#### GD-070 · Échelle d'asymétrie en 4 niveaux
- **Description** : 1 bonus de départ, 2 unité propre, 3 mécanique propre, 4 victoire propre. Commencer aux niveaux 1-2 ; monter quand l'IA d'équilibrage existe. Tableau dans `clans.md` § 4.
- **Pourquoi** : avancer par petits pas ; chaque niveau se teste avant le suivant.
- **Coût** : S. **Dépendances** : —. **Sources** : S-02.

#### GD-071 · Triangle des styles
- **Description** : agressif bat économie, économie bat défensif, défensif bat agressif. Chaque clan a sa place, ou est « opportuniste ».
- **Pourquoi** : aucune stratégie dominante ; donne un langage commun pour équilibrer.
- **Coût** : S. **Dépendances** : —. **Sources** : S-02, S-04.

#### GD-072 · Triangle des unités
- **Description** : lances > cavaliers > archers > lances ; siège fort contre les villes ; navires à part. Bonus affiché dans la prévisualisation.
- **Pourquoi** : tactique lisible, choix de production intéressant.
- **Coût** : S. **Dépendances** : GD-007. **Sources** : —.

#### GD-073 · Équilibrage par parties simulées
- **Description** : l'IA (S3) joue des milliers de parties clan contre clan ; on lit la matrice des taux de victoire ; cibles : aucun clan au-dessus de 55 % en moyenne ni de 65 % contre un autre ; on ajuste les chiffres des fichiers de clan.
- **Pourquoi** : l'équilibre se mesure au lieu de se deviner ; c'est le point fort du projet (moteur rapide et reproductible).
- **Coût** : M. **Dépendances** : GD-077, moteur sans écran (S1), IA (S3). **Sources** : S-02.

#### GD-074 · Tirage et interdiction de clans
- **Description** : en multijoueur, chaque joueur reçoit deux clans tirés et en garde un ; ou chaque joueur en interdit un avant le choix.
- **Pourquoi** : évite que tout le monde joue le clan le plus fort du moment.
- **Coût** : S. **Dépendances** : multijoueur. **Sources** : —.

#### GD-075 · Clans à deux pièces
- **Description** : un clan = une **origine** (bonus de départ, unité propre) + une **tradition** (mécanique propre), choisies séparément. 6 origines × 6 traditions = 36 combinaisons.
- **Pourquoi** : beaucoup de variété pour peu de contenu ; mais plus dur à équilibrer et à identifier.
- **Coût** : M. **Dépendances** : GD-077, GD-073. **Sources** : —.

#### GD-076 · Victoire propre au clan
- **Description** : chaque clan a un objectif de victoire annoncé à tous (exemples dans `clans.md`), progrès visible.
- **Pourquoi** : asymétrie forte et lisible, chaque partie raconte une histoire.
- **Coût** : L. **Dépendances** : GD-002, GD-010, GD-073. **Sources** : S-02.

#### GD-077 · Clans décrits par des données
- **Description** : un fichier par clan (chiffres de départ, avancée de départ, unité propre, capacités prises dans une liste commune). Le moteur lit ces fichiers.
- **Pourquoi** : ajouter un clan sans code ; l'IA peut faire varier les chiffres pour équilibrer.
- **Coût** : S. **Dépendances** : architecture du moteur (S1). **Sources** : —.

### Socle

#### GD-090 · Socle minimal V1
- **Description** :
  - **Carte** : Escarmouche 11×11, 2 à 3 joueurs (limite du jeu : GD-015), brume avec mémoire (GD-016), 8 à 10 villages neutres, quelques lieux remarquables ; terrains plaine, forêt, colline, montagne, eau (sans bateaux au début).
  - **Victoire** : score au tour 30, ou domination avant (GD-001, GD-002 sans victoire de clan).
  - **Économie** : une monnaie (GD-004), villages à rallier (GD-005), croissance à choix (GD-013), plafond d'unités (GD-011).
  - **Combat** : sans hasard (GD-017), prévisualisé (GD-007), triangle des unités (GD-072) : 5 unités communes (éclaireur, lancier, archer, cavalier, catapulte) + 1 unité propre par clan.
  - **Arbre** : arbre A, 16 nœuds (GD-020, GD-025, GD-026, GD-027).
  - **Clans** : 4, bouquet A de `clans.md` : Crocs-Rouges, Murailliers, Vents-Errants, Veilleurs d'Astres, en asymétrie niveaux 1 et 2 (GD-070), décrits par des données (GD-077).
  - **Équilibrage** : parties simulées dès que le moteur et une IA simple existent (GD-073).
- **Pourquoi** : le plus petit ensemble qui couvre les quatre X, les styles demandés et le triangle ; tout se joue sans écran.
- **Coût** : M. **Dépendances** : P1.4 (plateau et tour par tour, S1). **Sources** : toutes.
