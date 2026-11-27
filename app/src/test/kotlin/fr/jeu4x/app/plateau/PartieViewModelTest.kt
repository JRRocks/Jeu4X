package fr.jeu4x.app.plateau

import fr.jeu4x.app.moteur.TypeUnite
import fr.jeu4x.app.moteur.Visibilite

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

/** Écran de partie branché sur le vrai moteur Rust (bibliothèque compilée pour le PC). */
class PartieViewModelTest {
    private val vm = PartieViewModel { 7L }

    @Test
    fun `toucher une unite montre sa portee, puis la deplacer`() {
        val e = vm.affichage.etat
        assertEquals(1, e.tour)
        val mienne = e.unites.first { it.joueur == 0 }
        vm.toucherCase(mienne.case)
        assertEquals(mienne.case, vm.affichage.choisie)
        assertTrue(vm.affichage.portee.isNotEmpty())
        val cible = vm.affichage.portee.first()
        vm.toucherCase(cible)
        assertNotNull(vm.affichage.etat.uniteSur(cible))
        assertEquals(cible, vm.affichage.choisie)
    }

    @Test
    fun `une unite adverse ne se choisit pas`() {
        val bleues = vm.affichage.etat.unites.filter { it.joueur == 0 }
        vm.finDuTour()
        // Au tour du Rouge, les cases des unités bleues ne donnent rien à choisir.
        for (u in bleues) {
            vm.toucherCase(u.case)
            assertNull(vm.affichage.choisie)
        }
    }

    @Test
    fun `le brouillard cache ce que le joueur ne sait pas`() {
        val e = vm.affichage.etat
        assertEquals(2, e.nbJoueurs)
        assertTrue(e.visibilite.contains(Visibilite.INCONNUE))
        for (c in 0 until e.taille * e.taille) {
            assertEquals(e.visibilite[c] == Visibilite.INCONNUE, e.terrains[c] == null)
        }
        // Ses deux unités de départ (un Lancier, un Éclaireur) et sa capitale sont connues.
        val miennes = e.unites.filter { it.joueur == 0 }
        assertEquals(setOf(TypeUnite.LANCIER, TypeUnite.ECLAIREUR), miennes.map { it.type }.toSet())
        assertTrue(e.villes.any { it.joueur == 0 && it.capitale })
        // Toute unité affichée est sur une case visible.
        assertTrue(e.unites.all { e.visibilite[it.case] == Visibilite.VISIBLE })
    }

    @Test
    fun `fin du tour passe la main, puis le tour suivant`() {
        vm.finDuTour()
        assertEquals(1, vm.affichage.etat.joueurCourant)
        vm.finDuTour()
        assertEquals(0, vm.affichage.etat.joueurCourant)
        assertEquals(2, vm.affichage.etat.tour)
    }

    @Test
    fun `la partie se termine a la limite de tours`() {
        repeat(2 * vm.affichage.etat.limiteTours) { vm.finDuTour() }
        assertTrue(vm.affichage.etat.finie)
        vm.nouvellePartie()
        assertEquals(1, vm.affichage.etat.tour)
    }
}
