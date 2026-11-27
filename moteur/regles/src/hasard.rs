//! Générateur de hasard déterministe (PCG32, variante XSH RR), écrit dans le projet.
//! Il est rangé dans l'état de la partie : copier l'état copie aussi le hasard.

const MULTIPLICATEUR: u64 = 6364136223846793005;
const INCREMENT: u64 = 1442695040888963407;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hasard {
    etat: u64,
}

impl Hasard {
    pub fn new(graine: u64) -> Self {
        let mut h = Hasard { etat: 0 };
        h.suivant_u32();
        h.etat = h.etat.wrapping_add(graine);
        h.suivant_u32();
        h
    }

    pub fn suivant_u32(&mut self) -> u32 {
        let ancien = self.etat;
        self.etat = ancien.wrapping_mul(MULTIPLICATEUR).wrapping_add(INCREMENT);
        let xorshift = (((ancien >> 18) ^ ancien) >> 27) as u32;
        let rotation = (ancien >> 59) as u32;
        xorshift.rotate_right(rotation)
    }

    /// Entier dans `0..borne` (borne > 0), sans biais (méthode de rejet).
    pub fn sous(&mut self, borne: u32) -> u32 {
        assert!(borne > 0, "borne nulle");
        let seuil = borne.wrapping_neg() % borne;
        loop {
            let x = self.suivant_u32();
            if x >= seuil {
                return x % borne;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meme_graine_meme_suite() {
        let mut a = Hasard::new(7);
        let mut b = Hasard::new(7);
        for _ in 0..100 {
            assert_eq!(a.suivant_u32(), b.suivant_u32());
        }
    }

    #[test]
    fn sous_reste_dans_la_borne() {
        let mut h = Hasard::new(1);
        for borne in 1..50 {
            for _ in 0..50 {
                assert!(h.sous(borne) < borne);
            }
        }
    }
}
