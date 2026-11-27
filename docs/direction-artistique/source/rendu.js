// Rend tous les aperçus en PNG (échelle 2) avec le Playwright de la session.
// Usage : node rendu.js <dossier node_modules/@fontsource>   (polices OFL installées par npm)
const fs = require('fs'), path = require('path');
const { chromium } = require('playwright');
const src = process.argv[2];
const here = __dirname, out = path.join(here, '..', 'apercus');
const fonts = ['im-fell-french-canon-latin-400-normal', 'im-fell-french-canon-latin-400-italic', 'pixelify-sans-latin-400-normal', 'pixelify-sans-latin-700-normal',
  'fredoka-latin-400-normal', 'fredoka-latin-600-normal', 'bitter-latin-400-normal', 'bitter-latin-700-normal', 'chakra-petch-latin-400-normal', 'chakra-petch-latin-700-normal',
  'nunito-latin-400-normal', 'nunito-latin-700-normal', 'atkinson-hyperlegible-latin-400-normal', 'atkinson-hyperlegible-latin-700-normal'];
fs.mkdirSync(path.join(here, 'fonts'), { recursive: true });
for (const f of fonts) { const fam = f.replace(/-latin-.*/, ''); fs.copyFileSync(path.join(src, fam, 'files', f + '.woff2'), path.join(here, 'fonts', f + '.woff2')); }
const jobs = [
  ['DA-000-comparatif', 'vue=comparatif'],
  ['DA-001-style-carnet', 'vue=style&id=encre'], ['DA-002-style-pixel', 'vue=style&id=pixel'], ['DA-003-style-papier', 'vue=style&id=papier'],
  ['DA-004-style-bois', 'vue=style&id=bois'], ['DA-005-style-tactique', 'vue=style&id=tactique'],
  ['DA-006-palette-jour', 'vue=palette&id=jour'], ['DA-007-palette-nuit', 'vue=palette&id=nuit'], ['DA-008-palette-lavis', 'vue=palette&id=lavis'], ['DA-009-palette-soleil', 'vue=palette&id=soleil'],
  ['DA-010-interface-pouce', 'vue=interface&id=pouce'], ['DA-011-interface-anneau', 'vue=interface&id=anneau'], ['DA-012-interface-onglets', 'vue=interface&id=onglets'],
];
(async () => {
  const b = await chromium.launch();
  const p = await b.newPage({ deviceScaleFactor: 2, viewport: { width: 1600, height: 1200 } });
  p.on('pageerror', e => console.log('ERREUR', e.message));
  for (const [name, qs] of jobs) {
    if (process.argv[3] && !name.includes(process.argv[3])) continue;
    await p.goto('file://' + path.join(here, 'apercu.html') + '?' + qs);
    await p.waitForSelector('body[data-pret="1"]');
    await p.locator('#shot').screenshot({ path: path.join(out, name + '.png') });
    console.log('ok', name);
  }
  await b.close();
})();
