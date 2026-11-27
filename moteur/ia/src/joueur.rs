//! Joueurs IA.

use crate::criteres::{noter, NB_CRITERES};
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
        let mut note_max = noter(partie, moi, &self.poids);
        for a in partie.actions_permises() {
            if a == Action::FinDuTour {
                continue;
            }
            let mut essai = partie.clone();
            if essai.appliquer(a).is_err() {
                continue;
            }
            let note = noter(&essai, moi, &self.poids);
            if note > note_max {
                note_max = note;
                meilleure = a;
            }
        }
        meilleure
    }
}

/// L'adversaire provisoire des règles (`regles::ia_simple`, S1), comme référence à battre.
/// Il calcule tout son tour d'un coup ; on le rend action par action.
pub struct JoueurSimple {
    file: std::collections::VecDeque<Action>,
}

impl JoueurSimple {
    pub fn new() -> Self {
        JoueurSimple { file: Default::default() }
    }
}

impl Default for JoueurSimple {
    fn default() -> Self {
        Self::new()
    }
}

impl Joueur for JoueurSimple {
    fn choisir(&mut self, partie: &Partie) -> Action {
        if self.file.is_empty() {
            self.file = regles::ia_simple::jouer_tour(partie).into();
        }
        self.file.pop_front().unwrap_or(Action::FinDuTour)
    }
}
