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
| P2.2 | Règles précises de la première version, d'après le tri (D-23, D-28) | Un résumé d'une page, validé par l'utilisateur | S2 | ✅ validé (D-30) |
| P6.2 | Deuxième série d'images : papier découpé en perspective, palette Jour franc et Nuit, interface Pouce (D-27) | Des images avec variantes ; un choix de l'utilisateur | S6 | ✅ choix faits (D-31 à D-33) |
| P3.2 | Terrain d'essai des IA et première IA simple, sans rien d'affiché (D-24) | Un tournoi d'IA lancé par une commande, son résultat écrit | S3 | ✅ terrain d'essai et IA à utilité (s3-ia, 4762d74) |
| P6.3 | Guide de style pour S1 : couleurs, formes et dimensions du papier découpé retenu, de jour et de nuit | S1 peut dessiner le style sans deviner ; l'utilisateur voit le jeu dans le style choisi | S6 | ✅ livré (e796775), remplacé par la 3D (D-36) |
| P6.4 | Modèles 3D low poly dans Blender, rendus en sprites pour le plateau (D-36) | Des scènes de jeu en images, avec variantes ; un choix de l'utilisateur avant toute intégration | S6 | 🔄 |
| P1.6 | Version jouable complète (D-34) : toutes les règles de la première version, adversaire tenu par l'ordinateur, graphismes provisoires | Sur l'essai : une partie entière, de la nouvelle partie à la victoire, contre l'ordinateur | S1 et S3 | 📱 essai jouable 0.1.17, vert à l'émulateur ; une partie entière à faire sur le téléphone |
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
| R8 | essai 0.1.17 | Joue une partie entière contre l'ordinateur, de « Nouvelle partie » à l'écran de fin | La partie se joue sans blocage ; les panneaux (ville, évolution, combat) se lisent sur le téléphone | à essayer |
