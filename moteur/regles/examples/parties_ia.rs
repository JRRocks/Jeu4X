//! Parties entre adversaires simples : durée et façon de finir (réglage des chiffres).
//! `cargo run --release --example parties_ia -p regles`
use regles::ia_simple::jouer_tour;
use regles::partie::{Partie, Reglages};

fn main() {
    for nb in 2..=4u8 {
        let (mut capitale, mut seuil, mut nulles, mut tours) = (0, 0, 0, 0u32);
        let n = 40;
        for graine in 0..n {
            let mut p = Partie::nouvelle(Reglages::pour(nb), graine);
            while !p.est_finie() {
                for a in jouer_tour(&p) {
                    p.appliquer(a).unwrap();
                }
            }
            tours += p.tour() as u32;
            match p.gagnant() {
                None => nulles += 1,
                Some(g) if p.scores()[g as usize] >= p.reglages().seuil() => seuil += 1,
                Some(_) => capitale += 1,
            }
        }
        println!("{nb} joueurs : {capitale} par capitale, {seuil} au seuil, {nulles} nulles, {:.1} tours en moyenne", tours as f64 / n as f64);
    }
}
