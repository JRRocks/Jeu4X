package fr.jeu4x.app.plateau

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import fr.jeu4x.app.moteur.ActionPermise
import fr.jeu4x.app.moteur.Annonce
import fr.jeu4x.app.moteur.Clan
import fr.jeu4x.app.moteur.Etat
import fr.jeu4x.app.moteur.Partie
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

/** Choix d'une nouvelle partie : nombre de joueurs et clan du joueur humain. */
data class Reglage(val nbJoueurs: Int = 2, val clan: Clan = Clan.CROCS_ROUGES)

/**
 * État affiché : ce que voit le joueur humain, ce qu'il peut faire, et ce qu'il a choisi
 * (une unité, une ville ou une case de son territoire, une attaque en attente).
 */
data class Affichage(
    val etat: Etat,
    val choisie: Int? = null,
    val portee: Set<Int> = emptySet(),
    val numeroPartie: Int = 0,
    val humain: Int = 0,
    val permises: List<ActionPermise> = emptyList(),
    val cibles: Set<Int> = emptySet(),
    val villeChoisie: Int? = null,
    val caseChoisie: Int? = null,
    val attaque: Pair<Int, Annonce>? = null,
    val arbreOuvert: Boolean = false,
    val iaJoue: Boolean = false,
    val graine: Long = 0,
) {
    val tourHumain get() = !etat.finie && etat.joueurCourant == humain && !iaJoue
    fun permise(genre: Int, a: Int = -1, b: Int = -1) =
        permises.firstOrNull { it.genre == genre && (a < 0 || it.a == a) && (b < 0 || it.b == b) }
}

/**
 * Logique d'écran d'une partie contre l'ordinateur. Les règles restent dans le moteur
 * Rust : ici, seulement les choix du joueur, l'envoi des actions et le tour de l'IA.
 * `immediat` : l'IA joue sans pause (tests).
 */
class PartieViewModel(
    private val nouvelleGraine: () -> Long = { System.nanoTime() and 0x7FFF_FFFF },
    private val immediat: Boolean = false,
) : ViewModel() {
    private var partie: Partie? = null
    private var numero = 0
    var reglage by mutableStateOf(Reglage())
        private set
    var enPartie by mutableStateOf(false)
        private set
    var affichage by mutableStateOf(Affichage(Etat(11, 1, 0, false, 80, List(121) { null }, emptyList())))
        private set

    fun choisirReglage(r: Reglage) {
        reglage = r
    }

    /** Lance une partie avec le réglage choisi. La place du joueur humain est tirée de la graine. */
    fun commencer() {
        partie?.fermer()
        val g = nouvelleGraine()
        val nb = reglage.nbJoueurs
        val humain = (g % nb).toInt().let { if (it < 0) it + nb else it }
        val autres = Clan.entries.filter { it != reglage.clan }
        var k = 0
        val clans = List(nb) { j -> if (j == humain) reglage.clan else autres[k++ % autres.size] }
        val p = Partie(g, nb, clans)
        partie = p
        numero++
        affichage = Affichage(p.etat(humain), numeroPartie = numero, humain = humain, graine = g)
        enPartie = true
        rafraichir()
        jouerIa()
    }

    /** Retour à l'écran de réglage. */
    fun nouvellePartie() {
        enPartie = false
    }

    private fun rafraichir(garder: Boolean = true) {
        val p = partie ?: return
        val a = affichage
        val etat = p.etat(a.humain)
        val permises = if (!etat.finie && etat.joueurCourant == a.humain) p.permises() else emptyList()
        var nouveau = a.copy(etat = etat, permises = permises, attaque = null)
        if (!garder) nouveau = nouveau.copy(choisie = null, villeChoisie = null, caseChoisie = null, portee = emptySet(), cibles = emptySet())
        affichage = nouveau.choisie?.let { c -> if (etat.uniteSur(c)?.joueur == a.humain) choisirUnite(nouveau, c) else nouveau.copy(choisie = null, portee = emptySet(), cibles = emptySet()) } ?: nouveau
    }

    private fun choisirUnite(a: Affichage, case: Int) = a.copy(
        choisie = case,
        villeChoisie = null,
        caseChoisie = null,
        portee = a.permises.filter { it.genre == ActionPermise.DEPLACER && it.a == case }.map { it.b }.toSet(),
        cibles = a.permises.filter { it.genre == ActionPermise.ATTAQUER && it.a == case }.map { it.b }.toSet(),
    )

    private fun faire(action: ActionPermise?): Boolean {
        val p = partie ?: return false
        if (action == null || !affichage.tourHumain) return false
        val ok = p.appliquer(action.numero)
        rafraichir()
        return ok
    }

    fun toucherCase(case: Int) {
        val a = affichage
        if (!a.tourHumain) return
        val e = a.etat
        val choisie = a.choisie
        if (a.attaque != null) {
            affichage = a.copy(attaque = null)
            return
        }
        if (choisie != null) {
            if (case in a.cibles) {
                partie?.annonce(choisie, case)?.let { affichage = a.copy(attaque = case to it) }
                return
            }
            if (case in a.portee) {
                faire(a.permise(ActionPermise.DEPLACER, choisie, case))
                // L'unité peut s'être arrêtée en chemin (surprise, ruine) : on garde le choix si elle est là.
                val apres = affichage
                affichage = if (apres.etat.uniteSur(case)?.joueur == apres.humain) choisirUnite(apres, case)
                else apres.copy(choisie = null, portee = emptySet(), cibles = emptySet())
                return
            }
            if (case == choisie) {
                // Deuxième toucher sur l'unité : la ville sous elle, s'il y en a une à soi.
                affichage = if (e.villeSur(case)?.joueur == a.humain) a.copy(choisie = null, portee = emptySet(), cibles = emptySet(), villeChoisie = case)
                else a.copy(choisie = null, portee = emptySet(), cibles = emptySet())
                return
            }
        }
        val u = e.uniteSur(case)
        val v = e.villeSur(case)
        affichage = when {
            u != null && u.joueur == a.humain -> choisirUnite(a, case)
            v != null -> a.copy(choisie = null, portee = emptySet(), cibles = emptySet(), villeChoisie = case, caseChoisie = null)
            a.permise(ActionPermise.EXPLOITER, case) != null -> a.copy(choisie = null, portee = emptySet(), cibles = emptySet(), villeChoisie = null, caseChoisie = case)
            else -> a.copy(choisie = null, portee = emptySet(), cibles = emptySet(), villeChoisie = null, caseChoisie = null)
        }
    }

    fun annulerAttaque() {
        affichage = affichage.copy(attaque = null)
    }

    fun confirmerAttaque() {
        val a = affichage
        val (cible, _) = a.attaque ?: return
        val depart = a.choisie ?: return
        faire(a.permise(ActionPermise.ATTAQUER, depart, cible))
    }

    fun rallier() {
        val c = affichage.choisie ?: return
        if (faire(affichage.permise(ActionPermise.RALLIER, c))) affichage = affichage.copy(villeChoisie = c)
        jouerIa()
    }

    fun renvoyer() {
        val c = affichage.choisie ?: return
        faire(affichage.permise(ActionPermise.RENVOYER, c))
        affichage = affichage.copy(choisie = null, portee = emptySet(), cibles = emptySet())
    }

    fun produire(type: Int) {
        val v = affichage.villeChoisie ?: return
        faire(affichage.permise(ActionPermise.PRODUIRE, v, type))
    }

    fun agrandir(cadeauB: Boolean) {
        val v = affichage.villeChoisie ?: return
        faire(affichage.permise(ActionPermise.AGRANDIR, v, if (cadeauB) 1 else 0))
    }

    fun batir(batiment: Int) {
        val v = affichage.villeChoisie ?: return
        faire(affichage.permise(ActionPermise.BATIR, v, batiment))
    }

    fun exploiter() {
        val c = affichage.caseChoisie ?: return
        faire(affichage.permise(ActionPermise.EXPLOITER, c))
        affichage = affichage.copy(caseChoisie = null)
    }

    fun rechercher(avancee: Int) {
        faire(affichage.permise(ActionPermise.RECHERCHER, avancee))
    }

    fun ouvrirArbre(ouvert: Boolean) {
        affichage = affichage.copy(arbreOuvert = ouvert)
    }

    fun finDuTour() {
        if (!affichage.tourHumain) return
        faire(affichage.permise(ActionPermise.FIN))
        rafraichir(garder = false)
        jouerIa()
    }

    /** Débogage et captures : l'ordinateur joue le tour du joueur humain à sa place. */
    internal fun jouerAMaPlace() {
        val p = partie ?: return
        if (!affichage.tourHumain) return
        p.tourIa().forEach { p.appliquer(it) }
        rafraichir(garder = false)
        jouerIa()
    }

    /** Fait jouer l'ordinateur tant que ce n'est pas au joueur humain, action par action. */
    private fun jouerIa() {
        val p = partie ?: return
        if (affichage.etat.finie || affichage.etat.joueurCourant == affichage.humain) return
        if (immediat) {
            while (!affichage.etat.finie && affichage.etat.joueurCourant != affichage.humain) {
                val nums = p.tourIa()
                if (nums.isEmpty()) break
                nums.forEach { p.appliquer(it) }
                rafraichir(garder = false)
            }
            return
        }
        val numeroLance = numero
        affichage = affichage.copy(iaJoue = true)
        viewModelScope.launch {
            while (numeroLance == numero && !affichage.etat.finie && affichage.etat.joueurCourant != affichage.humain) {
                val nums = p.tourIa()
                if (nums.isEmpty()) break
                for (n in nums) {
                    if (numeroLance != numero) return@launch
                    val avant = affichage.etat
                    p.appliquer(n)
                    rafraichir(garder = false)
                    affichage = affichage.copy(iaJoue = true)
                    // Une pause seulement quand quelque chose se voit (unité ou ville adverse en vue).
                    if (affichage.etat.unites != avant.unites || affichage.etat.villes != avant.villes) delay(220)
                }
            }
            if (numeroLance == numero) {
                affichage = affichage.copy(iaJoue = false)
                rafraichir(garder = false)
            }
        }
    }

    override fun onCleared() {
        partie?.fermer()
    }
}
