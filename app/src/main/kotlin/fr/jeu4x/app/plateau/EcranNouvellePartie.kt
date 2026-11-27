package fr.jeu4x.app.plateau

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringArrayResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import fr.jeu4x.app.R
import fr.jeu4x.app.moteur.Clan

/** Écran de départ provisoire : nombre de joueurs et clan, les autres places à l'ordinateur. */
@Composable
fun EcranNouvellePartie(reglage: Reglage, surChoix: (Reglage) -> Unit, surCommencer: () -> Unit) {
    val noms = stringArrayResource(R.array.noms_clans)
    val resumes = stringArrayResource(R.array.clans_resumes)
    Column(
        Modifier.fillMaxSize().background(Color(0xFFF6F3EC)).verticalScroll(rememberScrollState()).padding(20.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text(stringResource(R.string.reglage_titre), fontSize = 24.sp, fontWeight = FontWeight.Bold)
        Text(stringResource(R.string.reglage_joueurs), fontSize = 16.sp, fontWeight = FontWeight.Bold)
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            for (n in 2..4) Pastille("$n", reglage.nbJoueurs == n, Modifier.testTag("joueurs-$n")) { surChoix(reglage.copy(nbJoueurs = n)) }
        }
        Text(stringResource(R.string.reglage_clan), fontSize = 16.sp, fontWeight = FontWeight.Bold)
        for (c in Clan.entries) {
            val choisi = reglage.clan == c
            Column(
                Modifier.fillMaxWidth()
                    .background(if (choisi) Color(0xFFDCE8F8) else Color.White, RoundedCornerShape(12.dp))
                    .border(2.dp, if (choisi) Color(0xFF2F6FD0) else Color(0xFFDDDDDD), RoundedCornerShape(12.dp))
                    .clickable { surChoix(reglage.copy(clan = c)) }
                    .padding(12.dp),
            ) {
                Text(noms[c.ordinal], fontWeight = FontWeight.Bold, fontSize = 15.sp)
                Text(resumes[c.ordinal], fontSize = 13.sp, color = Color(0xFF444444))
            }
        }
        Text(stringResource(R.string.reglage_adversaires), fontSize = 13.sp, color = Color(0xFF444444))
        Button(onClick = surCommencer, modifier = Modifier.fillMaxWidth().height(52.dp).testTag("commencer")) {
            Text(stringResource(R.string.reglage_commencer), fontSize = 17.sp, fontWeight = FontWeight.Bold)
        }
    }
}

@Composable
private fun Pastille(texte: String, choisie: Boolean, modifier: Modifier, surToucher: () -> Unit) {
    Column(
        modifier.background(if (choisie) Color(0xFF2F6FD0) else Color.White, RoundedCornerShape(24.dp))
            .border(2.dp, Color(0xFF2F6FD0), RoundedCornerShape(24.dp))
            .clickable(onClick = surToucher).padding(horizontal = 22.dp, vertical = 10.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(texte, color = if (choisie) Color.White else Color(0xFF2F6FD0), fontSize = 18.sp, fontWeight = FontWeight.Bold)
    }
}
