//! Adversaire provisoire très simple (D-34) : il joue un tour entier, sans rien voir de
//! caché, en attendant l'IA de la session 3. Règles de priorité fixes, pas de hasard :
//! prendre ou rallier, attaquer quand c'est rentable, grandir, produire, exploiter,
//! rechercher, puis avancer chaque unité vers le but le plus proche.

use crate::partie::{Action, Partie, TypeUnite, Visibilite};
use crate::plateau::Case;

/// Actions de tout le tour du joueur courant, fin du tour comprise. N'altère pas `partie`.
pub fn jouer_tour(partie: &Partie) -> Vec<Action> {
    let mut p = partie.clone();
    let j = p.joueur_courant();
    let mut sortie = Vec::new();
    let mut deja_bouge: Vec<Case> = Vec::new();
    for _ in 0..120 {
        if p.est_finie() || p.joueur_courant() != j {
            break;
        }
        let a = choisir(&p, j, &deja_bouge);
        if let Action::Deplacer { arrivee, .. } = a {
            deja_bouge.push(arrivee);
        }
        if p.appliquer(a).is_err() {
            break;
        }
        sortie.push(a);
        if a == Action::FinDuTour {
            return sortie;
        }
    }
    if !p.est_finie() && p.joueur_courant() == j {
        sortie.push(Action::FinDuTour);
    }
    sortie
}

fn choisir(p: &Partie, j: u8, deja_bouge: &[Case]) -> Action {
    let permises = p.actions_permises();
    let vivres = p.vue(j).vivres;
    // 1. Prendre ou rallier.
    if let Some(&a) = permises.iter().find(|a| matches!(a, Action::Rallier { .. })) {
        return a;
    }
    // 2. Attaquer quand on fait plus de mal qu'on n'en reçoit, sans y laisser l'unité.
    let mut meilleure: Option<(i32, Action)> = None;
    for &a in &permises {
        if let Action::Attaquer { depart, cible } = a {
            if let Some(an) = p.annonce_combat(depart, cible) {
                let gain = an.degats - an.riposte + if an.cible_detruite { 10 } else { 0 };
                if !an.attaquant_detruit && an.degats >= an.riposte && meilleure.is_none_or(|(g, _)| gain > g) {
                    meilleure = Some((gain, a));
                }
            }
        }
    }
    if let Some((_, a)) = meilleure {
        return a;
    }
    // 3. Économie : grandir (cadeau a), produire la meilleure unité, exploiter, rechercher.
    if let Some(&a) = permises.iter().find(|a| matches!(a, Action::Agrandir { recompense_b: false, .. })) {
        return a;
    }
    let mut prod: Vec<Action> = permises.iter().copied().filter(|a| matches!(a, Action::Produire { .. })).collect();
    prod.sort_by_key(|a| match a {
        Action::Produire { type_unite, .. } => std::cmp::Reverse(rang_unite(*type_unite)),
        _ => std::cmp::Reverse(0),
    });
    if p.nb_unites(j) < 3 || vivres >= 8 {
        if let Some(&a) = prod.first() {
            return a;
        }
    }
    if let Some(&a) = permises.iter().find(|a| matches!(a, Action::Exploiter { .. })) {
        return a;
    }
    if let Some(&a) = permises.iter().find(|a| matches!(a, Action::Batir { .. })) {
        return a;
    }
    if let Some(&a) = permises.iter().find(|a| matches!(a, Action::Rechercher { .. })) {
        return a;
    }
    // 4. Avancer : chaque unité une fois, vers le but connu le plus proche.
    let buts = buts(p, j);
    let pl = p.plateau();
    let mut meilleur: Option<(i32, Action)> = None;
    for &a in &permises {
        if let Action::Deplacer { depart, arrivee } = a {
            if deja_bouge.contains(&depart) || buts.is_empty() {
                continue;
            }
            let d_avant = buts.iter().map(|&b| pl.distance(depart, b)).min().unwrap() as i32;
            let d_apres = buts.iter().map(|&b| pl.distance(arrivee, b)).min().unwrap() as i32;
            let gain = d_avant - d_apres;
            if gain > 0 && meilleur.is_none_or(|(g, _)| gain > g) {
                meilleur = Some((gain, a));
            }
        }
    }
    if let Some((_, a)) = meilleur {
        return a;
    }
    Action::FinDuTour
}

fn rang_unite(t: TypeUnite) -> u8 {
    match t {
        TypeUnite::Cavalier => 6,
        TypeUnite::Archer => 5,
        TypeUnite::CoureurDeCrete | TypeUnite::PortePavois => 4,
        TypeUnite::Catapulte => 3,
        TypeUnite::Lancier => 2,
        TypeUnite::GuetteurPlaneur | TypeUnite::Astronome => 1,
        TypeUnite::Eclaireur => 0,
    }
}

/// Buts connus du joueur : villes qui ne sont pas à lui et sans unité à lui, unités
/// adverses visibles, et à défaut les cases inconnues au bord de ce qu'il sait.
fn buts(p: &Partie, j: u8) -> Vec<Case> {
    let vue = p.vue(j);
    let miennes: Vec<Case> = vue.unites.iter().filter(|u| u.joueur == j).map(|u| u.case).collect();
    let mut b: Vec<Case> = vue.villes.iter().filter(|v| v.joueur != Some(j) && !miennes.contains(&v.case)).map(|v| v.case).collect();
    b.extend(vue.unites.iter().filter(|u| u.joueur != j).map(|u| u.case));
    if b.is_empty() {
        let pl = p.plateau();
        let mut v = [0; 8];
        for c in 0..pl.nb_cases() as Case {
            if vue.cases[c as usize] == Visibilite::Inconnue {
                let n = pl.voisines(c, &mut v);
                if v[..n].iter().any(|&w| vue.cases[w as usize] != Visibilite::Inconnue) {
                    b.push(c);
                }
            }
        }
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::partie::Reglages;

    #[test]
    fn des_parties_entre_ia_vont_au_bout_et_se_reproduisent() {
        let mut fins = [0; 3];
        for nb in 2..=4 {
            for graine in 0..6 {
                let mut p = Partie::nouvelle(Reglages::pour(nb), graine);
                let mut q = p.clone();
                while !p.est_finie() {
                    for a in jouer_tour(&p) {
                        p.appliquer(a).expect("l'IA ne joue que des actions permises");
                    }
                }
                while !q.est_finie() {
                    for a in jouer_tour(&q) {
                        q.appliquer(a).unwrap();
                    }
                }
                assert_eq!(p.empreinte(), q.empreinte());
                fins[match p.gagnant() {
                    Some(_) if p.tour() < 80 => 0,
                    Some(_) => 1,
                    None => 2,
                }] += 1;
            }
        }
        // Au moins quelques parties gagnées avant la limite.
        assert!(fins[0] > 0, "victoires, tardives, nulles : {fins:?}");
    }
}
