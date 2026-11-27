# Journal de la session 6 : direction artistique

> Entrée la plus récente en haut. Gabarit de `docs/JOURNAL.md`.

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
