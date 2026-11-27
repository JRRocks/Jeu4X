//! Classement Elo des IA (IA-001).
//!
//! Plutôt qu'un Elo mis à jour partie après partie (qui dépend de l'ordre des parties),
//! on ajuste d'un coup le modèle de Bradley-Terry, dont l'Elo est l'échelle : sur un
//! écart de 400 points, le plus fort marque 10 fois plus de points que l'autre. Une
//! demi-victoire fictive dans chaque sens, par paire, évite un classement infini quand
//! une IA gagne tout. Le premier profil sert d'ancre à 1000 points, pour que les
//! classements de tournois différents se comparent.

/// `demi[a][b]` : demi-points marqués par `a` contre `b` (victoire 2, égalité 1).
pub fn classer(demi: &[Vec<u32>]) -> Vec<f64> {
    let n = demi.len();
    if n == 0 {
        return Vec::new();
    }
    // Points (en parties) et nombre de parties, avec l'a priori.
    let mut w = vec![vec![0.0f64; n]; n];
    let mut joue = vec![vec![false; n]; n];
    for a in 0..n {
        for b in 0..n {
            if a != b && demi[a][b] + demi[b][a] > 0 {
                w[a][b] = demi[a][b] as f64 / 2.0 + 0.5;
                joue[a][b] = true;
            }
        }
    }
    let mut force = vec![1.0f64; n];
    // Algorithme de minorisation-majoration de Hunter (2004), nombre d'itérations fixe.
    for _ in 0..2000 {
        let mut nouvelle = force.clone();
        for a in 0..n {
            let gains: f64 = (0..n).filter(|&b| joue[a][b]).map(|b| w[a][b]).sum();
            let denom: f64 =
                (0..n).filter(|&b| joue[a][b]).map(|b| (w[a][b] + w[b][a]) / (force[a] + force[b])).sum();
            if denom > 0.0 {
                nouvelle[a] = gains / denom;
            }
        }
        let ancre = nouvelle[0];
        force = nouvelle.iter().map(|f| f / ancre).collect();
    }
    force.iter().map(|f| 1000.0 + 400.0 * f.log10()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_plus_fort_est_devant() {
        // b gagne 80 % contre a, c gagne 80 % contre b.
        let demi = vec![vec![0, 40, 10], vec![160, 0, 40], vec![190, 160, 0]];
        let e = classer(&demi);
        assert!((e[0] - 1000.0).abs() < 1e-9);
        assert!(e[1] > e[0] + 150.0 && e[2] > e[1] + 150.0, "{e:?}");
    }

    #[test]
    fn egalite_parfaite() {
        let demi = vec![vec![0, 50], vec![50, 0]];
        let e = classer(&demi);
        assert!((e[1] - 1000.0).abs() < 1e-6);
    }

    #[test]
    fn tout_gagner_reste_fini() {
        let e = classer(&[vec![0, 0], vec![200, 0]]);
        assert!(e[1].is_finite() && e[1] > 1500.0);
    }
}
