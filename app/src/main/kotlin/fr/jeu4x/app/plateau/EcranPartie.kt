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
import androidx.compose.foundation.isSystemInDarkTheme
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
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
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
import androidx.compose.ui.graphics.FilterQuality
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
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import fr.jeu4x.app.R
import fr.jeu4x.app.moteur.ActionPermise
import fr.jeu4x.app.moteur.Avancee
import fr.jeu4x.app.moteur.Batiment
import fr.jeu4x.app.moteur.Clan
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
    // Table sous le plateau (guide de S6, § 2) : papier kraft de jour, encre de nuit.
    val fond = Color(0xFFE2C9A0)
    val fondNuit = Color(0xFF0C1018)
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

/**
 * Chaque joueur porte la couleur de son clan, celle de ses sprites (S6) : rouge, jaune,
 * indigo, turquoise. Les clans sont tous différents dans une partie.
 */
fun couleurClan(clan: Clan) = when (clan) {
    Clan.CROCS_ROUGES -> Color(0xFFE03A2A)
    Clan.MURAILLIERS -> Color(0xFFE8B820)
    Clan.VEILLEURS -> Color(0xFF5A5AF0)
    Clan.VENTS_ERRANTS -> Color(0xFF2FB8D0)
}

/** Encre posée sur la couleur d'un clan (guide de S6, § 2) : blanc, ou brun très foncé sur les clans clairs. */
private fun encreClan(clan: Clan?): Color = when (clan) {
    Clan.MURAILLIERS, Clan.VENTS_ERRANTS -> Color(0xFF2B2118)
    else -> Color.White
}

fun couleurJoueur(etat: Etat, joueur: Int): Color = etat.joueurs.getOrNull(joueur)?.let { couleurClan(it.clan) } ?: Color.White

@Composable
private fun nomJoueur(etat: Etat, joueur: Int): String =
    etat.joueurs.getOrNull(joueur)?.let { stringArrayResource(R.array.noms_clans)[it.clan.ordinal] } ?: ""

/** Codes des sprites de S6 (`art/README.md`). */
private val CODE_CLAN = listOf("crocs", "mur", "veil", "vents")
private val CODE_UNITE = listOf("eclaireur", "lancier", "archer", "cavalier", "catapulte", "coureur", "pavois", "astronome", "planeur")
private val CODE_TERRAIN = listOf("P", "F", "C", "M", "E")
private val CODE_EXPLOITATION = listOf("cueillette", "champ", "carriere")


/** Couleurs des barres et panneaux (guide de S6, § 2, « Interface »), selon le mode du téléphone (D-33). */
private class Interface(val panneau: Color, val texte: Color, val secondaire: Color, val or: Color)

private val INTERFACE_JOUR = Interface(Color(0xFFFFFFFF), Color(0xFF1C1E24), Color(0xFF555B66), Color(0xFF8A5A00))
private val INTERFACE_NUIT = Interface(Color(0xFF1C212A), Color(0xFFEEF1F5), Color(0xFFA9B1BD), Color(0xFFFFE14A))

@Composable
private fun teintesInterface() = if (isSystemInDarkTheme()) INTERFACE_NUIT else INTERFACE_JOUR

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
    val nomCourant = nomJoueur(e, e.joueurCourant)
    val ui = teintesInterface()
    Column(Modifier.fillMaxWidth().background(ui.panneau).padding(horizontal = 12.dp, vertical = 6.dp)) {
        Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.SpaceBetween) {
            Text(
                if (e.tour >= 70) stringResource(R.string.partie_tour_limite, e.tour, e.limiteTours) else stringResource(R.string.partie_tour, e.tour),
                color = ui.texte, fontSize = 15.sp,
            )
            NomPastille(
                if (e.joueurCourant == a.humain) stringResource(R.string.partie_votre_tour) else stringResource(R.string.partie_tour_de, nomCourant),
                couleurJoueur(e, e.joueurCourant), ui.texte, 15.sp, FontWeight.Bold,
            )
        }
        Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text(stringResource(R.string.partie_vivres, e.vivres, e.revenu), color = ui.or, fontSize = 13.sp)
            Text(stringResource(R.string.partie_unites, e.nbMesUnites, e.entretien), color = ui.texte, fontSize = 13.sp)
        }
        Row(Modifier.fillMaxWidth().horizontalScroll(rememberScrollState()), horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            e.joueurs.forEachIndexed { j, info ->
                val nom = nomJoueur(e, j) + if (j == a.humain) " (" + stringResource(R.string.partie_vous) + ")" else ""
                NomPastille(
                    "$nom ${info.score}/${e.seuil}" + if (info.annonce) " !" else "",
                    if (info.elimine) ui.secondaire else couleurJoueur(e, j),
                    if (info.elimine) ui.secondaire else ui.texte,
                    12.sp,
                    if (j == e.joueurCourant) FontWeight.Bold else FontWeight.Normal,
                )
            }
        }
        e.joueurs.indexOfFirst { it.annonce }.takeIf { it >= 0 }?.let { j ->
            Text(stringResource(R.string.partie_annonce, nomJoueur(e, j)), color = ui.texte, fontSize = 12.sp, fontWeight = FontWeight.Bold)
        }
    }
}

/**
 * Nom à la couleur d'un clan : texte à l'encre de l'interface, précédé d'une pastille de la couleur
 * du clan, pour que le texte reste lisible (4,5:1) et que la couleur ne soit pas le seul repère.
 */
@Composable
private fun NomPastille(texte: String, couleur: Color, encre: Color, taille: TextUnit, graisse: FontWeight) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        Box(Modifier.size(if (taille.value >= 14f) 12.dp else 10.dp).background(couleur, CircleShape))
        Spacer(Modifier.width(4.dp))
        Text(texte, color = encre, fontSize = taille, fontWeight = graisse)
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun PanneauBas(a: Affichage, vm: PartieViewModel) {
    val e = a.etat
    val nomsUnites = stringArrayResource(R.array.noms_unites)
    val nuit = isSystemInDarkTheme()
    // De nuit, le panneau passe en sombre ; les boutons suivent avec le jeu de couleurs sombre de Material.
    MaterialTheme(colorScheme = if (nuit) darkColorScheme() else MaterialTheme.colorScheme) {
    Column(Modifier.fillMaxWidth().background(teintesInterface().panneau).padding(horizontal = 12.dp, vertical = 8.dp)) {
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
                colors = ButtonDefaults.buttonColors(containerColor = couleurJoueur(a.etat, a.humain), contentColor = encreClan(a.etat.joueurs.getOrNull(a.humain)?.clan)),
            ) {
                Text(stringResource(R.string.partie_fin_du_tour), fontSize = 16.sp, fontWeight = FontWeight.Bold)
            }
        }
    }
    }
}

@Composable
private fun Texte(t: String) = Text(t, fontSize = 13.sp, color = teintesInterface().texte)

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
        v.joueur != a.humain -> Texte(stringResource(R.string.ville_adverse, nomJoueur(e, v.joueur), v.niveau))
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
                else stringResource(R.string.fin_gagnant, nomJoueur(e, e.gagnant), e.tour),
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
    // Mode sombre du téléphone : jeu de sprites de Nuit (D-33).
    val nuit = isSystemInDarkTheme()
    val sprites = rememberSprites(nuit)
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
        Modifier.fillMaxSize().background(if (nuit) Teintes.fondNuit else Teintes.fond).testTag("plateau")
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
                    // Les unités de devant d'abord : ce sont elles qu'on voit par-dessus.
                    val surUnite = etat.unites.sortedByDescending { etat.colonne(it.case) + (etat.taille - 1 - etat.rangee(it.case)) }.firstOrNull { u ->
                        val (cx, cy) = perspective.centre(etat.colonne(u.case), etat.rangee(u.case))
                        val dx = p.x - cx
                        val dy = p.y - (cy - SILHOUETTE_Y)
                        dx * dx + dy * dy <= SILHOUETTE_R * SILHOUETTE_R
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
            dessinerPlateau(perspective, affichage, sprites)
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

/** Silhouette d'une unité pour le toucher : un disque au-dessus du centre de la case (repère plateau). */
private const val SILHOUETTE_Y = 24f
private const val SILHOUETTE_R = 26f

/** Dessine un sprite de S6, son ancre sur le point (cx, cy), à la largeur d'une case (2 × demi-largeur). */
private fun DrawScope.sprite(s: Sprites, nom: String, cx: Float, cy: Float, dl: Float, alpha: Float = 1f) {
    val img = s[nom] ?: return
    val k = 2 * dl / Sprites.LARGEUR
    withTransform({
        translate(cx - Sprites.ANCRE_X * k, cy - Sprites.ANCRE_Y * k)
        scale(k, k, Offset.Zero)
    }) {
        drawImage(img, dstSize = IntSize(img.width, img.height), alpha = alpha, filterQuality = FilterQuality.Medium)
    }
}

/**
 * Plateau en sprites 3D (D-37), de l'arrière vers l'avant, chaque case dans l'ordre de
 * `art/README.md` : terrain (ou nuage), exploitation, remparts du fond, lieu, surcouches,
 * voile « vue autrefois », unité. Les repères de jeu (territoire, pastilles, points de vie)
 * restent dessinés en 2D par-dessus.
 */
private fun DrawScope.dessinerPlateau(p: Perspective, a: Affichage, s: Sprites) {
    val etat = a.etat
    val dl = p.demiLargeur
    val dh = p.demiHauteur
    for ((x, y) in p.ordreDeDessin()) {
        val case = y * etat.taille + x
        val (sx, sy) = p.sommet(x, y)
        val (cx, cy) = p.centre(x, y)
        val terrain = etat.terrains[case]
        if (terrain == null || etat.visibilite[case] == Visibilite.INCONNUE) {
            sprite(s, "nuage", cx, cy, dl)
            continue
        }
        val souvenir = etat.visibilite[case] == Visibilite.VUE_AUTREFOIS
        sprite(s, "terrain_" + CODE_TERRAIN[terrain.ordinal], cx, cy, dl)
        // Territoire : un voile léger à la couleur du propriétaire, sur le dessus de la case.
        val proprio = etat.territoire[case]
        if (proprio >= 0) drawPath(losange(sx, sy, dl, dh, retrait = 3f), couleurJoueur(etat, proprio).copy(alpha = 0.18f))
        etat.exploitations[case]?.let { sprite(s, CODE_EXPLOITATION[it.ordinal], cx, cy, dl) }
        val ville = etat.villeSur(case)
        val clanVille = ville?.joueur?.takeIf { it >= 0 }?.let { etat.joueurs.getOrNull(it)?.clan }
        if (ville != null && ville.a(Batiment.REMPARTS)) sprite(s, "remparts_arriere", cx, cy, dl)
        when {
            ville != null && clanVille != null -> sprite(s, "ville_${CODE_CLAN[clanVille.ordinal]}_${ville.niveau.coerceIn(1, 5)}", cx, cy, dl)
            ville != null -> sprite(s, "village", cx, cy, dl)
            etat.ruines[case] -> sprite(s, "ruine", cx, cy, dl)
        }
        if (ville != null) {
            if (ville.a(Batiment.REMPARTS)) sprite(s, "remparts_avant", cx, cy, dl)
            if (ville.a(Batiment.MARCHE)) sprite(s, "marche", cx, cy, dl)
            // Chaque merveille a son sprite, aux couleurs du clan qui la tient.
            if (clanVille != null) {
                val c = CODE_CLAN[clanVille.ordinal]
                if (ville.a(Batiment.ARENE)) sprite(s, "merveille_arene_$c", cx, cy, dl)
                if (ville.a(Batiment.VERGER)) sprite(s, "merveille_verger_$c", cx, cy, dl)
                if (ville.a(Batiment.TOUR)) sprite(s, "merveille_tour_$c", cx, cy, dl)
            }
            if (ville.capitale && clanVille != null) sprite(s, "capitale_${CODE_CLAN[clanVille.ordinal]}", cx, cy, dl)
            if (clanVille != null) badgeNiveau(cx + dl * 0.55f, cy + dh * 0.35f, couleurClan(clanVille), ville.niveau)
        }
        if (souvenir) sprite(s, "calque", cx, cy, dl)
        if (case == a.villeChoisie || case == a.caseChoisie) drawPath(losange(sx, sy, dl, dh, retrait = 2f), Teintes.choix, style = Stroke(3f))
        // Pastilles (D-31) : blanches pour aller.
        if (case in a.portee) drawCircle(Teintes.portee, radius = 6f, center = Offset(cx, cy))
        val u = etat.uniteSur(case) ?: continue
        val epuisee = u.joueur == a.humain && (u.pointsRestants == 0 || !u.peutAgir)
        if (a.choisie == case) drawOval(Teintes.choix, topLeft = Offset(cx - dl * 0.5f, cy - dh * 0.4f), size = Size(dl, dh * 0.8f), style = Stroke(3.5f))
        val clanU = etat.joueurs.getOrNull(u.joueur)?.clan ?: Clan.CROCS_ROUGES
        // Anneau 3D à la couleur du clan, sous les pieds, juste avant l'unité.
        sprite(s, "anneau_${CODE_CLAN[clanU.ordinal]}", cx, cy, dl)
        sprite(s, "unite_${CODE_CLAN[clanU.ordinal]}_${CODE_UNITE[u.type.ordinal]}", cx, cy, dl, alpha = if (epuisee) 0.6f else 1f)
        barreVie(cx, cy + dh * 0.45f, u.pv, u.pvMax)
        // Cibles : un anneau rouge autour de la silhouette.
        if (case in a.cibles) drawCircle(Teintes.cible, radius = SILHOUETTE_R, center = Offset(cx, cy - SILHOUETTE_Y), style = Stroke(3.5f))
        if (a.attaque?.first == case) drawCircle(Teintes.cible, radius = SILHOUETTE_R + 4f, center = Offset(cx, cy - SILHOUETTE_Y), style = Stroke(5f))
    }
}

private fun DrawScope.badgeNiveau(x: Float, y: Float, couleur: Color, niveau: Int) {
    drawCircle(Color.White, radius = 7f, center = Offset(x, y))
    drawCircle(couleur, radius = 7f, center = Offset(x, y), style = Stroke(2f))
    texte("$niveau", x, y + 3.5f, 10f, Color.Black)
}

private fun DrawScope.barreVie(cx: Float, y: Float, pv: Int, pvMax: Int) {
    val largeur = 24f
    drawRoundRect(Color(0xAA000000), topLeft = Offset(cx - largeur / 2, y), size = Size(largeur, 4f), cornerRadius = CornerRadius(2f, 2f))
    val part = (pv.toFloat() / pvMax.coerceAtLeast(1)).coerceIn(0f, 1f)
    drawRoundRect(
        if (part > 0.5f) Color(0xFF6BD26B) else if (part > 0.25f) Color(0xFFF2C14E) else Color(0xFFE5533D),
        topLeft = Offset(cx - largeur / 2, y), size = Size(largeur * part, 4f), cornerRadius = CornerRadius(2f, 2f),
    )
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
