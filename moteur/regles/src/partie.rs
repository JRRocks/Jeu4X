//! Une partie : réglages, état, actions permises, application d'une action.
//!
//! Interface demandée par la session 3 (`docs/ia/besoins-moteur.md`, § 3) :
//! nouvelle partie depuis une graine, joueur courant, masque des actions permises dans un
//! espace fixe, application, copie (`Clone`), fin de partie et scores, vue d'un joueur
//! (brouillard), codage en plans, actions en texte, empreinte.
//!
//! Règles V1 (`docs/game-design/regles-v1.md`, D-30) : 2 à 4 joueurs, 8 directions,
//! brouillard qui se souvient, villes et villages, vivres, croissance avec cadeau,
//! exploitations, bâtiments et merveilles, combat sans hasard, arbre en trois branches,
//! quatre clans, ruines, victoire par la capitale ou le seuil, élimination, nulle au tour 80.

use crate::carte;
use crate::donnees::*;
use crate::empreinte::Empreinte;
use crate::hasard::Hasard;
use crate::plateau::{Case, Plateau, Terrain, NB_TERRAINS, TAILLE_MAX};

pub use crate::donnees::{Avancee, Batiment, Clan, Exploitation, TypeUnite};

/// Nombre de joueurs le plus grand permis.
pub const JOUEURS_MAX: u8 = 4;
/// Vision d'une ville (§ 3.2).
pub const VISION_VILLE: u8 = 2;
const AUCUNE: u8 = 0xFF;
const DETRUITE: u8 = 0xFE;

/// Réglages d'une partie (§ 12.1). Avec la graine, ils forment le code de carte (§ 2.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Reglages {
    /// 2 à 4.
    pub nb_joueurs: u8,
    /// Côté du plateau carré (11, 12 ou 14 selon le nombre de joueurs ; 16 au plus).
    pub taille: u8,
    /// Distance minimale entre deux capitales.
    pub distance_capitales: u8,
    /// Limite de sécurité : partie nulle après ce tour (§ 9.4).
    pub limite_tours: u16,
    /// Clan de chaque joueur (indices de [`Clan`]), dans l'ordre des joueurs.
    pub clans: [u8; 4],
}

impl Reglages {
    /// Réglages de la V1 pour `nb_joueurs` joueurs (§ 1.2, § 12.1). Clans 0, 1, 2, 3.
    pub fn pour(nb_joueurs: u8) -> Reglages {
        let (taille, distance_capitales) = match nb_joueurs {
            2 => (11, 6),
            3 => (12, 6),
            4 => (14, 7),
            _ => panic!("2 à 4 joueurs, pas {nb_joueurs}"),
        };
        Reglages { nb_joueurs, taille, distance_capitales, limite_tours: 80, clans: [0, 1, 2, 3] }
    }

    pub fn seuil(&self) -> i32 {
        villages_ruines_seuil(self.nb_joueurs).2
    }
}

impl Default for Reglages {
    fn default() -> Self {
        Reglages::pour(2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Unite {
    pub joueur: u8,
    pub case: Case,
    pub type_unite: TypeUnite,
    pub pv: i32,
    pub points_restants: u8,
    /// A bougé pendant ce tour de son joueur (§ 6.1).
    pub a_bouge: bool,
    /// A attaqué, rallié ou regardé à la lunette pendant ce tour.
    pub a_attaque: bool,
    /// Faux pour une unité produite ou reçue pendant ce tour : elle agira au suivant.
    pub peut_agir: bool,
}

/// Une ville, ou un village neutre (`joueur == None`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ville {
    pub case: Case,
    pub joueur: Option<u8>,
    /// Capitale de départ de ce joueur (le reste même prise).
    pub capitale_de: Option<u8>,
    pub niveau: u8,
    /// Récompenses de croissance (§ 4.1), perdues si la ville est prise.
    pub bonus_revenu: i32,
    pub bonus_entretien: i32,
    pub bonus_points: i32,
    pub rayon: u8,
    /// Bâtiments présents (bits de [`Batiment`]).
    pub batiments: u8,
    /// A produit une unité pendant ce tour.
    pub a_produit: bool,
}

impl Ville {
    pub fn a(&self, b: Batiment) -> bool {
        self.batiments & b.bit() != 0
    }
    pub fn capitale(&self) -> bool {
        self.capitale_de.is_some() && self.capitale_de == self.joueur
    }
}

/// Un joueur : clan, stock, avancées, points gardés pour toujours.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Joueur {
    pub clan: Clan,
    pub vivres: i32,
    pub avancees: u16,
    /// Points acquis (§ 9.1) : avancées, unités détruites, ruines.
    pub points_acquis: i32,
    pub elimine: bool,
    /// Annoncé au seuil à la fin de son dernier tour (§ 9.2).
    pub annonce: bool,
}

impl Joueur {
    pub fn possede(&self, a: Avancee) -> bool {
        self.avancees & a.bit() != 0
    }
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
    FinDuTour,
    /// Déplacer l'unité posée sur `depart` jusqu'à `arrivee`, par le chemin le moins cher.
    Deplacer { depart: Case, arrivee: Case },
    /// L'unité posée sur `depart` attaque l'unité posée sur `cible`.
    Attaquer { depart: Case, cible: Case },
    /// L'Astronome posé sur `depart` regarde autour de `cible` (§ 7).
    Lunette { depart: Case, cible: Case },
    /// L'unité posée sur `case` rallie le village ou prend la ville (§ 4.2).
    Rallier { case: Case },
    /// L'unité posée sur `case` disparaît.
    Renvoyer { case: Case },
    /// La ville posée sur `ville` produit une unité.
    Produire { ville: Case, type_unite: TypeUnite },
    /// La ville monte d'un niveau ; `recompense_b` choisit le cadeau b, sinon a.
    Agrandir { ville: Case, recompense_b: bool },
    /// Poser l'exploitation que permet le terrain de `case`.
    Exploiter { case: Case },
    Batir { ville: Case, batiment: Batiment },
    Rechercher { avancee: Avancee },
}

/// Nombre de cases de l'espace d'actions (plateau le plus grand permis).
pub const CASES_MAX: u32 = TAILLE_MAX as u32 * TAILLE_MAX as u32;
const PORTEE_ATT: i32 = 3;
const COTE_ATT: u32 = (2 * PORTEE_ATT + 1) as u32;
const PORTEE_LUN: i32 = 4;
const COTE_LUN: u32 = (2 * PORTEE_LUN + 1) as u32;
/// Début de chaque bloc de numéros (E4). Les cases sont numérotées sur une grille 16 × 16.
pub const B_DEPLACER: u32 = 1;
pub const B_ATTAQUER: u32 = B_DEPLACER + CASES_MAX * CASES_MAX;
pub const B_LUNETTE: u32 = B_ATTAQUER + CASES_MAX * COTE_ATT * COTE_ATT;
pub const B_RALLIER: u32 = B_LUNETTE + CASES_MAX * COTE_LUN * COTE_LUN;
pub const B_RENVOYER: u32 = B_RALLIER + CASES_MAX;
pub const B_PRODUIRE: u32 = B_RENVOYER + CASES_MAX;
pub const B_AGRANDIR: u32 = B_PRODUIRE + CASES_MAX * NB_TYPES_UNITE as u32;
pub const B_EXPLOITER: u32 = B_AGRANDIR + CASES_MAX * 2;
pub const B_BATIR: u32 = B_EXPLOITER + CASES_MAX;
pub const B_RECHERCHER: u32 = B_BATIR + CASES_MAX * NB_BATIMENTS as u32;
/// Taille de l'espace d'actions.
pub const NB_ACTIONS: u32 = B_RECHERCHER + NB_AVANCEES as u32;

fn decalage(depart: Case, cible: Case, taille: u8, portee: i32) -> Option<u32> {
    let t = taille as i32;
    let (dx, dy) = (cible as i32 % t - depart as i32 % t, cible as i32 / t - depart as i32 / t);
    if dx.abs() > portee || dy.abs() > portee {
        return None;
    }
    let cote = 2 * portee + 1;
    Some(((dy + portee) * cote + dx + portee) as u32)
}

fn depuis_decalage(depart: Case, k: u32, taille: u8, portee: i32) -> Option<Case> {
    let cote = 2 * portee + 1;
    let (dx, dy) = (k as i32 % cote - portee, k as i32 / cote - portee);
    let t = taille as i32;
    let (x, y) = (depart as i32 % t + dx, depart as i32 / t + dy);
    (x >= 0 && y >= 0 && x < t && y < t).then(|| (y * t + x) as Case)
}

impl Action {
    /// Numéro fixe de l'action (E4). `taille` : côté du plateau de la partie. Les actions
    /// dont la cible sort de la zone de leur bloc (impossibles en jeu) rendent `u32::MAX`.
    pub fn numero(self, taille: u8) -> u32 {
        let g = |c: Case| vers_grille(c, taille);
        match self {
            Action::FinDuTour => 0,
            Action::Deplacer { depart, arrivee } => B_DEPLACER + g(depart) * CASES_MAX + g(arrivee),
            Action::Attaquer { depart, cible } => match decalage(depart, cible, taille, PORTEE_ATT) {
                Some(k) => B_ATTAQUER + g(depart) * COTE_ATT * COTE_ATT + k,
                None => u32::MAX,
            },
            Action::Lunette { depart, cible } => match decalage(depart, cible, taille, PORTEE_LUN) {
                Some(k) => B_LUNETTE + g(depart) * COTE_LUN * COTE_LUN + k,
                None => u32::MAX,
            },
            Action::Rallier { case } => B_RALLIER + g(case),
            Action::Renvoyer { case } => B_RENVOYER + g(case),
            Action::Produire { ville, type_unite } => B_PRODUIRE + g(ville) * NB_TYPES_UNITE as u32 + type_unite as u32,
            Action::Agrandir { ville, recompense_b } => B_AGRANDIR + g(ville) * 2 + recompense_b as u32,
            Action::Exploiter { case } => B_EXPLOITER + g(case),
            Action::Batir { ville, batiment } => B_BATIR + g(ville) * NB_BATIMENTS as u32 + batiment as u32,
            Action::Rechercher { avancee } => B_RECHERCHER + avancee as u32,
        }
    }

    /// Inverse de [`Action::numero`] ; `None` si le numéro sort de l'espace ou du plateau.
    pub fn depuis_numero(n: u32, taille: u8) -> Option<Action> {
        let c = |g: u32| depuis_grille(g, taille);
        Some(if n == 0 {
            Action::FinDuTour
        } else if n < B_ATTAQUER {
            let k = n - B_DEPLACER;
            Action::Deplacer { depart: c(k / CASES_MAX)?, arrivee: c(k % CASES_MAX)? }
        } else if n < B_LUNETTE {
            let k = n - B_ATTAQUER;
            let depart = c(k / (COTE_ATT * COTE_ATT))?;
            Action::Attaquer { depart, cible: depuis_decalage(depart, k % (COTE_ATT * COTE_ATT), taille, PORTEE_ATT)? }
        } else if n < B_RALLIER {
            let k = n - B_LUNETTE;
            let depart = c(k / (COTE_LUN * COTE_LUN))?;
            Action::Lunette { depart, cible: depuis_decalage(depart, k % (COTE_LUN * COTE_LUN), taille, PORTEE_LUN)? }
        } else if n < B_RENVOYER {
            Action::Rallier { case: c(n - B_RALLIER)? }
        } else if n < B_PRODUIRE {
            Action::Renvoyer { case: c(n - B_RENVOYER)? }
        } else if n < B_AGRANDIR {
            let k = n - B_PRODUIRE;
            Action::Produire { ville: c(k / NB_TYPES_UNITE as u32)?, type_unite: TypeUnite::depuis_u8((k % NB_TYPES_UNITE as u32) as u8)? }
        } else if n < B_EXPLOITER {
            let k = n - B_AGRANDIR;
            Action::Agrandir { ville: c(k / 2)?, recompense_b: k % 2 == 1 }
        } else if n < B_BATIR {
            Action::Exploiter { case: c(n - B_EXPLOITER)? }
        } else if n < B_RECHERCHER {
            let k = n - B_BATIR;
            Action::Batir { ville: c(k / NB_BATIMENTS as u32)?, batiment: Batiment::TOUS[(k % NB_BATIMENTS as u32) as usize] }
        } else if n < NB_ACTIONS {
            Action::Rechercher { avancee: Avancee::depuis_u8((n - B_RECHERCHER) as u8)? }
        } else {
            return None;
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

/// Annonce d'un combat (§ 6.3, GD-007), calculée sans rien changer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Annonce {
    pub degats: i32,
    pub riposte: i32,
    pub cible_detruite: bool,
    pub attaquant_detruit: bool,
}

/// Résultat du calcul des déplacements : points restants et case précédente sur le chemin.
struct Chemins {
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
    joueurs: Vec<Joueur>,
    brouillards: Vec<Brouillard>,
    /// Ville propriétaire de chaque case (indice dans `villes`), ou `AUCUNE`.
    territoire: Vec<u8>,
    /// Exploitation de chaque case, ou `AUCUNE`.
    exploitations: Vec<u8>,
    /// Trésor de la ruine de chaque case, ou `AUCUNE`.
    ruines: Vec<u8>,
    /// Ville qui tient chaque merveille (arène, verger, tour), `AUCUNE` ou `DETRUITE`.
    merveilles: [u8; 3],
    /// Cases regardées à la lunette pendant ce tour (centres).
    lunettes: Vec<Case>,
    joueur_courant: u8,
    tour: u16,
    hasard: Hasard,
    finie: bool,
    gagnant: Option<u8>,
}

/// Essais de graines dérivées au plus (§ 2.3) ; en pratique un ou deux suffisent.
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
        let n = plateau.nb_cases();
        let mut villes: Vec<Ville> = Vec::new();
        let ville = |case, joueur: Option<u8>| Ville {
            case,
            joueur,
            capitale_de: joueur,
            niveau: 1,
            bonus_revenu: 0,
            bonus_entretien: 0,
            bonus_points: 0,
            rayon: 1,
            batiments: 0,
            a_produit: false,
        };
        for (j, &case) in carte.capitales.iter().enumerate() {
            villes.push(ville(case, Some(j as u8)));
        }
        for &case in &carte.villages {
            villes.push(ville(case, None));
        }
        let mut ruines = vec![AUCUNE; n];
        for &(c, t) in &carte.ruines {
            ruines[c as usize] = t as u8;
        }
        // Ordre des joueurs tiré par la graine (§ 2.6) : les capitales le sont déjà ;
        // le joueur de rang k reçoit k vivres de plus.
        let joueurs: Vec<Joueur> = (0..r.nb_joueurs)
            .map(|j| Joueur {
                clan: Clan::depuis_u8(r.clans[j as usize]),
                vivres: 5 + j as i32,
                avancees: 0,
                points_acquis: 0,
                elimine: false,
                annonce: false,
            })
            .collect();
        let brouillards = (0..r.nb_joueurs)
            .map(|_| Brouillard { cases: vec![Visibilite::Inconnue; n], villes: vec![AUCUNE; villes.len()] })
            .collect();
        let mut p = Partie {
            reglages: r,
            graine,
            plateau,
            unites: Vec::new(),
            villes,
            joueurs,
            brouillards,
            territoire: vec![AUCUNE; n],
            exploitations: vec![AUCUNE; n],
            ruines,
            merveilles: [AUCUNE; 3],
            lunettes: Vec::new(),
            joueur_courant: 0,
            tour: 1,
            hasard: Hasard::new(0),
            finie: false,
            gagnant: None,
        };
        for j in 0..r.nb_joueurs {
            p.etendre_territoire(j as usize);
            if p.joueurs[j as usize].clan == Clan::Murailliers {
                p.villes[j as usize].batiments |= Batiment::Remparts.bit();
            }
        }
        // Départ (§ 5.2) : un Lancier et l'éclaireur du clan sur deux voisines de la capitale,
        // franchissables, tirées au hasard de la carte.
        let mut v = [0; 8];
        for (j, &cap) in carte.capitales.iter().enumerate() {
            let nv = p.plateau.voisines(cap, &mut v);
            let mut libres: Vec<Case> =
                v[..nv].iter().copied().filter(|&w| p.plateau.terrain(w).cout().is_some() && p.ruines[w as usize] == AUCUNE).collect();
            for type_unite in [TypeUnite::Lancier, p.joueurs[j].clan.eclaireur()] {
                if libres.is_empty() {
                    break;
                }
                let case = libres.remove(hasard.sous(libres.len() as u32) as usize);
                p.creer_unite(j as u8, type_unite, case, true);
            }
        }
        p.hasard = hasard;
        p.recalculer_toutes_les_visions();
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
    pub fn joueurs(&self) -> &[Joueur] {
        &self.joueurs
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
    /// Exploitation de la case, s'il y en a une.
    pub fn exploitation(&self, c: Case) -> Option<Exploitation> {
        match self.exploitations[c as usize] {
            0 => Some(Exploitation::Cueillette),
            1 => Some(Exploitation::Champ),
            2 => Some(Exploitation::Carriere),
            _ => None,
        }
    }
    /// Une ruine est-elle sur la case (son trésor reste caché) ?
    pub fn ruine(&self, c: Case) -> bool {
        self.ruines[c as usize] != AUCUNE
    }
    /// Ville propriétaire de la case, s'il y en a une.
    pub fn territoire(&self, c: Case) -> Option<&Ville> {
        let v = self.territoire[c as usize];
        (v != AUCUNE).then(|| &self.villes[v as usize])
    }
    /// Ville qui tient la merveille `k` (0 arène, 1 verger, 2 tour), si elle est bâtie et debout.
    pub fn merveille(&self, k: usize) -> Option<&Ville> {
        let v = self.merveilles[k];
        (v != AUCUNE && v != DETRUITE).then(|| &self.villes[v as usize])
    }
    /// La merveille `k` ne peut plus être bâtie (déjà bâtie ou détruite).
    pub fn merveille_prise(&self, k: usize) -> bool {
        self.merveilles[k] != AUCUNE
    }

    /// Capitale de départ du joueur.
    pub fn capitale(&self, joueur: u8) -> Case {
        self.villes.iter().find(|v| v.capitale_de == Some(joueur)).map(|v| v.case).expect("capitale")
    }

    /// État de la case `c` pour `joueur`.
    pub fn visibilite(&self, joueur: u8, c: Case) -> Visibilite {
        self.brouillards[joueur as usize].cases[c as usize]
    }

    fn unite_sur(&self, c: Case) -> Option<usize> {
        self.unites.iter().position(|u| u.case == c)
    }

    fn ville_sur(&self, c: Case) -> Option<usize> {
        self.villes.iter().position(|v| v.case == c)
    }

    fn vivant(&self, j: u8) -> bool {
        !self.joueurs[j as usize].elimine
    }

    fn possede(&self, j: u8, a: Avancee) -> bool {
        self.joueurs[j as usize].possede(a)
    }

    /// Le joueur tient-il la merveille `k` ?
    fn tient_merveille(&self, j: u8, k: usize) -> bool {
        self.merveille(k).is_some_and(|v| v.joueur == Some(j))
    }

    /// Points de vie maximum (§ 12.3 ; +2 avec l'Arène des Crêtes).
    pub fn pv_max(&self, u: &Unite) -> i32 {
        u.type_unite.fiche().pv + if self.tient_merveille(u.joueur, 0) { 2 } else { 0 }
    }

    fn creer_unite(&mut self, joueur: u8, type_unite: TypeUnite, case: Case, peut_agir: bool) {
        let mut u = Unite { joueur, case, type_unite, pv: 0, points_restants: 0, a_bouge: false, a_attaque: false, peut_agir };
        u.pv = self.pv_max(&u);
        if peut_agir {
            u.points_restants = type_unite.mouvement();
        }
        self.unites.push(u);
    }

    /// Case libre la plus proche de `pres_de` où l'unité peut se tenir, la plus basse dans
    /// l'ordre des cases à égalité (§ 14.1).
    fn case_libre_proche(&self, pres_de: Case, type_unite: TypeUnite) -> Option<Case> {
        let t = self.plateau.taille();
        (0..t).find_map(|r| {
            self.plateau
                .carre(pres_de, r)
                .filter(|&c| self.plateau.distance(c, pres_de) == r)
                .find(|&c| self.unite_sur(c).is_none() && self.cout_entree(type_unite, 0, self.plateau.terrain(c)).is_some())
        })
    }

    fn donner_unite(&mut self, joueur: u8, type_unite: TypeUnite, pres_de: Case) {
        if let Some(c) = self.case_libre_proche(pres_de, type_unite) {
            self.creer_unite(joueur, type_unite, c, false);
        }
    }

    /// Coût d'entrée d'une unité dans un terrain, `None` si elle ne peut pas y entrer.
    fn cout_entree(&self, type_unite: TypeUnite, joueur: u8, t: Terrain) -> Option<u8> {
        match t {
            Terrain::Montagne if type_unite == TypeUnite::GuetteurPlaneur => Some(2),
            Terrain::Foret if (joueur as usize) < self.joueurs.len() && self.possede(joueur, Avancee::Sentiers) => Some(1),
            _ => t.cout(),
        }
    }

    fn vision_unite(&self, u: &Unite) -> u8 {
        let j = &self.joueurs[u.joueur as usize];
        u.type_unite.vision()
            + self.plateau.terrain(u.case).bonus_vision()
            + (j.clan == Clan::VentsErrants) as u8
            + j.possede(Avancee::Cartographes) as u8
    }

    fn vision_ville(&self, v: &Ville) -> u8 {
        let j = v.joueur.expect("ville tenue");
        VISION_VILLE + self.possede(j, Avancee::Cartographes) as u8 + if v.a(Batiment::TourBrumes) { 2 } else { 0 }
    }

    /// Vision de `joueur` refaite d'après ses unités, ses villes et ses coups de lunette
    /// (§ 3.2) : ce qui sort de la vue passe à « vue autrefois », les villes vues sont
    /// notées avec leur propriétaire.
    fn recalculer_vision(&mut self, joueur: u8) {
        let mut sources: Vec<(Case, u8)> = Vec::new();
        if self.vivant(joueur) {
            for u in self.unites.iter().filter(|u| u.joueur == joueur) {
                sources.push((u.case, self.vision_unite(u)));
            }
            for v in self.villes.iter().filter(|v| v.joueur == Some(joueur)) {
                sources.push((v.case, self.vision_ville(v)));
            }
            if joueur == self.joueur_courant {
                sources.extend(self.lunettes.iter().map(|&c| (c, 1)));
            }
        }
        let b = &mut self.brouillards[joueur as usize];
        for e in b.cases.iter_mut() {
            if *e == Visibilite::Visible {
                *e = Visibilite::VueAutrefois;
            }
        }
        for (c, vision) in sources {
            for w in self.plateau.carre(c, vision) {
                b.cases[w as usize] = Visibilite::Visible;
            }
        }
        for (i, v) in self.villes.iter().enumerate() {
            if b.cases[v.case as usize] == Visibilite::Visible {
                b.villes[i] = v.joueur.unwrap_or(VILLE_NEUTRE);
            }
        }
    }

    fn recalculer_toutes_les_visions(&mut self) {
        for j in 0..self.reglages.nb_joueurs {
            self.recalculer_vision(j);
        }
    }

    /// Donne à `joueur` la connaissance des cases à `rayon` de `c` (au moins « vue autrefois »).
    fn reveler(&mut self, joueur: u8, c: Case, rayon: u8) {
        let cases: Vec<Case> = self.plateau.carre(c, rayon).collect();
        let b = &mut self.brouillards[joueur as usize];
        for w in cases {
            if b.cases[w as usize] == Visibilite::Inconnue {
                b.cases[w as usize] = Visibilite::VueAutrefois;
            }
        }
        for (i, v) in self.villes.iter().enumerate() {
            if b.cases[v.case as usize] != Visibilite::Inconnue && b.villes[i] == AUCUNE {
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

    /// Le territoire de la ville `i` prend les cases libres à son rayon (§ 4.3).
    fn etendre_territoire(&mut self, i: usize) {
        let v = self.villes[i];
        let cases: Vec<Case> = self.plateau.carre(v.case, v.rayon).collect();
        for c in cases {
            if self.territoire[c as usize] == AUCUNE {
                self.territoire[c as usize] = i as u8;
            }
        }
        self.territoire[v.case as usize] = i as u8;
    }

    // ---------- Économie ----------

    /// Revenu d'une ville par tour (§ 5.1).
    pub fn revenu_ville(&self, i: usize) -> i32 {
        let v = &self.villes[i];
        let mut r = REVENU_NIVEAU[v.niveau as usize - 1] + v.bonus_revenu;
        if v.capitale() {
            r += BONUS_REVENU_CAPITALE;
        }
        if v.a(Batiment::Marche) {
            r += 2;
        }
        if v.a(Batiment::Verger) {
            r += 4;
        }
        for (c, &e) in self.exploitations.iter().enumerate() {
            if e != AUCUNE && self.territoire[c] == i as u8 {
                r += [1, 1, 2][e as usize];
            }
        }
        r
    }

    /// Revenu du joueur au début de son prochain tour (affiché en permanence, GD-004).
    pub fn revenu(&self, j: u8) -> i32 {
        (0..self.villes.len()).filter(|&i| self.villes[i].joueur == Some(j)).map(|i| self.revenu_ville(i)).sum()
    }

    /// Unités que le joueur peut entretenir (§ 4.4).
    pub fn entretien(&self, j: u8) -> i32 {
        self.villes
            .iter()
            .filter(|v| v.joueur == Some(j))
            .map(|v| ENTRETIEN_NIVEAU[v.niveau as usize - 1] + v.bonus_entretien + if v.capitale() { BONUS_ENTRETIEN_CAPITALE } else { 0 })
            .sum()
    }

    pub fn nb_unites(&self, j: u8) -> i32 {
        self.unites.iter().filter(|u| u.joueur == j).count() as i32
    }

    /// Score public du joueur (§ 9.1) : points tenus et acquis.
    pub fn score(&self, j: u8) -> i32 {
        let mut s = self.joueurs[j as usize].points_acquis;
        for v in self.villes.iter().filter(|v| v.joueur == Some(j)) {
            s += POINTS_NIVEAU[v.niveau as usize - 1] + v.bonus_points;
            for b in [Batiment::Arene, Batiment::Verger, Batiment::TourBrumes] {
                if v.a(b) {
                    s += POINTS_MERVEILLE;
                }
            }
        }
        s
    }

    /// Scores de tous les joueurs (§ 9.1).
    pub fn scores(&self) -> Vec<i32> {
        (0..self.reglages.nb_joueurs).map(|j| self.score(j)).collect()
    }

    /// Prix d'une avancée pour le joueur (§ 7, § 8.3 : Lecture du ciel, Écoles), 1 au moins.
    pub fn prix_avancee(&self, j: u8, a: Avancee) -> i32 {
        let mut p = a.prix_de_base();
        if self.joueurs[j as usize].clan == Clan::VeilleursAstres {
            p = p * 4 / 5;
        }
        if self.possede(j, Avancee::Ecoles) {
            p -= 2;
        }
        p.max(1)
    }

    /// L'avancée est-elle à la portée du joueur (sans compter les vivres) ?
    pub fn avancee_ouverte(&self, j: u8, a: Avancee) -> bool {
        let jo = &self.joueurs[j as usize];
        !jo.possede(a)
            && a.exclue_par().is_none_or(|x| !jo.possede(x))
            && (a.prerequis().is_empty() || a.prerequis().iter().any(|&x| jo.possede(x)))
    }

    /// Prix de la prochaine croissance de la ville (Greniers : 2 de moins).
    pub fn prix_croissance(&self, i: usize) -> i32 {
        let v = &self.villes[i];
        let j = v.joueur.expect("ville tenue");
        (COUT_NIVEAU[(v.niveau as usize).min(4)] - if self.possede(j, Avancee::Greniers) { 2 } else { 0 }).max(1)
    }

    /// Prix d'un bâtiment pour le joueur (Pierres levées : remparts à 3).
    pub fn prix_batiment(&self, j: u8, b: Batiment) -> i32 {
        match b {
            Batiment::Remparts if self.joueurs[j as usize].clan == Clan::Murailliers => 3,
            Batiment::Remparts | Batiment::Marche => 6,
            _ => 15,
        }
    }

    pub fn unite_debloquee(&self, j: u8, t: TypeUnite) -> bool {
        match t {
            TypeUnite::Eclaireur | TypeUnite::Lancier => true,
            TypeUnite::Archer => self.possede(j, Avancee::Arcs),
            TypeUnite::Cavalier => self.possede(j, Avancee::Dressage),
            TypeUnite::Catapulte => self.possede(j, Avancee::Mecanique),
            propre => self.joueurs[j as usize].clan.unite_propre() == propre,
        }
    }

    // ---------- Déplacements ----------

    /// Chemins de l'unité `i`, d'après ce que sait son joueur (§ 6.1) : cases connues,
    /// franchissables, libres de ses unités et des adverses qu'il voit ; une case voisine
    /// d'une adverse visible arrête le déplacement (zone de contrôle), une ruine aussi
    /// (§ 14.9) ; une unité qui n'a pas encore bougé entre toujours dans une case voisine.
    fn chemins(&self, i: usize) -> Chemins {
        let u = self.unites[i];
        let n = self.plateau.nb_cases();
        let b = &self.brouillards[u.joueur as usize];
        let mut occupe = vec![false; n];
        let mut arret = vec![false; n];
        for w in &self.unites {
            if w.joueur == u.joueur {
                occupe[w.case as usize] = true;
            } else if b.cases[w.case as usize] == Visibilite::Visible {
                occupe[w.case as usize] = true;
                for c in self.plateau.carre(w.case, 1) {
                    arret[c as usize] = true;
                }
            }
        }
        for c in 0..n {
            if self.ruines[c] != AUCUNE && b.cases[c] != Visibilite::Inconnue {
                arret[c] = true;
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
                if cout[c as usize] as usize != d || (c != u.case && arret[c as usize]) {
                    continue;
                }
                let nv = self.plateau.voisines(c, &mut v);
                for &w in &v[..nv] {
                    if occupe[w as usize] || b.cases[w as usize] == Visibilite::Inconnue {
                        continue;
                    }
                    let Some(pas) = self.cout_entree(u.type_unite, u.joueur, self.plateau.terrain(w)) else { continue };
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
                } else if arret[c] {
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
        let u = &self.unites[i];
        if u.points_restants == 0 || !u.peut_agir {
            return Vec::new();
        }
        let ch = self.chemins(i);
        ch.reste.iter().enumerate().filter_map(|(c, r)| r.map(|r| (c as Case, r))).collect()
    }

    // ---------- Combat ----------

    /// Cases que l'unité posée sur `depart` peut attaquer maintenant (§ 6.1, § 6.2).
    pub fn cibles(&self, depart: Case) -> Vec<Case> {
        let Some(i) = self.unite_sur(depart) else { return Vec::new() };
        let u = self.unites[i];
        let f = u.type_unite.fiche();
        if !u.peut_agir || u.a_attaque || u.type_unite.sans_attaque() || f.portee == 0 {
            return Vec::new();
        }
        let mut c: Vec<Case> = self
            .adverses_visibles(u.joueur)
            .into_iter()
            .map(|k| self.unites[k].case)
            .filter(|&w| {
                let d = self.plateau.distance(depart, w);
                d >= f.portee_min && d <= f.portee
            })
            .collect();
        c.sort_unstable();
        c
    }

    /// Défense en ville de l'unité, selon la ville de sa case (§ 6.3).
    fn defense_ville(&self, d: &Unite) -> i32 {
        match self.ville_sur(d.case) {
            Some(i) if self.villes[i].joueur == Some(d.joueur) => {
                if self.villes[i].a(Batiment::Remparts) {
                    4
                } else {
                    2
                }
            }
            _ => 0,
        }
    }

    /// +1 en défense si un Porte-pavois ami est voisin (sans cumul).
    fn garde_voisine(&self, d: &Unite) -> i32 {
        self.unites
            .iter()
            .any(|w| w.joueur == d.joueur && w.type_unite == TypeUnite::PortePavois && w.case != d.case && self.plateau.distance(w.case, d.case) == 1)
            as i32
    }

    fn calculer_combat(&self, a: &Unite, d: &Unite) -> Annonce {
        let (fa, fd) = (a.type_unite.fiche(), d.type_unite.fiche());
        let distance = self.plateau.distance(a.case, d.case);
        let mut attaque = fa.attaque + fa.classe.bonus_contre(fd.classe);
        if fa.classe == Classe::Siege && self.ville_sur(d.case).is_some() {
            attaque += 4;
        }
        if distance == 1 && self.possede(a.joueur, Avancee::Forge) {
            attaque += 1;
        }
        let defense = fd.defense
            + fd.classe.bonus_contre(fa.classe)
            + defense_terrain(self.plateau.terrain(d.case))
            + self.defense_ville(d)
            + self.garde_voisine(d);
        let degats = (4 + attaque - defense).max(1);
        let cible_detruite = degats >= d.pv;
        let riposte = if !cible_detruite && !d.type_unite.sans_attaque() && distance >= fd.portee_min && distance <= fd.portee {
            let att_d = fd.attaque + fd.classe.bonus_contre(fa.classe);
            let def_a = fa.defense + fa.classe.bonus_contre(fd.classe) + defense_terrain(self.plateau.terrain(a.case));
            ((4 + att_d - def_a) / 2).max(0)
        } else {
            0
        };
        Annonce { degats, riposte, cible_detruite, attaquant_detruit: riposte >= a.pv }
    }

    /// Annonce du combat (GD-007), sans rien changer ; `None` si l'attaque n'est pas permise.
    pub fn annonce_combat(&self, depart: Case, cible: Case) -> Option<Annonce> {
        if !self.cibles(depart).contains(&cible) {
            return None;
        }
        let a = self.unites[self.unite_sur(depart)?];
        let d = self.unites[self.unite_sur(cible)?];
        Some(self.calculer_combat(&a, &d))
    }

    /// Une unité de `vainqueur` en a détruit une : point acquis, Razzia des Crocs-Rouges.
    fn compter_destruction(&mut self, vainqueur: u8) {
        let j = &mut self.joueurs[vainqueur as usize];
        j.points_acquis += 1;
        if j.clan == Clan::CrocsRouges {
            j.vivres += 2;
        }
    }

    // ---------- Villes, élimination, victoire ----------

    /// Élimine le joueur (§ 9.2, § 9.3, § 14.12) : ses villes redeviennent des villages
    /// neutres, bâtiments et merveilles détruits, ses unités disparaissent.
    fn eliminer(&mut self, x: u8) {
        self.joueurs[x as usize].elimine = true;
        self.joueurs[x as usize].annonce = false;
        for i in 0..self.villes.len() {
            if self.villes[i].joueur == Some(x) {
                for k in 0..3 {
                    if self.merveilles[k] == i as u8 {
                        self.merveilles[k] = DETRUITE;
                    }
                }
                let v = &mut self.villes[i];
                v.joueur = None;
                v.niveau = 1;
                v.bonus_revenu = 0;
                v.bonus_entretien = 0;
                v.bonus_points = 0;
                v.batiments = 0;
            }
        }
        self.unites.retain(|u| u.joueur != x);
        let vivants: Vec<u8> = (0..self.reglages.nb_joueurs).filter(|&j| self.vivant(j)).collect();
        if vivants.len() == 1 {
            self.finie = true;
            self.gagnant = Some(vivants[0]);
        }
    }

    /// Début du tour de `k` (§ 5.1, § 6.1, § 9.2) : victoire au seuil, revenu, soins.
    fn debut_tour(&mut self, k: u8) {
        let seuil = self.reglages.seuil();
        if self.joueurs[k as usize].annonce {
            if self.score(k) >= seuil {
                self.finie = true;
                self.gagnant = Some(k);
                return;
            }
            self.joueurs[k as usize].annonce = false;
        }
        if self.tour > 1 {
            self.joueurs[k as usize].vivres += self.revenu(k);
        }
        for i in 0..self.unites.len() {
            if self.unites[i].joueur != k {
                continue;
            }
            let u = self.unites[i];
            if !u.a_bouge && !u.a_attaque {
                let en_ville = self.ville_sur(u.case).is_some_and(|v| self.villes[v].joueur == Some(k));
                let max = self.pv_max(&u);
                self.unites[i].pv = (u.pv + if en_ville { 4 } else { 2 }).min(max);
            }
            let u = &mut self.unites[i];
            u.points_restants = u.type_unite.mouvement();
            u.a_bouge = false;
            u.a_attaque = false;
            u.peut_agir = true;
        }
        for v in self.villes.iter_mut().filter(|v| v.joueur == Some(k)) {
            v.a_produit = false;
        }
        self.recalculer_vision(k);
    }

    /// Gagnant de la partie : `Some(joueur)`, ou `None` pour une partie nulle ou en cours.
    pub fn gagnant(&self) -> Option<u8> {
        self.gagnant
    }

    /// Partie terminée sans vainqueur (tour 80 passé, § 9.4).
    pub fn est_nulle(&self) -> bool {
        self.finie && self.gagnant.is_none()
    }

    // ---------- Actions permises ----------

    /// Actions permises, dans un ordre fixe (E4). Vide si la partie est finie.
    pub fn actions_permises(&self) -> Vec<Action> {
        let mut s = Vec::new();
        if self.finie {
            return s;
        }
        let j = self.joueur_courant;
        let jo = &self.joueurs[j as usize];
        s.push(Action::FinDuTour);
        let mut miennes: Vec<usize> = (0..self.unites.len()).filter(|&i| self.unites[i].joueur == j).collect();
        miennes.sort_unstable_by_key(|&i| self.unites[i].case);
        for &i in &miennes {
            let depart = self.unites[i].case;
            for (arrivee, _) in self.atteignables(depart) {
                s.push(Action::Deplacer { depart, arrivee });
            }
        }
        for &i in &miennes {
            let depart = self.unites[i].case;
            for cible in self.cibles(depart) {
                s.push(Action::Attaquer { depart, cible });
            }
        }
        for &i in &miennes {
            let u = self.unites[i];
            if u.type_unite == TypeUnite::Astronome && u.peut_agir && !u.a_attaque {
                for cible in self.plateau.carre(u.case, 4) {
                    s.push(Action::Lunette { depart: u.case, cible });
                }
            }
        }
        for &i in &miennes {
            if self.ralliement_permis(j, i) {
                s.push(Action::Rallier { case: self.unites[i].case });
            }
        }
        for &i in &miennes {
            s.push(Action::Renvoyer { case: self.unites[i].case });
        }
        let mut mes_villes: Vec<usize> = (0..self.villes.len()).filter(|&i| self.villes[i].joueur == Some(j)).collect();
        mes_villes.sort_unstable_by_key(|&i| self.villes[i].case);
        for &i in &mes_villes {
            for t in TypeUnite::TOUS {
                if self.production_permise(j, i, t) {
                    s.push(Action::Produire { ville: self.villes[i].case, type_unite: t });
                }
            }
        }
        for &i in &mes_villes {
            for b in [false, true] {
                if self.croissance_permise(j, i, b) {
                    s.push(Action::Agrandir { ville: self.villes[i].case, recompense_b: b });
                }
            }
        }
        for c in 0..self.plateau.nb_cases() as Case {
            if self.exploitation_permise(j, c) {
                s.push(Action::Exploiter { case: c });
            }
        }
        for &i in &mes_villes {
            for b in Batiment::TOUS {
                if self.batiment_permis(j, i, b) {
                    s.push(Action::Batir { ville: self.villes[i].case, batiment: b });
                }
            }
        }
        for a in AVANCEES {
            if self.avancee_ouverte(j, a) && jo.vivres >= self.prix_avancee(j, a) {
                s.push(Action::Rechercher { avancee: a });
            }
        }
        s
    }

    /// Rallier ou prendre (§ 4.2) : sur la case depuis le début du tour, rien fait encore.
    fn ralliement_permis(&self, j: u8, i: usize) -> bool {
        let u = self.unites[i];
        u.joueur == j
            && u.peut_agir
            && !u.a_bouge
            && !u.a_attaque
            && self.ville_sur(u.case).is_some_and(|v| self.villes[v].joueur != Some(j))
    }

    fn production_permise(&self, j: u8, i: usize, t: TypeUnite) -> bool {
        let v = &self.villes[i];
        v.joueur == Some(j)
            && !v.a_produit
            && self.unite_sur(v.case).is_none()
            && self.nb_unites(j) < self.entretien(j)
            && self.unite_debloquee(j, t)
            && self.joueurs[j as usize].vivres >= t.fiche().cout
    }

    fn croissance_permise(&self, j: u8, i: usize, recompense_b: bool) -> bool {
        let v = &self.villes[i];
        v.joueur == Some(j)
            && v.niveau < 5
            && self.joueurs[j as usize].vivres >= self.prix_croissance(i)
            && !(recompense_b && v.niveau + 1 == 3 && v.a(Batiment::Remparts))
    }

    fn exploitation_permise(&self, j: u8, c: Case) -> bool {
        let t = self.territoire[c as usize];
        if t == AUCUNE || self.villes[t as usize].joueur != Some(j) || self.exploitations[c as usize] != AUCUNE {
            return false;
        }
        if self.ville_sur(c).is_some() || self.unite_sur(c).is_some_and(|i| self.unites[i].joueur != j) {
            return false;
        }
        let Some(e) = Exploitation::pour_terrain(self.plateau.terrain(c)) else { return false };
        e.requiert().is_none_or(|a| self.possede(j, a)) && self.joueurs[j as usize].vivres >= e.cout()
    }

    fn batiment_permis(&self, j: u8, i: usize, b: Batiment) -> bool {
        let v = &self.villes[i];
        if v.joueur != Some(j) || v.a(b) || self.joueurs[j as usize].vivres < self.prix_batiment(j, b) {
            return false;
        }
        let tech = self.possede(j, b.requiert()) || (b == Batiment::Remparts && self.joueurs[j as usize].clan == Clan::Murailliers);
        match b.merveille() {
            Some(k) => tech && v.niveau >= 3 && self.merveilles[k] == AUCUNE,
            None => tech,
        }
    }

    /// Masque des actions permises dans l'espace fixe de `NB_ACTIONS` numéros.
    pub fn masque(&self) -> Vec<bool> {
        let mut m = vec![false; NB_ACTIONS as usize];
        for a in self.actions_permises() {
            m[a.numero(self.reglages.taille) as usize] = true;
        }
        m
    }

    // ---------- Application ----------

    /// Applique une action ; refuse (sans rien changer) une action non permise.
    pub fn appliquer(&mut self, action: Action) -> Result<(), ActionRefusee> {
        if self.finie {
            return Err(ActionRefusee::PartieFinie);
        }
        let j = self.joueur_courant;
        match action {
            Action::FinDuTour => {
                self.fin_du_tour();
                Ok(())
            }
            Action::Deplacer { depart, arrivee } => self.deplacer(depart, arrivee),
            Action::Attaquer { depart, cible } => {
                if self.unite_sur(depart).is_some_and(|i| self.unites[i].joueur != j) {
                    return Err(ActionRefusee::PasLeTourDeCetteUnite);
                }
                let an = self.annonce_combat(depart, cible).ok_or(ActionRefusee::NonPermise)?;
                let ia = self.unite_sur(depart).unwrap();
                let id = self.unite_sur(cible).unwrap();
                let victime = self.unites[id].joueur;
                self.unites[id].pv -= an.degats;
                self.unites[ia].pv -= an.riposte;
                let a = &mut self.unites[ia];
                a.a_attaque = true;
                if a.type_unite != TypeUnite::CoureurDeCrete {
                    a.points_restants = 0;
                }
                if an.cible_detruite {
                    self.compter_destruction(j);
                }
                if an.attaquant_detruit {
                    self.compter_destruction(victime);
                }
                self.unites.retain(|u| u.pv > 0);
                self.recalculer_toutes_les_visions();
                Ok(())
            }
            Action::Lunette { depart, cible } => {
                let i = self.unite_sur(depart).ok_or(ActionRefusee::PasDUnite)?;
                let u = self.unites[i];
                if u.joueur != j || u.type_unite != TypeUnite::Astronome || !u.peut_agir || u.a_attaque || self.plateau.distance(depart, cible) > 4 {
                    return Err(ActionRefusee::NonPermise);
                }
                self.unites[i].a_attaque = true;
                self.lunettes.push(cible);
                self.recalculer_vision(j);
                Ok(())
            }
            Action::Rallier { case } => {
                let iu = self.unite_sur(case).ok_or(ActionRefusee::PasDUnite)?;
                if !self.ralliement_permis(j, iu) {
                    return Err(ActionRefusee::NonPermise);
                }
                let iv = self.ville_sur(case).unwrap();
                let u = &mut self.unites[iu];
                u.points_restants = 0;
                u.a_attaque = true;
                let ancien = self.villes[iv].joueur;
                let etait_capitale = self.villes[iv].capitale();
                let v = &mut self.villes[iv];
                v.joueur = Some(j);
                v.bonus_revenu = 0;
                v.bonus_entretien = 0;
                v.bonus_points = 0;
                v.a_produit = true;
                if ancien.is_none() {
                    v.niveau = 1;
                    self.etendre_territoire(iv);
                }
                if let Some(x) = ancien {
                    if etait_capitale || !self.villes.iter().any(|v| v.joueur == Some(x)) {
                        self.eliminer(x);
                    }
                }
                self.recalculer_toutes_les_visions();
                Ok(())
            }
            Action::Renvoyer { case } => {
                let i = self.unite_sur(case).ok_or(ActionRefusee::PasDUnite)?;
                if self.unites[i].joueur != j {
                    return Err(ActionRefusee::PasLeTourDeCetteUnite);
                }
                self.unites.remove(i);
                self.recalculer_vision(j);
                Ok(())
            }
            Action::Produire { ville, type_unite } => {
                let iv = self.ville_sur(ville).ok_or(ActionRefusee::NonPermise)?;
                if !self.production_permise(j, iv, type_unite) {
                    return Err(ActionRefusee::NonPermise);
                }
                self.joueurs[j as usize].vivres -= type_unite.fiche().cout;
                self.villes[iv].a_produit = true;
                self.creer_unite(j, type_unite, ville, false);
                self.recalculer_vision(j);
                Ok(())
            }
            Action::Agrandir { ville, recompense_b } => {
                let iv = self.ville_sur(ville).ok_or(ActionRefusee::NonPermise)?;
                if !self.croissance_permise(j, iv, recompense_b) {
                    return Err(ActionRefusee::NonPermise);
                }
                self.joueurs[j as usize].vivres -= self.prix_croissance(iv);
                self.villes[iv].niveau += 1;
                let niveau = self.villes[iv].niveau;
                match (niveau, recompense_b) {
                    (2, false) => self.villes[iv].bonus_revenu += 1,
                    (2, true) => self.donner_unite(j, TypeUnite::Lancier, ville),
                    (3, false) => {
                        self.villes[iv].rayon = 2;
                        self.etendre_territoire(iv);
                    }
                    (3, true) => self.villes[iv].batiments |= Batiment::Remparts.bit(),
                    (4, false) => self.villes[iv].bonus_revenu += 2,
                    (4, true) => self.villes[iv].bonus_entretien += 2,
                    (5, false) => self.villes[iv].bonus_points += 2,
                    _ => self.donner_unite(j, TypeUnite::Cavalier, ville),
                }
                self.recalculer_vision(j);
                Ok(())
            }
            Action::Exploiter { case } => {
                if !self.exploitation_permise(j, case) {
                    return Err(ActionRefusee::NonPermise);
                }
                let e = Exploitation::pour_terrain(self.plateau.terrain(case)).unwrap();
                self.joueurs[j as usize].vivres -= e.cout();
                self.exploitations[case as usize] = e as u8;
                Ok(())
            }
            Action::Batir { ville, batiment } => {
                let iv = self.ville_sur(ville).ok_or(ActionRefusee::NonPermise)?;
                if !self.batiment_permis(j, iv, batiment) {
                    return Err(ActionRefusee::NonPermise);
                }
                self.joueurs[j as usize].vivres -= self.prix_batiment(j, batiment);
                self.villes[iv].batiments |= batiment.bit();
                if let Some(k) = batiment.merveille() {
                    self.merveilles[k] = iv as u8;
                }
                if batiment == Batiment::TourBrumes {
                    let t = self.plateau.taille();
                    self.reveler(j, 0, 2 * t);
                }
                self.recalculer_vision(j);
                Ok(())
            }
            Action::Rechercher { avancee } => {
                let prix = self.prix_avancee(j, avancee);
                if !self.avancee_ouverte(j, avancee) || self.joueurs[j as usize].vivres < prix {
                    return Err(ActionRefusee::NonPermise);
                }
                let jo = &mut self.joueurs[j as usize];
                jo.vivres -= prix;
                jo.avancees |= avancee.bit();
                jo.points_acquis += 1;
                self.recalculer_vision(j);
                Ok(())
            }
        }
    }

    fn deplacer(&mut self, depart: Case, arrivee: Case) -> Result<(), ActionRefusee> {
        let i = self.unite_sur(depart).ok_or(ActionRefusee::PasDUnite)?;
        let j = self.joueur_courant;
        if self.unites[i].joueur != j {
            return Err(ActionRefusee::PasLeTourDeCetteUnite);
        }
        if self.unites[i].points_restants == 0 || !self.unites[i].peut_agir {
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
        // Case par case (§ 6.1, surprise) : si une adverse apparaît, l'unité s'arrête sur la
        // dernière case atteinte et perd le reste de son mouvement. Une ruine l'arrête aussi.
        let avant = self.adverses_visibles(j);
        let mut arretee = false;
        for c in chemin {
            debug_assert!(self.unite_sur(c).is_none(), "le chemin passe par une case connue et libre");
            self.unites[i].case = c;
            self.recalculer_vision(j);
            if self.ruines[c as usize] != AUCUNE {
                self.fouiller(i, c);
                arretee = true;
                break;
            }
            if self.adverses_visibles(j).iter().any(|k| !avant.contains(k)) {
                arretee = true;
                break;
            }
        }
        let u = &mut self.unites[i];
        u.points_restants = if arretee { 0 } else { reste };
        u.a_bouge = true;
        Ok(())
    }

    /// Fouille de la ruine de la case `c` par l'unité `i` (§ 3.3).
    fn fouiller(&mut self, i: usize, c: Case) {
        let j = self.unites[i].joueur;
        let tresor = self.ruines[c as usize];
        self.ruines[c as usize] = AUCUNE;
        self.joueurs[j as usize].points_acquis += 1;
        let reserve = |p: &mut Partie| {
            let gain = if p.joueurs[j as usize].clan == Clan::VentsErrants { 12 } else { 6 };
            p.joueurs[j as usize].vivres += gain;
        };
        match tresor {
            0 => reserve(self),
            1 => {
                let ouvertes: Vec<Avancee> = AVANCEES.iter().copied().filter(|&a| self.avancee_ouverte(j, a)).collect();
                match ouvertes.iter().min_by_key(|&&a| (self.prix_avancee(j, a), a as u8)) {
                    Some(&a) => {
                        self.joueurs[j as usize].avancees |= a.bit();
                        self.joueurs[j as usize].points_acquis += 1;
                    }
                    None => reserve(self),
                }
            }
            2 => self.donner_unite(j, TypeUnite::Lancier, c),
            _ => self.reveler(j, c, 4),
        }
        self.recalculer_vision(j);
    }

    /// Fin du tour du joueur courant (§ 9.2, § 9.4) : annonce au seuil, passage au joueur
    /// vivant suivant, fin de sécurité après le dernier tour.
    fn fin_du_tour(&mut self) {
        let j = self.joueur_courant;
        let seuil = self.reglages.seuil();
        self.joueurs[j as usize].annonce = self.score(j) >= seuil;
        self.lunettes.clear();
        self.recalculer_vision(j);
        let n = self.reglages.nb_joueurs;
        let mut k = j;
        loop {
            k = (k + 1) % n;
            if k == 0 {
                if self.tour >= self.reglages.limite_tours {
                    self.finie = true;
                    self.gagnant = None;
                    return;
                }
                self.tour += 1;
            }
            if self.vivant(k) {
                break;
            }
        }
        self.joueur_courant = k;
        self.debut_tour(k);
    }

    /// Applique l'action de numéro `n` (forme utilisée par l'entraînement et les replays).
    pub fn appliquer_numero(&mut self, n: u32) -> Result<(), ActionRefusee> {
        let a = Action::depuis_numero(n, self.reglages.taille).ok_or(ActionRefusee::NumeroInconnu)?;
        self.appliquer(a)
    }

    /// Ce que voit `joueur` (§ 3, besoin de S3) : rien de ce qu'il ne sait pas.
    pub fn vue(&self, joueur: u8) -> Vue {
        let b = &self.brouillards[joueur as usize];
        let n = self.plateau.nb_cases();
        let connue = |c: usize| b.cases[c] != Visibilite::Inconnue;
        let terrains = (0..n).map(|c| connue(c).then(|| self.plateau.terrain(c as Case))).collect();
        let unites = self.unites.iter().filter(|u| u.joueur == joueur || b.cases[u.case as usize] == Visibilite::Visible).copied().collect();
        let villes = self
            .villes
            .iter()
            .enumerate()
            .filter(|(i, _)| b.villes[*i] != AUCUNE)
            .map(|(i, v)| {
                let mut v = *v;
                v.joueur = (b.villes[i] != VILLE_NEUTRE).then_some(b.villes[i]);
                v
            })
            .collect();
        Vue {
            joueur,
            taille: self.reglages.taille,
            nb_joueurs: self.reglages.nb_joueurs,
            tour: self.tour,
            joueur_courant: self.joueur_courant,
            finie: self.finie,
            gagnant: self.gagnant,
            limite_tours: self.reglages.limite_tours,
            seuil: self.reglages.seuil(),
            cases: b.cases.clone(),
            terrains,
            unites,
            villes,
            ruines: (0..n).filter(|&c| connue(c) && self.ruines[c] != AUCUNE).map(|c| c as Case).collect(),
            exploitations: (0..n).filter(|&c| connue(c)).filter_map(|c| self.exploitation(c as Case).map(|e| (c as Case, e))).collect(),
            vivres: self.joueurs[joueur as usize].vivres,
            revenu: self.revenu(joueur),
            scores: self.scores(),
            joueurs: self.joueurs.iter().map(|jo| Joueur { vivres: 0, ..*jo }).collect(),
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
            e.ajouter_u32(u.pv as u32);
            e.ajouter_octet(u.points_restants);
            e.ajouter_octet(u.a_bouge as u8 | (u.a_attaque as u8) << 1 | (u.peut_agir as u8) << 2);
        }
        for v in &self.villes {
            e.ajouter_u32(v.case as u32);
            e.ajouter_octet(v.joueur.unwrap_or(VILLE_NEUTRE));
            e.ajouter_octet(v.capitale_de.unwrap_or(VILLE_NEUTRE));
            e.ajouter_octet(v.niveau);
            e.ajouter_u32(v.bonus_revenu as u32);
            e.ajouter_u32(v.bonus_entretien as u32);
            e.ajouter_u32(v.bonus_points as u32);
            e.ajouter_octet(v.rayon);
            e.ajouter_octet(v.batiments);
            e.ajouter_octet(v.a_produit as u8);
        }
        for jo in &self.joueurs {
            e.ajouter_octet(jo.clan as u8);
            e.ajouter_u32(jo.vivres as u32);
            e.ajouter_u32(jo.avancees as u32);
            e.ajouter_u32(jo.points_acquis as u32);
            e.ajouter_octet(jo.elimine as u8 | (jo.annonce as u8) << 1);
        }
        for b in &self.brouillards {
            for &c in &b.cases {
                e.ajouter_octet(c as u8);
            }
            for &v in &b.villes {
                e.ajouter_octet(v);
            }
        }
        for t in [&self.territoire, &self.exploitations, &self.ruines] {
            for &x in t.iter() {
                e.ajouter_octet(x);
            }
        }
        for &m in &self.merveilles {
            e.ajouter_octet(m);
        }
        for &c in &self.lunettes {
            e.ajouter_u32(c as u32);
        }
        e.ajouter_octet(self.joueur_courant);
        e.ajouter_u32(self.tour as u32);
        e.ajouter_octet(self.finie as u8);
        e.ajouter_octet(self.gagnant.unwrap_or(AUCUNE));
        e.valeur()
    }

    /// Texte lisible d'une action (débogage, replays ; jamais à l'écran) : son numéro,
    /// précédé des adresses internes pour les actions sur deux cases.
    pub fn action_vers_texte(&self, a: Action) -> String {
        let ad = |c: Case| self.plateau.adresse(c);
        match a {
            Action::FinDuTour => "FIN".to_string(),
            Action::Deplacer { depart, arrivee } => format!("{}>{}", ad(depart), ad(arrivee)),
            Action::Attaquer { depart, cible } => format!("{}*{}", ad(depart), ad(cible)),
            _ => format!("#{}", a.numero(self.reglages.taille)),
        }
    }

    pub fn texte_vers_action(&self, texte: &str) -> Option<Action> {
        let t = texte.trim();
        let ad = |s: &str| self.plateau.depuis_adresse(s.trim());
        if t.eq_ignore_ascii_case("FIN") {
            return Some(Action::FinDuTour);
        }
        if let Some(n) = t.strip_prefix('#') {
            return Action::depuis_numero(n.parse().ok()?, self.reglages.taille);
        }
        if let Some((a, b)) = t.split_once('>') {
            return Some(Action::Deplacer { depart: ad(a)?, arrivee: ad(b)? });
        }
        let (a, b) = t.split_once('*')?;
        Some(Action::Attaquer { depart: ad(a)?, cible: ad(b)? })
    }

    /// Hasard de la partie. La V1 n'en tire plus après la création de la carte (§ 0).
    pub fn hasard_mut(&mut self) -> &mut Hasard {
        &mut self.hasard
    }
}

/// Vue d'un joueur : ce qu'il sait, rien de plus (§ 3.1). Les vivres des autres sont privés.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vue {
    pub joueur: u8,
    pub taille: u8,
    pub nb_joueurs: u8,
    pub tour: u16,
    pub joueur_courant: u8,
    pub finie: bool,
    pub gagnant: Option<u8>,
    pub limite_tours: u16,
    pub seuil: i32,
    /// État de chaque case pour ce joueur.
    pub cases: Vec<Visibilite>,
    /// Terrain des cases connues ; `None` pour une case inconnue.
    pub terrains: Vec<Option<Terrain>>,
    /// Ses unités et les unités adverses visibles.
    pub unites: Vec<Unite>,
    /// Villes et villages connus, avec le dernier propriétaire vu.
    pub villes: Vec<Ville>,
    /// Ruines sur des cases connues.
    pub ruines: Vec<Case>,
    /// Exploitations sur des cases connues.
    pub exploitations: Vec<(Case, Exploitation)>,
    pub vivres: i32,
    pub revenu: i32,
    /// Scores publics de tous les joueurs.
    pub scores: Vec<i32>,
    /// Informations publiques des joueurs (clan, avancées, annonce, élimination ; vivres à 0).
    pub joueurs: Vec<Joueur>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionRefusee {
    PartieFinie,
    PasDUnite,
    PasLeTourDeCetteUnite,
    HorsDePortee,
    NumeroInconnu,
    NonPermise,
}

/// Codage pour un réseau de neurones (E7), version du codage.
pub const VERSION_CODAGE: u32 = 3;
/// Canaux : 5 terrains (cases connues), inconnue, visible, mes unités, unités adverses
/// visibles, mes points restants, points de vie (fraction), ma ville, villes adverses,
/// villages neutres, ruines, exploitations, mon territoire, case hors carte.
pub const NB_CANAUX: usize = NB_TERRAINS + 13;
/// Vecteur : tour, mon tour, joueurs, limite, mes vivres, mon revenu, mon score,
/// meilleur score adverse, seuil.
pub const TAILLE_VECTEUR: usize = 9;

/// Codage de la vue de `joueur` : plans `NB_CANAUX × 16 × 16` en octets, plus un petit
/// vecteur. Rien de caché n'y entre. Le plateau est retourné (gauche-droite, haut-bas)
/// pour que la capitale du joueur soit toujours dans le quart en bas à gauche.
pub fn coder(partie: &Partie, joueur: u8) -> (Vec<u8>, [i32; TAILLE_VECTEUR]) {
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
    let k = NB_TERRAINS;
    let (c_inconnue, c_visible, c_miennes, c_adverses, c_points, c_pv, c_ma_ville, c_villes, c_villages, c_ruines, c_expl, c_terr, c_hors) =
        (k, k + 1, k + 2, k + 3, k + 4, k + 5, k + 6, k + 7, k + 8, k + 9, k + 10, k + 11, k + 12);
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
        if partie.territoire(c).is_some_and(|v| v.joueur == Some(joueur)) {
            plans[idx(c_terr, x, y)] = 255;
        }
    }
    for u in &vue.unites {
        let (x, y) = orienter(u.case);
        plans[idx(if u.joueur == joueur { c_miennes } else { c_adverses }, x, y)] = 255;
        if u.joueur == joueur {
            plans[idx(c_points, x, y)] = (u.points_restants as u32 * 255 / u.type_unite.mouvement() as u32) as u8;
        }
        plans[idx(c_pv, x, y)] = (u.pv.clamp(0, 14) as u32 * 255 / 14) as u8;
    }
    for v in &vue.villes {
        let (x, y) = orienter(v.case);
        let canal = match v.joueur {
            Some(j) if j == joueur => c_ma_ville,
            Some(_) => c_villes,
            None => c_villages,
        };
        plans[idx(canal, x, y)] = 255;
    }
    for &c in &vue.ruines {
        let (x, y) = orienter(c);
        plans[idx(c_ruines, x, y)] = 255;
    }
    for &(c, _) in &vue.exploitations {
        let (x, y) = orienter(c);
        plans[idx(c_expl, x, y)] = 255;
    }
    let meilleur_adverse = (0..vue.nb_joueurs).filter(|&j| j != joueur).map(|j| vue.scores[j as usize]).max().unwrap_or(0);
    let vecteur = [
        vue.tour as i32,
        (vue.joueur_courant == joueur) as i32,
        vue.nb_joueurs as i32,
        vue.limite_tours as i32,
        vue.vivres,
        vue.revenu,
        vue.scores[joueur as usize],
        meilleur_adverse,
        vue.seuil,
    ];
    (plans, vecteur)
}

#[cfg(test)]
mod tests;
