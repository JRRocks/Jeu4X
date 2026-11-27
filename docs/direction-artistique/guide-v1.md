# Guide de dessin de la V1 (pour S1)

> Session 6, 2026-11-27. À suivre sans deviner pour dessiner le plateau et l'écran de jeu dans l'application (Kotlin + Compose, D-14).
> Décisions appliquées : D-16 (perspective, aucune adresse affichée), D-20 (départ zoomé), D-21 (glissé de la vue), D-27 (papier découpé, Jour franc et Nuit, interface Pouce), D-31 (livre animé, maisons découpées, personnages debout, nuages et calque, pastilles et panneau), D-33 (Nuit = mode sombre du téléphone).
> Règles et noms : `docs/game-design/regles-v1.md` (branche `s2-game-design`).
> **Référence exacte des formes** : `docs/direction-artistique/source/iso.js`. Ce code dessine toutes les planches ci-dessous. Ses coordonnées font foi ; ce guide en donne les règles et les valeurs clés. Ce n'est pas du code de l'application.

## Planches de référence

| N° | Image | Contenu |
|---|---|---|
| DA-019 | `apercus/DA-019-guide-couleurs.png` | Toutes les couleurs, Jour franc et Nuit, contrastes mesurés |
| DA-020 | `apercus/DA-020-guide-terrains.png` | Cinq terrains, ruine, exploitations, trois états du brouillard, coupe cotée |
| DA-021 | `apercus/DA-021-guide-villes.png` | Village neutre, villes de niveau 1 à 5, capitale, remparts, marché, étiquette |
| DA-022 | `apercus/DA-022-guide-unites.png` | Les 9 unités de la V1 pour les quatre clans, test en gris, nuit |
| DA-023 | `apercus/DA-023-guide-ordre-de-dessin.png` | Ordre de dessin, couche par couche |
| DA-024 | `apercus/DA-024-guide-pouce-combat.png` | Disposition Pouce cotée, panneau du combat de jour et de nuit |

## 1. Unités de mesure

- Toutes les tailles sont en **dp** et rapportées à un écran de 360 × 780 dp.
- Sur le plateau, tout se mesure en **W**, la largeur d'une case. On note **H = W / 2** sa hauteur et **s = W / 80** l'échelle des personnages et des villes.
- Le zoom ne change que W, et tout le reste suit.
  - **Départ : W = 72 dp** (H = 36), environ deux fois la vue d'ensemble (D-20).
  - **Plus petit** : le plateau entier tient dans la largeur de l'écran (W = 360 / côté du plateau, soit environ 33 dp pour 11 cases).
  - **Plus grand** : W = 96 dp.

## 2. Couleurs exactes (DA-019)

### Terrains

| Terrain | Jour franc | Nuit |
|---|---|---|
| Plaine | `#9CCF63` | `#61754C` |
| Forêt | `#3E7140` | `#15561D` |
| Colline | `#C98A3E` | `#AB8C2C` |
| Montagne | `#8E8C99` | `#8C8A9A` |
| Eau | `#3193EC` | `#17375A` |

> La colline est nouvelle (règles V1). Pour que la montagne reste distincte de la colline chez les daltoniens, son gris est refroidi : Jour `#A88E78` → `#8E8C99`, Nuit `#8A7590` → `#8C8A9A`. **À valider par l'utilisateur.** Écart mini entre terrains (OKLab, pire des trois daltonismes) : 0,108 de jour, 0,071 de nuit.

### Papier et plateau

| Élément | Jour franc | Nuit |
|---|---|---|
| Table (fond sous le plateau) | `#E2C9A0` | `#0C1018` |
| Tranche des cases, côté gauche | `#FFF8EA` | `#7A6E5A` |
| Tranche des cases, côté droit | `#E0DACE` | `#6B614F` |
| Bord blanc des découpes (personnages, montagnes) | `#FFFFFF` | `#E8ECF2` |
| Ombre portée | `#3B2A1A` à 35 % | `#000000` à 60 % |
| Visages | `#F2D3AE` | `#D8B793` |
| Murs des maisons | `#FFF6E5` | `#D9D2C4` |
| Toits du village neutre | `#B8A98E` | `#8A8478` |
| Bois | `#7A5232` ; bois sombre `#5A4128` ; bois clair `#9C7650` | idem |
| Métal (lames, pointes) | `#8C96A8` | idem |
| Laiton (longue-vue) | `#C99A3A`, reflet `#F2D27A` | idem |
| Cheval | `#C9A27A`, crins et pattes `#9C7650` | idem |

### Clans

| Clan | Jour franc | Nuit | Encre sur la couleur |
|---|---|---|---|
| Crocs-Rouges | `#BA1A0A` | `#FF5C4D` | blanc de jour, `#2B2118` de nuit |
| Murailliers | `#F3B01D` | `#FFE14A` | `#2B2118` |
| Veilleurs d'Astres | `#4A45D1` | `#8C7BFF` | blanc |
| Vents-Errants | `#5CCBFA` | `#62D8F0` | `#2B2118` |

Règle : l'emblème posé sur une couleur de clan est blanc si son contraste avec blanc atteint 3:1, sinon `#2B2118`. La teinte foncée du clan (coiffes, grand bouclier) est la couleur du clan mêlée à 35 % de noir.

### Interface

| Rôle | Jour franc | Nuit |
|---|---|---|
| Cartes et panneaux | `#FFFFFF` | `#1C212A` |
| Fond des boutons | `#F7F4EC` | `#101318` |
| Texte | `#1C1E24` | `#EEF1F5` |
| Texte secondaire | `#555B66` | `#A9B1BD` |
| Traits, bords, poignée | `#D9D3C4` | `#2C3440` |
| « Fin du tour », fond / texte | `#1F5FBF` / `#FFFFFF` | `#7CC4FF` / `#0B1220` |
| « Attaquer », fond / texte | `#B3261E` / `#FFFFFF` | `#FF8A80` / `#0B1220` |
| Accent (savoir) | `#1F5FBF` | `#7CC4FF` |
| Or (icône) | `#E0A21A` | `#FFE14A` |
| Barre de vie : reste / perte annoncée | `#2E8B57` / `#F2B8B0` | idem |

Contrastes mesurés (WCAG) : tous les textes passent 4,5:1 (DA-019). La palette suit le réglage clair/sombre du téléphone (D-33). Les « nuits du jeu » (D-32) auront plus tard leur propre allure, qui n'est pas décrite ici.

## 3. Projection et toucher

- **Centre du dessus** de la case (c, r) : `x = ox + (c − r) · W/2`, `y = oy + (c + r) · H/2 − z`, où **z** est l'épaisseur de la feuille (§ 4). `ox` et `oy` viennent du cadrage.
- **Cadrage de départ** : la vue est centrée sur les unités du joueur qui commence (D-20). Le centre visé est placé à y = 330 dp, au milieu de la zone libre entre la barre du haut et le panneau. À chaque tour, la vue glisse en 500 ms jusqu'aux unités du joueur dont c'est le tour (D-21).
- **Toucher** :
  1. Si le doigt tombe dans la silhouette d'une unité visible, la case touchée est celle de l'unité. La silhouette est un rectangle de 30 s de large, qui monte de 1,25 H au-dessus du centre de sa case.
  2. Sinon, la case touchée est celle dont le losange du dessus contient le doigt. Le calcul inverse, avec `y' = y + 4·W/72` : `c = round(((x − ox)/(W/2) + (y' − oy)/(H/2)) / 2)` et `r = round(((y' − oy)/(H/2) − (x − ox)/(W/2)) / 2)`, puis on vérifie que le point est bien dans le losange.
  3. Les décors (arbres, collines, montagnes, maisons) ne prennent jamais le toucher.
- **Plafond des décors : 1,25 H** au-dessus du centre du dessus, pour les décors comme pour les unités et les villes (DA-020, coupe). Ainsi, la moitié arrière de la case de derrière reste toujours visible et touchable.
- **Transparence** : quand une case porte une marque (pastille, case choisie, ennemi visé), les décors debout des cases situées devant elle (c + 1, r + 1 et leurs voisines devant) s'affichent à 45 % d'opacité. Cette règle n'est pas dessinée dans les planches.

## 4. Terrains en livre animé (DA-020)

- **Feuille** : un losange de (W − 2,5) × (H − 1,2) dp, qui laisse un mince joint de table entre les cases. Son épaisseur z est de **4 dp × W/72**, et de 2 dp pour l'eau. On dessine la tranche gauche, puis la tranche droite, puis le dessus.
- **Bord découpé** : le dessus a 12 points, soit les 4 sommets plus 2 points par côté, au tiers et aux deux tiers. Chaque point de côté est décalé au hasard d'au plus ±1 dp à l'horizontale et ±0,5 dp à la verticale. Ce hasard est tiré une fois pour toutes de la graine de la carte, et rien ne bouge ensuite.
- **Décors** (fonction `decor` de `iso.js`, variante `popup`) :
  - **Plaine** : 3 touffes d'herbe en trois traits, couleur du terrain mêlée à 30 % de noir.
  - **Forêt** : 5 sapins, placés en (dx, dy) × W par rapport au centre : (−0,17 ; −0,02), (0,15 ; −0,06), (0 ; 0,12), (0,22 ; 0,10), (−0,24 ; 0,14), à l'échelle 1 ; 0,9 ; 1,05 ; 0,8 ; 0,8. Chaque sapin est un triangle de 20 × 14 (× W/84) en deux moitiés : la gauche éclaircie de 18 %, la droite assombrie de 30 %. Il a un tronc de bois et une ombre elliptique à 22 %. On peint les sapins par dy croissant.
  - **Colline** : deux bosses de papier debout, l'arrière assombrie de 6 %, l'avant éclaircie de 8 %, avec un bord crème de 2 dp et une hauteur d'au plus 0,78 H.
  - **Montagne** : une seule découpe arrondie (pas de pyramide), avec un bord crème de 2 dp, une calotte de neige blanche et une hauteur de 1,1 H.
  - **Eau** : deux vagues en traits clairs (la couleur de l'eau mêlée à 40 % de blanc).
- **Ruine** : deux fûts de pierre (16 et 9 dp de haut × W/72) et un bloc couché, avec une touche de mousse.
- **Exploitations** :
  - **Champ** : trois sillons jaunes `#E8C65A`, dans l'axe de la case.
  - **Cueillette** : un panier avec deux baies, au coin avant de la forêt.
  - **Carrière** : deux blocs de pierre taillée sur la colline.

## 5. Villages et villes (DA-021)

- **Maison** : un cube de 0,24 W × f de côté, monté sur f × 10 dp de murs, avec un toit à deux pans de la couleur du clan.
  - Le pan clair est éclairci de 12 %, le pan sombre assombri de 20 %.
  - Le toit porte un bord blanc de 1 dp. Les murs ont un côté gauche assombri de 10 % et un côté droit assombri de 22 %.
- **Places des maisons**, en (dx, dy) × W et échelle f : (0 ; −0,12 ; 1), (−0,2 ; 0,02 ; 0,9), (0,2 ; 0,02 ; 0,9), (0 ; 0,15 ; 0,85), (−0,06 ; −0,25 ; 0,8), (0,14 ; −0,2 ; 0,75). On les peint de l'arrière vers l'avant.
- **Nombre de maisons** :
  - village neutre : 2, sans drapeau, toits `#B8A98E` ;
  - ville de niveau n : n + 1, et une tour de plus au niveau 5.
- **Drapeau** : il est planté en (−0,3 W ; −4 dp). Mât de bois de 26 dp, fanion de 14 × 9 dp à la couleur du clan, emblème dessus.
- **Capitale** : une petite couronne dorée `#F3B01D` en haut du mât.
- **Bâtiments** (règles V1, § 5.4) :
  - **Remparts** : un muret crème qui ferme l'avant de la case.
  - **Marché** : un auvent rouge `#E04A3A` sur deux piquets.
  - La merveille n'est pas encore dessinée.
- **Étiquette** : une pastille blanche (`#1C212A` de nuit) de 20 dp de haut, posée 22 dp sous le centre de la ville. Elle porte le nom en Fredoka 600 de 12,5 sp et le niveau en points de la couleur du clan. Elle passe toujours au-dessus du plateau (§ 8).

## 6. Unités des quatre clans (DA-022)

Les 9 unités de la V1 sont l'Éclaireur, le Lancier, l'Archer, le Cavalier et la Catapulte, puis l'unité propre de chaque clan : Coureur de crête, Porte-pavois, Astronome, Guetteur-planeur.

- **Personnage** : ses coordonnées sont en unités s (`figure` dans `iso.js`). À W = 72, s vaut 0,9.
  - L'ombre est une ellipse de 15 s × 7 s à 22 %.
  - Le corps est un trapèze de 16 s de large et de 22 s de haut, à la couleur du clan. Il porte un bord blanc de 2,2 s et l'emblème au centre de la poitrine.
  - La tête est un cercle de rayon 6 s couleur visage, cerclé de blanc.
- **Socle du clan**, posé à plat sous les pieds, à la couleur du clan avec un bord blanc de 2 dp :
  - Crocs-Rouges : dents (16 pointes, rayons 13 et 9) ;
  - Murailliers : carré ;
  - Veilleurs d'Astres : étoile à 4 branches ;
  - Vents-Errants : goutte.
- **Coiffe du clan** :
  - Crocs-Rouges : crête à cinq pointes ;
  - Murailliers : casque carré ;
  - Veilleurs d'Astres : capuche pointue avec une étoile jaune ;
  - Vents-Errants : chapeau large et écharpe au vent.
- **Type d'unité**, reconnu à son objet :
  - Éclaireur : fanion sur un bâton ;
  - Lancier : lance ;
  - Archer : arc ;
  - Cavalier : sur un cheval de papier, lance ;
  - Catapulte : machine de bois à deux roues, fanion du clan ;
  - Coureur de crête : deux lames et traits de vitesse ;
  - Porte-pavois : grand bouclier à l'emblème ;
  - Astronome : longue-vue de laiton ;
  - Guetteur-planeur : ailes de toile au-dessus de la tête.
- **Test en gris** (DA-022, dernière colonne) : chaque clan doit rester reconnaissable sans couleur. Toute nouvelle unité doit passer ce test.
- **Hauteur maximale** : 1,25 H au-dessus du centre de sa case (§ 3).
- **Unité choisie** : elle se soulève de 5 dp en 120 ms, et son ombre passe à 32 %.

## 7. Brouillard qui se souvient (DA-020, règles V1 § 3)

| État | Dessin |
|---|---|
| **Visible** | Tout, unités comprises |
| **Vue autrefois** | Terrain, décor, villes, villages et ruines dans leur dernier état connu, **sans unité**. Par-dessus, un losange de calque à la hauteur du dessus : `#FFFFFF` à 58 % de jour, `#0C1018` à 55 % de nuit |
| **Inconnue** | Une feuille plate de 3 dp (dessus `#ECE8E0`, côtés `#D9D3C8` et `#C9C2B5` ; de nuit `#2A3140`, `#202634`, `#181D28`). Dessus, trois nuages ronds de papier, centrés en (−0,12 W ; −8), (0,10 W ; −10) et (0 ; −4) dp, de rayons 0,13 W, 0,15 W et 0,14 W, blancs à bord `#D9D3C8` (de nuit `#3A4254` à bord `#2A3140`) |

## 8. Ordre de dessin (DA-023)

1. La table, en fond uni.
2. **Un seul parcours des cases**, de l'arrière vers l'avant, par (c + r) croissant puis par c croissant. Pour chaque case, dans l'ordre :
   1. la tranche gauche, la tranche droite, le dessus ;
   2. le décor debout (avec 45 % d'opacité s'il passe devant une case marquée, § 3) ;
   3. le voile de brouillard (calque ou nuages) ;
   4. les marques : pastille, case choisie, ennemi visé ;
   5. l'exploitation, la ruine, le village ou la ville ;
   6. l'unité, seulement si la case est visible.
3. Les étiquettes de ville.
4. L'interface : la barre du haut, puis le panneau du bas.

Pour la fluidité, on peut pré-calculer chaque case avec son décor dans un cache d'images, une fois par niveau de zoom, et ne redessiner à chaque image que les marques, les unités et l'interface.

## 9. Marques sur le plateau

- **Case atteignable** : une pastille blanche, ellipse de 9 × 4,5 dp avec ombre portée, au centre du dessus.
- **Case choisie** : un losange de (W − 4) × (H − 2), bordé de blanc sur 3,5 dp.
- **Ennemi visé** : un losange de (W − 6) × (H − 3), rempli de la couleur « Attaquer » à 25 % et bordé de cette couleur sur 2,6 dp.

## 10. Disposition « Pouce » (DA-024)

- Le plateau occupe tout l'écran, et l'interface flotte par-dessus.
- **Barre du haut** : carte de 62 dp de haut, à 12 dp du haut et 10 dp des bords, rayon 20, ombre portée. Elle contient, de gauche à droite :
  - l'emblème du clan, dans un rond de 34 dp ;
  - « Tour » (11 sp) et son numéro (20 sp, poids 600) ;
  - l'or et le savoir (17 sp, poids 600), chacun avec son revenu en 12 sp ;
  - le bouton menu de 48 × 48 dp.
- **Panneau du bas** : 239 dp de haut, coins du haut au rayon 24, marges de 14 dp, ombre vers le haut. On y trouve, de haut en bas :
  - la poignée de 40 × 5 dp ;
  - la carte de l'unité : personnage à l'échelle 1, nom en 18 sp (600), caractéristiques en 13 sp, barre de vie de 130 × 7 dp ;
  - trois boutons carrés de 56 dp de haut, au rayon 16 et à 10 dp d'écart, chacun avec une icône et un mot en 13 sp (600) ;
  - « Fin du tour » sur toute la largeur, 58 dp de haut, rayon 29, en 21 sp (600).
- **Zone libre du plateau** : de y = 86 à y = 541 dp. Le cadrage y place les unités (§ 3).
- **Police** : Fredoka 400 et 600 (licence SIL OFL), embarquée dans l'APK. Les textes viennent des fichiers de ressources, en français.

## 11. Panneau du combat (DA-024)

Toucher un ennemi à portée remplace le contenu du panneau du bas, qui garde la même hauteur.

1. La question « Attaquer le Coureur de crête ? », en 16 sp (600), centrée.
2. Le face-à-face, sur trois colonnes (1 fr, 28 dp, 1 fr).
   - Chaque côté montre le personnage, son nom (15 sp, 600) et « PV 10 ➜ 7 » (13 sp). Il ajoute « · détruit » si l'unité tombe à 0.
   - Dessous, une barre de 110 × 7 dp : le vert représente ce qui reste, le rose la perte annoncée.
   - Au milieu, deux épées croisées.
3. Les boutons « Annuler » (1/3 de la largeur, style bouton carré) et « Attaquer » (2/3 de la largeur, 58 dp de haut, rayon 29, couleur « Attaquer »).

Les chiffres viennent du calcul d'annonce du moteur (règles V1 : combat annoncé). Ce calcul ne change pas l'état de la partie. « Annuler » ou un toucher ailleurs sur le plateau referme le panneau.

## 12. Ce qui reste à vérifier sur le téléphone

- Les cases de 36 dp de haut au départ sont sous les 48 dp recommandés. Le toucher par silhouette (§ 3) compense, mais il faut l'essayer en main.
- La lisibilité des personnages à W = 72, puis au zoom le plus petit (environ 33 dp).
- La fluidité avec les ombres portées. Si besoin, remplacer le flou par une forme décalée à 25 %.
- La lecture du calque « vue autrefois » de nuit.
