# Art 3D low poly (D-36)

Modèles faits uniquement par des scripts Python pour Blender (bpy), sans modèle ni texture venus d'ailleurs.
Rendus en sprites PNG à fond transparent, vus par une caméra orthographique calée sur la projection du plateau de l'application (losange 2 de large pour 1 de haut).

| Dossier | Contenu |
|---|---|
| `blender/` | Scripts : couleurs, formes, rendu des sprites |
| `sprites/` | Sprites rendus, par variante, palette et taille |

Commande (Blender 4.x ou 5.x, ou le module `bpy` de PyPI) :

```
python art/blender/rendu.py --variante A --palette jour --taille 192
# ou : blender -b -P art/blender/rendu.py -- --variante A --palette jour --taille 192
```

Moteur de rendu : Cycles sur processeur (Workbench et EEVEE demandent une carte graphique et ne rendent rien sans écran).

## Contrat des sprites (pour S1)

- **Moteur** : Cycles sur processeur, 48 échantillons, débruitage, gestion des couleurs « Standard » pour que les couleurs de la palette sortent telles quelles.
- **Caméra** : orthographique, inclinée de 60° et tournée de 45°. Une case de 1 × 1 devient un losange de 2 pour 1, comme dans `Perspective.kt` (`demiHauteur = demiLargeur / 2`).
- **Image** : `taille × 1,25 taille` pixels, fond transparent. Le losange de la case occupe toute la largeur (`taille × taille/2`).
- **Ancre** : le centre du dessus de la case, au point (`taille/2` ; `0,85 taille`) de l'image. Pour dessiner un sprite sur la case, on aligne cette ancre sur le centre du losange de `Perspective.centre()`, et on donne au sprite la largeur `2 × demiLargeur` à l'écran.
- **Lumière** : la même pour tous. Un soleil vient de l'avant-gauche et d'en haut ; la face gauche est claire, la droite sombre, et l'ombre tombe vers la droite. Un ciel faiblement bleuté sert d'ambiance.
- **Tailles** : 96, 192 et 288 px de large, soit 1×, 2× et 3× une case de 96 dp, le zoom le plus fort. L'application choisit la taille la plus proche au-dessus de la taille affichée.
- **Ombres** : les objets posés (unités, villes, constructions) portent leur ombre sur leur propre case, grâce à un capteur d'ombre. Une ombre ne déborde pas sur la case voisine.
- **Liste** : `sprites.json` dans chaque dossier.

| Sprite | Rôle | Ordre de dessin sur la case |
|---|---|---|
| `terrain_P`, `_F`, `_C`, `_M`, `_E` | Plaine, forêt, colline, montagne, eau | 1 |
| `nuage` | Case inconnue (remplace le terrain) | 1 |
| `champ`, `cueillette`, `carriere` | Exploitations | 2 |
| `remparts_arriere` | Murailles du fond | 3 |
| `village`, `ville_<clan>_<1..5>`, `ruine`, `merveille_veil` | Lieux | 4 |
| `remparts_avant`, `marche`, `capitale_<clan>` | Surcouches de ville | 5 |
| `calque` | Voile « vue autrefois » (2D) | 6 |
| `unite_<clan>_<type>` | Unités, si la case est visible | 7 |

Clans : `crocs`, `mur`, `veil`, `vents`.
Types communs : `eclaireur`, `lancier`, `archer`, `cavalier`, `catapulte`.
Unités propres : `coureur` (crocs), `pavois` (mur), `astronome` (veil), `planeur` (vents).
Les cases se dessinent de l'arrière vers l'avant, comme dans `Perspective.ordreDeDessin()`.

## Réglage retenu (D-37)

Dalles épaisses, facettes nettes, personnages trapus : `sprites/epaisse-facettes-trapu/<jour|nuit>/<96|192|288>/`.

Les variantes écartées (dalles fines, ombrage doux, personnages élancés) ne sont plus dans le dépôt. Elles se refont à la commande, par exemple :

```
python art/blender/rendu.py --palette jour --taille 192 --epaisseur fine --quoi terrains
python art/blender/rendu.py --palette jour --taille 192 --ombrage doux
python art/blender/rendu.py --palette jour --taille 192 --proportions elance --quoi unites
```
