package fr.jeu4x.app.plateau

import android.content.res.AssetManager
import android.graphics.BitmapFactory
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.platform.LocalContext
import java.io.IOException

/**
 * Sprites 3D low poly de S6 (D-36, D-37), variante « dalles épaisses, facettes, trapus »,
 * rangés dans `assets/sprites/<jour|nuit>/` en 192 px. Chargés à la demande, gardés en mémoire.
 * Contrat (`art/README.md` sur `s6-direction-artistique`) : image 192 × 240, ancre au centre
 * du dessus de la case en (96 ; 163,2), losange de la case sur toute la largeur.
 */
class Sprites(private val assets: AssetManager, val palette: String) {
    private val cache = HashMap<String, ImageBitmap?>()

    operator fun get(nom: String): ImageBitmap? = cache.getOrPut(nom) {
        try {
            assets.open("sprites/$palette/$nom.png").use { BitmapFactory.decodeStream(it)?.asImageBitmap() }
        } catch (e: IOException) {
            null
        }
    }

    companion object {
        const val LARGEUR = 192f
        const val ANCRE_X = 96f
        const val ANCRE_Y = 163.2f
    }
}

@Composable
fun rememberSprites(nuit: Boolean): Sprites {
    val contexte = LocalContext.current
    return remember(nuit) { Sprites(contexte.assets, if (nuit) "nuit" else "jour") }
}
