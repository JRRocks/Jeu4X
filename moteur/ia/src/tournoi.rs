//! Tournois entre IA (IA-001) : parties sans écran, en parallèle, reproductibles.
//!
//! Pour chaque paire d'IA, `parties` parties : chaque carte est jouée deux fois, une
//! fois de chaque côté, pour que l'avantage d'une carte ou d'une place s'annule. Les
//! cartes ne dépendent que de la graine du tournoi : toutes les paires jouent les mêmes.

use crate::melanger;
use crate::profil::Profil;
use regles::partie::{Partie, Reglages};
use regles::replay::{enregistrer, Replay};

/// Garde-fou : au-delà, la partie est perdue par l'IA qui ne finit pas son tour.
pub const ACTIONS_MAX: usize = 20_000;

/// Une partie à jouer : indices des profils aux places 0 et 1, graines.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rencontre {
    pub place0: usize,
    pub place1: usize,
    pub graine_partie: u64,
    pub graine_ia: u64,
}

/// Résultat d'une partie.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Issue {
    pub rencontre: Rencontre,
    /// Place gagnante (0 ou 1), `None` pour une égalité.
    pub gagnant: Option<u8>,
    /// Place fautive si une IA a joué une action refusée ou n'a pas fini son tour.
    pub forfait: Option<u8>,
    pub scores: Vec<i32>,
    pub replay: Replay,
}

impl Issue {
    /// Points de la place `p` : 1 victoire, ½ égalité, 0 défaite, en demi-points.
    pub fn demi_points(&self, p: u8) -> u32 {
        match self.gagnant {
            Some(g) if g == p => 2,
            Some(_) => 0,
            None => 1,
        }
    }
}

/// Joue une partie entière entre deux profils.
pub fn jouer(profils: &[Profil], r: Rencontre, reglages: Reglages) -> Issue {
    let mut joueurs = [
        profils[r.place0].fabriquer(melanger(r.graine_ia)),
        profils[r.place1].fabriquer(melanger(r.graine_ia ^ 1)),
    ];
    let mut p = Partie::nouvelle(reglages, r.graine_partie);
    let mut actions = Vec::new();
    let mut forfait = None;
    while !p.est_finie() {
        let qui = p.joueur_courant();
        if actions.len() >= ACTIONS_MAX {
            forfait = Some(qui);
            break;
        }
        let a = joueurs[qui as usize].choisir(&p);
        if p.appliquer(a).is_err() {
            forfait = Some(qui);
            break;
        }
        actions.push(a.numero(reglages.taille));
    }
    let gagnant = match forfait {
        Some(f) => Some(1 - f),
        None => p.gagnant(),
    };
    let replay = enregistrer(reglages, r.graine_partie, &actions).expect("les actions viennent d'être acceptées");
    Issue { rencontre: r, gagnant, forfait, scores: p.scores(), replay }
}

/// Liste des rencontres d'un ensemble de paires, dans un ordre fixe.
pub fn rencontres(paires: &[(usize, usize)], parties: u32, graine: u64) -> Vec<Rencontre> {
    let mut sortie = Vec::new();
    for (np, &(a, b)) in paires.iter().enumerate() {
        for k in 0..parties {
            let carte = melanger(graine ^ melanger(k as u64 / 2));
            let (place0, place1) = if k % 2 == 0 { (a, b) } else { (b, a) };
            let graine_ia = melanger(carte ^ melanger(np as u64 + 1));
            sortie.push(Rencontre { place0, place1, graine_partie: carte, graine_ia });
        }
    }
    sortie
}

/// Toutes les paires d'une liste de `n` profils (chacun contre chacun).
pub fn toutes_les_paires(n: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for a in 0..n {
        for b in a + 1..n {
            v.push((a, b));
        }
    }
    v
}

/// Joue toutes les rencontres sur `fils` fils d'exécution. L'ordre des résultats est
/// celui des rencontres, quel que soit le nombre de fils : le résultat est reproductible.
pub fn jouer_tout(profils: &[Profil], liste: &[Rencontre], reglages: Reglages, fils: usize) -> Vec<Issue> {
    let fils = fils.max(1);
    let part = liste.len().div_ceil(fils).max(1);
    std::thread::scope(|s| {
        let taches: Vec<_> = liste
            .chunks(part)
            .map(|morceau| s.spawn(move || morceau.iter().map(|&r| jouer(profils, r, reglages)).collect::<Vec<_>>()))
            .collect();
        taches.into_iter().flat_map(|t| t.join().expect("un fil a échoué")).collect()
    })
}

/// Bilan d'une paire, vu de `a` : victoires, égalités, défaites.
pub fn bilan(issues: &[Issue], a: usize, b: usize) -> (u32, u32, u32) {
    let mut v = (0, 0, 0);
    for i in issues {
        let r = i.rencontre;
        let place_a = if r.place0 == a && r.place1 == b {
            0
        } else if r.place0 == b && r.place1 == a {
            1
        } else {
            continue;
        };
        match i.demi_points(place_a) {
            2 => v.0 += 1,
            1 => v.1 += 1,
            _ => v.2 += 1,
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deux() -> Vec<Profil> {
        vec![Profil::hasard("hasard"), Profil::utilite("utilite", [100, -50, 5, 3])]
    }

    #[test]
    fn meme_graine_meme_tournoi_quel_que_soit_le_nombre_de_fils() {
        let p = deux();
        let l = rencontres(&[(0, 1)], 20, 5);
        let a = jouer_tout(&p, &l, Reglages::default(), 1);
        let b = jouer_tout(&p, &l, Reglages::default(), 3);
        assert_eq!(a, b);
        let c = jouer_tout(&p, &rencontres(&[(0, 1)], 20, 6), Reglages::default(), 2);
        assert_ne!(a, c);
    }

    #[test]
    fn chaque_carte_est_jouee_des_deux_cotes() {
        let l = rencontres(&[(0, 1)], 4, 9);
        assert_eq!(l[0].graine_partie, l[1].graine_partie);
        assert_eq!((l[0].place0, l[1].place0), (0, 1));
        assert_ne!(l[0].graine_partie, l[2].graine_partie);
    }

    #[test]
    fn l_ia_a_utilite_bat_nettement_le_hasard() {
        let p = deux();
        let issues = jouer_tout(&p, &rencontres(&[(0, 1)], 200, 1), Reglages::default(), 4);
        assert!(issues.iter().all(|i| i.forfait.is_none()), "une IA a joué une action refusée");
        let (v, e, d) = bilan(&issues, 1, 0);
        // Score provisoire des règles 2 (cases découvertes) : les deux camps finissent souvent
        // par tout découvrir, d'où beaucoup d'égalités. On compte donc les points.
        assert!(2 * v + e >= 2 * 200 * 7 / 10 && d <= 10, "utilité contre hasard : {v} victoires, {e} égalités, {d} défaites");
    }

    #[test]
    fn chaque_partie_se_rejoue() {
        let issues = jouer_tout(&deux(), &rencontres(&[(0, 1)], 10, 3), Reglages::default(), 2);
        for i in &issues {
            let fin = i.replay.rejouer().expect("replay valide");
            assert_eq!(fin.scores(), i.scores);
            assert_eq!(fin.gagnant(), i.gagnant);
        }
    }
}
