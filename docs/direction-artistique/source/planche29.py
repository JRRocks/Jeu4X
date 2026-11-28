# Planche DA-029 : merveilles et anneaux, assemblée à partir des sprites rendus.
from PIL import Image, ImageOps
R='/home/user/Jeu4X/art/sprites/epaisse-facettes-trapu/'
W,H=192,240
def tile(pal,layers,gris=False):
    im=Image.new('RGBA',(W,H),(0,0,0,0))
    for l in layers: im.alpha_composite(Image.open(f'{R}{pal}/192/{l}.png'))
    if gris: im=Image.merge('RGBA',(*[ImageOps.grayscale(im.convert('RGB'))]*3, im.split()[3]))
    return im
rows=[]
for pal,bg in [('jour',(243,230,204,255)),('nuit',(20,26,36,255))]:
    r1=[tile(pal,['terrain_P',f'merveille_{m}_{k}']) for m,k in [('tour','veil'),('arene','crocs'),('verger','mur'),('arene','vents')]]
    r2=[tile(pal,['terrain_P',f'anneau_{k}',f'unite_{k}_lancier']) for k in ['crocs','mur','veil','vents']]+[tile(pal,['terrain_F','anneau_veil','unite_veil_archer'])]
    rows.append((bg,r1+r2))
rows.append(((243,230,204,255),[tile('jour',['terrain_P',f'anneau_{k}',f'unite_{k}_lancier'],True) for k in ['crocs','mur','veil','vents']]))
out=Image.new('RGBA',(9*W+40,len(rows)*H+40),(233,231,226,255))
for i,(bg,ims) in enumerate(rows):
    out.alpha_composite(Image.new('RGBA',(9*W+20,H),bg),(10,20+i*H))
    for j,im in enumerate(ims): out.alpha_composite(im,(20+j*W,20+i*H))
out.convert('RGB').save('/home/user/Jeu4X/docs/direction-artistique/apercus/DA-029-3d-merveilles-anneaux.png',optimize=True)
