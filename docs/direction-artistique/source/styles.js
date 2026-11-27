// Les cinq univers visuels (DA-001 à DA-005) et le style « plat » qui sert aux palettes et aux interfaces.
(function () {
  const { mix, placeGlyph, glyph } = Scene;
  const S = {};

  // ---------- DA-001 Carnet d'explorateur : encre et lavis sur papier ----------
  {
    const ink = '#3A2D20', wash = { P: '#C9D48E', F: '#93B57A', M: '#C4AE8E', E: '#8FBFCC', S: '#E7CF8E', X: '#EADFC6' };
    S.encre = {
      id: 'encre', num: 'DA-001', nom: 'Carnet d’explorateur', sous: 'Encre et lavis sur papier',
      cell: 54, lab: 18, clans: ['#2B5A9A', '#A8321E'],
      defs: `<defs>
        <filter id="e-wash" x="-5%" y="-5%" width="110%" height="110%"><feTurbulence type="fractalNoise" baseFrequency="0.06" numOctaves="3" seed="3"/><feDisplacementMap in="SourceGraphic" scale="7"/></filter>
        <filter id="e-rough"><feTurbulence type="fractalNoise" baseFrequency="0.9" numOctaves="1" seed="5"/><feDisplacementMap in="SourceGraphic" scale="1.6"/></filter>
        <pattern id="e-hatch" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(35)"><path d="M0 0V6" stroke="${ink}" stroke-opacity=".28" stroke-width="1"/></pattern>
      </defs>`,
      tile(t, x, y, s, R) {
        let g = `<rect x="${x + 1}" y="${y + 1}" width="${s - 2}" height="${s - 2}" fill="${wash[t]}" opacity="${t === 'X' ? 1 : 0.85}" filter="url(#e-wash)"/>`;
        const L = `stroke="${ink}" stroke-width="1.3" fill="none" stroke-linecap="round" stroke-linejoin="round"`;
        let d = '';
        if (t === 'P') for (let i = 0; i < 3; i++) { const a = x + 10 + R() * (s - 22), b = y + 14 + R() * (s - 24); d += `<path d="M${a} ${b}l2 -5M${a + 4} ${b}l1 -6M${a + 8} ${b}l-1 -4" ${L}/>`; }
        if (t === 'F') for (const [a, b] of [[.3, .38], [.66, .34], [.48, .68]]) { const cx = x + a * s, cy = y + b * s; d += `<circle cx="${cx}" cy="${cy}" r="7" fill="${mix(wash.F, '#2f4f2a', .25)}" ${L.replace('fill="none"', '')}/><path d="M${cx} ${cy + 7}v6" ${L}/>`; }
        if (t === 'M') { d += `<path d="M${x + 8} ${y + s - 12}L${x + s * .38} ${y + 14}L${x + s * .62} ${y + s - 12}M${x + s * .5} ${y + s * .52}L${x + s * .68} ${y + 22}L${x + s - 8} ${y + s - 12}" ${L} stroke-width="1.6"/>`; for (let i = 0; i < 5; i++) d += `<path d="M${x + s * .38 + 2 + i * 2.6} ${y + 18 + i * 4.5}l${-2 - i * .6} ${3}" ${L} stroke-width="1"/>`; }
        if (t === 'E') for (let i = 0; i < 2; i++) { const a = x + 9 + R() * 8, b = y + 18 + i * 16; d += `<path d="M${a} ${b}q4 -4 8 0t8 0t8 0" ${L} stroke-width="1.1" stroke="#2E5A6A"/>`; }
        if (t === 'S') for (let i = 0; i < 14; i++) d += `<circle cx="${x + 6 + R() * (s - 12)}" cy="${y + 6 + R() * (s - 12)}" r=".9" fill="${ink}" opacity=".6"/>`;
        if (t === 'X') g += `<rect x="${x}" y="${y}" width="${s}" height="${s}" fill="url(#e-hatch)"/>`;
        return g + `<g filter="url(#e-rough)">${d}</g>`;
      },
      grid(W, H, s, lab) {
        let d = '';
        for (let c = 0; c <= 6; c++) d += `M${lab + c * s} ${lab}V${H}`;
        for (let r = 0; r <= 8; r++) d += `M${lab} ${lab + r * s}H${W}`;
        return `<path d="${d}" stroke="${ink}" stroke-opacity=".45" stroke-width="1" stroke-dasharray="3 3" filter="url(#e-rough)"/><rect x="${lab}" y="${lab}" width="${W - lab}" height="${H - lab}" fill="none" stroke="${ink}" stroke-width="1.8" filter="url(#e-rough)"/>`;
      },
      label: (t, x, y) => `<text x="${x}" y="${y + 5}" text-anchor="middle" font-family="Fell" font-style="italic" font-size="15" fill="${ink}">${t}</text>`,
      city(x, y, s, col) {
        const L = `stroke="${ink}" stroke-width="1.6" stroke-linejoin="round"`;
        return `<g filter="url(#e-rough)"><path d="M${x + 9} ${y + s - 9}V${y + 24}h5v-4h4v4h5v-4h4v4h5v-4h4v4h5V${y + s - 9}Z" fill="#E8D9B5" ${L}/><path d="M${x + s / 2 - 5} ${y + s - 9}v-9a5 5 0 0 1 10 0v9" fill="${ink}" opacity=".8"/><path d="M${x + s / 2} ${y + 20}V${y + 6}" ${L}/><path d="M${x + s / 2} ${y + 6}h11l-3 4l3 4h-11" fill="${col}" ${L} stroke-width="1.2"/></g>`;
      },
      move: (x, y, s) => `<circle cx="${x + s / 2}" cy="${y + s / 2}" r="3.5" fill="${ink}" opacity=".55"/>`,
      attack: (x, y, s) => `<g stroke="#A8321E" stroke-width="2.6" stroke-linecap="round" filter="url(#e-rough)"><path d="M${x + 7} ${y + 7}l9 9M${x + s - 7} ${y + 7}l-9 9"/></g>`,
      select: (x, y, s) => `<g fill="none" stroke="${ink}" stroke-width="1.6" filter="url(#e-rough)"><ellipse cx="${x + s / 2}" cy="${y + s / 2}" rx="${s / 2 - 2}" ry="${s / 2 - 3}"/><ellipse cx="${x + s / 2 + 1}" cy="${y + s / 2 - 1}" rx="${s / 2 - 4}" ry="${s / 2 - 2}" stroke-opacity=".6"/></g>`,
      unit(u, x, y, s, col) {
        const cx = x + s / 2, cy = y + s / 2, r = 15;
        const shape = u.clan === 0 ? `<circle cx="${cx}" cy="${cy}" r="${r}"/>` : `<path d="M${cx} ${cy - r - 2}L${cx + r + 2} ${cy}L${cx} ${cy + r + 2}L${cx - r - 2} ${cy}Z"/>`;
        return `<g filter="url(#e-rough)"><g fill="${col}" stroke="${ink}" stroke-width="1.6">${shape}</g><g fill="none" stroke="#F4EBD6" stroke-width="1" transform="translate(${cx} ${cy}) scale(.82) translate(${-cx} ${-cy})">${shape}</g>${placeGlyph(u.type, '#F7EFDC', cx, cy, 20, 2.4)}</g>`;
      },
      miniToken: (t, col) => `<svg width="44" height="44" viewBox="0 0 44 44"><circle cx="22" cy="22" r="19" fill="${col}" stroke="${ink}" stroke-width="1.6"/>${placeGlyph(t, '#F7EFDC', 22, 22, 26, 2.4)}</svg>`,
      emblem: col => `<svg width="34" height="34" viewBox="0 0 34 34"><path d="M5 4H29V20Q29 28 17 31Q5 28 5 20Z" fill="${col}" stroke="${ink}" stroke-width="1.6"/><path d="M10 12H24M17 8V26" stroke="#F4EBD6" stroke-width="2"/></svg>`,
    };
  }

  // ---------- DA-002 Pixel 16 : art en gros pixels ----------
  {
    const P = { k: '#14161F', s: '#F2C69B', w: '#D9DEE8', b: '#8A5A2B', g: '#4C9A3F', G: '#2E6B2C', m: '#8C8A98', M: '#5E5C6B', n: '#F4F6FF', y: '#E8C36A', Y: '#C99D47', r: '#B4483A', R: '#7E2D25', v: '#3F78D0', V: '#2C5AA8', l: '#77ABF0' };
    const BODY = [
      '................', '................', '......kkkk......', '.....kssssk.....', '.....kskskk.....', '.....kssssk.....', '......kkkk......', '.....kcccck.....',
      '....kccddcck....', '....kcccccck....', '....kcddddck....', '....kcccccck....', '.....kcckcck....', '.....kbk.kbk....', '.....kkk.kkk....', '................'];
    const CITY = [
      '.......kk.......', '.......kcck.....', '.......kccck....', '.......k........', '...kkkkkkkkkk...', '..krrrrrrrrrrk..', '.krRrrrrrrrRrrk.', 'kkkkkkkkkkkkkkkk',
      '.kmmmmmmmmmmmmk.', '.kmmkkmmmmkkmmk.', '.kmmkkmmmmkkmmk.', '.kmmmmmkkmmmmmk.', '.kmmmmmkkmmmmmk.', '.kMMMMMkkMMMMMk.', 'kkkkkkkkkkkkkkkk', '................'];
    const TREE = ['...kk...', '..kggk..', '.kgggGk.', '.kggGGk.', 'kgggGGGk', '.kkkkkk.', '...bb...', '...kk...'];
    const px = (bm, x, y, k, pal) => { let g = ''; bm.forEach((row, j) => { for (let i = 0; i < row.length; i++) { const ch = row[i]; if (ch !== '.' && ch !== ' ') g += `<rect x="${x + i * k}" y="${y + j * k}" width="${k}" height="${k}" fill="${pal[ch]}"/>`; } }); return g; };
    const dot = (x, y, k, col) => `<rect x="${x}" y="${y}" width="${k}" height="${k}" fill="${col}"/>`;
    S.pixel = {
      id: 'pixel', num: 'DA-002', nom: 'Pixel 16', sous: 'Gros pixels, 16 × 16 par case',
      cell: 48, lab: 24, clans: ['#3F78D0', '#D9542B'],
      under: (W, H) => `<rect width="${W}" height="${H}" fill="#14161F"/>`,
      tile(t, x, y, s, R) {
        const k = 3; const base = { P: '#5DAA48', F: '#3B8236', M: '#77758A', E: '#2F64C0', S: '#E3BC66', X: '#1E2130' }[t];
        let g = `<rect x="${x}" y="${y}" width="${s}" height="${s}" fill="${base}"/>`;
        if (t === 'P') for (let i = 0; i < 9; i++) g += dot(x + Math.floor(R() * 16) * k, y + Math.floor(R() * 16) * k, k, R() < .6 ? '#7CC85E' : '#468A37');
        if (t === 'F') { g += px(TREE, x + 2 * k, y + 1 * k, k, P) + px(TREE, x + 8 * k, y + 4 * k, k, P) + px(TREE, x + 3 * k, y + 8 * k, k, P); }
        if (t === 'M') { const MT = ['.......kk.......', '......knnk......', '.....knnmmk.....', '....knmmmMMk....', '...kmmmmmMMMk...', '..kmmmmmmMMMMk..', '.kmmmmmmmMMMMMk.', 'kkkkkkkkkkkkkkkk']; g += px(MT, x, y + 6 * k, k, P); }
        if (t === 'E') for (let j = 0; j < 3; j++) { const ox = Math.floor(R() * 8); g += dot(x + (ox + 1) * k, y + (3 + j * 5) * k, k, P.l) + dot(x + (ox + 2) * k, y + (3 + j * 5) * k, k, P.l) + dot(x + (ox + 3) * k, y + (4 + j * 5) * k, k, P.l) + dot(x + (ox + 4) * k, y + (3 + j * 5) * k, k, P.l); }
        if (t === 'S') for (let i = 0; i < 8; i++) g += dot(x + Math.floor(R() * 16) * k, y + Math.floor(R() * 16) * k, k, '#C59A49');
        if (t === 'X') for (let j = 0; j < 16; j += 2) for (let i = (j / 2) % 2 * 2; i < 16; i += 4) g += dot(x + i * k, y + j * k, k, '#272B3D');
        return g;
      },
      grid(W, H, s, lab) { let d = ''; for (let c = 0; c <= 6; c++) d += `M${lab + c * s} ${lab}V${H}`; for (let r = 0; r <= 8; r++) d += `M${lab} ${lab + r * s}H${W}`; return `<path d="${d}" stroke="#14161F" stroke-opacity=".55" stroke-width="1"/>`; },
      label: (t, x, y) => `<text x="${x}" y="${y + 6}" text-anchor="middle" font-family="Pixelify" font-size="16" fill="#E8ECF8">${t}</text>`,
      city: (x, y, s, col) => px(CITY, x, y, 3, { ...P, c: col }),
      move: (x, y, s) => `<rect x="${x + s / 2 - 4.5}" y="${y + s / 2 - 4.5}" width="9" height="9" fill="#FFFFFF" opacity=".85"/><rect x="${x + s / 2 - 1.5}" y="${y + s / 2 - 1.5}" width="3" height="3" fill="#14161F"/>`,
      attack: (x, y, s) => { const k = 3, c = '#FF3B2F'; let g = ''; for (const [i, j] of [[0, 0], [1, 0], [0, 1], [15, 0], [14, 0], [15, 1], [0, 15], [1, 15], [0, 14], [15, 15], [14, 15], [15, 14]]) g += dot(x + i * k, y + j * k, k, c); return g; },
      select: (x, y, s) => { const k = 3; let g = ''; for (let i = 0; i < 16; i++) for (const j of [0, 15]) if (i < 4 || i > 11) { g += dot(x + i * k, y + j * k, k, '#FFE14A') + dot(x + j * k, y + i * k, k, '#FFE14A'); } return g; },
      unit(u, x, y, s, col) {
        const pal = { ...P, c: col, d: mix(col, '#000000', .35) };
        let g = px(BODY, x, y, 3, pal), k = 3;
        if (u.type === 'lance') { for (let j = 1; j <= 13; j++) g += dot(x + 12 * k, y + j * k, k, P.b); g += dot(x + 12 * k, y, k, P.w) + dot(x + 11 * k, y + k, k, P.w) + dot(x + 13 * k, y + k, k, P.w) + dot(x + 12 * k, y + k, k, P.w); }
        if (u.type === 'arc') { for (const [i, j] of [[12, 4], [13, 5], [13, 6], [13, 7], [13, 8], [13, 9], [12, 10], [11, 11]]) g += dot(x + i * k, y + j * k, k, P.b); for (let j = 5; j <= 10; j++) g += dot(x + 11 * k, y + j * k, k, P.w); }
        if (u.type === 'hache') { for (let j = 4; j <= 13; j++) g += dot(x + 12 * k, y + j * k, k, P.b); for (const [i, j] of [[13, 3], [14, 3], [13, 4], [14, 4], [15, 4], [13, 5], [14, 5], [15, 5], [13, 6], [14, 6]]) g += dot(x + i * k, y + j * k, k, P.w); }
        // Marque de clan sous l'unité : carré plein (nous) ou creux (ennemi), pour ne pas dépendre de la couleur seule.
        g += u.clan === 0 ? dot(x + 1 * k, y + 13 * k, 2 * k, col) : `<rect x="${x + 1 * k + 1}" y="${y + 13 * k + 1}" width="${2 * k - 2}" height="${2 * k - 2}" fill="none" stroke="${col}" stroke-width="2"/>`;
        return g;
      },
      miniToken: (t, col) => `<svg width="48" height="48" viewBox="0 0 48 48" shape-rendering="crispEdges">${px(BODY, 0, 0, 3, { ...P, c: col, d: mix(col, '#000000', .35) })}${(() => { let g = ''; for (let j = 1; j <= 13; j++) g += dot(36, j * 3, 3, P.b); return g + dot(36, 0, 3, P.w) + dot(33, 3, 3, P.w) + dot(39, 3, 3, P.w); })()}</svg>`,
      emblem: col => `<svg width="30" height="30" viewBox="0 0 10 10" shape-rendering="crispEdges">${px(['kkkkkkkkk.', 'kcccccccck', 'kcnncnncck', 'kcccccccck', 'kcnnnnnnck', 'kcccccccck', '.kccccccK.', '..kccccK..', '...kkkk...'], 0, 0, 1, { ...P, c: col, K: '#14161F' })}</svg>`,
    };
  }

  // ---------- DA-003 Papier découpé : formes découpées, ombres douces ----------
  {
    const jit = (x, y, w, h, R, a = 1.4) => {
      const pts = [];
      const n = 4;
      for (let i = 0; i < n; i++) pts.push([x + w * i / n, y]);
      for (let i = 0; i < n; i++) pts.push([x + w, y + h * i / n]);
      for (let i = 0; i < n; i++) pts.push([x + w - w * i / n, y + h]);
      for (let i = 0; i < n; i++) pts.push([x, y + h - h * i / n]);
      return 'M' + pts.map(([a1, b]) => `${(a1 + (R() - .5) * a * 2).toFixed(1)} ${(b + (R() - .5) * a * 2).toFixed(1)}`).join('L') + 'Z';
    };
    const col = { P: '#8FD16A', F: '#2E9E5B', M: '#B9A9A0', E: '#3BA7E0', S: '#F6D37A', X: '#6C6478' };
    S.papier = {
      id: 'papier', num: 'DA-003', nom: 'Papier découpé', sous: 'Formes découpées, ombres douces, couleurs vives',
      cell: 54, lab: 18, clans: ['#4B3FD6', '#F0642A'],
      defs: `<defs><filter id="p-sh" x="-20%" y="-20%" width="140%" height="150%"><feDropShadow dx="0" dy="1.6" stdDeviation="1.2" flood-color="#3b2a1a" flood-opacity=".35"/></filter>
        <filter id="p-sh2" x="-30%" y="-30%" width="160%" height="170%"><feDropShadow dx="0" dy="2.5" stdDeviation="1.8" flood-color="#2a1a0a" flood-opacity=".45"/></filter></defs>`,
      tile(t, x, y, s, R) {
        let g = `<path d="${jit(x + 2, y + 2, s - 4, s - 4, R)}" fill="${col[t]}" filter="url(#p-sh)"/>`;
        if (t === 'P') for (let i = 0; i < 2; i++) { const a = x + 10 + R() * (s - 26), b = y + 12 + R() * (s - 26); g += `<path d="M${a} ${b + 6}q4 -9 8 0q4 -7 7 0z" fill="#B3E58E" filter="url(#p-sh)"/>`; }
        if (t === 'F') for (const [a, b, r] of [[.32, .4, 9], [.68, .36, 8], [.5, .68, 9.5]]) g += `<circle cx="${x + a * s}" cy="${y + b * s}" r="${r}" fill="#1F7A44" filter="url(#p-sh)"/><circle cx="${x + a * s - 2.5}" cy="${y + b * s - 2.5}" r="${r * .45}" fill="#3FBF72"/>`;
        if (t === 'M') g += `<path d="M${x + 7} ${y + s - 9}L${x + s * .45} ${y + 11}L${x + s - 7} ${y + s - 9}Z" fill="#8E7F78" filter="url(#p-sh)"/><path d="M${x + s * .45} ${y + 11}L${x + s * .58} ${y + 21}L${x + s * .5} ${y + 19}L${x + s * .42} ${y + 23}L${x + s * .35} ${y + 19}Z" fill="#FFFFFF"/>`;
        if (t === 'E') for (let i = 0; i < 2; i++) { const a = x + 9 + R() * 10, b = y + 17 + i * 15; g += `<path d="M${a} ${b}q5 -5 10 0q5 -5 10 0v3q-5 -5 -10 0q-5 -5 -10 0z" fill="#9BDDF7"/>`; }
        if (t === 'S') g += `<path d="M${x + 6} ${y + s - 12}q${s * .3} -14 ${s - 12} -6v6h${-(s - 12)}z" fill="#E8B955"/>`;
        if (t === 'X') g += `<path d="${jit(x + 6, y + 6, s - 12, s - 12, R, 2)}" fill="#5A5266"/>`;
        return g;
      },
      label: (t, x, y) => `<text x="${x}" y="${y + 5}" text-anchor="middle" font-family="Fredoka" font-weight="600" font-size="14" fill="#6B4E31">${t}</text>`,
      city: (x, y, s, c) => `<g filter="url(#p-sh2)"><path d="M${x + 8} ${y + s - 8}V${y + 26}L${x + 18} ${y + 17}L${x + 28} ${y + 26}V${y + s - 8}Z" fill="#FFF6E5"/><path d="M${x + 24} ${y + s - 8}V${y + 20}L${x + 35} ${y + 10}L${x + 46} ${y + 20}V${y + s - 8}Z" fill="#FFFFFF"/><path d="M${x + 6} ${y + 27}L${x + 18} ${y + 15}L${x + 30} ${y + 27}Z M${x + 22} ${y + 21}L${x + 35} ${y + 8}L${x + 48} ${y + 21}Z" fill="${c}"/><rect x="${x + 31}" y="${y + s - 19}" width="8" height="11" rx="4" fill="#6B4E31"/></g>`,
      move: (x, y, s) => `<circle cx="${x + s / 2}" cy="${y + s / 2}" r="6" fill="#FFFFFF" opacity=".9" filter="url(#p-sh)"/>`,
      attack: (x, y, s) => `<g filter="url(#p-sh2)"><circle cx="${x + s - 10}" cy="${y + 10}" r="9" fill="#FFFFFF"/><path d="M${x + s - 14} ${y + 6}l8 8M${x + s - 6} ${y + 6}l-8 8" stroke="#D33A1A" stroke-width="2.6" stroke-linecap="round"/></g>`,
      select: (x, y, s) => `<rect x="${x + 1}" y="${y + 1}" width="${s - 2}" height="${s - 2}" rx="8" fill="none" stroke="#FFFFFF" stroke-width="3.5" stroke-dasharray="7 4"/>`,
      unit(u, x, y, s, c) {
        const cx = x + s / 2, cy = y + s / 2;
        const sh = u.clan === 0 ? `<circle cx="${cx}" cy="${cy}" r="17" fill="#FFFFFF"/><circle cx="${cx}" cy="${cy}" r="14" fill="${c}"/>` : `<rect x="${cx - 16}" y="${cy - 16}" width="32" height="32" rx="7" fill="#FFFFFF"/><rect x="${cx - 13}" y="${cy - 13}" width="26" height="26" rx="5" fill="${c}"/>`;
        return `<g filter="url(#p-sh2)">${sh}</g>${placeGlyph(u.type, '#FFFFFF', cx, cy, 20, 2.6)}`;
      },
      miniToken: (t, c) => `<svg width="46" height="46" viewBox="0 0 46 46"><circle cx="23" cy="23" r="21" fill="#FFFFFF"/><circle cx="23" cy="23" r="17.5" fill="${c}"/>${placeGlyph(t, '#FFFFFF', 23, 23, 24, 2.6)}</svg>`,
      emblem: c => `<svg width="34" height="34" viewBox="0 0 34 34"><circle cx="17" cy="17" r="15" fill="#FFFFFF"/><circle cx="17" cy="17" r="12" fill="${c}"/><path d="M11 21L17 10L23 21Z" fill="#FFFFFF"/></svg>`,
    };
  }

  // ---------- DA-004 Plateau de bois : tuiles et pions de jeu de société ----------
  {
    const col = { P: '#B5C46E', F: '#5E8A4A', M: '#A39684', E: '#5E93B0', S: '#DCC184', X: '#4A3A2C' };
    const icon = (t, x, y, s) => {
      const cx = x + s / 2, cy = y + s / 2;
      if (t === 'F') return `M${cx} ${cy - 15}L${cx + 11} ${cy + 3}H${cx + 4}L${cx + 9} ${cy + 10}H${cx - 9}L${cx - 4} ${cy + 3}H${cx - 11}Z M${cx} ${cy + 10}V${cy + 15}`;
      if (t === 'M') return `M${cx - 17} ${cy + 11}L${cx - 5} ${cy - 11}L${cx + 3} ${cy + 1}L${cx + 8} ${cy - 6}L${cx + 17} ${cy + 11}Z`;
      if (t === 'E') return `M${cx - 15} ${cy - 4}q5 -5 10 0t10 0t10 0M${cx - 15} ${cy + 6}q5 -5 10 0t10 0t10 0`;
      if (t === 'S') return `M${cx - 16} ${cy + 8}q10 -14 20 -4q6 -6 12 4`;
      if (t === 'P') return `M${cx - 10} ${cy + 8}l3 -10M${cx} ${cy + 8}V${cy - 6}M${cx + 10} ${cy + 8}l-3 -10`;
      return '';
    };
    S.bois = {
      id: 'bois', num: 'DA-004', nom: 'Plateau de bois', sous: 'Tuiles peintes et pions de jeu de société',
      cell: 54, lab: 18, clans: ['#2F6FB5', '#C8402A'],
      defs: `<defs>
        <linearGradient id="b-bev" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#fff" stop-opacity=".38"/><stop offset=".5" stop-color="#fff" stop-opacity="0"/><stop offset="1" stop-color="#000" stop-opacity=".28"/></linearGradient>
        <radialGradient id="b-disc" cx=".4" cy=".35" r=".7"><stop offset="0" stop-color="#F3DDB2"/><stop offset="1" stop-color="#C79A5E"/></radialGradient>
        <filter id="b-sh" x="-30%" y="-30%" width="160%" height="170%"><feDropShadow dx="1.5" dy="2.5" stdDeviation="1.5" flood-color="#1a0d03" flood-opacity=".55"/></filter>
        <pattern id="b-grain" width="54" height="9" patternUnits="userSpaceOnUse"><path d="M0 4Q14 1 27 4T54 4" stroke="#000" stroke-opacity=".07" fill="none"/></pattern>
      </defs>`,
      under: (W, H) => `<rect width="${W}" height="${H}" rx="8" fill="#5A3B22"/><rect width="${W}" height="${H}" rx="8" fill="url(#b-grain)"/>`,
      tile(t, x, y, s) {
        const d = icon(t, x, y, s);
        let g = `<rect x="${x + 2}" y="${y + 2}" width="${s - 4}" height="${s - 4}" rx="5" fill="${col[t]}"/><rect x="${x + 2}" y="${y + 2}" width="${s - 4}" height="${s - 4}" rx="5" fill="url(#b-grain)"/><rect x="${x + 2}" y="${y + 2}" width="${s - 4}" height="${s - 4}" rx="5" fill="url(#b-bev)"/>`;
        if (d) g += `<path d="${d}" transform="translate(.8 .8)" fill="none" stroke="#fff" stroke-opacity=".35" stroke-width="2" stroke-linejoin="round" stroke-linecap="round"/><path d="${d}" fill="${t === 'F' || t === 'M' ? mix(col[t], '#000000', .28) : 'none'}" stroke="${mix(col[t], '#000000', .45)}" stroke-width="2" stroke-linejoin="round" stroke-linecap="round"/>`;
        if (t === 'X') g += `<text x="${x + s / 2}" y="${y + s / 2 + 7}" text-anchor="middle" font-family="Bitter" font-weight="700" font-size="20" fill="#6E5640">?</text>`;
        return g;
      },
      label: (t, x, y) => `<text x="${x}" y="${y + 5}" text-anchor="middle" font-family="Bitter" font-weight="700" font-size="13" fill="#E9CFA4">${t}</text>`,
      city: (x, y, s, c) => `<g filter="url(#b-sh)"><path d="M${x + 9} ${y + s - 10}V${y + 25}L${x + 19} ${y + 15}L${x + 29} ${y + 25}V${y + s - 10}Z" fill="${c}"/><path d="M${x + 26} ${y + s - 10}V${y + 21}L${x + 36} ${y + 11}L${x + 46} ${y + 21}V${y + s - 10}Z" fill="${mix(c, '#000000', .15)}"/></g><path d="M${x + 9} ${y + 25}L${x + 19} ${y + 15}L${x + 29} ${y + 25}M${x + 26} ${y + 21}L${x + 36} ${y + 11}L${x + 46} ${y + 21}" stroke="#fff" stroke-opacity=".45" stroke-width="1.5" fill="none"/>`,
      move: (x, y, s) => `<circle cx="${x + s / 2}" cy="${y + s / 2}" r="6" fill="#F6E7C6" stroke="#5A3B22" stroke-width="1.5" filter="url(#b-sh)"/>`,
      attack: (x, y, s) => `<g filter="url(#b-sh)"><circle cx="${x + s - 10}" cy="${y + 10}" r="9" fill="#C8402A" stroke="#F6E7C6" stroke-width="1.5"/></g><path d="M${x + s - 13.5} ${y + 6.5}l7 7M${x + s - 6.5} ${y + 6.5}l-7 7" stroke="#F6E7C6" stroke-width="2.2" stroke-linecap="round"/>`,
      select: (x, y, s) => `<rect x="${x + .5}" y="${y + .5}" width="${s - 1}" height="${s - 1}" rx="6" fill="none" stroke="#FFD34D" stroke-width="3"/>`,
      unit(u, x, y, s, c) {
        const cx = x + s / 2, cy = y + s / 2;
        const outer = u.clan === 0 ? `<circle cx="${cx}" cy="${cy}" r="18"/>` : `<path d="M${cx} ${cy - 19}L${cx + 17} ${cy - 9}V${cy + 9}L${cx} ${cy + 19}L${cx - 17} ${cy + 9}V${cy - 9}Z"/>`;
        return `<g filter="url(#b-sh)"><g fill="${c}">${outer}</g></g><g transform="translate(${cx} ${cy}) scale(.74) translate(${-cx} ${-cy})" fill="url(#b-disc)">${outer}</g>${placeGlyph(u.type, '#5A3B22', cx, cy, 19, 2.6)}`;
      },
      miniToken: (t, c) => `<svg width="46" height="46" viewBox="0 0 46 46"><circle cx="23" cy="23" r="20" fill="${c}"/><circle cx="23" cy="23" r="14.5" fill="#E9CC98"/>${placeGlyph(t, '#5A3B22', 23, 23, 22, 2.6)}</svg>`,
      emblem: c => `<svg width="34" height="34" viewBox="0 0 34 34"><circle cx="17" cy="17" r="15" fill="${c}"/><circle cx="17" cy="17" r="10" fill="#E9CC98"/><path d="M12 21L17 11L22 21Z" fill="#5A3B22"/></svg>`,
    };
  }

  // ---------- DA-005 Table tactique : lignes lumineuses sur fond sombre ----------
  {
    const line = { P: '#6FE08A', F: '#3FC46A', M: '#C9B6FF', E: '#4FC3FF', S: '#FFD166', X: '#2A3550' };
    const fill = { P: '#0F2A1E', F: '#0C2318', M: '#1E1A33', E: '#0A2036', S: '#2A2210', X: '#070A12' };
    S.tactique = {
      id: 'tactique', num: 'DA-005', nom: 'Table tactique', sous: 'Lignes lumineuses sur fond sombre',
      cell: 54, lab: 18, clans: ['#4FC3FF', '#FF8A3D'],
      defs: `<defs><filter id="t-glow" x="-30%" y="-30%" width="160%" height="160%"><feGaussianBlur stdDeviation="1.8" result="b"/><feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge></filter>
        <pattern id="t-dots" width="9" height="9" patternUnits="userSpaceOnUse"><circle cx="4.5" cy="4.5" r="1" fill="${line.P}" fill-opacity=".55"/></pattern>
        <pattern id="t-sand" width="5" height="5" patternUnits="userSpaceOnUse"><circle cx="2.5" cy="2.5" r=".8" fill="${line.S}" fill-opacity=".6"/></pattern>
        <pattern id="t-water" width="10" height="7" patternUnits="userSpaceOnUse"><path d="M1 3.5H7" stroke="${line.E}" stroke-opacity=".7" stroke-width="1.2"/></pattern>
        <pattern id="t-fog" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><path d="M0 0V6" stroke="#2A3550" stroke-width="1.2"/></pattern></defs>`,
      under: (W, H) => `<rect width="${W}" height="${H}" fill="#060912"/>`,
      tile(t, x, y, s) {
        let g = `<rect x="${x + 1.5}" y="${y + 1.5}" width="${s - 3}" height="${s - 3}" rx="3" fill="${fill[t]}"/>`;
        const pat = { P: 't-dots', S: 't-sand', E: 't-water', X: 't-fog' }[t];
        if (pat) g += `<rect x="${x + 1.5}" y="${y + 1.5}" width="${s - 3}" height="${s - 3}" rx="3" fill="url(#${pat})"/>`;
        if (t === 'F') { let d = ''; for (const [a, b] of [[.3, .35], [.68, .32], [.5, .66], [.22, .72], [.78, .7]]) d += `M${x + a * s - 5} ${y + b * s + 4}L${x + a * s} ${y + b * s - 5}L${x + a * s + 5} ${y + b * s + 4}`; g += `<path d="${d}" stroke="${line.F}" stroke-width="1.6" fill="none" filter="url(#t-glow)"/>`; }
        if (t === 'M') { let d = ''; for (let i = 0; i < 3; i++) { const k = 5 + i * 4.5; d += `M${x + s / 2 - k * 1.4} ${y + s / 2 + k * .7}Q${x + s / 2} ${y + s / 2 - k * 1.3} ${x + s / 2 + k * 1.4} ${y + s / 2 + k * .7}`; } g += `<path d="${d}" stroke="${line.M}" stroke-width="1.3" fill="none" filter="url(#t-glow)"/>`; }
        if (t !== 'X') g += `<rect x="${x + 1.5}" y="${y + 1.5}" width="${s - 3}" height="${s - 3}" rx="3" fill="none" stroke="${line[t]}" stroke-opacity=".35"/>`;
        return g;
      },
      label: (t, x, y) => `<text x="${x}" y="${y + 5}" text-anchor="middle" font-family="Chakra" font-weight="700" font-size="13" fill="#7F92B8">${t}</text>`,
      city: (x, y, s, c) => `<g fill="none" stroke="${c}" stroke-width="2" filter="url(#t-glow)"><rect x="${x + 9}" y="${y + 9}" width="${s - 18}" height="${s - 18}" rx="2"/><rect x="${x + 17}" y="${y + 17}" width="${s - 34}" height="${s - 34}"/><path d="M${x + s / 2} ${y + 3}V${y + 9}M${x + s / 2} ${y + s - 9}V${y + s - 3}M${x + 3} ${y + s / 2}H${x + 9}M${x + s - 9} ${y + s / 2}H${x + s - 3}"/></g>`,
      move: (x, y, s) => `<rect x="${x + 4}" y="${y + 4}" width="${s - 8}" height="${s - 8}" rx="2" fill="#4FC3FF" fill-opacity=".12" stroke="#4FC3FF" stroke-opacity=".6" stroke-dasharray="4 3"/>`,
      attack: (x, y, s) => `<g stroke="#FF5A4D" stroke-width="2.2" fill="none" filter="url(#t-glow)"><path d="M${x + 4} ${y + 14}V${y + 4}H${x + 14}M${x + s - 14} ${y + 4}H${x + s - 4}V${y + 14}M${x + 4} ${y + s - 14}V${y + s - 4}H${x + 14}M${x + s - 14} ${y + s - 4}H${x + s - 4}V${y + s - 14}"/></g>`,
      select: (x, y, s) => `<rect x="${x + 1}" y="${y + 1}" width="${s - 2}" height="${s - 2}" rx="3" fill="none" stroke="#FFFFFF" stroke-width="2" filter="url(#t-glow)"/>`,
      unit(u, x, y, s, c) {
        const cx = x + s / 2, cy = y + s / 2;
        const sh = u.clan === 0 ? `<circle cx="${cx}" cy="${cy}" r="15"/>` : `<path d="M${cx} ${cy - 17}L${cx + 17} ${cy}L${cx} ${cy + 17}L${cx - 17} ${cy}Z"/>`;
        return `<g fill="#060912" fill-opacity=".85" stroke="${c}" stroke-width="2.2" filter="url(#t-glow)">${sh}</g><g filter="url(#t-glow)">${placeGlyph(u.type, c, cx, cy, 19, 2.2)}</g>`;
      },
      miniToken: (t, c) => `<svg width="44" height="44" viewBox="0 0 44 44"><circle cx="22" cy="22" r="18" fill="#060912" stroke="${c}" stroke-width="2.2"/>${placeGlyph(t, c, 22, 22, 24, 2.2)}</svg>`,
      emblem: c => `<svg width="32" height="32" viewBox="0 0 32 32"><path d="M16 3L28 10V22L16 29L4 22V10Z" fill="none" stroke="${c}" stroke-width="2.2"/><path d="M11 20L16 11L21 20Z" fill="${c}"/></svg>`,
    };
  }

  // ---------- Style plat, piloté par une palette : sert aux fiches de palettes et aux interfaces ----------
  S.plat = (pal, clanIdx = [0, 2]) => {
    const t2 = pal.terr, dark = Scene.mix;
    const symCol = t => dark(t2[t], '#000000', .38);
    return {
      id: 'plat', cell: 54, lab: 18, clans: clanIdx.map(i => pal.clans[i]), pal,
      defs: `<defs><pattern id="f-fog" width="7" height="7" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><path d="M0 0V7" stroke="${dark(t2.X, '#FFFFFF', .14)}" stroke-width="2"/></pattern></defs>`,
      tile(t, x, y, s, R) {
        let g = `<rect x="${x}" y="${y}" width="${s}" height="${s}" fill="${t2[t]}"/>`;
        const c = symCol(t), cx = x + s / 2, cy = y + s / 2;
        if (t === 'F') g += `<path d="M${cx - 12} ${cy + 8}l7 -16l7 16zM${cx + 1} ${cy + 10}l6 -14l6 14z" fill="${c}"/>`;
        if (t === 'M') g += `<path d="M${cx - 16} ${cy + 11}L${cx - 4} ${cy - 11}L${cx + 3} ${cy}L${cx + 8} ${cy - 6}L${cx + 17} ${cy + 11}Z" fill="${c}"/>`;
        if (t === 'E') g += `<path d="M${cx - 13} ${cy - 3}q3.25 -4 6.5 0t6.5 0t6.5 0t6.5 0M${cx - 13} ${cy + 7}q3.25 -4 6.5 0t6.5 0t6.5 0t6.5 0" stroke="${dark(t2.E, '#FFFFFF', .45)}" stroke-width="2" fill="none" stroke-linecap="round"/>`;
        if (t === 'S') g += `<g fill="${c}"><circle cx="${cx - 8}" cy="${cy - 5}" r="1.6"/><circle cx="${cx + 6}" cy="${cy - 8}" r="1.6"/><circle cx="${cx}" cy="${cy + 2}" r="1.6"/><circle cx="${cx - 10}" cy="${cy + 9}" r="1.6"/><circle cx="${cx + 10}" cy="${cy + 7}" r="1.6"/></g>`;
        if (t === 'P') g += `<path d="M${cx - 9} ${cy + 6}l2 -7M${cx} ${cy + 6}v-8M${cx + 9} ${cy + 6}l-2 -7" stroke="${c}" stroke-width="2" stroke-linecap="round"/>`;
        if (t === 'X') g += `<rect x="${x}" y="${y}" width="${s}" height="${s}" fill="url(#f-fog)"/>`;
        return g;
      },
      grid(W, H, s, lab) { let d = ''; for (let c = 0; c <= 6; c++) d += `M${lab + c * s} ${lab}V${H}`; for (let r = 0; r <= 8; r++) d += `M${lab} ${lab + r * s}H${W}`; return `<path d="${d}" stroke="#000" stroke-opacity=".18" stroke-width="1"/>`; },
      label: (t, x, y) => `<text x="${x}" y="${y + 5}" text-anchor="middle" font-family="inherit" font-weight="700" font-size="13" fill="${pal.ui.text2}">${t}</text>`,
      city: (x, y, s, c) => `<rect x="${x + 6}" y="${y + 6}" width="${s - 12}" height="${s - 12}" rx="6" fill="${pal.ui.surf}" stroke="${c}" stroke-width="3"/><path d="M${x + 14} ${y + s - 14}V${y + 25}L${x + 21} ${y + 18}L${x + 28} ${y + 25}V${y + s - 14}ZM${x + 27} ${y + s - 14}V${y + 22}L${x + 33} ${y + 15}L${x + 40} ${y + 22}V${y + s - 14}Z" fill="${c}" stroke="${pal.ui.text}" stroke-width="1.2"/>`,
      move: (x, y, s) => `<circle cx="${x + s / 2}" cy="${y + s / 2}" r="6" fill="${pal.ui.surf}" stroke="${pal.ui.text}" stroke-width="1.5"/>`,
      attack: (x, y, s) => `<circle cx="${x + s - 10}" cy="${y + 10}" r="8" fill="${pal.ui.alert}" stroke="${pal.ui.surf}" stroke-width="1.5"/><path d="M${x + s - 13.5} ${y + 6.5}l7 7M${x + s - 6.5} ${y + 6.5}l-7 7" stroke="#fff" stroke-width="2" stroke-linecap="round"/>`,
      select: (x, y, s) => `<rect x="${x + 1.5}" y="${y + 1.5}" width="${s - 3}" height="${s - 3}" rx="4" fill="none" stroke="${pal.ui.text}" stroke-width="3"/>`,
      unit(u, x, y, s, c) {
        const cx = x + s / 2, cy = y + s / 2, ink = Couleurs.contrast(c, '#000000') > Couleurs.contrast(c, '#FFFFFF') ? '#111111' : '#FFFFFF';
        const sh = u.clan === 0 ? `<circle cx="${cx}" cy="${cy}" r="16"/>` : `<path d="M${cx} ${cy - 18}L${cx + 18} ${cy}L${cx} ${cy + 18}L${cx - 18} ${cy}Z"/>`;
        return `<g fill="${c}" stroke="${pal.ui.text}" stroke-width="2">${sh}</g>${placeGlyph(u.type, ink, cx, cy, 20, 2.6)}`;
      },
      miniToken: (t, c) => { const ink = Couleurs.contrast(c, '#000000') > Couleurs.contrast(c, '#FFFFFF') ? '#111111' : '#FFFFFF'; return `<svg width="44" height="44" viewBox="0 0 44 44"><circle cx="22" cy="22" r="19" fill="${c}" stroke="${pal.ui.text}" stroke-width="2"/>${placeGlyph(t, ink, 22, 22, 24, 2.6)}</svg>`; },
      emblem: c => `<svg width="32" height="32" viewBox="0 0 32 32"><circle cx="16" cy="16" r="14" fill="${c}" stroke="${pal.ui.text}" stroke-width="2"/><path d="M10 21L16 10L22 21Z" fill="${pal.ui.text}"/></svg>`,
    };
  };

  window.STYLES = S;
})();
