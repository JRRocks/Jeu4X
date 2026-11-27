//! Mesures d'équilibre pour la session 2 : durée des parties et façon dont elles se gagnent.
//!
//! Une même IA tient toutes les places ; les clans tournent d'une partie à l'autre pour que
//! chaque clan occupe chaque place. Reproductible par graine.

use crate::melanger;
use crate::profil::Profil;
use regles::partie::{Partie, Reglages};

/// Comment une partie s'est terminée.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fin {
    Capitale,
    Points,
    Nulle,
    /// Garde-fou du terrain d'essai : trop d'actions (ne devrait pas arriver).
    Bloquee,
}

#[derive(Clone, Debug)]
pub struct Bilan {
    pub tours: u16,
    pub fin: Fin,
    pub gagnant: Option<u8>,
    pub clans: Vec<u8>,
    pub scores: Vec<i32>,
    pub villes_gagnant: i32,
}

/// Clans de la partie `g` : chaque clan passe par chaque place.
pub fn clans_de(g: u64, nb: u8) -> [u8; 4] {
    let mut c = [0u8; 4];
    let decalage = (g % 4) as u8;
    // Pas premier avec 4 (1 ou 3) pour que les clans restent distincts ; à deux, le pas 2
    // donne aussi les paires opposées (0-2, 1-3).
    let pas = if nb == 2 { 1 + ((g / 4) % 3) as u8 } else { [1, 3][((g / 4) % 2) as usize] };
    for (i, x) in c.iter_mut().enumerate().take(nb as usize) {
        *x = (decalage + pas * i as u8) % 4;
    }
    c
}

pub fn jouer(profil: &Profil, nb: u8, g: u64, graine: u64) -> Bilan {
    let mut r = Reglages::pour(nb);
    r.clans = clans_de(g, nb);
    let graine_partie = melanger(graine ^ melanger(g));
    let mut joueurs: Vec<_> = (0..nb).map(|j| profil.fabriquer(melanger(graine_partie ^ (j as u64 + 1)))).collect();
    let mut p = Partie::nouvelle(r, graine_partie);
    let mut actions = 0;
    let mut bloquee = false;
    while !p.est_finie() {
        if actions > 50_000 {
            bloquee = true;
            break;
        }
        let j = p.joueur_courant() as usize;
        let a = joueurs[j].choisir(&p);
        if p.appliquer(a).is_err() {
            bloquee = true;
            break;
        }
        actions += 1;
    }
    let scores: Vec<i32> = (0..nb).map(|j| p.score(j)).collect();
    let gagnant = p.gagnant();
    let fin = if bloquee {
        Fin::Bloquee
    } else {
        match gagnant {
            None => Fin::Nulle,
            // Un adversaire éliminé : une capitale a été prise.
            Some(_) if p.joueurs().iter().any(|j| j.elimine) => Fin::Capitale,
            Some(_) => Fin::Points,
        }
    };
    let villes_gagnant = gagnant.map_or(0, |g| p.villes().iter().filter(|v| v.joueur == Some(g)).count() as i32);
    Bilan { tours: p.tour(), fin, gagnant, clans: r.clans[..nb as usize].to_vec(), scores, villes_gagnant }
}

pub fn jouer_tout(profil: &Profil, nb: u8, parties: u64, graine: u64, fils: usize) -> Vec<Bilan> {
    let liste: Vec<u64> = (0..parties).collect();
    let part = liste.len().div_ceil(fils.max(1)).max(1);
    std::thread::scope(|s| {
        let t: Vec<_> = liste
            .chunks(part)
            .map(|m| s.spawn(move || m.iter().map(|&g| jouer(profil, nb, g, graine)).collect::<Vec<_>>()))
            .collect();
        t.into_iter().flat_map(|h| h.join().expect("un fil a échoué")).collect()
    })
}

const NOMS_CLANS: [&str; 4] = ["Crocs-Rouges", "Murailliers", "Veilleurs d'Astres", "Vents-Errants"];

/// Rapport en Markdown d'une série de parties à `nb` joueurs.
pub fn rapport(nb: u8, b: &[Bilan]) -> String {
    let n = b.len().max(1) as f64;
    let mut tours: Vec<u16> = b.iter().map(|x| x.tours).collect();
    tours.sort_unstable();
    let moy = tours.iter().map(|&t| t as f64).sum::<f64>() / n;
    let med = tours.get(tours.len() / 2).copied().unwrap_or(0);
    let pc = |f: Fin| 100.0 * b.iter().filter(|x| x.fin == f).count() as f64 / n;
    let r = Reglages::pour(nb);
    let mut s = format!(
        "### {nb} joueurs ({t}×{t}, seuil {seuil})\n\n{} parties. Durée : {moy:.1} tours en moyenne, médiane {med}, de {} à {}.\n\n",
        b.len(),
        tours.first().unwrap_or(&0),
        tours.last().unwrap_or(&0),
        t = r.taille,
        seuil = r.seuil()
    );
    s += &format!(
        "| Fin | Part |\n|---|---|\n| capitale prise (ou dernier debout) | {:.0} % |\n| seuil de points | {:.0} % |\n| nulle au tour {} | {:.0} % |\n",
        pc(Fin::Capitale),
        pc(Fin::Points),
        r.limite_tours,
        pc(Fin::Nulle)
    );
    if pc(Fin::Bloquee) > 0.0 {
        s += &format!("| bloquée (erreur) | {:.0} % |\n", pc(Fin::Bloquee));
    }
    // Répartition de la durée par tranches de 5 tours.
    s += "\n| Tours | Parties |\n|---|---|\n";
    let max = *tours.last().unwrap_or(&0) as usize;
    for d in (0..=max).step_by(5) {
        let k = tours.iter().filter(|&&t| (t as usize) >= d && (t as usize) < d + 5).count();
        if k > 0 {
            s += &format!("| {d} à {} | {k} |\n", d + 4);
        }
    }
    // Victoires par clan (rapportées aux parties où le clan jouait) et par place.
    s += "\n| Clan | Parties jouées | Victoires |\n|---|---|---|\n";
    for (c, nom) in NOMS_CLANS.iter().enumerate() {
        let jouees = b.iter().filter(|x| x.clans.contains(&(c as u8))).count();
        let gagnees = b.iter().filter(|x| x.gagnant.is_some_and(|g| x.clans[g as usize] == c as u8)).count();
        if jouees > 0 {
            s += &format!("| {nom} | {jouees} | {:.0} % |\n", 100.0 * gagnees as f64 / jouees as f64);
        }
    }
    s += "\n| Place | Victoires |\n|---|---|\n";
    for j in 0..nb {
        let g = b.iter().filter(|x| x.gagnant == Some(j)).count();
        s += &format!("| {} | {:.0} % |\n", j + 1, 100.0 * g as f64 / n);
    }
    let gagnes: Vec<&Bilan> = b.iter().filter(|x| x.gagnant.is_some()).collect();
    if !gagnes.is_empty() {
        let k = gagnes.len() as f64;
        let sc = gagnes.iter().map(|x| x.scores[x.gagnant.unwrap() as usize] as f64).sum::<f64>() / k;
        let vi = gagnes.iter().map(|x| x.villes_gagnant as f64).sum::<f64>() / k;
        s += &format!("\nLe gagnant finit en moyenne avec {sc:.1} points et {vi:.1} villes.\n");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_clan_occupe_chaque_place() {
        let mut vu = [[false; 4]; 4];
        for g in 0..12 {
            let c = clans_de(g, 4);
            let mut tri = c;
            tri.sort_unstable();
            assert_eq!(tri, [0, 1, 2, 3], "partie {g} : {c:?}");
            for (place, &clan) in c.iter().enumerate() {
                vu[place][clan as usize] = true;
            }
        }
        assert!(vu.iter().all(|l| l.iter().all(|&x| x)));
    }

    #[test]
    fn des_parties_a_trois_et_quatre_vont_au_bout() {
        let p = Profil::utilite("u", crate::criteres::POIDS_ESSAI);
        for nb in [3, 4] {
            for b in jouer_tout(&p, nb, 4, 1, 2) {
                assert_ne!(b.fin, Fin::Bloquee);
            }
        }
    }
}
