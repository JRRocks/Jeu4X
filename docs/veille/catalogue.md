# Catalogue d'idées de la veille (session 5)

> Écrit par Claude, session 5, le 2026-11-27. Branche `s5-veille`.
> Consolide quatre notes de veille : 4X légers et Polytopia ; tactique, plateau numérique et roguelites ; tendances et modèles économiques ; IA qui apprend et systèmes de 4X.
> Une idée vue dans plusieurs jeux n'a qu'une fiche, qui cite tous les jeux d'origine. Les numéros `VE-` ne seront jamais réutilisés : l'utilisateur trie d'un mot (« garde VE-009 »).
> Rien n'est repris des univers étudiés (noms, peuples, unités, dessins) : on ne garde que des principes de jeu (D-03). Sources : numéros `[S-xxx]` de `sources.md`. Fiches des jeux : `fiches-jeux.md`. Tendances du marché : `tendances.md`.

**80 idées** : 71 fiches développées (VE-001 à VE-071), 7 renvois à la session 2 (VE-072 à VE-078), 2 renvois à la session 3 (VE-079 et VE-080). Les fiches marquées « → IA (session 3) » concernent l'IA : elles décrivent ce que le joueur voit, pas la technique.

Adéquation : forte / moyenne / faible. Coût : S (petit), M (moyen), L (gros).

## Tableau résumé

| N° | Titre | L'idée en une ligne | Intérêt | Adéq. | Coût |
|---|---|---|---|---|---|
| **Lisibilité et équité** | | | | | |
| VE-001 | Intentions adverses affichées | L'adversaire montre ce qu'il prépare (cible, menace) avant de le jouer | Partie lisible, IA perçue comme juste | forte | M |
| VE-002 | Aperçu de l'effet avant de valider | Un appui long montre ce que l'action va changer | Remplace un long tutoriel, évite les erreurs de doigt | forte | M |
| VE-003 | Combat sans hasard, issue annoncée | Le résultat d'une attaque se calcule et s'affiche avant d'attaquer | Lisible, reproductible, simple pour l'IA | forte | S |
| VE-004 | Annuler ce qui ne révèle rien | Retour arrière libre tant que rien de caché n'est dévoilé, plus une remise à zéro de tour par partie | Indispensable au toucher | forte | S |
| VE-005 | Comportement adverse lisible d'un signe | Couleur ou forme indique comment une unité adverse agit | On apprend l'ennemi sans lire | forte | S |
| VE-006 | La clarté d'abord | Ce qu'on ne sait pas montrer simplement est simplifié ou coupé | Garde le jeu lisible en grandissant | forte | S |
| VE-007 | Parties personnalisées équitables | Partie miroir (même clan pour tous) et exclusion d'un clan ou d'une unité | Désamorce les disputes d'équilibre | moyenne | S |
| **Règles contre l'effet boule de neige** | | | | | |
| VE-008 | Trois freins à l'emballement | Entretien des unités, coût croissant des villes, budget d'actions par tour | Le joueur en tête ne gagne pas d'office | moyenne | S/M |
| **Modes de jeu** | | | | | |
| VE-009 | Défi à graine commune | Chaque jour ou chaque semaine, la même partie pour tous, score affiché | Rendez-vous régulier presque gratuit grâce au moteur reproductible | forte | S (local) / M |
| VE-010 | Énigmes tactiques du jour | Une position fixe à résoudre, série de jours, course chronométrée en bonus | Format très court pour revenir chaque jour | forte | M |
| VE-011 | Passe-et-joue | Plusieurs joueurs sur un téléphone, écran de passage, sièges vides tenus par l'IA | Multijoueur sans serveur | forte | S/M |
| VE-012 | Multijoueur asynchrone | Chacun joue quand il veut : par fichier de partie, ou en ligne avec délai long | Le format le plus adapté au téléphone entre amis | moyenne | M / L |
| VE-013 | Expédition de mini-parties | 4 à 6 petites parties enchaînées sur une carte à embranchements | Progression sans allonger chaque partie | moyenne | M/L |
| VE-014 | Scénarios à règle changée et règles à la carte | Scénarios fixes avec une règle modifiée, puis modificateurs au choix | Contenu bon marché, sert aussi de tests pour l'IA | forte | S/M |
| **Parties courtes et victoire** | | | | | |
| VE-015 | Modes notés à durée bornée | Score en un nombre de tours fixe, ou conquête notée selon la rapidité | Parties courtes garanties, mesure simple pour l'IA | forte | S |
| VE-016 | Victoire par capitale ou par seuil de points | Premier à un seuil de points, ou prise de la capitale adverse | Une autre voie que l'élimination totale | forte | S |
| VE-017 | Score d'accomplissements à plusieurs voies | Explorer, bâtir, vaincre rapportent des points ; jalons visibles par voie | Plusieurs styles peuvent gagner | forte | M |
| VE-018 | Lieux clés de la carte | Lieux spéciaux dévoilés un à un ; tenir la majorité peut faire gagner | Rythme le début, force à choisir ses fronts | moyenne | S/M |
| VE-019 | Objectifs qui surgissent | En cours de partie, un objectif de carte ou personnel apparaît | Casse les parties jouées par cœur | moyenne | M |
| VE-020 | Objectifs bonus par carte | 1 ou 2 objectifs facultatifs par carte | Rejouabilité à très bas prix | forte | S |
| VE-021 | Fin de partie raccourcie | Limite de tours, victoire aux points ou reddition d'un camp trop en retard | Corrige la critique la plus universelle du genre | forte | S |
| VE-022 | Défaite par jauge commune | On perd quand une jauge partagée (population, moral) tombe à zéro | Dilemmes « protéger le village ou l'armée » | moyenne | S/M |
| **Cartes et brouillard** | | | | | |
| VE-023 | Familles de relief | Terres pleines, lacs, continents, archipel… tailles selon le nombre de joueurs | Variété immédiate avec un seul générateur | forte | M |
| VE-024 | Départs équilibrés garantis | Placer les départs d'abord, contrôler, refaire la carte si besoin | Sur petite carte, un mauvais départ décide de tout | forte | M |
| VE-025 | Cartes notées par tournois d'IA | Faire jouer les IA sur chaque graine et écarter les injustes | Les IA servent aussi de contrôle qualité (→ IA) | forte | M |
| VE-026 | Carte entière sur un écran | Taille de carte qui tient sans défilement | Rien de caché hors de l'écran | forte | S |
| VE-027 | Brouillard qui se referme | Ce qu'on ne voit plus redevient incertain ; attaques à couvert | Tension, exige une IA qui ne triche pas | forte | S |
| **Événements et diplomatie** | | | | | |
| VE-028 | Événements à deux ou trois choix | Un événement lié à la situation, effet montré avant de choisir | Événements à un geste, adaptés au téléphone | forte | M |
| VE-029 | Directeur de rythme | Un régisseur invisible dose calme et temps forts, sans toucher à la force de l'IA | Parties courtes mais rythmées | moyenne | M |
| VE-030 | Décisions irréversibles mais amendables | Certaines décisions sont définitives, une suivante peut les nuancer | Donne du poids aux choix | moyenne | M |
| VE-031 | Villages neutres à rallier | Un village neutre se rallie par une petite condition, sinon par la force | Voie pacifique à côté de la conquête | moyenne | M |
| VE-032 | Diplomatie à trois verbes | Paix, alliance, rupture ; chaque demande coûte ; rompre a un prix visible | Tient sur un écran de téléphone | forte | M |
| VE-033 | Opinion honnête, souvenirs, parole tenue | L'humeur de chaque clan IA est affichée, expliquée, et ses promesses tenues | Les revirements paraissent justes | forte | M |
| **Premier contact, tutoriel, accessibilité** | | | | | |
| VE-034 | Tutoriel en chapitres et petits niveaux | Une mécanique par chapitre de 2 minutes ; premier tour jouable sans explication | Manque reproché à Polytopia | forte | M |
| VE-035 | Actions utiles en courte liste | En début de tour, quelques choix utiles proposés | Lisibilité sur petit écran | moyenne | M |
| VE-036 | Difficulté changeable, jusqu'au mode libre | Niveau changeable sans pénalité ; mode sans score ni défaite | Accessibilité, premier contact sans peur | forte | S |
| VE-037 | Tour adverse instantané | Le tour de l'IA ne fait jamais attendre ; avance rapide, combat rapide | Reproche à part entière des grands 4X | forte | S/M |
| VE-038 | Hors ligne, deux orientations, reprise immédiate | Tout le solo sans réseau, portrait et paysage, sauvegarde à chaque tour | Attendu sur téléphone, sessions interrompues | forte | S/M |
| VE-039 | Batterie économe | Pas de rafraîchissement continu, IA sobre | Critique de jeux de plateau numériques | forte | S |
| **Partage, replays, classements** | | | | | |
| VE-040 | Code de graine partageable | Un code court pour rejouer exactement la même carte | Social sans serveur, utile au debug et à l'IA | forte | S |
| VE-041 | Rediffusion d'une partie | Revoir une partie terminée coup par coup | Gratuit avec un moteur reproductible | forte | S |
| VE-042 | Résumé partageable sans dévoiler | Une petite grille à copier qui raconte la partie sans la solution | Diffusion gratuite, aucun serveur | forte | S |
| VE-043 | Succès et défis liés au style | « Gagner sans armée », « gagner en moins de 15 tours »… | Pousse à essayer d'autres stratégies | forte | S |
| VE-044 | Classement en ligne vérifiable | Scores vérifiés par rejeu, paliers, saisons partielles, abandon moins coûteux | Compétition, mais demande un serveur | faible | M/L |
| **IA visible** | | | | | |
| VE-045 | Spectateur IA contre IA | Regarder des IA s'affronter en accéléré | Montre les progrès de l'IA, vitrine du projet | forte | S |
| VE-046 | Niveaux « joueur humain » plutôt que bonus | Chaque niveau joue comme un joueur d'un certain niveau, sans avantage caché | Répond à « le niveau max est de la triche ou trop facile » | forte | M |
| VE-047 | Bonus affichés | Si un niveau avantage l'IA, l'écran de lancement le dit | Supprime le soupçon de triche | forte | S |
| VE-048 | L'IA montre ce qu'elle a appris | « Cette IA a joué N parties », date de mise à jour, courbe ; ne rien promettre d'invisible | Rend visible l'originalité du jeu | forte | S/M |
| VE-049 | Même information que le joueur | L'IA voit ce que voit le joueur, brouillard compris, sans exception | Principal motif de colère contre les IA apprises | forte | S |
| VE-050 | Ciblage juste, agressivité réglable | Les IA ne se liguent pas d'office contre l'humain ; elles visent le joueur en tête | Plainte récurrente | moyenne | S |
| VE-051 | Erreurs crédibles, IA professeur | Aux bas niveaux, l'IA se trompe comme un débutant et montre les bons réflexes (→ IA) | Les erreurs grossières se repèrent et se détestent | forte | M |
| VE-052 | Personnalités qui se lisent dans les actes | Chaque clan IA a un caractère visible dans ses choix (→ IA) | Variété, IA moins uniforme | forte | M |
| VE-053 | Mémoire de l'adversaire | Un chef IA se souvient des parties précédentes contre le joueur | Histoires sans scénario | moyenne | M |
| VE-054 | Comportements débloqués au fil des parties | L'IA dévoile peu à peu de nouvelles tactiques, jamais en réaction à une défaite | Impression d'apprentissage dès le début | moyenne | S |
| VE-055 | Rival fantôme | Une IA imite le style d'un joueur et devient un adversaire hors ligne (→ IA) | Concrétise « l'IA progresse avec les joueurs » | moyenne | L |
| VE-056 | IA mise à jour, défi « trouve la faille » | Nouvelles versions périodiques ; les victoires par ruse nourrissent la suivante (→ IA) | Une IA figée est vite « résolue » | forte | M |
| VE-057 | Partenaire d'entraînement | Une IA un peu plus forte aide le joueur à progresser (→ IA) | Usage le plus réussi des IA apprises livrées | faible | L |
| **Fidélisation et saisons** | | | | | |
| VE-058 | Progression sans puissance | Échelle de difficulté débloquée par la victoire, variantes de départ à gagner | Rejouer sans fausser l'équilibre | forte | S/M |
| VE-059 | Saisons légères | Un mode ou une règle de saison par mois, sans passe payant | Nouveauté sans gros contenu | moyenne | M |
| VE-060 | Éditeur de cartes | Créer et partager des cartes, en fichier ou en code | Contenu par les joueurs | faible | L |
| VE-061 | Règles et clans dans des fichiers de données | Valeurs, unités, clans décrits hors du code | Équilibrage et entraînement de l'IA plus simples | forte | M |
| VE-062 | Aucune notification par défaut | Au plus un rappel de défi, désactivé par défaut | Respect du joueur | moyenne | S |
| VE-063 | Communauté minimale | Page de projet et discussions du dépôt d'abord, salon de discussion plus tard | Réaliste pour un projet personnel | moyenne | S |
| **Modèle économique et distribution** | | | | | |
| VE-064 | Règle : rien qui donne de la force ne se vend | Ni puissance, ni jeu en ligne, ni aide payants ; aucune publicité imposée | Les critiques « payer pour gagner » sont les plus constantes | forte | S |
| VE-065 | Tout gratuit, sans pub ni achat | Jeu entier gratuit, code public, lien de soutien hors du jeu | Aucun code de paiement, aucune critique possible | forte | S |
| VE-066 | Clans supplémentaires payants une fois | Quelques clans gratuits, d'autres achetables (ou gagnables en jouant) | Modèle bien accueilli chez Polytopia | moyenne | M |
| VE-067 | Essai gratuit puis achat unique | Premières cartes gratuites, le reste en un achat | Premium sans barrière d'entrée | moyenne | M |
| VE-068 | Engagement affiché | « Sans pub, sans achat, sans IA générative » sur la page du jeu | Répond à la méfiance des joueurs | forte | S |
| VE-069 | APK direct, distribution limitée | Releases GitHub signées d'une clé stable ; compte Android « distribution limitée » le moment venu | Suffit pour l'utilisateur et ses proches | forte | S |
| VE-070 | Autres vitrines | itch.io, F-Droid, Google Play, chacune à son heure | Public plus large si un jour souhaité | faible | S/M |
| VE-071 | Version PC ou navigateur | Même moteur de règles, autre affichage | Grand écran, entraînement de l'IA | moyenne | M |
| **Renvois à la session 2 (→ GD)** | | | | | |
| VE-072 à VE-078 | Clans, arbre, unités | Voir la section finale | — | — | — |
| **Renvois à la session 3 (→ IA)** | | | | | |
| VE-079, VE-080 | Architecture d'apprentissage | Voir la section finale | — | — | — |

## Top 10 recommandé

Critères : exploite le moteur reproductible, coûte peu, sert l'originalité, corrige les critiques de Polytopia (payer pour gagner, IA faible ou tricheuse, fin de partie longue, conquête seule victoire), n'exige pas de serveur au départ.

1. **VE-009 Défi à graine commune** : avec « même graine, même partie », un défi daté hors ligne ne coûte presque rien et donne une raison de revenir.
2. **VE-015 Modes notés à durée bornée** : un score en N tours garantit des parties courtes et fournit à l'IA une mesure simple pour s'entraîner.
3. **VE-016 Victoire par capitale ou par seuil de points** : deux règles de quelques lignes qui cassent le « seule la conquête paie » et raccourcissent les fins de partie.
4. **VE-004 Annuler ce qui ne révèle rien** : indispensable au toucher, presque gratuit si le moteur garde l'historique, et c'est le premier reproche fait à Root.
5. **VE-049 Même information que le joueur** : règle de conduite gratuite qui coupe court au soupçon de triche, premier motif de colère contre les IA apprises.
6. **VE-046 Niveaux « joueur humain » plutôt que bonus** : répond directement à « le niveau max est de la triche ou trop facile » et c'est le terrain naturel de notre IA qui apprend.
7. **VE-048 L'IA montre ce qu'elle a appris** : rend visible notre originalité ; aucun jeu trouvé ne le fait, et ne rien promettre d'invisible évite l'échec de Hello Neighbor.
8. **VE-045 Spectateur IA contre IA** : quasi gratuit avec un moteur sans écran, il montre les progrès de l'IA et sert de vitrine.
9. **VE-040 Code de graine partageable** : social sans serveur, et le même code sert au debug et à l'entraînement.
10. **VE-064 Règle : rien qui donne de la force ne se vend** : une décision, pas du code, qui ferme la critique la plus constante faite à Polytopia.

Juste derrière : VE-041 (rediffusion, gratuite avec le moteur), VE-021 (fin de partie raccourcie), VE-024 (départs équilibrés), VE-003 (combat sans hasard).

---

# Fiches

## Lisibilité et équité

<a id="ve-001"></a>

### VE-001 · Intentions adverses affichées
- **Jeux d'origine** : Into the Breach ; Cicero (IA de Diplomacy honnête sur ses intentions) ; principe « une bonne IA est prévisible » (Jon Shafer).
- **Description** : avant la fin de son tour, l'adversaire laisse voir ce qu'il prépare : case menacée, flèche vers une cible, ou, à l'échelle d'un 4X, un indice (« troupes massées vers ta frontière »). L'IA s'engage sur ce coup. Le degré d'information peut dépendre du niveau.
- **Pourquoi** : supprime la frustration du hasard caché, fait du tour une petite énigme, rend l'IA « juste » sur un petit écran.
- **Adéquation** : forte (une version partielle « menace sur cette case » suffit pour un 4X). **Coût** : M.
- **Dépendances** : moteur qui sépare « intention » et « résolution » (S1) ; comportement de l'IA (S3) ; affichage (S6).
- **Sources** : [S-107] [S-113] [S-212] [S-220]

<a id="ve-002"></a>

### VE-002 · Aperçu de l'effet avant de valider
- **Jeux d'origine** : Reigns (glisser la carte à moitié montre les jauges touchées) ; Into the Breach.
- **Description** : un appui long ou un geste à moitié montre ce que l'action va changer (ressources, dégâts, ville menacée) avant de valider.
- **Pourquoi** : remplace une bonne part du tutoriel et rassure sur un téléphone où l'on touche vite.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : moteur capable de simuler une action sans l'appliquer (S1) ; aperçu visuel (S6) ; geste à vérifier sur le téléphone.
- **Sources** : [S-156] [S-107]

<a id="ve-003"></a>

### VE-003 · Combat sans hasard, issue annoncée
- **Jeu d'origine** : Ozymandias (combats résolus par comparaison de puissance).
- **Description** : l'issue d'une attaque se calcule à partir des forces en présence et s'affiche avant d'attaquer ; pas de dé.
- **Pourquoi** : lisible, reproductible, simple à évaluer pour l'IA ; se combine avec VE-002.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : règles de combat (S2 pour les valeurs) ; moteur (S1).
- **Sources** : [S-088] [S-089]

<a id="ve-004"></a>

### VE-004 · Annuler ce qui ne révèle rien
- **Jeux d'origine** : Antiyoy et Through the Ages (annulation jusqu'à la fin du tour) ; Root (absence d'annulation reprochée) ; Into the Breach (une remise à zéro de tour par bataille) ; Advance Wars (retour en arrière sur ses tours).
- **Description** : tout coup qui ne dévoile rien (pas de brouillard levé, pas de tirage) s'annule librement jusqu'à la validation du tour ; dès qu'une information nouvelle apparaît, le coup est figé. En plus, une remise à zéro complète du tour par partie.
- **Pourquoi** : pardonne l'erreur de doigt sans tuer la tension ; règle facile à expliquer.
- **Adéquation** : forte. **Coût** : S (si le moteur garde l'historique des coups).
- **Dépendances** : moteur reproductible avec historique (S1).
- **Sources** : [S-030] [S-031] [S-090] [S-096] [S-097] [S-110] [S-121]

<a id="ve-005"></a>

### VE-005 · Comportement adverse lisible d'un signe
- **Jeux d'origine** : Hitman GO (comportement donné par la couleur de la veste) ; Hoplite (chaque type d'ennemi a une portée et un comportement propres).
- **Description** : chaque type d'unité adverse a un comportement fixe signalé par un signe visuel (couleur, forme) : « charge », « garde », « tire de loin ».
- **Pourquoi** : le joueur apprend l'ennemi sans lire de texte.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : direction artistique (S6) ; types d'unités (S2).
- **Sources** : [S-154] [S-114]

<a id="ve-006"></a>

### VE-006 · La clarté d'abord
- **Jeu d'origine** : Into the Breach (les créateurs sacrifient toute idée qui nuit à la clarté).
- **Description** : principe de travail : une règle ou une unité qu'on ne sait pas montrer en une petite animation ou un aperçu est simplifiée ou coupée.
- **Pourquoi** : garde le jeu lisible à mesure qu'on ajoute du contenu ; cohérent avec « règles simples, lisibles sur un téléphone ».
- **Adéquation** : forte. **Coût** : S (discipline, pas code).
- **Dépendances** : à proposer comme décision à la session maître ; concerne S2 et S6.
- **Sources** : [S-107]

<a id="ve-007"></a>

### VE-007 · Parties personnalisées équitables
- **Jeu d'origine** : The Battle of Polytopia (partie miroir ; exclusion de ce qui est jugé déséquilibré, demandée par des joueurs).
- **Description** : en partie personnalisée, option « tous le même clan » et option « exclure tel clan ou telle unité ».
- **Pourquoi** : équité parfaite entre joueurs, disputes d'équilibre désamorcées ; la partie miroir sert aussi à mesurer l'IA.
- **Adéquation** : moyenne. **Coût** : S.
- **Dépendances** : clans (S2) ; mesures de l'IA (S3).
- **Sources** : [S-001] [S-005] (la demande d'exclusion vient de la note sur Polytopia, sans source isolée)

## Règles contre l'effet boule de neige

<a id="ve-008"></a>

### VE-008 · Trois freins à l'emballement
- **Jeux d'origine** : Antiyoy (entretien des unités, revenu négatif = unités perdues) ; Northgard (coût de colonisation croissant) ; Old World (nombre limité d'« ordres » par tour, partagé entre toutes les actions).
- **Description** : trois leviers à essayer séparément : chaque unité coûte par tour ; chaque nouvelle ville coûte plus que la précédente ; un budget d'actions par tour.
- **Pourquoi** : le joueur qui prend de l'avance ne devient pas imbattable (critique faite à Antiyoy et aux grands 4X) ; les tours tardifs restent courts.
- **Adéquation** : moyenne (chacun change l'équilibre ; à choisir avec S2). **Coût** : S à M.
- **Dépendances** : règles (S2) ; mesure par parties d'IA (S3).
- **Sources** : [S-030] [S-031] [S-168] [S-066]

## Modes de jeu

<a id="ve-009"></a>

### VE-009 · Défi à graine commune
- **Jeux d'origine** : The Battle of Polytopia (défis hebdomadaires, même graine, même clan, même carte, 20 tours) ; Slay the Spire (Montée du jour, graine commune et modificateurs, classement) ; Wordle (une grille par jour, la même pour tous) ; Balatro (défi du jour promis, sortie non confirmée).
- **Description** : une graine calculée à partir de la date ; tout le monde joue la même partie, avec éventuellement 2 ou 3 modificateurs de règles. Version quotidienne (petite carte, 10-15 tours, série de jours) et version hebdomadaire. Une tentative comptée, score affiché.
- **Pourquoi** : rendez-vous régulier sans contenu nouveau ; s'appuie directement sur « même graine, même partie » ; fonctionne hors ligne.
- **Adéquation** : forte. **Coût** : S (local, sans classement) / M (avec classement, voir VE-044).
- **Dépendances** : moteur reproductible (S1) ; modificateurs (VE-014).
- **Sources** : [S-009] [S-010] [S-018] [S-127] [S-134] [S-151]

<a id="ve-010"></a>

### VE-010 · Énigmes tactiques du jour
- **Jeux d'origine** : Chess.com (énigme du jour commune, série gardée avec 48 h de grâce ; Puzzle Rush chronométré hors classement) ; Hitman GO (scénarios-énigmes courts).
- **Description** : une position fixe (« gagne en 3 tours », « sauve la capitale ») à résoudre ; série de jours avec délai de grâce ; en bonus, une course de mini-énigmes en 3 minutes, fin à la troisième erreur.
- **Pourquoi** : format très court ; les positions peuvent sortir des parties de l'IA.
- **Adéquation** : forte (course chronométrée : moyenne). **Coût** : M.
- **Dépendances** : moteur capable de charger une position (S1) ; extraction de positions depuis les parties de l'IA (S3, S4).
- **Sources** : [S-148] [S-149] [S-150] [S-154]

<a id="ve-011"></a>

### VE-011 · Passe-et-joue
- **Jeux d'origine** : Antiyoy (jusqu'à 7 sur un appareil) ; Polytopia ; Root (sièges vides tenus par l'IA) ; Ticket to Ride ; Advance Wars (jeu local salué, en ligne pauvre).
- **Description** : plusieurs joueurs sur un téléphone ; écran de passage qui masque la vue du joueur précédent ; sièges vides remplis par l'IA.
- **Pourquoi** : le multijoueur le moins cher, aucun serveur.
- **Adéquation** : forte. **Coût** : S/M.
- **Dépendances** : brouillard par joueur (S1) ; écran de passage (S6).
- **Sources** : [S-030] [S-096] [S-098] [S-103] [S-119]

<a id="ve-012"></a>

### VE-012 · Multijoueur asynchrone
- **Jeux d'origine** : Unciv (échange de fichiers de sauvegarde via un stockage simple) ; Through the Ages (règles adaptées pour réduire l'attente, historique des actions) ; Wingspan (jusqu'à 72 h par tour, chiffre d'une seule critique) ; Ticket to Ride (notification « c'est ton tour ») ; Root ; Freeciv (parties « longturn » à un tour par jour).
- **Description** : deux paliers. (a) Par fichier : chacun reçoit l'état, joue, renvoie ; un simple stockage suffit. (b) En ligne : délai de tour long et fixe, notification. Dans les deux cas, un journal des actions adverses résume ce qui s'est passé pendant l'absence.
- **Pourquoi** : le format le mieux adapté au téléphone pour jouer entre amis.
- **Adéquation** : moyenne. **Coût** : M (fichier) / L (serveur).
- **Dépendances** : moteur reproductible (rejouer les coups) (S1) ; serveur ou stockage (S4) ; attention à la batterie (VE-039).
- **Sources** : [S-026] [S-027] [S-090] [S-100] [S-103] [S-096] [S-042]

<a id="ve-013"></a>

### VE-013 · Expédition de mini-parties
- **Jeux d'origine** : Wargroove 2 (mode Conquête, petites batailles reliées par une carte à nœuds) ; Slay the Spire (carte à embranchements) ; Hoplite (autel d'amélioration entre deux niveaux).
- **Description** : 4 à 6 mini-parties sur une carte à embranchements ; entre deux, on garde ou recrute quelques unités, ou on choisit une amélioration.
- **Pourquoi** : une progression sans allonger chaque partie. Attention : chez Wargroove 2, certains objets cassent l'équilibre.
- **Adéquation** : moyenne/forte. **Coût** : M/L.
- **Dépendances** : mini-cartes (S1) ; contenu (S2).
- **Sources** : [S-123] [S-124] [S-125] [S-114]

<a id="ve-014"></a>

### VE-014 · Scénarios à règle changée et règles à la carte
- **Jeux d'origine** : Root (défis qui changent les règles) ; Balatro (paquets de départ qui modifient une règle plutôt que la puissance ; défis) ; Slay the Spire (mode Personnalisé débloqué par le défi du jour) ; Polytopia (mode Creative).
- **Description** : une liste de scénarios fixes, chacun avec une règle changée (une seule ville, rivières praticables, pas d'unité à distance…) ; puis un mode où le joueur coche les modificateurs qu'il veut.
- **Pourquoi** : contenu bon marché qui renouvelle le jeu ; chaque modificateur est aussi un test pour l'IA ; réutilisé par VE-009 et VE-059.
- **Adéquation** : forte. **Coût** : S/M.
- **Dépendances** : règles paramétrables (S1, VE-061) ; robustesse de l'IA face aux variantes (S3).
- **Sources** : [S-096] [S-128] [S-135] [S-008]

## Parties courtes et victoire

<a id="ve-015"></a>

### VE-015 · Modes notés à durée bornée
- **Jeux d'origine** : The Battle of Polytopia (Perfection : 30 tours, meilleur score ; Domination noté selon les tours, les pertes et la difficulté) ; Marvel Snap (six tours, durée parfois modifiée d'un tour par un lieu).
- **Description** : un mode « N tours, meilleur score » (exploration, croissance, conquête), et une conquête notée selon sa rapidité. Le nombre de tours est annoncé dès le début.
- **Pourquoi** : partie courte garantie, tension de fin ; score comparable entre parties de même graine ; mesure simple pour l'entraînement de l'IA.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : barème de score (S2) ; objectif d'entraînement (S3).
- **Sources** : [S-005] [S-139]

<a id="ve-016"></a>

### VE-016 · Victoire par capitale ou par seuil de points
- **Jeux d'origine** : The Battle of Polytopia (à plusieurs : premier à un seuil de points, ou premier à prendre toutes les capitales) ; Advance Wars (élimination ou prise du QG).
- **Description** : on gagne en atteignant un seuil de points, ou en prenant la capitale adverse, sans devoir éliminer chaque unité.
- **Pourquoi** : répond à « seule la conquête paie » ; une partie peut finir vite sur un coup audacieux.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : règles (S2).
- **Sources** : [S-005] [S-122]

<a id="ve-017"></a>

### VE-017 · Score d'accomplissements à plusieurs voies
- **Jeux d'origine** : Humankind (une seule mesure, la renommée, gagnée par des accomplissements variés) ; Ozymandias (merveilles = seuils atteints) ; Northgard (victoires militaire, savoir, commerce, renommée) ; Civilization VII (quatre voies avec jalons) ; Islanders (seuils de score qui ouvrent une nouvelle zone).
- **Description** : explorer, bâtir, vaincre rapportent des « étoiles » qui forment un score unique ; chaque voie a deux ou trois jalons visibles ; atteindre un palier ouvre un choix.
- **Pourquoi** : une mesure lisible, plusieurs chemins. Attention : chez Humankind, victoire jugée monotone et trop facile si mal dosée.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : barème (S2) ; équilibrage par parties d'IA (S3).
- **Sources** : [S-071] [S-088] [S-168] [S-056] [S-057] [S-160]

<a id="ve-018"></a>

### VE-018 · Lieux clés de la carte
- **Jeu d'origine** : Marvel Snap (trois lieux dévoilés aux tours 1, 2 et 3 ; on gagne en tenant deux lieux sur trois).
- **Description** : quelques lieux spéciaux de la carte révèlent leur effet un par tour au début ; variante de victoire : tenir la majorité de ces lieux à la fin, égalité tranchée par l'écart.
- **Pourquoi** : rythme le début, varie chaque carte, oblige à choisir ses fronts.
- **Adéquation** : moyenne. **Coût** : S/M.
- **Dépendances** : générateur de cartes (S1) ; effets (S2).
- **Sources** : [S-139] [S-140]

<a id="ve-019"></a>

### VE-019 · Objectifs qui surgissent
- **Jeux d'origine** : Northgard (conditions propres à la carte qui apparaissent en cours de partie) ; Old World (« ambitions » proposées selon la façon de jouer).
- **Description** : à mi-partie, un objectif de carte (tenir un lieu, achever un monument) ou un objectif personnel adapté (vaincre N unités, tenir N cases) apparaît et rapporte des points ou la victoire.
- **Pourquoi** : change les plans, donne un cap au débutant, diversifie les victoires.
- **Adéquation** : moyenne/forte. **Coût** : M.
- **Dépendances** : tirage déterministe depuis la graine (S1) ; contenu (S2).
- **Sources** : [S-168] [S-066]

<a id="ve-020"></a>

### VE-020 · Objectifs bonus par carte
- **Jeu d'origine** : Into the Breach.
- **Description** : chaque carte propose 1 ou 2 objectifs facultatifs (garder une ville intacte, gagner en moins de N tours) qui rapportent un bonus.
- **Pourquoi** : rejouabilité, et apprend d'autres façons de jouer.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : règles (S2).
- **Sources** : [S-106] [S-111]

<a id="ve-021"></a>

### VE-021 · Fin de partie raccourcie
- **Jeux d'origine** : critique des grands 4X (fin longue et répétitive, IA qui gère mal la fin) ; critique de Polytopia (fin de partie pauvre).
- **Description** : limite de tours, victoire aux points, ou reddition automatique d'un camp trop en retard.
- **Pourquoi** : la fin de partie longue est la critique la plus universelle du genre.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : règles (S2) ; seuil de reddition mesuré par l'IA (S3).
- **Sources** : [S-063] [S-064] [S-002]

<a id="ve-022"></a>

### VE-022 · Défaite par jauge commune
- **Jeu d'origine** : Into the Breach (la défaite vient du réseau électrique des villes, pas de la perte des unités).
- **Description** : la partie se perd aussi quand une jauge partagée (population, moral) tombe à zéro ; les unités deviennent sacrifiables.
- **Pourquoi** : dilemmes « protéger le village ou l'armée ».
- **Adéquation** : moyenne. **Coût** : S/M.
- **Dépendances** : règles (S2).
- **Sources** : [S-106]

## Cartes et brouillard

<a id="ve-023"></a>

### VE-023 · Familles de relief
- **Jeu d'origine** : The Battle of Polytopia (terres sèches, lacs, pangée, continents, archipel, monde d'eau ; cinq tailles).
- **Description** : un générateur paramétré par famille de relief, avec des tailles graduées selon le nombre de joueurs.
- **Pourquoi** : variété immédiate avec un seul générateur.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : générateur déterministe (S1).
- **Sources** : [S-006] [S-005]

<a id="ve-024"></a>

### VE-024 · Départs équilibrés garantis
- **Jeux d'origine** : critique de Polytopia et de Hexarchy (départs inégaux, hasard du début) ; Age of Wonders III et le mod Better Balanced Maps (départs placés d'abord, contrôles, carte refaite) ; Civilization VII (cartes équilibrées pour le multijoueur).
- **Description** : placer les départs d'abord avec un minimum garanti de ressources et de place et une distance minimale entre joueurs, puis le terrain ; refaire la carte si un contrôle échoue.
- **Pourquoi** : sur une petite carte, un mauvais départ décide de la partie ; utile aussi pour un entraînement d'IA propre.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : générateur (S1) ; contrôle par l'IA (VE-025).
- **Sources** : [S-082] [S-084] [S-086] [S-057] [S-002]

<a id="ve-025"></a>

### VE-025 · Cartes notées par tournois d'IA → IA (session 3)
- **Jeu d'origine** : recherche universitaire sur Planet Wars (faire évoluer des cartes et mesurer leur équilibre par des tournois d'IA).
- **Description** : faire jouer nos IA sur chaque graine candidate et écarter celles où un départ gagne trop souvent.
- **Pourquoi** : nos IA servent deux fois : adversaires et contrôle qualité des cartes.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : moteur rapide sans écran (S1) ; IA (S3) ; stockage des résultats (S4).
- **Sources** : [S-218] [S-219]

<a id="ve-026"></a>

### VE-026 · Carte entière sur un écran
- **Jeux d'origine** : Hoplite (aucun défilement) ; Into the Breach (8 × 8 cases).
- **Description** : choisir les tailles de carte pour qu'elles tiennent sans défilement en portrait, au moins pour les petites parties.
- **Pourquoi** : rien de caché hors de l'écran ; les testeurs de Hoplite saluent l'absence de défilement.
- **Adéquation** : forte. **Coût** : S (choix de conception).
- **Dépendances** : S1 et S6 ; lisibilité des cases à vérifier sur le téléphone.
- **Sources** : [S-114] [S-115] [S-110]

<a id="ve-027"></a>

### VE-027 · Brouillard qui se referme
- **Jeux d'origine** : Age of Wonders 4 (ce qui sort de la vue redevient flou) ; autre jeu cité par la note IA sans fiche (Mad Rocket).
- **Description** : ce qu'on ne voit plus redevient incertain ; le brouillard permet des attaques à couvert.
- **Pourquoi** : ajoute de la tension ; impose que l'IA ne triche pas sur la vision (VE-049).
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : vision par joueur dans le moteur (S1) ; IA sous information partielle (S3).
- **Sources** : [S-080]

## Événements et diplomatie

<a id="ve-028"></a>

### VE-028 · Événements à deux ou trois choix
- **Jeux d'origine** : Reigns (une décision par geste, aperçu des jauges touchées, défaite par excès comme par manque) ; Age of Wonders 4 (événements d'exploration, issues annoncées) ; Old World (événements qui réagissent à l'état de la partie).
- **Description** : découvrir une ruine, un peuple, un lieu, ou une situation de la partie (guerre, proximité) déclenche un petit choix à deux ou trois options, avec l'effet annoncé.
- **Pourquoi** : donne du sel aux premiers tours, se joue d'un geste. Attention : Reigns est critiqué pour des événements trop répétés, Old World pour des événements mal placés en fin de partie.
- **Adéquation** : forte/moyenne. **Coût** : M.
- **Dépendances** : tirage déterministe (S1) ; contenu (S2) ; l'IA doit savoir répondre aux événements (S3).
- **Sources** : [S-156] [S-158] [S-081] [S-069]

<a id="ve-029"></a>

### VE-029 · Directeur de rythme
- **Jeux d'origine** : Left 4 Dead (directeur qui règle calmes et pics) ; Alien: Isolation (directeur qui dose la pression).
- **Description** : un régisseur invisible dose les événements (calme, menace, temps fort) selon l'état de la partie, sans toucher à la force de l'IA adverse.
- **Pourquoi** : parties courtes mais rythmées ; le rythme est distinct de la difficulté.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : doit rester déterministe (S1) ; événements (VE-028).
- **Sources** : [S-187] [S-188] [S-189] [S-185]

<a id="ve-030"></a>

### VE-030 · Décisions irréversibles mais amendables
- **Jeux d'origine** : Frostpunk (livre des lois, choix gris) ; Frostpunk 2 (factions qui réagissent aux lois).
- **Description** : certaines décisions de règne sont définitives ; une décision ultérieure peut les nuancer, pas les effacer. Pas de « bonne réponse ».
- **Pourquoi** : donne du poids aux choix sans tout bloquer.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : règles (S2).
- **Sources** : [S-163] [S-164] [S-167]

<a id="ve-031"></a>

### VE-031 · Villages neutres à rallier
- **Jeu d'origine** : Endless Legend 2 (peuplades neutres pacifiées par la force ou par des quêtes, puis assimilées).
- **Description** : un village neutre se rallie si l'on remplit une condition simple (l'entourer, lui apporter une ressource), sinon par la force.
- **Pourquoi** : alternative pacifique à la conquête.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : règles (S2).
- **Sources** : [S-077] [S-078]

<a id="ve-032"></a>

### VE-032 · Diplomatie à trois verbes
- **Jeux d'origine** : The Battle of Polytopia (paix rompable, ambassade qui révèle les alentours et rapporte un peu ; rupture punie) ; Old World (pas de table de marchandage, missions coûteuses une à la fois) ; Civilization IV (refus d'échange signalé clairement).
- **Description** : paix, alliance, rupture ; chaque demande coûte et prend du temps ; rompre une paix a un prix visible.
- **Pourquoi** : une autre voie que la guerre, qui tient sur un écran ; évite l'IA « distributeur » qu'on exploite. Attention : la diplomatie de Polytopia a été jugée clivante.
- **Adéquation** : forte/moyenne. **Coût** : M.
- **Dépendances** : règles (S2) ; IA de diplomatie (S3).
- **Sources** : [S-011] [S-013] [S-070] [S-062]

<a id="ve-033"></a>

### VE-033 · Opinion honnête, souvenirs, parole tenue
- **Jeux d'origine** : Stellaris (l'opinion affichée est celle que l'IA ressent) ; Old World (souvenirs qui expliquent les revirements) ; Civilization IV ; Cicero (IA honnête sur ses intentions, sans perte de force).
- **Description** : l'humeur de chaque clan IA envers le joueur est affichée et expliquée (« tu as pris notre ville ») ; l'IA ne promet que ce qu'elle compte faire et respecte ses traités, sauf caractère « traître » annoncé. La partie « parole tenue » → IA (session 3).
- **Pourquoi** : les revirements paraissent justes au lieu d'aléatoires ; la diplomatie devient fiable.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : diplomatie (VE-032) ; IA (S3).
- **Sources** : [S-083] [S-070] [S-062] [S-220] [S-222]

## Premier contact, tutoriel, accessibilité

<a id="ve-034"></a>

### VE-034 · Tutoriel en chapitres et petits niveaux
- **Jeux d'origine** : Through the Ages (tutoriel en étapes très salué) ; Root (tutoriel qui explique la logique) ; Wingspan (contre-exemple : une heure sans tout comprendre, selon une critique) ; Antiyoy (campagne de 119 petits niveaux) ; tendance du 4X « casualisé ».
- **Description** : une mécanique par chapitre de 2 minutes, qui explique aussi « pourquoi » ; une suite de petites cartes fixes à difficulté croissante ; le premier tour se joue sans explication.
- **Pourquoi** : le manque de tutoriel est reproché à Polytopia ; le marché pousse vers des 4X lisibles en deux minutes.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : cartes fixes chargeables (S1) ; écrans (S6).
- **Sources** : [S-093] [S-096] [S-101] [S-030] [S-229]

<a id="ve-035"></a>

### VE-035 · Actions utiles en courte liste
- **Jeu d'origine** : Hexarchy (chaque tour commence par une main de cartes qui présente les choix).
- **Description** : au début du tour, les actions utiles sont proposées en quelques choix, sans remplacer le jeu libre.
- **Pourquoi** : lisibilité sur petit écran, apprentissage.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : liste des coups légaux dans le moteur (S1) ; écran (S6).
- **Sources** : [S-085]

<a id="ve-036"></a>

### VE-036 · Difficulté changeable, jusqu'au mode libre
- **Jeux d'origine** : Advance Wars (deux difficultés changeables depuis la carte de campagne sans pénalité) ; Islanders New Shores (mode libre sans score) ; mise en garde contre l'ajustement caché pendant la partie.
- **Description** : le niveau se choisit et se change entre deux parties, sans honte ni pénalité ; un mode sans défaite ni score permet de découvrir.
- **Pourquoi** : accessibilité ; choisir l'adversaire avant la partie plutôt que tricher pendant.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : niveaux d'IA (S3).
- **Sources** : [S-120] [S-161] [S-214]

<a id="ve-037"></a>

### VE-037 · Tour adverse instantané
- **Jeux d'origine** : Advance Wars (avance rapide) ; Root (combat rapide conseillé) ; Civilization VII (attente entre les tours reprochée) ; critique des grands 4X.
- **Description** : le tour de toutes les IA ne fait jamais attendre (la note IA propose un objectif chiffré de moins d'une seconde sur téléphone → IA, session 3) ; animations accélérées ou sautables, option « combat rapide ».
- **Pourquoi** : le temps d'attente est un reproche à part entière.
- **Adéquation** : forte. **Coût** : S (affichage) / M (vitesse de l'IA).
- **Dépendances** : moteur rapide (S1) ; IA (S3).
- **Sources** : [S-121] [S-096] [S-052] [S-064]

<a id="ve-038"></a>

### VE-038 · Hors ligne, deux orientations, reprise immédiate
- **Jeux d'origine** : The Battle of Polytopia (jeu hors ligne, portrait ou paysage) ; tendance des sessions courtes et interrompues.
- **Description** : tout le solo sans réseau ; portrait et paysage ; sauvegarde automatique à chaque tour pour reprendre après une interruption.
- **Pourquoi** : attendu sur téléphone. Le choix du portrait est à tester sur le téléphone, pas à décider sur la foi d'une tendance (aucune enquête fiable trouvée).
- **Adéquation** : forte. **Coût** : S/M.
- **Dépendances** : S1 ; S6 ; vérification sur le téléphone.
- **Sources** : [S-001] [S-003] [S-237]

<a id="ve-039"></a>

### VE-039 · Batterie économe
- **Jeux d'origine** : Through the Ages (gourmand en batterie en ligne ; autonomie et performance de l'IA critiquées).
- **Description** : pas de rafraîchissement continu de l'écran, IA qui calcule sobrement, surtout en ligne.
- **Pourquoi** : seul le téléphone le vérifie, mais un défaut ici fait désinstaller.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : S1 ; S3 ; vérification sur le téléphone.
- **Sources** : [S-091] [S-093]

## Partage, replays, classements

<a id="ve-040"></a>

### VE-040 · Code de graine partageable
- **Jeux d'origine** : demande des joueurs de Polytopia (graines partageables) ; Age of Empires (codes de graine, cité par la note IA) ; Civilization.
- **Description** : chaque carte a un code court qu'on envoie à un ami pour jouer exactement la même carte.
- **Pourquoi** : social sans serveur ; utile aussi au debug et à l'entraînement de l'IA.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : moteur reproductible (S1).
- **Sources** : [S-021]

<a id="ve-041"></a>

### VE-041 · Rediffusion d'une partie
- **Jeu d'origine** : Antiyoy (rediffusions).
- **Description** : rejouer une partie terminée coup par coup, en avant et en arrière.
- **Pourquoi** : gratuit si la partie est reproductible (graine + coups) ; base de VE-012, VE-045 et du debug.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : moteur reproductible (S1).
- **Sources** : [S-030]

<a id="ve-042"></a>

### VE-042 · Résumé partageable sans dévoiler
- **Jeu d'origine** : Wordle (grille de couleurs, idée venue des joueurs).
- **Description** : en fin de partie, une petite grille ou ligne de symboles (tours joués, villes, victoire) à copier, qui ne dévoile pas la solution du défi.
- **Pourquoi** : diffusion gratuite, aucun serveur.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : défi commun (VE-009) ; forme visuelle (S6).
- **Sources** : [S-151] [S-152]

<a id="ve-043"></a>

### VE-043 · Succès et défis liés au style
- **Jeux d'origine** : The Battle of Polytopia (succès et défis ajoutés, longtemps réclamés) ; Balatro (succès et défis pour faire explorer le jeu) ; Google Play Games les fournit.
- **Description** : succès locaux d'abord (« gagner sans armée », « gagner en moins de 15 tours »).
- **Pourquoi** : pousse à essayer d'autres stratégies, ce qui donne aussi à l'IA des parties variées à observer.
- **Adéquation** : forte. **Coût** : S (local) / M (Play Games).
- **Dépendances** : S1 ; collecte de parties (S4).
- **Sources** : [S-003] [S-136]

<a id="ve-044"></a>

### VE-044 · Classement en ligne vérifiable
- **Jeux d'origine** : The Battle of Polytopia (classements, tournois) ; Clash Royale (paliers qu'on ne redescend pas, remise à zéro partielle par saison, réglage changé plusieurs fois) ; Marvel Snap (abandonner tôt coûte moins).
- **Description** : envoyer le score d'un défi vers un classement ; la triche se contrôle en rejouant la partie depuis la graine et les coups ; paliers « plancher », saisons qui ne remettent à zéro qu'une partie du score ; abandon moins coûteux qu'une défaite au bout.
- **Pourquoi** : compétition ; mais demande un service en ligne.
- **Adéquation** : faible à moyenne (pas avant d'avoir des joueurs). **Coût** : M/L.
- **Dépendances** : serveur (S4) ; moteur reproductible (S1).
- **Sources** : [S-023] [S-144] [S-145] [S-147] [S-141]

## IA visible

<a id="ve-045"></a>

### VE-045 · Spectateur IA contre IA
- **Jeu d'origine** : Antiyoy (regarder des IA s'affronter).
- **Description** : lancer une partie entre IA et la regarder en accéléré, éventuellement entre deux générations d'IA.
- **Pourquoi** : montre les progrès de l'IA à l'utilisateur ; vitrine du projet ; presque gratuit avec un moteur sans écran.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : moteur (S1) ; versions d'IA (S3).
- **Sources** : [S-030] [S-031]

<a id="ve-046"></a>

### VE-046 · Niveaux « joueur humain » plutôt que bonus
- **Jeux d'origine** : Maia (IA qui joue comme un humain d'un niveau donné) ; The Battle of Polytopia (niveaux faits surtout de bonus de revenu, niveau max jugé trop facile ; les développeurs disaient préférer une IA plus intelligente) ; Gran Turismo Sophy (adversaire « propre » plutôt que surhumain ; effet des niveaux jugé peu sensible par des joueurs).
- **Description** : chaque niveau est une IA qui joue comme un joueur d'un certain niveau, sans avantage caché ; le niveau se choisit avant la partie. Régler la force d'une IA apprise est un chantier à part (session 3).
- **Pourquoi** : répond au reproche principal fait à Polytopia et à Civilization ; terrain naturel de notre IA qui apprend.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : IA (S3) ; données de parties (S4).
- **Sources** : [S-019] [S-020] [S-201] [S-202] [S-177] [S-178]

<a id="ve-047"></a>

### VE-047 · Bonus affichés
- **Jeux d'origine** : Civilization VI (gros bonus aux niveaux élevés, vus comme une béquille) ; Humankind ; Polytopia.
- **Description** : si un niveau donne un avantage à l'IA, l'écran de lancement le dit en clair.
- **Pourquoi** : supprime le soupçon de triche à peu de frais, même avant que l'IA apprise soit prête.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : S3 ; écran (S6).
- **Sources** : [S-058] [S-059] [S-076] [S-019]

<a id="ve-048"></a>

### VE-048 · L'IA montre ce qu'elle a appris
- **Jeux d'origine** : propre au projet (aucun jeu trouvé qui le fasse) ; contre-exemple Hello Neighbor (IA « qui apprend » promise, déception) ; méfiance des joueurs envers l'IA générative.
- **Description** : afficher « cette IA a joué N parties », la date de sa dernière mise à jour, sa courbe de progression ou sa « génération » ; ne dire « l'IA apprend » que si le joueur peut le constater.
- **Pourquoi** : transforme notre particularité technique en raison de revenir ; argument honnête (adversaire entraîné, pas IA générative). À tester : aucun précédent.
- **Adéquation** : forte. **Coût** : S (compteur) / M (courbe, saisons).
- **Dépendances** : IA (S3) ; suivi des données (S4) ; écran (S6).
- **Sources** : [S-193] [S-194] [S-244]

<a id="ve-049"></a>

### VE-049 · Même information que le joueur
- **Jeux d'origine** : Forza Horizon 6 (IA jugée trop rapide et suspectée de tricher, correctif publié) ; AlphaStar (débat sur la vision globale et la vitesse surhumaine).
- **Description** : l'IA a exactement la même information que le joueur, brouillard compris, et les mêmes règles ; aucune exception.
- **Pourquoi** : principal motif de colère contre les IA apprises ; un comportement « surhumain » visible est vécu comme de la triche.
- **Adéquation** : forte. **Coût** : S (règle de conduite, à inscrire dans l'interface du moteur).
- **Dépendances** : vision par joueur (S1) ; IA (S3).
- **Sources** : [S-173] [S-174] [S-196] [S-197]

<a id="ve-050"></a>

### VE-050 · Ciblage juste, agressivité réglable
- **Jeux d'origine** : critique de Polytopia (IA qui se liguent contre le joueur) ; Hexonia (IA trop agressive) ; Root (IA qui cible le mauvais joueur).
- **Description** : les IA ne s'unissent pas toutes contre l'humain sauf réglage ; elles visent plutôt le joueur en tête ; agressivité réglable en partie personnalisée.
- **Pourquoi** : plainte récurrente, facile à éviter.
- **Adéquation** : moyenne. **Coût** : S.
- **Dépendances** : IA (S3).
- **Sources** : [S-002] [S-033] [S-097]

<a id="ve-051"></a>

### VE-051 · Erreurs crédibles, IA professeur → IA (session 3)
- **Jeux d'origine** : principes de Mick West (erreurs naturelles, jamais grossières) et de Soren Johnson (l'IA amusante joue comme un personnage crédible) ; Humankind (IA qui déroute les nouveaux joueurs).
- **Description** : aux bas niveaux, l'IA se trompe comme un débutant (oubli de défense, expansion lente), jamais par des coups absurdes, et joue « proprement » pour montrer les bons réflexes.
- **Pourquoi** : les joueurs repèrent et détestent les erreurs volontaires évidentes ; une IA déroutante apprend de mauvaises habitudes.
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : IA (S3).
- **Sources** : [S-211] [S-065] [S-074]

<a id="ve-052"></a>

### VE-052 · Personnalités qui se lisent dans les actes → IA (session 3)
- **Jeux d'origine** : Civilization VI (programmes fixes et cachés des dirigeants, critiqués comme « deux lubies ») ; Stellaris.
- **Description** : chaque clan IA a un caractère (bâtisseur, conquérant, prudent…) visible dans ses choix, pas seulement dans une étiquette.
- **Pourquoi** : variété entre les parties ; évite le reproche d'IA qui jouent toujours la même stratégie (fait à Polytopia).
- **Adéquation** : forte. **Coût** : M.
- **Dépendances** : IA (S3) ; clans (S2).
- **Sources** : [S-060] [S-061] [S-083]

<a id="ve-053"></a>

### VE-053 · Mémoire de l'adversaire
- **Jeu d'origine** : Shadow of Mordor / Shadow of War (chefs ennemis qui se souviennent du joueur et montent en grade).
- **Description** : un chef IA se souvient du joueur d'une partie à l'autre (« tu m'as battu deux fois sur la côte ») et adapte une préférence. Ne reprendre que le principe général : un point juridique sur ce système reste à vérifier.
- **Pourquoi** : crée des histoires sans scénario.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : stockage local entre parties (S4) ; IA (S3).
- **Sources** : [S-190] [S-191] [S-192]

<a id="ve-054"></a>

### VE-054 · Comportements débloqués au fil des parties
- **Jeu d'origine** : Alien: Isolation (comportements verrouillés qui se débloquent selon ce que fait le joueur, jamais sur sa mort).
- **Description** : l'IA dévoile de nouvelles tactiques au fil de la partie ou des parties, jamais en réaction à une défaite du joueur.
- **Pourquoi** : donne l'impression d'apprendre avant même que l'IA apprise soit prête ; l'illusion fonctionne même chez ceux qui connaissent le truc.
- **Adéquation** : moyenne. **Coût** : S.
- **Dépendances** : IA (S3) ; à concilier avec VE-048 (ne rien promettre de faux).
- **Sources** : [S-185] [S-186]

<a id="ve-055"></a>

### VE-055 · Rival fantôme → IA (session 3)
- **Jeux d'origine** : Forza (Drivatar, profil d'un joueur qui court dans les parties des autres) ; Real Racing 3 (fantômes pilotés par l'IA) ; Trackmania (fantômes).
- **Description** : une IA entraînée à imiter le style d'un joueur (le sien ou celui d'un ami) devient un adversaire jouable hors ligne.
- **Pourquoi** : concrétise « l'IA progresse avec les joueurs ». Attention : copier les humains copie aussi leurs mauvaises habitudes, et le côté « clone de mes amis » dérange certains.
- **Adéquation** : moyenne. **Coût** : L.
- **Dépendances** : IA (S3) ; données (S4).
- **Sources** : [S-170] [S-171] [S-172] [S-216] [S-217]

<a id="ve-056"></a>

### VE-056 · IA mise à jour, défi « trouve la faille » → IA (session 3)
- **Jeu d'origine** : OpenAI Five Arena (IA figée, failles trouvées et partagées par la communauté, battue par 42 équipes).
- **Description** : livrer de nouvelles versions de l'IA de temps en temps ; un mode où l'on affronte l'IA la plus forte et où l'on peut signaler une partie gagnée par une ruse, qui nourrit l'entraînement suivant.
- **Pourquoi** : une IA figée est vite « résolue » ; les joueurs aiment chercher la faille.
- **Adéquation** : forte (mises à jour) / moyenne (défi). **Coût** : M.
- **Dépendances** : IA (S3) ; collecte des parties (S4) ; livraison de mises à jour (S1).
- **Sources** : [S-199] [S-200]

<a id="ve-057"></a>

### VE-057 · Partenaire d'entraînement → IA (session 3)
- **Jeux d'origine** : Gran Turismo Sophy ; Naruto Mobile (agents appris déployés comme partenaires d'entraînement).
- **Description** : une IA de niveau légèrement supérieur rejoue une partie avec le joueur pour l'aider à progresser.
- **Pourquoi** : usage le plus réussi des IA apprises livrées à ce jour.
- **Adéquation** : faible (pour l'instant). **Coût** : L.
- **Dépendances** : IA (S3) ; rediffusion (VE-041).
- **Sources** : [S-176] [S-209]

## Fidélisation et saisons

<a id="ve-058"></a>

### VE-058 · Progression sans puissance
- **Jeux d'origine** : Slay the Spire (Ascension : niveaux débloqués en gagnant, contraintes cumulatives) ; Balatro (mises, paquets de départ qui changent une règle) ; Wargroove 2 (monnaie qui débloque modificateurs et cartes).
- **Description** : gagner à un niveau débloque le suivant, qui ajoute une contrainte (moins de ressources de départ, IA plus agressive) ; ce qu'on gagne entre les parties, ce sont des variantes, jamais des bonus de force.
- **Pourquoi** : on rejoue pour progresser sans fausser l'équilibre ; les parties restent comparables, ce qui sert l'IA.
- **Adéquation** : forte. **Coût** : S/M.
- **Dépendances** : modificateurs (VE-014) ; stockage local (S1).
- **Sources** : [S-129] [S-130] [S-135] [S-124]

<a id="ve-059"></a>

### VE-059 · Saisons légères
- **Jeux d'origine** : live ops des gros jeux mobiles ; Polytopia (rotation des défis et des apparences).
- **Description** : chaque mois un mode de saison (carte d'archipel, règle spéciale), sans passe de saison payant ; peut coïncider avec une nouvelle version de l'IA (VE-048, VE-056).
- **Pourquoi** : nouveauté sans gros contenu ; tient dans le moteur de règles.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : modificateurs (VE-014) ; livraison régulière (S1).
- **Sources** : [S-225] [S-018]

<a id="ve-060"></a>

### VE-060 · Éditeur de cartes
- **Jeux d'origine** : Antiyoy (éditeur, niveaux de la communauté) ; Polytopia (mode Creative ; éditeur demandé, un éditeur libre non officiel existe) ; Unciv (cartes et mods).
- **Description** : éditeur simple sur téléphone, cartes exportées en fichier ou en code.
- **Pourquoi** : contenu par les joueurs ; mais l'équilibre des cartes est une critique fréquente.
- **Adéquation** : faible pour l'instant. **Coût** : L.
- **Dépendances** : format de carte (S1) ; écran (S6).
- **Sources** : [S-030] [S-008] [S-021] [S-024] [S-026]

<a id="ve-061"></a>

### VE-061 · Règles et clans dans des fichiers de données
- **Jeu d'origine** : Unciv (conçu pour le modding, règles décrites dans des fichiers).
- **Description** : clans, unités et valeurs décrits dans des fichiers de données, pas dans le code.
- **Pourquoi** : équilibrage plus simple, variantes et saisons à bas prix, entraînement de l'IA sur des règles changeantes ; ouvre la porte à la communauté (dépôt public).
- **Adéquation** : forte (architecture) / faible (ouvrir au public tout de suite). **Coût** : M.
- **Dépendances** : architecture du moteur (S1) ; contenu (S2).
- **Sources** : [S-026] [S-028]

<a id="ve-062"></a>

### VE-062 · Aucune notification par défaut
- **Jeux d'origine** : choix de conduite (aucune source chiffrée sur le rejet des notifications) ; Ticket to Ride (notification « c'est ton tour » en asynchrone).
- **Description** : au plus un rappel « nouveau défi », désactivé par défaut ; en asynchrone, une notification de tour que le joueur active.
- **Pourquoi** : respect du joueur ; à ne pas présenter comme un fait mesuré.
- **Adéquation** : moyenne. **Coût** : S.
- **Dépendances** : S1.
- **Sources** : [S-103]

<a id="ve-063"></a>

### VE-063 · Communauté minimale
- **Jeu d'origine** : Polytopia (salon de discussion officiel et forum très actifs, à un tout autre stade).
- **Description** : ne pas ouvrir de salon de discussion tant qu'il n'y a pas de joueurs ; commencer par une page de projet et les discussions du dépôt.
- **Pourquoi** : réaliste pour un projet personnel, sans modération lourde.
- **Adéquation** : moyenne. **Coût** : S.
- **Dépendances** : décision de l'utilisateur (dépôt public, rien de privé).
- **Sources** : [S-018]

## Modèle économique et distribution

<a id="ve-064"></a>

### VE-064 · Règle : rien qui donne de la force ne se vend
- **Jeux d'origine** : critique de Polytopia (peuples payants perçus comme trop forts, jeu en ligne verrouillé derrière un achat) ; Hitman GO (indices payants reprochés) ; étude Deloitte 2025 (52 % arrêteraient un jeu aux pubs intrusives) ; Polytopia (apparences seulement en vente).
- **Description** : décision de conduite : tout ce qui se vendrait un jour resterait cosmétique ; ni puissance, ni jeu en ligne, ni aide ou indice payants ; aucune publicité imposée (au pire une publicité récompensée et optionnelle).
- **Pourquoi** : les critiques « payer pour gagner » et « multijoueur verrouillé » sont les plus constantes ; règles identiques pour tous, donc neutres pour l'IA.
- **Adéquation** : forte. **Coût** : S (décision).
- **Dépendances** : décision de l'utilisateur, à inscrire par la session maître.
- **Sources** : [S-022] [S-002] [S-155] [S-235] [S-018]

<a id="ve-065"></a>

### VE-065 · Tout gratuit, sans pub ni achat
- **Jeux d'origine** : Unciv (gratuit, libre, sur F-Droid) ; Antiyoy (gratuit, sans publicité ni achat) ; itch.io (« payez ce que vous voulez »).
- **Description** : tout le jeu gratuit, code public, pas de boutique ni de compte ; au plus un lien de soutien sur la page du projet, rien dans le jeu.
- **Pourquoi** : cohérent avec un projet personnel et un dépôt public ; aucun code de paiement ; aucune critique « payer pour gagner » possible.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : décision de l'utilisateur.
- **Sources** : [S-028] [S-030] [S-255]

<a id="ve-066"></a>

### VE-066 · Clans supplémentaires payants une fois
- **Jeu d'origine** : The Battle of Polytopia (peuples gratuits et peuples achetables une fois, sans monnaie virtuelle ; des joueurs demandent aussi à les débloquer en jouant).
- **Description** : quelques clans gratuits, d'autres achetables une fois chacun, ou gagnables en jouant ; jamais plus forts.
- **Pourquoi** : modèle bien accueilli ; la critique vient surtout du multijoueur verrouillé et de clans jugés trop forts.
- **Adéquation** : moyenne (paiement, compte Play, comptabilité). **Coût** : M.
- **Dépendances** : VE-064 ; distribution sur Play (VE-070).
- **Sources** : [S-022] [S-003]

<a id="ve-067"></a>

### VE-067 · Essai gratuit puis achat unique
- **Jeux d'origine** : Songs of Conquest mobile (une carte de campagne et une de conquête gratuites, jeu complet en un achat) ; essais limités de Google Play avec progression gardée ; Balatro (payant une fois, succès mobile).
- **Description** : la première partie ou les premières cartes gratuites, le reste en un achat.
- **Pourquoi** : le modèle premium le plus lisible sur mobile.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : paiement et magasin (VE-070).
- **Sources** : [S-104] [S-240] [S-132]

<a id="ve-068"></a>

### VE-068 · Engagement affiché
- **Jeux d'origine** : méfiance des joueurs envers l'IA générative et les pubs (enquêtes 2025-2026) ; Polytopia met en avant l'absence de monétisation agressive.
- **Description** : une phrase sur la page du jeu : « sans pub, sans achat, sans IA générative », et ce que fait vraiment notre IA (adversaire entraîné).
- **Pourquoi** : notre « IA qui apprend » risque d'être confondue avec l'IA générative ; le dire évite le malentendu. Les dessins et textes du jeu : décider explicitement, puis l'afficher honnêtement.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : décisions de l'utilisateur ; S6 pour l'origine des dessins.
- **Sources** : [S-235] [S-244] [S-018]

<a id="ve-069"></a>

### VE-069 · APK direct, distribution limitée
- **Jeux d'origine** : pratique actuelle de l'utilisateur ; vérification obligatoire des développeurs Android.
- **Description** : continuer à livrer l'APK par les Releases GitHub, signé d'une clé stable ; quand la vérification s'appliquera en France, inscrire un compte gratuit « distribution limitée » (au plus 20 appareils).
- **Pourquoi** : suffit pour l'utilisateur et quelques proches ; aucune démarche de magasin.
- **Adéquation** : forte. **Coût** : S.
- **Dépendances** : chaîne de livraison (S1) ; à surveiller (voir `tendances.md`).
- **Sources** : [S-249] [S-251]

<a id="ve-070"></a>

### VE-070 · Autres vitrines
- **Jeux d'origine** : itch.io (accepte les APK ; client libre Mitch pour installer et mettre à jour sans Play) ; Unciv (distribué sur F-Droid) ; Google Play (12 testeurs pendant 14 jours pour un compte personnel récent).
- **Description** : itch.io si l'utilisateur veut des joueurs extérieurs ; F-Droid si le jeu reste entièrement libre (compilation reproductible, aucune dépendance propriétaire), position incertaine face à la vérification ; Google Play seulement pour un public large, après avoir réuni 12 testeurs.
- **Pourquoi** : élargir le public le jour où c'est souhaité.
- **Adéquation** : faible à court terme (itch.io et F-Droid : moyenne). **Coût** : S (itch.io) / M (F-Droid, Play).
- **Dépendances** : décision de l'utilisateur ; S1.
- **Sources** : [S-255] [S-256] [S-028] [S-252] [S-253]

<a id="ve-071"></a>

### VE-071 · Version PC ou navigateur
- **Jeux d'origine** : Google Play Games sur PC (prix unique mobile et PC) ; Polytopia (mobile, Switch, Steam, Epic).
- **Description** : le moteur de règles séparé permet une version de bureau ou de navigateur, pour jouer sur grand écran et pour l'entraînement de l'IA.
- **Pourquoi** : coût réduit grâce à la séparation règles / affichage déjà décidée ; idée pour plus tard.
- **Adéquation** : moyenne. **Coût** : M.
- **Dépendances** : S1 ; S3.
- **Sources** : [S-240] [S-018]

---

## Renvois à la session 2 (→ GD)

Idées de clans, d'arbre ou d'unités trouvées ailleurs : notées ici, non développées.

| N° | Idée | Jeux d'origine | Sources |
|---|---|---|---|
| <a id="ve-072"></a>VE-072 | Clans très asymétriques avec un but commun, chacun favorisé pour une voie de victoire → GD | Endless Legend 2, Polytopia, Root, Northgard | [S-077] [S-096] [S-168] |
| <a id="ve-073"></a>VE-073 | Ampleur du contenu : arbre technologique plus long et fin de partie plus riche (critique de Polytopia), ou au contraire arbre et villes coupés pour la clarté (Into the Breach, à confirmer) ; peu de types d'unités aux règles nettes (Hoplite, Antiyoy) → GD | Polytopia, Into the Breach, Hoplite, Antiyoy | [S-002] [S-109] [S-114] [S-030] |
| <a id="ve-074"></a>VE-074 | Mécanique de milieu marin (refonte navale, cases inondées) → GD | Polytopia | [S-014] |
| <a id="ve-075"></a>VE-075 | Commandants débloquables qui changent le style de jeu → GD | Wargroove 2 | [S-123] |
| <a id="ve-076"></a>VE-076 | Factions internes qui réagissent aux lois et aux techniques → GD | Frostpunk 2 | [S-167] |
| <a id="ve-077"></a>VE-077 | Unité d'infiltration, invisible jusqu'à sa première action (à manier avec précaution : frustration) → GD | Polytopia | [S-011] |
| <a id="ve-078"></a>VE-078 | Caractère des clans IA issu des « gènes » de l'algorithme génétique, avec lignée visible → GD (et session 3) | Creatures | [S-182] [S-183] |

## Renvois à la session 3 (→ IA)

Leçons d'architecture d'apprentissage, sans effet direct sur ce que voit le joueur : notées ici, non développées.

| N° | Idée | Jeux d'origine | Sources |
|---|---|---|---|
| <a id="ve-079"></a>VE-079 | Pré-entraîner plutôt qu'apprendre en direct à partir de quelques actions du joueur (trop peu d'exemples) ; ne laisser au joueur que des réglages lisibles → IA | Black & White | [S-179] [S-181] |
| <a id="ve-080"></a>VE-080 | Entraînement en ligue d'adversaires variés plutôt que contre un seul adversaire → IA | AlphaStar, Naruto Mobile | [S-197] [S-209] |
