package fr.jeu4x.app.capture

import android.graphics.Bitmap
import android.graphics.Canvas
import androidx.activity.ComponentActivity
import androidx.compose.material3.MaterialTheme
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import fr.jeu4x.app.EcranAccueil
import fr.jeu4x.app.EtatAccueil
import fr.jeu4x.app.EtatMoteur
import fr.jeu4x.app.moteur.TypeUnite
import fr.jeu4x.app.plateau.EcranPartie
import fr.jeu4x.app.plateau.PartieViewModel
import java.io.File
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import org.robolectric.annotation.GraphicsMode

/** Écrans en PNG sous app/build/captures/, pour les voir sans téléphone. */
@RunWith(RobolectricTestRunner::class)
@GraphicsMode(GraphicsMode.Mode.NATIVE)
@Config(sdk = [35], qualifiers = "w360dp-h800dp-xxhdpi", application = android.app.Application::class)
class CapturesTest {
    @get:Rule val compose = createAndroidComposeRule<ComponentActivity>()

    @Test
    fun accueil() {
        compose.setContent { MaterialTheme { EcranAccueil(EtatAccueil("0.1.1", EtatMoteur.Ok(1))) } }
        compose.onNodeWithText("Moteur de règles 1 : contrôle réussi").assertExists()
        capturer("accueil")
    }

    @Test
    fun plateau() {
        val vm = PartieViewModel { 7L }
        compose.setContent { MaterialTheme { EcranPartie(vm.affichage, vm::toucherCase, vm::finDuTour, vm::nouvellePartie) } }
        compose.onNodeWithText("Fin du tour").assertExists()
        capturer("plateau")
        // Une unité choisie : sa portée en jaune.
        vm.toucherCase(vm.affichage.etat.unites.first { it.joueur == 0 }.case)
        compose.waitForIdle()
        capturer("plateau-choix")
        // L'éclaireur part au plus loin : les cases qu'il quitte passent à « vue autrefois ».
        val e = vm.affichage.etat
        val eclaireur = e.unites.first { it.joueur == 0 && it.type == TypeUnite.ECLAIREUR }
        vm.toucherCase(eclaireur.case)
        val loin = vm.affichage.portee.maxBy { c ->
            maxOf(kotlin.math.abs(e.colonne(c) - e.colonne(eclaireur.case)), kotlin.math.abs(e.rangee(c) - e.rangee(eclaireur.case))) * 1000 - c
        }
        vm.toucherCase(loin)
        compose.waitForIdle()
        capturer("plateau-souvenir")
        // Tour du joueur Rouge : la vue glisse vers ses unités (D-21) et montre ce qu'il sait.
        vm.finDuTour()
        compose.waitForIdle()
        capturer("plateau-tour-rouge")
    }

    private fun capturer(nom: String) {
        compose.waitForIdle()
        // captureToImage() attend un rafraîchissement que Robolectric ne donne pas : on dessine la vue.
        val vue = compose.activity.window.decorView
        val image = Bitmap.createBitmap(vue.width, vue.height, Bitmap.Config.ARGB_8888)
        vue.draw(Canvas(image))
        val dossier = File("build/captures").apply { mkdirs() }
        File(dossier, "$nom.png").outputStream().use { image.compress(Bitmap.CompressFormat.PNG, 100, it) }
    }
}
