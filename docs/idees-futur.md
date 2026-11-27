# Idées pour plus tard

> À ne lire que sur demande de l'utilisateur. Rien d'ici n'est dans le plan.

## Prévu juste après la première version (D-32)

- La nuit comme mécanique de jeu, avec des monstres de nuit et une allure propre aux nuits (D-33). À concevoir par S2, à valider par l'utilisateur, puis à coder en premier dans la version suivante.

## Tri du 2026-11-27 : idées mises « plus tard »

> Venues des catalogues des sessions 2 à 6 ; fiche complète sur la branche de chaque session.

### Game design (26, branche `s2-game-design`)

- GD-001 · Partie limitée en tours, gagnée au score : La partie s'arrête à un tour fixé, par exemple le trentième, et le meilleur score gagne. Le score de chacun reste visible.
- GD-003 · Plusieurs tailles et formes de carte : Du duel sur petite carte au grand monde, en continent, archipel, anneau autour d'un lac ou carte qui rétrécit.
- GD-008 · Coup de pouce au joueur en retard : Le dernier au score paie un peu moins ses avancées, pour revenir dans la partie.
- GD-009 · Un grand événement en milieu de partie : Vers le quinzième tour, annoncé à l'avance : la mer monte, des bêtes migrent, une éclipse cache tout.
- GD-012 · Bêtes sauvages qui gardent les ruines : Des bêtes protègent certains lieux et attaquent ce qui approche ; les vaincre rapporte.
- GD-021 · Arbre en quatre ères, avancées tirées au sort : À chaque ère, cinq avancées tirées au sort ; en prendre trois fait passer à l'ère suivante.
- GD-022 · Arbre propre au clan, gagné par exploits : Chaque clan a son petit arbre, qui pousse quand on accomplit des exploits, sans rien dépenser.
- GD-023 · Savoirs qui circulent entre voisins : Une avancée déjà connue d'un voisin rencontré coûte un quart de moins.
- GD-024 · Comment montent les prix des avancées : Comparer trois façons de faire monter les prix : par palier, par nombre de villes, par avancées déjà prises.
- GD-042 · Sèves, le clan dont les unités mutent : Leurs créatures choisissent une mutation après avoir combattu ou survécu, et deviennent redoutables avec le temps.
- GD-045 · Guilde des Gués, le clan marchand : Des marchands qui ouvrent des comptoirs chez les autres et s'enrichissent avec eux.
- GD-046 · Conseil des Plumes, le clan diplomate : Des émissaires qui gagnent par les pactes et rallient les villages sans combattre.
- GD-047 · Chanteurs du Soir, le clan qui convertit : Leur influence s'étend autour de leurs villes et fait basculer les villes voisines.
- GD-048 · Peuple des Chariots, le clan nomade : Leurs villes se replient en convoi et vont se réinstaller ailleurs.
- GD-049 · Fils de l'Écume, le clan marin : Des marins qui bâtissent sur l'eau et attaquent depuis la mer.
- GD-050 · Racines, le clan de la forêt : Ils font pousser la forêt, s'y cachent et y sont plus forts.
- GD-051 · Tailleurs de Pierre, le clan bâtisseur : Des maîtres d'œuvre qui bâtissent moins cher et peuvent élever deux merveilles.
- GD-052 · Ruche Grise, le clan de la multitude : Beaucoup d'unités faibles et bon marché qui se renforcent en restant groupées.
- GD-053 · Ordre des Lames, le clan des champions : Quelques champions très forts qui gagnent en renommée à chaque victoire.
- GD-055 · Voyants de la Brume, le clan mystique : Des prêtres qui lancent des rituels sur la brume, le terrain et le temps.
- GD-056 · Fouisseurs, le clan des tunnels : Des mineurs qui creusent des tunnels secrets entre leurs villes et exploitent les montagnes.
- GD-058 · Bergers des Cimes, le clan des dresseurs : Ils apprivoisent les bêtes sauvages au lieu de les tuer.
- GD-059 · Gardiens du Feu, le clan qui brûle : Ils incendient la forêt pour en faire des champs et brûlent le terrain devant l'ennemi.
- GD-074 · Clans tirés au sort ou interdits : En partie à plusieurs, chacun reçoit deux clans tirés au sort ou en interdit un.
- GD-075 · Clans à assembler en deux pièces : Un clan se compose d'une origine et d'une tradition choisies séparément, pour des dizaines de combinaisons.
- GD-076 · Un objectif de victoire propre à chaque clan : Chaque clan a son but, annoncé à tous, comme convertir trois villes ou accomplir un grand rite.

### IA (9, branche `s3-ia`)

- IA-010 · Partager ses parties pour faire progresser l'IA : Avec votre accord, vos parties sont envoyées sans aucune donnée personnelle et vérifiées avant de servir à l'entraînement.
- IA-011 · Une IA qui apprend des joueurs : L'IA apprend à jouer comme les bons joueurs à partir des parties partagées.
- IA-012 · Une IA très forte qui s'entraîne seule : L'IA s'entraîne longuement contre elle-même avec un petit cerveau artificiel, comme les meilleures IA de jeux de plateau.
- IA-013 · Une IA qui progresse de mois en mois : À intervalles réguliers, une nouvelle IA entraînée sur les parties de tous est publiée si elle bat l'ancienne.
- IA-014 · Affronter le double d'un joueur : Une IA imite le style d'un joueur précis, pour l'affronter ou le défier entre amis, avec son accord.
- IA-015 · Nouvelles IA sans réinstaller le jeu : Les IA améliorées se téléchargent à part, sans nouvelle version du jeu, et l'ancienne est gardée.
- IA-016 · L'IA explique ses coups : L'IA peut dire pourquoi elle a joué un coup, par exemple une menace sur votre ville.
- IA-017 · Des IA qui cherchent les failles : Des IA spéciales cherchent les points faibles de la meilleure IA, qui apprend ensuite à les corriger.
- IA-018 · Un petit cerveau qui évolue seul : Un petit cerveau artificiel dont la forme évolue au fil des générations.

### Données (13, branche `s4-donnees`)

- DO-008 · Envoi des parties quand le réseau revient : Le téléphone envoie ses nouvelles parties d'un coup, une fois par jour ou au retour du Wi-Fi.
- DO-009 · Un petit serveur gratuit pour recevoir : Un service en ligne gratuit (Cloudflare) range les parties reçues ; quelques dollars par mois au-delà de mille joueurs.
- DO-010 · Rejouer chaque partie pour démasquer les tricheurs : Chaque partie reçue est rejouée : un coup impossible ou une fausse victoire, et elle est écartée.
- DO-011 · Vérifier que l'envoi vient du vrai jeu : Le Play Store certifie que les parties viennent du jeu officiel sur un vrai téléphone.
- DO-012 · Protéger l'IA des mauvaises leçons : Les parties des joueurs sont filtrées et plafonnées, et une nouvelle IA doit battre l'ancienne avant d'être livrée.
- DO-013 · Demander l'accord et permettre d'effacer : Une question claire à la première partie, un réglage pour changer d'avis, un bouton pour tout effacer.
- DO-014 · Un numéro de joueur au hasard : Chaque joueur reçoit un numéro tiré au hasard, sans lien avec son téléphone ni son compte.
- DO-015 · Nouvelle IA sans réinstaller le jeu : La nouvelle IA se télécharge seule depuis GitHub, vérifiée avant usage, avec l'ancienne gardée en secours.
- DO-016 · Essayer une nouvelle IA sur quelques joueurs : La nouvelle IA ne va d'abord qu'à un joueur sur cinq ; on compare, puis on la généralise ou on revient.
- DO-018 · Un écran du progrès de l'IA : Dans le jeu, une courbe du niveau de l'IA et la part de vos parties dans ce progrès.
- DO-019 · Effacer les vieilles parties après deux ans : Les parties détaillées sont effacées au bout de 24 mois, seuls les totaux restent.
- DO-020 · Revoir une partie coup par coup : Une partie enregistrée se rejoue à l'écran, tour après tour.
- DO-021 · Un serveur loué à prix fixe : Un petit serveur loué 5 à 7 € par mois, à entretenir soi-même.

### Veille (58, branche `s5-veille`)

- VE-001 · Voir ce que prépare l'adversaire : Avant de jouer, l'adversaire laisse voir sa cible ou la case qu'il menace.
- VE-002 · Voir l'effet avant de valider : Un appui long montre ce que l'action va changer avant de la confirmer.
- VE-005 · Un signe montre comment agit l'ennemi : Une couleur ou une forme indique si une unité adverse charge, garde ou tire de loin.
- VE-006 · La clarté avant tout : Une règle impossible à montrer simplement est simplifiée ou supprimée.
- VE-007 · Parties équitables : même clan, exclusions : En partie personnalisée, tout le monde joue le même clan, ou on retire un clan jugé trop fort.
- VE-008 · Freiner le joueur qui s'emballe : Entretien des unités, villes de plus en plus chères ou nombre d'actions limité par tour.
- VE-010 · Énigme tactique du jour : Une situation fixe à résoudre en quelques tours, avec une série de jours à tenir.
- VE-012 · Parties à distance, chacun à son rythme : Chacun joue quand il veut, par échange de fichier ou en ligne avec un long délai par tour.
- VE-013 · Expédition de petites parties enchaînées : Quatre à six mini-parties sur une carte à embranchements, en gardant quelques unités.
- VE-014 · Scénarios à règle changée : Des scénarios où une règle change, puis un mode où l'on choisit ses règles.
- VE-017 · Score fait d'exploits variés : Explorer, bâtir et vaincre rapportent des étoiles ; chaque voie a des étapes visibles.
- VE-018 · Lieux clés révélés un à un : Des lieux spéciaux dévoilent leur effet au fil des premiers tours ; les tenir peut faire gagner.
- VE-019 · Objectifs qui apparaissent en cours de partie : À mi-partie, un objectif de carte ou personnel apparaît et rapporte des points.
- VE-020 · Objectifs bonus sur chaque carte : Un ou deux objectifs facultatifs par carte, comme gagner en moins de N tours.
- VE-022 · Perdre quand le moral s'effondre : On perd aussi quand une jauge commune tombe à zéro, pas seulement en perdant ses unités.
- VE-023 · Plusieurs types de cartes : Terres pleines, lacs, continents, archipels, avec des tailles selon le nombre de joueurs.
- VE-025 · L'IA teste l'équité des cartes : Les IA jouent chaque carte avant nous et on écarte celles qui avantagent un camp.
- VE-027 · Brouillard qui revient : Ce qu'on ne voit plus redevient incertain, ce qui permet des attaques surprises.
- VE-028 · Événements à deux ou trois choix : Une découverte déclenche un petit choix, avec l'effet montré avant de décider.
- VE-029 · Un régisseur pour le rythme : Un régisseur invisible alterne calme et temps forts sans rendre l'IA plus forte.
- VE-030 · Décisions définitives mais nuançables : Certaines décisions ne s'annulent pas ; une décision suivante peut seulement les nuancer.
- VE-031 · Rallier des villages neutres : Un village neutre se rallie si l'on remplit une condition simple, sinon par la force.
- VE-032 · Diplomatie en trois mots : Paix, alliance, rupture ; chaque demande coûte et rompre une paix se paie.
- VE-033 · Humeur des clans affichée honnêtement : Chaque clan IA montre ce qu'il pense de vous et pourquoi, et tient ses promesses.
- VE-034 · Tutoriel en petits chapitres : Une règle par chapitre de deux minutes et de petites cartes de difficulté croissante.
- VE-035 · Quelques actions conseillées à chaque tour : Au début du tour, le jeu propose quelques actions utiles.
- VE-036 · Changer la difficulté librement : Le niveau se change sans pénalité ; un mode libre sans score ni défaite existe.
- VE-037 · Le tour de l'IA sans attente : Le tour de l'ordinateur passe instantanément ou en accéléré, avec combat rapide.
- VE-038 · Jouer sans réseau, reprendre à tout moment : Tout le solo marche hors ligne, en portrait ou paysage, avec sauvegarde à chaque tour.
- VE-039 · Ménager la batterie : L'écran ne se redessine pas sans cesse et l'IA calcule sobrement.
- VE-042 · Résumé de partie à partager : Une petite grille à copier raconte la partie sans révéler la solution du défi.
- VE-043 · Succès liés au style de jeu : Des succès comme « gagner sans armée » ou « gagner en moins de 15 tours ».
- VE-044 · Classement en ligne vérifié : Les scores sont envoyés en ligne et vérifiés en rejouant la partie.
- VE-050 · Les IA ne se liguent pas contre vous : Les IA visent plutôt le joueur en tête ; leur agressivité se règle.
- VE-051 · Des erreurs d'IA crédibles : Aux niveaux faciles, l'IA se trompe comme un débutant et montre les bons réflexes.
- VE-052 · Des clans IA au caractère visible : Chaque clan IA a un caractère qui se voit dans ses choix.
- VE-053 · Un adversaire qui se souvient de vous : Un chef IA se rappelle vos parties précédentes et adapte ses préférences.
- VE-054 · L'IA dévoile de nouvelles tactiques : L'IA montre peu à peu de nouvelles tactiques, jamais pour punir une défaite.
- VE-055 · Un rival qui imite un joueur : Une IA apprend le style d'un joueur et devient un adversaire hors ligne.
- VE-056 · IA mise à jour, défi de la faille : L'IA reçoit des nouvelles versions ; les victoires par ruse servent à la corriger.
- VE-057 · Une IA qui vous entraîne : Une IA un peu plus forte rejoue avec vous pour vous aider à progresser.
- VE-058 · Progresser sans devenir plus fort : Gagner débloque un niveau plus dur ou une variante, jamais un bonus de puissance.
- VE-059 · Petites saisons mensuelles : Chaque mois, un mode ou une règle spéciale, sans passe payant.
- VE-060 · Éditeur de cartes : Créer ses cartes et les partager par fichier ou par code.
- VE-062 · Aucune notification par défaut : Au plus un rappel de défi, désactivé tant que le joueur ne l'active pas.
- VE-063 · Une communauté qui démarre petit : D'abord une page de projet et les discussions du dépôt, un salon plus tard.
- VE-068 · Annoncer sans pub, sans IA générative : Une phrase sur la page du jeu dit ce qu'il n'y a pas et ce que fait vraiment l'IA.
- VE-070 · Autres boutiques : itch.io, F-Droid, Play : Publier ailleurs le jour où l'on voudra des joueurs extérieurs.
- VE-071 · Une version pour ordinateur : Le même moteur de règles avec un autre affichage, pour PC ou navigateur.
- VE-072 · Clans très différents, même but : Chaque clan a sa façon de jouer et une voie de victoire favorite.
- VE-073 · Taille de l'arbre et des unités : Arbre plus long pour la fin de partie, ou au contraire réduit pour rester clair.
- VE-074 · Jeu sur la mer : Une mécanique propre à la mer, comme des cases inondées.
- VE-075 · Commandants qui changent le style : Des chefs à débloquer qui changent la façon de jouer.
- VE-076 · Factions internes qui réagissent : Des groupes internes réagissent aux lois et aux techniques choisies.
- VE-077 · Unité d'infiltration invisible : Une unité invisible jusqu'à sa première action, risque de frustration.
- VE-078 · Caractère des clans né des gènes : Le caractère des clans IA vient de l'évolution génétique, avec leur lignée visible.
- VE-079 · Entraîner l'IA avant, pas en direct : L'IA apprend avant la sortie ; le joueur ne règle que des choses lisibles.
- VE-080 · Entraîner contre des adversaires variés : L'IA s'entraîne contre une ligue d'adversaires différents.

### Direction artistique (3, branche `s6-direction-artistique`)

- DA-008 · Lavis, tons de papier : Terrains très pâles comme de l’aquarelle, clans en couleurs d’encre franches.
- DA-011 · Anneau autour de ce qu’on touche : Le plateau occupe tout l’écran ; les actions s’ouvrent en cercle autour de l’unité ou de la ville.
- DA-012 · Onglets en bas de l’écran : Barre d’onglets Carte, Villes, Savoir, Clans ; un écran par tâche.
