//! Archive d'un tournoi : tous ses replays dans un fichier texte, et leur rejeu (IA-001).
//!
//! ```text
//! jeu4x-tournoi 1
//! ia hasard-r2 2 hasard                 (une ligne par profil, dans l'ordre des indices)
//! ia utilite-r2-v1 2 utilite/2 100 -50 5 3
//! partie 0 1 gagnant 1 forfait - scores 20 31
//! jeu4x-replay 1                        (le replay de la partie, format des règles)
//! …
//! fin 1234
//! ```
//!
//! Rejouer l'archive refait chaque partie depuis sa graine avec les actions enregistrées
//! (pas avec les IA) et vérifie empreintes, scores et gagnant.

use crate::profil::Profil;
use crate::tournoi::{Issue, Rencontre};
use regles::replay::Replay;

pub const VERSION_ARCHIVE: u32 = 1;
/// Lignes d'un replay des règles (`Replay::vers_texte`).
const LIGNES_REPLAY: usize = 8;

pub fn ecrire(profils: &[Profil], issues: &[Issue]) -> String {
    let mut s = format!("jeu4x-tournoi {VERSION_ARCHIVE}\n");
    for p in profils {
        s += &format!("ia {}\n", p.vers_ligne());
    }
    let opt = |o: Option<u8>| o.map_or("-".to_string(), |v| v.to_string());
    for i in issues {
        s += &format!(
            "partie {} {} gagnant {} forfait {} scores {}\n",
            i.rencontre.place0,
            i.rencontre.place1,
            opt(i.gagnant),
            opt(i.forfait),
            i.scores.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" ")
        );
        s += &i.replay.vers_texte();
    }
    s
}

/// Ce que le rejeu d'une archive a trouvé.
#[derive(Debug, Default)]
pub struct Controle {
    pub parties: usize,
    pub conformes: usize,
    pub erreurs: Vec<String>,
}

/// Relit et rejoue une archive. Rend les profils, les issues relues et le contrôle.
pub fn rejouer(texte: &str) -> Result<(Vec<Profil>, Vec<Issue>, Controle), String> {
    let lignes: Vec<&str> = texte.lines().collect();
    let mut k = 0;
    if lignes.first().map(|l| l.trim()) != Some(&format!("jeu4x-tournoi {VERSION_ARCHIVE}")[..]) {
        return Err("en-tête d'archive absent ou d'une autre version".into());
    }
    k += 1;
    let mut profils = Vec::new();
    while k < lignes.len() && lignes[k].starts_with("ia ") {
        profils.push(Profil::depuis_ligne(&lignes[k][3..])?);
        k += 1;
    }
    let mut issues = Vec::new();
    let mut c = Controle::default();
    while k < lignes.len() {
        let entete: Vec<&str> = lignes[k].split_whitespace().collect();
        if entete.len() < 9 || entete[0] != "partie" || entete[7] != "scores" {
            return Err(format!("ligne {} : « partie » attendu", k + 1));
        }
        let num = |m: &str| m.parse::<usize>().map_err(|_| format!("ligne {} illisible", k + 1));
        let opt = |m: &str| -> Result<Option<u8>, String> {
            if m == "-" {
                Ok(None)
            } else {
                m.parse::<u8>().map(Some).map_err(|_| format!("ligne {} illisible", k + 1))
            }
        };
        let (place0, place1) = (num(entete[1])?, num(entete[2])?);
        let gagnant = opt(entete[4])?;
        let forfait = opt(entete[6])?;
        let scores = entete[8..].iter().map(|m| m.parse::<i32>().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?;
        if place0 >= profils.len() || place1 >= profils.len() {
            return Err(format!("ligne {} : IA inconnue", k + 1));
        }
        let fin = (k + 1 + LIGNES_REPLAY).min(lignes.len());
        let bloc = lignes[k + 1..fin].join("\n");
        let replay = Replay::depuis_texte(&bloc).map_err(|e| format!("partie {} : {e:?}", c.parties))?;
        let n = c.parties;
        c.parties += 1;
        match replay.rejouer() {
            Ok(etat) => {
                // Une partie perdue par forfait s'arrête avant la fin : on ne compare que l'état.
                let attendu = if forfait.is_some() { gagnant } else { etat.gagnant() };
                if etat.scores() == scores && attendu == gagnant && (forfait.is_some() || etat.est_finie()) {
                    c.conformes += 1;
                } else {
                    c.erreurs.push(format!("partie {n} : le rejeu ne redonne pas le résultat enregistré"));
                }
            }
            Err(e) => c.erreurs.push(format!("partie {n} : {e:?}")),
        }
        let rencontre = Rencontre { place0, place1, graine_partie: replay.graine, graine_ia: 0 };
        issues.push(Issue { rencontre, gagnant, forfait, scores, replay });
        k = fin;
    }
    Ok((profils, issues, c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tournoi::{jouer_tout, rencontres};
    use regles::partie::Reglages;

    fn petit() -> (Vec<Profil>, Vec<Issue>) {
        let p = vec![Profil::hasard("h"), Profil::utilite("u", crate::criteres::POIDS_ESSAI)];
        let i = jouer_tout(&p, &rencontres(&[(0, 1)], 6, 2), Reglages::default(), 2);
        (p, i)
    }

    #[test]
    fn une_archive_se_relit_et_se_rejoue() {
        let (p, i) = petit();
        let (p2, i2, c) = rejouer(&ecrire(&p, &i)).unwrap();
        assert_eq!(p2, p);
        assert_eq!(c.parties, 6);
        assert_eq!(c.conformes, 6, "{:?}", c.erreurs);
        assert_eq!(i2.iter().map(|x| x.gagnant).collect::<Vec<_>>(), i.iter().map(|x| x.gagnant).collect::<Vec<_>>());
    }

    #[test]
    fn un_resultat_falsifie_est_detecte() {
        let (p, mut i) = petit();
        // On attribue la première partie à l'autre camp.
        i[0].gagnant = i[0].gagnant.map(|g| 1 - g).or(Some(0));
        let (_, _, c) = rejouer(&ecrire(&p, &i)).unwrap();
        assert_eq!(c.conformes, 5);
        assert_eq!(c.erreurs.len(), 1);
    }

    #[test]
    fn une_action_falsifiee_est_detectee() {
        let (p, mut i) = petit();
        let a = &mut i[1].replay.actions;
        let m = a.len() / 2;
        a[m] = if a[m] == 0 { 1 } else { 0 };
        let (_, _, c) = rejouer(&ecrire(&p, &i)).unwrap();
        assert!(c.conformes < 6);
    }
}
