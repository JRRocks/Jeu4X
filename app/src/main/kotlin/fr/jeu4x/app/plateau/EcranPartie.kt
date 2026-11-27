package fr.jeu4x.app.plateau

import androidx.compose.animation.core.animate
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.gestures.detectTransformGestures
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.OutlinedButton
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
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.drawscope.withTransform
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringArrayResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import fr.jeu4x.app.R
import fr.jeu4x.app.moteur.ActionPermise
import fr.jeu4x.app.moteur.Avancee
import fr.jeu4x.app.moteur.Batiment
import fr.jeu4x.app.moteur.COUT_UNITE
import fr.jeu4x.app.moteur.Etat
import fr.jeu4x.app.moteur.Exploitation
import fr.jeu4x.app.moteur.Terrain
import fr.jeu4x.app.moteur.TypeUnite
import fr.jeu4x.app.moteur.Visibilite

/**
 * Couleurs provisoires, reprises de l'aperçu C et du guide de S6 quand ça ne coûte rien
 * (montagne en gris froid, colline ocre). Le style définitif revient à la session 6.
 */
private object Teintes {
    val fond = Color(0xFF3C5A3A)
    val barre = Color(0xFF2B2B2B)
    val plaine = Color(0xFFB9D98C)
    val foret = Color(0xFF5F9A55)
    val sapin = Color(0xFF3D6E3A)
    val colline = Color(0xFFD7B878)
    val collineBosse = Color(0xFFB8954F)
    val montagne = Color(0xFFA3A8AE)
    val montagneFlancG = Color(0xFF80868D)
    val montagneFlancD = Color(0xFF6A7077)
    val roche = Color(0xFF767C83)
    val neige = Color(0xFFF2F4F6)
    val eau = Color(0xFF6FA8D6)
    val vague = Color(0xFFE8F3FB)
    val bord = Color(0x22000000)
    val portee = Color(0xEEFFFFFF)
    val cible = Color(0xFFE53935)
    val choix = Color(0xFFFFD34D)
    val inconnue = Color(0xFF2A3A29)
    val souvenir = Color(0x66000000)
    val bleu = Color(0xFF2F6FD0)
    val rouge = Color(0xFFD0482F)
    val violet = Color(0xFF8E4FC0)
    val jaune = Color(0xFFD9A21B)
    val ombre = Color(0x33000000)
    val creme = Color(0xFFF6F3EC)
    val ruine = Color(0xFF8C8172)
    val champ = Color(0xFFE6C55A)
}

fun couleurJoueur(joueur: Int) = when (joueur) {
    0 -> Teintes.bleu
    1 -> Teintes.rouge
    2 -> Teintes.violet
    3 -> Teintes.jaune
    else -> Color.White
}

private fun nomCouleur(joueur: Int) = when (joueur) {
    0 -> R.string.joueur_bleu
    1 -> R.string.joueur_rouge
    2 -> R.string.joueur_violet
    else -> R.string.joueur_jaune
}

/** Sigle provisoire de chaque type d'unité, dessiné sur le jeton. */
private val SIGLES = listOf("É", "L", "A", "C", "K", "R", "P", "S", "G")

@Composable
fun EcranPartie(affichage: Affichage, vm: PartieViewModel) {
    val etat = affichage.etat
    Box(Modifier.fillMaxSize().background(Teintes.creme)) {
        Column(Modifier.fillMaxSize()) {
            BarreHaut(affichage)
            Box(Modifier.weight(1f).fillMaxWidth()) {
                Plateau(affichage, vm::toucherCase)
            }
            PanneauBas(affichage, vm)
        }
        if (affichage.arbreOuvert) Arbre(affichage, vm)
        if (etat.finie) Fin(affichage, vm)
    }
}

@Composable
private fun BarreHaut(a: Affichage) {
    val e = a.etat
    val nomCourant = stringResource(nomCouleur(e.joueurCourant))
    Column(Modifier.fillMaxWidth().background(Teintes.barre).padding(horizontal = 12.dp, vertical = 6.dp)) {
        Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.SpaceBetween) {
            Text(
                if (e.tour >= 70) stringResource(R.string.partie_tour_limite, e.tour, e.limiteTours) else stringResource(R.string.partie_tour, e.tour),
                color = Color.White, fontSize = 15.sp,
            )
            Text(
                if (e.joueurCourant == a.humain) stringResource(R.string.partie_votre_tour) else stringResource(R.string.partie_tour_de, nomCourant),
                color = couleurJoueur(e.joueurCourant), fontSize = 15.sp, fontWeight = FontWeight.Bold,
            )
        }
        Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text(stringResource(R.string.partie_vivres, e.vivres, e.revenu), color = Teintes.champ, fontSize = 13.sp)
            Text(stringResource(R.string.partie_unites, e.nbMesUnites, e.entretien), color = Color.White, fontSize = 13.sp)
        }
        Row(Modifier.fillMaxWidth().horizontalScroll(rememberScrollState()), horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            e.joueurs.forEachIndexed { j, info ->
                val nom = stringResource(nomCouleur(j)) + if (j == a.humain) " (" + stringResource(R.string.partie_vous) + ")" else ""
                Text(
                    "$nom ${info.score}/${e.seuil}" + if (info.annonce) " !" else "",
                    color = if (info.elimine) Color.Gray else couleurJoueur(j),
                    fontSize = 12.sp,
                    fontWeight = if (j == e.joueurCourant) FontWeight.Bold else FontWeight.Normal,
                )
            }
        }
        e.joueurs.indexOfFirst { it.annonce }.takeIf { it >= 0 }?.let { j ->
            Text(stringResource(R.string.partie_annonce, stringResource(nomCouleur(j))), color = Teintes.choix, fontSize = 12.sp)
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun PanneauBas(a: Affichage, vm: PartieViewModel) {
    val e = a.etat
    val nomsUnites = stringArrayResource(R.array.noms_unites)
    Column(Modifier.fillMaxWidth().background(Color.White).padding(horizontal = 12.dp, vertical = 8.dp)) {
        val attaque = a.attaque
        val choisie = a.choisie?.let { e.uniteSur(it) }
        val ville = a.villeChoisie?.let { e.villeSur(it) }
        when {
            !a.tourHumain && !e.finie -> Texte(stringResource(R.string.partie_ia_joue))
            attaque != null && choisie != null -> {
                val (cible, an) = attaque
                val adverse = e.uniteSur(cible)
                Text(stringResource(R.string.combat_titre, adverse?.let { nomsUnites[it.type.ordinal] } ?: ""), fontWeight = FontWeight.Bold, fontSize = 15.sp)
                Texte(
                    if (an.cibleDetruite) stringResource(R.string.combat_detruit, an.degats)
                    else stringResource(R.string.combat_degats, an.degats, (adverse?.pv ?: 0) - an.degats),
                )
                Texte(
                    when {
                        an.attaquantDetruit -> stringResource(R.string.combat_perdu, an.riposte)
                        an.riposte > 0 -> stringResource(R.string.combat_riposte, an.riposte, choisie.pv - an.riposte)
                        else -> stringResource(R.string.combat_sans_riposte)
                    },
                )
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    OutlinedButton(onClick = vm::annulerAttaque) { Text(stringResource(R.string.combat_annuler)) }
                    Button(onClick = vm::confirmerAttaque, colors = ButtonDefaults.buttonColors(containerColor = Teintes.cible), modifier = Modifier.testTag("attaquer")) {
                        Text(stringResource(R.string.combat_attaquer))
                    }
                }
            }
            choisie != null -> {
                Texte(stringResource(R.string.unite_resume, nomsUnites[choisie.type.ordinal], choisie.pv, choisie.pvMax, choisie.pointsRestants))
                Texte(
                    when {
                        !choisie.peutAgir -> stringResource(R.string.partie_unite_attend)
                        a.portee.isEmpty() && a.cibles.isEmpty() -> stringResource(R.string.partie_unite_epuisee)
                        else -> stringResource(R.string.partie_choisir_case)
                    },
                )
                FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    if (a.permise(ActionPermise.RALLIER, choisie.case) != null) {
                        val v = e.villeSur(choisie.case)
                        Button(onClick = vm::rallier, modifier = Modifier.testTag("rallier")) {
                            Text(stringResource(if (v?.joueur == -1) R.string.unite_rallier else R.string.unite_prendre))
                        }
                    }
                    OutlinedButton(onClick = vm::renvoyer) { Text(stringResource(R.string.unite_renvoyer)) }
                }
            }
            ville != null -> PanneauVille(a, ville.case, vm)
            a.caseChoisie != null -> {
                val c = a.caseChoisie
                val t = e.terrains[c]
                val ex = when (t) {
                    Terrain.FORET -> Exploitation.CUEILLETTE
                    Terrain.PLAINE -> Exploitation.CHAMP
                    else -> Exploitation.CARRIERE
                }
                val noms = stringArrayResource(R.array.noms_exploitations)
                Button(onClick = vm::exploiter) {
                    Text(stringResource(R.string.case_exploiter, noms[ex.ordinal], listOf(2, 3, 5)[ex.ordinal], listOf(1, 1, 2)[ex.ordinal]))
                }
            }
            else -> Texte(stringResource(R.string.partie_choisir_unite))
        }
        Spacer(Modifier.height(6.dp))
        Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            OutlinedButton(onClick = { vm.ouvrirArbre(true) }, enabled = a.tourHumain, modifier = Modifier.height(48.dp)) {
                Text(stringResource(R.string.partie_evolution))
            }
            Button(
                onClick = vm::finDuTour,
                enabled = a.tourHumain,
                modifier = Modifier.weight(1f).height(48.dp).testTag("bouton-tour"),
                shape = RoundedCornerShape(24.dp),
                colors = ButtonDefaults.buttonColors(containerColor = couleurJoueur(a.humain)),
            ) {
                Text(stringResource(R.string.partie_fin_du_tour), fontSize = 16.sp, fontWeight = FontWeight.Bold)
            }
        }
    }
}

@Composable
private fun Texte(t: String) = Text(t, fontSize = 13.sp, color = Color(0xFF333333))

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun PanneauVille(a: Affichage, case: Int, vm: PartieViewModel) {
    val e = a.etat
    val v = e.villeSur(case) ?: return
    val nomsUnites = stringArrayResource(R.array.noms_unites)
    val nomsBat = stringArrayResource(R.array.noms_batiments)
    val cadeauxA = stringArrayResource(R.array.cadeaux_a)
    val cadeauxB = stringArrayResource(R.array.cadeaux_b)
    when {
        v.joueur == -1 -> Texte(stringResource(R.string.ville_village))
        v.joueur != a.humain -> Texte(stringResource(R.string.ville_adverse, stringResource(nomCouleur(v.joueur)), v.niveau))
        else -> {
            Text(
                stringResource(if (v.capitale) R.string.ville_capitale else R.string.ville_resume, v.niveau, v.revenu),
                fontWeight = FontWeight.Bold, fontSize = 14.sp,
            )
            val boutons = ArrayList<@Composable () -> Unit>()
            for (p in a.permises.filter { it.genre == ActionPermise.PRODUIRE && it.a == case }) {
                val t = TypeUnite.entries[p.b]
                boutons.add { Button(onClick = { vm.produire(p.b) }) { Text(stringResource(R.string.ville_produire, nomsUnites[t.ordinal], COUT_UNITE[t] ?: 0)) } }
            }
            for (b in listOf(false, true)) {
                if (a.permise(ActionPermise.AGRANDIR, case, if (b) 1 else 0) != null) {
                    val cadeau = (if (b) cadeauxB else cadeauxA)[(v.niveau - 1).coerceIn(0, 3)]
                    boutons.add { OutlinedButton(onClick = { vm.agrandir(b) }) { Text(stringResource(R.string.ville_grandir, v.prixCroissance, cadeau)) } }
                }
            }
            for (p in a.permises.filter { it.genre == ActionPermise.BATIR && it.a == case }) {
                val prix = when (Batiment.entries[p.b]) {
                    Batiment.REMPARTS -> if (e.joueurs.getOrNull(a.humain)?.clan == fr.jeu4x.app.moteur.Clan.MURAILLIERS) 3 else 6
                    Batiment.MARCHE -> 6
                    else -> 15
                }
                boutons.add { OutlinedButton(onClick = { vm.batir(p.b) }) { Text(stringResource(R.string.ville_batir, nomsBat[p.b], prix)) } }
            }
            if (boutons.isEmpty()) Texte(stringResource(R.string.ville_rien))
            Column(Modifier.heightIn(max = 120.dp).verticalScroll(rememberScrollState())) {
                FlowRow(horizontalArrangement = Arrangement.spacedBy(6.dp)) { boutons.forEach { it() } }
            }
        }
    }
}

/** Arbre d'évolution : trois branches côte à côte, l'avancée choisie se recherche d'un toucher. */
@Composable
private fun Arbre(a: Affichage, vm: PartieViewModel) {
    val e = a.etat
    val noms = stringArrayResource(R.array.noms_avancees)
    val effets = stringArrayResource(R.array.effets_avancees)
    val branches = stringArrayResource(R.array.branches)
    val info = e.joueurs.getOrNull(a.humain)
    Column(Modifier.fillMaxSize().background(Color(0xF0222222)).clickable(enabled = false) {}.padding(12.dp)) {
        Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.SpaceBetween) {
            Text(stringResource(R.string.arbre_titre, e.vivres), color = Color.White, fontSize = 17.sp, fontWeight = FontWeight.Bold)
            Button(onClick = { vm.ouvrirArbre(false) }) { Text(stringResource(R.string.arbre_fermer)) }
        }
        Row(Modifier.fillMaxSize().padding(top = 8.dp), horizontalArrangement = Arrangement.spacedBy(6.dp)) {
            for (b in 0 until 3) {
                Column(Modifier.weight(1f).verticalScroll(rememberScrollState()), verticalArrangement = Arrangement.spacedBy(6.dp)) {
                    Text(branches[b], color = Teintes.champ, fontWeight = FontWeight.Bold, modifier = Modifier.fillMaxWidth(), textAlign = TextAlign.Center)
                    for (k in 0 until 5) {
                        val av = Avancee.entries[b * 5 + k]
                        val acquise = info?.possede(av) == true
                        val permise = a.permise(ActionPermise.RECHERCHER, av.ordinal) != null
                        val ouverte = e.avanceesOuvertes[av.ordinal]
                        val fond = when {
                            acquise -> Color(0xFF4C7A3F)
                            permise -> Color(0xFF2F6FD0)
                            ouverte -> Color(0xFF4A4A4A)
                            else -> Color(0xFF333333)
                        }
                        Column(
                            Modifier.fillMaxWidth().background(fond, RoundedCornerShape(8.dp))
                                .clickable(enabled = permise) { vm.rechercher(av.ordinal) }.padding(6.dp),
                        ) {
                            Text(noms[av.ordinal], color = Color.White, fontSize = 12.sp, fontWeight = FontWeight.Bold)
                            Text(effets[av.ordinal], color = Color(0xFFDDDDDD), fontSize = 10.sp)
                            Text(
                                when {
                                    acquise -> stringResource(R.string.arbre_acquise)
                                    !ouverte -> stringResource(R.string.arbre_fermee)
                                    else -> stringResource(R.string.arbre_prix, e.prixAvancees[av.ordinal])
                                },
                                color = Teintes.champ, fontSize = 10.sp,
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun Fin(a: Affichage, vm: PartieViewModel) {
    val e = a.etat
    Box(Modifier.fillMaxSize().background(Color(0x99000000)).clickable(enabled = false) {}, contentAlignment = Alignment.Center) {
        Column(
            Modifier.padding(24.dp).background(Color.White, RoundedCornerShape(16.dp)).padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Text(
                stringResource(
                    when (e.gagnant) {
                        -1 -> R.string.fin_nulle
                        a.humain -> R.string.fin_victoire
                        else -> R.string.fin_defaite
                    },
                ),
                fontSize = 26.sp, fontWeight = FontWeight.Bold,
            )
            Text(
                if (e.gagnant < 0) stringResource(R.string.fin_nulle_detail, e.limiteTours)
                else stringResource(R.string.fin_gagnant, stringResource(nomCouleur(e.gagnant)), e.tour),
                fontSize = 14.sp,
            )
            Text(stringResource(R.string.reglage_carte, a.graine), fontSize = 12.sp, color = Color.Gray)
            Spacer(Modifier.height(12.dp))
            Button(onClick = vm::nouvellePartie) { Text(stringResource(R.string.partie_nouvelle)) }
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
    val miennes = { etat.unites.filter { it.joueur == affichage.humain }.map { etat.colonne(it.case) to etat.rangee(it.case) } }

    // Lancement d'une partie (D-20) : vue zoomée sur les unités du joueur humain.
    LaunchedEffect(affichage.numeroPartie, tailleVue) {
        if (echelle == 0f && tailleVue.width > 0 && tailleVue.height > 0) {
            val (e, dx, dy) = perspective.cadrageSur(miennes(), tailleVue.width.toFloat(), tailleVue.height.toFloat())
            echelle = e
            decalage = Offset(dx, dy)
        }
    }

    // Début de son tour (D-21) : la vue glisse en une demi-seconde vers ses unités, au zoom choisi.
    val sonTour = etat.joueurCourant == affichage.humain && !affichage.iaJoue
    LaunchedEffect(affichage.numeroPartie, etat.tour, sonTour) {
        if (!sonTour || echelle == 0f || tailleVue.width == 0) return@LaunchedEffect
        val (cx, cy) = perspective.decalagePour(miennes(), tailleVue.width.toFloat(), tailleVue.height.toFloat(), echelle)
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
                    // Guide S6 : le toucher vise d'abord la silhouette d'une unité, puis le losange.
                    val surUnite = etat.unites.firstOrNull { u ->
                        val (cx, cy) = perspective.centre(etat.colonne(u.case), etat.rangee(u.case))
                        val haut = if (etat.terrains[u.case] == Terrain.MONTAGNE) 7f else 0f
                        val dx = p.x - cx
                        val dy = p.y - (cy - 14f - haut)
                        dx * dx + dy * dy <= 15f * 15f
                    }
                    if (surUnite != null) surToucherCase(surUnite.case)
                    else perspective.caseSous(p.x, p.y)?.let { (x, y) -> surToucherCase(y * etat.taille + x) }
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
        // Territoire : un voile à la couleur du propriétaire.
        val proprio = etat.territoire[case]
        if (proprio >= 0) drawPath(losange(sx, haut, dl, dh, retrait = 2f), couleurJoueur(proprio).copy(alpha = 0.22f))
        drawPath(forme, Teintes.bord, style = Stroke(1f))
        decor(terrain, sx, haut + dh)
        etat.exploitations[case]?.let { dessinerExploitation(it, sx, haut + dh) }
        if (etat.ruines[case]) dessinerRuine(sx, haut + dh)
        etat.villeSur(case)?.let { dessinerVille(sx, haut + dh, if (it.joueur >= 0) couleurJoueur(it.joueur) else Color.White, it.niveau, it.capitale) }
        // Vue autrefois : le terrain et les villes tels qu'on s'en souvient, assombris.
        if (souvenir) drawPath(forme, Teintes.souvenir)
        if (case == a.villeChoisie || case == a.caseChoisie) drawPath(losange(sx, haut, dl, dh, retrait = 2f), Teintes.choix, style = Stroke(3f))
    }
    // Les unités après le terrain, dans le même ordre, pour rester par-dessus.
    for ((x, y) in p.ordreDeDessin()) {
        val case = y * etat.taille + x
        val (cx, cy) = p.centre(x, y)
        val haut = if (etat.terrains[case] == Terrain.MONTAGNE) relief else 0f
        etat.uniteSur(case)?.let { u ->
            dessinerUnite(cx, cy - 14f - haut, couleurJoueur(u.joueur), u.type, u.pv, u.pvMax, choisie = a.choisie == case, epuisee = u.joueur == a.humain && (u.pointsRestants == 0 || !u.peutAgir))
        }
        // Pastilles (D-31) : blanches pour aller, rouges pour attaquer.
        if (case in a.portee) drawCircle(Teintes.portee, radius = 6f, center = Offset(cx, cy - haut))
        if (case in a.cibles) drawCircle(Teintes.cible, radius = 17f, center = Offset(cx, cy - 14f - haut), style = Stroke(3.5f))
        if (a.attaque?.first == case) drawCircle(Teintes.cible, radius = 20f, center = Offset(cx, cy - 14f - haut), style = Stroke(5f))
    }
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
                topLeft = Offset(cx + dx - r, cy + dy - r * 0.8f), size = Size(2 * r, 1.6f * r),
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

private fun DrawScope.dessinerExploitation(e: Exploitation, cx: Float, cy: Float) {
    when (e) {
        Exploitation.CHAMP -> for (o in listOf(-6f, 0f, 6f)) drawLine(Teintes.champ, Offset(cx - 14f, cy + o + 4f), Offset(cx + 2f, cy + o - 4f), strokeWidth = 2.5f)
        Exploitation.CUEILLETTE -> for (dx in listOf(-14f, -8f)) drawCircle(Color(0xFFC0392B), radius = 2.5f, center = Offset(cx + dx, cy + 6f))
        Exploitation.CARRIERE -> drawRect(Color(0xFF8E8E8E), topLeft = Offset(cx - 16f, cy + 2f), size = Size(9f, 6f))
    }
}

/** Ruine, provisoire : deux piliers et un linteau. */
private fun DrawScope.dessinerRuine(cx: Float, cy: Float) {
    drawRect(Teintes.ruine, topLeft = Offset(cx + 4f, cy - 6f), size = Size(3f, 10f))
    drawRect(Teintes.ruine, topLeft = Offset(cx + 13f, cy - 4f), size = Size(3f, 8f))
    drawRect(Teintes.ruine, topLeft = Offset(cx + 3f, cy - 8f), size = Size(11f, 3f))
}

/** Ville, provisoire : une maison à la couleur de son propriétaire (blanche si neutre), son niveau, une étoile pour la capitale. */
private fun DrawScope.dessinerVille(cx: Float, cy: Float, couleur: Color, niveau: Int, capitale: Boolean) {
    drawRect(Teintes.ombre, topLeft = Offset(cx - 9f, cy - 4f), size = Size(20f, 12f))
    drawRect(couleur, topLeft = Offset(cx - 10f, cy - 6f), size = Size(20f, 12f))
    drawPath(Path().apply { moveTo(cx - 13f, cy - 6f); lineTo(cx, cy - 17f); lineTo(cx + 13f, cy - 6f); close() }, if (capitale) Teintes.choix else Teintes.creme)
    drawRect(Teintes.creme, topLeft = Offset(cx - 3f, cy - 1f), size = Size(6f, 7f))
    if (couleur != Color.White) {
        drawCircle(Color.White, radius = 6.5f, center = Offset(cx + 14f, cy + 4f))
        drawCircle(couleur, radius = 6.5f, center = Offset(cx + 14f, cy + 4f), style = Stroke(1.5f))
        texte("$niveau", cx + 14f, cy + 7.5f, 9f, Color.Black)
    }
}

private fun DrawScope.texte(t: String, x: Float, y: Float, taille: Float, couleur: Color) {
    val peinture = android.graphics.Paint().apply {
        isAntiAlias = true
        textSize = taille
        color = android.graphics.Color.argb((couleur.alpha * 255).toInt(), (couleur.red * 255).toInt(), (couleur.green * 255).toInt(), (couleur.blue * 255).toInt())
        textAlign = android.graphics.Paint.Align.CENTER
        isFakeBoldText = true
    }
    drawContext.canvas.nativeCanvas.drawText(t, x, y, peinture)
}

private fun DrawScope.dessinerUnite(cx: Float, cy: Float, couleur: Color, type: TypeUnite, pv: Int, pvMax: Int, choisie: Boolean, epuisee: Boolean) {
    val r = 11f
    if (choisie) drawCircle(Teintes.choix, radius = r + 4f, center = Offset(cx, cy), style = Stroke(3.5f))
    drawCircle(Teintes.ombre, radius = r, center = Offset(cx, cy + 2f))
    drawCircle(if (epuisee) couleur.copy(alpha = 0.55f) else couleur, radius = r, center = Offset(cx, cy))
    drawCircle(Color.White, radius = r, center = Offset(cx, cy), style = Stroke(2f))
    texte(SIGLES[type.ordinal], cx, cy + 4.5f, 12f, Color.White)
    // Points de vie : une barre sous le jeton.
    val largeur = 20f
    drawRoundRect(Color(0xAA000000), topLeft = Offset(cx - largeur / 2, cy + r + 2f), size = Size(largeur, 4f), cornerRadius = CornerRadius(2f, 2f))
    val part = (pv.toFloat() / pvMax.coerceAtLeast(1)).coerceIn(0f, 1f)
    drawRoundRect(
        if (part > 0.5f) Color(0xFF6BD26B) else if (part > 0.25f) Color(0xFFF2C14E) else Color(0xFFE5533D),
        topLeft = Offset(cx - largeur / 2, cy + r + 2f), size = Size(largeur * part, 4f), cornerRadius = CornerRadius(2f, 2f),
    )
}
