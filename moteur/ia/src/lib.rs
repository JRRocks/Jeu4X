//! IA de Jeu4X (session 3).
//!
//! - `joueur` : ce qu'est un joueur IA, et les deux premiers (au hasard, à utilité).
//! - `criteres` : ce que l'IA à utilité mesure dans un état ; seul endroit à reprendre
//!   quand les règles de la première version arriveront.
//! - `profil` : une IA décrite par une ligne de texte (galerie, candidates).
//! - `tournoi` : parties entre IA, en parallèle, reproductibles par graine (IA-001).
//! - `elo` : classement des IA à partir des résultats (IA-001).
//! - `archive` : fichier de replays d'un tournoi, et son rejeu de contrôle (IA-001).
//! - `validation` : une nouvelle IA doit battre l'ancienne (IA-004).
//!
//! L'IA ne passe que par l'interface publique des règles : elle ne peut jouer qu'une
//! action permise. Les nombres à virgule ne servent qu'au classement, jamais aux choix.

pub mod archive;
pub mod criteres;
pub mod elo;
pub mod joueur;
pub mod profil;
pub mod tournoi;
pub mod validation;

/// Mélange de graines (SplitMix64) : dérive des graines indépendantes d'une seule.
pub fn melanger(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}
