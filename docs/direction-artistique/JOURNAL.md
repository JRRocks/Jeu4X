# Journal de la session 6 : direction artistique

> Entrée la plus récente en haut. Gabarit de `docs/JOURNAL.md`.

## 2026-11-27 · Mission 3 : guide de dessin de la V1 pour S1

- **Auteur** : Claude, session S6
- **Branche et commit** : `s6-direction-artistique`, voir `git log` ; `main` fusionnée avant de commencer
- **Objectif** : après D-31 à D-33, écrire `guide-v1.md` pour que S1 dessine le jeu dans Compose sans deviner, avec des planches de référence.
- **Réalisé** : `guide-v1.md` (couleurs, projection et toucher, terrains, villes 1 à 5, 9 unités × 4 clans, brouillard, ordre de dessin, marques, Pouce, panneau du combat) ; planches DA-019 à DA-024 ; `iso.js` complété (colline, lancier, cavalier, catapulte, villes niveau 5, capitale, remparts, marché, ruine, exploitations) ; page `source/guide.html`.
- **Décisions** : aucune décision de l'utilisateur. Choix de travail, à valider : la colline (nouvelle) en `#C98A3E` / `#AB8C2C`, et la montagne refroidie (`#8E8C99` / `#8C8A9A`) pour rester distincte de la colline chez les daltoniens ; texte foncé sur « Attaquer » de nuit (le blanc n'atteignait pas 3:1) ; plafond des décors et des unités à 1,25 H ; toucher par silhouette d'unité puis par losange.
- **Vérifications** : chaque planche regardée après rendu ; contrastes calculés (`couleurs.js`) ; images sous 1 Mo (DA-024 réduite à 256 couleurs).
- **Non vérifié sur téléphone** : toucher des cases de 36 dp de haut, lisibilité des personnages au zoom le plus petit, fluidité des ombres.
- **Problèmes et dette** : la transparence des décors devant une case marquée est décrite mais pas dessinée ; la merveille n'est pas dessinée ; les images de la série 2 gardent les anciennes formes (guerrier, ailes plus hautes).
- **Prochaine étape** : attendre les retours de la session maître et de S1.

## 2026-11-27 · Mission 2 : deuxième série, en perspective

- **Auteur** : Claude, session S6
- **Branche et commit** : `s6-direction-artistique`, voir `git log` ; `main` fusionnée dans la branche avant de commencer
- **Objectif** : mission 2 (`MISSION.md`) après le tri D-27 : terrains en perspective, villages et villes, unités des quatre clans, brouillard, unité choisie et combat, écran Pouce de jour et de nuit.
- **Réalisé** : 6 images DA-013 à DA-018 (variantes A/B/C), moteur de rendu `source/iso.js` et page `source/apercu2.html`, catalogue complété, `catalogue-2.json` (questions DA-Q5 à DA-Q10).
- **Décisions** : aucune décision de l'utilisateur. Choix de travail : cases de 72 × 36 px au départ (environ deux fois la vue d'ensemble, D-20), aucune adresse affichée (D-16), clans en couleurs Jour franc (Crocs-Rouges rouge, Murailliers or, Veilleurs indigo, Vents-Errants ciel).
- **Vérifications** : chaque image regardée après rendu ; JSON validé ; chemins d'image vérifiés ; DA-013 réduite à 256 couleurs pour passer sous 1 Mo.
- **Non vérifié sur téléphone** : taille des cases (36 px de haut, sous les 48 dp) et des personnages en main, lisibilité du « déjà vu » en mode Nuit.
- **Problèmes et dette** : les montagnes en couches de carton peuvent rappeler des pyramides à gradins ; noms d'unités provisoires, à caler sur les règles de S2.
- **Prochaine étape** : attendre les choix DA-Q5 à DA-Q10.

## 2026-11-27 · Information reçue : vue du plateau en perspective (D-16)

- **Auteur** : Claude, session S6
- **Réalisé** : rien de nouveau. La session maître signale que l'utilisateur a choisi un plateau en perspective, en cases losanges (variante C de la session 1), sans adresse de case visible (D-16). Mes aperçus sont en vue de dessus, avec les adresses en marge.
- **Conséquence** : une éventuelle deuxième série (DA-Q4) se fera dans cette vue en perspective, sans adresses à l'écran, en restant loin de l'allure de Polytopia (D-03). Point d'attention : Polytopia utilise lui aussi des losanges ; l'originalité devra venir du trait, des couleurs et des volumes, pas de la grille.
- **Prochaine étape** : attendre le résultat du tri.

## 2026-11-27 · Question DA-Q4 ajoutée au catalogue

- **Auteur** : Claude, session S6
- **Branche et commit** : `s6-direction-artistique`, a55c81d
- **Réalisé** : à la demande de la session maître, ajout dans `catalogue.json` de la question 4 du rapport (deuxième série de variantes avant la session 1), en DA-Q4 avec les options a et b, recommandation a.
- **Vérifications** : `python3 -m json.tool` passe.
- **Note** : le commit a55c81d avait aussi modifié l'entrée précédente du journal ; cette modification est annulée ici, le passé du journal ne se réécrit pas.

## 2026-11-27 · Catalogue au format de la page de choix

- **Auteur** : Claude, session S6
- **Branche et commit** : `s6-direction-artistique`, voir `git log`
- **Objectif** : demande de la session maître : fichier `catalogue.json` pour la page où l'utilisateur coche ses choix.
- **Réalisé** : `catalogue.json` (12 idées, 3 questions, une image par option) ; DA-000 et DA-001 réduites à 256 couleurs pour passer sous 1 Mo (1,5 et 1,3 Mo avant, 0,3 et 0,5 Mo après).
- **Vérifications** : `python3 -m json.tool` passe ; chaque chemin d'image existe ; limites de mots contrôlées par script ; DA-001 réduite regardée, pas de défaut visible.
- **Prochaine étape** : attendre les choix de l'utilisateur.

## 2026-11-27 · Premiers aperçus : cinq styles, quatre palettes, trois interfaces

- **Auteur** : Claude, session S6
- **Branche et commit** : `s6-direction-artistique`, voir `git log`
- **Objectif** : mission S6, point 1 à 3 : styles graphiques, palettes, interfaces, montrés sur la même scène type.
- **Réalisé** : 13 images DA-000 à DA-012 (`apercus/`), catalogue, sources, et dans `source/` le moteur de rendu (HTML et SVG dessinés par le code, rendu Playwright à l'échelle 2). Couleurs des clans choisies par recherche automatique pour maximiser l'écart minimal (OKLab) sous trois simulations de daltonisme, puis ajustées à la main ; terrains de même.
- **Décisions** : aucune décision de l'utilisateur ; propositions seulement. Choix de travail : cases carrées vues de dessus (pas d'isométrie, pour s'éloigner de Polytopia), nos pions ronds et ceux de l'ennemi en losange pour ne pas dépendre de la couleur.
- **Vérifications** : chaque image regardée après rendu ; contrastes et écarts calculés par `couleurs.js` (valeurs dans le catalogue) ; PNG contrôlés sans bloc de métadonnées (seulement IHDR, IDAT, IEND) ; contrôle `depot-public` avant push.
- **Non vérifié sur téléphone** : taille réelle des cases en main, plein soleil, confort du pouce, fluidité des effets, lecture par des personnes daltoniennes (simulation seulement).
- **Problèmes et dette** : la palette Lavis a des terrains très proches (0,04), seuls les symboles les séparent ; la police du Carnet a des chiffres à l'ancienne (1 ≈ I).
- **Prochaine étape** : attendre les choix de l'utilisateur transmis par la session maître.
