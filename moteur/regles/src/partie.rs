//! Une partie : réglages, état, actions permises, application d'une action.
//!
//! Interface demandée par la session 3 (`docs/ia/besoins-moteur.md`, § 3) :
//! nouvelle partie depuis une graine, joueur courant, masque des actions permises dans un
//! espace fixe, application, copie (`Clone`), fin de partie et scores, vue d'un joueur
//! (brouillard), codage en plans, actions en texte, empreinte.
//!
//! Règles V1 codées ici (étape a) : 2 à 4 joueurs, taille de carte selon le nombre de
//! joueurs, déplacements dans 8 directions, collines, brouillard par joueur qui se
//! souvient. Villes, combat, arbre et victoire viennent aux étapes suivantes.

use crate::carte;
use crate::empreinte::Empreinte;
use crate::hasard::Hasard;
use crate::plateau::{Case, Plateau, Terrain, NB_TERRAINS, TAILLE_MAX};

/// Nombre de joueurs le plus grand permis.
pub const JOUEURS_MAX: u8 = 4;
/// Vision d'une ville (§ 3.2).
pub const VISION_VILLE: u8 = 2;

/// Réglages d'une partie (§ 12.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Reglages {
    /// 2 à 4.
    pub nb_joueurs: u8,
    /// Côté du plateau carré (11, 12 ou 14 selon le nombre de joueurs ; 16 au plus).
    pub taille: u8,
    /// Distance minimale entre deux capitales.
    pub distance_capitales: u8,
    /// Limite de sécurité : toute partie se termine (§ 9.4).
    pub limite_tours: u16,
}

impl Reglages {
    /// Réglages de la V1 pour `nb_joueurs` joueurs (§ 1.2, § 12.1).
    pub fn pour(nb_joueurs: u8) -> Reglages {
        let (taille, distance_capitales) = match nb_joueurs {
            2 => (11, 6),
            3 => (12, 6),
            4 => (14, 7),
            _ => panic!("2 à 4 joueurs, pas {nb_joueurs}"),
        };
        Reglages { nb_joueurs, taille, distance_capitales, limite_tours: 80 }
    }
}

impl Default for Reglages {
    fn default() -> Self {
        Reglages::pour(2)
    }
}

/// Types d'unités présents à l'étape a : ceux du départ (§ 5.2, § 12.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TypeUnite {
    Eclaireur = 0,
    Lancier = 1,
}

impl TypeUnite {
    pub fn mouvement(self) -> u8 {
        match self {
            TypeUnite::Eclaireur => 3,
            TypeUnite::Lancier => 1,
        }
    }
    pub fn vision(self) -> u8 {
        match self {
            TypeUnite::Eclaireur => 3,
            TypeUnite::Lancier => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Unite {
    pub joueur: u8,
    pub case: Case,
    pub type_unite: TypeUnite,
    pub points_restants: u8,
    /// A déjà bougé pendant ce tour de son joueur (§ 6.1).
    pub a_bouge: bool,
}

/// Une ville. À l'étape a, seulement les capitales (les villages viennent à l'étape b).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ville {
    pub case: Case,
    /// Propriétaire ; `None` pour un village neutre.
    pub joueur: Option<u8>,
    pub capitale: bool,
}

/// État d'une case pour un joueur (§ 3.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Visibilite {
    Inconnue = 0,
    VueAutrefois = 1,
    Visible = 2,
}

/// Souvenir d'une ville : jamais vue, ou dernier propriétaire connu.
const VILLE_JAMAIS_VUE: u8 = 0xFF;
const VILLE_NEUTRE: u8 = 0xFE;

/// Ce que sait un joueur (§ 3.1) : état de chaque case et dernier propriétaire vu de chaque ville.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Brouillard {
    cases: Vec<Visibilite>,
    villes: Vec<u8>,
}

/// Une action, dans sa forme lisible. Son numéro fixe est donné par [`Action::numero`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    /// Le joueur courant passe la main.
    FinDuTour,
    /// Déplacer l'unité posée sur `depart` jusqu'à `arrivee`, par le chemin le moins cher.
    Deplacer { depart: Case, arrivee: Case },
}

/// Nombre de cases de l'espace d'actions (plateau le plus grand permis).
pub const CASES_MAX: u32 = TAILLE_MAX as u32 * TAILLE_MAX as u32;
/// Taille de l'espace d'actions : 1 (fin du tour) + case de départ × case d'arrivée.
/// Les cases sont numérotées sur une grille 16 × 16 quelle que soit la taille réelle.
pub const NB_ACTIONS: u32 = 1 + CASES_MAX * CASES_MAX;

impl Action {
    /// Numéro fixe de l'action (E4). `taille` : côté du plateau de la partie.
    pub fn numero(self, taille: u8) -> u32 {
        match self {
            Action::FinDuTour => 0,
            Action::Deplacer { depart, arrivee } => {
                1 + vers_grille(depart, taille) * CASES_MAX + vers_grille(arrivee, taille)
            }
        }
    }

    /// Inverse de [`Action::numero`] ; `None` si le numéro sort de l'espace ou du plateau.
    pub fn depuis_numero(n: u32, taille: u8) -> Option<Action> {
        if n == 0 {
            return Some(Action::FinDuTour);
        }
        if n >= NB_ACTIONS {
            return None;
        }
        let k = n - 1;
        Some(Action::Deplacer {
            depart: depuis_grille(k / CASES_MAX, taille)?,
            arrivee: depuis_grille(k % CASES_MAX, taille)?,
        })
    }
}

fn vers_grille(c: Case, taille: u8) -> u32 {
    let (x, y) = (c as u32 % taille as u32, c as u32 / taille as u32);
    y * TAILLE_MAX as u32 + x
}

fn depuis_grille(g: u32, taille: u8) -> Option<Case> {
    let (x, y) = (g % TAILLE_MAX as u32, g / TAILLE_MAX as u32);
    if x >= taille as u32 || y >= taille as u32 {
        return None;
    }
    Some((y * taille as u32 + x) as Case)
}

/// Résultat du calcul des déplacements : coût de chaque case et case précédente sur le chemin.
struct Chemins {
    /// Points restants à l'arrivée, `None` si la case n'est pas atteignable.
    reste: Vec<Option<u8>>,
    precedente: Vec<Case>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Partie {
    reglages: Reglages,
    graine: u64,
    plateau: Plateau,
    unites: Vec<Unite>,
    villes: Vec<Ville>,
    brouillards: Vec<Brouillard>,
    joueur_courant: u8,
    tour: u16,
    hasard: Hasard,
    finie: bool,
}

/// Essais de graines dérivées au plus (§ 2.3) ; en pratique quelques-uns suffisent.
const ESSAIS_CARTE: u64 = 10_000;

/// Graine du `k`-ième essai de carte. Le pas est grand et impair : les essais d'une graine
/// ne retombent pas sur ceux de la graine voisine (42 et 43 donnent deux cartes différentes).
pub fn graine_essai(graine: u64, k: u64) -> u64 {
    graine.wrapping_add(k.wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

/// Nombre d'essais qu'il a fallu pour la carte de cette graine (mesure et tests).
pub fn essais_carte(reglages: Reglages, graine: u64) -> u64 {
    (0..ESSAIS_CARTE)
        .find(|&k| {
            let mut h = Hasard::new(graine_essai(graine, k));
            carte::essayer(reglages.taille, reglages.nb_joueurs, reglages.distance_capitales, &mut h).is_some()
        })
        .map_or(ESSAIS_CARTE, |k| k + 1)
}

impl Partie {
    /// Nouvelle partie : la carte est tirée de la graine, rien d'autre n'intervient (§ 0).
    /// Si la carte ne donne pas des départs équitables, on recommence avec une graine
    /// dérivée ([`graine_essai`]) : même graine, même carte.
    pub fn nouvelle(reglages: Reglages, graine: u64) -> Partie {
        let r = reglages;
        assert!((2..=JOUEURS_MAX).contains(&r.nb_joueurs), "2 à 4 joueurs");
        assert!(r.limite_tours >= 1);
        let mut trouvee = None;
        for k in 0..ESSAIS_CARTE {
            let mut h = Hasard::new(graine_essai(graine, k));
            if let Some(c) = carte::essayer(r.taille, r.nb_joueurs, r.distance_capitales, &mut h) {
                trouvee = Some((c, h));
                break;
            }
        }
        let (carte, mut hasard) = trouvee.expect("aucune carte équitable : réglages impossibles");
        let plateau = carte.plateau;
        let villes: Vec<Ville> = carte
            .capitales
            .iter()
            .enumerate()
            .map(|(j, &case)| Ville { case, joueur: Some(j as u8), capitale: true })
            .collect();
        // Départ (§ 5.2) : un Lancier et un Éclaireur sur deux cases voisines de la capitale,
        // franchissables, tirées au hasard de la carte.
        let mut unites = Vec::with_capacity(2 * r.nb_joueurs as usize);
        let mut v = [0; 8];
        for (j, &cap) in carte.capitales.iter().enumerate() {
            let nv = plateau.voisines(cap, &mut v);
            let mut libres: Vec<Case> = v[..nv].iter().copied().filter(|&w| plateau.terrain(w).cout().is_some()).collect();
            for type_unite in [TypeUnite::Lancier, TypeUnite::Eclaireur] {
                let case = libres.remove(hasard.sous(libres.len() as u32) as usize);
                unites.push(Unite { joueur: j as u8, case, type_unite, points_restants: type_unite.mouvement(), a_bouge: false });
            }
        }
        let n = plateau.nb_cases();
        let brouillards = (0..r.nb_joueurs)
            .map(|_| Brouillard { cases: vec![Visibilite::Inconnue; n], villes: vec![VILLE_JAMAIS_VUE; villes.len()] })
            .collect();
        let mut p = Partie { reglages: r, graine, plateau, unites, villes, brouillards, joueur_courant: 0, tour: 1, hasard, finie: false };
        for j in 0..r.nb_joueurs {
            p.recalculer_vision(j);
        }
        p
    }

    pub fn reglages(&self) -> Reglages {
        self.reglages
    }
    pub fn graine(&self) -> u64 {
        self.graine
    }
    pub fn plateau(&self) -> &Plateau {
        &self.plateau
    }
    pub fn unites(&self) -> &[Unite] {
        &self.unites
    }
    pub fn villes(&self) -> &[Ville] {
        &self.villes
    }
    pub fn joueur_courant(&self) -> u8 {
        self.joueur_courant
    }
    pub fn tour(&self) -> u16 {
        self.tour
    }
    pub fn est_finie(&self) -> bool {
        self.finie
    }

    /// Capitale de départ du joueur.
    pub fn capitale(&self, joueur: u8) -> Case {
        self.villes.iter().find(|v| v.capitale && v.joueur == Some(joueur)).map(|v| v.case).expect("capitale")
    }

    /// État de la case `c` pour `joueur`.
    pub fn visibilite(&self, joueur: u8, c: Case) -> Visibilite {
        self.brouillards[joueur as usize].cases[c as usize]
    }

    fn unite_sur(&self, c: Case) -> Option<usize> {
        self.unites.iter().position(|u| u.case == c)
    }

    /// Vision de `joueur` refaite d'après ses unités et ses villes (§ 3.2) : ce qui sort
    /// de la vue passe à « vue autrefois », les villes vues sont notées avec leur propriétaire.
    fn recalculer_vision(&mut self, joueur: u8) {
        let b = &mut self.brouillards[joueur as usize];
        for e in b.cases.iter_mut() {
            if *e == Visibilite::Visible {
                *e = Visibilite::VueAutrefois;
            }
        }
        let p = &self.plateau;
        let sources = self
            .unites
            .iter()
            .filter(|u| u.joueur == joueur)
            .map(|u| (u.case, u.type_unite.vision()))
            .chain(self.villes.iter().filter(|v| v.joueur == Some(joueur)).map(|v| (v.case, VISION_VILLE)));
        for (c, vision) in sources {
            for w in p.carre(c, vision + p.terrain(c).bonus_vision()) {
                b.cases[w as usize] = Visibilite::Visible;
            }
        }
        for (i, v) in self.villes.iter().enumerate() {
            if b.cases[v.case as usize] == Visibilite::Visible {
                b.villes[i] = v.joueur.unwrap_or(VILLE_NEUTRE);
            }
        }
    }

    /// Unités adverses que `joueur` voit en ce moment (indices dans `unites`).
    fn adverses_visibles(&self, joueur: u8) -> Vec<usize> {
        let b = &self.brouillards[joueur as usize];
        (0..self.unites.len())
            .filter(|&i| self.unites[i].joueur != joueur && b.cases[self.unites[i].case as usize] == Visibilite::Visible)
            .collect()
    }

    /// Chemins de l'unité `i`, d'après ce que sait son joueur (§ 6.1) : cases connues,
    /// franchissables, libres de ses unités et des adverses qu'il voit ; une case voisine
    /// d'une adverse visible arrête le déplacement (zone de contrôle) ; une unité qui n'a pas
    /// encore bougé entre toujours dans une case voisine. Rien de caché n'y intervient.
    fn chemins(&self, i: usize) -> Chemins {
        let u = self.unites[i];
        let n = self.plateau.nb_cases();
        let b = &self.brouillards[u.joueur as usize];
        let mut occupe = vec![false; n];
        let mut controle = vec![false; n];
        for w in &self.unites {
            if w.joueur == u.joueur {
                occupe[w.case as usize] = true;
            } else if b.cases[w.case as usize] == Visibilite::Visible {
                occupe[w.case as usize] = true;
                for c in self.plateau.carre(w.case, 1) {
                    controle[c as usize] = true;
                }
            }
        }
        let budget = u.points_restants;
        let mut cout = vec![u8::MAX; n];
        let mut precedente = vec![u.case; n];
        cout[u.case as usize] = 0;
        // Coûts de 1 ou 2 et petit budget : une file par coût suffit (Dijkstra par seaux).
        let mut seaux: Vec<Vec<Case>> = vec![Vec::new(); budget as usize + 1];
        seaux[0].push(u.case);
        let mut v = [0; 8];
        for d in 0..=budget as usize {
            let mut k = 0;
            while k < seaux[d].len() {
                let c = seaux[d][k];
                k += 1;
                if cout[c as usize] as usize != d || (c != u.case && controle[c as usize]) {
                    continue;
                }
                let nv = self.plateau.voisines(c, &mut v);
                for &w in &v[..nv] {
                    if occupe[w as usize] || b.cases[w as usize] == Visibilite::Inconnue {
                        continue;
                    }
                    let Some(pas) = self.plateau.terrain(w).cout() else { continue };
                    let mut nd = d + pas as usize;
                    if nd > budget as usize && c == u.case && !u.a_bouge && budget > 0 {
                        nd = budget as usize;
                    }
                    if nd <= budget as usize && (nd as u8) < cout[w as usize] {
                        cout[w as usize] = nd as u8;
                        precedente[w as usize] = c;
                        seaux[nd].push(w);
                    }
                }
            }
        }
        let reste = (0..n)
            .map(|c| {
                if c == u.case as usize || cout[c] == u8::MAX {
                    None
                } else if controle[c] {
                    Some(0)
                } else {
                    Some(budget - cout[c])
                }
            })
            .collect();
        Chemins { reste, precedente }
    }

    /// Cases atteignables par l'unité posée sur `depart`, avec les points restants à
    /// l'arrivée. Ordre croissant des cases (reproductible).
    pub fn atteignables(&self, depart: Case) -> Vec<(Case, u8)> {
        let Some(i) = self.unite_sur(depart) else { return Vec::new() };
        if self.unites[i].points_restants == 0 {
            return Vec::new();
        }
        let ch = self.chemins(i);
        ch.reste.iter().enumerate().filter_map(|(c, r)| r.map(|r| (c as Case, r))).collect()
    }

    /// Actions permises, dans un ordre fixe (E4). Vide si la partie est finie.
    pub fn actions_permises(&self) -> Vec<Action> {
        let mut sortie = Vec::new();
        if self.finie {
            return sortie;
        }
        sortie.push(Action::FinDuTour);
        let mut miennes: Vec<Case> = self
            .unites
            .iter()
            .filter(|u| u.joueur == self.joueur_courant && u.points_restants > 0)
            .map(|u| u.case)
            .collect();
        miennes.sort_unstable();
        for depart in miennes {
            for (arrivee, _) in self.atteignables(depart) {
                sortie.push(Action::Deplacer { depart, arrivee });
            }
        }
        sortie
    }

    /// Masque des actions permises dans l'espace fixe de `NB_ACTIONS` numéros.
    pub fn masque(&self) -> Vec<bool> {
        let mut m = vec![false; NB_ACTIONS as usize];
        for a in self.actions_permises() {
            m[a.numero(self.reglages.taille) as usize] = true;
        }
        m
    }

    /// Applique une action ; refuse (sans rien changer) une action non permise.
    pub fn appliquer(&mut self, action: Action) -> Result<(), ActionRefusee> {
        if self.finie {
            return Err(ActionRefusee::PartieFinie);
        }
        match action {
            Action::FinDuTour => {
                self.joueur_courant = (self.joueur_courant + 1) % self.reglages.nb_joueurs;
                if self.joueur_courant == 0 {
                    if self.tour >= self.reglages.limite_tours {
                        self.finie = true;
                        return Ok(());
                    }
                    self.tour += 1;
                }
                let j = self.joueur_courant;
                for u in self.unites.iter_mut().filter(|u| u.joueur == j) {
                    u.points_restants = u.type_unite.mouvement();
                    u.a_bouge = false;
                }
                Ok(())
            }
            Action::Deplacer { depart, arrivee } => {
                let i = self.unite_sur(depart).ok_or(ActionRefusee::PasDUnite)?;
                if self.unites[i].joueur != self.joueur_courant {
                    return Err(ActionRefusee::PasLeTourDeCetteUnite);
                }
                if self.unites[i].points_restants == 0 {
                    return Err(ActionRefusee::HorsDePortee);
                }
                let ch = self.chemins(i);
                let reste = ch.reste.get(arrivee as usize).copied().flatten().ok_or(ActionRefusee::HorsDePortee)?;
                let mut chemin = vec![arrivee];
                while *chemin.last().unwrap() != depart {
                    let c = *chemin.last().unwrap();
                    chemin.push(ch.precedente[c as usize]);
                }
                chemin.pop();
                chemin.reverse();
                // Case par case (§ 6.1, surprise) : si une adverse apparaît, l'unité s'arrête
                // sur la dernière case atteinte et perd le reste de son mouvement.
                let j = self.joueur_courant;
                let avant = self.adverses_visibles(j);
                let mut surprise = false;
                for c in chemin {
                    debug_assert!(self.unite_sur(c).is_none(), "le chemin passe par une case connue et libre");
                    self.unites[i].case = c;
                    self.recalculer_vision(j);
                    if self.adverses_visibles(j).iter().any(|k| !avant.contains(k)) {
                        surprise = true;
                        break;
                    }
                }
                let u = &mut self.unites[i];
                u.points_restants = if surprise { 0 } else { reste };
                u.a_bouge = true;
                Ok(())
            }
        }
    }

    /// Applique l'action de numéro `n` (forme utilisée par l'entraînement et les replays).
    pub fn appliquer_numero(&mut self, n: u32) -> Result<(), ActionRefusee> {
        let a = Action::depuis_numero(n, self.reglages.taille).ok_or(ActionRefusee::NumeroInconnu)?;
        self.appliquer(a)
    }

    /// Score provisoire par joueur : nombre de cases découvertes (visibles ou vues autrefois).
    /// Les points de D-28 (§ 9.1) le remplaceront à l'étape e.
    pub fn scores(&self) -> Vec<i32> {
        self.brouillards
            .iter()
            .map(|b| b.cases.iter().filter(|&&e| e != Visibilite::Inconnue).count() as i32)
            .collect()
    }

    /// Gagnant à la fin de la partie : `Some(joueur)` si un seul a le meilleur score,
    /// `None` pour une égalité ou une partie en cours. Provisoire (étape e : D-28).
    pub fn gagnant(&self) -> Option<u8> {
        if !self.finie {
            return None;
        }
        let s = self.scores();
        let max = *s.iter().max()?;
        let mut meilleurs = (0..s.len()).filter(|&j| s[j] == max);
        let premier = meilleurs.next()?;
        if meilleurs.next().is_some() {
            None
        } else {
            Some(premier as u8)
        }
    }

    /// Ce que voit `joueur` (§ 3, besoin de S3) : rien de ce qu'il ne sait pas.
    pub fn vue(&self, joueur: u8) -> Vue {
        let b = &self.brouillards[joueur as usize];
        let n = self.plateau.nb_cases();
        let terrains = (0..n as Case)
            .map(|c| (b.cases[c as usize] != Visibilite::Inconnue).then(|| self.plateau.terrain(c)))
            .collect();
        let unites = self
            .unites
            .iter()
            .filter(|u| u.joueur == joueur || b.cases[u.case as usize] == Visibilite::Visible)
            .copied()
            .collect();
        let villes = self
            .villes
            .iter()
            .enumerate()
            .filter(|(i, _)| b.villes[*i] != VILLE_JAMAIS_VUE)
            .map(|(i, v)| Ville { case: v.case, joueur: (b.villes[i] != VILLE_NEUTRE).then_some(b.villes[i]), capitale: v.capitale })
            .collect();
        Vue {
            joueur,
            taille: self.reglages.taille,
            nb_joueurs: self.reglages.nb_joueurs,
            tour: self.tour,
            joueur_courant: self.joueur_courant,
            finie: self.finie,
            limite_tours: self.reglages.limite_tours,
            cases: b.cases.clone(),
            terrains,
            unites,
            villes,
        }
    }

    /// Empreinte de 64 bits de tout l'état, brouillards compris (E3).
    pub fn empreinte(&self) -> u64 {
        let mut e = Empreinte::new();
        e.ajouter_octet(self.reglages.nb_joueurs);
        e.ajouter_u32(self.reglages.taille as u32);
        for t in self.plateau.terrains_bruts() {
            e.ajouter_octet(t);
        }
        for u in &self.unites {
            e.ajouter_octet(u.joueur);
            e.ajouter_u32(u.case as u32);
            e.ajouter_octet(u.type_unite as u8);
            e.ajouter_octet(u.points_restants);
            e.ajouter_octet(u.a_bouge as u8);
        }
        for v in &self.villes {
            e.ajouter_u32(v.case as u32);
            e.ajouter_octet(v.joueur.unwrap_or(VILLE_NEUTRE));
            e.ajouter_octet(v.capitale as u8);
        }
        for b in &self.brouillards {
            for &c in &b.cases {
                e.ajouter_octet(c as u8);
            }
            for &v in &b.villes {
                e.ajouter_octet(v);
            }
        }
        e.ajouter_octet(self.joueur_courant);
        e.ajouter_u32(self.tour as u32);
        e.ajouter_octet(self.finie as u8);
        e.valeur()
    }

    /// Texte lisible d'une action, avec les adresses internes (débogage, replays ; jamais à l'écran).
    pub fn action_vers_texte(&self, a: Action) -> String {
        match a {
            Action::FinDuTour => "FIN".to_string(),
            Action::Deplacer { depart, arrivee } => {
                format!("{}>{}", self.plateau.adresse(depart), self.plateau.adresse(arrivee))
            }
        }
    }

    pub fn texte_vers_action(&self, texte: &str) -> Option<Action> {
        let texte = texte.trim();
        if texte.eq_ignore_ascii_case("FIN") {
            return Some(Action::FinDuTour);
        }
        let (a, b) = texte.split_once('>')?;
        Some(Action::Deplacer {
            depart: self.plateau.depuis_adresse(a.trim())?,
            arrivee: self.plateau.depuis_adresse(b.trim())?,
        })
    }

    /// Hasard de la partie. La V1 n'en tire plus après la création de la carte (§ 0).
    pub fn hasard_mut(&mut self) -> &mut Hasard {
        &mut self.hasard
    }
}

/// Vue d'un joueur : ce qu'il sait, rien de plus (§ 3.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vue {
    pub joueur: u8,
    pub taille: u8,
    pub nb_joueurs: u8,
    pub tour: u16,
    pub joueur_courant: u8,
    pub finie: bool,
    pub limite_tours: u16,
    /// État de chaque case pour ce joueur.
    pub cases: Vec<Visibilite>,
    /// Terrain des cases connues ; `None` pour une case inconnue.
    pub terrains: Vec<Option<Terrain>>,
    /// Ses unités et les unités adverses visibles.
    pub unites: Vec<Unite>,
    /// Villes connues, avec le dernier propriétaire vu.
    pub villes: Vec<Ville>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionRefusee {
    PartieFinie,
    PasDUnite,
    PasLeTourDeCetteUnite,
    HorsDePortee,
    NumeroInconnu,
}

/// Codage pour un réseau de neurones (E7), version du codage.
pub const VERSION_CODAGE: u32 = 2;
/// Canaux : 5 terrains (cases connues), inconnue, visible, mes unités, unités adverses
/// visibles, mes points restants (fraction du mouvement × 255), éclaireur, ma capitale,
/// villes adverses connues, case hors carte.
pub const NB_CANAUX: usize = NB_TERRAINS + 9;
/// Vecteur : tour, mon tour (0 ou 1), nombre de joueurs, limite de tours.
pub const TAILLE_VECTEUR: usize = 4;

/// Codage de la vue de `joueur` : plans `NB_CANAUX × 16 × 16` en octets, plus un petit
/// vecteur. Rien de caché n'y entre. Le plateau est retourné (gauche-droite, haut-bas)
/// pour que la capitale du joueur soit toujours dans le quart en bas à gauche.
pub fn coder(partie: &Partie, joueur: u8) -> (Vec<u8>, [u16; TAILLE_VECTEUR]) {
    let vue = partie.vue(joueur);
    let p = &partie.plateau;
    let tm = TAILLE_MAX as usize;
    let t = vue.taille as usize;
    let mut plans = vec![0u8; NB_CANAUX * tm * tm];
    let idx = |canal: usize, x: usize, y: usize| canal * tm * tm + y * tm + x;
    let cap = partie.capitale(joueur);
    let (inv_x, inv_y) = (2 * p.colonne(cap) as usize > t - 1, 2 * p.rangee(cap) as usize > t - 1);
    let orienter = |c: Case| {
        let (x, y) = (p.colonne(c) as usize, p.rangee(c) as usize);
        (if inv_x { t - 1 - x } else { x }, if inv_y { t - 1 - y } else { y })
    };
    let (c_inconnue, c_visible, c_miennes, c_adverses, c_points, c_eclaireur, c_capitale, c_villes, c_hors) =
        (NB_TERRAINS, NB_TERRAINS + 1, NB_TERRAINS + 2, NB_TERRAINS + 3, NB_TERRAINS + 4, NB_TERRAINS + 5, NB_TERRAINS + 6, NB_TERRAINS + 7, NB_TERRAINS + 8);
    for y in 0..tm {
        for x in 0..tm {
            if x >= t || y >= t {
                plans[idx(c_hors, x, y)] = 255;
            }
        }
    }
    for c in 0..p.nb_cases() as Case {
        let (x, y) = orienter(c);
        match vue.terrains[c as usize] {
            Some(terrain) => plans[idx(terrain as usize, x, y)] = 255,
            None => plans[idx(c_inconnue, x, y)] = 255,
        }
        if vue.cases[c as usize] == Visibilite::Visible {
            plans[idx(c_visible, x, y)] = 255;
        }
    }
    for u in &vue.unites {
        let (x, y) = orienter(u.case);
        if u.joueur == joueur {
            plans[idx(c_miennes, x, y)] = 255;
            plans[idx(c_points, x, y)] = (u.points_restants as u32 * 255 / u.type_unite.mouvement() as u32) as u8;
        } else {
            plans[idx(c_adverses, x, y)] = 255;
        }
        if u.type_unite == TypeUnite::Eclaireur {
            plans[idx(c_eclaireur, x, y)] = 255;
        }
    }
    for v in &vue.villes {
        let (x, y) = orienter(v.case);
        if v.joueur == Some(joueur) {
            plans[idx(c_capitale, x, y)] = 255;
        } else {
            plans[idx(c_villes, x, y)] = 255;
        }
    }
    let vecteur = [vue.tour, (vue.joueur_courant == joueur) as u16, vue.nb_joueurs as u16, vue.limite_tours];
    (plans, vecteur)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Partie de test : tout en plaine, tout connu du joueur 0, unités posées à la main.
    fn partie_plaine(unites: &[(u8, u8, u8, TypeUnite)]) -> Partie {
        let mut p = Partie::nouvelle(Reglages::default(), 1);
        for c in 0..p.plateau.nb_cases() as Case {
            p.plateau.poser(c, Terrain::Plaine);
        }
        p.unites = unites
            .iter()
            .map(|&(joueur, x, y, type_unite)| Unite {
                joueur,
                case: p.plateau.case(x, y),
                type_unite,
                points_restants: type_unite.mouvement(),
                a_bouge: false,
            })
            .collect();
        p.villes.clear();
        for b in p.brouillards.iter_mut() {
            b.villes.clear();
            for e in b.cases.iter_mut() {
                *e = Visibilite::VueAutrefois;
            }
        }
        for j in 0..2 {
            p.recalculer_vision(j);
        }
        p
    }

    #[test]
    fn meme_graine_meme_carte() {
        for nb in 2..=4 {
            let a = Partie::nouvelle(Reglages::pour(nb), 42);
            let b = Partie::nouvelle(Reglages::pour(nb), 42);
            assert_eq!(a, b);
            assert_ne!(a.empreinte(), Partie::nouvelle(Reglages::pour(nb), 43).empreinte());
        }
    }

    #[test]
    fn tailles_selon_le_nombre_de_joueurs() {
        for (nb, taille) in [(2, 11), (3, 12), (4, 14)] {
            let p = Partie::nouvelle(Reglages::pour(nb), 7);
            assert_eq!(p.plateau().taille(), taille);
            assert_eq!(p.unites().len(), 2 * nb as usize);
            assert_eq!(p.villes().len(), nb as usize);
        }
    }

    #[test]
    fn departs_conformes_sur_beaucoup_de_graines() {
        for nb in 2..=4u8 {
            let r = Reglages::pour(nb);
            for graine in 0..200 {
                let p = Partie::nouvelle(r, graine);
                let caps: Vec<Case> = (0..nb).map(|j| p.capitale(j)).collect();
                for (a, &ca) in caps.iter().enumerate() {
                    assert_eq!(p.plateau.terrain(ca), Terrain::Plaine);
                    assert!(p.plateau.distance_au_bord(ca) >= 2);
                    for &cb in &caps[a + 1..] {
                        assert!(p.plateau.distance(ca, cb) >= r.distance_capitales);
                    }
                }
                for u in p.unites() {
                    assert_eq!(p.plateau.distance(u.case, caps[u.joueur as usize]), 1);
                    assert!(p.plateau.terrain(u.case).cout().is_some());
                }
                let mut cases: Vec<Case> = p.unites().iter().map(|u| u.case).collect();
                cases.sort_unstable();
                cases.dedup();
                assert_eq!(cases.len(), p.unites().len(), "une unité par case");
            }
        }
    }

    #[test]
    fn huit_directions_et_diagonales() {
        let p = partie_plaine(&[(0, 5, 5, TypeUnite::Lancier)]);
        let a = p.atteignables(p.plateau.case(5, 5));
        assert_eq!(a.len(), 8, "un Lancier atteint ses 8 voisines");
        assert!(a.contains(&(p.plateau.case(6, 6), 0)));
        let q = partie_plaine(&[(0, 5, 5, TypeUnite::Eclaireur)]);
        // Éclaireur, 3 points en plaine : le carré 7 × 7 moins sa case.
        assert_eq!(q.atteignables(q.plateau.case(5, 5)).len(), 48);
    }

    #[test]
    fn colline_et_foret_coutent_deux_eau_et_montagne_bloquent() {
        let mut p = partie_plaine(&[(0, 5, 5, TypeUnite::Eclaireur)]);
        let pl = p.plateau.clone();
        p.plateau.poser(pl.case(6, 5), Terrain::Colline);
        p.plateau.poser(pl.case(4, 5), Terrain::Eau);
        p.plateau.poser(pl.case(5, 6), Terrain::Montagne);
        p.plateau.poser(pl.case(5, 4), Terrain::Foret);
        let a = p.atteignables(pl.case(5, 5));
        assert!(a.contains(&(pl.case(6, 5), 1)), "colline : 3 - 2 = 1");
        assert!(a.contains(&(pl.case(5, 4), 1)), "forêt : 3 - 2 = 1");
        assert!(!a.iter().any(|&(c, _)| c == pl.case(4, 5) || c == pl.case(5, 6)));
    }

    #[test]
    fn une_unite_qui_n_a_pas_bouge_entre_toujours_a_cote() {
        let mut p = partie_plaine(&[(0, 5, 5, TypeUnite::Lancier)]);
        let c = p.plateau.case(6, 5);
        p.plateau.poser(c, Terrain::Foret);
        assert!(p.atteignables(p.plateau.case(5, 5)).contains(&(c, 0)), "1 point, forêt à 2 : permis au premier pas");
    }

    #[test]
    fn zone_de_controle_arrete_le_deplacement() {
        let p = partie_plaine(&[(0, 2, 5, TypeUnite::Eclaireur), (1, 5, 5, TypeUnite::Lancier)]);
        let a = p.atteignables(p.plateau.case(2, 5));
        // (4,5) est voisine de l'adverse : on y entre, à 0 point, sans aller plus loin.
        assert!(a.contains(&(p.plateau.case(4, 5), 0)));
        assert!(!a.iter().any(|&(c, _)| c == p.plateau.case(5, 5)), "case occupée");
    }

    #[test]
    fn les_cases_inconnues_ne_se_traversent_pas() {
        let mut p = partie_plaine(&[(0, 0, 0, TypeUnite::Eclaireur)]);
        for e in p.brouillards[0].cases.iter_mut() {
            *e = Visibilite::Inconnue;
        }
        p.recalculer_vision(0);
        let a = p.atteignables(0);
        assert!(a.iter().all(|&(c, _)| p.visibilite(0, c) != Visibilite::Inconnue));
    }

    #[test]
    fn le_brouillard_se_souvient() {
        let mut p = partie_plaine(&[(0, 0, 0, TypeUnite::Lancier), (1, 10, 10, TypeUnite::Lancier)]);
        for b in p.brouillards.iter_mut() {
            for e in b.cases.iter_mut() {
                *e = Visibilite::Inconnue;
            }
        }
        p.recalculer_vision(0);
        let pl = p.plateau.clone();
        assert_eq!(p.visibilite(0, pl.case(2, 2)), Visibilite::Visible);
        assert_eq!(p.visibilite(0, pl.case(3, 0)), Visibilite::Inconnue);
        p.appliquer(Action::Deplacer { depart: 0, arrivee: pl.case(1, 0) }).unwrap();
        assert_eq!(p.visibilite(0, pl.case(3, 0)), Visibilite::Visible);
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::Deplacer { depart: pl.case(1, 0), arrivee: pl.case(1, 1) }).unwrap();
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::Deplacer { depart: pl.case(1, 1), arrivee: pl.case(2, 2) }).unwrap();
        // (0,0) est à distance 2 de (2,2) : encore visible ; (3,0) aussi. On s'éloigne encore.
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::Deplacer { depart: pl.case(2, 2), arrivee: pl.case(3, 3) }).unwrap();
        assert_eq!(p.visibilite(0, pl.case(0, 0)), Visibilite::VueAutrefois, "sortie de la vue, gardée en mémoire");
        // La vue du joueur 0 ne contient pas l'unité adverse lointaine.
        let vue = p.vue(0);
        assert_eq!(vue.unites.len(), 1);
        assert_eq!(vue.terrains[pl.case(10, 10) as usize], None);
    }

    #[test]
    fn colline_donne_plus_de_vision() {
        let mut p = partie_plaine(&[(0, 5, 5, TypeUnite::Lancier)]);
        for e in p.brouillards[0].cases.iter_mut() {
            *e = Visibilite::Inconnue;
        }
        let c = p.plateau.case(5, 5);
        p.plateau.poser(c, Terrain::Colline);
        p.recalculer_vision(0);
        assert_eq!(p.visibilite(0, p.plateau.case(8, 5)), Visibilite::Visible, "vision 2 + 1");
        assert_eq!(p.visibilite(0, p.plateau.case(9, 5)), Visibilite::Inconnue);
    }

    #[test]
    fn surprise_arrete_l_unite() {
        // Une adverse cachée derrière une case vue : l'éclaireur la découvre en chemin.
        let mut p = partie_plaine(&[(0, 0, 5, TypeUnite::Eclaireur), (1, 5, 5, TypeUnite::Lancier)]);
        let pl = p.plateau.clone();
        // Le joueur 0 ne voit pas encore l'adverse (vision 3 : jusqu'à la colonne 3).
        assert_eq!(p.visibilite(0, pl.case(5, 5)), Visibilite::VueAutrefois);
        p.appliquer(Action::Deplacer { depart: pl.case(0, 5), arrivee: pl.case(3, 5) }).unwrap();
        let u = p.unites[0];
        assert_eq!(u.case, pl.case(2, 5), "arrêtée au deuxième pas : l'adverse est apparue à distance 3");
        assert_eq!(u.points_restants, 0);
    }

    #[test]
    fn tour_par_tour_a_quatre_et_fin_de_partie() {
        let r = Reglages { limite_tours: 2, ..Reglages::pour(4) };
        let mut p = Partie::nouvelle(r, 3);
        for attendu in [(1, 1), (2, 1), (3, 1), (0, 2)] {
            p.appliquer(Action::FinDuTour).unwrap();
            assert_eq!((p.joueur_courant(), p.tour()), attendu);
        }
        for _ in 0..4 {
            p.appliquer(Action::FinDuTour).unwrap();
        }
        assert!(p.est_finie());
        assert!(p.actions_permises().is_empty());
        assert_eq!(p.appliquer(Action::FinDuTour), Err(ActionRefusee::PartieFinie));
    }

    #[test]
    fn une_action_non_permise_ne_change_rien() {
        let mut p = partie_plaine(&[(0, 0, 0, TypeUnite::Lancier), (1, 10, 10, TypeUnite::Lancier)]);
        let avant = p.clone();
        let adverse = p.plateau.case(10, 10);
        assert_eq!(
            p.appliquer(Action::Deplacer { depart: adverse, arrivee: adverse - 1 }),
            Err(ActionRefusee::PasLeTourDeCetteUnite)
        );
        let loin = p.plateau.case(10, 5);
        assert_eq!(p.appliquer(Action::Deplacer { depart: 0, arrivee: loin }), Err(ActionRefusee::HorsDePortee));
        assert_eq!(p, avant);
    }

    #[test]
    fn les_points_se_depensent_puis_reviennent() {
        let mut p = partie_plaine(&[(0, 0, 0, TypeUnite::Eclaireur)]);
        p.appliquer(Action::Deplacer { depart: 0, arrivee: p.plateau.case(2, 2) }).unwrap();
        assert_eq!(p.unites[0].points_restants, 1);
        assert!(p.unites[0].a_bouge);
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::FinDuTour).unwrap();
        assert_eq!(p.unites[0].points_restants, 3);
        assert!(!p.unites[0].a_bouge);
    }

    #[test]
    fn numeros_d_action_aller_retour() {
        for nb in 2..=4 {
            let p = Partie::nouvelle(Reglages::pour(nb), 9);
            let taille = p.reglages().taille;
            for a in p.actions_permises() {
                let n = a.numero(taille);
                assert!(n < NB_ACTIONS);
                assert_eq!(Action::depuis_numero(n, taille), Some(a));
                assert_eq!(p.texte_vers_action(&p.action_vers_texte(a)), Some(a));
            }
        }
        assert_eq!(Action::depuis_numero(NB_ACTIONS, 11), None);
        // Case hors d'un plateau 11 × 11 dans la grille 16 × 16 : numéro refusé.
        assert_eq!(Action::depuis_numero(1 + 12, 11), None);
    }

    #[test]
    fn masque_et_liste_concordent() {
        let p = Partie::nouvelle(Reglages::default(), 5);
        let m = p.masque();
        assert_eq!(m.iter().filter(|&&b| b).count(), p.actions_permises().len());
    }

    #[test]
    fn la_vue_ne_dit_rien_de_cache() {
        let p = Partie::nouvelle(Reglages::pour(4), 11);
        for j in 0..4 {
            let v = p.vue(j);
            for u in &v.unites {
                assert!(u.joueur == j || p.visibilite(j, u.case) == Visibilite::Visible);
            }
            for c in 0..p.plateau.nb_cases() {
                assert_eq!(v.terrains[c].is_some(), v.cases[c] != Visibilite::Inconnue);
            }
            assert!(v.villes.iter().any(|vi| vi.joueur == Some(j) && vi.capitale));
        }
    }

    #[test]
    fn codage_du_point_de_vue_de_chaque_joueur() {
        let p = Partie::nouvelle(Reglages::pour(4), 8);
        let tm = TAILLE_MAX as usize;
        for j in 0..4 {
            let (plans, v) = coder(&p, j);
            assert_eq!(plans.len(), NB_CANAUX * tm * tm);
            let canal = |k: usize| (0..tm * tm).filter(|&i| plans[k * tm * tm + i] == 255).count();
            assert_eq!(canal(NB_TERRAINS + 2), 2, "mes deux unités");
            assert_eq!(canal(NB_TERRAINS + 6), 1, "ma capitale");
            // Ma capitale est dans le quart en bas à gauche.
            let i = (0..tm * tm).find(|&i| plans[(NB_TERRAINS + 6) * tm * tm + i] == 255).unwrap();
            assert!(i % tm <= 6 && i / tm <= 6);
            assert_eq!((v[1], v[2]), ((j == 0) as u16, 4));
            // Hors carte : colonnes 14 et 15.
            assert_eq!(plans[(NB_TERRAINS + 8) * tm * tm + 14], 255);
        }
    }
}
