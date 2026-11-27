# Décisions

> Une décision ne se réécrit pas. Pour la changer : une ligne nouvelle qui la remplace, et la
> mention « remplacée par D-xx » sur l'ancienne.
> Colonne « Qui » : `utilisateur` pour ce qu'il a tranché, `développeur` pour un choix technique
> sur lequel il peut revenir.

| N° | Date | Décision | Pourquoi | Qui |
|---|---|---|---|---|
| D-01 | 2026-11-27 | Dépôt `JRRocks/Jeu4X` **public** ; Jeu4X est un nom provisoire | Compilation automatique gratuite et illimitée | utilisateur |
| D-02 | 2026-11-27 | Jeu 4X au tour par tour pour téléphone Android, livré en APK ; The Battle of Polytopia comme référence de base du genre | Demande de départ | utilisateur |
| D-03 | 2026-11-27 | Rien n'est copié d'un jeu existant : on reprend des principes de jeu, jamais les noms, tribus, personnages, dessins, icônes, musique, cartes ni l'allure | Le jeu doit être original | développeur |
| D-04 | 2026-11-27 | Technologie : la session 1 compare d'abord, l'utilisateur choisit ensuite | Choix coûteux à changer plus tard | utilisateur |
| D-05 | 2026-11-27 | Travail en sessions : une session maître, seule à parler à l'utilisateur, et six sessions de travail (environnement, game design, IA, données, veille, direction artistique) | Demande de départ | utilisateur |
| D-06 | 2026-11-27 | Interface en français ; documents, journaux et commits en français ; code : identifiants en anglais, commentaires en français | Convention à fixer dès le départ | développeur |
| D-07 | 2026-11-27 | Game design : recherche seulement ; l'utilisateur trie, seul le retenu est implémenté | Demande de départ (« faire le tri des patates ») | utilisateur |
| D-08 | 2026-11-27 | Session 6 : styles graphiques, palettes de couleurs et interface | Réponse de l'utilisateur | utilisateur |
| D-09 | 2026-11-27 | Une branche et un dossier par session ; `main` tenue par la session maître ; pas de pull request | Six sessions en parallèle sans se marcher dessus | développeur |
| D-10 | 2026-11-27 | L'IA progresse avec le nombre de joueurs et de parties ; elle est pré-entraînée pour ne pas partir de zéro (apprentissage par renforcement, mutation génétique et autres méthodes) | Demande de départ | utilisateur |
| D-11 | 2026-11-27 | Sessions 3 à 6 : étude et propositions d'abord, rien d'implémenté dans le jeu avant validation | Le moteur n'existe pas encore ; avancer par petits pas | développeur |
| D-12 | 2026-11-27 | Technologie : un mélange, chaque technologie là où elle est la plus forte. L'entraînement de base de l'IA se fait sur ordinateur, idéalement en Rust ; le téléphone reçoit l'IA déjà entraînée. Les recommandations des sessions sont suivies sans nouvelle question. Complète D-04 | Réponse de l'utilisateur au comparatif de S1 (Kotlin recommandé par S1, Rust par S3) | utilisateur |
| D-13 | 2026-11-27 | Tri des idées sur une page à cocher (garder, plus tard, jeter), recommandations des sessions pré-cochées | Réponse de l'utilisateur | utilisateur |
