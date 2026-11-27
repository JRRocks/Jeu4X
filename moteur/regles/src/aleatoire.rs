//! Joueur au hasard : sert aux tests, aux mesures de vitesse et de base aux IA.

use crate::hasard::Hasard;
use crate::partie::{Partie, Reglages};

/// Joue une partie entière au hasard. Rend l'état final et la suite des numéros d'actions.
/// `graine_joueur` est séparée de la graine de la partie : même carte, autres coups.
pub fn jouer_au_hasard(reglages: Reglages, graine_partie: u64, graine_joueur: u64) -> (Partie, Vec<u32>) {
    let mut p = Partie::nouvelle(reglages, graine_partie);
    let mut h = Hasard::new(graine_joueur);
    let mut actions = Vec::new();
    while !p.est_finie() {
        let permises = p.actions_permises();
        // La fin du tour est toujours permise ; on la tire moins souvent pour que les unités bougent.
        let choix = if permises.len() > 1 && h.sous(4) != 0 {
            permises[1 + h.sous(permises.len() as u32 - 1) as usize]
        } else {
            permises[0]
        };
        actions.push(choix.numero(reglages.taille));
        p.appliquer(choix).expect("une action permise est toujours acceptée");
    }
    (p, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn une_partie_au_hasard_se_termine_et_se_reproduit() {
        let (a, actions_a) = jouer_au_hasard(Reglages::default(), 10, 20);
        let (b, actions_b) = jouer_au_hasard(Reglages::default(), 10, 20);
        assert!(a.est_finie());
        assert_eq!(a.empreinte(), b.empreinte());
        assert_eq!(actions_a, actions_b);
        let (c, _) = jouer_au_hasard(Reglages::default(), 10, 21);
        assert_ne!(a.empreinte(), c.empreinte());
    }

    #[test]
    fn la_copie_est_independante() {
        let p = Partie::nouvelle(Reglages::default(), 4);
        let mut q = p.clone();
        let a = q.actions_permises()[1];
        q.appliquer(a).unwrap();
        assert_ne!(p.empreinte(), q.empreinte());
        assert_eq!(p, Partie::nouvelle(Reglages::default(), 4));
    }

    #[test]
    fn toutes_les_tailles_de_carte_jouent() {
        for taille in [6u8, 11, 14, 16] {
            let (p, _) = jouer_au_hasard(Reglages { taille, ..Reglages::default() }, 3, 3);
            assert!(p.est_finie());
        }
    }
}
