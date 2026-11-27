//! Règles du jeu Jeu4X.
//!
//! Contraintes (besoins de la session 3, `docs/ia/besoins-moteur.md`) :
//! - aucune dépendance à un écran, à l'horloge ni au hasard du système ;
//! - entiers seulement, jamais de virgule ;
//! - le générateur de hasard vit dans l'état et ne dépend que de la graine ;
//! - aucun état global : une partie par valeur, plusieurs parties en parallèle.
//!
//! Contenu : plateau carré à adresses internes (`plateau`), création de la carte (`carte`), partie au tour par tour
//! (`partie`), replays (`replay`), joueur au hasard (`aleatoire`). Les adresses A1… ne
//! sont jamais affichées au joueur (D-16).

pub mod aleatoire;
pub mod carte;
pub mod empreinte;
pub mod hasard;
pub mod partie;
pub mod plateau;
pub mod replay;

/// Version des règles, inscrite dans chaque replay et chaque modèle d'IA (E10).
pub const VERSION_REGLES: u32 = 2;

/// Auto-contrôle : tire `n` nombres depuis `graine` et rend l'empreinte de la suite.
/// Sert au test de la frontière (même valeur sur le PC et sur le téléphone).
pub fn auto_controle(graine: u64, n: u32) -> u64 {
    let mut h = hasard::Hasard::new(graine);
    let mut e = empreinte::Empreinte::new();
    for _ in 0..n {
        e.ajouter_u32(h.suivant_u32());
    }
    e.valeur()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_controle_reproductible() {
        assert_eq!(auto_controle(42, 1000), auto_controle(42, 1000));
        assert_ne!(auto_controle(42, 1000), auto_controle(43, 1000));
    }

    #[test]
    fn auto_controle_valeur_figee() {
        // Valeur de référence : si elle change, les replays existants ne se rejouent plus.
        // Toute modification volontaire doit monter VERSION_REGLES.
        assert_eq!(auto_controle(42, 1000), REFERENCE_42_1000);
    }

    pub const REFERENCE_42_1000: u64 = 14318864696927641462;
}
