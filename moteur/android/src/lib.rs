//! Pont JNI : seules fonctions appelées par Kotlin (`fr.jeu4x.app.moteur.Moteur`).
//! Règle : une interface étroite, des données en blocs, jamais un appel par case.
//!
//! Une partie vit côté Rust ; Kotlin n'en tient qu'une poignée (`jlong`, pointeur vers
//! une `Box<Partie>`), qu'il libère par `liberer`.

use jni::objects::JClass;
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

#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_nouvellePartie(
    _env: JNIEnv,
    _c: JClass,
    graine: jlong,
    taille: jint,
) -> jlong {
    let taille = taille.clamp(regles::plateau::TAILLE_MIN as i32, regles::plateau::TAILLE_MAX as i32) as u8;
    let r = Reglages { taille, ..Reglages::default() };
    Box::into_raw(Box::new(Partie::nouvelle(r, graine as u64))) as jlong
}

#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_liberer(_env: JNIEnv, _c: JClass, h: jlong) {
    if h != 0 {
        // SAFETY : même pointeur que celui rendu par `nouvellePartie`, libéré une seule fois.
        drop(unsafe { Box::from_raw(h as *mut Partie) });
    }
}

/// État en un seul bloc : [taille, tour, joueur courant, finie, limite de tours,
/// nombre d'unités, terrains (taille²)…, puis par unité : joueur, case, points restants].
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_etat(mut env: JNIEnv, _c: JClass, h: jlong) -> jintArray {
    let p = partie(h);
    let r = p.reglages();
    let mut v = vec![
        r.taille as i32,
        p.tour() as i32,
        p.joueur_courant() as i32,
        p.est_finie() as i32,
        r.limite_tours as i32,
        p.unites().len() as i32,
    ];
    v.extend(p.plateau().terrains_bruts().map(|t| t as i32));
    for u in p.unites() {
        v.extend([u.joueur as i32, u.case as i32, u.points_restants as i32]);
    }
    tableau(&mut env, &v)
}

/// Cases atteignables depuis l'unité posée sur `depart` (vide si aucune ou pas son tour).
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_atteignables(
    mut env: JNIEnv,
    _c: JClass,
    h: jlong,
    depart: jint,
) -> jintArray {
    let p = partie(h);
    let a = match p.unites().iter().find(|u| u.case as i32 == depart) {
        Some(u) if u.joueur == p.joueur_courant() && !p.est_finie() => p.atteignables(depart as u16),
        _ => Vec::new(),
    };
    let v: Vec<i32> = a.into_iter().map(|(c, _)| c as i32).collect();
    tableau(&mut env, &v)
}

fn code(r: Result<(), regles::partie::ActionRefusee>) -> jint {
    match r {
        Ok(()) => 0,
        Err(e) => 1 + e as jint,
    }
}

/// 0 si le déplacement est fait, sinon le code du refus (rien n'a changé).
#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_deplacer(
    _env: JNIEnv,
    _c: JClass,
    h: jlong,
    depart: jint,
    arrivee: jint,
) -> jint {
    if depart < 0 || arrivee < 0 {
        return code(Err(regles::partie::ActionRefusee::NumeroInconnu));
    }
    code(partie(h).appliquer(Action::Deplacer { depart: depart as u16, arrivee: arrivee as u16 }))
}

#[no_mangle]
pub extern "system" fn Java_fr_jeu4x_app_moteur_Moteur_finDuTour(_env: JNIEnv, _c: JClass, h: jlong) -> jint {
    code(partie(h).appliquer(Action::FinDuTour))
}
