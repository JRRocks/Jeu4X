use super::*;

/// Partie de test : tout en plaine, sans ruines, tout connu, unités posées à la main.
fn partie_plaine(unites: &[(u8, u8, u8, TypeUnite)]) -> Partie {
    let mut p = Partie::nouvelle(Reglages::default(), 1);
    for c in 0..p.plateau.nb_cases() as Case {
        p.plateau.poser(c, Terrain::Plaine);
        p.ruines[c as usize] = AUCUNE;
    }
    p.unites.clear();
    for &(joueur, x, y, t) in unites {
        let c = p.plateau.case(x, y);
        p.creer_unite(joueur, t, c, true);
    }
    for b in p.brouillards.iter_mut() {
        for e in b.cases.iter_mut() {
            *e = Visibilite::VueAutrefois;
        }
    }
    p.recalculer_toutes_les_visions();
    p
}

fn cacher(p: &mut Partie, j: u8) {
    for e in p.brouillards[j as usize].cases.iter_mut() {
        *e = Visibilite::Inconnue;
    }
    p.recalculer_vision(j);
}

#[test]
fn meme_graine_meme_carte() {
    for nb in 2..=4 {
        let a = Partie::nouvelle(Reglages::pour(nb), 42);
        let b = Partie::nouvelle(Reglages::pour(nb), 42);
        assert_eq!(a, b);
        assert_ne!(a.empreinte(), Partie::nouvelle(Reglages::pour(nb), 43).empreinte());
    }
}

#[test]
fn departs_conformes_sur_beaucoup_de_graines() {
    for nb in 2..=4u8 {
        let r = Reglages::pour(nb);
        let (nb_villages, nb_ruines, _) = villages_ruines_seuil(nb);
        for graine in 0..100 {
            let p = Partie::nouvelle(r, graine);
            assert_eq!(p.plateau().taille(), r.taille);
            assert_eq!(p.villes().len(), nb as usize + nb_villages);
            assert_eq!((0..p.plateau.nb_cases() as Case).filter(|&c| p.ruine(c)).count(), nb_ruines);
            let caps: Vec<Case> = (0..nb).map(|j| p.capitale(j)).collect();
            for (a, &ca) in caps.iter().enumerate() {
                assert_eq!(p.plateau.terrain(ca), Terrain::Plaine);
                assert!(p.plateau.distance_au_bord(ca) >= 2);
                for &cb in &caps[a + 1..] {
                    assert!(p.plateau.distance(ca, cb) >= r.distance_capitales);
                }
            }
            for u in p.unites() {
                assert_eq!(p.plateau.distance(u.case, caps[u.joueur as usize]), 1);
            }
            assert_eq!(p.unites().len(), 2 * nb as usize);
        }
    }
}

#[test]
fn huit_directions_et_terrains() {
    let p = partie_plaine(&[(0, 5, 5, TypeUnite::Lancier)]);
    assert_eq!(p.atteignables(p.plateau.case(5, 5)).len(), 8);
    let mut q = partie_plaine(&[(0, 5, 5, TypeUnite::Eclaireur)]);
    assert_eq!(q.atteignables(q.plateau.case(5, 5)).len(), 48);
    let pl = q.plateau.clone();
    q.plateau.poser(pl.case(6, 5), Terrain::Colline);
    q.plateau.poser(pl.case(4, 5), Terrain::Eau);
    q.plateau.poser(pl.case(5, 6), Terrain::Montagne);
    let a = q.atteignables(pl.case(5, 5));
    assert!(a.contains(&(pl.case(6, 5), 1)));
    assert!(!a.iter().any(|&(c, _)| c == pl.case(4, 5) || c == pl.case(5, 6)));
    // Premier pas toujours permis : un Lancier entre en forêt avec 1 point.
    let mut r = partie_plaine(&[(0, 5, 5, TypeUnite::Lancier)]);
    r.plateau.poser(pl.case(6, 5), Terrain::Foret);
    assert!(r.atteignables(pl.case(5, 5)).contains(&(pl.case(6, 5), 0)));
}

#[test]
fn zone_de_controle_et_surprise() {
    let p = partie_plaine(&[(0, 2, 5, TypeUnite::Eclaireur), (1, 5, 5, TypeUnite::Lancier)]);
    let a = p.atteignables(p.plateau.case(2, 5));
    assert!(a.contains(&(p.plateau.case(4, 5), 0)));
    let mut q = partie_plaine(&[(0, 0, 5, TypeUnite::Eclaireur), (1, 5, 5, TypeUnite::Lancier)]);
    let pl = q.plateau.clone();
    q.appliquer(Action::Deplacer { depart: pl.case(0, 5), arrivee: pl.case(3, 5) }).unwrap();
    assert_eq!(q.unites[0].case, pl.case(2, 5), "arrêtée quand l'adverse apparaît");
    assert_eq!(q.unites[0].points_restants, 0);
}

#[test]
fn le_brouillard_se_souvient() {
    let mut p = partie_plaine(&[(0, 0, 0, TypeUnite::Eclaireur), (1, 10, 10, TypeUnite::Lancier)]);
    cacher(&mut p, 0);
    let pl = p.plateau.clone();
    assert_eq!(p.visibilite(0, pl.case(3, 0)), Visibilite::Visible);
    p.appliquer(Action::Deplacer { depart: 0, arrivee: pl.case(3, 3) }).unwrap();
    p.appliquer(Action::FinDuTour).unwrap();
    p.appliquer(Action::FinDuTour).unwrap();
    p.appliquer(Action::Deplacer { depart: pl.case(3, 3), arrivee: pl.case(6, 6) }).unwrap();
    // (0,0) n'est vue que si une ville du joueur 0 est assez près : on teste une case loin de tout.
    let vue = p.vue(0);
    assert!(vue.unites.iter().all(|u| u.joueur == 0 || p.visibilite(0, u.case) == Visibilite::Visible));
    assert!(vue.cases.iter().any(|&e| e == Visibilite::VueAutrefois));
}

#[test]
fn combats_des_exemples_du_paragraphe_6_3() {
    let p = partie_plaine(&[(0, 5, 5, TypeUnite::Lancier), (1, 6, 5, TypeUnite::Lancier)]);
    let an = p.annonce_combat(p.plateau.case(5, 5), p.plateau.case(6, 5)).unwrap();
    assert_eq!((an.degats, an.riposte), (4, 2));
    let q = partie_plaine(&[(0, 5, 5, TypeUnite::Cavalier), (1, 6, 5, TypeUnite::Archer)]);
    let an = q.annonce_combat(q.plateau.case(5, 5), q.plateau.case(6, 5)).unwrap();
    assert_eq!(an.degats, 9);
    assert!(an.cible_detruite);
    let r = partie_plaine(&[(0, 5, 5, TypeUnite::Archer), (1, 7, 5, TypeUnite::Lancier)]);
    let an = r.annonce_combat(r.plateau.case(5, 5), r.plateau.case(7, 5)).unwrap();
    assert_eq!((an.degats, an.riposte), (6, 0));
    // Lancier contre Lancier dans une capitale à remparts : 4 + 3 − (3 + 4 + 2) → 1, riposte 2.
    let mut s = partie_plaine(&[]);
    let cap = s.capitale(1);
    let (x, y) = (s.plateau.colonne(cap), s.plateau.rangee(cap));
    s.creer_unite(1, TypeUnite::Lancier, cap, true);
    let voisine = s.plateau.case(if x > 0 { x - 1 } else { x + 1 }, y);
    s.creer_unite(0, TypeUnite::Lancier, voisine, true);
    s.villes[1].batiments |= Batiment::Remparts.bit();
    s.recalculer_toutes_les_visions();
    let an = s.annonce_combat(voisine, cap).unwrap();
    assert_eq!((an.degats, an.riposte), (1, 2));
}

#[test]
fn attaquer_applique_l_annonce() {
    let mut p = partie_plaine(&[(0, 5, 5, TypeUnite::Lancier), (1, 6, 5, TypeUnite::Lancier)]);
    let (a, c) = (p.plateau.case(5, 5), p.plateau.case(6, 5));
    p.appliquer(Action::Attaquer { depart: a, cible: c }).unwrap();
    assert_eq!(p.unites[0].pv, 8);
    assert_eq!(p.unites[1].pv, 6);
    assert!(p.cibles(a).is_empty(), "une attaque par tour");
}

#[test]
fn prendre_la_capitale_a_deux_gagne() {
    let mut p = partie_plaine(&[]);
    let cap = p.capitale(1);
    p.creer_unite(0, TypeUnite::Lancier, cap, true);
    p.recalculer_toutes_les_visions();
    p.appliquer(Action::Rallier { case: cap }).unwrap();
    assert!(p.est_finie());
    assert_eq!(p.gagnant(), Some(0));
}

#[test]
fn rallier_un_village() {
    let mut p = Partie::nouvelle(Reglages::default(), 3);
    let village = p.villes().iter().find(|v| v.joueur.is_none()).unwrap().case;
    p.unites[0].case = village;
    p.recalculer_toutes_les_visions();
    let avant = p.score(0);
    p.appliquer(Action::Rallier { case: village }).unwrap();
    assert_eq!(p.villes().iter().find(|v| v.case == village).unwrap().joueur, Some(0));
    assert_eq!(p.score(0), avant + 1);
    assert!(p.entretien(0) > 3);
}

#[test]
fn victoire_au_seuil_au_debut_du_tour_suivant() {
    let mut p = Partie::nouvelle(Reglages::default(), 4);
    p.joueurs[0].points_acquis = 60;
    p.appliquer(Action::FinDuTour).unwrap();
    assert!(p.joueurs[0].annonce);
    assert!(!p.est_finie());
    p.appliquer(Action::FinDuTour).unwrap();
    assert_eq!(p.gagnant(), Some(0));
}

#[test]
fn partie_nulle_apres_le_dernier_tour() {
    let r = Reglages { limite_tours: 2, ..Reglages::pour(3) };
    let mut p = Partie::nouvelle(r, 3);
    for _ in 0..6 {
        p.appliquer(Action::FinDuTour).unwrap();
    }
    assert!(p.est_nulle());
    assert!(p.actions_permises().is_empty());
}

#[test]
fn economie_croissance_recherche() {
    let mut p = Partie::nouvelle(Reglages::default(), 6);
    p.joueurs[0].vivres = 100;
    let cap = p.capitale(0);
    let r0 = p.revenu(0);
    assert_eq!(r0, 4, "capitale niveau 1 : 2 + 2");
    p.appliquer(Action::Agrandir { ville: cap, recompense_b: false }).unwrap();
    assert_eq!(p.revenu(0), r0 + 1, "niveau 2 (+0) et cadeau a (+1)");
    p.appliquer(Action::Rechercher { avancee: Avancee::Champs }).unwrap();
    assert!(!p.actions_permises().contains(&Action::Rechercher { avancee: Avancee::Champs }));
    p.appliquer(Action::Rechercher { avancee: Avancee::Carrieres }).unwrap();
    p.appliquer(Action::Rechercher { avancee: Avancee::Marches }).unwrap();
    assert!(!p.avancee_ouverte(0, Avancee::Greniers), "choix exclusif");
    p.appliquer(Action::Batir { ville: cap, batiment: Batiment::Marche }).unwrap();
    assert_eq!(p.revenu(0), r0 + 3);
}

#[test]
fn numeros_d_action_aller_retour_en_cours_de_partie() {
    for nb in 2..=4 {
        let (_, actions) = crate::aleatoire::jouer_au_hasard(Reglages::pour(nb), 9, 9);
        let mut p = Partie::nouvelle(Reglages::pour(nb), 9);
        let taille = p.reglages().taille;
        for (k, &n) in actions.iter().enumerate() {
            if k % 7 == 0 {
                for a in p.actions_permises() {
                    let m = a.numero(taille);
                    assert!(m < NB_ACTIONS, "{a:?}");
                    assert_eq!(Action::depuis_numero(m, taille), Some(a));
                    assert_eq!(p.texte_vers_action(&p.action_vers_texte(a)), Some(a));
                }
            }
            p.appliquer_numero(n).unwrap();
        }
    }
}

#[test]
fn la_vue_ne_dit_rien_de_cache() {
    let p = Partie::nouvelle(Reglages::pour(4), 11);
    for j in 0..4 {
        let v = p.vue(j);
        for u in &v.unites {
            assert!(u.joueur == j || p.visibilite(j, u.case) == Visibilite::Visible);
        }
        for c in 0..p.plateau.nb_cases() {
            assert_eq!(v.terrains[c].is_some(), v.cases[c] != Visibilite::Inconnue);
        }
        assert!(v.villes.iter().any(|vi| vi.joueur == Some(j) && vi.capitale()));
        assert!(v.joueurs.iter().all(|jo| jo.vivres == 0));
    }
}

#[test]
fn codage_du_point_de_vue_de_chaque_joueur() {
    let p = Partie::nouvelle(Reglages::pour(4), 8);
    let tm = TAILLE_MAX as usize;
    for j in 0..4 {
        let (plans, v) = coder(&p, j);
        assert_eq!(plans.len(), NB_CANAUX * tm * tm);
        let ma_ville = NB_TERRAINS + 6;
        let i = (0..tm * tm).find(|&i| plans[ma_ville * tm * tm + i] == 255).unwrap();
        assert!(i % tm <= 6 && i / tm <= 6, "ma capitale en bas à gauche");
        assert_eq!(v[2], 4);
    }
}
