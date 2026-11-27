package fr.jeu4x.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import fr.jeu4x.app.crash.CrashActivity
import fr.jeu4x.app.crash.CrashReporter
import fr.jeu4x.app.moteur.Moteur
import fr.jeu4x.app.plateau.EcranNouvellePartie
import fr.jeu4x.app.plateau.EcranPartie
import fr.jeu4x.app.plateau.PartieViewModel
import androidx.lifecycle.viewmodel.compose.viewModel

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        if (CrashReporter.hasPending(this)) {
            startActivity(CrashActivity.intent(this))
            finish()
            return
        }
        if (BuildConfig.PLANTAGE_DE_TEST && intent.getBooleanExtra(EXTRA_PLANTAGE_DE_TEST, false)) {
            // Vérification du rapport de plantage à l'émulateur (version d'essai seulement).
            throw IllegalStateException("Plantage de test demandé")
        }
        val moteur = lireMoteur()
        setContent {
            MaterialTheme {
                if (moteur is EtatMoteur.Ok) {
                    val vm: PartieViewModel = viewModel()
                    if (vm.enPartie) EcranPartie(vm.affichage, vm)
                    else EcranNouvellePartie(vm.reglage, vm::choisirReglage, vm::commencer)
                } else {
                    // Moteur absent ou faux : l'écran de contrôle dit pourquoi, plutôt qu'un plantage.
                    EcranAccueil(EtatAccueil(BuildConfig.VERSION_NAME, moteur))
                }
            }
        }
    }

    companion object {
        const val EXTRA_PLANTAGE_DE_TEST = "fr.jeu4x.app.PLANTAGE_DE_TEST"
    }

    private fun lireMoteur(): EtatMoteur = try {
        if (Moteur.controleReussi()) EtatMoteur.Ok(Moteur.versionRegles())
        else EtatMoteur.Erreur("empreinte différente du PC")
    } catch (e: UnsatisfiedLinkError) {
        EtatMoteur.Erreur("bibliothèque introuvable")
    }
}

sealed interface EtatMoteur {
    data class Ok(val version: Int) : EtatMoteur
    data class Erreur(val raison: String) : EtatMoteur
}

data class EtatAccueil(val version: String, val moteur: EtatMoteur)

/** Écran provisoire de l'étape B : vérifie l'installation et le pont vers le moteur. */
@Composable
fun EcranAccueil(etat: EtatAccueil) {
    Surface(Modifier.fillMaxSize()) {
        Column(
            Modifier.fillMaxSize().padding(24.dp),
            verticalArrangement = Arrangement.Center,
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Text(stringResource(R.string.app_name), style = MaterialTheme.typography.headlineMedium)
            Text(stringResource(R.string.accueil_version, etat.version))
            Text(
                when (val m = etat.moteur) {
                    is EtatMoteur.Ok -> stringResource(R.string.accueil_moteur_ok, m.version)
                    is EtatMoteur.Erreur -> stringResource(R.string.accueil_moteur_ko, m.raison)
                },
            )
        }
    }
}
