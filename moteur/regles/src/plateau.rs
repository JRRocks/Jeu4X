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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Terrain {
    Plaine = 0,
    Foret = 1,
    Montagne = 2,
    Eau = 3,
}

pub const NB_TERRAINS: usize = 4;

impl Terrain {
    pub fn depuis_u8(v: u8) -> Terrain {
        match v {
            0 => Terrain::Plaine,
            1 => Terrain::Foret,
            2 => Terrain::Montagne,
            _ => Terrain::Eau,
        }
    }

    /// Coût d'entrée dans la case, en points de mouvement ; `None` = infranchissable.
    pub fn cout(self) -> Option<u8> {
        match self {
            Terrain::Plaine => Some(1),
            Terrain::Foret => Some(2),
            Terrain::Montagne | Terrain::Eau => None,
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

    /// Voisines orthogonales, dans un ordre fixe (est, ouest, nord, sud) : reproductible.
    pub fn voisines(&self, c: Case, sortie: &mut [Case; 4]) -> usize {
        let (x, y, t) = (self.colonne(c), self.rangee(c), self.taille);
        let mut n = 0;
        if x + 1 < t {
            sortie[n] = c + 1;
            n += 1;
        }
        if x > 0 {
            sortie[n] = c - 1;
            n += 1;
        }
        if y + 1 < t {
            sortie[n] = c + t as Case;
            n += 1;
        }
        if y > 0 {
            sortie[n] = c - t as Case;
            n += 1;
        }
        n
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
        let mut v = [0; 4];
        assert_eq!(p.voisines(0, &mut v), 2);
        assert_eq!(p.voisines(p.case(3, 3), &mut v), 4);
    }

    #[test]
    fn couts_des_terrains() {
        assert_eq!(Terrain::Plaine.cout(), Some(1));
        assert_eq!(Terrain::Foret.cout(), Some(2));
        assert_eq!(Terrain::Montagne.cout(), None);
        assert_eq!(Terrain::Eau.cout(), None);
    }
}
