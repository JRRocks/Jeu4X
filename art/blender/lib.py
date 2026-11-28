# Outils communs : couleurs, formes low poly, caméra calée sur le plateau, lumière, rendu.
# Repère : une case fait 1 × 1 (x, y), z vers le haut. Le dessus de la case est à z = T.
# La caméra voit l'avant de la case depuis (+x, -y) : la face -y paraît à gauche, la face +x à droite.
import math
import random

import bpy  # noqa: F401  (à importer avant bmesh hors de Blender)
import bmesh
from mathutils import Matrix, Vector

# ---------- Couleurs (palettes Jour franc et Nuit, guide-v1.md § 2) ----------
PAL = {
    'jour': {
        'P': '#9CCF63', 'F': '#3E7140', 'C': '#C98A3E', 'M': '#8E8C99', 'E': '#3193EC',
        'terre': '#9A7650', 'roche': '#7C726A', 'neige': '#FFFFFF', 'peau': '#F2D3AE', 'bois': '#7A5232',
        'bois_clair': '#A47A50', 'metal': '#8C96A8', 'laiton': '#C99A3A', 'mur': '#FFF6E5', 'chaume': '#B8A98E',
        'pierre': '#C9C3B8', 'vierge': '#ECE8E0', 'nuage': '#FFFFFF', 'or': '#F3B01D', 'cheval': '#B88A5E',
        'crocs': '#BA1A0A', 'mur_clan': '#F3B01D', 'veil': '#4A45D1', 'vents': '#5CCBFA',
    },
    'nuit': {
        'P': '#61754C', 'F': '#15561D', 'C': '#AB8C2C', 'M': '#8C8A9A', 'E': '#17375A',
        'terre': '#4A3A2C', 'roche': '#4C4852', 'neige': '#C9CED8', 'peau': '#D8B793', 'bois': '#5A4128',
        'bois_clair': '#7A5A3C', 'metal': '#7C8698', 'laiton': '#A88430', 'mur': '#D9D2C4', 'chaume': '#8A8478',
        'pierre': '#8E8A86', 'vierge': '#2A3140', 'nuage': '#3A4254', 'or': '#FFE14A', 'cheval': '#8E6A48',
        'crocs': '#FF5C4D', 'mur_clan': '#FFE14A', 'veil': '#8C7BFF', 'vents': '#62D8F0',
    },
}
CLANS = ['crocs', 'mur', 'veil', 'vents']


def lin(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def rgb(hexa):
    h = hexa.lstrip('#')
    return tuple(lin(int(h[i:i + 2], 16) / 255) for i in (0, 2, 4)) + (1.0,)


def mix(hexa, autre, t):
    a = [int(hexa.lstrip('#')[i:i + 2], 16) for i in (0, 2, 4)]
    b = [int(autre.lstrip('#')[i:i + 2], 16) for i in (0, 2, 4)]
    return '#' + ''.join(f'{round(x + (y - x) * t):02X}' for x, y in zip(a, b))


_MATS = {}


def mat(hexa, rough=0.85):
    """Matériau mat d'une seule couleur (les facettes font le relief)."""
    key = (hexa, rough)
    if key in _MATS and _MATS[key].name in bpy.data.materials:
        return _MATS[key]
    m = bpy.data.materials.new(f'm{hexa}')
    m.use_nodes = True
    b = m.node_tree.nodes.get('Principled BSDF')
    b.inputs['Base Color'].default_value = rgb(hexa)
    b.inputs['Roughness'].default_value = rough
    if 'Specular IOR Level' in b.inputs:
        b.inputs['Specular IOR Level'].default_value = 0.15
    _MATS[key] = m
    return m


# ---------- Formes ----------
def objet(nom, bm, couleur, parent=None):
    me = bpy.data.meshes.new(nom)
    bm.to_mesh(me)
    bm.free()
    for p in me.polygons:
        p.use_smooth = False
    ob = bpy.data.objects.new(nom, me)
    ob.data.materials.append(mat(couleur))
    bpy.context.scene.collection.objects.link(ob)
    if parent:
        ob.parent = parent
    return ob


def boite(nom, sx, sy, sz, couleur, loc=(0, 0, 0), rot=(0, 0, 0)):
    bm = bmesh.new()
    bmesh.ops.create_cube(bm, size=1.0)
    bmesh.ops.scale(bm, vec=(sx, sy, sz), verts=bm.verts)
    ob = objet(nom, bm, couleur)
    ob.location = loc
    ob.rotation_euler = rot
    return ob


def cylindre(nom, r1, r2, h, couleur, loc=(0, 0, 0), rot=(0, 0, 0), cotes=6):
    """Cylindre ou cône tronqué posé sur sa base (z de 0 à h)."""
    bm = bmesh.new()
    bmesh.ops.create_cone(bm, cap_ends=True, cap_tris=False, segments=cotes, radius1=r1, radius2=r2, depth=h)
    bmesh.ops.translate(bm, vec=(0, 0, h / 2), verts=bm.verts)
    ob = objet(nom, bm, couleur)
    ob.location = loc
    ob.rotation_euler = rot
    return ob


def boule(nom, r, couleur, loc=(0, 0, 0), echelle=(1, 1, 1), subdiv=1, alea=0.0, graine=0):
    bm = bmesh.new()
    bmesh.ops.create_icosphere(bm, subdivisions=subdiv, radius=r)
    if alea:
        rnd = random.Random(graine)
        for v in bm.verts:
            v.co += Vector((rnd.uniform(-1, 1), rnd.uniform(-1, 1), rnd.uniform(-1, 1))) * alea * r
    bmesh.ops.scale(bm, vec=echelle, verts=bm.verts)
    ob = objet(nom, bm, couleur)
    ob.location = loc
    return ob


def prisme(nom, points, h, couleur, loc=(0, 0, 0)):
    """Extrusion verticale d'un polygone (x, y) sur une hauteur h."""
    bm = bmesh.new()
    bas = [bm.verts.new((x, y, 0)) for x, y in points]
    f = bm.faces.new(bas)
    r = bmesh.ops.extrude_face_region(bm, geom=[f])
    vs = [e for e in r['geom'] if isinstance(e, bmesh.types.BMVert)]
    bmesh.ops.translate(bm, vec=(0, 0, h), verts=vs)
    bmesh.ops.recalc_face_normals(bm, faces=bm.faces)
    ob = objet(nom, bm, couleur)
    ob.location = loc
    return ob


def toit(nom, sx, sy, h, couleur, loc):
    """Toit à deux pans, faîte selon x."""
    bm = bmesh.new()
    p = [(-sx / 2, -sy / 2, 0), (sx / 2, -sy / 2, 0), (sx / 2, sy / 2, 0), (-sx / 2, sy / 2, 0), (-sx / 2, 0, h), (sx / 2, 0, h)]
    v = [bm.verts.new(c) for c in p]
    for f in [(0, 1, 2, 3), (0, 1, 5, 4), (3, 2, 5, 4), (0, 4, 3), (1, 2, 5)]:
        bm.faces.new([v[i] for i in f])
    bmesh.ops.recalc_face_normals(bm, faces=bm.faces)
    ob = objet(nom, bm, couleur)
    ob.location = loc
    return ob


def groupe(nom, loc=(0, 0, 0), rotz=0.0):
    e = bpy.data.objects.new(nom, None)
    bpy.context.scene.collection.objects.link(e)
    e.location = loc
    e.rotation_euler = (0, 0, rotz)
    return e


def rattacher(parent, *obs):
    for o in obs:
        o.parent = parent


# ---------- Scène, caméra, lumière ----------
CAM_ROT = (math.radians(60), 0, math.radians(45))   # projection 2:1 du plateau
RATIO = 1.25                                          # image de taille × 1,25 taille
ANCRE_Y = 0.68                                        # l'ancre est à 68 % de la hauteur, depuis le haut


def nouvelle_scene():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    _MATS.clear()
    return bpy.context.scene


def regler(scene, taille, ombrage, echantillons=48):
    """Caméra orthographique, lumière commune à tous les sprites, Cycles sur processeur."""
    sc = scene
    sc.render.engine = 'CYCLES'
    sc.cycles.device = 'CPU'
    sc.cycles.samples = echantillons
    sc.cycles.use_denoising = True
    sc.render.resolution_x = taille
    sc.render.resolution_y = round(taille * RATIO)
    sc.render.film_transparent = True
    sc.render.image_settings.file_format = 'PNG'
    sc.render.image_settings.color_mode = 'RGBA'
    sc.view_settings.view_transform = 'Standard'
    sc.view_settings.look = 'None'
    # caméra : le losange de la case (largeur √2 en unités du monde) remplit la largeur de l'image
    cam = bpy.data.objects.new('camera', bpy.data.cameras.new('camera'))
    sc.collection.objects.link(cam)
    sc.camera = cam
    cam.data.type = 'ORTHO'
    cam.data.sensor_fit = 'HORIZONTAL'
    cam.data.ortho_scale = math.sqrt(2)
    cam.rotation_euler = CAM_ROT
    vue = Matrix.Rotation(CAM_ROT[2], 4, 'Z') @ Matrix.Rotation(CAM_ROT[0], 4, 'X') @ Vector((0, 0, 1, 0))
    cam['vue'] = list(vue[:3])
    haut = Matrix.Rotation(CAM_ROT[2], 4, 'Z') @ Matrix.Rotation(CAM_ROT[0], 4, 'X') @ Vector((0, 1, 0, 0))
    cam['haut'] = list(haut[:3])
    # soleil : vient de l'avant-gauche et d'en haut, éclaire la face gauche, laisse la droite dans l'ombre
    sun = bpy.data.objects.new('soleil', bpy.data.lights.new('soleil', 'SUN'))
    sc.collection.objects.link(sun)
    vers_soleil = Vector((-0.35, -1.0, 1.55)).normalized()
    sun.rotation_euler = vers_soleil.to_track_quat('Z', 'Y').to_euler()
    doux = ombrage == 'doux'
    sun.data.energy = 3.2 if doux else 3.6
    sun.data.angle = math.radians(28 if doux else 2)
    # ciel : lumière d'ambiance douce, légèrement bleutée
    w = bpy.data.worlds.new('ciel')
    sc.world = w
    w.use_nodes = True
    bg = w.node_tree.nodes['Background']
    bg.inputs['Color'].default_value = (0.82, 0.88, 1.0, 1)
    bg.inputs['Strength'].default_value = 1.25 if doux else 0.95
    return cam


def viser(cam, ancre):
    """Place la caméra pour que le point `ancre` tombe sur le point d'ancrage de l'image."""
    v, h = Vector(cam['vue']), Vector(cam['haut'])
    decalage = (ANCRE_Y - 0.5) * RATIO * math.sqrt(2)   # l'ancre sous le centre de l'image
    cam.location = Vector(ancre) + v * 30 + h * decalage


def capteur_ombre(z, taille=1.0):
    """Plan invisible qui ne garde que les ombres (pour les objets posés sur une case)."""
    bm = bmesh.new()
    bmesh.ops.create_grid(bm, x_segments=1, y_segments=1, size=taille / 2)
    ob = objet('capteur', bm, '#FFFFFF')
    ob.location = (0, 0, z)
    ob.is_shadow_catcher = True
    return ob


def rendre(chemin):
    sc = bpy.context.scene
    sc.render.filepath = chemin
    bpy.ops.render.render(write_still=True)
