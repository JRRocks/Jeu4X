//! Critères mesurés par l'IA à utilité, du point de vue d'un joueur.
//!
//! L'IA ne lit que ce que son joueur sait (`Partie::vue`) : brouillard respecté. Les
//! règles changent par étapes (S1) : à chaque changement, on revoit les critères ici et
//! on monte `VERSION_CRITERES` ; les profils d'une autre version sont refusés par `profil`.
//!
//! Version 3 (règles 3, V1 complète) : victoire, score D-28, économie, armée, villes,
//! approche des buts, exploration, unités prêtes à rallier.

use regles::partie::{Partie, Visibilite};
use regles::plateau::Case;

/// Version de la liste des critères (inscrite dans chaque profil : `utilite/<version>`).
pub const VERSION_CRITERES: u32 = 3;

/// Nombre de critères, donc de poids d'un profil à utilité.
pub const NB_CRITERES: usize = 11;

/// Noms des critères, dans l'ordre des poids (pour les rapports).
pub const NOMS: [&str; NB_CRITERES] = [
    "issue",
    "mon score",
    "meilleur score adverse",
    "vivres",
    "revenu",
    "mon armée",
    "armée adverse vue",
    "mes villes",
    "approche",
    "exploration",
    "prêtes à rallier",
];

/// Poids d'essai pour les tests (ceux de la première IA à utilité des règles 3).
pub const POIDS_ESSAI: [i32; NB_CRITERES] = [100_000, 100, -60, 2, 15, 8, -8, 150, 6, 3, 120];

/// Poids absurdes pour les tests : l'IA fait tout à l'envers.
pub const POIDS_RECUL: [i32; NB_CRITERES] = [-100_000, -100, 60, -2, -15, -8, 8, -150, -6, -3, -120];

/// Mesure les critères de `partie` pour `joueur`. Entiers seulement.
pub fn mesurer(partie: &Partie, joueur: u8) -> [i32; NB_CRITERES] {
    let vue = partie.vue(joueur);
    let plateau = partie.plateau();
    let moi = joueur as usize;
    let issue = match vue.gagnant {
        Some(g) if g == joueur => 1,
        Some(_) => -1,
        None => 0,
    };
    let adverse = vue.scores.iter().enumerate().filter(|&(j, _)| j != moi).map(|(_, &s)| s).max().unwrap_or(0);
    let mut armee = 0;
    let mut armee_adverse = 0;
    for u in &vue.unites {
        if u.joueur == joueur {
            armee += u.pv;
        } else {
            armee_adverse += u.pv;
        }
    }
    let mes_villes = vue.villes.iter().filter(|v| v.joueur == Some(joueur)).count() as i32;
    let miennes: Vec<Case> = vue.unites.iter().filter(|u| u.joueur == joueur).map(|u| u.case).collect();
    // Buts : villes et villages qui ne sont pas à moi, ruines, unités adverses vues.
    let mut buts: Vec<Case> = vue.villes.iter().filter(|v| v.joueur != Some(joueur)).map(|v| v.case).collect();
    buts.extend(vue.ruines.iter().copied());
    buts.extend(vue.unites.iter().filter(|u| u.joueur != joueur).map(|u| u.case));
    let pretes = vue.villes.iter().filter(|v| v.joueur != Some(joueur) && miennes.contains(&v.case)).count() as i32;
    // Frontière du connu : cases inconnues voisines d'une case connue.
    let mut frontiere = Vec::new();
    let mut v = [0; 8];
    for c in 0..vue.cases.len() as Case {
        if vue.cases[c as usize] == Visibilite::Inconnue {
            let n = plateau.voisines(c, &mut v);
            if v[..n].iter().any(|&w| vue.cases[w as usize] != Visibilite::Inconnue) {
                frontiere.push(c);
            }
        }
    }
    let mut approche = 0;
    let mut exploration = 0;
    for &c in &miennes {
        if let Some(d) = buts.iter().map(|&b| plateau.distance(c, b)).min() {
            approche -= d as i32;
        }
        if let Some(d) = frontiere.iter().map(|&b| plateau.distance(c, b)).min() {
            exploration -= d as i32;
        }
    }
    [issue, vue.scores[moi], adverse, vue.vivres, vue.revenu, armee, armee_adverse, mes_villes, approche, exploration, pretes]
}

/// Note d'un état : somme pondérée des critères.
pub fn noter(partie: &Partie, joueur: u8, poids: &[i32; NB_CRITERES]) -> i64 {
    mesurer(partie, joueur).iter().zip(poids).map(|(&c, &p)| c as i64 * p as i64).sum()
}
