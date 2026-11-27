package fr.jeu4x.app.moteur

/**
 * Pont vers le moteur de règles écrit en Rust (`moteur/android`).
 * Interface étroite : peu d'appels, des données en blocs.
 */
object Moteur {
    init {
        System.loadLibrary("jeu4x_android")
    }

    /** Graine et longueur de l'auto-contrôle, et la valeur attendue (figée côté Rust). */
    const val GRAINE_CONTROLE = 42L
    const val LONGUEUR_CONTROLE = 1000
    const val EMPREINTE_ATTENDUE = -4127879376781910154L // 14318864696927641462 en u64

    @JvmStatic external fun versionRegles(): Int
    @JvmStatic external fun autoControle(graine: Long, n: Int): Long

    @JvmStatic external fun nouvellePartie(graine: Long, nbJoueurs: Int, clans: IntArray): Long
    @JvmStatic external fun liberer(partie: Long)
    @JvmStatic external fun etat(partie: Long, joueur: Int): IntArray
    @JvmStatic external fun permises(partie: Long): IntArray
    @JvmStatic external fun appliquer(partie: Long, numero: Int): Int
    @JvmStatic external fun annonce(partie: Long, depart: Int, cible: Int): IntArray
    @JvmStatic external fun tourIa(partie: Long): IntArray

    /** Vrai si le moteur chargé donne la même empreinte que sur le PC. */
    fun controleReussi(): Boolean = autoControle(GRAINE_CONTROLE, LONGUEUR_CONTROLE) == EMPREINTE_ATTENDUE
}
