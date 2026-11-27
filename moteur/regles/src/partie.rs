//! Une partie : réglages, état, actions permises, application d'une action.
//!
//! Interface demandée par la session 3 (`docs/ia/besoins-moteur.md`, § 3) :
//! nouvelle partie depuis une graine, joueur courant, masque des actions permises dans un
//! espace fixe, application, copie (`Clone`), fin de partie et scores, vue d'un joueur,
//! codage en plans, actions en texte, empreinte.

use crate::empreinte::Empreinte;
use crate::hasard::Hasard;
use crate::plateau::{Case, Plateau, Terrain, NB_TERRAINS, TAILLE_MAX};

pub const NB_JOUEURS: usize = 2;

/// Réglages d'une partie. Valeurs provisoires : le tri de la session 2 n'est pas fait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Reglages {
    /// Côté du plateau carré (6 à 16 ; proposition de S2 : 11, puis 14 au plus).
    pub taille: u8,
    pub unites_par_joueur: u8,
    pub points_mouvement: u8,
    /// Limite de tours : toute partie se termine (E2).
    pub limite_tours: u16,
}

impl Default for Reglages {
    fn default() -> Self {
        Reglages { taille: 11, unites_par_joueur: 3, points_mouvement: 3, limite_tours: 30 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Unite {
    pub joueur: u8,
    pub case: Case,
    pub points_restants: u8,
}

/// Une action, dans sa forme lisible. Son numéro fixe est donné par [`Action::numero`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    /// Le joueur courant passe la main.
    FinDuTour,
    /// Déplacer l'unité posée sur `depart` jusqu'à `arrivee`, en une action.
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Partie {
    reglages: Reglages,
    graine: u64,
    plateau: Plateau,
    unites: Vec<Unite>,
    joueur_courant: u8,
    tour: u16,
    hasard: Hasard,
    finie: bool,
}

impl Partie {
    /// Nouvelle partie : la carte est tirée de la graine, rien d'autre n'intervient (E3).
    pub fn nouvelle(reglages: Reglages, graine: u64) -> Partie {
        assert!(reglages.unites_par_joueur >= 1 && reglages.unites_par_joueur as usize <= reglages.taille as usize);
        assert!(reglages.points_mouvement >= 1);
        let mut hasard = Hasard::new(graine);
        let t = reglages.taille;
        let mut plateau = Plateau::uni(t, Terrain::Plaine);
        for c in 0..plateau.nb_cases() as Case {
            // 60 % plaine, 20 % forêt, 10 % montagne, 10 % eau.
            let terrain = match hasard.sous(10) {
                0..=5 => Terrain::Plaine,
                6 | 7 => Terrain::Foret,
                8 => Terrain::Montagne,
                _ => Terrain::Eau,
            };
            plateau.poser(c, terrain);
        }
        // Départs : rangée du bas pour le joueur 0, du haut pour le joueur 1, en plaine.
        let mut unites = Vec::with_capacity(2 * reglages.unites_par_joueur as usize);
        for joueur in 0..NB_JOUEURS as u8 {
            for i in 0..reglages.unites_par_joueur {
                let (x, y) = if joueur == 0 { (i, 0) } else { (t - 1 - i, t - 1) };
                let c = plateau.case(x, y);
                plateau.poser(c, Terrain::Plaine);
                unites.push(Unite { joueur, case: c, points_restants: reglages.points_mouvement });
            }
        }
        // Les plaines de départ restent reliées au reste de la carte : leurs voisines aussi.
        let departs: Vec<Case> = unites.iter().map(|u| u.case).collect();
        let mut v = [0; 4];
        for c in departs {
            let n = plateau.voisines(c, &mut v);
            for &voisine in &v[..n] {
                if plateau.terrain(voisine).cout().is_none() {
                    plateau.poser(voisine, Terrain::Plaine);
                }
            }
        }
        Partie { reglages, graine, plateau, unites, joueur_courant: 0, tour: 1, hasard, finie: false }
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
    pub fn joueur_courant(&self) -> u8 {
        self.joueur_courant
    }
    pub fn tour(&self) -> u16 {
        self.tour
    }
    pub fn est_finie(&self) -> bool {
        self.finie
    }

    fn unite_sur(&self, c: Case) -> Option<usize> {
        self.unites.iter().position(|u| u.case == c)
    }

    /// Cases atteignables par l'unité posée sur `depart`, avec le coût restant de chacune.
    /// Ordre de sortie déterministe (ordre croissant des cases).
    pub fn atteignables(&self, depart: Case) -> Vec<(Case, u8)> {
        let Some(i) = self.unite_sur(depart) else { return Vec::new() };
        let budget = self.unites[i].points_restants;
        let n = self.plateau.nb_cases();
        let mut cout = vec![u8::MAX; n];
        let mut occupe = vec![false; n];
        for u in &self.unites {
            occupe[u.case as usize] = true;
        }
        cout[depart as usize] = 0;
        // Coûts de 1 ou 2 et budget petit : une file par coût suffit (Dijkstra par seaux).
        let mut seaux: Vec<Vec<Case>> = vec![Vec::new(); budget as usize + 1];
        seaux[0].push(depart);
        let mut v = [0; 4];
        for d in 0..=budget as usize {
            let mut k = 0;
            while k < seaux[d].len() {
                let c = seaux[d][k];
                k += 1;
                if cout[c as usize] as usize != d {
                    continue;
                }
                let nv = self.plateau.voisines(c, &mut v);
                for &w in &v[..nv] {
                    if occupe[w as usize] {
                        continue;
                    }
                    let Some(pas) = self.plateau.terrain(w).cout() else { continue };
                    let nd = d + pas as usize;
                    if nd <= budget as usize && (nd as u8) < cout[w as usize] {
                        cout[w as usize] = nd as u8;
                        seaux[nd].push(w);
                    }
                }
            }
        }
        (0..n as Case)
            .filter(|&c| c != depart && cout[c as usize] != u8::MAX)
            .map(|c| (c, budget - cout[c as usize]))
            .collect()
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
                self.joueur_courant = (self.joueur_courant + 1) % NB_JOUEURS as u8;
                if self.joueur_courant == 0 {
                    if self.tour >= self.reglages.limite_tours {
                        self.finie = true;
                        return Ok(());
                    }
                    self.tour += 1;
                }
                let pm = self.reglages.points_mouvement;
                let j = self.joueur_courant;
                for u in self.unites.iter_mut().filter(|u| u.joueur == j) {
                    u.points_restants = pm;
                }
                Ok(())
            }
            Action::Deplacer { depart, arrivee } => {
                let i = self.unite_sur(depart).ok_or(ActionRefusee::PasDUnite)?;
                if self.unites[i].joueur != self.joueur_courant {
                    return Err(ActionRefusee::PasLeTourDeCetteUnite);
                }
                let reste = self
                    .atteignables(depart)
                    .into_iter()
                    .find(|&(c, _)| c == arrivee)
                    .map(|(_, r)| r)
                    .ok_or(ActionRefusee::HorsDePortee)?;
                let u = &mut self.unites[i];
                u.case = arrivee;
                u.points_restants = reste;
                Ok(())
            }
        }
    }

    /// Applique l'action de numéro `n` (forme utilisée par l'entraînement et les replays).
    pub fn appliquer_numero(&mut self, n: u32) -> Result<(), ActionRefusee> {
        let a = Action::depuis_numero(n, self.reglages.taille).ok_or(ActionRefusee::NumeroInconnu)?;
        self.appliquer(a)
    }

    /// Score provisoire par joueur (E8) : nombre de cases de la moitié adverse occupées.
    /// À remplacer quand la session 2 aura fixé les règles de victoire.
    pub fn scores(&self) -> [i32; NB_JOUEURS] {
        let t = self.reglages.taille;
        let mut s = [0; NB_JOUEURS];
        for u in &self.unites {
            let y = self.plateau.rangee(u.case);
            let avance = if u.joueur == 0 { y >= t / 2 + t % 2 } else { y < t / 2 };
            if avance {
                s[u.joueur as usize] += 1;
            }
        }
        s
    }

    /// Gagnant à la fin de la partie : `Some(joueur)`, ou `None` pour une égalité ou une partie en cours.
    pub fn gagnant(&self) -> Option<u8> {
        if !self.finie {
            return None;
        }
        let s = self.scores();
        match s[0].cmp(&s[1]) {
            std::cmp::Ordering::Greater => Some(0),
            std::cmp::Ordering::Less => Some(1),
            std::cmp::Ordering::Equal => None,
        }
    }

    /// Empreinte de 64 bits de tout l'état (E3).
    pub fn empreinte(&self) -> u64 {
        let mut e = Empreinte::new();
        e.ajouter_u32(self.reglages.taille as u32);
        for t in self.plateau.terrains_bruts() {
            e.ajouter_octet(t);
        }
        for u in &self.unites {
            e.ajouter_octet(u.joueur);
            e.ajouter_u32(u.case as u32);
            e.ajouter_octet(u.points_restants);
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

    /// Hasard de la partie (pour de futures règles : combats, événements).
    pub fn hasard_mut(&mut self) -> &mut Hasard {
        &mut self.hasard
    }
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
pub const VERSION_CODAGE: u32 = 1;
/// Canaux : 4 terrains, mes unités, unités adverses, mes points restants (0 à 1 en
/// fraction de `points_mouvement`, multipliée par 255), case hors carte.
pub const NB_CANAUX: usize = NB_TERRAINS + 4;
/// Vecteur : tour / limite, joueur courant = moi.
pub const TAILLE_VECTEUR: usize = 2;

/// Codage de l'état du point de vue de `joueur` (lui = joueur 0) : plans
/// `NB_CANAUX × 16 × 16` en octets (0 ou 255), plus un petit vecteur. Le plateau est
/// retourné de 180° pour le joueur 1, pour qu'il parte lui aussi du bas.
/// Sans brouillard pour l'instant : la vue d'un joueur est l'état complet (E6 à venir).
pub fn coder(partie: &Partie, joueur: u8) -> (Vec<u8>, [u16; TAILLE_VECTEUR]) {
    let tm = TAILLE_MAX as usize;
    let t = partie.reglages.taille as usize;
    let mut plans = vec![0u8; NB_CANAUX * tm * tm];
    let idx = |canal: usize, x: usize, y: usize| canal * tm * tm + y * tm + x;
    let orienter = |x: usize, y: usize| if joueur == 0 { (x, y) } else { (t - 1 - x, t - 1 - y) };
    for y in 0..tm {
        for x in 0..tm {
            if x >= t || y >= t {
                plans[idx(NB_CANAUX - 1, x, y)] = 255;
            }
        }
    }
    for c in 0..partie.plateau.nb_cases() as Case {
        let (x, y) = orienter(partie.plateau.colonne(c) as usize, partie.plateau.rangee(c) as usize);
        plans[idx(partie.plateau.terrain(c) as usize, x, y)] = 255;
    }
    let pm = partie.reglages.points_mouvement as u32;
    for u in &partie.unites {
        let (x, y) = orienter(partie.plateau.colonne(u.case) as usize, partie.plateau.rangee(u.case) as usize);
        let canal = if u.joueur == joueur { NB_TERRAINS } else { NB_TERRAINS + 1 };
        plans[idx(canal, x, y)] = 255;
        if u.joueur == joueur {
            plans[idx(NB_TERRAINS + 2, x, y)] = (u.points_restants as u32 * 255 / pm) as u8;
        }
    }
    let vecteur = [partie.tour, (partie.joueur_courant == joueur) as u16];
    (plans, vecteur)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partie_plaine() -> Partie {
        let mut p = Partie::nouvelle(Reglages::default(), 1);
        for c in 0..p.plateau.nb_cases() as Case {
            p.plateau.poser(c, Terrain::Plaine);
        }
        p
    }

    #[test]
    fn meme_graine_meme_carte() {
        let a = Partie::nouvelle(Reglages::default(), 42);
        let b = Partie::nouvelle(Reglages::default(), 42);
        assert_eq!(a, b);
        assert_ne!(a.empreinte(), Partie::nouvelle(Reglages::default(), 43).empreinte());
    }

    #[test]
    fn deplacement_en_plaine_jusqu_a_trois_cases() {
        let p = partie_plaine();
        let depart = p.plateau.case(0, 0);
        let atteintes = p.atteignables(depart);
        // Losange de rayon 3 dans le coin, moins les cases des deux autres unités alliées.
        assert!(atteintes.iter().all(|&(c, _)| {
            let (x, y) = (p.plateau.colonne(c) as i32, p.plateau.rangee(c) as i32);
            x + y <= 3
        }));
        assert!(atteintes.contains(&(p.plateau.case(0, 3), 0)));
        assert!(!atteintes.iter().any(|&(c, _)| c == p.plateau.case(1, 0)), "case occupée");
    }

    #[test]
    fn la_foret_coute_deux_et_l_eau_bloque() {
        let mut p = partie_plaine();
        let d = p.plateau.case(5, 5);
        p.unites[0].case = d;
        p.plateau.poser(p.plateau.case(6, 5), Terrain::Foret);
        p.plateau.poser(p.plateau.case(4, 5), Terrain::Eau);
        let a = p.atteignables(d);
        assert!(a.contains(&(p.plateau.case(6, 5), 1)), "forêt : 3 - 2 = 1 point restant");
        assert!(!a.iter().any(|&(c, _)| c == p.plateau.case(4, 5)), "eau infranchissable");
        assert!(a.contains(&(p.plateau.case(7, 5), 0)), "plaine derrière la forêt : 3 points");
        assert!(!a.iter().any(|&(c, _)| c == p.plateau.case(8, 5)));
    }

    #[test]
    fn tour_par_tour_et_fin_de_partie() {
        let r = Reglages { limite_tours: 2, ..Reglages::default() };
        let mut p = Partie::nouvelle(r, 3);
        assert_eq!((p.joueur_courant(), p.tour()), (0, 1));
        p.appliquer(Action::FinDuTour).unwrap();
        assert_eq!((p.joueur_courant(), p.tour()), (1, 1));
        p.appliquer(Action::FinDuTour).unwrap();
        assert_eq!((p.joueur_courant(), p.tour()), (0, 2));
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::FinDuTour).unwrap();
        assert!(p.est_finie());
        assert!(p.actions_permises().is_empty());
        assert_eq!(p.appliquer(Action::FinDuTour), Err(ActionRefusee::PartieFinie));
    }

    #[test]
    fn une_action_non_permise_ne_change_rien() {
        let mut p = partie_plaine();
        let avant = p.clone();
        let adverse = p.unites.iter().find(|u| u.joueur == 1).unwrap().case;
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
        let mut p = partie_plaine();
        p.appliquer(Action::Deplacer { depart: 0, arrivee: p.plateau.case(0, 2) }).unwrap();
        assert_eq!(p.unites[0].points_restants, 1);
        p.appliquer(Action::FinDuTour).unwrap();
        p.appliquer(Action::FinDuTour).unwrap();
        assert_eq!(p.unites[0].points_restants, 3);
    }

    #[test]
    fn numeros_d_action_aller_retour() {
        for taille in [6u8, 11, 14, 16] {
            let p = Partie::nouvelle(Reglages { taille, ..Reglages::default() }, 9);
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
    fn codage_du_point_de_vue_de_chaque_joueur() {
        let p = Partie::nouvelle(Reglages::default(), 8);
        let (plans0, v0) = coder(&p, 0);
        let (plans1, v1) = coder(&p, 1);
        assert_eq!(plans0.len(), NB_CANAUX * 256);
        let tm = TAILLE_MAX as usize;
        let mes = |plans: &Vec<u8>| (0..tm * tm).filter(|&i| plans[NB_TERRAINS * tm * tm + i] == 255).count();
        assert_eq!(mes(&plans0), 3);
        assert_eq!(mes(&plans1), 3);
        // Chacun voit ses unités en bas du plateau.
        assert_eq!(plans0[NB_TERRAINS * tm * tm], 255);
        assert_eq!(plans1[NB_TERRAINS * tm * tm], 255);
        assert_eq!((v0[1], v1[1]), (1, 0));
        // Hors carte : colonnes 11 à 15.
        assert_eq!(plans0[(NB_CANAUX - 1) * tm * tm + 11], 255);
    }
}
