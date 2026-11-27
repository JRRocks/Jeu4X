//! Trace d'une partie entre adversaires simples (débogage).
use regles::ia_simple::jouer_tour;
use regles::partie::{Partie, Reglages};

fn main() {
    let mut p = Partie::nouvelle(Reglages::pour(2), 1);
    while !p.est_finie() {
        let j = p.joueur_courant();
        let acts = jouer_tour(&p);
        for a in &acts {
            p.appliquer(*a).unwrap();
        }
        let jo = p.joueurs()[j as usize];
        let villes: Vec<u8> = p.villes().iter().filter(|v| v.joueur == Some(j)).map(|v| v.niveau).collect();
        println!("tour {} j{} : score {} (acquis {}), vivres {}, villes {:?}, avancées {:b}, unités {}, {} actions", p.tour(), j, p.score(j), jo.points_acquis, jo.vivres, villes, jo.avancees, p.nb_unites(j), acts.len());
    }
}
