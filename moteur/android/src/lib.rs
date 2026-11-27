//! Pont JNI : seules fonctions appelées par Kotlin (`fr.jeu4x.app.moteur.Moteur`).
//! Règle : une interface étroite, des données en blocs, jamais un appel par case.
//!
//! Une partie vit côté Rust ; Kotlin n'en tient qu'une poignée (`jlong`, pointeur vers
//! une `Box<Partie>`), qu'il libère par `liberer`.

use jni::objects::{JClass, JIntArray};
use jni::sys::{jint, jintArray, jlong};
use jni::JNIEnv;
use regles::partie::{Action, Partie, Reglages};

#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_versionRegles(_env: JNIEnv, _c: JClass) -> jint {
    regles::VERSION_REGLES as jint
}

#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_autoControle(
    _env: JNIEnv,
    _c: JClass,
    graine: jlong,
    n: jint,
) -> jlong {
    regles::auto_controle(graine as u64, n.max(0) as u32) as jlong
}

fn partie<'a>(h: jlong) -> &'a mut Partie {
    // SAFETY : `h` vient de `nouvellePartie` et n'a pas été libérée (garanti côté Kotlin).
    unsafe { &mut *(h as *mut Partie) }
}

fn tableau(env: &mut JNIEnv, v: &[i32]) -> jintArray {
    match env.new_int_array(v.len() as i32) {
        Ok(a) => {
            let _ = env.set_int_array_region(&a, 0, v);
            a.into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Nouvelle partie de `nb_joueurs` joueurs (2 à 4), réglages de la V1, clans donnés
/// dans l'ordre des joueurs (indices de `Clan`).
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_nouvellePartie(
    env: JNIEnv,
    _c: JClass,
    graine: jlong,
    nb_joueurs: jint,
    clans: JIntArray,
) -> jlong {
    let mut r = Reglages::pour(nb_joueurs.clamp(2, regles::partie::JOUEURS_MAX as i32) as u8);
    let mut c = [0i32; 4];
    if let Ok(n) = env.get_array_length(&clans) {
        let n = (n as usize).min(4);
        let _ = env.get_int_array_region(&clans, 0, &mut c[..n]);
    }
    for (k, v) in c.iter().enumerate() {
        r.clans[k] = (*v).rem_euclid(4) as u8;
    }
    Box::into_raw(Box::new(Partie::nouvelle(r, graine as u64))) as jlong
}

#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_liberer(_env: JNIEnv, _c: JClass, h: jlong) {
    if h != 0 {
        // SAFETY : même pointeur que celui rendu par `nouvellePartie`, libéré une seule fois.
        drop(unsafe { Box::from_raw(h as *mut Partie) });
    }
}

/// Vue de `joueur` en un seul bloc (rien de ce qu'il ne sait pas). Ordre :
/// en-tête [taille, tour, joueur courant, finie, gagnant (-1), limite, seuil, joueurs,
/// joueur qui regarde, vivres, revenu, entretien, nombre de ses unités] ;
/// par joueur [clan, score, avancées (bits), annoncé, éliminé] ;
/// par case : terrain (-1 inconnue), visibilité, joueur du territoire (-1), exploitation (-1), ruine (0/1) ;
/// nombre d'unités, puis [joueur, case, type, pv, pv max, points, a bougé, a attaqué, peut agir] ;
/// nombre de villes, puis [case, joueur (-1), capitale, niveau, revenu, prix de croissance, bâtiments, rayon] ;
/// par avancée [prix, ouverte] ; par merveille [prise].
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_etat(mut env: JNIEnv, _c: JClass, h: jlong, joueur: jint) -> jintArray {
    let p = partie(h);
    let j = joueur.clamp(0, p.reglages().nb_joueurs as i32 - 1) as u8;
    let vue = p.vue(j);
    let n = p.plateau().nb_cases();
    let mut v = vec![
        vue.taille as i32,
        vue.tour as i32,
        vue.joueur_courant as i32,
        vue.finie as i32,
        vue.gagnant.map_or(-1, |g| g as i32),
        vue.limite_tours as i32,
        vue.seuil,
        vue.nb_joueurs as i32,
        j as i32,
        vue.vivres,
        vue.revenu,
        p.entretien(j),
        p.nb_unites(j),
    ];
    for (k, jo) in vue.joueurs.iter().enumerate() {
        v.extend([jo.clan as i32, vue.scores[k], jo.avancees as i32, jo.annonce as i32, jo.elimine as i32]);
    }
    v.extend(vue.terrains.iter().map(|t| t.map_or(-1, |t| t as i32)));
    v.extend(vue.cases.iter().map(|&e| e as i32));
    for c in 0..n {
        let connue = vue.terrains[c].is_some();
        v.push(match p.territoire(c as u16) {
            Some(vi) if connue => vi.joueur.map_or(-1, |x| x as i32),
            _ => -1,
        });
    }
    let mut expl = vec![-1; n];
    for &(c, e) in &vue.exploitations {
        expl[c as usize] = e as i32;
    }
    v.extend(expl);
    let mut ruines = vec![0; n];
    for &c in &vue.ruines {
        ruines[c as usize] = 1;
    }
    v.extend(ruines);
    v.push(vue.unites.len() as i32);
    for u in &vue.unites {
        v.extend([
            u.joueur as i32,
            u.case as i32,
            u.type_unite as i32,
            u.pv,
            p.pv_max(u),
            u.points_restants as i32,
            u.a_bouge as i32,
            u.a_attaque as i32,
            u.peut_agir as i32,
        ]);
    }
    v.push(vue.villes.len() as i32);
    for vi in &vue.villes {
        let i = p.villes().iter().position(|w| w.case == vi.case).unwrap_or(0);
        let mienne = vi.joueur == Some(j);
        v.extend([
            vi.case as i32,
            vi.joueur.map_or(-1, |x| x as i32),
            vi.capitale() as i32,
            vi.niveau as i32,
            if mienne { p.revenu_ville(i) } else { 0 },
            if mienne && vi.niveau < 5 { p.prix_croissance(i) } else { 0 },
            vi.batiments as i32,
            vi.rayon as i32,
        ]);
    }
    for a in regles::donnees::AVANCEES {
        v.extend([p.prix_avancee(j, a), p.avancee_ouverte(j, a) as i32]);
    }
    for k in 0..3 {
        v.push(p.merveille_prise(k) as i32);
    }
    tableau(&mut env, &v)
}

/// Actions permises au joueur courant : par action [numéro, genre, a, b] (voir `decrire`).
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_permises(mut env: JNIEnv, _c: JClass, h: jlong) -> jintArray {
    let p = partie(h);
    let t = p.reglages().taille;
    let mut v = Vec::new();
    for a in p.actions_permises() {
        v.push(a.numero(t) as i32);
        v.extend(decrire(a));
    }
    tableau(&mut env, &v)
}

/// Genre et paramètres d'une action : 0 fin du tour, 1 déplacer (départ, arrivée),
/// 2 attaquer (départ, cible), 3 lunette (départ, cible), 4 rallier (case), 5 renvoyer (case),
/// 6 produire (ville, type), 7 agrandir (ville, 0 a / 1 b), 8 exploiter (case),
/// 9 bâtir (ville, bâtiment), 10 rechercher (avancée).
fn decrire(a: Action) -> [i32; 3] {
    match a {
        Action::FinDuTour => [0, 0, 0],
        Action::Deplacer { depart, arrivee } => [1, depart as i32, arrivee as i32],
        Action::Attaquer { depart, cible } => [2, depart as i32, cible as i32],
        Action::Lunette { depart, cible } => [3, depart as i32, cible as i32],
        Action::Rallier { case } => [4, case as i32, 0],
        Action::Renvoyer { case } => [5, case as i32, 0],
        Action::Produire { ville, type_unite } => [6, ville as i32, type_unite as i32],
        Action::Agrandir { ville, recompense_b } => [7, ville as i32, recompense_b as i32],
        Action::Exploiter { case } => [8, case as i32, 0],
        Action::Batir { ville, batiment } => [9, ville as i32, batiment as i32],
        Action::Rechercher { avancee } => [10, avancee as i32, 0],
    }
}

/// 0 si l'action de numéro `n` est faite, sinon le code du refus (rien n'a changé).
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_appliquer(_env: JNIEnv, _c: JClass, h: jlong, n: jint) -> jint {
    if n < 0 {
        return code(Err(regles::partie::ActionRefusee::NumeroInconnu));
    }
    code(partie(h).appliquer_numero(n as u32))
}

/// Annonce du combat [dégâts, riposte, cible détruite, attaquant détruit], vide si l'attaque n'est pas permise.
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_annonce(mut env: JNIEnv, _c: JClass, h: jlong, depart: jint, cible: jint) -> jintArray {
    let v = match partie(h).annonce_combat(depart.max(0) as u16, cible.max(0) as u16) {
        Some(a) => vec![a.degats, a.riposte, a.cible_detruite as i32, a.attaquant_detruit as i32],
        None => Vec::new(),
    };
    tableau(&mut env, &v)
}

/// Tour entier de l'adversaire tenu par l'ordinateur pour le joueur courant (numéros
/// d'actions, fin du tour comprise). Ne change rien : l'application les applique une à une.
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_tourIa(mut env: JNIEnv, _c: JClass, h: jlong) -> jintArray {
    let p = partie(h);
    let t = p.reglages().taille;
    let v: Vec<i32> = regles::ia_simple::jouer_tour(p).into_iter().map(|a| a.numero(t) as i32).collect();
    tableau(&mut env, &v)
}

fn code(r: Result<(), regles::partie::ActionRefusee>) -> jint {
    match r {
        Ok(()) => 0,
        Err(e) => 1 + e as jint,
    }
}
