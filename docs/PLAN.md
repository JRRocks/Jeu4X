# Plan

> Statuts : ⬜ à faire · 🔄 en cours · ✅ terminé (critère vérifié, vérification écrite au journal) ·
> 📱 fait, à vérifier sur le téléphone · ⛔ bloqué (avec la raison).
> Aucune idée « pour plus tard » ici : elles vont dans `docs/idees-futur.md`.

## But

Poser les fondations d'un jeu de stratégie 4X original pour Android : un plateau de cases repérées comme aux échecs, des déplacements au tour par tour et de petits graphismes. En parallèle, réunir la matière de game design, d'IA, de données, de veille et de direction artistique que l'utilisateur triera.

## Tâches

| N° | Tâche | Critère que l'utilisateur peut observer | Qui | Statut |
|---|---|---|---|---|
| P1.1 | Comparer les technologies, domaine par domaine | Un comparatif court avec une recommandation, présenté par la session maître ; le choix noté dans les décisions | S1 | ✅ D-14 |
| P1.2 | Application vide qui s'installe, avec sa chaîne de livraison | Un `Jeu4X.<version>.apk` qui s'installe et s'ouvre sur le téléphone | S1 | 🔄 essai vérifié à l'émulateur (0.1.6) ; vrai APK : clé à créer |
| P1.5 | Test automatique sur émulateur Android dans la CI, à chaque version (D-19) | Une capture de l'émulateur jointe à chaque pré-version d'essai, troisième ligne « contrôle réussi » | S1 | ✅ essai 0.1.6, exécution 6 |
| P1.3 | Aperçus du plateau, avec variantes | Des images du plateau (cases adressées A1, B2…) ; une variante choisie | S1 | ✅ variante C, adresses internes (D-16) |
| P1.4 | Plateau, déplacements au tour par tour, petits graphismes | Sur le téléphone : toucher une unité, voir où elle peut aller, la déplacer, passer le tour | S1 | 📱 essai 0.1.10 : jouable, cadrage au lancement (D-20) et vue qui suit le tour (D-21), vérifié à l'émulateur ; gestes à voir sur le téléphone |
| P2.1 | Catalogue de game design : arbre d'évolution, clans, styles de jeu | Un catalogue `GD-` à trier | S2 | ✅ 54 idées |
| P3.1 | Étude de l'IA évolutive et de ses besoins envers le moteur | Une étude et un catalogue `IA-` à trier | S3 | ✅ 19 idées |
| P4.1 | Étude du regroupement des données | Un comparatif, une recommandation et un catalogue `DO-` | S4 | ✅ 21 idées |
| P5.1 | Veille : idées venues du monde du jeu vidéo | Un catalogue `VE-` à trier | S5 | ✅ 80 idées |
| P6.1 | Direction artistique : styles graphiques, palettes, interface | Des aperçus en images, plusieurs variantes de chaque | S6 | ✅ 12 propositions |
| P0.1 | Tri des idées avec l'utilisateur, sur une page à cocher (D-13) | Le retenu entre dans les décisions et le plan, le reste dans `docs/idees-futur.md` | maître | ✅ D-22, détail dans `docs/TRI.md` |

## À vérifier sur le téléphone

| N° | Depuis la version | Ce qu'il fait | Ce qu'il doit voir | État |
|---|---|---|---|---|
| R1 | essai 0.1.4 | Installe « Jeu4X essai » depuis le lien de la pré-version `essai-0.1.4`, puis l'ouvre | Trois lignes : « Jeu4X », « Version 0.1.4 », « Moteur de règles 1 : contrôle réussi » ; en cas d'arrêt, l'écran « Jeu4X s'est arrêté » avec Copier et Partager | remplacé par le test à l'émulateur (D-19, P1.5) |
| R2 | essai 0.1.6 | Ouvre « Jeu4X essai » sur son téléphone (vrai processeur ARM) | « Moteur de règles 1 : contrôle réussi » | à essayer, séance groupée |
| R3 | essai suivant | Installe un nouvel essai par-dessus le précédent | Installation sans désinstaller | à essayer, séance groupée |
| R4 | essai 0.1.6 | Sur l'écran « Jeu4X s'est arrêté », touche Copier puis Partager | Le rapport se copie et se partage | à essayer, séance groupée |
| R5 | essai 0.1.10 | Passe le tour plusieurs fois | La vue glisse en douceur vers les unités du joueur suivant | à essayer, séance groupée |
| R6 | essai 0.1.10 | Fait glisser la vue, pince pour zoomer et dézoomer, touche une unité puis une case jaune | Les gestes répondent, la case touchée est la bonne | à essayer, séance groupée |
| R7 | essai 0.1.10 | Joue quelques tours | Les cases et les unités se lisent bien au zoom de départ | à essayer, séance groupée |
