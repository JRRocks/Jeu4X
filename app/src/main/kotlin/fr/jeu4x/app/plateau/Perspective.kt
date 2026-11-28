package fr.jeu4x.app.plateau

import kotlin.math.floor

/**
 * Vue en perspective (losanges, D-16 variante C). Calculs purs, testés sans écran.
 *
 * Repère « plateau » : la case (colonne x, rangée y) a son sommet haut en
 * ((x - y') × demiLargeur, (x + y') × demiHauteur), avec y' = taille - 1 - y : la rangée 0
 * (départ du joueur 0) est en bas de l'écran.
 */
class Perspective(val taille: Int, val demiLargeur: Float, val demiHauteur: Float = demiLargeur / 2f) {

    /** Sommet haut du losange de la case, dans le repère plateau. */
    fun sommet(colonne: Int, rangee: Int): Pair<Float, Float> {
        val yp = taille - 1 - rangee
        return (colonne - yp) * demiLargeur to (colonne + yp) * demiHauteur
    }

    /** Centre du losange. */
    fun centre(colonne: Int, rangee: Int): Pair<Float, Float> {
        val (sx, sy) = sommet(colonne, rangee)
        return sx to sy + demiHauteur
    }

    /** Case sous un point du repère plateau, ou null hors du plateau. */
    fun caseSous(px: Float, py: Float): Pair<Int, Int>? {
        val u = py / demiHauteur
        val v = px / demiLargeur
        val colonne = floor((u + v) / 2f).toInt()
        val yp = floor((u - v) / 2f).toInt()
        val rangee = taille - 1 - yp
        if (colonne !in 0 until taille || rangee !in 0 until taille) return null
        return colonne to rangee
    }

    /** Ordre de dessin : du fond vers l'avant, pour que les reliefs se recouvrent bien. */
    fun ordreDeDessin(): List<Pair<Int, Int>> =
        (0 until taille).flatMap { x -> (0 until taille).map { y -> x to y } }
            .sortedWith(compareBy({ it.first + (taille - 1 - it.second) }, { it.first }))

    /** Boîte englobante du plateau : (gauche, haut, droite, bas). */
    fun limites(): FloatArray = floatArrayOf(
        -(taille - 1) * demiLargeur - demiLargeur,
        0f,
        (taille - 1) * demiLargeur + demiLargeur,
        2 * (taille - 1) * demiHauteur + 2 * demiHauteur,
    )

    /** Échelle à laquelle tout le plateau tient dans une vue de `largeur` × `hauteur`. */
    fun echelleEnsemble(largeur: Float, hauteur: Float): Float {
        val l = limites()
        return minOf(largeur / (l[2] - l[0]), hauteur / (l[3] - l[1])) * 0.95f
    }

    /**
     * Cadrage au lancement d'une partie (D-20) : cases deux fois plus grosses que la vue
     * d'ensemble, centrées sur les cases données (les unités du joueur qui commence).
     * Rend l'échelle et le décalage (x, y) à appliquer au repère plateau.
     */
    fun cadrageSur(cases: List<Pair<Int, Int>>, largeur: Float, hauteur: Float, grossissement: Float = 2f): Triple<Float, Float, Float> {
        val echelle = echelleEnsemble(largeur, hauteur) * grossissement
        val (dx, dy) = decalagePour(cases, largeur, hauteur, echelle)
        return Triple(echelle, dx, dy)
    }

    /**
     * Décalage pour voir les cases données (D-20, D-21) : leur centre moyen part du milieu de la vue,
     * puis la vue glisse au plus d'un sixième de sa taille pour que le plateau couvre l'écran
     * (pas de grand vide quand les unités sont au bord du plateau).
     */
    fun decalagePour(cases: List<Pair<Int, Int>>, largeur: Float, hauteur: Float, echelle: Float): Pair<Float, Float> {
        val centres = if (cases.isEmpty()) listOf(centre((taille - 1) / 2, (taille - 1) / 2)) else cases.map { (x, y) -> centre(x, y) }
        val cx = centres.map { it.first }.average().toFloat()
        val cy = centres.map { it.second }.average().toFloat()
        val l = limites()
        val dx = ajuster(largeur / 2f - cx * echelle, l[0] * echelle, l[2] * echelle, largeur)
        val dy = ajuster(hauteur / 2f - cy * echelle, l[1] * echelle, l[3] * echelle, hauteur)
        return dx to dy
    }

    /** Sur un axe : `ideal` rapproché de la plage où le plateau [debut, fin] couvre la vue, de vue/6 au plus. */
    private fun ajuster(ideal: Float, debut: Float, fin: Float, vue: Float): Float {
        val couvrant = if (fin - debut >= vue) ideal.coerceIn(vue - fin, -debut) else (vue - debut - fin) / 2f
        return couvrant.coerceIn(ideal - vue / 6f, ideal + vue / 6f)
    }
}
