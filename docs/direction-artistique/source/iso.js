// Deuxième série (mission 2) : plateau en perspective, cases en losange, style papier découpé.
// Aucune adresse n'est dessinée (D-16) ; les coordonnées (c, r) ne servent qu'au calcul.
(function () {
  const { mix, rng } = Scene;

  // Carte de travail 11 × 11 : P plaine, F forêt, M montagne, E eau, S sable.
  const MAP = ['EEEPPFFMMPP', 'EEPPPFFMPPS', 'ESPPPPFPPSS', 'EPPFPPPPFPE', 'SPPFFPPMMPE', 'EPPPPPFMPPE',
    'EEPPSPPPPFE', 'EEPPPPFFPPE', 'EESPPPFMMPE', 'EEESPPPPPEE', 'EEEESPPPEEE'];
  const N = 11, T = (c, r) => MAP[r][c];

  const CLANS = {
    crocs: { nom: 'Crocs-Rouges', jour: '#BA1A0A', nuit: '#FF5C4D', propre: 'coureur', nomPropre: 'Coureur de crête' },
    mur: { nom: 'Murailliers', jour: '#F3B01D', nuit: '#FFE14A', propre: 'pavois', nomPropre: 'Porte-pavois' },
    veil: { nom: 'Veilleurs d’Astres', jour: '#4A45D1', nuit: '#8C7BFF', propre: 'astronome', nomPropre: 'Astronome' },
    vents: { nom: 'Vents-Errants', jour: '#5CCBFA', nuit: '#62D8F0', propre: 'planeur', nomPropre: 'Guetteur-planeur' },
  };

  const PAL = {
    jour: { id: 'jour', terr: PALETTES.jour.terr, ui: PALETTES.jour.ui, table: '#E2C9A0', side1: '#F1E4CA', side2: '#D8C5A2', edge: '#FFF8EA',
      blank: '#F6EDDC', skin: '#F2D3AE', ink: '#3A2A1C', shadow: '#3b2a1a', souv: '#CFC2A9', night: false },
    nuit: { id: 'nuit', terr: PALETTES.nuit.terr, ui: PALETTES.nuit.ui, table: '#0C1018', side1: '#4B4337', side2: '#342E26', edge: '#7A6E5A',
      blank: '#1B2230', skin: '#D8B793', ink: '#0B0E13', shadow: '#000000', souv: '#3A3F4A', night: true },
  };
  const clanCol = (k, pal) => CLANS[k][pal.night ? 'nuit' : 'jour'];
  const inkOn = c => Couleurs.contrast(c, '#FFFFFF') >= 3 ? '#FFFFFF' : '#2B2118';

  const pts = a => a.map(p => p.map(v => v.toFixed(1)).join(',')).join(' ');
  const diamond = (cx, cy, w, h) => [[cx, cy - h / 2], [cx + w / 2, cy], [cx, cy + h / 2], [cx - w / 2, cy]];
  // Losange au bord légèrement irrégulier, comme découpé aux ciseaux.
  function jitDiamond(cx, cy, w, h, R, a = 1.3) {
    const d = diamond(cx, cy, w, h), out = [];
    for (let i = 0; i < 4; i++) {
      const p = d[i], q = d[(i + 1) % 4];
      out.push(p);
      for (const t of [0.33, 0.66]) out.push([p[0] + (q[0] - p[0]) * t + (R() - .5) * a * 2, p[1] + (q[1] - p[1]) * t + (R() - .5) * a]);
    }
    return out;
  }
  // Prisme : faces gauche et droite de z0 à z1, puis le dessus.
  function prism(cx, cy, w, h, z0, z1, top, left, right, R, edge) {
    const L = `<polygon points="${pts([[cx - w / 2, cy - z1], [cx, cy + h / 2 - z1], [cx, cy + h / 2 - z0], [cx - w / 2, cy - z0]])}" fill="${left}"/>`;
    const Rt = `<polygon points="${pts([[cx, cy + h / 2 - z1], [cx + w / 2, cy - z1], [cx + w / 2, cy - z0], [cx, cy + h / 2 - z0]])}" fill="${right}"/>`;
    const tp = R ? jitDiamond(cx, cy - z1, w, h, R, 1) : diamond(cx, cy - z1, w, h);
    return (z1 > z0 ? L + Rt : '') + `<polygon points="${pts(tp)}" fill="${top}"${edge ? ` stroke="${edge}" stroke-width="1.2" stroke-linejoin="round"` : ''}/>`;
  }

  // Hauteur du dessus de la case, selon la variante de terrain.
  const SURF = { carton: { E: 1, S: 5, C: 7, P: 7, F: 7, M: 7 }, popup: { E: 2, S: 4, C: 4, P: 4, F: 4, M: 4 }, plat: { E: 0, S: 0, C: 0, P: 0, F: 0, M: 0 } };

  // ---------- Décors ----------
  function tree(x, y, s, pal, col, flat) {
    const dk = mix(col, '#000000', .3), lt = mix(col, '#FFFFFF', .18);
    if (flat) return `<ellipse cx="${x}" cy="${y}" rx="${7 * s}" ry="${3.6 * s}" fill="${dk}"/><ellipse cx="${x - 1}" cy="${y - 1}" rx="${5 * s}" ry="${2.5 * s}" fill="${lt}"/>`;
    return `<ellipse cx="${x}" cy="${y}" rx="${6 * s}" ry="${2.6 * s}" fill="${pal.shadow}" opacity=".22"/>`
      + `<rect x="${x - 1.2 * s}" y="${y - 4 * s}" width="${2.4 * s}" height="${4 * s}" fill="#7A5232"/>`
      + `<polygon points="${pts([[x, y - 20 * s], [x - 7 * s, y - 3 * s], [x, y - 4.5 * s]])}" fill="${lt}"/>`
      + `<polygon points="${pts([[x, y - 20 * s], [x + 7 * s, y - 3 * s], [x, y - 4.5 * s]])}" fill="${dk}"/>`;
  }
  function decor(v, t, cx, cy, W, H, pal, colf, R) {
    const terr = pal.terr, k = W / 84;
    let g = '';
    if (t === 'F') {
      const c = colf(terr.F);
      for (const [dx, dy, s] of [[-.17, -.02, 1], [.15, -.06, .9], [0, .12, 1.05], [.22, .1, .8], [-.24, .14, .8]]) g += tree(cx + dx * W, cy + dy * W, s * k, pal, c, v === 'plat');
    }
    if (t === 'M') {
      const c = colf(terr.M), snow = colf(pal.night ? '#C9CED8' : '#FFFFFF');
      if (v === 'carton') {
        g += prism(cx, cy - 7, W * .66, H * .66, 0, 7, colf(mix(terr.M, '#FFFFFF', .08)), colf(pal.side1), colf(pal.side2));
        g += prism(cx, cy - 14, W * .4, H * .4, 0, 7, colf(mix(terr.M, '#FFFFFF', .16)), colf(pal.side1), colf(pal.side2));
        g += prism(cx, cy - 21, W * .18, H * .18, 0, 4, snow, colf(pal.side1), colf(pal.side2));
      } else if (v === 'popup') {
        g += `<ellipse cx="${cx}" cy="${cy + 2}" rx="${W * .3}" ry="${H * .2}" fill="${pal.shadow}" opacity=".2"/>`;
        g += `<path d="M${cx - W * .3} ${cy + 3}Q${cx - W * .12} ${cy - H * 1.05} ${cx + W * .02} ${cy - H * 1.1}Q${cx + W * .16} ${cy - H * .5} ${cx + W * .3} ${cy + 3}Z" fill="${mix(c, '#000000', .08)}" stroke="${colf(pal.edge)}" stroke-width="2"/>`
          + `<path d="M${cx - W * .07} ${cy - H * .82}Q${cx} ${cy - H * 1.1} ${cx + W * .02} ${cy - H * 1.1}Q${cx + W * .07} ${cy - H * .9} ${cx + W * .1} ${cy - H * .76}Q${cx + W * .02} ${cy - H * .84} ${cx - W * .07} ${cy - H * .82}Z" fill="${snow}"/>`;
      } else {
        g += `<path d="M${cx - W * .26} ${cy + 2}L${cx - W * .06} ${cy - H * .32}L${cx + W * .04} ${cy - H * .12}L${cx + W * .12} ${cy - H * .26}L${cx + W * .27} ${cy + 3}Z" fill="${mix(c, '#000000', .28)}"/><path d="M${cx - W * .06} ${cy - H * .32}L${cx - W * .12} ${cy - H * .2}L${cx} ${cy - H * .2}Z" fill="${snow}"/>`;
      }
    }
    if (t === 'E') {
      const c = colf(mix(terr.E, '#FFFFFF', .4));
      for (const [dx, dy] of [[-.12, -.06], [.1, .08]]) { const x = cx + dx * W, y = cy + dy * W; g += `<path d="M${x - 10 * k} ${y + 2.5 * k}q${3.3 * k} -4 ${6.6 * k} -1.6t${6.6 * k} -1.6t${6.6 * k} -1.6" stroke="${c}" stroke-width="${2.4 * k}" fill="none" stroke-linecap="round"/>`; }
    }
    if (t === 'C') {
      const c = colf(terr.C), ed = colf(pal.edge);
      g += `<ellipse cx="${cx}" cy="${cy + 2}" rx="${W * .3}" ry="${H * .18}" fill="${pal.shadow}" opacity=".18"/>`;
      g += `<path d="M${cx - W * .32} ${cy + 3}Q${cx - W * .17} ${cy - H * .62} ${cx - W * .02} ${cy + 3}Z" fill="${mix(c, '#000000', .06)}" stroke="${ed}" stroke-width="2"/>`;
      g += `<path d="M${cx - W * .1} ${cy + 5}Q${cx + W * .1} ${cy - H * .78} ${cx + W * .3} ${cy + 5}Z" fill="${mix(c, '#FFFFFF', .08)}" stroke="${ed}" stroke-width="2"/>`;
    }
    if (t === 'S') g += `<path d="M${cx - W * .22} ${cy + H * .1}q${W * .14} ${-H * .3} ${W * .3} ${-H * .08}q${W * .06} ${-H * .08} ${W * .14} ${H * .02}" stroke="${colf(mix(terr.S, '#000000', .18))}" stroke-width="${2.2 * k}" fill="none" stroke-linecap="round"/>`;
    if (t === 'P') for (const [dx, dy] of [[-.14, .02], [.12, -.06], [.04, .14]]) { const x = cx + dx * W, y = cy + dy * W; g += `<path d="M${x - 3 * k} ${y}l${1 * k} ${-4.5 * k}M${x} ${y}v${-6 * k}M${x + 3 * k} ${y}l${-1 * k} ${-4.5 * k}" stroke="${colf(mix(terr.P, '#000000', .3))}" stroke-width="${1.6 * k}" stroke-linecap="round"/>`; }
    return g;
  }

  // ---------- Une case ----------
  // state : 'vue', 'souvenir' ou 'jamais' ; fog : 'papier', 'calque' ou 'encre'.
  function tile(v, t, cx, cy, W, H, pal, R, state, fog, extra) {
    const z = SURF[v][t];
    if (state === 'jamais') {
      if (fog === 'calque') {
        let g = prism(cx, cy, W - 1, H - .5, 0, 3, pal.night ? '#2A3140' : '#ECE8E0', pal.night ? '#202634' : '#D9D3C8', pal.night ? '#181D28' : '#C9C2B5', R);
        const b = (x, y, r) => `<circle cx="${x}" cy="${y}" r="${r}" fill="${pal.night ? '#3A4254' : '#FFFFFF'}" stroke="${pal.night ? '#2A3140' : '#D9D3C8'}" stroke-width="1"/>`;
        return g + `<g opacity=".95">${b(cx - W * .12, cy - 8, W * .13)}${b(cx + W * .1, cy - 10, W * .15)}${b(cx, cy - 4, W * .14)}</g>`;
      }
      if (fog === 'encre') return `<polygon points="${pts(diamond(cx, cy, W, H))}" fill="${pal.night ? '#06080C' : '#2E2A26'}" stroke="${pal.night ? '#141821' : '#3A352F'}" stroke-width="1"/>`;
      return `<polygon points="${pts(jitDiamond(cx, cy, W - 1, H - .5, R, 1.2))}" fill="${pal.blank}" stroke="${pal.night ? '#252D3D' : '#E3D6BE'}" stroke-width="1"/>`;
    }
    const souv = state === 'souvenir';
    const colf = souv && fog !== 'calque' ? (c => mix(c, pal.souv, .68)) : (c => c);
    const top = colf(extra && extra.tint ? mix(pal.terr[t], '#FFFFFF', extra.tint) : pal.terr[t]);
    const lift = extra && extra.lift || 0;
    let g = '';
    if (v === 'carton') {
      if (t === 'E') g += prism(cx, cy, W, H, 0, z, top, colf(mix(pal.terr.E, '#000000', .25)), colf(mix(pal.terr.E, '#000000', .4)));
      else g += prism(cx, cy, W - 1.5, H - .75, 0, z + lift, top, colf(pal.side1), colf(pal.side2), null, colf(pal.edge));
    } else if (v === 'popup') {
      g += prism(cx, cy, W - 2.5, H - 1.2, 0, z + lift, top, colf(pal.edge), colf(mix(pal.edge, '#000000', .12)), R);
    } else {
      g += `<polygon points="${pts(jitDiamond(cx, cy - lift, W - 2, H - 1, R, 1.6))}" fill="${top}"/>`;
    }
    g += decor(v, t, cx, cy - z - lift, W, H, pal, colf, R);
    if (souv && fog === 'calque') g += `<polygon points="${pts(diamond(cx, cy - z - 1, W, H))}" fill="${pal.night ? '#0C1018' : '#FFFFFF'}" fill-opacity="${pal.night ? .55 : .58}"/>`;
    if (souv && fog === 'encre') g += `<polygon points="${pts(diamond(cx, cy - z, W - 2, H - 1))}" fill="url(#hach-${pal.id})"/>`;
    return g;
  }

  // ---------- Unités : personnages découpés, debout ----------
  // Socle propre à chaque clan (vu en perspective) : dents, carré, étoile, goutte.
  function socle(k, cx, cy, s, fill, stroke) {
    const sq = (x, y) => [cx + x * s, cy + y * s * .5];
    let p;
    if (k === 'crocs') { p = []; for (let i = 0; i < 16; i++) { const a = i / 16 * Math.PI * 2, r = i % 2 ? 9 : 13; p.push(sq(Math.cos(a) * r, Math.sin(a) * r)); } }
    else if (k === 'veil') { p = []; for (let i = 0; i < 8; i++) { const a = i / 8 * Math.PI * 2 - Math.PI / 2, r = i % 2 ? 5.5 : 14; p.push(sq(Math.cos(a) * r, Math.sin(a) * r)); } }
    else { p = [sq(17, 0)]; for (let i = 0; i <= 16; i++) { const a = (40 + i * 280 / 16) * Math.PI / 180; p.push(sq(-3 + Math.cos(a) * 10, Math.sin(a) * 10)); } }
    if (k === 'mur') p = [sq(-11, -11), sq(11, -11), sq(11, 11), sq(-11, 11)];
    return `<polygon points="${pts(p)}" fill="${fill}" stroke="${stroke}" stroke-width="2" stroke-linejoin="round"/>`;
  }
  // Petit emblème de clan (boîte 10 × 10 centrée).
  function emblem(k, x, y, s, col) {
    const T = (d) => `<path d="${d}" transform="translate(${x - 5 * s} ${y - 5 * s}) scale(${s})" fill="${col}"/>`;
    if (k === 'crocs') return T('M1 1H9L7.5 4L6 9L5 4L4 9L2.5 4Z');
    if (k === 'mur') return T('M1 9V2H3V4H4.2V2H5.8V4H7V2H9V9Z');
    if (k === 'veil') return T('M5 0L6.2 3.8L10 5L6.2 6.2L5 10L3.8 6.2L0 5L3.8 3.8Z');
    return T('M1 6Q3 1 7 3Q9 4 8 6Q7 4.6 5 5.5Q3 6.5 4 9Q0.5 8.5 1 6Z');
  }
  function figure(type, k, cx, cy, pal, s = 1, opt = {}) {
    const col = clanCol(k, pal), dk = mix(col, '#000000', .35), wh = pal.night ? '#E8ECF2' : '#FFFFFF', sk = pal.skin;
    const X = v => cx + v * s, Y = v => cy + v * s;
    let g = `<ellipse cx="${cx}" cy="${cy}" rx="${15 * s}" ry="${7 * s}" fill="${pal.shadow}" opacity="${opt.sel ? .32 : .22}"/>`;
    g += socle(k, cx, cy - 1 * s, s, col, wh);
    const lift = opt.sel ? 5 : 0;
    const by = v => Y(v) - lift;
    let back = '', front = '';
    // armes et attributs
    if (type === 'lancier') front += `<path d="M${X(10)} ${by(-1)}L${X(12)} ${by(-40)}" stroke="#7A5232" stroke-width="${2.4 * s}" stroke-linecap="round"/><path d="M${X(12)} ${by(-46)}L${X(14.6)} ${by(-38)}L${X(9.6)} ${by(-38.4)}Z" fill="#8C96A8" stroke="${wh}" stroke-width="${1 * s}"/>`;
    if (type === 'guerrier') front += `<path d="M${X(9)} ${by(-6)}L${X(14)} ${by(-30)}" stroke="#8C96A8" stroke-width="${3 * s}" stroke-linecap="round"/><path d="M${X(7.5)} ${by(-12)}L${X(12.5)} ${by(-10.5)}" stroke="#5A4128" stroke-width="${2.4 * s}" stroke-linecap="round"/>`;
    if (type === 'archer') front += `<path d="M${X(-11)} ${by(-34)}Q${X(-20)} ${by(-20)} ${X(-11)} ${by(-6)}" stroke="#7A5232" stroke-width="${2.6 * s}" fill="none" stroke-linecap="round"/><path d="M${X(-11)} ${by(-34)}L${X(-11)} ${by(-6)}" stroke="${wh}" stroke-width="${1 * s}"/>`;
    if (type === 'eclaireur') back += `<path d="M${X(-9)} ${by(-2)}L${X(-12)} ${by(-40)}" stroke="#7A5232" stroke-width="${2.4 * s}" stroke-linecap="round"/><path d="M${X(-12)} ${by(-40)}L${X(-1)} ${by(-36)}L${X(-11)} ${by(-31)}Z" fill="${col}" stroke="${wh}" stroke-width="${1.2 * s}"/>`;
    if (type === 'coureur') front += `<path d="M${X(-10)} ${by(-14)}L${X(-17)} ${by(-24)}M${X(10)} ${by(-14)}L${X(17)} ${by(-24)}" stroke="#8C96A8" stroke-width="${2.6 * s}" stroke-linecap="round"/>`, back += `<path d="M${X(-22)} ${by(-12)}h${-6 * s}M${X(-22)} ${by(-18)}h${-9 * s}M${X(-22)} ${by(-24)}h${-6 * s}" stroke="${col}" stroke-width="${2 * s}" stroke-linecap="round" opacity=".8"/>`;
    if (type === 'pavois') front += `<path d="M${X(-11)} ${by(-34)}H${X(11)}V${by(-8)}Q${X(0)} ${by(0)} ${X(-11)} ${by(-8)}Z" fill="${dk}" stroke="${wh}" stroke-width="${2 * s}"/>` + emblem(k, X(0), by(-21), 1.3 * s, wh);
    if (type === 'astronome') front += `<path d="M${X(4)} ${by(-18)}L${X(20)} ${by(-34)}" stroke="#C99A3A" stroke-width="${4.5 * s}" stroke-linecap="round"/><path d="M${X(4)} ${by(-18)}L${X(20)} ${by(-34)}" stroke="#F2D27A" stroke-width="${1.6 * s}" stroke-linecap="round"/>`;
    if (type === 'planeur') back += `<path d="M${X(-26)} ${by(-36)}Q${X(0)} ${by(-48)} ${X(26)} ${by(-36)}L${X(0)} ${by(-42)}Z" fill="${col}" stroke="${wh}" stroke-width="${1.8 * s}" stroke-linejoin="round"/><path d="M${X(0)} ${by(-42)}V${by(-30)}" stroke="#7A5232" stroke-width="${1.6 * s}"/>`;
    if (type === 'cavalier' || type === 'catapulte') return g + monture(type, k, cx, cy, pal, s, lift, col, dk, wh, sk, opt);
    // corps
    const body = `<path d="M${X(-8)} ${by(-3)}L${X(-5.5)} ${by(-22)}Q${X(0)} ${by(-25)} ${X(5.5)} ${by(-22)}L${X(8)} ${by(-3)}Z" fill="${col}" stroke="${wh}" stroke-width="${2.2 * s}" stroke-linejoin="round"/>`;
    const chest = type === 'pavois' ? '' : emblem(k, X(0), by(-13), 1 * s, inkOn(col));
    const head = `<circle cx="${X(0)}" cy="${by(-29)}" r="${6 * s}" fill="${sk}" stroke="${wh}" stroke-width="${1.8 * s}"/>`;
    // coiffes propres à chaque clan
    let hat = '';
    if (k === 'crocs') hat = `<path d="M${X(-6)} ${by(-31)}L${X(-5)} ${by(-40)}L${X(-2)} ${by(-34)}L${X(0)} ${by(-43)}L${X(2)} ${by(-34)}L${X(5)} ${by(-40)}L${X(6)} ${by(-31)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.2 * s}" stroke-linejoin="round"/>`;
    if (k === 'mur') hat = `<path d="M${X(-7.5)} ${by(-30)}V${by(-37)}H${X(7.5)}V${by(-30)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.2 * s}"/>`;
    if (k === 'veil') hat = `<path d="M${X(-7)} ${by(-27)}Q${X(-6)} ${by(-36)} ${X(0)} ${by(-46)}Q${X(6)} ${by(-36)} ${X(7)} ${by(-27)}Q${X(0)} ${by(-33)} ${X(-7)} ${by(-27)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.2 * s}"/>` + emblem('veil', X(0), by(-38), .55 * s, '#FFE14A');
    if (k === 'vents') hat = `<path d="M${X(-10)} ${by(-32)}Q${X(0)} ${by(-38)} ${X(10)} ${by(-32)}Q${X(0)} ${by(-35)} ${X(-10)} ${by(-32)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.2 * s}"/><path d="M${X(4)} ${by(-22)}Q${X(14)} ${by(-24)} ${X(19)} ${by(-18)}Q${X(13)} ${by(-20)} ${X(5)} ${by(-19)}Z" fill="${mix(col, '#FFFFFF', .35)}" stroke="${wh}" stroke-width="${1 * s}"/>`;
    g += `<g filter="url(#ombre)">${back}${body}${chest}${head}${hat}${front}</g>`;
    return g;
  }
  // Cavalier (personnage sur un cheval de papier) et catapulte (machine de bois au fanion du clan).
  function monture(type, k, cx, cy, pal, s, lift, col, dk, wh, sk, opt) {
    const X = v => cx + v * s, by = v => cy + v * s - lift;
    let g = '';
    if (type === 'cavalier') {
      const horse = '#C9A27A', hd = '#9C7650';
      g += `<g filter="url(#ombre)">`
        + `<path d="M${X(-12)} ${by(-13)}L${X(-14)} ${by(-1)}M${X(-7)} ${by(-13)}L${X(-6)} ${by(-1)}M${X(5)} ${by(-13)}L${X(4)} ${by(-1)}M${X(9)} ${by(-13)}L${X(11)} ${by(-1)}" stroke="${hd}" stroke-width="${2.6 * s}" stroke-linecap="round"/>`
        + `<path d="M${X(-15)} ${by(-17)}Q${X(-22)} ${by(-14)} ${X(-20)} ${by(-5)}" stroke="${hd}" stroke-width="${3 * s}" fill="none" stroke-linecap="round"/>`
        + `<ellipse cx="${X(-2)}" cy="${by(-16)}" rx="${14 * s}" ry="${5.5 * s}" fill="${horse}" stroke="${wh}" stroke-width="${2 * s}"/>`
        + `<path d="M${X(7)} ${by(-19)}L${X(13)} ${by(-31)}L${X(17)} ${by(-31)}L${X(13)} ${by(-15)}Z" fill="${horse}" stroke="${wh}" stroke-width="${1.8 * s}" stroke-linejoin="round"/>`
        + `<path d="M${X(12)} ${by(-31)}L${X(21)} ${by(-28)}Q${X(23)} ${by(-25)} ${X(20)} ${by(-24)}L${X(14)} ${by(-25)}Z" fill="${horse}" stroke="${wh}" stroke-width="${1.6 * s}" stroke-linejoin="round"/>`
        + `<path d="M${X(13)} ${by(-31)}L${X(14)} ${by(-35)}L${X(16)} ${by(-31)}" fill="${hd}"/><path d="M${X(11)} ${by(-30)}L${X(8)} ${by(-19)}" stroke="${hd}" stroke-width="${2 * s}" stroke-linecap="round"/>`
        + `<path d="M${X(-7)} ${by(-20)}L${X(-5)} ${by(-34)}Q${X(0)} ${by(-36)} ${X(4)} ${by(-34)}L${X(5)} ${by(-20)}Z" fill="${col}" stroke="${wh}" stroke-width="${2 * s}" stroke-linejoin="round"/>`
        + emblem(k, X(-.5), by(-27), .8 * s, inkOn(col))
        + `<circle cx="${X(-.5)}" cy="${by(-40)}" r="${5 * s}" fill="${sk}" stroke="${wh}" stroke-width="${1.6 * s}"/>`
        + coiffe(k, v => X(v - 1), v => by(v - 11), s, dk, wh)
        + `<path d="M${X(-9)} ${by(-24)}L${X(-12)} ${by(-46)}" stroke="#7A5232" stroke-width="${2 * s}" stroke-linecap="round"/><path d="M${X(-12)} ${by(-50)}L${X(-10)} ${by(-44)}L${X(-14)} ${by(-44)}Z" fill="#8C96A8"/></g>`;
      return g;
    }
    // catapulte
    g += `<g filter="url(#ombre)"><path d="M${X(-16)} ${by(-8)}H${X(14)}" stroke="#7A5232" stroke-width="${5 * s}" stroke-linecap="round"/>`
      + `<circle cx="${X(-11)}" cy="${by(-4)}" r="${5 * s}" fill="#9C7650" stroke="${wh}" stroke-width="${1.6 * s}"/><circle cx="${X(9)}" cy="${by(-4)}" r="${5 * s}" fill="#9C7650" stroke="${wh}" stroke-width="${1.6 * s}"/>`
      + `<path d="M${X(-4)} ${by(-9)}L${X(0)} ${by(-24)}L${X(4)} ${by(-9)}" stroke="#7A5232" stroke-width="${3 * s}" fill="none" stroke-linejoin="round"/>`
      + `<path d="M${X(0)} ${by(-22)}L${X(-16)} ${by(-36)}" stroke="#A47A50" stroke-width="${3 * s}" stroke-linecap="round"/><circle cx="${X(-17)}" cy="${by(-37)}" r="${3.6 * s}" fill="#8C96A8" stroke="${wh}" stroke-width="${1 * s}"/>`
      + `<path d="M${X(12)} ${by(-9)}V${by(-34)}" stroke="#5A4128" stroke-width="${1.6 * s}"/><path d="M${X(12)} ${by(-34)}h${12 * s}l${-3 * s} ${4 * s}l${3 * s} ${4 * s}h${-12 * s}z" fill="${col}" stroke="${wh}" stroke-width="${1.2 * s}"/>`
      + emblem(k, X(17), by(-30), .5 * s, inkOn(col)) + `</g>`;
    return g;
  }
  // Coiffe de clan, réutilisée par le cavalier.
  function coiffe(k, X, by, s, dk, wh) {
    if (k === 'crocs') return `<path d="M${X(-5)} ${by(-31)}L${X(-4)} ${by(-38)}L${X(-1.5)} ${by(-33)}L${X(.5)} ${by(-41)}L${X(2.5)} ${by(-33)}L${X(5)} ${by(-38)}L${X(6)} ${by(-31)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.1 * s}" stroke-linejoin="round"/>`;
    if (k === 'mur') return `<path d="M${X(-6)} ${by(-30)}V${by(-36)}H${X(7)}V${by(-30)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.1 * s}"/>`;
    if (k === 'veil') return `<path d="M${X(-5.5)} ${by(-28)}Q${X(-5)} ${by(-36)} ${X(.5)} ${by(-44)}Q${X(6)} ${by(-36)} ${X(6.5)} ${by(-28)}Q${X(.5)} ${by(-33)} ${X(-5.5)} ${by(-28)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.1 * s}"/>`;
    return `<path d="M${X(-8.5)} ${by(-32)}Q${X(.5)} ${by(-37)} ${X(9.5)} ${by(-32)}Q${X(.5)} ${by(-34.5)} ${X(-8.5)} ${by(-32)}Z" fill="${dk}" stroke="${wh}" stroke-width="${1.1 * s}"/>`;
  }
  // Ruine : deux colonnes brisées et un bloc tombé.
  function ruine(cx, cy, W, pal) {
    const st = pal.night ? '#8A8A96' : '#B9B4AA', H = W / 2;
    return `<g filter="url(#ombre)">` + prism(cx - W * .12, cy - 2, W * .1, H * .1, 0, 16, mix(st, '#FFFFFF', .2), mix(st, '#000000', .05), mix(st, '#000000', .2))
      + prism(cx + W * .1, cy - 5, W * .1, H * .1, 0, 9, mix(st, '#FFFFFF', .2), mix(st, '#000000', .05), mix(st, '#000000', .2))
      + prism(cx + W * .02, cy + 6, W * .16, H * .1, 0, 4, mix(st, '#FFFFFF', .2), mix(st, '#000000', .05), mix(st, '#000000', .2)) + `</g>`
      + `<ellipse cx="${cx - W * .12}" cy="${cy - 18}" rx="3" ry="1.5" fill="#6F9A4A"/>`;
  }
  // Exploitations, posées à plat sur la case (champ, cueillette, carrière).
  function exploitation(kind, cx, cy, W, pal) {
    const H = W / 2;
    if (kind === 'champ') { const P = (a, b) => [cx + (a - b) * W / 2, cy + (a + b) * H / 2]; let g = ''; for (const f of [-.22, 0, .22]) { const [x1, y1] = P(-.34, f), [x2, y2] = P(.34, f); g += `<path d="M${x1} ${y1}L${x2} ${y2}" stroke="#E8C65A" stroke-width="${W / 16}" stroke-linecap="round"/>`; } return g; }
    if (kind === 'cueillette') return `<g filter="url(#ombre)"><path d="M${cx + W * .1} ${cy + H * .18}h${12}l-2 7h-8z" fill="#B8864E" stroke="#FFFFFF" stroke-width="1.2"/><circle cx="${cx + W * .1 + 3}" cy="${cy + H * .18 - 1}" r="2.2" fill="#C8102E"/><circle cx="${cx + W * .1 + 8}" cy="${cy + H * .18 - 1.5}" r="2.2" fill="#7A2E8E"/></g>`;
    return `<g filter="url(#ombre)">` + prism(cx + W * .14, cy + H * .16, W * .12, H * .12, 0, 5, '#D8D4CC', '#BDB8AE', '#A39E94') + prism(cx + W * .24, cy + H * .06, W * .1, H * .1, 0, 4, '#D8D4CC', '#BDB8AE', '#A39E94') + `</g>`;
  }
  // Variante « jeton à plat » : la forme du socle agrandie, symbole de l'unité dessus.
  function jeton(type, k, cx, cy, pal, s = 1) {
    const col = clanCol(k, pal), wh = pal.night ? '#E8ECF2' : '#FFFFFF';
    const gl = { guerrier: 'lance', archer: 'arc', eclaireur: 'hache', coureur: 'hache', pavois: 'lance', astronome: 'arc', planeur: 'hache' }[type];
    return `<ellipse cx="${cx}" cy="${cy + 2}" rx="${24 * s}" ry="${11 * s}" fill="${pal.shadow}" opacity=".25"/>` + `<g filter="url(#ombre)">${socle(k, cx, cy - 2, 1.75 * s, col, wh)}</g>`
      + `<g transform="translate(${cx} ${cy - 2}) scale(1 .5) translate(${-cx} ${-(cy - 2)})">${Scene.placeGlyph(gl, inkOn(col), cx, cy - 2, 22 * s, 2.8)}</g>`
      + emblem(k, cx, cy - 14 * s, 1.1 * s, col);
  }

  // ---------- Villages et villes ----------
  function house(x, y, w, hz, wall, roof, pal) {
    const h = w / 2, wl = mix(wall, '#000000', .1), wr = mix(wall, '#000000', .22);
    let g = prism(x, y, w, h, 0, hz, wall, wl, wr);
    const top = y - hz;
    // toit à deux pans, faîte parallèle à la diagonale
    g += `<polygon points="${pts([[x - w / 2, top], [x - w / 4, top - h * .9 - 4], [x + w / 4, top - h * .4 - 4], [x, top + h / 2]])}" fill="${mix(roof, '#FFFFFF', .12)}" stroke="${pal.night ? '#E8ECF2' : '#FFFFFF'}" stroke-width="1"/>`;
    g += `<polygon points="${pts([[x, top + h / 2], [x + w / 4, top - h * .4 - 4], [x + w / 2 + 1, top], [x + w / 2, top]])}" fill="${mix(roof, '#000000', .2)}"/>`;
    return g;
  }
  const flag = (x, y, col, pal, k) => `<path d="M${x} ${y}V${y - 26}" stroke="#5A4128" stroke-width="1.8"/><path d="M${x} ${y - 26}h14l-3.5 4.5l3.5 4.5h-14z" fill="${col}" stroke="${pal.night ? '#E8ECF2' : '#FFFFFF'}" stroke-width="1.2"/>` + (k ? emblem(k, x + 6, y - 21.5, .55, inkOn(col)) : '');
  function ville(kind, size, k, cx, cy, W, pal, opt = {}) {
    const H = W / 2, wall = pal.night ? '#D9D2C4' : '#FFF6E5', col = k ? clanCol(k, pal) : (pal.night ? '#8A8478' : '#B8A98E');
    let g = `<g filter="url(#ombre)">`;
    if (kind === 'empile') {
      // ville en gradins empilés, un gradin par niveau
      for (let i = 0; i < size; i++) { const f = 1 - i * .2; g += prism(cx, cy - i * 7, W * .62 * f, H * .62 * f, 0, 7, i === size - 1 ? col : wall, mix(wall, '#000000', .1), mix(wall, '#000000', .22)); }
      g += `</g>` + flag(cx, cy - size * 7 - 2, col, pal, k);
      return g;
    }
    const spots = [[0, -.12, 1], [-.2, .02, .9], [.2, .02, .9], [0, .15, .85], [-.06, -.25, .8], [.14, -.2, .75]];
    const n = kind === 'village' ? 2 : Math.min(6, size + 1);
    const order = [3, 1, 2, 0, 4, 5].slice(0, n).sort((a, b) => spots[a][1] - spots[b][1]);
    if (kind !== 'village' && (opt.remparts ?? size >= 3)) g += `<polyline points="${pts([[cx - W * .4, cy], [cx, cy + H * .4], [cx + W * .4, cy]])}" fill="none" stroke="${mix(wall, '#000000', .25)}" stroke-width="5" stroke-linejoin="round"/>`;
    for (const i of order) { const [dx, dy, f] = spots[i]; g += house(cx + dx * W, cy + dy * W, W * .24 * f, 10 * f, wall, kind === 'village' ? col : col, pal); }
    if (kind !== 'village' && (opt.remparts ?? size >= 3)) g += `<polyline points="${pts([[cx - W * .4, cy + 1], [cx, cy + H * .4 + 1], [cx + W * .4, cy + 1]])}" fill="none" stroke="${wall}" stroke-width="3" stroke-linejoin="round"/>`;
    if (kind !== 'village' && size >= 5) g += prism(cx + W * .26, cy - W * .1, W * .14, H * .14, 0, 22, col, mix(wall, '#000000', .1), mix(wall, '#000000', .22));
    g += `</g>`;
    if (kind !== 'village' && opt.marche) g += `<g filter="url(#ombre)"><path d="M${cx - W * .02} ${cy + H * .3}v-7M${cx + W * .16} ${cy + H * .2}v-7" stroke="#7A5232" stroke-width="1.5"/><path d="M${cx - W * .05} ${cy + H * .3 - 7}l${W * .24} ${-H * .12}l-2 -4l${-W * .24} ${H * .12}z" fill="#E04A3A" stroke="#FFFFFF" stroke-width="1"/></g>`;
    if (kind !== 'village') g += flag(cx - W * .3, cy - 4, col, pal, k);
    if (kind !== 'village' && opt.capitale) g += `<path d="M${cx - W * .3 - 4} ${cy - 36}l2.5 -6l2.5 4l2.5 -4l2.5 6z" fill="#F3B01D" stroke="#FFFFFF" stroke-width="1"/>`;
    return g;
  }
  // Étiquette de ville : nom et niveau en pastilles.
  function etiquette(nom, size, k, cx, cy, pal) {
    const col = k ? clanCol(k, pal) : '#9A8F86', w = nom.length * 7 + 16 + size * 7;
    let dots = ''; for (let i = 0; i < size; i++) dots += `<circle cx="${cx + w / 2 - 8 - i * 7}" cy="${cy}" r="2.6" fill="${col}"/>`;
    return `<g filter="url(#ombre)"><rect x="${cx - w / 2}" y="${cy - 10}" width="${w}" height="20" rx="10" fill="${pal.night ? '#1C212A' : '#FFFDF8'}"/></g><text x="${cx - w / 2 + 9}" y="${cy + 4.5}" font-family="Fredoka" font-weight="600" font-size="12.5" fill="${pal.ui.text}">${nom}</text>${dots}`;
  }

  // ---------- Plateau ----------
  // o : { v, pal, W, w, h, view:[c,r], state(c,r), fog, objects: {key: fn(cx, cy, W)}, overlay(c,r,cx,cy) }
  function plateau(o) {
    const W = o.W, H = W / 2, R = rng(11);
    const ox = o.w / 2 - (o.view[0] - o.view[1]) * W / 2, oy = (o.cy ?? o.h / 2) - (o.view[0] + o.view[1]) * H / 2;
    const cells = [];
    for (let r = 0; r < N; r++) for (let c = 0; c < N; c++) cells.push([c, r]);
    cells.sort((a, b) => (a[0] + a[1]) - (b[0] + b[1]) || a[0] - b[0]);
    let g = `<rect width="${o.w}" height="${o.h}" fill="${o.pal.table}"/>`;
    let late = '';
    for (const [c, r] of cells) {
      const cx = ox + (c - r) * W / 2, cy = oy + (c + r) * H / 2;
      if (cx < -W || cx > o.w + W || cy < -H * 2 || cy > o.h + H * 3) { R(); R(); continue; }
      const t = T(c, r), st = o.state ? o.state(c, r) : 'vue';
      const ex = o.extra ? o.extra(c, r) : null;
      g += tile(o.v, t, cx, cy, W, H, o.pal, R, st, o.fog, ex);
      const z = st === 'jamais' ? 0 : SURF[o.v][t] + (ex && ex.lift || 0);
      if (o.under) g += o.under(c, r, cx, cy - z, W, st) || '';
      if (o.objects) g += o.objects(c, r, cx, cy - z, W, st) || '';
      if (o.late) late += o.late(c, r, cx, cy - z, W, st) || '';
    }
    const hatch = `<pattern id="hach-${o.pal.id}" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(30)"><path d="M0 0V6" stroke="${o.pal.night ? '#000000' : '#5A4A38'}" stroke-opacity=".35" stroke-width="1.4"/></pattern>`;
    const defs = `<defs>${hatch}<filter id="ombre" x="-30%" y="-30%" width="160%" height="170%"><feDropShadow dx="0" dy="1.6" stdDeviation="1.2" flood-color="${o.pal.shadow}" flood-opacity="${o.pal.night ? .6 : .35}"/></filter></defs>`;
    return `<svg xmlns="http://www.w3.org/2000/svg" width="${o.w}" height="${o.h}" viewBox="0 0 ${o.w} ${o.h}">${defs}${g}${late}</svg>`;
  }

  // Palettes V1 : Jour franc et Nuit, avec la colline ajoutée et la montagne refroidie (guide-v1.md, § 2).
  const PALV1 = {
    jour: { ...PAL.jour, terr: { ...PAL.jour.terr, M: '#8E8C99', C: '#C98A3E' } },
    nuit: { ...PAL.nuit, terr: { ...PAL.nuit.terr, M: '#8C8A9A', C: '#AB8C2C' } },
  };
  window.Iso = { MAP, N, T, CLANS, PAL, PALV1, clanCol, inkOn, pts, diamond, prism, tile, figure, jeton, ville, etiquette, plateau, emblem, socle, SURF, ruine, exploitation, decorOnly: decor };
})();
