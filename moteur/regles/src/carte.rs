//! Création de la carte (règles V1, § 2) : le seul moment où le hasard intervient.
//!
//! Terrain en taches (forêts, chaînes de collines et de montagnes, lacs) tiré de deux
//! champs lissés, « altitude » et « humidité », puis classé pour tomber exactement sur
//! les parts du § 1.3. Capitales placées loin l'une de l'autre, puis vérifiées pour des
//! départs équitables (§ 2.3). Entiers seulement.

use crate::hasard::Hasard;
use crate::plateau::{Case, Plateau, Terrain};

/// Parts de la carte, en pour cent (§ 12.2) : eau, montagne, colline, forêt ; le reste en plaine.
const PART_EAU: usize = 15;
const PART_MONTAGNE: usize = 8;
const PART_COLLINE: usize = 12;
const PART_FORET: usize = 20;

/// Essais de placement des capitales sur une même carte.
const ESSAIS_PLACEMENT: u32 = 200;

/// Distance minimale d'une capitale au bord (§ 2.2).
const BORD_MIN: u8 = 2;

pub struct Carte {
    pub plateau: Plateau,
    /// Capitale de chaque joueur, dans l'ordre des joueurs.
    pub capitales: Vec<Case>,
}

/// Champ de valeurs 0..1000 lissé deux fois (moyenne des 3 × 3 cases) : des taches, pas du bruit.
fn champ_lisse(p: &Plateau, h: &mut Hasard) -> Vec<u32> {
    let n = p.nb_cases();
    let mut v: Vec<u32> = (0..n).map(|_| h.sous(1000)).collect();
    for _ in 0..2 {
        let mut w = vec![0u32; n];
        for c in 0..n as Case {
            let (mut somme, mut nb) = (0u32, 0u32);
            for d in p.carre(c, 1) {
                somme += v[d as usize];
                nb += 1;
            }
            w[c as usize] = somme / nb;
        }
        v = w;
    }
    v
}

/// Terrain : les plus basses altitudes en eau, les plus hautes en montagne puis colline,
/// les plus humides du reste en forêt. À égalité, l'ordre des cases tranche.
fn tirer_terrain(taille: u8, h: &mut Hasard) -> Plateau {
    let mut p = Plateau::uni(taille, Terrain::Plaine);
    let n = p.nb_cases();
    let altitude = champ_lisse(&p, h);
    let humidite = champ_lisse(&p, h);
    let mut par_altitude: Vec<Case> = (0..n as Case).collect();
    par_altitude.sort_by_key(|&c| (altitude[c as usize], c));
    let (n_eau, n_mont, n_coll) = (n * PART_EAU / 100, n * PART_MONTAGNE / 100, n * PART_COLLINE / 100);
    for &c in &par_altitude[..n_eau] {
        p.poser(c, Terrain::Eau);
    }
    for &c in &par_altitude[n - n_mont..] {
        p.poser(c, Terrain::Montagne);
    }
    for &c in &par_altitude[n - n_mont - n_coll..n - n_mont] {
        p.poser(c, Terrain::Colline);
    }
    let mut reste: Vec<Case> = (0..n as Case).filter(|&c| p.terrain(c) == Terrain::Plaine).collect();
    reste.sort_by_key(|&c| (std::cmp::Reverse(humidite[c as usize]), c));
    for &c in reste.iter().take(n * PART_FORET / 100) {
        p.poser(c, Terrain::Foret);
    }
    p
}

/// Mélange de Fisher-Yates, tiré du hasard de la carte.
fn melanger(v: &mut [Case], h: &mut Hasard) {
    for i in (1..v.len()).rev() {
        let j = h.sous(i as u32 + 1) as usize;
        v.swap(i, j);
    }
}

/// Échange le terrain de `c` avec celui d'une case tirée loin des capitales (à 3 au moins)
/// et dont le terrain vérifie `accepte`. Les parts du § 1.3 restent exactes. `false` si
/// aucune case ne convient.
fn echanger_au_loin(p: &mut Plateau, capitales: &[Case], c: Case, accepte: impl Fn(Terrain) -> bool, h: &mut Hasard) -> bool {
    let loin: Vec<Case> = (0..p.nb_cases() as Case)
        .filter(|&w| accepte(p.terrain(w)) && capitales.iter().all(|&k| p.distance(k, w) >= 3))
        .collect();
    if loin.is_empty() {
        return false;
    }
    let w = loin[h.sous(loin.len() as u32) as usize];
    let (tc, tw) = (p.terrain(c), p.terrain(w));
    p.poser(w, tc);
    p.poser(c, tw);
    true
}

/// Plaines à distance 2 ou moins de la capitale (sa case non comprise).
fn plaines_autour(p: &Plateau, c: Case) -> i32 {
    p.carre(c, 2).filter(|&w| w != c && p.terrain(w) == Terrain::Plaine).count() as i32
}

/// Retouche les abords des capitales pour des départs équitables (§ 2.3), par échanges
/// de terrain avec des cases lointaines : au moins 4 voisines franchissables, puis un
/// nombre de plaines proche pour tous. Les conditions restent vérifiées ensuite.
fn equilibrer(p: &mut Plateau, capitales: &[Case], h: &mut Hasard) {
    let mut v = [0; 8];
    for &c in capitales {
        loop {
            let nv = p.voisines(c, &mut v);
            let libres = v[..nv].iter().filter(|&&w| p.terrain(w).cout().is_some()).count();
            let Some(&bloquee) = v[..nv].iter().find(|&&w| p.terrain(w).cout().is_none()) else { break };
            if libres >= 4 || !echanger_au_loin(p, capitales, bloquee, |t| t == Terrain::Plaine, h) {
                break;
            }
        }
    }
    for _ in 0..24 {
        let plaines: Vec<i32> = capitales.iter().map(|&c| plaines_autour(p, c)).collect();
        let (n, somme) = (plaines.len() as i32, plaines.iter().sum::<i32>());
        let (imin, imax) = (0..plaines.len()).fold((0, 0), |(a, b), i| {
            (if plaines[i] < plaines[a] { i } else { a }, if plaines[i] > plaines[b] { i } else { b })
        });
        let c = if n * plaines[imin] - somme < -2 * n {
            // Trop peu de plaines : une case non plaine de ses abords devient plaine.
            let cap = capitales[imin];
            let cibles: Vec<Case> = p.carre(cap, 2).filter(|&w| w != cap && p.terrain(w) != Terrain::Plaine).collect();
            cibles.get(h.sous(cibles.len().max(1) as u32) as usize).copied().map(|w| (w, true))
        } else if n * plaines[imax] - somme > 2 * n {
            // Trop de plaines : une plaine à distance 2 devient forêt ou colline (franchissable).
            let cap = capitales[imax];
            let cibles: Vec<Case> = p.carre(cap, 2).filter(|&w| p.distance(cap, w) == 2 && p.terrain(w) == Terrain::Plaine).collect();
            cibles.get(h.sous(cibles.len().max(1) as u32) as usize).copied().map(|w| (w, false))
        } else {
            break;
        };
        let Some((w, vers_plaine)) = c else { break };
        let ok = if vers_plaine {
            echanger_au_loin(p, capitales, w, |t| t == Terrain::Plaine, h)
        } else {
            echanger_au_loin(p, capitales, w, |t| matches!(t, Terrain::Foret | Terrain::Colline), h)
        };
        if !ok {
            break;
        }
    }
}

/// Cases franchissables reliées à `depart` (parcours en largeur).
fn reliees(p: &Plateau, depart: Case) -> Vec<bool> {
    let mut vu = vec![false; p.nb_cases()];
    let mut file = vec![depart];
    vu[depart as usize] = true;
    let mut v = [0; 8];
    let mut k = 0;
    while k < file.len() {
        let c = file[k];
        k += 1;
        let nv = p.voisines(c, &mut v);
        for &w in &v[..nv] {
            if !vu[w as usize] && p.terrain(w).cout().is_some() {
                vu[w as usize] = true;
                file.push(w);
            }
        }
    }
    vu
}

/// Un essai de carte. `None` si une condition des départs équitables échoue (§ 2.3) :
/// l'appelant recommence avec une graine dérivée.
pub fn essayer(taille: u8, nb_joueurs: u8, distance_min: u8, h: &mut Hasard) -> Option<Carte> {
    let mut p = tirer_terrain(taille, h);
    // Capitales : cases à 2 au moins du bord, prises dans un ordre tiré au hasard et
    // gardées si elles sont assez loin des précédentes. Pure géométrie : on retente
    // jusqu'à `ESSAIS_PLACEMENT` fois sur la même carte avant de la refuser.
    let mut candidates: Vec<Case> = (0..p.nb_cases() as Case).filter(|&c| p.distance_au_bord(c) >= BORD_MIN).collect();
    let mut capitales: Vec<Case> = Vec::with_capacity(nb_joueurs as usize);
    for _ in 0..ESSAIS_PLACEMENT {
        melanger(&mut candidates, h);
        capitales.clear();
        for &c in &candidates {
            if capitales.iter().all(|&k| p.distance(k, c) >= distance_min) {
                capitales.push(c);
                if capitales.len() == nb_joueurs as usize {
                    break;
                }
            }
        }
        if capitales.len() == nb_joueurs as usize {
            break;
        }
    }
    if capitales.len() < nb_joueurs as usize {
        return None;
    }
    // Une capitale est sur une plaine : si sa case n'en est pas une, elle échange son terrain
    // avec une plaine tirée loin des capitales. Les parts du § 1.3 restent exactes.
    for &c in &capitales {
        if p.terrain(c) != Terrain::Plaine && !echanger_au_loin(&mut p, &capitales, c, |t| t == Terrain::Plaine, h) {
            return None;
        }
    }
    equilibrer(&mut p, &capitales, h);
    let mut v = [0; 8];
    // Au moins 4 voisines franchissables.
    for &c in &capitales {
        let nv = p.voisines(c, &mut v);
        if v[..nv].iter().filter(|&&w| p.terrain(w).cout().is_some()).count() < 4 {
            return None;
        }
    }
    // Plaines à distance 2 ou moins : chacun à 2 près de la moyenne (en entiers : |n·x − somme| ≤ 2n).
    let plaines: Vec<i32> = capitales
        .iter()
        .map(|&c| plaines_autour(&p, c))
        .collect();
    let (n, somme) = (plaines.len() as i32, plaines.iter().sum::<i32>());
    if plaines.iter().any(|&x| (n * x - somme).abs() > 2 * n) {
        return None;
    }
    // Un chemin praticable entre toutes les capitales.
    let r = reliees(&p, capitales[0]);
    if capitales.iter().any(|&c| !r[c as usize]) {
        return None;
    }
    Some(Carte { plateau: p, capitales })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_de_terrain_exactes() {
        let mut h = Hasard::new(5);
        let p = tirer_terrain(14, &mut h);
        let compte = |t: Terrain| (0..196u16).filter(|&c| p.terrain(c) == t).count();
        assert_eq!(compte(Terrain::Eau), 196 * 15 / 100);
        assert_eq!(compte(Terrain::Montagne), 196 * 8 / 100);
        assert_eq!(compte(Terrain::Colline), 196 * 12 / 100);
        assert_eq!(compte(Terrain::Foret), 196 * 20 / 100);
    }
}
