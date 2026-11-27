//! Pont JNI : seules fonctions appelées par Kotlin (`fr.jeu4x.app.moteur.Moteur`).
//! Règle : une interface étroite, des données en blocs, jamais un appel par case.

use jni::objects::JClass;
use jni::sys::{jint, jlong};
use jni::JNIEnv;

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
