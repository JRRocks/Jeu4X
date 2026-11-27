//! Profil d'une IA : une ligne de texte, lisible et versionnée.
//!
//! `nom version_regles type poids…`, par exemple `utilite-v1 1 utilite/1 100 -100 10 0`.
//! Lignes vides et lignes commençant par `#` ignorées.

use crate::criteres::{NB_CRITERES, VERSION_CRITERES};
use crate::joueur::{Joueur, JoueurHasard, JoueurUtilite};
use regles::VERSION_REGLES;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sorte {
    Hasard,
    Utilite { poids: [i32; NB_CRITERES] },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profil {
    pub nom: String,
    pub version_regles: u32,
    pub sorte: Sorte,
}

impl Profil {
    pub fn hasard(nom: &str) -> Profil {
        Profil { nom: nom.to_string(), version_regles: VERSION_REGLES, sorte: Sorte::Hasard }
    }

    pub fn utilite(nom: &str, poids: [i32; NB_CRITERES]) -> Profil {
        Profil { nom: nom.to_string(), version_regles: VERSION_REGLES, sorte: Sorte::Utilite { poids } }
    }

    /// Lit une ligne. Refuse un type inconnu ou des critères d'une autre version.
    pub fn depuis_ligne(ligne: &str) -> Result<Profil, String> {
        let mots: Vec<&str> = ligne.split_whitespace().collect();
        if mots.len() < 3 {
            return Err(format!("ligne incomplète : « {ligne} »"));
        }
        let nom = mots[0].to_string();
        let version_regles: u32 = mots[1].parse().map_err(|_| format!("{nom} : version des règles illisible"))?;
        let sorte = match mots[2] {
            "hasard" => Sorte::Hasard,
            t if t.starts_with("utilite/") => {
                let v: u32 = t[8..].parse().map_err(|_| format!("{nom} : version des critères illisible"))?;
                if v != VERSION_CRITERES {
                    return Err(format!("{nom} : critères version {v}, attendue {VERSION_CRITERES}"));
                }
                if mots.len() != 3 + NB_CRITERES {
                    return Err(format!("{nom} : {NB_CRITERES} poids attendus"));
                }
                let mut poids = [0; NB_CRITERES];
                for (p, m) in poids.iter_mut().zip(&mots[3..]) {
                    *p = m.parse().map_err(|_| format!("{nom} : poids illisible « {m} »"))?;
                }
                Sorte::Utilite { poids }
            }
            t => return Err(format!("{nom} : type inconnu « {t} »")),
        };
        Ok(Profil { nom, version_regles, sorte })
    }

    pub fn vers_ligne(&self) -> String {
        match &self.sorte {
            Sorte::Hasard => format!("{} {} hasard", self.nom, self.version_regles),
            Sorte::Utilite { poids } => {
                let p: Vec<String> = poids.iter().map(|x| x.to_string()).collect();
                format!("{} {} utilite/{} {}", self.nom, self.version_regles, VERSION_CRITERES, p.join(" "))
            }
        }
    }

    /// Le profil est-il fait pour les règles en cours ?
    pub fn a_jour(&self) -> bool {
        self.version_regles == VERSION_REGLES
    }

    /// Fabrique le joueur ; `graine` ne sert qu'aux IA qui tirent au sort.
    pub fn fabriquer(&self, graine: u64) -> Box<dyn Joueur> {
        match &self.sorte {
            Sorte::Hasard => Box::new(JoueurHasard::new(graine)),
            Sorte::Utilite { poids } => Box::new(JoueurUtilite::new(*poids)),
        }
    }
}

/// Lit un fichier de profils (galerie ou candidates).
pub fn lire_liste(texte: &str) -> Result<Vec<Profil>, String> {
    texte
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(Profil::depuis_ligne)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour_texte() {
        let p = Profil::utilite("essai", [1, -2, 3, 4]);
        assert_eq!(Profil::depuis_ligne(&p.vers_ligne()).unwrap(), p);
        let h = Profil::hasard("h");
        assert_eq!(Profil::depuis_ligne(&h.vers_ligne()).unwrap(), h);
    }

    #[test]
    fn criteres_d_une_autre_version_refuses() {
        assert!(Profil::depuis_ligne("x 1 utilite/999 1 2 3 4").is_err());
        assert!(Profil::depuis_ligne("x 1 utilite/1 1 2").is_err());
        assert!(Profil::depuis_ligne("x 1 inconnu").is_err());
    }

    #[test]
    fn fichiers_du_depot_lisibles() {
        let g = lire_liste(include_str!("../galerie.txt")).unwrap();
        assert!(!g.is_empty());
        lire_liste(include_str!("../candidats.txt")).unwrap();
    }
}
