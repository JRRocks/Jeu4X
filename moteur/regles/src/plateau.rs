//! Plateau carré de cases et terrains.
//!
//! Une case est un index `0..taille²` (rangée par rangée, rangée 0 en bas). Son adresse
//! « à la façon des échecs » (colonne en lettre, rangée en chiffre : A1, B2…) ne sert
//! qu'au moteur, aux replays et au débogage : elle n'est jamais affichée au joueur (D-16).

/// Taille maximale d'un côté : fixe la taille de l'espace d'actions et des plans (E4, E7).
pub const TAILLE_MAX: u8 = 16;
/// Taille minimale d'un côté.
pub const TAILLE_MIN: u8 = 6;

pub type Case = u16;

/// Les 8 directions, dans l'ordre de [`Plateau::voisines`].
const DIRECTIONS: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Terrain {
    Plaine = 0,
    Foret = 1,
    Colline = 2,
    Montagne = 3,
    Eau = 4,
}

pub const NB_TERRAINS: usize = 5;

impl Terrain {
    pub fn depuis_u8(v: u8) -> Terrain {
        match v {
            0 => Terrain::Plaine,
            1 => Terrain::Foret,
            2 => Terrain::Colline,
            3 => Terrain::Montagne,
            _ => Terrain::Eau,
        }
    }

    /// Coût d'entrée dans la case, en points de mouvement ; `None` = infranchissable
    /// (règles V1, § 1.3 et § 12.2).
    pub fn cout(self) -> Option<u8> {
        match self {
            Terrain::Plaine => Some(1),
            Terrain::Foret | Terrain::Colline => Some(2),
            Terrain::Montagne | Terrain::Eau => None,
        }
    }

    /// Vision en plus pour qui se tient sur la case (§ 3.2).
    pub fn bonus_vision(self) -> u8 {
        match self {
            Terrain::Colline | Terrain::Montagne => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Plateau {
    taille: u8,
    terrains: Vec<Terrain>,
}

impl Plateau {
    pub fn uni(taille: u8, terrain: Terrain) -> Plateau {
        assert!((TAILLE_MIN..=TAILLE_MAX).contains(&taille), "taille hors limites : {taille}");
        Plateau { taille, terrains: vec![terrain; taille as usize * taille as usize] }
    }

    pub fn taille(&self) -> u8 {
        self.taille
    }

    pub fn nb_cases(&self) -> usize {
        self.terrains.len()
    }

    pub fn terrain(&self, c: Case) -> Terrain {
        self.terrains[c as usize]
    }

    pub fn poser(&mut self, c: Case, t: Terrain) {
        self.terrains[c as usize] = t;
    }

    pub fn case(&self, colonne: u8, rangee: u8) -> Case {
        debug_assert!(colonne < self.taille && rangee < self.taille);
        rangee as Case * self.taille as Case + colonne as Case
    }

    pub fn colonne(&self, c: Case) -> u8 {
        (c % self.taille as Case) as u8
    }

    pub fn rangee(&self, c: Case) -> u8 {
        (c / self.taille as Case) as u8
    }

    /// Les 8 voisines (diagonales comprises, § 1.1), dans un ordre fixe : est, ouest,
    /// nord, sud, nord-est, nord-ouest, sud-est, sud-ouest. Reproductible.
    pub fn voisines(&self, c: Case, sortie: &mut [Case; 8]) -> usize {
        let (x, y, t) = (self.colonne(c) as i32, self.rangee(c) as i32, self.taille as i32);
        let mut n = 0;
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && ny >= 0 && nx < t && ny < t {
                sortie[n] = (ny * t + nx) as Case;
                n += 1;
            }
        }
        n
    }

    /// Distance entre deux cases : le plus grand des deux écarts (§ 1.1).
    pub fn distance(&self, a: Case, b: Case) -> u8 {
        let dx = (self.colonne(a) as i32 - self.colonne(b) as i32).unsigned_abs();
        let dy = (self.rangee(a) as i32 - self.rangee(b) as i32).unsigned_abs();
        dx.max(dy) as u8
    }

    /// Distance au bord le plus proche.
    pub fn distance_au_bord(&self, c: Case) -> u8 {
        let (x, y, t) = (self.colonne(c), self.rangee(c), self.taille);
        x.min(y).min(t - 1 - x).min(t - 1 - y)
    }

    /// Cases à distance `rayon` ou moins de `c` (carré), dans l'ordre croissant des cases.
    pub fn carre(&self, c: Case, rayon: u8) -> impl Iterator<Item = Case> + '_ {
        let (x, y, t, r) = (self.colonne(c) as i32, self.rangee(c) as i32, self.taille as i32, rayon as i32);
        let (x0, x1, y0, y1) = ((x - r).max(0), (x + r).min(t - 1), (y - r).max(0), (y + r).min(t - 1));
        (y0..=y1).flat_map(move |yy| (x0..=x1).map(move |xx| (yy * t + xx) as Case))
    }

    /// Adresse interne d'une case : « A1 » en bas à gauche.
    pub fn adresse(&self, c: Case) -> String {
        let lettre = (b'A' + self.colonne(c)) as char;
        format!("{lettre}{}", self.rangee(c) as u16 + 1)
    }

    /// Inverse de [`Plateau::adresse`] ; `None` si l'adresse est mal formée ou hors du plateau.
    pub fn depuis_adresse(&self, texte: &str) -> Option<Case> {
        let mut car = texte.chars();
        let lettre = car.next()?.to_ascii_uppercase();
        if !lettre.is_ascii_uppercase() {
            return None;
        }
        let colonne = lettre as u8 - b'A';
        let rangee: u16 = car.as_str().parse().ok()?;
        if colonne >= self.taille || rangee == 0 || rangee > self.taille as u16 {
            return None;
        }
        Some(self.case(colonne, (rangee - 1) as u8))
    }

    pub fn terrains_bruts(&self) -> impl Iterator<Item = u8> + '_ {
        self.terrains.iter().map(|t| *t as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adresses_comme_aux_echecs() {
        let p = Plateau::uni(11, Terrain::Plaine);
        assert_eq!(p.adresse(0), "A1");
        assert_eq!(p.adresse(p.case(1, 1)), "B2");
        assert_eq!(p.adresse(p.case(10, 10)), "K11");
        for c in 0..p.nb_cases() as Case {
            assert_eq!(p.depuis_adresse(&p.adresse(c)), Some(c));
        }
        assert_eq!(p.depuis_adresse("L1"), None);
        assert_eq!(p.depuis_adresse("A0"), None);
        assert_eq!(p.depuis_adresse("A12"), None);
        assert_eq!(p.depuis_adresse("?"), None);
    }

    #[test]
    fn voisines_au_bord() {
        let p = Plateau::uni(8, Terrain::Plaine);
        let mut v = [0; 8];
        assert_eq!(p.voisines(0, &mut v), 3);
        assert_eq!(p.voisines(p.case(3, 0), &mut v), 5);
        assert_eq!(p.voisines(p.case(3, 3), &mut v), 8);
        assert!(v.contains(&p.case(4, 4)) && v.contains(&p.case(2, 2)));
    }

    #[test]
    fn distance_et_carre() {
        let p = Plateau::uni(11, Terrain::Plaine);
        assert_eq!(p.distance(p.case(1, 1), p.case(3, 2)), 2);
        assert_eq!(p.distance(p.case(1, 1), p.case(4, 4)), 3);
        assert_eq!(p.carre(p.case(5, 5), 2).count(), 25);
        assert_eq!(p.carre(0, 2).count(), 9);
        assert_eq!(p.distance_au_bord(p.case(2, 8)), 2);
    }

    #[test]
    fn couts_des_terrains() {
        assert_eq!(Terrain::Plaine.cout(), Some(1));
        assert_eq!(Terrain::Foret.cout(), Some(2));
        assert_eq!(Terrain::Colline.cout(), Some(2));
        assert_eq!(Terrain::Colline.bonus_vision(), 1);
        assert_eq!(Terrain::Montagne.cout(), None);
        assert_eq!(Terrain::Eau.cout(), None);
    }
}
