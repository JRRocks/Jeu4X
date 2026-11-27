//! Critères mesurés par l'IA à utilité, du point de vue d'un joueur.
//!
//! L'IA ne lit que ce que son joueur sait (`Partie::vue`) et les scores, visibles de tous
//! (règles V1). Les règles arrivent par étapes (S1) : à chaque étape, on ajoute ou on
//! change des critères ici et on monte `VERSION_CRITERES` ; les profils écrits pour une
//! autre version des critères sont refusés par `profil`.
//!
//! Version 2 (règles 2, étape a : brouillard, capitales, déplacements en 8 directions) :
//! score (cases découvertes, provisoire), meilleur score adverse, exploration, menace sur
//! les villes adverses connues.

use regles::partie::{Partie, Visibilite};

/// Version de la liste des critères (inscrite dans chaque profil : `utilite/<version>`).
pub const VERSION_CRITERES: u32 = 2;

/// Nombre de critères, donc de poids d'un profil à utilité.
pub const NB_CRITERES: usize = 4;

/// Noms des critères, dans l'ordre des poids (pour les rapports).
pub const NOMS: [&str; NB_CRITERES] = ["mon score", "meilleur score adverse", "exploration", "menace"];

/// Mesure les critères de `partie` pour `joueur`. Entiers seulement.
pub fn mesurer(partie: &Partie, joueur: u8) -> [i32; NB_CRITERES] {
    let scores = partie.scores();
    let mon_score = scores[joueur as usize];
    let adverse = scores.iter().enumerate().filter(|&(j, _)| j != joueur as usize).map(|(_, &s)| s).max().unwrap_or(0);
    let vue = partie.vue(joueur);
    let plateau = partie.plateau();
    let inconnues: Vec<u16> = (0..vue.cases.len() as u16).filter(|&c| vue.cases[c as usize] == Visibilite::Inconnue).collect();
    let cibles: Vec<u16> = vue.villes.iter().filter(|v| v.joueur.is_some_and(|j| j != joueur)).map(|v| v.case).collect();
    let mut exploration = 0i32;
    let mut menace = 0i32;
    for u in vue.unites.iter().filter(|u| u.joueur == joueur) {
        // Plus l'unité est près d'une case inconnue (d'une ville adverse), mieux c'est.
        if let Some(d) = inconnues.iter().map(|&c| plateau.distance(u.case, c)).min() {
            exploration -= d as i32;
        }
        if let Some(d) = cibles.iter().map(|&c| plateau.distance(u.case, c)).min() {
            menace -= d as i32;
        }
    }
    [mon_score, adverse, exploration, menace]
}

/// Note d'un état : somme pondérée des critères.
pub fn noter(partie: &Partie, joueur: u8, poids: &[i32; NB_CRITERES]) -> i64 {
    mesurer(partie, joueur).iter().zip(poids).map(|(&c, &p)| c as i64 * p as i64).sum()
}
