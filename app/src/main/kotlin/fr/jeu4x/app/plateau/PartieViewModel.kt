package fr.jeu4x.app.plateau

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import fr.jeu4x.app.moteur.Partie

/**
 * Logique d'écran d'une partie à deux joueurs sur le même téléphone. Les règles restent
 * dans le moteur Rust : ici, seulement le choix d'une unité et l'envoi des actions.
 */
class PartieViewModel(private val nouvelleGraine: () -> Long = { System.nanoTime() }) : ViewModel() {
    private var partie = Partie(nouvelleGraine())
    var affichage by mutableStateOf(Affichage(partie.etat()))
        private set

    fun toucherCase(case: Int) {
        val a = affichage
        val etat = a.etat
        if (etat.finie) return
        val choisie = a.choisie
        if (choisie != null && case in a.portee) {
            if (partie.deplacer(choisie, case)) {
                val apres = partie.etat()
                affichage = Affichage(apres, choisie = case, portee = partie.atteignables(case))
            }
            return
        }
        val u = etat.uniteSur(case)
        affichage = if (u != null && u.joueur == etat.joueurCourant) {
            Affichage(etat, choisie = case, portee = partie.atteignables(case))
        } else {
            Affichage(etat)
        }
    }

    fun finDuTour() {
        if (partie.finDuTour()) affichage = Affichage(partie.etat())
    }

    fun nouvellePartie() {
        partie.fermer()
        partie = Partie(nouvelleGraine())
        affichage = Affichage(partie.etat())
    }

    override fun onCleared() {
        partie.fermer()
    }
}
