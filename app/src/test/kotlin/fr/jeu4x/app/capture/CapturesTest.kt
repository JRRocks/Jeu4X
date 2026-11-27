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
import fr.jeu4x.app.moteur.ActionPermise
import fr.jeu4x.app.moteur.Clan
import fr.jeu4x.app.moteur.TypeUnite
import fr.jeu4x.app.plateau.EcranNouvellePartie
import fr.jeu4x.app.plateau.Reglage
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
    fun partie() {
        val vm = PartieViewModel({ 7L }, immediat = true)
        compose.setContent {
            MaterialTheme {
                if (vm.enPartie) EcranPartie(vm.affichage, vm) else EcranNouvellePartie(vm.reglage, vm::choisirReglage, vm::commencer)
            }
        }
        vm.choisirReglage(Reglage(3, Clan.MURAILLIERS))
        compose.waitForIdle()
        capturer("reglage")
        vm.commencer()
        compose.onNodeWithText("Fin du tour").assertExists()
        capturer("plateau")
        // Une unité choisie : pastilles blanches pour aller.
        val h = vm.affichage.humain
        vm.toucherCase(vm.affichage.etat.unites.first { it.joueur == h && it.type == TypeUnite.LANCIER }.case)
        capturer("plateau-choix")
        // Sa capitale : produire, grandir, bâtir.
        val cap = vm.affichage.etat.villes.first { it.joueur == h && it.capitale }.case
        vm.toucherCase(cap)
        if (vm.affichage.villeChoisie != cap) vm.toucherCase(cap)
        capturer("ville")
        vm.ouvrirArbre(true)
        capturer("arbre")
        vm.ouvrirArbre(false)
        // L'ordinateur joue à notre place jusqu'à ce qu'une attaque soit possible : annonce du combat.
        var n = 0
        while (!vm.affichage.etat.finie && n++ < 60) {
            val att = vm.affichage.permises.firstOrNull { it.genre == ActionPermise.ATTAQUER }
            if (att != null) {
                vm.toucherCase(att.a)
                vm.toucherCase(att.b)
                capturer("combat")
                vm.annulerAttaque()
                break
            }
            vm.jouerAMaPlace()
        }
        n = 0
        while (!vm.affichage.etat.finie && n++ < 200) vm.jouerAMaPlace()
        capturer("fin")
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
