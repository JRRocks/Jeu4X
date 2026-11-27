package fr.jeu4x.app.plateau

import fr.jeu4x.app.moteur.ActionPermise
import fr.jeu4x.app.moteur.Clan
import fr.jeu4x.app.moteur.TypeUnite
import fr.jeu4x.app.moteur.Visibilite
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

/** Écran de partie branché sur le vrai moteur Rust (bibliothèque compilée pour le PC). */
class PartieViewModelTest {
    private val vm = PartieViewModel({ 7L }, immediat = true).apply {
        choisirReglage(Reglage(2, Clan.VENTS_ERRANTS))
        commencer()
    }

    @Test
    fun `la partie commence au tour du joueur humain, l'ordinateur ayant joue avant lui`() {
        val a = vm.affichage
        assertTrue(vm.enPartie)
        assertTrue(a.tourHumain)
        assertEquals(Clan.VENTS_ERRANTS, a.etat.joueurs[a.humain].clan)
        assertTrue(a.permises.any { it.genre == ActionPermise.FIN })
    }

    @Test
    fun `toucher une unite montre sa portee, puis la deplacer`() {
        val e = vm.affichage.etat
        val mienne = e.unites.first { it.joueur == vm.affichage.humain && it.type == TypeUnite.LANCIER }
        vm.toucherCase(mienne.case)
        assertEquals(mienne.case, vm.affichage.choisie)
        assertTrue(vm.affichage.portee.isNotEmpty())
        val cible = vm.affichage.portee.first()
        vm.toucherCase(cible)
        assertNotNull(vm.affichage.etat.uniteSur(cible))
    }

    @Test
    fun `toucher sa capitale ouvre la ville`() {
        val cap = vm.affichage.etat.villes.first { it.joueur == vm.affichage.humain && it.capitale }
        vm.toucherCase(cap.case)
        if (vm.affichage.choisie == cap.case) vm.toucherCase(cap.case)
        assertEquals(cap.case, vm.affichage.villeChoisie)
    }

    @Test
    fun `le brouillard cache ce que le joueur ne sait pas`() {
        val e = vm.affichage.etat
        assertTrue(e.visibilite.contains(Visibilite.INCONNUE))
        for (c in 0 until e.taille * e.taille) assertEquals(e.visibilite[c] == Visibilite.INCONNUE, e.terrains[c] == null)
        assertTrue(e.unites.all { it.joueur == vm.affichage.humain || e.visibilite[it.case] == Visibilite.VISIBLE })
    }

    @Test
    fun `fin du tour, l ordinateur joue, puis c est de nouveau a nous`() {
        val tour = vm.affichage.etat.tour
        vm.finDuTour()
        assertTrue(vm.affichage.tourHumain || vm.affichage.etat.finie)
        assertTrue(vm.affichage.etat.tour > tour || vm.affichage.etat.finie)
        assertNull(vm.affichage.choisie)
    }

    @Test
    fun `une partie entiere va jusqu'a la fin`() {
        var n = 0
        while (!vm.affichage.etat.finie && n++ < 200) vm.jouerAMaPlace()
        assertTrue(vm.affichage.etat.finie)
        vm.nouvellePartie()
        vm.commencer()
        assertTrue(vm.affichage.tourHumain)
    }
}
