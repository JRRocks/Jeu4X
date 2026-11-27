// Les trois modèles d'interface (DA-010 à DA-012), dessinés avec la palette « Jour franc » et le style plat.
(function () {
  const { board, ICON, placeGlyph } = Scene;
  const P = () => PALETTES.jour;
  const vars = (p, font) => `--bg:${p.ui.bg};--surf:${p.ui.surf};--text:${p.ui.text};--text2:${p.ui.text2};--accent:${p.ui.accent};--btn:${p.ui.btn};--btnText:${p.ui.btnText};--line:${p.ui.line};--c0:${p.clans[0]};--font:${font}`;
  const st = () => STYLES.plat(P());
  const tokenSvg = (t, c, size = 40) => { const ink = Couleurs.contrast(c, '#000000') > Couleurs.contrast(c, '#FFFFFF') ? '#111' : '#FFF'; return `<svg width="${size}" height="${size}" viewBox="0 0 40 40"><circle cx="20" cy="20" r="17.5" fill="${c}" stroke="#1C1E24" stroke-width="2"/>${placeGlyph(t, ink, 20, 20, 22, 2.6)}</svg>`; };
  const cellCenter = (c, r, s = 54, lab = 18) => [lab + c * s + s / 2, lab + r * s + s / 2];
  const topBar = () => `<div class="top"><div class="emb">${st().emblem(P().clans[0])}</div><div class="turn"><small>Tour</small><b>7</b></div><div class="res"><span class="r-or">${ICON.or}<b>12</b><small>+3</small></span><span class="r-sv">${ICON.savoir}<b>4</b><small>+1</small></span></div><button class="menu" aria-label="Menu">${ICON.menu}</button></div>`;
  const css = `
    .i-sheet { position: absolute; left: 0; right: 0; bottom: 0; background: var(--surf); border-radius: 22px 22px 0 0; box-shadow: 0 -4px 16px #0002; padding: 8px 14px 20px; display: flex; flex-direction: column; gap: 10px; z-index: 3; }
    .i-handle { width: 40px; height: 5px; border-radius: 3px; background: #C9C3B5; align-self: center; }
    .i-row { display: flex; gap: 10px; align-items: center; }
    .i-sq { flex: 1; height: 58px; border-radius: 14px; background: var(--bg); border: 1.5px solid var(--line); display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 2px; font-size: 13px; font-weight: 700; }
    .i-end { height: 60px; border-radius: 16px; background: var(--btn); color: var(--btnText); font-size: 20px; font-weight: 700; width: 100%; display: flex; align-items: center; justify-content: center; gap: 8px; }
    .i-hp { height: 6px; border-radius: 3px; background: #E3DED2; overflow: hidden; width: 120px; } .i-hp i { display: block; height: 100%; width: 100%; background: #2E8B57; }
    .i-list { display: flex; flex-direction: column; gap: 8px; }
    .i-item { display: flex; align-items: center; gap: 10px; padding: 6px 10px; border: 1.5px solid var(--line); border-radius: 14px; min-height: 58px; }
    .i-item .grow { flex: 1; display: flex; flex-direction: column; } .i-item small { color: var(--text2); font-size: 13px; }
    .i-buy { height: 44px; padding: 0 14px; border-radius: 12px; background: var(--btn); color: var(--btnText); font-weight: 700; display: flex; align-items: center; gap: 4px; }
    .i-buy.off { background: #E3DED2; color: #555B66; }
    .i-dim { position: absolute; inset: 0; background: #0005; z-index: 2; }
    .bw { position: relative; }
    .ring { position: absolute; width: 52px; height: 52px; border-radius: 50%; background: var(--surf); border: 2px solid var(--text); display: grid; place-items: center; transform: translate(-50%, -50%); box-shadow: 0 3px 8px #0003; z-index: 4; }
    .ring small { position: absolute; top: 54px; font-size: 12px; font-weight: 600; background: var(--text); color: var(--surf); padding: 1px 7px; border-radius: 8px; white-space: nowrap; }
    .ring.main { background: var(--btn); color: var(--btnText); border-color: var(--btn); }
    .ring .cost { position: absolute; top: -8px; right: -10px; background: var(--c0); color: #1C1E24; font-size: 12px; font-weight: 700; border-radius: 9px; padding: 0 6px; border: 1.5px solid #1C1E24; }
    .pills { position: absolute; top: 14px; left: 12px; right: 12px; display: flex; gap: 8px; align-items: center; z-index: 3; }
    .pillb { background: var(--surf); border-radius: 24px; height: 48px; padding: 0 14px; display: flex; align-items: center; gap: 8px; box-shadow: 0 2px 8px #0002; font-size: 17px; }
    .pillb small { font-size: 12px; color: var(--text2); }
    .fab { position: absolute; right: 16px; bottom: 22px; width: 92px; height: 92px; border-radius: 50%; background: var(--btn); color: var(--btnText); font-size: 16px; font-weight: 600; line-height: 1.1; display: grid; place-items: center; text-align: center; box-shadow: 0 6px 14px #0004; z-index: 3; }
    .chip { position: absolute; left: 14px; bottom: 34px; background: var(--surf); border-radius: 18px; padding: 8px 12px 8px 8px; display: flex; gap: 8px; align-items: center; box-shadow: 0 2px 8px #0002; z-index: 3; }
    .chip small { color: var(--text2); font-size: 12px; display: block; }
    .hint { position: absolute; top: 76px; left: 50%; transform: translateX(-50%); background: var(--text); color: var(--surf); padding: 6px 14px; border-radius: 14px; font-size: 14px; white-space: nowrap; z-index: 5; }
    .strip { margin-top: auto; background: var(--surf); border-top: 1px solid var(--line); padding: 10px 12px; display: flex; flex-direction: column; gap: 8px; }
    .nav { height: 72px; background: var(--surf); border-top: 1px solid var(--line); display: grid; grid-template-columns: repeat(4, 1fr); padding-bottom: 6px; }
    .nav div { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 2px; font-size: 13px; color: var(--text2); }
    .nav div.on { color: var(--text); font-weight: 700; } .nav div.on svg { background: #DCE6F7; border-radius: 14px; padding: 2px 14px; width: 56px; }
    .t-btn { height: 52px; border-radius: 12px; padding: 0 12px; display: flex; align-items: center; gap: 6px; font-weight: 700; font-size: 15px; border: 1.5px solid var(--text2); }
    .t-end { flex: 1; height: 52px; border-radius: 12px; background: var(--btn); color: var(--btnText); font-weight: 700; font-size: 18px; display: flex; align-items: center; justify-content: center; }
    .tech { display: flex; flex-direction: column; gap: 8px; padding: 12px; flex: 1; overflow: hidden; }
    .tech h3 { font-size: 20px; } .tech p { color: var(--text2); font-size: 14px; margin-bottom: 4px; }
    .badge { font-size: 12px; font-weight: 700; padding: 2px 8px; border-radius: 8px; }
  `;
  const head = (num, nom, txt) => `<div class="head"><h1><span>${num}</span>Interface « ${nom} »</h1><p>${txt}</p></div>`;
  const notes = (titre, items) => `<div class="notes"><b>${titre}</b><ol>${items.map(i => `<li>${i}</li>`).join('')}</ol></div>`;
  const wrap = (h, a, b) => `<style>${css}</style><div style="display:flex;flex-direction:column;gap:18px">${h}<div style="display:flex;gap:28px">${a}${b}</div></div>`;
  const screen = (font, inner) => `<div class="phone"><div class="screen s-plat" style="${vars(P(), font)}">${inner}</div></div>`;
  const unitCard = () => `<div class="i-row">${tokenSvg('lance', P().clans[0], 44)}<div style="display:flex;flex-direction:column;gap:4px;flex:1"><b style="font-size:18px">Lancier <small style="font-weight:400;font-size:13px;color:var(--text2)">en D4</small></b><div class="i-row" style="gap:8px"><div class="i-hp"><i></i></div><small style="font-size:13px;color:var(--text2)">PV 10/10 · Pas 1 · Force 2</small></div></div></div>`;

  const I = {};

  I.pouce = {
    html() {
      const f = 'Nunito';
      const a = screen(f, `${topBar()}<div class="board" style="flex:none;padding-top:4px">${board(st())}</div>
        <div class="i-sheet"><div class="i-handle"></div>${unitCard()}
          <div class="i-row"><button class="i-sq">${ICON.bouclier}Fortifier</button><button class="i-sq">${ICON.soin}Soigner</button><button class="i-sq">${ICON.fleche}Plus</button></div>
          <button class="i-end">Fin du tour</button></div>`);
      const prod = [['lance', 'Lancier', 'Défend bien · 2 tours', '4'], ['arc', 'Archère', 'Tire à 2 cases · 2 tours', '5'], ['hache', 'Éclaireur', 'Pas 2 · 1 tour', '3']];
      const b = screen(f, `${topBar()}<div class="board" style="flex:none;padding-top:4px">${board(st())}</div><div class="i-dim"></div>
        <div class="i-sheet" style="height:470px"><div class="i-handle"></div>
          <div class="i-row" style="justify-content:space-between"><div><b style="font-size:21px">Ostrelle</b><br><small style="color:var(--text2);font-size:14px">C3 · niveau 2 · +3 or par tour</small></div><button class="i-sq" style="flex:none;width:52px;height:48px;font-size:22px">×</button></div>
          <small style="font-weight:700;font-size:13px;letter-spacing:.05em;color:var(--text2)">PRODUIRE</small>
          <div class="i-list">${prod.map(([t, n, d, c]) => `<div class="i-item">${tokenSvg(t, P().clans[0], 38)}<div class="grow"><b>${n}</b><small>${d}</small></div><button class="i-buy">${ICON.or}${c}</button></div>`).join('')}
            <div class="i-item"><svg width="38" height="38" viewBox="0 0 24 24"><path d="M3 20V9h3V6h3v3h6V6h3v3h3v11Z" fill="#9A8F86" stroke="#1C1E24" stroke-width="1.5"/></svg><div class="grow"><b>Muraille</b><small>+50 % de défense</small></div><button class="i-buy off">${ICON.or}14</button></div></div>
          <button class="i-end">Fin du tour</button></div>`);
      return wrap(head('DA-010', 'Pouce', 'Tout se joue dans le tiers bas de l’écran, à portée du pouce. Un panneau glisse depuis le bas.'),
        `<div class="col">${a}${notes('Écran de jeu, Lancier choisi', ['Les actions de l’unité choisie sont en bas, à portée du pouce, icône et mot.', '« Fin du tour » est le plus gros bouton, toujours au même endroit.', 'Barre du haut : seulement le tour, l’or et le savoir.', 'Police : Nunito, ronde et lisible.'])}</div>`,
        `<div class="col">${b}${notes('Ville touchée : le panneau monte', ['Le panneau couvre la carte ; un geste vers le bas ou × le referme.', 'Chaque ligne : ce qu’on obtient, son effet, son prix. Bouton grisé si trop cher.', 'Cibles de 44 à 60 px, au moins 8 px d’écart.'])}</div>`);
    },
  };

  I.anneau = {
    html() {
      const f = 'Fredoka';
      const s = st();
      const [ux, uy] = cellCenter(3, 3);
      const ringA = [[0, -62, ICON.bouclier, 'Fortifier'], [62, 0, ICON.soin, 'Soigner'], [-62, 0, ICON.fleche, 'Infos']]
        .map(([dx, dy, ic, l]) => `<div class="ring" style="left:${ux + dx}px;top:${uy + dy}px">${ic}<small>${l}</small></div>`).join('');
      const pills = `<div class="pills"><div class="pillb"><small>Tour</small><b>7</b></div><div class="pillb" style="color:var(--text)"><span class="r-or" style="display:flex;gap:4px;align-items:center;color:${P().clans[0]}">${ICON.or}</span><b>12</b><small>+3</small><span style="display:flex;color:var(--accent)">${ICON.savoir}</span><b>4</b></div><div class="pillb" style="margin-left:auto;width:48px;padding:0;justify-content:center">${ICON.menu}</div></div>`;
      const a = screen(f, `${pills}<div class="board" style="padding-top:56px;padding-bottom:110px"><div class="bw">${board(s)}${ringA}</div></div>
        <div class="chip">${tokenSvg('lance', P().clans[0], 40)}<div><b>Lancier</b><small>PV 10/10 · Pas 1</small></div></div><button class="fab">Fin du<br>tour</button>`);
      const [cx, cy] = cellCenter(2, 2);
      const ringB = [[0, -66, 'lance', 'Lancier', '4'], [64, 26, 'arc', 'Archère', '5'], [-64, 26, 'hache', 'Éclaireur', '3']]
        .map(([dx, dy, t, l, c]) => `<div class="ring" style="left:${cx + dx}px;top:${cy + dy}px">${tokenSvg(t, P().clans[0], 40)}<span class="cost">${c}</span><small>${l}</small></div>`).join('')
        + `<div class="ring main" style="left:${cx}px;top:${cy}px;width:44px;height:44px;font-size:22px">×</div>`;
      const b = screen(f, `${pills}<div class="hint">Que produit Ostrelle ?</div><div class="board" style="padding-top:56px;padding-bottom:110px"><div class="bw">${board(s)}${ringB}</div></div>
        <div class="chip"><svg width="40" height="40" viewBox="0 0 40 40"><rect x="3" y="3" width="34" height="34" rx="8" fill="#fff" stroke="${P().clans[0]}" stroke-width="3"/></svg><div><b>Ostrelle</b><small>niveau 2 · +3 or</small></div></div><button class="fab">Fin du<br>tour</button>`);
      return wrap(head('DA-011', 'Anneau', 'Le plateau occupe tout l’écran ; les actions s’ouvrent en anneau autour de ce qu’on touche.'),
        `<div class="col">${a}${notes('Lancier touché : l’anneau s’ouvre', ['Toucher une unité ouvre ses actions autour d’elle ; toucher ailleurs referme.', 'Attaquer : toucher directement l’ennemi marqué.', '« Fin du tour » : gros bouton rond en bas à droite, pour le pouce droit.', 'Police : Fredoka, arrondie, ton léger.'])}</div>`,
        `<div class="col">${b}${notes('Ville touchée : production en anneau', ['Trois choix et leur prix autour de la ville, × au centre pour annuler.', 'Près d’un bord, l’anneau se replie vers l’intérieur.', 'Limite : le doigt peut cacher l’anneau, et plus de 4 choix ne tiennent pas.'])}</div>`);
    },
  };

  I.onglets = {
    html() {
      const f = 'Atkinson';
      const nav = on => `<div class="nav">${[['Carte', ICON.carte], ['Villes', ICON.ville], ['Savoir', ICON.savoirNav], ['Clans', ICON.clans]].map(([n, ic]) => `<div class="${n === on ? 'on' : ''}">${ic}${n}</div>`).join('')}</div>`;
      const a = screen(f, `${topBar()}<div class="board" style="flex:none;padding-top:2px">${board(st())}</div>
        <div class="strip">${unitCard()}<div class="i-row"><button class="t-btn">${ICON.bouclier}Fortifier</button><button class="t-end">Fin du tour</button></div></div>${nav('Carte')}`);
      const techs = [['Taille de pierre', 'Débloque la Muraille', '<span class="badge" style="background:#D7F0DE;color:#14532D">✓ acquis</span>'], ['Archerie', 'Débloque l’Archère', `<button class="i-buy">${ICON.savoir}5</button>`], ['Navigation côtière', 'Débloque le Radeau', `<button class="i-buy">${ICON.savoir}6</button>`], ['Élevage', '+1 nourriture en plaine', `<button class="i-buy">${ICON.savoir}4</button>`], ['Écriture', 'Demande Archerie', '<span class="badge" style="background:#E3DED2;color:#3C3F46">🔒 fermé</span>']];
      const b = screen(f, `${topBar()}<div class="tech"><h3>Savoir</h3><p>4 points, +1 par tour. Touchez une connaissance pour l’apprendre.</p>
        <div class="i-list">${techs.map(([n, d, x]) => `<div class="i-item" style="background:var(--surf)"><div class="grow"><b style="font-size:17px">${n}</b><small>${d}</small></div>${x}</div>`).join('')}</div></div>${nav('Savoir')}`);
      return wrap(head('DA-012', 'Onglets', 'Une barre d’onglets en bas range le jeu en quatre écrans : carte, villes, savoir, clans.'),
        `<div class="col">${a}${notes('Onglet Carte, Lancier choisi', ['Bandeau de l’unité au-dessus des onglets ; « Fin du tour » à côté de l’action principale.', 'Onglets toujours visibles, icône et mot, l’actif surligné.', 'Repère familier sur Android (barre de navigation Material).', 'Police : Atkinson Hyperlegible, pensée pour les vues faibles.'])}</div>`,
        `<div class="col">${b}${notes('Onglet Savoir', ['Chaque écran a une seule tâche ; liste simple, prix à droite.', 'État écrit en clair (acquis, fermé), pas seulement en couleur.', 'Limite : plus de pages, plus de va-et-vient pendant un tour.'])}</div>`);
    },
  };

  window.INTERFACES = I;
})();
