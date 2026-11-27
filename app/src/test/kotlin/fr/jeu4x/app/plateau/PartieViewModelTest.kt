package fr.jeu4x.app.plateau

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
        val adverse = vm.affichage.etat.unites.first { it.joueur == 1 }
        vm.toucherCase(adverse.case)
        assertNull(vm.affichage.choisie)
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
