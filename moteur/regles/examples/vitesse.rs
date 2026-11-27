//! Mesure « parties aléatoires par seconde » sur un cœur (besoin E2 de la session 3).
//! `cargo run --release --example vitesse -p regles`
use regles::aleatoire::jouer_au_hasard;
use regles::partie::Reglages;
use std::time::Instant;

fn main() {
    for taille in [11u8, 14] {
        let r = Reglages { taille, ..Reglages::default() };
        let n = 2000u64;
        let debut = Instant::now();
        let mut actions = 0usize;
        let mut controle = 0u64;
        for i in 0..n {
            let (p, a) = jouer_au_hasard(r, i, i + 1);
            actions += a.len();
            controle ^= p.empreinte();
        }
        let s = debut.elapsed().as_secs_f64();
        println!(
            "{taille}×{taille} : {:.0} parties/s, {:.0} actions par partie, {:.2} µs par action (contrôle {controle:x})",
            n as f64 / s,
            actions as f64 / n as f64,
            s * 1e6 / actions as f64
        );
    }
}
