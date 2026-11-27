package fr.jeu4x.app.moteur

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

/** Test de la frontière Kotlin ↔ Rust, sur la bibliothèque compilée pour le PC. */
class MoteurTest {
    @Test
    fun `la version des regles traverse le pont`() {
        assertEquals(1, Moteur.versionRegles())
    }

    @Test
    fun `l'auto-controle donne la meme empreinte que cote Rust`() {
        assertTrue(Moteur.controleReussi())
    }

    @Test
    fun `une autre graine donne une autre empreinte`() {
        assertTrue(Moteur.autoControle(43, 1000) != Moteur.EMPREINTE_ATTENDUE)
    }
}
