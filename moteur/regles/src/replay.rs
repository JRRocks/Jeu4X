//! Replay : la recette pour rejouer une partie, pas un film (E9).
//!
//! Format texte, une information par ligne, lisible par le moteur et par un programme
//! d'entraînement :
//!
//! ```text
//! jeu4x-replay 1
//! regles 1
//! codage 1
//! graine 42
//! reglages 11 3 3 30
//! actions 0 245 0 …
//! controle 2 1234567890 …     (tous les N actions : numéro d'action, empreinte)
//! fin 9876543210
//! ```

use crate::partie::{ActionRefusee, Partie, Reglages, VERSION_CODAGE};
use crate::VERSION_REGLES;

pub const VERSION_FORMAT: u32 = 1;
/// Une empreinte de contrôle toutes les `PAS_CONTROLE` actions.
pub const PAS_CONTROLE: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replay {
    pub version_regles: u32,
    pub version_codage: u32,
    pub graine: u64,
    pub reglages: Reglages,
    pub actions: Vec<u32>,
    /// (nombre d'actions jouées, empreinte à ce moment).
    pub controles: Vec<(usize, u64)>,
    pub empreinte_finale: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErreurReplay {
    Format(String),
    VersionRegles { attendue: u32, trouvee: u32 },
    ActionRefusee { rang: usize, cause: ActionRefusee },
    EmpreinteDifferente { rang: usize },
}

/// Joue les actions depuis la graine et enregistre le replay.
pub fn enregistrer(reglages: Reglages, graine: u64, actions: &[u32]) -> Result<Replay, ErreurReplay> {
    let mut p = Partie::nouvelle(reglages, graine);
    let mut controles = Vec::new();
    for (rang, &n) in actions.iter().enumerate() {
        p.appliquer_numero(n).map_err(|cause| ErreurReplay::ActionRefusee { rang, cause })?;
        if (rang + 1) % PAS_CONTROLE == 0 {
            controles.push((rang + 1, p.empreinte()));
        }
    }
    Ok(Replay {
        version_regles: VERSION_REGLES,
        version_codage: VERSION_CODAGE,
        graine,
        reglages,
        actions: actions.to_vec(),
        controles,
        empreinte_finale: p.empreinte(),
    })
}

impl Replay {
    /// Rejoue la partie et vérifie chaque empreinte. Rend l'état final.
    pub fn rejouer(&self) -> Result<Partie, ErreurReplay> {
        if self.version_regles != VERSION_REGLES {
            return Err(ErreurReplay::VersionRegles { attendue: VERSION_REGLES, trouvee: self.version_regles });
        }
        let mut p = Partie::nouvelle(self.reglages, self.graine);
        let mut k = 0;
        for (rang, &n) in self.actions.iter().enumerate() {
            p.appliquer_numero(n).map_err(|cause| ErreurReplay::ActionRefusee { rang, cause })?;
            if k < self.controles.len() && self.controles[k].0 == rang + 1 {
                if self.controles[k].1 != p.empreinte() {
                    return Err(ErreurReplay::EmpreinteDifferente { rang });
                }
                k += 1;
            }
        }
        if p.empreinte() != self.empreinte_finale {
            return Err(ErreurReplay::EmpreinteDifferente { rang: self.actions.len() });
        }
        Ok(p)
    }

    pub fn vers_texte(&self) -> String {
        let r = &self.reglages;
        let actions: Vec<String> = self.actions.iter().map(|a| a.to_string()).collect();
        let controles: Vec<String> = self.controles.iter().map(|(n, e)| format!("{n} {e}")).collect();
        format!(
            "jeu4x-replay {VERSION_FORMAT}\nregles {}\ncodage {}\ngraine {}\nreglages {} {} {} {}\nactions {}\ncontrole {}\nfin {}\n",
            self.version_regles,
            self.version_codage,
            self.graine,
            r.taille,
            r.unites_par_joueur,
            r.points_mouvement,
            r.limite_tours,
            actions.join(" "),
            controles.join(" "),
            self.empreinte_finale
        )
    }

    pub fn depuis_texte(texte: &str) -> Result<Replay, ErreurReplay> {
        let err = |m: &str| ErreurReplay::Format(m.to_string());
        let mut lignes = texte.lines();
        let mut champ = |nom: &str| -> Result<Vec<u64>, ErreurReplay> {
            let ligne = lignes.next().ok_or_else(|| err(nom))?;
            let mut mots = ligne.split_whitespace();
            if mots.next() != Some(nom) {
                return Err(err(nom));
            }
            mots.map(|m| m.parse::<u64>().map_err(|_| err(nom))).collect()
        };
        let format = champ("jeu4x-replay")?;
        if format != [VERSION_FORMAT as u64] {
            return Err(err("version du format"));
        }
        let regles = *champ("regles")?.first().ok_or_else(|| err("regles"))? as u32;
        let codage = *champ("codage")?.first().ok_or_else(|| err("codage"))? as u32;
        let graine = *champ("graine")?.first().ok_or_else(|| err("graine"))?;
        let rg = champ("reglages")?;
        if rg.len() != 4 {
            return Err(err("reglages"));
        }
        let reglages = Reglages {
            taille: rg[0] as u8,
            unites_par_joueur: rg[1] as u8,
            points_mouvement: rg[2] as u8,
            limite_tours: rg[3] as u16,
        };
        let actions = champ("actions")?.into_iter().map(|a| a as u32).collect();
        let c = champ("controle")?;
        if c.len() % 2 != 0 {
            return Err(err("controle"));
        }
        let controles = c.chunks(2).map(|p| (p[0] as usize, p[1])).collect();
        let empreinte_finale = *champ("fin")?.first().ok_or_else(|| err("fin"))?;
        Ok(Replay { version_regles: regles, version_codage: codage, graine, reglages, actions, controles, empreinte_finale })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::jouer_au_hasard;

    #[test]
    fn rejouer_donne_le_meme_etat() {
        let (fin, actions) = jouer_au_hasard(Reglages::default(), 42, 7);
        let r = enregistrer(Reglages::default(), 42, &actions).unwrap();
        assert!(r.actions.len() > PAS_CONTROLE);
        let rejouee = r.rejouer().unwrap();
        assert_eq!(rejouee, fin);
        let relu = Replay::depuis_texte(&r.vers_texte()).unwrap();
        assert_eq!(relu, r);
        assert_eq!(relu.rejouer().unwrap().empreinte(), fin.empreinte());
    }

    #[test]
    fn un_replay_altere_est_detecte() {
        let (_, actions) = jouer_au_hasard(Reglages::default(), 1, 2);
        let mut r = enregistrer(Reglages::default(), 1, &actions).unwrap();
        r.empreinte_finale ^= 1;
        assert!(matches!(r.rejouer(), Err(ErreurReplay::EmpreinteDifferente { .. })));
        let mut r2 = enregistrer(Reglages::default(), 1, &actions).unwrap();
        r2.version_regles += 1;
        assert!(matches!(r2.rejouer(), Err(ErreurReplay::VersionRegles { .. })));
    }
}
