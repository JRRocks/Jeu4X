//! Critères mesurés par l'IA à utilité, du point de vue d'un joueur.
//!
//! Les règles actuelles n'ont que des déplacements et un score provisoire (unités dans la
//! moitié adverse). Quand les règles de la première version arriveront (villes, combats,
//! monnaie, victoire par capitale ou par points, D-23, D-28), on ajoute des critères ici
//! et on monte `VERSION_CRITERES` : les profils écrits pour l'ancienne version sont alors
//! refusés par `profil`, et la galerie se requalifie.

use regles::partie::Partie;

/// Version de la liste des critères (inscrite dans chaque profil : `utilite/<version>`).
pub const VERSION_CRITERES: u32 = 1;

/// Nombre de critères, donc de poids d'un profil à utilité.
pub const NB_CRITERES: usize = 4;

/// Noms des critères, dans l'ordre des poids (pour les rapports).
pub const NOMS: [&str; NB_CRITERES] = ["mon score", "score adverse", "mon avancée", "avancée adverse"];

/// Distance (en points de mouvement, selon le terrain, sans compter les unités) de chaque
/// case à la moitié adverse de `joueur`. `u16::MAX` : la moitié adverse est hors d'atteinte.
pub fn distances_vers_moitie_adverse(partie: &Partie, joueur: u8) -> Vec<u16> {
    let plateau = partie.plateau();
    let t = plateau.taille();
    let n = plateau.nb_cases();
    let mut dist = vec![u16::MAX; n];
    // Coûts de 1 ou 2 : Dijkstra par seaux, cases d'arrivée à distance 0.
    let mut seaux: Vec<Vec<u16>> = vec![Vec::new(); 2 * n + 3];
    for c in 0..n as u16 {
        let y = plateau.rangee(c);
        let dans = if joueur == 0 { y >= t / 2 + t % 2 } else { y < t / 2 };
        if dans && plateau.terrain(c).cout().is_some() {
            dist[c as usize] = 0;
            seaux[0].push(c);
        }
    }
    let mut v = [0; 4];
    for d in 0..seaux.len() {
        let mut k = 0;
        while k < seaux[d].len() {
            let w = seaux[d][k];
            k += 1;
            if dist[w as usize] as usize != d {
                continue;
            }
            // Aller de c à w coûte le terrain de w.
            let Some(pas) = plateau.terrain(w).cout() else { continue };
            let nv = plateau.voisines(w, &mut v);
            for &c in &v[..nv] {
                let nd = d + pas as usize;
                if plateau.terrain(c).cout().is_some() && nd < seaux.len() && (nd as u16) < dist[c as usize] {
                    dist[c as usize] = nd as u16;
                    seaux[nd].push(c);
                }
            }
        }
    }
    dist
}

/// Mesure les critères de `partie` pour `joueur`. Entiers seulement.
/// L'avancée d'une unité = `borne − distance restante` jusqu'à la moitié adverse, en
/// suivant le terrain : contourner l'eau compte comme un progrès, pas un recul.
pub fn mesurer(partie: &Partie, joueur: u8) -> [i32; NB_CRITERES] {
    mesurer_avec(partie, joueur, &cartes(partie))
}

/// Cartes de distances des deux joueurs. Elles ne dépendent que du terrain : on les
/// calcule une fois par décision, pas une fois par action essayée.
pub fn cartes(partie: &Partie) -> [Vec<u16>; 2] {
    [distances_vers_moitie_adverse(partie, 0), distances_vers_moitie_adverse(partie, 1)]
}

/// Comme [`mesurer`], avec des cartes de distances déjà calculées pour ce terrain.
pub fn mesurer_avec(partie: &Partie, joueur: u8, cartes: &[Vec<u16>; 2]) -> [i32; NB_CRITERES] {
    let scores = partie.scores();
    let borne = 4 * partie.reglages().taille as i32;
    let mut avancee = [0i32; 2];
    for u in partie.unites() {
        let d = cartes[u.joueur as usize][u.case as usize];
        let reste = if d == u16::MAX { borne } else { (d as i32).min(borne) };
        avancee[u.joueur as usize] += borne - reste;
    }
    let moi = joueur as usize;
    let lui = 1 - moi;
    [scores[moi], scores[lui], avancee[moi], avancee[lui]]
}

/// Note d'un état : somme pondérée des critères.
pub fn noter(partie: &Partie, joueur: u8, poids: &[i32; NB_CRITERES], cartes: &[Vec<u16>; 2]) -> i64 {
    mesurer_avec(partie, joueur, cartes).iter().zip(poids).map(|(&c, &p)| c as i64 * p as i64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regles::partie::Reglages;

    #[test]
    fn distance_nulle_dans_la_moitie_adverse_et_croissante_ailleurs() {
        let p = Partie::nouvelle(Reglages::default(), 3);
        let d = distances_vers_moitie_adverse(&p, 0);
        let pl = p.plateau();
        for c in 0..pl.nb_cases() as u16 {
            if pl.rangee(c) >= 6 && pl.terrain(c).cout().is_some() {
                assert_eq!(d[c as usize], 0);
            }
        }
        // Les unités du joueur 0 partent du bas : loin, mais atteignables sur cette carte.
        for u in p.unites().iter().filter(|u| u.joueur == 0) {
            assert!(d[u.case as usize] >= 6);
        }
    }
}
