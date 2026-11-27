//! Joueurs IA.

use crate::criteres::{cartes, noter, NB_CRITERES};
use regles::hasard::Hasard;
use regles::partie::{Action, Partie};

/// Un joueur IA : reçoit l'état, rend une action permise.
pub trait Joueur: Send {
    fn choisir(&mut self, partie: &Partie) -> Action;
}

/// Joueur au hasard (référence la plus basse). Même politique que `regles::aleatoire` :
/// la fin du tour est tirée une fois sur quatre, pour que les unités bougent.
pub struct JoueurHasard {
    hasard: Hasard,
}

impl JoueurHasard {
    pub fn new(graine: u64) -> Self {
        JoueurHasard { hasard: Hasard::new(graine) }
    }
}

impl Joueur for JoueurHasard {
    fn choisir(&mut self, partie: &Partie) -> Action {
        let permises = partie.actions_permises();
        if permises.len() > 1 && self.hasard.sous(4) != 0 {
            permises[1 + self.hasard.sous(permises.len() as u32 - 1) as usize]
        } else {
            permises[0]
        }
    }
}

/// IA à utilité (IA-002) : essaie chaque action permise sur une copie de l'état, note
/// l'état obtenu avec ses poids, joue la meilleure ; finit son tour quand plus rien
/// n'améliore la note. Égalités : la première dans l'ordre fixe des actions permises,
/// donc entièrement reproductible.
pub struct JoueurUtilite {
    poids: [i32; NB_CRITERES],
}

impl JoueurUtilite {
    pub fn new(poids: [i32; NB_CRITERES]) -> Self {
        JoueurUtilite { poids }
    }
}

impl Joueur for JoueurUtilite {
    fn choisir(&mut self, partie: &Partie) -> Action {
        let moi = partie.joueur_courant();
        let mut meilleure = Action::FinDuTour;
        let cartes = cartes(partie);
        let mut note_max = noter(partie, moi, &self.poids, &cartes);
        for a in partie.actions_permises() {
            if a == Action::FinDuTour {
                continue;
            }
            let mut essai = partie.clone();
            if essai.appliquer(a).is_err() {
                continue;
            }
            let note = noter(&essai, moi, &self.poids, &cartes);
            if note > note_max {
                note_max = note;
                meilleure = a;
            }
        }
        meilleure
    }
}
