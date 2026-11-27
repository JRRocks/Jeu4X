//! Une nouvelle IA doit battre l'ancienne (IA-004).
//!
//! Une candidate entre dans la galerie seulement si :
//! 1. elle marque au moins `SEUIL_CHAMPION` des points contre l'IA en service (la
//!    dernière de la galerie faite pour les règles en cours) ;
//! 2. elle ne recule contre aucune autre version de la galerie : au moins
//!    `SEUIL_ANCIENNES` des points contre chacune (pas de cycle « A bat B, B bat C,
//!    C bat A »).
//!
//! Les versions faites pour d'autres règles sont écartées et signalées : elles devront être
//! requalifiées quand les règles changent.

use crate::profil::Profil;
use crate::tournoi::{bilan, jouer_tout, rencontres, Issue};
use regles::partie::Reglages;

/// Part des points exigée contre l'IA en service, en pour mille.
pub const SEUIL_CHAMPION: u32 = 550;
/// Part des points exigée contre chaque version plus ancienne, en pour mille.
pub const SEUIL_ANCIENNES: u32 = 500;

#[derive(Debug)]
pub struct Confrontation {
    pub adversaire: String,
    pub champion: bool,
    pub victoires: u32,
    pub egalites: u32,
    pub defaites: u32,
    /// Part des points de la candidate, en pour mille.
    pub pour_mille: u32,
    pub reussie: bool,
}

#[derive(Debug)]
pub struct Verdict {
    pub candidate: String,
    pub confrontations: Vec<Confrontation>,
    pub ecartees: Vec<String>,
    pub admise: bool,
    pub issues: Vec<Issue>,
}

/// Confronte `candidate` à la galerie. `parties` par adversaire (paire, de préférence).
pub fn valider(candidate: &Profil, galerie: &[Profil], parties: u32, graine: u64, reglages: Reglages, fils: usize) -> Verdict {
    let ecartees: Vec<String> = galerie.iter().filter(|p| !p.a_jour()).map(|p| p.nom.clone()).collect();
    let valables: Vec<Profil> = galerie.iter().filter(|p| p.a_jour() && p.nom != candidate.nom).cloned().collect();
    // Indice 0 : la candidate ; 1.. : la galerie valable, la dernière étant en service.
    let mut profils = vec![candidate.clone()];
    profils.extend(valables.iter().cloned());
    let paires: Vec<(usize, usize)> = (1..profils.len()).map(|b| (0, b)).collect();
    let issues = jouer_tout(&profils, &rencontres(&paires, parties, graine), reglages, fils);
    let mut confrontations = Vec::new();
    for b in 1..profils.len() {
        let (v, e, d) = bilan(&issues, 0, b);
        let total = v + e + d;
        let pour_mille = if total == 0 { 0 } else { (2 * v + e) * 1000 / (2 * total) };
        let champion = b == profils.len() - 1;
        let seuil = if champion { SEUIL_CHAMPION } else { SEUIL_ANCIENNES };
        confrontations.push(Confrontation {
            adversaire: profils[b].nom.clone(),
            champion,
            victoires: v,
            egalites: e,
            defaites: d,
            pour_mille,
            reussie: total > 0 && pour_mille >= seuil,
        });
    }
    let admise = !confrontations.is_empty() && confrontations.iter().all(|c| c.reussie);
    Verdict { candidate: candidate.nom.clone(), confrontations, ecartees, admise, issues }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn galerie() -> Vec<Profil> {
        vec![Profil::hasard("hasard-v1")]
    }

    #[test]
    fn l_ia_a_utilite_est_admise_contre_le_hasard() {
        let v = valider(&Profil::utilite("u1", [100, -100, 10, 0]), &galerie(), 100, 1, Reglages::default(), 4);
        assert!(v.admise, "{:?}", v.confrontations);
    }

    #[test]
    fn le_hasard_ne_detrone_pas_l_ia_a_utilite() {
        let g = vec![Profil::hasard("hasard-v1"), Profil::utilite("u1", [100, -100, 10, 0])];
        let v = valider(&Profil::hasard("hasard-v2"), &g, 100, 1, Reglages::default(), 4);
        assert!(!v.admise);
        assert!(v.confrontations.last().unwrap().champion);
    }

    #[test]
    fn une_ia_qui_recule_est_refusee() {
        let g = vec![Profil::hasard("hasard-v1"), Profil::utilite("u1", [100, -100, 10, 0])];
        let v = valider(&Profil::utilite("recul", [100, -100, -10, 0]), &g, 100, 1, Reglages::default(), 4);
        assert!(!v.admise);
    }

    #[test]
    fn une_version_d_autres_regles_est_ecartee() {
        let mut vieille = Profil::hasard("vieille");
        vieille.version_regles = 0;
        let g = vec![vieille, Profil::hasard("hasard-v1")];
        let v = valider(&Profil::utilite("u1", [100, -100, 10, 0]), &g, 20, 1, Reglages::default(), 2);
        assert_eq!(v.ecartees, vec!["vieille".to_string()]);
        assert_eq!(v.confrontations.len(), 1);
    }
}
