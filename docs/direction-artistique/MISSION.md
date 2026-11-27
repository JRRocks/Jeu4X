# Mission de la session 6 : direction artistique

> Écrite par la session maître le 2026-11-27. Branche : `s6-direction-artistique`. Dossier : `docs/direction-artistique/`. Préfixe des idées : `DA-`.
> Lire d'abord `CLAUDE.md`, `docs/ORGANISATION.md` et `docs/DECISIONS.md`.

## Ce que l'utilisateur a demandé

La sixième session travaille vraiment sur la colorisation et le style : elle propose différents modèles de **styles graphiques**, de **palettes de couleurs** et d'**interface** (D-08). La session 1 ne fait que des graphismes provisoires pour un premier aperçu.

Tu ne codes rien dans le jeu (D-11) : tu produis des aperçus.

## Ce qu'il faut produire

1. **Styles graphiques** : quatre à six univers visuels nettement différents, chacun montré sur la **même scène type** pour qu'on compare : un petit plateau de cases adressées (A1, B2…) avec plusieurs terrains, une ville, deux ou trois unités, une barre du haut et un bouton « Fin du tour », dans un cadre de téléphone en portrait. Les unités et la ville ne servent qu'à illustrer le style.
2. **Palettes de couleurs** : trois ou quatre jeux de couleurs (terrains, couleurs des clans, interface), lisibles sur un petit écran, couleurs de clans distinguables par les daltoniens, contrastes de l'interface vérifiés.
3. **Interface** : deux ou trois modèles (boutons, menus, police, icônes, disposition de l'écran de jeu), en t'appuyant sur le dossier UI/UX de la Library (`~/.claude/library/docs/ui-ux/`).

Pour chaque proposition : l'intention, ce qui la rend lisible sur un téléphone, ce qu'elle coûte à produire sans dessinateur (dessin par le code, images générées et leur licence, banques d'images libres et leur licence), son poids pour le téléphone.

Original (D-03) : rien qui ressemble à l'allure d'un jeu existant, Polytopia compris.

## Comment

- Images rendues depuis HTML ou SVG avec le Playwright de la session, à l'échelle 2 (`deviceScaleFactor: 2`), sans métadonnées, rangées dans `docs/direction-artistique/apercus/`.
- `docs/direction-artistique/README.md` : une ligne par image (numéro `DA-NNN`, ce qu'elle montre) ; `catalogue.md` au format commun ; `sources.md` ; `JOURNAL.md`.
- Rapport à la session maître : la liste des images avec leur chemin et une ligne chacune, ta recommandation, les questions pour l'utilisateur. Puis attendre ses choix.
