# Les modèles du jeu, faits de formes simples (low poly). Chaque fonction construit un sprite
# centré sur la case, dont le dessus est à z = T. Formes propres au jeu (D-03) : cases en dalle
# à strates, arbres en boules, montagnes en colonnes de roche, personnages en cloche.
import math
import random

import bpy  # noqa: F401
import bmesh
from mathutils import Vector

from lib import (PAL, boite, boule, cylindre, groupe, mix, objet, prisme, rattacher, toit)

FACE = math.radians(45)   # les personnages regardent le joueur (vers +x, -y)


def couleur_clan(p, k):
    return p['mur_clan'] if k == 'mur' else p[k]


# ---------- Dalle de la case ----------
def dalle(p, T, dessus, cote=None, eau=False):
    """Dalle de la case : un corps (terre et roche) et une couche de surface biseautée."""
    if T <= 0.08:   # dalle fine : un seul bloc de la couleur du terrain, un peu assombri sur les flancs
        corps = boite('corps', 0.995, 0.995, T, cote or mix(dessus, '#000000', 0.25), loc=(0, 0, T / 2))
    else:
        roche = boite('roche', 0.995, 0.995, T * 0.45, p['roche'], loc=(0, 0, T * 0.225))
        terre = boite('terre', 0.995, 0.995, T * 0.4, cote or p['terre'], loc=(0, 0, T * 0.45 + T * 0.2))
    # surface : plaque à bord biseauté, légèrement en retrait pour marquer les joints entre cases
    ep = 0.035 if not eau else 0.02
    bm = bmesh.new()
    bmesh.ops.create_cube(bm, size=1.0)
    bmesh.ops.scale(bm, vec=(0.985, 0.985, ep), verts=bm.verts)
    haut = [v for v in bm.verts if v.co.z > 0]
    bmesh.ops.scale(bm, vec=(0.955 / 0.985, 0.955 / 0.985, 1), verts=haut)
    surf = objet('surface', bm, dessus)
    surf.location = (0, 0, T - ep / 2 - (0.03 if eau else 0))
    return surf


def eau_facettes(p, T):
    """Surface de l'eau en petites facettes, sous le niveau des terres."""
    bm = bmesh.new()
    bmesh.ops.create_grid(bm, x_segments=5, y_segments=5, size=0.47)
    rnd = random.Random(4)
    for v in bm.verts:
        if abs(v.co.x) < 0.46 and abs(v.co.y) < 0.46:
            v.co.z += rnd.uniform(-0.012, 0.012)
    bmesh.ops.triangulate(bm, faces=bm.faces)
    ob = objet('vagues', bm, mix(p['E'], '#FFFFFF', 0.08))
    ob.location = (0, 0, T - 0.035)
    return ob


# ---------- Décors ----------
def touffe(p, x, y, T, couleur, s=1.0):
    cylindre('herbe', 0.025 * s, 0.0, 0.07 * s, couleur, loc=(x, y, T), cotes=4)


def arbre(p, x, y, T, s, couleur, graine):
    cylindre('tronc', 0.03 * s, 0.025 * s, 0.13 * s, p['bois'], loc=(x, y, T), cotes=5)
    boule('feuillage', 0.12 * s, couleur, loc=(x, y, T + 0.2 * s), echelle=(1, 1, 1.15), alea=0.12, graine=graine)


def terrain(p, t, T):
    rnd = random.Random({'P': 1, 'F': 2, 'C': 3, 'M': 4, 'E': 5}[t])
    if t == 'E':
        dalle(p, T, mix(p['E'], '#000000', 0.15), cote=mix(p['E'], '#000000', 0.35), eau=True)
        eau_facettes(p, T)
        for x, y in [(-0.18, 0.12), (0.2, -0.15)]:
            boite('ecume', 0.09, 0.025, 0.006, mix(p['E'], '#FFFFFF', 0.6), loc=(x, y, T - 0.02), rot=(0, 0, 0.6))
        return
    dalle(p, T, p[t])
    herbe = mix(p['P'], '#FFFFFF', 0.15)
    if t == 'P':
        for x, y in [(-0.22, -0.1), (0.15, 0.2), (0.25, -0.22), (-0.05, 0.05), (-0.2, 0.25)]:
            touffe(p, x, y, T, herbe, rnd.uniform(0.8, 1.2))
        boule('caillou', 0.03, p['pierre'], loc=(0.05, -0.28, T + 0.01), echelle=(1.3, 1, 0.6))
    if t == 'F':
        pos = [(-0.2, 0.18, 1.0), (0.18, 0.22, 0.95), (-0.02, -0.02, 1.1), (0.24, -0.16, 0.85), (-0.24, -0.2, 0.9)]
        for i, (x, y, s) in enumerate(pos):
            c = mix(p['F'], '#FFFFFF', 0.06) if i % 2 else mix(p['F'], '#000000', 0.06)
            arbre(p, x, y, T, s, c, i)
    if t == 'C':
        for (x, y, sx, sy, h) in [(-0.12, 0.12, 0.3, 0.26, 0.16), (0.16, -0.12, 0.24, 0.22, 0.12)]:
            boule('butte', 1.0, mix(p['C'], '#000000', 0.05), loc=(x, y, T - 0.01), echelle=(sx, sy, h), subdiv=1, alea=0.05, graine=int(x * 100))
        for x, y in [(-0.3, -0.25), (0.28, 0.24)]:
            touffe(p, x, y, T, herbe)
    if t == 'M':
        # colonnes de roche à pans, inclinées, coiffées de neige
        for (x, y, r, h, inc) in [(-0.08, 0.1, 0.17, 0.62, 0.05), (0.17, -0.06, 0.13, 0.42, -0.08), (-0.2, -0.18, 0.11, 0.3, 0.1)]:
            col = cylindre('roc', r, r * 0.8, h, p['M'], loc=(x, y, T - 0.02), rot=(inc, -inc * 0.6, rnd.uniform(0, 1)), cotes=6)
            neige = cylindre('neige', r * 0.82, r * 0.55, 0.05, p['neige'], loc=(0, 0, h - 0.005), cotes=6)
            neige.parent = col


# ---------- Brouillard ----------
def nuage(p, T):
    """Case inconnue : dalle pâle recouverte de nuages ronds."""
    dalle(p, T, p['vierge'], cote=mix(p['vierge'], '#000000', 0.12))
    for (x, y, r, z) in [(-0.14, 0.16, 0.24, 0.2), (0.16, 0.12, 0.26, 0.24), (0.02, -0.12, 0.25, 0.18), (0.26, -0.22, 0.17, 0.12),
                         (-0.26, -0.2, 0.17, 0.11), (0.0, 0.05, 0.22, 0.36)]:
        boule('nuage', r, p['nuage'], loc=(x, y, T + z), echelle=(1, 1, 0.8), subdiv=2, alea=0.06, graine=int(r * 100))


# ---------- Personnages ----------
def socle(p, k, T):
    c = couleur_clan(p, k)
    if k == 'crocs':
        pts = [(math.cos(a) * (0.2 if i % 2 == 0 else 0.15), math.sin(a) * (0.2 if i % 2 == 0 else 0.15)) for i, a in enumerate([j * math.pi / 8 for j in range(16)])]
    elif k == 'mur':
        pts = [(-0.16, -0.16), (0.16, -0.16), (0.16, 0.16), (-0.16, 0.16)]
    elif k == 'veil':
        pts = [(math.cos(a) * (0.23 if i % 2 == 0 else 0.09), math.sin(a) * (0.23 if i % 2 == 0 else 0.09)) for i, a in enumerate([j * math.pi / 4 + math.pi / 4 for j in range(8)])]
    else:
        pts = [(0.26, -0.0)] + [(math.cos(a) * 0.15 - 0.03, math.sin(a) * 0.15) for a in [math.radians(40 + i * 280 / 10) for i in range(11)]]
        pts = [(x * math.cos(-FACE) - y * math.sin(-FACE), x * math.sin(-FACE) + y * math.cos(-FACE)) for x, y in pts]
    pts = [(x * 1.3, y * 1.3) for x, y in pts]
    return prisme('socle', pts, 0.035, c, loc=(0, 0, T))


def coiffe(p, k, g, z, rt):
    """Coiffe propre au clan, posée au sommet de la tête (z), rayon de tête rt."""
    c = mix(couleur_clan(p, k), '#000000', 0.35)
    if k == 'crocs':     # casque à cornes recourbées
        rattacher(g, cylindre('casque', rt * 1.05, rt * 0.7, rt * 0.6, c, loc=(0, 0, z - rt * 0.45), cotes=6))
        for s in (-1, 1):
            rattacher(g, cylindre('corne', rt * 0.32, 0.0, rt * 1.3, p['pierre'], loc=(s * rt * 0.85, 0, z - rt * 0.2), rot=(0, s * 0.7, 0), cotes=4))
    elif k == 'mur':     # heaume carré à crête plate
        rattacher(g, boite('heaume', rt * 2.2, rt * 2.2, rt * 1.0, c, loc=(0, 0, z - rt * 0.25)))
        rattacher(g, boite('visiere', rt * 2.3, rt * 0.4, rt * 0.25, mix(c, '#000000', 0.3), loc=(0, -rt * 1.05, z - rt * 0.55)))
    elif k == 'veil':    # capuche pointue et étoile dorée
        rattacher(g, cylindre('capuche', rt * 1.15, 0.0, rt * 2.6, c, loc=(0, 0, z - rt * 0.8), cotes=6))
        rattacher(g, boule('etoile', rt * 0.28, p['or'], loc=(0, 0, z + rt * 1.85), subdiv=0))
    else:                # chapeau large et écharpe au vent
        rattacher(g, cylindre('bord', rt * 1.9, rt * 1.9, rt * 0.18, c, loc=(0, 0, z - rt * 0.3), cotes=8))
        rattacher(g, cylindre('calotte', rt * 0.9, rt * 0.6, rt * 0.6, c, loc=(0, 0, z - rt * 0.15), cotes=6))
        rattacher(g, boite('echarpe', rt * 0.5, rt * 2.2, rt * 0.3, mix(couleur_clan(p, k), '#FFFFFF', 0.4), loc=(-rt * 0.9, rt * 1.2, z - rt * 2.1), rot=(0.3, 0, 0.5)))


def personnage(p, k, T, prop, g, z0=0.0, echelle=1.35):
    """Corps en cloche, tête ronde, coiffe du clan. Rend (z du haut des épaules, rayon de tête)."""
    e = echelle
    if prop == 'elance':
        hb, rb, rh, rt = 0.36 * e, 0.11 * e, 0.06 * e, 0.075 * e
    else:
        hb, rb, rh, rt = 0.26 * e, 0.13 * e, 0.075 * e, 0.1 * e
    c = couleur_clan(p, k)
    z = T + z0
    rattacher(g, cylindre('robe', rb, rh, hb, c, loc=(0, 0, z), cotes=8))
    rattacher(g, cylindre('ceinture', rb * 0.82, rb * 0.8, 0.025 * e, mix(c, '#000000', 0.4), loc=(0, 0, z + hb * 0.42), cotes=8))
    tete_z = z + hb + rt * 0.9
    rattacher(g, boule('tete', rt, p['peau'], loc=(0, 0, tete_z), subdiv=1))
    coiffe(p, k, g, tete_z + rt, rt)
    return z + hb, rt


def unite(p, k, typ, T, prop):
    socle(p, k, T)
    g = groupe('unite', rotz=FACE)
    c = couleur_clan(p, k)
    if typ == 'cavalier':
        cheval(p, g, T)
        ep, rt = personnage(p, k, T, prop, g, z0=0.2, echelle=1.05)
        rattacher(g, cylindre('lance', 0.012, 0.012, 0.6, p['bois'], loc=(0.11, -0.05, T + 0.15), rot=(0.25, 0, 0), cotes=4))
        rattacher(g, cylindre('pointe', 0.03, 0.0, 0.07, p['metal'], loc=(0.11, 0.1, T + 0.73), rot=(0.25, 0, 0), cotes=4))
        return
    if typ == 'catapulte':
        catapulte(p, g, T, c)
        return
    ep, rt = personnage(p, k, T, prop, g)
    h = ep - T
    if typ == 'lancier':
        rattacher(g, cylindre('hampe', 0.013, 0.013, h + 0.28, p['bois'], loc=(0.15, -0.02, T), cotes=4))
        rattacher(g, cylindre('pointe', 0.035, 0.0, 0.08, p['metal'], loc=(0.15, -0.02, ep + 0.28), cotes=4))
    if typ == 'archer':
        pts = [(math.cos(a) * 0.2, math.sin(a) * 0.2) for a in [math.radians(-70 + d * 14) for d in range(11)]]
        pts += [(math.cos(a) * 0.17, math.sin(a) * 0.17) for a in [math.radians(70 - d * 14) for d in range(11)]]
        arc = prisme('arc', [(-x, y) for x, y in pts], 0.03, p['bois_clair'], loc=(-0.12, -0.06, T + h * 0.62))
        arc.rotation_euler = (math.radians(90), 0, 0)
        rattacher(g, arc)
        rattacher(g, cylindre('corde', 0.005, 0.005, 0.37, '#EEEEEE', loc=(-0.12 + 0.0, -0.075, T + h * 0.62 - 0.185), cotes=3))
    if typ == 'eclaireur':
        rattacher(g, cylindre('baton', 0.012, 0.012, h + 0.3, p['bois'], loc=(-0.15, -0.02, T), cotes=4))
        rattacher(g, boite('fanion', 0.2, 0.008, 0.13, c, loc=(-0.25, -0.02, ep + 0.22)))
        rattacher(g, boite('cape', 0.2, 0.03, h * 0.7, mix(c, '#FFFFFF', 0.3), loc=(0, 0.09, T + h * 0.55), rot=(0.12, 0, 0)))
    if typ == 'coureur':
        for s in (-1, 1):
            rattacher(g, boite('lame', 0.035, 0.02, 0.26, p['metal'], loc=(s * 0.2, -0.08, T + h * 0.6), rot=(0.3, s * 0.5, 0)))
        rattacher(g, boite('foulard', 0.18, 0.02, 0.04, mix(c, '#FFFFFF', 0.3), loc=(0, 0.11, ep - 0.03), rot=(0.4, 0, 0)))
    if typ == 'pavois':
        rattacher(g, boite('pavois', 0.26, 0.04, 0.34, mix(c, '#000000', 0.35), loc=(0, -0.15, T + 0.2)))
        rattacher(g, boite('bande', 0.04, 0.045, 0.3, p['mur'], loc=(0, -0.155, T + 0.2)))
    if typ == 'astronome':
        rattacher(g, cylindre('lunette', 0.03, 0.022, 0.3, p['laiton'], loc=(0.1, -0.08, T + h * 0.7), rot=(-0.9, 0.5, 0), cotes=6))
    if typ == 'planeur':
        for s in (-1, 1):
            rattacher(g, prisme('aile', [(0, 0), (s * 0.32, 0.06), (s * 0.3, 0.14)], 0.012, mix(c, '#FFFFFF', 0.35), loc=(0, 0.02, ep + rt * 2.6)))
        rattacher(g, cylindre('mat', 0.01, 0.01, rt * 2.5, p['bois'], loc=(0, 0.06, ep + rt * 0.3), cotes=4))


def cheval(p, g, T):
    """Cheval de profil : corps selon x (vers la droite de l'écran), tête à droite."""
    c = p['cheval']
    rattacher(g, boite('corps', 0.36, 0.14, 0.13, c, loc=(0, 0.02, T + 0.21)))
    for x in (-0.13, 0.12):
        for y in (-0.03, 0.07):
            rattacher(g, cylindre('patte', 0.022, 0.018, 0.16, mix(c, '#000000', 0.25), loc=(x, y, T), cotes=4))
    rattacher(g, boite('cou', 0.09, 0.08, 0.17, c, loc=(0.17, 0.02, T + 0.32), rot=(0, 0.55, 0)))
    rattacher(g, boite('tete', 0.15, 0.08, 0.07, c, loc=(0.26, 0.02, T + 0.39), rot=(0, -0.35, 0)))
    rattacher(g, boite('criniere', 0.14, 0.03, 0.04, mix(c, '#000000', 0.45), loc=(0.14, 0.02, T + 0.37), rot=(0, 0.55, 0)))
    rattacher(g, boite('queue', 0.04, 0.03, 0.15, mix(c, '#000000', 0.45), loc=(-0.2, 0.02, T + 0.19), rot=(0, -0.5, 0)))


def catapulte(p, g, T, c):
    b = p['bois']
    g.scale = (1.45, 1.45, 1.45)
    g.location.z = -T * 0.45
    for x in (-0.09, 0.09):
        rattacher(g, boite('longeron', 0.04, 0.36, 0.04, b, loc=(x, 0, T + 0.07)))
        for y in (-0.13, 0.13):
            rattacher(g, cylindre('roue', 0.055, 0.055, 0.03, p['bois_clair'], loc=(x * 1.45, y, T + 0.055), rot=(0, math.radians(90), 0), cotes=8))
    rattacher(g, boite('traverse', 0.22, 0.04, 0.04, b, loc=(0, 0.02, T + 0.1)))
    for x in (-0.07, 0.07):
        rattacher(g, boite('montant', 0.035, 0.035, 0.22, b, loc=(x, 0.02, T + 0.2), rot=(0.25, 0, 0)))
    rattacher(g, boite('bras', 0.035, 0.035, 0.38, p['bois_clair'], loc=(0, -0.05, T + 0.3), rot=(-0.9, 0, 0)))
    rattacher(g, boule('pierre', 0.045, p['pierre'], loc=(0, -0.2, T + 0.44), subdiv=0))
    rattacher(g, cylindre('mat', 0.008, 0.008, 0.38, b, loc=(0.1, 0.15, T + 0.08), cotes=4))
    rattacher(g, boite('fanion', 0.005, 0.1, 0.06, c, loc=(0.1, 0.2, T + 0.42)))


# ---------- Villes et constructions ----------
PLACES = [(0.02, 0.18, 1.0), (-0.22, -0.02, 0.92), (0.22, 0.0, 0.92), (0.0, -0.22, 0.88), (-0.24, 0.26, 0.8), (0.26, 0.26, 0.78)]


def maison(p, x, y, f, T, toit_c, rotz=0.0):
    g = groupe('maison', loc=(x, y, 0), rotz=rotz)
    l, h = 0.2 * f, 0.14 * f
    rattacher(g, boite('murs', l, l * 1.15, h, p['mur'], loc=(0, 0, T + h / 2)))
    rattacher(g, toit('toit', l * 1.18, l * 1.35, l * 0.62, toit_c, loc=(0, 0, T + h)))
    rattacher(g, boite('porte', 0.004, l * 0.3, h * 0.55, p['bois'], loc=(l / 2 + 0.002, 0, T + h * 0.28)))
    return g


def drapeau(p, x, y, T, c, haut=0.42):
    cylindre('mat', 0.01, 0.01, haut, p['bois'], loc=(x, y, T), cotes=4)
    boite('drapeau', 0.006, 0.13, 0.08, c, loc=(x, y + 0.07, T + haut - 0.05))


def ville(p, k, niveau, T):
    c = couleur_clan(p, k)
    n = min(6, niveau + 1)
    for i in range(n):
        x, y, f = PLACES[i]
        maison(p, x, y, f, T, c, rotz=(0 if i % 2 else math.radians(90)))
    if niveau >= 5:
        cylindre('tour', 0.08, 0.075, 0.5, p['mur'], loc=(0.28, -0.26, T), cotes=6)
        cylindre('toit_tour', 0.11, 0.0, 0.16, c, loc=(0.28, -0.26, T + 0.5), cotes=6)
    drapeau(p, -0.32, -0.3, T, c)


def village(p, T):
    for (x, y, s) in [(-0.12, 0.12, 1.0), (0.17, -0.12, 0.85)]:
        cylindre('hutte', 0.15 * s, 0.15 * s, 0.13 * s, p['mur'], loc=(x, y, T), cotes=7)
        cylindre('chaume', 0.2 * s, 0.0, 0.22 * s, p['chaume'], loc=(x, y, T + 0.13 * s), cotes=7)
    boite('palissade', 0.02, 0.3, 0.06, p['bois'], loc=(-0.32, -0.05, T + 0.03))


def capitale(p, k, T):
    """Surcouche : grand étendard à couronne, au fond de la case."""
    c = couleur_clan(p, k)
    cylindre('mat', 0.014, 0.014, 0.62, p['bois'], loc=(0.3, 0.3, T), cotes=5)
    boite('etendard', 0.008, 0.18, 0.13, c, loc=(0.3, 0.4, T + 0.5))
    for i in range(5):
        a = i * 2 * math.pi / 5
        cylindre('pointe', 0.022, 0.0, 0.08, p['or'], loc=(0.3 + math.cos(a) * 0.055, 0.3 + math.sin(a) * 0.055, T + 0.66), cotes=4)
    cylindre('cercle', 0.065, 0.065, 0.04, p['or'], loc=(0.3, 0.3, T + 0.62), cotes=8)


def remparts(p, T, partie):
    """Surcouche de murailles : 'arriere' (fond, à dessiner avant la ville) ou 'avant'."""
    s = p['pierre']
    segs = {'arriere': [((-0.44, 0.0), (0.03, 0.82)), ((0.0, 0.44), (0.82, 0.03))],
            'avant': [((0.44, 0.0), (0.03, 0.82)), ((0.0, -0.44), (0.82, 0.03))]}[partie]
    for (x, y), (sx, sy) in segs:
        boite('mur', sx + 0.02, sy + 0.02, 0.09, s, loc=(x, y, T + 0.045))
        for i in range(5):
            t = -0.36 + i * 0.18
            boite('creneau', 0.05, 0.05, 0.04, s, loc=(x + (t if sx > sy else 0), y + (t if sy > sx else 0), T + 0.11))
    tours = {'arriere': [(-0.44, 0.44)], 'avant': [(0.44, -0.44), (0.44, 0.44), (-0.44, -0.44)]}[partie]
    for x, y in tours:
        cylindre('tour', 0.06, 0.055, 0.16, s, loc=(x, y, T), cotes=6)


def marche(p, T):
    for x, y in [(0.12, -0.34), (0.38, -0.34), (0.12, -0.14), (0.38, -0.14)]:
        cylindre('piquet', 0.012, 0.012, 0.22, p['bois'], loc=(x, y, T), cotes=4)
    boite('auvent', 0.34, 0.28, 0.025, '#E04A3A', loc=(0.25, -0.24, T + 0.23), rot=(0.25, 0, 0))
    boite('etal', 0.28, 0.14, 0.08, p['bois_clair'], loc=(0.25, -0.26, T + 0.04))
    for i, col in enumerate(['#F3B01D', '#7BB342', '#E04A3A']):
        boule('fruit', 0.032, col, loc=(0.16 + i * 0.09, -0.27, T + 0.1), subdiv=0)


def champ(p, T):
    for i in range(4):
        y = -0.3 + i * 0.2
        boite('sillon', 0.72, 0.09, 0.03, '#E8C65A', loc=(0, y, T + 0.015))
        for j in range(5):
            cylindre('epi', 0.022, 0.0, 0.09, '#D9A93A', loc=(-0.28 + j * 0.14, y, T + 0.03), cotes=4)


def cueillette(p, T):
    for (x, y) in [(0.18, -0.3), (0.32, -0.18)]:
        boule('buisson', 0.08, mix(p['F'], '#FFFFFF', 0.15), loc=(x, y, T + 0.05), echelle=(1, 1, 0.8), alea=0.1, graine=7)
        for d in (-0.03, 0.03):
            boule('baie', 0.022, '#C8102E', loc=(x + d, y - 0.06, T + 0.08), subdiv=0)
    cylindre('panier', 0.07, 0.085, 0.08, p['bois_clair'], loc=(0.34, -0.36, T), cotes=7)
    for i, col in enumerate(['#C8102E', '#7A2E8E', '#C8102E']):
        boule('baie', 0.026, col, loc=(0.31 + i * 0.03, -0.36 + (i - 1) * 0.02, T + 0.09), subdiv=0)


def carriere(p, T):
    for (x, y, sx, sy, sz) in [(0.26, -0.26, 0.16, 0.14, 0.11), (0.12, -0.36, 0.12, 0.12, 0.08), (0.36, -0.08, 0.11, 0.11, 0.07), (0.3, -0.3, 0.08, 0.08, 0.06)]:
        boite('bloc', sx, sy, sz, p['pierre'], loc=(x, y, T + sz / 2 + (0.11 if (x, y) == (0.3, -0.3) else 0)))


def ruine(p, T):
    s = mix(p['pierre'], '#000000', 0.05)
    cylindre('fut', 0.06, 0.06, 0.3, s, loc=(-0.12, 0.08, T), cotes=7)
    cylindre('fut', 0.06, 0.06, 0.16, s, loc=(0.14, 0.14, T), cotes=7)
    boite('linteau', 0.3, 0.08, 0.07, s, loc=(0.05, -0.15, T + 0.035), rot=(0, 0, 0.4))
    boule('mousse', 0.04, mix(p['P'], '#000000', 0.2), loc=(-0.12, 0.08, T + 0.3), echelle=(1, 1, 0.5), subdiv=1)


def merveille(p, k, T):
    """La Tour des Brumes (exemple) : spirale de blocs et anneau de nuages."""
    c = couleur_clan(p, k)
    cylindre('base', 0.3, 0.27, 0.08, p['pierre'], loc=(0, 0, T), cotes=8)
    for i in range(7):
        boite('etage', 0.2 - i * 0.012, 0.2 - i * 0.012, 0.08, p['mur'], loc=(0, 0, T + 0.12 + i * 0.08), rot=(0, 0, i * 0.3))
    cylindre('fleche', 0.1, 0.0, 0.18, c, loc=(0, 0, T + 0.68), cotes=6)
    for i in range(6):
        a = i * math.pi / 3
        boule('brume', 0.06, p['nuage'], loc=(math.cos(a) * 0.2, math.sin(a) * 0.2, T + 0.42), echelle=(1, 1, 0.6), subdiv=1)
