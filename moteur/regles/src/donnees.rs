//! Tables de données des règles V1 (`regles-v1.md`, § 12). Valeurs de départ, à régler
//! par parties simulées. Identifiants en anglais dans le document, noms français ici (D-06 :
//! le code suit la convention du projet ; les noms affichés vivent dans l'application).

use crate::plateau::Terrain;

/// Classe d'unité pour le triangle (§ 6.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Classe {
    Aucune,
    Lance,
    Arc,
    Cheval,
    Siege,
}

impl Classe {
    /// +2 si `self` a l'avantage sur `autre` : lance > cheval > arc > lance.
    pub fn bonus_contre(self, autre: Classe) -> i32 {
        match (self, autre) {
            (Classe::Lance, Classe::Cheval) | (Classe::Cheval, Classe::Arc) | (Classe::Arc, Classe::Lance) => 2,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TypeUnite {
    Eclaireur = 0,
    Lancier = 1,
    Archer = 2,
    Cavalier = 3,
    Catapulte = 4,
    CoureurDeCrete = 5,
    PortePavois = 6,
    Astronome = 7,
    GuetteurPlaneur = 8,
}

pub const NB_TYPES_UNITE: usize = 9;

pub struct FicheUnite {
    pub cout: i32,
    pub pv: i32,
    pub attaque: i32,
    pub defense: i32,
    pub mouvement: u8,
    pub portee: u8,
    pub portee_min: u8,
    pub vision: u8,
    pub classe: Classe,
}

const fn fiche(cout: i32, pv: i32, attaque: i32, defense: i32, mouvement: u8, portee: u8, portee_min: u8, vision: u8, classe: Classe) -> FicheUnite {
    FicheUnite { cout, pv, attaque, defense, mouvement, portee, portee_min, vision, classe }
}

/// § 12.3, dans l'ordre de [`TypeUnite`].
const FICHES: [FicheUnite; NB_TYPES_UNITE] = [
    fiche(3, 6, 1, 1, 3, 1, 1, 3, Classe::Aucune),
    fiche(4, 10, 3, 3, 1, 1, 1, 2, Classe::Lance),
    fiche(5, 8, 3, 1, 1, 2, 1, 2, Classe::Arc),
    fiche(6, 10, 4, 2, 2, 1, 1, 2, Classe::Cheval),
    fiche(8, 6, 5, 0, 1, 3, 2, 1, Classe::Siege),
    fiche(4, 8, 3, 1, 2, 1, 1, 2, Classe::Aucune),
    fiche(5, 12, 1, 4, 1, 1, 1, 2, Classe::Aucune),
    fiche(4, 6, 0, 1, 2, 0, 0, 3, Classe::Aucune),
    fiche(5, 6, 2, 1, 3, 1, 1, 3, Classe::Aucune),
];

impl TypeUnite {
    pub const TOUS: [TypeUnite; NB_TYPES_UNITE] = [
        TypeUnite::Eclaireur,
        TypeUnite::Lancier,
        TypeUnite::Archer,
        TypeUnite::Cavalier,
        TypeUnite::Catapulte,
        TypeUnite::CoureurDeCrete,
        TypeUnite::PortePavois,
        TypeUnite::Astronome,
        TypeUnite::GuetteurPlaneur,
    ];
    pub fn depuis_u8(v: u8) -> Option<TypeUnite> {
        TypeUnite::TOUS.get(v as usize).copied()
    }
    pub fn fiche(self) -> &'static FicheUnite {
        &FICHES[self as usize]
    }
    pub fn mouvement(self) -> u8 {
        self.fiche().mouvement
    }
    pub fn vision(self) -> u8 {
        self.fiche().vision
    }
    /// Ne peut pas attaquer et ne riposte jamais (`no_attack`).
    pub fn sans_attaque(self) -> bool {
        self == TypeUnite::Astronome
    }
}

/// Défense donnée par le terrain à qui s'y trouve (§ 1.3).
pub fn defense_terrain(t: Terrain) -> i32 {
    match t {
        Terrain::Plaine | Terrain::Eau => 0,
        Terrain::Foret => 1,
        Terrain::Colline | Terrain::Montagne => 2,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Clan {
    CrocsRouges = 0,
    Murailliers = 1,
    VeilleursAstres = 2,
    VentsErrants = 3,
}

impl Clan {
    pub const TOUS: [Clan; 4] = [Clan::CrocsRouges, Clan::Murailliers, Clan::VeilleursAstres, Clan::VentsErrants];
    pub fn depuis_u8(v: u8) -> Clan {
        Clan::TOUS[v as usize % 4]
    }
    pub fn unite_propre(self) -> TypeUnite {
        match self {
            Clan::CrocsRouges => TypeUnite::CoureurDeCrete,
            Clan::Murailliers => TypeUnite::PortePavois,
            Clan::VeilleursAstres => TypeUnite::Astronome,
            Clan::VentsErrants => TypeUnite::GuetteurPlaneur,
        }
    }
    /// Unité d'éclaireur au départ (§ 7).
    pub fn eclaireur(self) -> TypeUnite {
        match self {
            Clan::CrocsRouges => TypeUnite::CoureurDeCrete,
            Clan::VentsErrants => TypeUnite::GuetteurPlaneur,
            _ => TypeUnite::Eclaireur,
        }
    }
}

/// Les 15 avancées de l'arbre, dans l'ordre de référence (§ 12.6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Avancee {
    Arcs = 0,
    Dressage = 1,
    Remparts = 2,
    Forge = 3,
    Arene = 4,
    Champs = 5,
    Carrieres = 6,
    Marches = 7,
    Greniers = 8,
    Verger = 9,
    Sentiers = 10,
    Mecanique = 11,
    Cartographes = 12,
    Ecoles = 13,
    TourBrumes = 14,
}

pub const NB_AVANCEES: usize = 15;

impl Avancee {
    pub fn depuis_u8(v: u8) -> Option<Avancee> {
        if (v as usize) < NB_AVANCEES {
            // SAFETY de forme : les valeurs 0..15 sont toutes des variantes (repr(u8) contigu).
            Some(AVANCEES[v as usize])
        } else {
            None
        }
    }
    pub fn bit(self) -> u16 {
        1 << self as u16
    }
    /// Branche (0 fer, 1 terre, 2 souffle) et rang (1 à 4).
    pub fn branche_rang(self) -> (u8, u8) {
        let i = self as u8;
        let b = i / 5;
        let r = match i % 5 {
            0 => 1,
            1 => 2,
            2 | 3 => 3,
            _ => 4,
        };
        (b, r)
    }
    pub fn prix_de_base(self) -> i32 {
        [5, 8, 11, 14][self.branche_rang().1 as usize - 1]
    }
    /// Avancées dont il faut au moins une (vide au rang 1).
    pub fn prerequis(self) -> &'static [Avancee] {
        use Avancee::*;
        match self {
            Arcs | Champs | Sentiers => &[],
            Dressage => &[Arcs],
            Remparts | Forge => &[Dressage],
            Arene => &[Remparts, Forge],
            Carrieres => &[Champs],
            Marches | Greniers => &[Carrieres],
            Verger => &[Marches, Greniers],
            Mecanique => &[Sentiers],
            Cartographes | Ecoles => &[Mecanique],
            TourBrumes => &[Cartographes, Ecoles],
        }
    }
    /// Choix exclusif du rang 3 (§ 8.1).
    pub fn exclue_par(self) -> Option<Avancee> {
        use Avancee::*;
        match self {
            Remparts => Some(Forge),
            Forge => Some(Remparts),
            Marches => Some(Greniers),
            Greniers => Some(Marches),
            Cartographes => Some(Ecoles),
            Ecoles => Some(Cartographes),
            _ => None,
        }
    }
}

pub const AVANCEES: [Avancee; NB_AVANCEES] = {
    use Avancee::*;
    [Arcs, Dressage, Remparts, Forge, Arene, Champs, Carrieres, Marches, Greniers, Verger, Sentiers, Mecanique, Cartographes, Ecoles, TourBrumes]
};

/// Exploitations (§ 5.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Exploitation {
    Cueillette = 0,
    Champ = 1,
    Carriere = 2,
}

impl Exploitation {
    pub fn pour_terrain(t: Terrain) -> Option<Exploitation> {
        match t {
            Terrain::Foret => Some(Exploitation::Cueillette),
            Terrain::Plaine => Some(Exploitation::Champ),
            Terrain::Colline => Some(Exploitation::Carriere),
            _ => None,
        }
    }
    pub fn cout(self) -> i32 {
        [2, 3, 5][self as usize]
    }
    pub fn revenu(self) -> i32 {
        [1, 1, 2][self as usize]
    }
    pub fn requiert(self) -> Option<Avancee> {
        match self {
            Exploitation::Cueillette => None,
            Exploitation::Champ => Some(Avancee::Champs),
            Exploitation::Carriere => Some(Avancee::Carrieres),
        }
    }
}

/// Bâtiments et merveilles (§ 5.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Batiment {
    Remparts = 0,
    Marche = 1,
    Arene = 2,
    Verger = 3,
    TourBrumes = 4,
}

pub const NB_BATIMENTS: usize = 5;

impl Batiment {
    pub const TOUS: [Batiment; NB_BATIMENTS] = [Batiment::Remparts, Batiment::Marche, Batiment::Arene, Batiment::Verger, Batiment::TourBrumes];
    pub fn bit(self) -> u8 {
        1 << self as u8
    }
    pub fn est_merveille(self) -> bool {
        (self as u8) >= 2
    }
    /// Indice de merveille 0..3 (arène, verger, tour).
    pub fn merveille(self) -> Option<usize> {
        self.est_merveille().then(|| self as usize - 2)
    }
    pub fn requiert(self) -> Avancee {
        match self {
            Batiment::Remparts => Avancee::Remparts,
            Batiment::Marche => Avancee::Marches,
            Batiment::Arene => Avancee::Arene,
            Batiment::Verger => Avancee::Verger,
            Batiment::TourBrumes => Avancee::TourBrumes,
        }
    }
}

/// Niveaux de ville (§ 12.4), indice = niveau - 1.
pub const COUT_NIVEAU: [i32; 5] = [0, 5, 8, 11, 14];
pub const REVENU_NIVEAU: [i32; 5] = [2, 3, 4, 5, 6];
pub const ENTRETIEN_NIVEAU: [i32; 5] = [2, 3, 4, 5, 6];
pub const POINTS_NIVEAU: [i32; 5] = [2, 4, 6, 8, 10];
pub const BONUS_REVENU_CAPITALE: i32 = 2;
pub const BONUS_ENTRETIEN_CAPITALE: i32 = 1;
pub const POINTS_MERVEILLE: i32 = 6;

/// Trésors des ruines (§ 3.3), avec leur poids sur 100.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tresor {
    Reserve = 0,
    Savoir = 1,
    Recrue = 2,
    Carte = 3,
}

impl Tresor {
    pub fn tirer(v: u32) -> Tresor {
        match v % 100 {
            0..=39 => Tresor::Reserve,
            40..=59 => Tresor::Savoir,
            60..=79 => Tresor::Recrue,
            _ => Tresor::Carte,
        }
    }
}

/// Réglages qui dépendent du nombre de joueurs (§ 12.1) : villages, ruines, seuil.
pub fn villages_ruines_seuil(nb_joueurs: u8) -> (usize, usize, i32) {
    match nb_joueurs {
        2 => (8, 4, 45),
        3 => (10, 5, 45),
        _ => (14, 7, 40),
    }
}
