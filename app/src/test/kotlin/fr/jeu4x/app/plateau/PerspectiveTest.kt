package fr.jeu4x.app.plateau

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
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

class CadrageTest {
    private val p = Perspective(taille = 11, demiLargeur = 44f)

    @Test
    fun `le lancement grossit deux fois et centre des unites au milieu du plateau`() {
        val unites = listOf(4 to 5, 5 to 5, 6 to 5)
        val (e, dx, dy) = p.cadrageSur(unites, 1080f, 1700f)
        assertEquals(p.echelleEnsemble(1080f, 1700f) * 2f, e, 1e-4f)
        val (cx, cy) = p.centre(5, 5)
        assertEquals(540f, cx * e + dx, 1f)
        assertEquals(850f, cy * e + dy, 1f)
    }

    @Test
    fun `au bord du plateau la vue glisse peu et les unites restent dans le tiers central`() {
        val unites = listOf(0 to 0, 1 to 0, 2 to 0)
        val (e, dx, dy) = p.cadrageSur(unites, 1080f, 1700f)
        val (cx, cy) = p.centre(1, 0)
        val x = cx * e + dx
        val y = cy * e + dy
        assertTrue(x in 360f..720f && y in 566f..1134f)
        // Le plateau couvre davantage la vue qu'avec un centrage strict.
        val l = p.limites()
        val vide = { d: Float, debut: Float, fin: Float, vue: Float -> maxOf(0f, debut * e + d) + maxOf(0f, vue - (fin * e + d)) }
        assertTrue(vide(dx, l[0], l[2], 1080f) + vide(dy, l[1], l[3], 1700f) < vide(540f - cx * e, l[0], l[2], 1080f) + vide(850f - cy * e, l[1], l[3], 1700f))
    }

    @Test
    fun `au changement de tour le zoom reste et la vue suit l'autre joueur`() {
        val rouges = listOf(10 to 10, 9 to 10, 8 to 10)
        val (dx, dy) = p.decalagePour(rouges, 1080f, 1700f, 3f)
        val (cx, cy) = p.centre(9, 10)
        assertTrue(cx * 3f + dx in 360f..720f && cy * 3f + dy in 566f..1134f)
    }
}
