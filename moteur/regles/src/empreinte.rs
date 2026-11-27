//! Empreinte de 64 bits (FNV-1a) pour comparer deux exécutions d'une partie (E3).

#[derive(Clone, Copy, Debug)]
pub struct Empreinte(u64);

impl Default for Empreinte {
    fn default() -> Self {
        Self::new()
    }
}

impl Empreinte {
    pub const fn new() -> Self {
        Empreinte(0xcbf2_9ce4_8422_2325)
    }

    pub fn ajouter_octet(&mut self, o: u8) {
        self.0 = (self.0 ^ o as u64).wrapping_mul(0x0000_0100_0000_01b3);
    }

    pub fn ajouter_u32(&mut self, v: u32) {
        for o in v.to_le_bytes() {
            self.ajouter_octet(o);
        }
    }

    pub fn valeur(&self) -> u64 {
        self.0
    }
}
