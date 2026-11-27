package fr.jeu4x.app.moteur

/** Terrains, dans l'ordre du moteur (`regles::plateau::Terrain`). */
enum class Terrain { PLAINE, FORET, COLLINE, MONTAGNE, EAU }

/** État d'une case pour le joueur qui regarde (`regles::partie::Visibilite`). */
enum class Visibilite { INCONNUE, VUE_AUTREFOIS, VISIBLE }

/** Types d'unités, dans l'ordre du moteur (`regles::partie::TypeUnite`). */
enum class TypeUnite { ECLAIREUR, LANCIER }

data class Unite(val joueur: Int, val case: Int, val pointsRestants: Int, val type: TypeUnite = TypeUnite.LANCIER)

/** Ville connue du joueur qui regarde ; `joueur` = -1 pour un village neutre. */
data class Ville(val case: Int, val joueur: Int, val capitale: Boolean)

/**
 * Ce que voit le joueur courant, lu en un seul appel au moteur : rien de ce qu'il ne sait
 * pas (brouillard). Les cases sont des numéros : jamais d'adresse à l'écran (D-16).
 */
data class Etat(
    val taille: Int,
    val tour: Int,
    val joueurCourant: Int,
    val finie: Boolean,
    val limiteTours: Int,
    /** Terrain de chaque case ; `null` pour une case inconnue. */
    val terrains: List<Terrain?>,
    val unites: List<Unite>,
    val nbJoueurs: Int = 2,
    val visibilite: List<Visibilite> = terrains.map { if (it == null) Visibilite.INCONNUE else Visibilite.VISIBLE },
    val villes: List<Ville> = emptyList(),
) {
    fun colonne(case: Int) = case % taille
    fun rangee(case: Int) = case / taille
    fun uniteSur(case: Int) = unites.firstOrNull { it.case == case }
    fun villeSur(case: Int) = villes.firstOrNull { it.case == case }

    companion object {
        fun depuisBloc(b: IntArray): Etat {
            val taille = b[0]
            val nbCases = taille * taille
            val nbUnites = b[6]
            val nbVilles = b[7]
            val debutTerrains = 8
            val debutVisibilite = debutTerrains + nbCases
            val debutUnites = debutVisibilite + nbCases
            val debutVilles = debutUnites + 4 * nbUnites
            return Etat(
                taille = taille,
                tour = b[1],
                joueurCourant = b[2],
                finie = b[3] != 0,
                limiteTours = b[4],
                nbJoueurs = b[5],
                terrains = (0 until nbCases).map { b[debutTerrains + it].let { t -> if (t < 0) null else Terrain.entries[t] } },
                visibilite = (0 until nbCases).map { Visibilite.entries[b[debutVisibilite + it]] },
                unites = (0 until nbUnites).map { i ->
                    val k = debutUnites + 4 * i
                    Unite(joueur = b[k], case = b[k + 1], type = TypeUnite.entries[b[k + 2]], pointsRestants = b[k + 3])
                },
                villes = (0 until nbVilles).map { i ->
                    val k = debutVilles + 3 * i
                    Ville(case = b[k], joueur = b[k + 1], capitale = b[k + 2] != 0)
                },
            )
        }
    }
}

/** Une partie tenue par le moteur Rust. À fermer par [fermer]. */
class Partie(graine: Long, nbJoueurs: Int = 2) {
    private var poignee: Long = Moteur.nouvellePartie(graine, nbJoueurs)

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
