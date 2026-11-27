// Scène type commune à tous les aperçus : un plateau 6 × 8 (A1 à F8), une ville, trois unités.
// Chaque style ne fait que dessiner les mêmes données à sa façon.
(function () {
  const COLS = 'ABCDEF';
  const MAP = ['EEPFFM', 'EPPPFM', 'SPPPFF', 'ESPPPM', 'EESPFF', 'EEEPPF', 'EEPXXX', 'XXXXXX'];
  const T = (c, r) => MAP[r][c];
  const CITY = { c: 2, r: 2, nom: 'Ostrelle' };
  const UNITS = [
    { c: 3, r: 3, type: 'lance', clan: 0, sel: true },
    { c: 1, r: 1, type: 'arc', clan: 0 },
    { c: 4, r: 4, type: 'hache', clan: 1 },
  ];
  const SEL = UNITS[0];
  const MOVES = [];
  for (let dr = -1; dr <= 1; dr++) for (let dc = -1; dc <= 1; dc++) {
    if (!dr && !dc) continue;
    const c = SEL.c + dc, r = SEL.r + dr;
    const enemy = UNITS.find(u => u.c === c && u.r === r && u.clan !== 0);
    MOVES.push({ c, r, attack: !!enemy });
  }
  const rng = seed => () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296; };

  // Mélange de deux couleurs hexadécimales.
  const mix = (a, b, t) => {
    const p = h => [1, 3, 5].map(i => parseInt(h.substr(i, 2), 16));
    const x = p(a), y = p(b);
    return '#' + x.map((v, i) => Math.round(v + (y[i] - v) * t).toString(16).padStart(2, '0')).join('');
  };

  // Symboles des unités, dans une boîte de 24 × 24.
  function glyph(t, col, sw = 2.2) {
    const st = `stroke="${col}" stroke-width="${sw}" stroke-linecap="round" stroke-linejoin="round" fill="none"`;
    if (t === 'lance') return `<path d="M5 19L16.5 7.5" ${st}/><path d="M14 5L20 4L19 10Z" fill="${col}" stroke="${col}" stroke-width="1.2" stroke-linejoin="round"/><path d="M7.5 13.5l3 3" ${st}/>`;
    if (t === 'arc') return `<path d="M8 4Q20 12 8 20" ${st}/><path d="M8 4L8 20" ${st} stroke-width="${sw * 0.5}"/><path d="M4 12H16" ${st}/><path d="M19.5 12l-4-2.4v4.8z" fill="${col}"/>`;
    return `<path d="M7 20L15.5 6" ${st}/><path d="M12.5 4.5Q21 3 20.2 11.8L15 8.8Z" fill="${col}" stroke="${col}" stroke-width="1.2" stroke-linejoin="round"/>`;
  }
  const placeGlyph = (t, col, cx, cy, size, sw) => `<g transform="translate(${cx - size / 2},${cy - size / 2}) scale(${size / 24})">${glyph(t, col, sw)}</g>`;

  // Icônes de l'interface (24 × 24, couleur du texte).
  const ICON = {
    or: '<svg viewBox="0 0 24 24" width="18" height="18"><circle cx="12" cy="12" r="8.5" fill="currentColor"/><circle cx="12" cy="12" r="5.2" fill="none" stroke="var(--icon-in,#0003)" stroke-width="1.6"/></svg>',
    savoir: '<svg viewBox="0 0 24 24" width="18" height="18"><path d="M3 6Q8 4 12 7Q16 4 21 6V19Q16 17 12 19.5Q8 17 3 19Z" fill="currentColor"/><path d="M12 7V19.5" stroke="var(--icon-in,#0003)" stroke-width="1.6"/></svg>',
    menu: '<svg viewBox="0 0 24 24" width="22" height="22"><path d="M4 6H20M4 12H20M4 18H20" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"/></svg>',
    bouclier: '<svg viewBox="0 0 24 24" width="18" height="18"><path d="M12 3L20 6V12Q20 18 12 21Q4 18 4 12V6Z" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linejoin="round"/></svg>',
    soin: '<svg viewBox="0 0 24 24" width="18" height="18"><path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="3" stroke-linecap="round"/></svg>',
    fleche: '<svg viewBox="0 0 24 24" width="18" height="18"><path d="M5 12H18M13 6L19 12L13 18" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/></svg>',
    carte: '<svg viewBox="0 0 24 24" width="22" height="22"><path d="M3 6L9 4L15 6L21 4V18L15 20L9 18L3 20Z M9 4V18 M15 6V20" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round"/></svg>',
    ville: '<svg viewBox="0 0 24 24" width="22" height="22"><path d="M3 20V10L7 7L11 10V20M11 20V5L16 3L21 5V20M3 20H21" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round"/></svg>',
    clans: '<svg viewBox="0 0 24 24" width="22" height="22"><path d="M6 21V3M6 4H18L15 8L18 12H6" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round"/></svg>',
    savoirNav: '<svg viewBox="0 0 24 24" width="22" height="22"><path d="M3 6Q8 4 12 7Q16 4 21 6V19Q16 17 12 19.5Q8 17 3 19Z M12 7V19.5" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round"/></svg>',
  };

  // Assemble le plateau à partir des fonctions de dessin d'un style.
  function board(st) {
    const s = st.cell, lab = st.lab, W = lab + 6 * s, H = lab + 8 * s, R = rng(7);
    let g = st.under ? st.under(W, H, s, lab) : '';
    for (let r = 0; r < 8; r++) for (let c = 0; c < 6; c++) g += st.tile(T(c, r), lab + c * s, lab + r * s, s, R, c, r);
    if (st.grid) g += st.grid(W, H, s, lab);
    for (let c = 0; c < 6; c++) g += st.label(COLS[c], lab + c * s + s / 2, lab / 2, 'col');
    for (let r = 0; r < 8; r++) g += st.label(String(r + 1), lab / 2, lab + r * s + s / 2, 'row');
    const at = (o) => [lab + o.c * s, lab + o.r * s];
    g += st.city(...at(CITY), s, st.clans[0]);
    for (const m of MOVES) if (!m.attack && !(m.c === CITY.c && m.r === CITY.r)) g += st.move(...at(m), s);
    g += st.select(...at(SEL), s);
    for (const u of UNITS) g += st.unit(u, ...at(u), s, st.clans[u.clan]);
    for (const m of MOVES) if (m.attack) g += st.attack(...at(m), s);
    return `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}">${st.defs || ''}${g}</svg>`;
  }

  // Interface commune de la scène type (barre du haut, plateau, panneau du bas).
  function ecran(st, opts = {}) {
    const u = SEL, clan = st.clans[0];
    const token = st.miniToken ? st.miniToken(u.type, clan) : '';
    return `<div class="screen s-${st.id}">
      ${st.decor || ''}
      <div class="top">
        <div class="emb">${st.emblem ? st.emblem(clan) : ''}</div>
        <div class="turn"><small>Tour</small><b>7</b></div>
        <div class="res"><span class="r-or">${ICON.or}<b>12</b><small>+3</small></span><span class="r-sv">${ICON.savoir}<b>4</b><small>+1</small></span></div>
        <button class="menu" aria-label="Menu">${ICON.menu}</button>
      </div>
      <div class="board">${board(st)}</div>
      <div class="panel">
        <div class="card">${token}<div class="info"><b>Lancier</b><small>D4 · PV 10/10 · Pas 1 · Force 2</small></div></div>
        <div class="actions"><button class="ghost">${ICON.bouclier}<span>Fortifier</span></button><button class="end">Fin du tour</button></div>
      </div>
    </div>`;
  }

  window.Scene = { COLS, MAP, T, CITY, UNITS, SEL, MOVES, rng, mix, glyph, placeGlyph, ICON, board, ecran };
})();
