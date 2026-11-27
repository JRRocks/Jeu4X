//! Pré-entraînement par évolution des poids (IA-003), version courte et gratuite (D-24).
//!
//! Stratégie (1 + λ) : à chaque génération, `enfants` copies mutées du parent jouent
//! `parties` parties contre les adversaires ; le meilleur enfant remplace le parent s'il
//! fait mieux que lui sur les mêmes cartes. Les cartes changent à chaque génération pour
//! ne pas apprendre un jeu de cartes par cœur. Entièrement reproductible par graine.
//!
//! Le résultat n'entre pas tout seul dans la galerie : il devient une candidate, que
//! `valider` confronte sur d'autres cartes (IA-004).

use crate::criteres::NB_CRITERES;
use crate::melanger;
use crate::profil::{Profil, Sorte};
use crate::tournoi::{bilan, jouer_tout, rencontres};
use regles::hasard::Hasard;
use regles::partie::Reglages;

#[derive(Clone, Debug)]
pub struct Reglage {
    pub generations: u32,
    pub enfants: u32,
    pub parties: u32,
    pub graine: u64,
    pub fils: usize,
}

/// Part des points (pour mille) de `profil` contre chacun des `adversaires`, en moyenne.
pub fn evaluer(profil: &Profil, adversaires: &[Profil], parties: u32, graine: u64, reglages: Reglages, fils: usize) -> u32 {
    let mut profils = vec![profil.clone()];
    profils.extend(adversaires.iter().cloned());
    let paires: Vec<(usize, usize)> = (1..profils.len()).map(|b| (0, b)).collect();
    let issues = jouer_tout(&profils, &rencontres(&paires, parties, graine), reglages, fils);
    let mut total = 0;
    for b in 1..profils.len() {
        let (v, e, d) = bilan(&issues, 0, b);
        total += (2 * v + e) * 1000 / (2 * (v + e + d)).max(1);
    }
    total / adversaires.len().max(1) as u32
}

fn muter(poids: &[i32; NB_CRITERES], h: &mut Hasard) -> [i32; NB_CRITERES] {
    let mut p = *poids;
    // Chaque poids a une chance sur trois de bouger : de −30 % à +30 %, ou de ±1 s'il est petit.
    for w in p.iter_mut() {
        if h.sous(3) == 0 {
            let pas = (w.abs() * 3 / 10).max(1);
            let delta = h.sous(2 * pas as u32 + 1) as i32 - pas;
            *w += if delta == 0 { 1 } else { delta };
        }
    }
    p
}

/// Fait évoluer `depart` ; rend le meilleur profil et le journal des générations.
pub fn evoluer(depart: &Profil, adversaires: &[Profil], r: &Reglage, reglages: Reglages, nom: &str) -> (Profil, Vec<String>) {
    let Sorte::Utilite { poids } = depart.sorte.clone() else {
        return (depart.clone(), vec!["seule une IA à utilité peut évoluer".into()]);
    };
    let mut parent = poids;
    let mut h = Hasard::new(melanger(r.graine));
    let mut journal = Vec::new();
    for g in 0..r.generations {
        let cartes = melanger(r.graine ^ melanger(1000 + g as u64));
        let note_parent = evaluer(&Profil::utilite(nom, parent), adversaires, r.parties, cartes, reglages, r.fils);
        let mut meilleur = (note_parent, parent);
        for _ in 0..r.enfants {
            let enfant = muter(&parent, &mut h);
            let note = evaluer(&Profil::utilite(nom, enfant), adversaires, r.parties, cartes, reglages, r.fils);
            if note > meilleur.0 {
                meilleur = (note, enfant);
            }
        }
        let change = meilleur.1 != parent;
        parent = meilleur.1;
        journal.push(format!(
            "génération {:>3} : parent {:.1} %, meilleur {:.1} %{}",
            g + 1,
            note_parent as f64 / 10.0,
            meilleur.0 as f64 / 10.0,
            if change { " (remplacé)" } else { "" }
        ));
    }
    (Profil::utilite(nom, parent), journal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::criteres::POIDS_ESSAI;

    #[test]
    fn une_evolution_courte_est_reproductible() {
        let r = Reglage { generations: 2, enfants: 2, parties: 4, graine: 3, fils: 2 };
        let adv = vec![Profil::hasard("h")];
        let depart = Profil::utilite("d", POIDS_ESSAI);
        let (a, _) = evoluer(&depart, &adv, &r, Reglages::default(), "x");
        let (b, _) = evoluer(&depart, &adv, &r, Reglages::default(), "x");
        assert_eq!(a, b);
    }
}
