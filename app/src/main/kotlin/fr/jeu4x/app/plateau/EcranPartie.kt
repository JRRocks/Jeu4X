package fr.jeu4x.app.plateau

import androidx.compose.animation.core.animate
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.gestures.detectTransformGestures
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.drawscope.withTransform
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import fr.jeu4x.app.R
import fr.jeu4x.app.moteur.Etat
import fr.jeu4x.app.moteur.Terrain
import fr.jeu4x.app.moteur.TypeUnite
import fr.jeu4x.app.moteur.Visibilite

/** Couleurs provisoires, reprises de l'aperçu C (le style définitif revient à la session 6). */
private object Teintes {
    val fond = Color(0xFF3C5A3A)
    val barre = Color(0xFF2B2B2B)
    val plaine = Color(0xFFB9D98C)
    val foret = Color(0xFF5F9A55)
    val sapin = Color(0xFF3D6E3A)
    val colline = Color(0xFFA7BF72)
    val collineBosse = Color(0xFF8AA35A)
    val montagne = Color(0xFFA59E93)
    val montagneFlancG = Color(0xFF857E74)
    val montagneFlancD = Color(0xFF6F695F)
    val roche = Color(0xFF7B746A)
    val neige = Color(0xFFF2F0EC)
    val eau = Color(0xFF6FA8D6)
    val vague = Color(0xFFE8F3FB)
    val bord = Color(0x22000000)
    val portee = Color(0x55FFFFFF)
    val choix = Color(0xFFFFD34D)
    val inconnue = Color(0xFF2A3A29)
    val souvenir = Color(0x66000000)
    val bleu = Color(0xFF2F6FD0)
    val rouge = Color(0xFFD0482F)
    val violet = Color(0xFF8E4FC0)
    val jaune = Color(0xFFD9A21B)
    val ombre = Color(0x33000000)
    val info = Color(0xFFFFFFFF)
    val creme = Color(0xFFF6F3EC)
}

fun couleurJoueur(joueur: Int) = when (joueur) {
    0 -> Teintes.bleu
    1 -> Teintes.rouge
    2 -> Teintes.violet
    else -> Teintes.jaune
}

private fun nomCouleur(joueur: Int) = when (joueur) {
    0 -> R.string.joueur_bleu
    1 -> R.string.joueur_rouge
    2 -> R.string.joueur_violet
    else -> R.string.joueur_jaune
}

/** État affiché : la partie, l'unité choisie et ses cases atteignables. */
data class Affichage(val etat: Etat, val choisie: Int? = null, val portee: Set<Int> = emptySet(), val numeroPartie: Int = 0)

@Composable
fun EcranPartie(
    affichage: Affichage,
    surToucherCase: (Int) -> Unit,
    surFinDuTour: () -> Unit,
    surNouvellePartie: () -> Unit,
) {
    val etat = affichage.etat
    val nomJoueur = stringResource(nomCouleur(etat.joueurCourant))
    Column(Modifier.fillMaxSize().background(Teintes.creme)) {
        Row(
            Modifier.fillMaxWidth().height(56.dp).background(Teintes.barre).padding(horizontal = 16.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(stringResource(R.string.partie_tour, etat.tour, etat.limiteTours), color = Color.White, fontSize = 16.sp)
            Text(
                stringResource(R.string.partie_joueur, nomJoueur),
                color = couleurJoueur(etat.joueurCourant).copy(alpha = 1f),
                fontSize = 16.sp,
                fontWeight = FontWeight.Bold,
            )
        }
        Box(Modifier.weight(1f).fillMaxWidth()) {
            Plateau(affichage, surToucherCase)
        }
        Text(
            when {
                etat.finie -> stringResource(R.string.partie_finie)
                affichage.choisie != null && affichage.portee.isEmpty() -> stringResource(R.string.partie_unite_epuisee)
                affichage.choisie != null -> stringResource(R.string.partie_choisir_case)
                else -> stringResource(R.string.partie_choisir_unite)
            },
            Modifier.fillMaxWidth().background(Color.White).padding(horizontal = 16.dp, vertical = 10.dp),
            fontSize = 13.sp,
            color = Color(0xFF333333),
        )
        Button(
            onClick = if (etat.finie) surNouvellePartie else surFinDuTour,
            modifier = Modifier.fillMaxWidth().padding(16.dp).height(48.dp).testTag("bouton-tour"),
            shape = RoundedCornerShape(24.dp),
            colors = ButtonDefaults.buttonColors(containerColor = couleurJoueur(etat.joueurCourant)),
        ) {
            Text(
                stringResource(if (etat.finie) R.string.partie_nouvelle else R.string.partie_fin_du_tour),
                fontSize = 16.sp,
                fontWeight = FontWeight.Bold,
            )
        }
    }
}

@Composable
private fun Plateau(affichage: Affichage, surToucherCase: (Int) -> Unit) {
    val etat = affichage.etat
    val perspective = remember(etat.taille) { Perspective(etat.taille, demiLargeur = 44f) }
    // Remis à zéro à chaque nouvelle partie : le cadrage de lancement se refait.
    var echelle by remember(affichage.numeroPartie) { mutableFloatStateOf(0f) } // 0 : pas encore cadré
    var decalage by remember(affichage.numeroPartie) { mutableStateOf(Offset.Zero) }
    var tailleVue by remember { mutableStateOf(IntSize.Zero) }

    // Lancement d'une partie (D-20) : vue zoomée sur les unités du joueur qui commence.
    LaunchedEffect(affichage.numeroPartie, tailleVue) {
        if (echelle == 0f && tailleVue.width > 0 && tailleVue.height > 0) {
            val miennes = etat.unites.filter { it.joueur == etat.joueurCourant }
                .map { etat.colonne(it.case) to etat.rangee(it.case) }
            val (e, dx, dy) = perspective.cadrageSur(miennes, tailleVue.width.toFloat(), tailleVue.height.toFloat())
            echelle = e
            decalage = Offset(dx, dy)
        }
    }

    // Début de chaque tour (D-21) : la vue glisse en une demi-seconde vers les unités du
    // joueur dont c'est le tour, au zoom choisi. Rien au premier cadrage (déjà centré).
    LaunchedEffect(affichage.numeroPartie, etat.tour, etat.joueurCourant) {
        if (echelle == 0f || tailleVue.width == 0) return@LaunchedEffect
        val miennes = etat.unites.filter { it.joueur == etat.joueurCourant }
            .map { etat.colonne(it.case) to etat.rangee(it.case) }
        val (cx, cy) = perspective.decalagePour(miennes, tailleVue.width.toFloat(), tailleVue.height.toFloat(), echelle)
        val depart = decalage
        animate(0f, 1f, animationSpec = tween(durationMillis = 500)) { t, _ ->
            decalage = Offset(depart.x + (cx - depart.x) * t, depart.y + (cy - depart.y) * t)
        }
    }

    Canvas(
        Modifier.fillMaxSize().background(Teintes.fond).testTag("plateau")
            .onSizeChanged { tailleVue = it }
            .pointerInput(perspective) {
                detectTransformGestures { centroide, deplacement, zoom, _ ->
                    val nouvelle = (echelle * zoom).coerceIn(0.4f, 4f)
                    // Le point sous les doigts reste sous les doigts.
                    decalage = centroide - (centroide - decalage) * (nouvelle / echelle) + deplacement
                    echelle = nouvelle
                }
            }
            .pointerInput(perspective, etat) {
                detectTapGestures { point ->
                    if (echelle == 0f) return@detectTapGestures
                    val p = (point - decalage) / echelle
                    perspective.caseSous(p.x, p.y)?.let { (x, y) -> surToucherCase(y * etat.taille + x) }
                }
            },
    ) {
        if (echelle == 0f) return@Canvas
        withTransform({
            translate(decalage.x, decalage.y)
            scale(echelle, echelle, Offset.Zero)
        }) {
            dessinerPlateau(perspective, affichage)
        }
    }
}

private fun losange(cx: Float, haut: Float, dl: Float, dh: Float, retrait: Float = 0f) = Path().apply {
    moveTo(cx, haut + retrait)
    lineTo(cx + dl - 2 * retrait, haut + dh)
    lineTo(cx, haut + 2 * dh - retrait)
    lineTo(cx - dl + 2 * retrait, haut + dh)
    close()
}

private fun DrawScope.dessinerPlateau(p: Perspective, a: Affichage) {
    val etat = a.etat
    val dl = p.demiLargeur
    val dh = p.demiHauteur
    val relief = 7f
    for ((x, y) in p.ordreDeDessin()) {
        val case = y * etat.taille + x
        val (sx, sy0) = p.sommet(x, y)
        val terrain = etat.terrains[case]
        // Case inconnue (brouillard) : une dalle sombre, rien de ce qu'elle contient.
        if (terrain == null || etat.visibilite[case] == Visibilite.INCONNUE) {
            val forme = losange(sx, sy0, dl, dh)
            drawPath(forme, Teintes.inconnue)
            drawPath(forme, Teintes.bord, style = Stroke(1f))
            continue
        }
        val souvenir = etat.visibilite[case] == Visibilite.VUE_AUTREFOIS
        val haut = if (terrain == Terrain.MONTAGNE) sy0 - relief else sy0
        if (terrain == Terrain.MONTAGNE) {
            val flancs = listOf(
                Path().apply { moveTo(sx - dl, haut + dh); lineTo(sx, haut + 2 * dh); lineTo(sx, sy0 + 2 * dh); lineTo(sx - dl, sy0 + dh); close() } to Teintes.montagneFlancG,
                Path().apply { moveTo(sx + dl, haut + dh); lineTo(sx, haut + 2 * dh); lineTo(sx, sy0 + 2 * dh); lineTo(sx + dl, sy0 + dh); close() } to Teintes.montagneFlancD,
            )
            for ((chemin, teinte) in flancs) {
                drawPath(chemin, teinte)
                if (souvenir) drawPath(chemin, Teintes.souvenir)
            }
        }
        val forme = losange(sx, haut, dl, dh)
        drawPath(forme, couleurTerrain(terrain))
        drawPath(forme, Teintes.bord, style = Stroke(1f))
        decor(terrain, sx, haut + dh)
        etat.villeSur(case)?.let { dessinerVille(sx, haut + dh, if (it.joueur >= 0) couleurJoueur(it.joueur) else Color.White) }
        // Vue autrefois : le terrain et les villes tels qu'on s'en souvient, assombris.
        if (souvenir) drawPath(forme, Teintes.souvenir)
        if (case in a.portee) {
            val l = losange(sx, haut, dl, dh, retrait = 3f)
            drawPath(l, Teintes.portee)
            drawPath(l, Teintes.choix, style = Stroke(2.5f))
        }
    }
    // Les unités après le terrain, dans le même ordre, pour rester par-dessus.
    for ((x, y) in p.ordreDeDessin()) {
        val case = y * etat.taille + x
        val u = etat.uniteSur(case) ?: continue
        val (cx, cy) = p.centre(x, y)
        val haut = if (etat.terrains[case] == Terrain.MONTAGNE) relief else 0f
        dessinerUnite(cx, cy - 14f - haut, couleurJoueur(u.joueur), u.type, choisie = a.choisie == case, epuisee = u.pointsRestants == 0)
    }
}

/** Ville, provisoire : une maison à la couleur de son propriétaire. */
private fun DrawScope.dessinerVille(cx: Float, cy: Float, couleur: Color) {
    drawRect(Teintes.ombre, topLeft = Offset(cx - 9f, cy - 4f), size = androidx.compose.ui.geometry.Size(20f, 12f))
    drawRect(couleur, topLeft = Offset(cx - 10f, cy - 6f), size = androidx.compose.ui.geometry.Size(20f, 12f))
    drawPath(Path().apply { moveTo(cx - 13f, cy - 6f); lineTo(cx, cy - 17f); lineTo(cx + 13f, cy - 6f); close() }, Teintes.creme)
    drawRect(Teintes.creme, topLeft = Offset(cx - 3f, cy - 1f), size = androidx.compose.ui.geometry.Size(6f, 7f))
}

private fun couleurTerrain(t: Terrain) = when (t) {
    Terrain.PLAINE -> Teintes.plaine
    Terrain.FORET -> Teintes.foret
    Terrain.COLLINE -> Teintes.colline
    Terrain.MONTAGNE -> Teintes.montagne
    Terrain.EAU -> Teintes.eau
}

private fun DrawScope.decor(t: Terrain, cx: Float, cy: Float) {
    when (t) {
        Terrain.FORET -> for ((dx, dy) in listOf(-9f to -2f, 9f to 3f)) {
            drawPath(Path().apply {
                moveTo(cx + dx - 6f, cy + dy + 5f); lineTo(cx + dx, cy + dy - 8f); lineTo(cx + dx + 6f, cy + dy + 5f); close()
            }, Teintes.sapin)
        }
        Terrain.COLLINE -> for ((dx, dy, r) in listOf(Triple(-7f, 3f, 9f), Triple(7f, -1f, 7f))) {
            drawArc(
                Teintes.collineBosse, startAngle = 180f, sweepAngle = 180f, useCenter = true,
                topLeft = Offset(cx + dx - r, cy + dy - r * 0.8f), size = androidx.compose.ui.geometry.Size(2 * r, 1.6f * r),
            )
        }
        Terrain.MONTAGNE -> {
            drawPath(Path().apply { moveTo(cx - 14f, cy + 6f); lineTo(cx, cy - 14f); lineTo(cx + 14f, cy + 6f); close() }, Teintes.roche)
            drawPath(Path().apply { moveTo(cx - 4f, cy - 8f); lineTo(cx, cy - 14f); lineTo(cx + 4f, cy - 8f); close() }, Teintes.neige)
        }
        Terrain.EAU -> for (o in listOf(-4f, 5f)) {
            drawPath(Path().apply {
                moveTo(cx - 12f, cy + o); quadraticTo(cx - 6f, cy + o - 4f, cx, cy + o); quadraticTo(cx + 6f, cy + o + 4f, cx + 12f, cy + o)
            }, Teintes.vague, style = Stroke(1.8f))
        }
        Terrain.PLAINE -> Unit
    }
}

private fun DrawScope.dessinerUnite(cx: Float, cy: Float, couleur: Color, type: TypeUnite, choisie: Boolean, epuisee: Boolean) {
    val r = 11f
    if (choisie) drawCircle(Teintes.choix, radius = r + 4f, center = Offset(cx, cy), style = Stroke(3.5f))
    drawCircle(Teintes.ombre, radius = r, center = Offset(cx, cy + 2f))
    drawCircle(if (epuisee) couleur.copy(alpha = 0.55f) else couleur, radius = r, center = Offset(cx, cy))
    drawCircle(Color.White, radius = r, center = Offset(cx, cy), style = Stroke(2f))
    when (type) {
        // Lancier : un trait debout. Éclaireur : une pointe de flèche. Signes provisoires.
        TypeUnite.LANCIER -> drawRoundRect(
            Color.White,
            topLeft = Offset(cx - 3f, cy - 6f),
            size = androidx.compose.ui.geometry.Size(6f, 12f),
            cornerRadius = androidx.compose.ui.geometry.CornerRadius(2f, 2f),
        )
        TypeUnite.ECLAIREUR -> drawPath(
            Path().apply { moveTo(cx - 6f, cy + 4f); lineTo(cx, cy - 6f); lineTo(cx + 6f, cy + 4f); lineTo(cx, cy + 1f); close() },
            Color.White,
        )
    }
}
