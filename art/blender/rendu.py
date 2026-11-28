# Rend les sprites du jeu en PNG transparents.
# Usage : python rendu.py --palette jour --taille 192 [--epaisseur epaisse|fine] [--ombrage facettes|doux]
#                         [--proportions trapu|elance] [--quoi terrains,unites,villes] [--sortie DOSSIER]
# Chaque image fait taille × 1,25 taille px ; le losange de la case occupe toute la largeur (taille × taille/2) ;
# l'ancre (centre du dessus de la case) est à (0,5 ; 0,68) de l'image, soit 0,85 taille depuis le haut.
import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib  # noqa: E402
import modeles as M  # noqa: E402

UNITES = ['eclaireur', 'lancier', 'archer', 'cavalier', 'catapulte']
PROPRES = {'crocs': 'coureur', 'mur': 'pavois', 'veil': 'astronome', 'vents': 'planeur'}


def liste(quoi):
    s = []
    if 'terrains' in quoi:
        s += [(f'terrain_{t}', 'terrain', (lambda t: lambda p, T, o: M.terrain(p, t, T))(t)) for t in 'PFCME']
        s += [('nuage', 'terrain', lambda p, T, o: M.nuage(p, T))]
    if 'unites' in quoi:
        for k in lib.CLANS:
            for u in UNITES + [PROPRES[k]]:
                s.append((f'unite_{k}_{u}', 'objet', (lambda k, u: lambda p, T, o: M.unite(p, k, u, T, o.proportions))(k, u)))
            s.append((f'anneau_{k}', 'objet', (lambda k: lambda p, T, o: M.anneau(p, k, T))(k)))
    if 'villes' in quoi:
        s.append(('village', 'objet', lambda p, T, o: M.village(p, T)))
        for k in lib.CLANS:
            for n in range(1, 6):
                s.append((f'ville_{k}_{n}', 'objet', (lambda k, n: lambda p, T, o: M.ville(p, k, n, T))(k, n)))
            s.append((f'capitale_{k}', 'objet', (lambda k: lambda p, T, o: M.capitale(p, k, T))(k)))
        s += [('remparts_arriere', 'objet', lambda p, T, o: M.remparts(p, T, 'arriere')),
              ('remparts_avant', 'objet', lambda p, T, o: M.remparts(p, T, 'avant')),
              ('marche', 'objet', lambda p, T, o: M.marche(p, T)),
              ('champ', 'objet', lambda p, T, o: M.champ(p, T)),
              ('cueillette', 'objet', lambda p, T, o: M.cueillette(p, T)),
              ('carriere', 'objet', lambda p, T, o: M.carriere(p, T)),
              ('ruine', 'objet', lambda p, T, o: M.ruine(p, T)),
              ('merveille_veil', 'objet', lambda p, T, o: M.merveille(p, 'veil', T))]
        for k in lib.CLANS:
            s += [(f'merveille_tour_{k}', 'objet', (lambda k: lambda p, T, o: M.merveille(p, k, T))(k)),
                  (f'merveille_arene_{k}', 'objet', (lambda k: lambda p, T, o: M.arene(p, k, T))(k)),
                  (f'merveille_verger_{k}', 'objet', (lambda k: lambda p, T, o: M.verger(p, k, T))(k))]
    return s


def nettoyer(chemin):
    """Efface le voile presque invisible que le capteur d'ombre laisse sur toute la case."""
    try:
        from PIL import Image
    except ImportError:
        return
    im = Image.open(chemin)
    px = im.load()
    for y in range(im.height):
        for x in range(im.width):
            r, g, b, a = px[x, y]
            if a < 10:
                px[x, y] = (0, 0, 0, 0)
    im.save(chemin)


def calque(chemin, taille, nuit):
    """Voile « vue autrefois », en 2D : le losange du dessus de la case, semi-transparent."""
    from PIL import Image, ImageDraw
    H = round(taille * lib.RATIO)
    im = Image.new('RGBA', (taille, H), (0, 0, 0, 0))
    cx, cy, w, h = taille / 2, H * lib.ANCRE_Y, taille / 2, taille / 4
    col = (12, 16, 24, 140) if nuit else (255, 255, 255, 148)
    ImageDraw.Draw(im).polygon([(cx, cy - h), (cx + w, cy), (cx, cy + h), (cx - w, cy)], fill=col)
    im.save(chemin)


def main():
    argv = sys.argv[sys.argv.index('--') + 1:] if '--' in sys.argv else sys.argv[1:]
    ap = argparse.ArgumentParser()
    ap.add_argument('--palette', default='jour')
    ap.add_argument('--taille', type=int, default=192)
    ap.add_argument('--epaisseur', default='epaisse')
    ap.add_argument('--ombrage', default='facettes')
    ap.add_argument('--proportions', default='trapu')
    ap.add_argument('--quoi', default='terrains,unites,villes')
    ap.add_argument('--seul', default='')
    ap.add_argument('--echantillons', type=int, default=48)
    ap.add_argument('--sortie', default='')
    o = ap.parse_args(argv)
    T = 0.2 if o.epaisseur == 'epaisse' else 0.06
    p = lib.PAL[o.palette]
    racine = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'sprites')
    dossier = o.sortie or os.path.join(racine, f'{o.epaisseur}-{o.ombrage}-{o.proportions}', o.palette, str(o.taille))
    os.makedirs(dossier, exist_ok=True)
    faits = []
    for nom, genre, fn in liste(o.quoi.split(',')):
        if o.seul and o.seul not in nom:
            continue
        sc = lib.nouvelle_scene()
        cam = lib.regler(sc, o.taille, o.ombrage, o.echantillons)
        fn(p, T, o)
        if genre == 'objet':
            lib.capteur_ombre(T)
        lib.viser(cam, (0, 0, T))
        lib.rendre(os.path.join(dossier, nom + '.png'))
        if genre == 'objet':
            nettoyer(os.path.join(dossier, nom + '.png'))
        faits.append(nom)
        print('ok', nom, flush=True)
    if not o.seul:
        calque(os.path.join(dossier, 'calque.png'), o.taille, o.palette == 'nuit')
        faits.append('calque')
    with open(os.path.join(dossier, 'sprites.json'), 'w', encoding='utf-8') as f:
        json.dump({'taille': [o.taille, round(o.taille * lib.RATIO)], 'ancre_px': [o.taille / 2, round(o.taille * lib.RATIO) * lib.ANCRE_Y], 'losange': [o.taille, o.taille // 2],
                   'epaisseur_case': T, 'variante': [o.epaisseur, o.ombrage, o.proportions], 'palette': o.palette,
                   'sprites': faits}, f, ensure_ascii=False, indent=1)


main()
