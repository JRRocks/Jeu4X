//! L'adversaire tenu par l'ordinateur, tel que l'application l'appelle (D-34).
//!
//! Une seule fonction à brancher : [`jouer_tour`]. Elle reçoit l'état de la partie au
//! début (ou au milieu) du tour de l'IA et rend la suite des actions de son tour, la fin du
//! tour comprise. Elle ne modifie pas l'état reçu : l'application applique ensuite les
//! actions une à une, ce qui lui permet de les montrer.
//!
//! Garanties :
//! - chaque action rendue est permise au moment où elle sera appliquée, dans l'ordre ;
//! - la dernière est `Action::FinDuTour` (sauf si la partie se termine avant) ;
//! - même état, même niveau : mêmes actions (aucune horloge, aucun hasard extérieur) ;
//! - le nombre d'actions d'un tour est borné ([`ACTIONS_PAR_TOUR_MAX`]).

use crate::joueur::Joueur;
use crate::profil::{lire_liste, Profil};
use regles::partie::{Action, Partie};

/// Garde-fou : au-delà, l'IA finit son tour.
pub const ACTIONS_PAR_TOUR_MAX: usize = 200;

/// Niveau de l'adversaire.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niveau {
    /// Joue au hasard (pour débuter, ou pour tester l'écran).
    Facile,
    /// L'IA en service de la galerie (`moteur/ia/galerie.txt`).
    Normal,
}

impl Niveau {
    /// Niveau depuis un entier (0 facile, 1 et plus normal), pratique pour le pont JNI.
    pub fn depuis_u8(n: u8) -> Niveau {
        if n == 0 {
            Niveau::Facile
        } else {
            Niveau::Normal
        }
    }
}

/// L'IA en service : la dernière de la galerie faite pour les règles en cours.
/// La galerie est incluse dans la bibliothèque à la compilation.
pub fn ia_en_service() -> Profil {
    lire_liste(include_str!("../galerie.txt"))
        .ok()
        .and_then(|g| g.into_iter().rev().find(|p| p.a_jour()))
        .unwrap_or_else(|| Profil::hasard("hasard-secours"))
}

fn profil(niveau: Niveau) -> Profil {
    match niveau {
        Niveau::Facile => Profil::hasard("facile"),
        Niveau::Normal => ia_en_service(),
    }
}

/// Les actions du tour du joueur courant, fin du tour comprise.
pub fn jouer_tour(partie: &Partie, niveau: Niveau) -> Vec<Action> {
    // Graine du joueur tirée de l'état : reproductible, et différente à chaque tour.
    let graine = partie.empreinte();
    let mut ia: Box<dyn Joueur> = profil(niveau).fabriquer(graine);
    let mut copie = partie.clone();
    let joueur = copie.joueur_courant();
    let mut actions = Vec::new();
    while !copie.est_finie() && copie.joueur_courant() == joueur {
        let mut a = ia.choisir(&copie);
        if actions.len() + 1 >= ACTIONS_PAR_TOUR_MAX {
            a = Action::FinDuTour;
        }
        if copie.appliquer(a).is_err() {
            // Ne devrait jamais arriver ; on finit le tour plutôt que de bloquer la partie.
            a = Action::FinDuTour;
            if copie.appliquer(a).is_err() {
                break;
            }
        }
        actions.push(a);
        if a == Action::FinDuTour {
            break;
        }
    }
    actions
}

/// Même chose en numéros d'actions (forme la plus simple à passer par le pont JNI).
pub fn jouer_tour_numeros(partie: &Partie, niveau: Niveau) -> Vec<u32> {
    let taille = partie.reglages().taille;
    jouer_tour(partie, niveau).into_iter().map(|a| a.numero(taille)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regles::partie::Reglages;

    /// Joue des parties entières IA contre IA par `jouer_tour`, sans erreur.
    fn partie_complete(graine: u64, niveaux: [Niveau; 2]) -> Partie {
        let mut p = Partie::nouvelle(Reglages::default(), graine);
        let mut tours = 0;
        while !p.est_finie() {
            let qui = p.joueur_courant();
            let actions = jouer_tour(&p, niveaux[qui as usize]);
            assert!(!actions.is_empty());
            for a in actions {
                p.appliquer(a).expect("action rendue par l'IA refusée");
            }
            assert!(p.est_finie() || p.joueur_courant() != qui, "le tour n'a pas été rendu");
            tours += 1;
            assert!(tours < 10_000);
        }
        p
    }

    #[test]
    fn des_parties_ia_contre_ia_vont_au_bout() {
        for g in 0..20 {
            partie_complete(g, [Niveau::Normal, Niveau::Normal]);
            partie_complete(g, [Niveau::Facile, Niveau::Normal]);
            partie_complete(g, [Niveau::Normal, Niveau::Facile]);
        }
    }

    #[test]
    fn meme_etat_memes_actions_et_l_etat_recu_reste_intact() {
        let p = Partie::nouvelle(Reglages::default(), 5);
        let avant = p.empreinte();
        assert_eq!(jouer_tour(&p, Niveau::Normal), jouer_tour(&p, Niveau::Normal));
        assert_eq!(jouer_tour(&p, Niveau::Facile), jouer_tour(&p, Niveau::Facile));
        assert_eq!(p.empreinte(), avant);
    }

    #[test]
    fn le_niveau_normal_bat_le_niveau_facile() {
        let mut gagnees = 0;
        for g in 0..40 {
            // Le niveau normal joue tantôt en premier, tantôt en second.
            let (niveaux, place) = if g % 2 == 0 {
                ([Niveau::Normal, Niveau::Facile], 0)
            } else {
                ([Niveau::Facile, Niveau::Normal], 1)
            };
            if partie_complete(g, niveaux).gagnant() == Some(place) {
                gagnees += 1;
            }
        }
        assert!(gagnees >= 32, "{gagnees} victoires sur 40");
    }

    #[test]
    fn l_ia_en_service_vient_de_la_galerie() {
        assert_ne!(ia_en_service().nom, "hasard-secours");
    }
}
