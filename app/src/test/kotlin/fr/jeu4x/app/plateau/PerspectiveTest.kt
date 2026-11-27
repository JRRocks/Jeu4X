package fr.jeu4x.app.plateau

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class PerspectiveTest {
    private val p = Perspective(taille = 11, demiLargeur = 44f)

    @Test
    fun `le centre de chaque case retombe sur la meme case`() {
        for (x in 0 until 11) for (y in 0 until 11) {
            val (cx, cy) = p.centre(x, y)
            assertEquals(x to y, p.caseSous(cx, cy))
            // Un peu à côté du centre, toujours dans le losange.
            assertEquals(x to y, p.caseSous(cx + 20f, cy + 5f))
        }
    }

    @Test
    fun `hors du plateau rien n'est touche`() {
        val l = p.limites()
        assertNull(p.caseSous(l[0] - 5f, l[1] + 1f))
        assertNull(p.caseSous(0f, l[3] + 10f))
    }

    @Test
    fun `la rangee 0 est en bas de l'ecran`() {
        val (_, basY) = p.centre(5, 0)
        val (_, hautY) = p.centre(5, 10)
        assert(basY > hautY)
    }

    @Test
    fun `l'ordre de dessin va du fond vers l'avant`() {
        val ordre = p.ordreDeDessin()
        assertEquals(121, ordre.size)
        val profondeurs = ordre.map { (x, y) -> p.centre(x, y).second }
        assertEquals(profondeurs.sorted(), profondeurs)
    }
}
