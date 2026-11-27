# Direction artistique (session 6)

Aperçus à comparer : rien n'est codé dans le jeu (D-11). Détail de chaque proposition : [`catalogue.md`](catalogue.md).

## Images (`apercus/`, PNG à l'échelle 2, sans métadonnées)

| N° | Fichier | Ce qu'elle montre |
|---|---|---|
| DA-000 | `DA-000-comparatif.png` | Les cinq styles côte à côte sur la même scène |
| DA-001 | `DA-001-style-carnet.png` | Style « Carnet d'explorateur » : encre et lavis sur papier |
| DA-002 | `DA-002-style-pixel.png` | Style « Pixel 16 » : gros pixels, 16 × 16 par case |
| DA-003 | `DA-003-style-papier.png` | Style « Papier découpé » : formes découpées, ombres douces |
| DA-004 | `DA-004-style-bois.png` | Style « Plateau de bois » : tuiles et pions de jeu de société |
| DA-005 | `DA-005-style-tactique.png` | Style « Table tactique » : lignes lumineuses sur fond sombre |
| DA-006 | `DA-006-palette-jour.png` | Palette « Jour franc » : terrains, 6 clans simulés pour 3 daltonismes, contrastes |
| DA-007 | `DA-007-palette-nuit.png` | Palette « Nuit » (fond sombre) |
| DA-008 | `DA-008-palette-lavis.png` | Palette « Lavis » (tons de papier) |
| DA-009 | `DA-009-palette-soleil.png` | Palette « Plein soleil » (contraste maximal) |
| DA-010 | `DA-010-interface-pouce.png` | Interface « Pouce » : panneau bas, deux écrans |
| DA-011 | `DA-011-interface-anneau.png` | Interface « Anneau » : actions en anneau, deux écrans |
| DA-012 | `DA-012-interface-onglets.png` | Interface « Onglets » : barre d'onglets, deux écrans |
| DA-013 | `DA-013-terrains-perspective.png` | Série 2 : terrains en perspective, trois façons de poser le papier |
| DA-014 | `DA-014-villes.png` | Série 2 : village libre et villes de niveau 1 à 4, deux variantes |
| DA-015 | `DA-015-unites-clans.png` | Série 2 : unités des quatre clans, en couleur, en gris, en jeton |
| DA-016 | `DA-016-brouillard.png` | Série 2 : brouillard qui se souvient, trois variantes |
| DA-017 | `DA-017-unite-choisie-combat.png` | Série 2 : unité choisie et annonce du combat, trois variantes |
| DA-018 | `DA-018-pouce-jour-nuit.png` | Série 2 : écran Pouce de jour et de nuit |
| DA-019 | `DA-019-guide-couleurs.png` | Guide V1 : couleurs exactes, Jour franc et Nuit |
| DA-020 | `DA-020-guide-terrains.png` | Guide V1 : terrains en livre animé, ruine, exploitations, brouillard, coupe cotée |
| DA-021 | `DA-021-guide-villes.png` | Guide V1 : village neutre, villes de niveau 1 à 5, bâtiments, étiquette |
| DA-022 | `DA-022-guide-unites.png` | Guide V1 : les 9 unités pour les quatre clans, test en gris, nuit |
| DA-023 | `DA-023-guide-ordre-de-dessin.png` | Guide V1 : ordre de dessin |
| DA-024 | `DA-024-guide-pouce-combat.png` | Guide V1 : disposition Pouce cotée et panneau du combat |

## Refaire les images

Le dossier `source/` contient la scène type (`scene.js`), les styles (`styles.js`), les palettes (`palettes.js`), les interfaces (`interfaces.js`), les calculs de couleur (`couleurs.js`) et le script de rendu. La série 2 utilise `iso.js` et `apercu2.html` ; les planches du guide V1, `iso.js` et `guide.html`.

```
npm i @fontsource/fredoka @fontsource/im-fell-french-canon @fontsource/pixelify-sans @fontsource/chakra-petch @fontsource/bitter @fontsource/atkinson-hyperlegible @fontsource/nunito
node source/rendu.js <chemin>/node_modules/@fontsource
```

Les polices (licence SIL OFL 1.1) sont copiées dans `source/fonts/`, qui n'est pas versionné.

`DA-000`, `DA-001`, `DA-013` et `DA-024` sont ensuite réduites à 256 couleurs (Pillow, tramage) pour rester sous 1 Mo.

## Catalogue pour la page de choix

`catalogue.json` reprend les idées et les questions au format demandé par la session maître.

`catalogue-2.json` fait de même pour la deuxième série (questions DA-Q5 à DA-Q10).

**Guide de dessin pour S1 : [`guide-v1.md`](guide-v1.md).**
