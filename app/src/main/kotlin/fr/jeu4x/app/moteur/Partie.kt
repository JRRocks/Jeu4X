package fr.jeu4x.app.moteur

/** Terrains, dans l'ordre du moteur (`regles::plateau::Terrain`). */
enum class Terrain { PLAINE, FORET, MONTAGNE, EAU }

data class Unite(val joueur: Int, val case: Int, val pointsRestants: Int)

/** Photographie de l'état, lue en un seul appel au moteur. Les cases sont des numéros : jamais d'adresse à l'écran (D-16). */
data class Etat(
    val taille: Int,
    val tour: Int,
    val joueurCourant: Int,
    val finie: Boolean,
    val limiteTours: Int,
    val terrains: List<Terrain>,
    val unites: List<Unite>,
) {
    fun colonne(case: Int) = case % taille
    fun rangee(case: Int) = case / taille
    fun uniteSur(case: Int) = unites.firstOrNull { it.case == case }

    companion object {
        fun depuisBloc(b: IntArray): Etat {
            val taille = b[0]
            val nbUnites = b[5]
            val debutUnites = 6 + taille * taille
            return Etat(
                taille = taille,
                tour = b[1],
                joueurCourant = b[2],
                finie = b[3] != 0,
                limiteTours = b[4],
                terrains = (0 until taille * taille).map { Terrain.entries[b[6 + it]] },
                unites = (0 until nbUnites).map { i ->
                    val k = debutUnites + 3 * i
                    Unite(joueur = b[k], case = b[k + 1], pointsRestants = b[k + 2])
                },
            )
        }
    }
}

/** Une partie tenue par le moteur Rust. À fermer par [fermer]. */
class Partie(graine: Long, taille: Int = 11) {
    private var poignee: Long = Moteur.nouvellePartie(graine, taille)

    fun etat(): Etat = Etat.depuisBloc(Moteur.etat(vivante()))
    fun atteignables(depart: Int): Set<Int> = Moteur.atteignables(vivante(), depart).toSet()
    fun deplacer(depart: Int, arrivee: Int): Boolean = Moteur.deplacer(vivante(), depart, arrivee) == 0
    fun finDuTour(): Boolean = Moteur.finDuTour(vivante()) == 0

    fun fermer() {
        if (poignee != 0L) {
            Moteur.liberer(poignee)
            poignee = 0L
        }
    }

    private fun vivante(): Long {
        check(poignee != 0L) { "partie déjà fermée" }
        return poignee
    }
}
