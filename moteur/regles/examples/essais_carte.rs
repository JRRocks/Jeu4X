//! Mesure : combien d'essais de carte faut-il pour des départs équitables (§ 2.3) ?
//! `cargo run --release --example essais_carte -p regles`
use regles::partie::{essais_carte, Reglages};

fn main() {
    for nb in 2..=4u8 {
        let r = Reglages::pour(nb);
        let n = 2000u64;
        let essais: Vec<u64> = (0..n).map(|g| essais_carte(r, g)).collect();
        let max = essais.iter().max().unwrap();
        let moyenne = essais.iter().sum::<u64>() as f64 / n as f64;
        println!("{nb} joueurs : {moyenne:.1} essais en moyenne, {max} au plus, sur {n} graines");
    }
}
