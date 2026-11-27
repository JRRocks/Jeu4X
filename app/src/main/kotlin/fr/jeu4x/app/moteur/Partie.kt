package fr.jeu4x.app.moteur

/** Terrains, dans l'ordre du moteur (`regles::plateau::Terrain`). */
enum class Terrain { PLAINE, FORET, COLLINE, MONTAGNE, EAU }

/** État d'une case pour le joueur qui regarde (`regles::partie::Visibilite`). */
enum class Visibilite { INCONNUE, VUE_AUTREFOIS, VISIBLE }

/** Types d'unités, dans l'ordre du moteur (`regles::donnees::TypeUnite`). */
enum class TypeUnite { ECLAIREUR, LANCIER, ARCHER, CAVALIER, CATAPULTE, COUREUR, PORTE_PAVOIS, ASTRONOME, PLANEUR }

/** Clans, dans l'ordre du moteur (`regles::donnees::Clan`). */
enum class Clan { CROCS_ROUGES, MURAILLIERS, VEILLEURS, VENTS_ERRANTS }

/** Avancées, dans l'ordre du moteur (`regles::donnees::Avancee`) : trois branches de cinq. */
enum class Avancee { ARCS, DRESSAGE, REMPARTS, FORGE, ARENE, CHAMPS, CARRIERES, MARCHES, GRENIERS, VERGER, SENTIERS, MECANIQUE, CARTOGRAPHES, ECOLES, TOUR }

/** Bâtiments, dans l'ordre du moteur (`regles::donnees::Batiment`). */
enum class Batiment { REMPARTS, MARCHE, ARENE, VERGER, TOUR }

/** Exploitations, dans l'ordre du moteur. */
enum class Exploitation { CUEILLETTE, CHAMP, CARRIERE }

/** Coût de production de chaque type d'unité (§ 12.3). */
val COUT_UNITE = mapOf(
    TypeUnite.ECLAIREUR to 3, TypeUnite.LANCIER to 4, TypeUnite.ARCHER to 5, TypeUnite.CAVALIER to 6, TypeUnite.CATAPULTE to 8,
    TypeUnite.COUREUR to 4, TypeUnite.PORTE_PAVOIS to 5, TypeUnite.ASTRONOME to 4, TypeUnite.PLANEUR to 5,
)

data class Unite(
    val joueur: Int,
    val case: Int,
    val pointsRestants: Int,
    val type: TypeUnite = TypeUnite.LANCIER,
    val pv: Int = 10,
    val pvMax: Int = 10,
    val aBouge: Boolean = false,
    val aAttaque: Boolean = false,
    val peutAgir: Boolean = true,
)

/** Ville ou village connu ; `joueur` = -1 pour un village neutre. Revenu et prix : les siennes seulement. */
data class Ville(
    val case: Int,
    val joueur: Int,
    val capitale: Boolean,
    val niveau: Int = 1,
    val revenu: Int = 0,
    val prixCroissance: Int = 0,
    val batiments: Int = 0,
    val rayon: Int = 1,
) {
    fun a(b: Batiment) = batiments and (1 shl b.ordinal) != 0
}

/** Informations publiques d'un joueur. */
data class InfoJoueur(val clan: Clan, val score: Int, val avancees: Int, val annonce: Boolean, val elimine: Boolean) {
    fun possede(a: Avancee) = avancees and (1 shl a.ordinal) != 0
}

/** Une action permise, telle que le moteur la décrit (`moteur/android`, `decrire`). */
data class ActionPermise(val numero: Int, val genre: Int, val a: Int, val b: Int) {
    companion object {
        const val FIN = 0
        const val DEPLACER = 1
        const val ATTAQUER = 2
        const val LUNETTE = 3
        const val RALLIER = 4
        const val RENVOYER = 5
        const val PRODUIRE = 6
        const val AGRANDIR = 7
        const val EXPLOITER = 8
        const val BATIR = 9
        const val RECHERCHER = 10

        fun depuisBloc(b: IntArray) = (0 until b.size / 4).map { ActionPermise(b[4 * it], b[4 * it + 1], b[4 * it + 2], b[4 * it + 3]) }
    }
}

data class Annonce(val degats: Int, val riposte: Int, val cibleDetruite: Boolean, val attaquantDetruit: Boolean)

/**
 * Ce que voit un joueur, lu en un seul appel au moteur : rien de ce qu'il ne sait pas
 * (brouillard). Les cases sont des numéros : jamais d'adresse à l'écran (D-16).
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
    val gagnant: Int = -1,
    val seuil: Int = 45,
    val joueur: Int = 0,
    val vivres: Int = 0,
    val revenu: Int = 0,
    val entretien: Int = 0,
    val nbMesUnites: Int = 0,
    val joueurs: List<InfoJoueur> = emptyList(),
    /** Joueur dont la ville tient la case (-1 : aucun), cases connues seulement. */
    val territoire: List<Int> = List(terrains.size) { -1 },
    val exploitations: List<Exploitation?> = List(terrains.size) { null },
    val ruines: List<Boolean> = List(terrains.size) { false },
    /** Prix de chaque avancée pour ce joueur, et si elle lui est ouverte. */
    val prixAvancees: List<Int> = List(15) { 0 },
    val avanceesOuvertes: List<Boolean> = List(15) { false },
    val merveillesPrises: List<Boolean> = List(3) { false },
) {
    fun colonne(case: Int) = case % taille
    fun rangee(case: Int) = case / taille
    fun uniteSur(case: Int) = unites.firstOrNull { it.case == case }
    fun villeSur(case: Int) = villes.firstOrNull { it.case == case }
    fun distance(a: Int, b: Int) = maxOf(kotlin.math.abs(colonne(a) - colonne(b)), kotlin.math.abs(rangee(a) - rangee(b)))

    companion object {
        fun depuisBloc(b: IntArray): Etat {
            var k = 0
            fun lire() = b[k++]
            val taille = lire(); val tour = lire(); val courant = lire(); val finie = lire() != 0
            val gagnant = lire(); val limite = lire(); val seuil = lire(); val nb = lire()
            val joueur = lire(); val vivres = lire(); val revenu = lire(); val entretien = lire(); val nbMiennes = lire()
            val joueurs = List(nb) { InfoJoueur(Clan.entries[lire()], lire(), lire(), lire() != 0, lire() != 0) }
            val n = taille * taille
            val terrains = List(n) { lire().let { t -> if (t < 0) null else Terrain.entries[t] } }
            val visibilite = List(n) { Visibilite.entries[lire()] }
            val territoire = List(n) { lire() }
            val exploitations = List(n) { lire().let { e -> if (e < 0) null else Exploitation.entries[e] } }
            val ruines = List(n) { lire() != 0 }
            val unites = List(lire()) {
                Unite(
                    joueur = lire(), case = lire(), type = TypeUnite.entries[lire()], pv = lire(), pvMax = lire(),
                    pointsRestants = lire(), aBouge = lire() != 0, aAttaque = lire() != 0, peutAgir = lire() != 0,
                )
            }
            val villes = List(lire()) {
                Ville(case = lire(), joueur = lire(), capitale = lire() != 0, niveau = lire(), revenu = lire(), prixCroissance = lire(), batiments = lire(), rayon = lire())
            }
            val prix = ArrayList<Int>(); val ouvertes = ArrayList<Boolean>()
            repeat(15) { prix.add(lire()); ouvertes.add(lire() != 0) }
            val merveilles = List(3) { lire() != 0 }
            return Etat(
                taille = taille, tour = tour, joueurCourant = courant, finie = finie, limiteTours = limite,
                terrains = terrains, unites = unites, nbJoueurs = nb, visibilite = visibilite, villes = villes,
                gagnant = gagnant, seuil = seuil, joueur = joueur, vivres = vivres, revenu = revenu, entretien = entretien,
                nbMesUnites = nbMiennes, joueurs = joueurs, territoire = territoire, exploitations = exploitations, ruines = ruines,
                prixAvancees = prix, avanceesOuvertes = ouvertes, merveillesPrises = merveilles,
            )
        }
    }
}

/** Une partie tenue par le moteur Rust. À fermer par [fermer]. */
class Partie(val graine: Long, val nbJoueurs: Int = 2, clans: List<Clan> = Clan.entries.take(nbJoueurs)) {
    private var poignee: Long = Moteur.nouvellePartie(graine, nbJoueurs, clans.map { it.ordinal }.toIntArray())

    fun etat(joueur: Int): Etat = Etat.depuisBloc(Moteur.etat(vivante(), joueur))
    fun permises(): List<ActionPermise> = ActionPermise.depuisBloc(Moteur.permises(vivante()))
    fun appliquer(numero: Int): Boolean = Moteur.appliquer(vivante(), numero) == 0
    fun annonce(depart: Int, cible: Int): Annonce? =
        Moteur.annonce(vivante(), depart, cible).takeIf { it.size == 4 }?.let { Annonce(it[0], it[1], it[2] != 0, it[3] != 0) }
    fun tourIa(): IntArray = Moteur.tourIa(vivante())

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
