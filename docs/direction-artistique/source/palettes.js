// Les quatre palettes proposées (DA-006 à DA-009). Terrains : P plaine, F forêt, M montagne, E eau, S sable, X brouillard.
window.PALETTES = {
  jour: {
    num: 'DA-006', nom: 'Jour franc', intention: 'Couleurs vives et nettes, lisibles de jour, ton joyeux.',
    terr: { P: '#9CCF63', F: '#3E7140', M: '#A88E78', E: '#3193EC', S: '#F3E2B5', X: '#2A3F63' },
    clans: ['#F3B01D', '#4A45D1', '#BA1A0A', '#5CCBFA', '#F2F2F2', '#2A2A2A'],
    clanNoms: ['Or', 'Indigo', 'Rouge', 'Ciel', 'Neige', 'Charbon'],
    ui: { bg: '#F7F4EC', surf: '#FFFFFF', text: '#1C1E24', text2: '#555B66', accent: '#1F5FBF', btn: '#1F5FBF', btnText: '#FFFFFF', alert: '#B3261E', line: '#D9D3C4' },
  },
  nuit: {
    num: 'DA-007', nom: 'Nuit', intention: 'Fond sombre pour jouer le soir et ménager la batterie (écran OLED).',
    terr: { P: '#61754C', F: '#15561D', M: '#8A7590', E: '#17375A', S: '#AB8C2C', X: '#05070F' },
    clans: ['#FFE14A', '#8C7BFF', '#FF5C4D', '#62D8F0', '#F5F5F5', '#C0486E'],
    clanNoms: ['Soleil', 'Lavande', 'Braise', 'Glace', 'Lune', 'Grenat'],
    ui: { bg: '#101318', surf: '#1C212A', text: '#EEF1F5', text2: '#A9B1BD', accent: '#7CC4FF', btn: '#7CC4FF', btnText: '#0B1220', alert: '#FF8A80', line: '#2C3440' },
  },
  lavis: {
    num: 'DA-008', nom: 'Lavis', intention: 'Tons doux de papier et d’aquarelle, clans à l’encre franche.',
    terr: { P: '#D3D9AE', F: '#7CA773', M: '#D2B48C', E: '#6FB5C8', S: '#FAEBCB', X: '#E9DFC6' },
    clans: ['#26529C', '#D9A440', '#B0301E', '#7CC6F0', '#2E2A26', '#F2EAD6'],
    clanNoms: ['Outremer', 'Ocre', 'Vermillon', 'Azur', 'Encre', 'Vélin'],
    ui: { bg: '#F3EAD6', surf: '#FBF6EA', text: '#2E2A26', text2: '#5E554A', accent: '#8E2A1C', btn: '#8E2A1C', btnText: '#FBF6EA', alert: '#8E2A1C', line: '#CDBF9F' },
  },
  soleil: {
    num: 'DA-009', nom: 'Plein soleil', intention: 'Contraste maximal pour jouer dehors ou avec une vue fatiguée.',
    terr: { P: '#ACCE6B', F: '#2E8540', M: '#8C8C8C', E: '#002E8A', S: '#FFF4C2', X: '#000000' },
    clans: ['#FFE600', '#3A10E5', '#D44400', '#00E5FF', '#000000', '#FFFFFF'],
    clanNoms: ['Jaune', 'Roi', 'Feu', 'Cyan', 'Noir', 'Blanc'],
    ui: { bg: '#FFFFFF', surf: '#FFFFFF', text: '#000000', text2: '#2B2B2B', accent: '#0047AB', btn: '#000000', btnText: '#FFE600', alert: '#C00000', line: '#000000' },
  },
};
