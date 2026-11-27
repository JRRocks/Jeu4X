//! Joueur au hasard : sert aux tests, aux mesures de vitesse et de base aux IA.

use crate::hasard::Hasard;
use crate::partie::{Action, Partie, Reglages};

/// Joue une partie entière au hasard. Rend l'état final et la suite des numéros d'actions.
/// `graine_joueur` est séparée de la graine de la partie : même carte, autres coups.
pub fn jouer_au_hasard(reglages: Reglages, graine_partie: u64, graine_joueur: u64) -> (Partie, Vec<u32>) {
    let mut p = Partie::nouvelle(reglages, graine_partie);
    let mut h = Hasard::new(graine_joueur);
    let mut actions = Vec::new();
    while !p.est_finie() {
        // Renvoyer une unité est permis mais ne sert qu'à perdre : le joueur au hasard l'évite.
        let permises: Vec<Action> = p.actions_permises().into_iter().filter(|a| !matches!(a, Action::Renvoyer { .. })).collect();
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
    fn de_deux_a_quatre_joueurs_tout_joue() {
        for nb in 2..=4 {
            let (p, _) = jouer_au_hasard(Reglages::pour(nb), 3, 3);
            assert!(p.est_finie());
        }
        // Plateau plus grand que la V1 (grille interne de 16) : il joue aussi.
        let (p, _) = jouer_au_hasard(Reglages { taille: 16, ..Reglages::pour(4) }, 3, 3);
        assert!(p.est_finie());
    }
}
