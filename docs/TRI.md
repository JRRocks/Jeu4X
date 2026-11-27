# Résultat du tri

> Tri fait par l'utilisateur le 2026-11-27 sur la page de tri, recommandations des sessions pré-cochées, les cinq catalogues validés. Décisions D-22 à D-27. Les idées « plus tard » sont dans `docs/idees-futur.md`.

| Catalogue | Gardées | Plus tard | Jetées | Changé par l'utilisateur |
|---|---|---|---|---|
| Game design (S2) | 25 | 26 | 3 | GD-001 : garder → plus tard, GD-017 : garder → jeter |
| IA (S3) | 9 | 9 | 1 | IA-009 : plus tard → garder, IA-018 : jeter → plus tard |
| Données (S4) | 7 | 13 | 1 | rien |
| Veille (S5) | 20 | 58 | 2 | rien |
| Direction artistique (S6) | 4 | 3 | 5 | DA-001 : plus tard → jeter, DA-002 : plus tard → jeter, DA-004 : plus tard → jeter, DA-005 : plus tard → jeter, DA-009 : plus tard → jeter |
| **Total** | **65** | **109** | **12** | |

## Réponses aux questions

- **GD-Q1** Le clan dont les unités mutent entre-t-il dans la première version ? → **Plus tard, en deuxième version** (recommandation confirmée). On commence par quatre clans simples, il arrive ensuite.
- **GD-Q2** Quel arbre d'évolution pour commencer ? → **Trois branches sur un écran** (recommandation acceptée). Seize avancées en trois colonnes : force, croissance, savoir.
- **GD-Q3** Combien de monnaies dans le jeu ? → **Une seule monnaie** (recommandation acceptée). Elle paie tout : unités, bâtiments et avancées.
- **GD-Q4** L'univers des îles sortant de la brume vous convient-il ? → **Oui, comme base provisoire** (recommandation acceptée). On le garde le temps de construire le jeu.
- **GD-Q5** Jusqu'où les clans diffèrent-ils dans la première version ? → **Bonus de départ et unité propre** (recommandation acceptée). Chaque clan a son avancée de départ et une unité à lui.
- **GD-Q6** Quelle taille de carte et combien de joueurs au plus ? → **Moyenne, jusqu'à quatre joueurs** (recommandation acceptée). Quatorze cases de côté au plus ; on commence à onze.
- **GD-Q7** Brouillard sur la carte ou non ? → **Brouillard qui se souvient** (recommandation acceptée). Ce qui a été vu reste connu ; les ennemis ne se voient qu'à portée.
- **GD-Q8** Les combats ont-ils une part de chance ? → **Aucune chance** (recommandation acceptée). Le résultat annoncé est exact ; la chance ne sert qu'à créer la carte.
- **IA-Q1** Accepterais-tu de payer de la puissance de calcul pour rendre l'IA très forte ? → **Non, gratuit seulement pour commencer** (recommandation acceptée). Ordinateur et offres gratuites en ligne ; on reverra la question quand on y sera.
- **DO-Q1** Quand d'autres personnes joueront, leurs parties peuvent-elles être envoyées pour faire progresser l'IA ? → **Oui si le joueur accepte** (recommandation acceptée). Question posée à la première partie, réponse « non » par défaut, rien d'envoyé avant 15 ans.
- **DO-Q2** Où recevoir les parties des autres joueurs, le moment venu ? → **Service gratuit Cloudflare** (recommandation acceptée). Gratuit jusqu'à environ mille joueurs, sans mise en veille ni carte bancaire, rien à entretenir.
- **DO-Q3** Vos propres parties doivent-elles se copier seules sur votre Google Drive dès maintenant ? → **Oui, copie automatique** (recommandation acceptée). Il faudra autoriser le jeu une fois à écrire dans un dossier de votre Drive.
- **DO-Q4** Comment voulez-vous suivre les progrès de l'IA ? → **Page web publique gratuite** (recommandation acceptée). Visible par tous, s'ouvre sur le téléphone, rien à entretenir.
- **VE-Q1** Quel modèle économique pour le jeu ? → **Tout gratuit, sans pub** (recommandation acceptée). Aucun achat, aucune publicité, au plus un lien de soutien hors du jeu.
- **VE-Q2** Comment gagne-t-on une partie dans la première version ? → **Score en tours fixes** (recommandation acceptée). Par défaut un nombre de tours connu, avec capitale ou seuil de points comme victoires rapides.
- **VE-Q3** Quel premier rendez-vous régulier ? → **Défi daté, sans réseau** (recommandation acceptée). Défi du jour et de la semaine sur le téléphone, avec code et résumé à partager.
- **VE-Q4** Quel multijoueur en premier ? → **Sur le même téléphone** (recommandation acceptée). On se passe le téléphone, l'IA tient les places vides.
- **VE-Q5** Que montrer de l'IA au joueur ? → **Compteur et mode spectateur** (recommandation acceptée). Nombre de parties jouées, progression, et parties IA contre IA à regarder.
- **DA-Q1** Quel style graphique ? → **Papier découpé** (recommandation acceptée). Joyeux et lisible, tout dessiné par le code.
- **DA-Q2** Quelles palettes de couleurs ? → **Jour franc** (recommandation acceptée). Vive et la plus sûre pour les daltoniens.
- **DA-Q3** Quelle disposition de l’interface ? → **Pouce** (recommandation acceptée). Tout en bas, à portée du pouce.
- **DA-Q4** Une deuxième série de variantes avant de lancer le dessin ? → **Oui, une deuxième série** (recommandation acceptée). Je montre des variantes de ville, d’unités et de brouillard dans le style retenu.

## Idées gardées pour la première version

### Game design (25)

**Boucle de jeu**

- GD-002 · Plusieurs façons de gagner : On gagne en prenant toutes les capitales, au score, ou plus tard par un objectif propre à son clan.
- GD-004 · Une seule monnaie pour tout : Une seule ressource paie les unités, les bâtiments et les avancées, avec le gain du prochain tour affiché.
- GD-005 · S'étendre en ralliant des villages : Des villages neutres parsèment la carte ; une unité qui s'y arrête un tour en fait une de ses villes.
- GD-006 · Brume et lieux à découvrir : La carte est cachée ; ruines, sources et sommets récompensent ceux qui les trouvent les premiers.
- GD-007 · Combat annoncé avant d'attaquer : Avant de frapper, on voit les dégâts qu'on va donner et recevoir.
- GD-010 · Tours restants et objectifs affichés : On voit combien de tours il reste et où en est chacun de son objectif.
- GD-011 · Nombre d'unités limité par ville : Chaque ville entretient autant d'unités que sa taille, pas plus.
- GD-013 · Ville qui grandit : deux cadeaux, un seul : À chaque croissance d'une ville, on choisit une récompense parmi deux.
- GD-014 · Tours rapides d'une seule main : Toucher une unité, voir où elle peut aller, toucher pour agir ; la fin de tour signale les unités oubliées.
- GD-015 · Taille de la carte et nombre de joueurs : Fixer la plus grande carte et le nombre maximal de joueurs ; le choix est posé dans les questions.
- GD-016 · Brouillard sur la carte ou non : Décider ce que chaque joueur voit de la carte ; le choix est posé dans les questions.

**Arbre d'évolution**

- GD-020 · Arbre en trois branches : force, croissance, savoir : Seize avancées en trois colonnes, avec un choix exclusif dans chaque branche et une merveille au bout.
- GD-025 · Avancées qui en ferment une autre : À certains paliers, prendre une avancée ferme l'autre pour toute la partie.
- GD-026 · Chaque clan démarre avec sa propre avancée : Une avancée propre au clan est acquise dès le premier tour.
- GD-027 · Merveilles uniques au bout des branches : Au bout de chaque branche, une merveille que seul le premier qui la bâtit obtient.

**Clans**

- GD-040 · Crocs-Rouges, le clan qui attaque : Des pillards qui frappent tôt et gagnent des ressources à chaque victoire, mais s'essoufflent s'ils échouent.
- GD-041 · Murailliers, le clan qui se défend : Des bâtisseurs de forts dont le territoire ralentit et use les envahisseurs.
- GD-043 · Veilleurs d'Astres, le clan savant : Des savants qui paient moins leurs avancées et copient celles de leurs voisins.
- GD-044 · Vents-Errants, le clan explorateur : Des voyageurs rapides qui voient loin et tirent plus des ruines et des découvertes.

**Différences et équilibre**

- GD-070 · Clans de plus en plus différents, par étapes : D'abord un bonus de départ et une unité propre, puis une règle propre, enfin un objectif propre.
- GD-071 · Attaque, économie, défense : chacun bat l'autre : L'attaquant bat l'économe, l'économe bat le défenseur, le défenseur bat l'attaquant.
- GD-072 · Lances, cavaliers, archers : chacun bat l'autre : Les lances battent les cavaliers, les cavaliers les archers, les archers les lances ; plus des engins de siège et des navires.
- GD-073 · Équilibrer les clans en simulant des parties : L'ordinateur joue des milliers de parties entre clans, et on ajuste les chiffres jusqu'à ce qu'aucun ne domine.
- GD-077 · Clans décrits par une simple fiche : Chaque clan est une fiche de chiffres et de capacités, ce qui permet d'ajouter ou de régler un clan facilement.

**Première version**

- GD-090 · Première version jouable : le socle : Carte moyenne, quatre clans, arbre en trois branches, victoire au score au trentième tour ou par conquête.

### IA (9)

**Les bases**

- IA-001 · Un terrain d'essai pour mesurer les IA : Les IA jouent seules des milliers de parties entre elles, et chaque version reçoit une note de force pour savoir si elle progresse vraiment.
- IA-002 · Une première IA simple et rapide : L'IA note chaque coup possible selon quelques critères, comme le danger ou la valeur d'une case, et joue le meilleur.
- IA-003 · Une IA déjà entraînée avant la sortie : Avant la sortie, l'IA se perfectionne seule sur l'ordinateur en jouant des millions de parties et en gardant les meilleurs réglages.
- IA-004 · Une nouvelle IA doit battre l'ancienne : Toutes les versions sont gardées ; une nouvelle n'est publiée que si elle bat celle en service et toutes les précédentes.

**Niveaux et caractère**

- IA-006 · Des niveaux selon le temps de réflexion : Le niveau choisi règle combien de temps l'IA réfléchit : facile, elle réfléchit peu ; difficile, beaucoup.
- IA-007 · Un adversaire à votre mesure : Le jeu propose l'IA qui vous laisse environ une chance sur deux de gagner, et vous pouvez toujours choisir vous-même.
- IA-008 · Chaque clan a son caractère : Chaque clan joue à sa façon : l'un attaque tôt, l'autre bâtit, un autre explore.
- IA-009 · L'IA devine votre style de jeu : Pendant la partie, l'IA repère si vous attaquez, bâtissez ou explorez, et adapte sa réponse.

**Une IA plus forte**

- IA-005 · L'IA prépare son tour entier : À chaque tour, l'IA essaie plusieurs enchaînements de coups pour toutes ses unités et garde le meilleur.

### Données (7)

**Garder ses parties**

- DO-001 · Une partie enregistrée en tout petit : On garde le point de départ et la liste des coups : environ 2 Ko par partie, de quoi la rejouer à l'identique.
- DO-002 · Le carnet de toutes vos parties : Le téléphone garde chaque partie jouée, même sans réseau.
- DO-003 · Envoyer ses parties en un fichier : Un bouton crée un fichier de vos parties à partager où vous voulez.
- DO-004 · Copie automatique sur votre Google Drive : Vos parties arrivent seules dans un dossier de votre Drive, où l'ordinateur peut les relire.

**Mesurer les progrès de l'IA**

- DO-005 · Tournois entre versions de l'IA : Des milliers de parties entre l'ancienne et la nouvelle IA, jouées automatiquement en ligne, sans aucun joueur.
- DO-006 · Une note de niveau pour chaque IA : Chaque version d'IA reçoit une note, comme un classement aux échecs, adaptée aux parties à 3 ou 4.
- DO-007 · Les courbes de l'IA sur une page web : Une page gratuite, mise à jour après chaque tournoi, qui s'ouvre sur le téléphone.

### Veille (20)

**Lisibilité et équité**

- VE-003 · Combat sans dé, résultat annoncé : Le résultat d'une attaque se calcule d'avance et s'affiche avant d'attaquer.
- VE-004 · Annuler ses coups tant que rien n'est révélé : On revient librement en arrière tant que rien de caché n'apparaît, plus une remise à zéro du tour par partie.

**Modes de jeu**

- VE-009 · Défi du jour ou de la semaine : Chaque jour ou chaque semaine, tout le monde joue exactement la même partie et compare son score.
- VE-011 · Jouer à plusieurs sur un téléphone : On se passe le téléphone ; un écran cache le jeu du précédent ; l'IA tient les places vides.

**Parties courtes et victoire**

- VE-015 · Parties à nombre de tours fixe : On vise le meilleur score en un nombre de tours connu d'avance, ou on gagne vite par conquête.
- VE-016 · Gagner par la capitale ou les points : On gagne en prenant la capitale adverse ou en atteignant un seuil de points.
- VE-021 · Fin de partie plus courte : Limite de tours, victoire aux points, ou abandon automatique d'un camp trop en retard.

**Cartes et brouillard**

- VE-024 · Des départs équitables garantis : Le jeu vérifie que chaque joueur commence avec autant de ressources et de place.
- VE-026 · Toute la carte sur l'écran : Les petites cartes tiennent sur l'écran sans faire défiler.

**Partage, replays, classements**

- VE-040 · Un code pour partager une carte : Un code court permet à un ami de jouer exactement la même carte.
- VE-041 · Revoir une partie coup par coup : On rejoue une partie terminée en avant et en arrière.

**IA visible**

- VE-045 · Regarder des IA s'affronter : On lance une partie entre IA et on la regarde en accéléré.
- VE-046 · Niveaux qui jouent comme des humains : Chaque niveau joue comme un joueur d'un certain niveau, sans avantage caché.
- VE-047 · Avantages de l'IA affichés : Si un niveau donne un avantage à l'IA, le jeu le dit clairement.
- VE-048 · L'IA montre ce qu'elle a appris : Le jeu affiche combien de parties l'IA a jouées et comment elle a progressé.
- VE-049 · L'IA voit ce que vous voyez : L'IA a exactement les mêmes informations et règles que le joueur, brouillard compris.

**Fidélisation et saisons**

- VE-061 · Règles décrites dans des fichiers : Les clans, unités et valeurs sont décrits dans des fichiers, pas dans le code.

**Modèle économique et distribution**

- VE-064 · Ne jamais vendre de puissance : Rien de payant qui avantage : ni puissance, ni jeu en ligne, ni aide, ni publicité imposée.
- VE-065 · Tout gratuit, sans pub ni achat : Le jeu entier est gratuit, avec au plus un lien de soutien hors du jeu.
- VE-069 · Livrer l'APK directement : Le fichier à installer est publié sur GitHub, signé toujours de la même clé.

### Direction artistique (4)

**Styles graphiques**

- DA-003 · Papier découpé : Formes de papier coloré superposées, bords irréguliers, ombres douces, pions en gommettes.

**Palettes de couleurs**

- DA-006 · Jour franc, couleurs vives : Couleurs vives et nettes sur fond crème, six couleurs de clans bien distinctes.
- DA-007 · Nuit, fond sombre : Fond presque noir, terrains sombres, clans en couleurs claires, pour jouer le soir.

**Interface**

- DA-010 · Pouce : tout en bas de l’écran : Actions et « Fin du tour » dans le tiers bas ; un panneau monte pour la ville.

## Idées jetées

- GD-017 · Combats avec ou sans part de chance
- GD-054 · Masques, le clan de la ruse
- GD-057 · Glaneurs, le clan des récupérateurs
- IA-019 · Les téléphones entraînent l'IA eux-mêmes
- DO-017 · Publier les parties pour les chercheurs
- VE-066 · Clans en plus payants une fois
- VE-067 · Essai gratuit puis achat unique
- DA-001 · Carnet d’explorateur, encre et lavis
- DA-002 · Pixel 16, gros pixels
- DA-004 · Plateau de bois
- DA-005 · Table tactique lumineuse
- DA-009 · Plein soleil, contraste maximal

## Points à trancher

- **Victoire** : GD-001 (partie limitée en tours, gagnée au score) a été mise en « plus tard », mais la réponse VE-Q2 (score en tours fixes) et le socle GD-090 (score au trentième tour ou conquête) la supposent. Question posée à l'utilisateur le 2026-11-27.
- **Carte entière à l'écran (VE-026)** : D-20 (départ zoomé sur ses unités) prévaut ; la carte entière reste visible en dézoomant.
- **GD-017 jetée** : c'était la fiche-question sur le hasard des combats ; la réponse GD-Q8 (aucune chance) tient, comme VE-003 (combat sans dé).
