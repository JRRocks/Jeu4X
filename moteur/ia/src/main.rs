//! `terrain` : la commande du terrain d'essai des IA (IA-001, IA-004).
//!
//! ```text
//! cargo run --release -p ia -- tournoi [--ia a,b,…] [--parties 100] [--graine 1]
//!                                     [--taille 11] [--fils 4] [--rapport f.md] [--archive f.txt]
//! cargo run --release -p ia -- rejouer <archive>
//! cargo run --release -p ia -- valider <nom> [--parties 400] [--graine 1] [--inscrire]
//! cargo run --release -p ia -- galerie
//! ```
//! Sans `--ia`, le tournoi oppose toute la galerie et toutes les candidates.

use ia::archive;
use ia::criteres::NOMS;
use ia::elo::classer;
use ia::profil::{lire_liste, Profil, Sorte};
use ia::tournoi::{bilan, jouer_tout, rencontres, toutes_les_paires};
use ia::validation::{valider, SEUIL_ANCIENNES, SEUIL_CHAMPION};
use regles::partie::Reglages;
use regles::VERSION_REGLES;
use std::process::ExitCode;
use std::time::Instant;

const DOSSIER: &str = env!("CARGO_MANIFEST_DIR");

struct Options {
    libres: Vec<String>,
    valeurs: Vec<(String, String)>,
    drapeaux: Vec<String>,
}

impl Options {
    fn lire(args: &[String]) -> Options {
        let mut o = Options { libres: vec![], valeurs: vec![], drapeaux: vec![] };
        let mut i = 0;
        while i < args.len() {
            let a = &args[i];
            if let Some(nom) = a.strip_prefix("--") {
                if nom == "inscrire" {
                    o.drapeaux.push(nom.to_string());
                } else if i + 1 < args.len() {
                    o.valeurs.push((nom.to_string(), args[i + 1].clone()));
                    i += 1;
                }
            } else {
                o.libres.push(a.clone());
            }
            i += 1;
        }
        o
    }
    fn texte(&self, nom: &str) -> Option<&str> {
        self.valeurs.iter().rev().find(|(n, _)| n == nom).map(|(_, v)| v.as_str())
    }
    fn nombre(&self, nom: &str, defaut: u64) -> Result<u64, String> {
        match self.texte(nom) {
            None => Ok(defaut),
            Some(v) => v.parse().map_err(|_| format!("--{nom} : nombre attendu, reçu « {v} »")),
        }
    }
    fn drapeau(&self, nom: &str) -> bool {
        self.drapeaux.iter().any(|d| d == nom)
    }
}

fn chemin(o: &Options, nom: &str, defaut: &str) -> String {
    o.texte(nom).map(str::to_string).unwrap_or_else(|| format!("{DOSSIER}/{defaut}"))
}

fn lire_fichier(chemin: &str) -> Result<String, String> {
    std::fs::read_to_string(chemin).map_err(|e| format!("{chemin} : {e}"))
}

fn galerie_et_candidates(o: &Options) -> Result<(Vec<Profil>, Vec<Profil>), String> {
    let g = lire_liste(&lire_fichier(&chemin(o, "galerie", "galerie.txt"))?)?;
    let c = lire_liste(&lire_fichier(&chemin(o, "candidats", "candidats.txt"))?)?;
    // Une candidate admise reste écrite dans candidats.txt ; elle ne compte plus qu'une fois.
    let c = c.into_iter().filter(|p| !g.iter().any(|q| q.nom == p.nom)).collect();
    Ok((g, c))
}

fn reglages(o: &Options) -> Result<Reglages, String> {
    let taille = o.nombre("taille", Reglages::default().taille as u64)? as u8;
    if !(regles::plateau::TAILLE_MIN..=regles::plateau::TAILLE_MAX).contains(&taille) {
        return Err(format!("--taille : entre {} et {}", regles::plateau::TAILLE_MIN, regles::plateau::TAILLE_MAX));
    }
    Ok(Reglages { taille, ..Reglages::default() })
}

fn fils(o: &Options) -> Result<usize, String> {
    let defaut = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1) as u64;
    Ok(o.nombre("fils", defaut)?.max(1) as usize)
}

fn decrire(p: &Profil) -> String {
    match &p.sorte {
        Sorte::Hasard => "au hasard".to_string(),
        Sorte::Perimee { .. } => format!("faite pour les règles {}", p.version_regles),
        Sorte::Utilite { poids } => {
            let v: Vec<String> = NOMS.iter().zip(poids).map(|(n, w)| format!("{n} {w}")).collect();
            format!("utilité : {}", v.join(", "))
        }
    }
}

fn tournoi(o: &Options) -> Result<(), String> {
    let (g, c) = galerie_et_candidates(o)?;
    // Les IA faites pour d'autres règles ne jouent pas : elles sont à requalifier.
    let tous: Vec<Profil> = g.into_iter().chain(c).filter(|p| p.a_jour()).collect();
    let profils: Vec<Profil> = match o.texte("ia") {
        None => tous,
        Some(liste) => liste
            .split(',')
            .map(|n| tous.iter().find(|p| p.nom == n.trim()).cloned().ok_or(format!("IA inconnue : {n}")))
            .collect::<Result<_, _>>()?,
    };
    if profils.len() < 2 {
        return Err("il faut au moins deux IA".into());
    }
    let parties = o.nombre("parties", 100)? as u32;
    let graine = o.nombre("graine", 1)?;
    let r = reglages(o)?;
    let nf = fils(o)?;
    let liste = rencontres(&toutes_les_paires(profils.len()), parties, graine);
    let debut = Instant::now();
    let issues = jouer_tout(&profils, &liste, r, nf);
    let duree = debut.elapsed().as_secs_f64();

    let n = profils.len();
    let mut demi = vec![vec![0u32; n]; n];
    for a in 0..n {
        for b in 0..n {
            if a != b {
                let (v, e, _) = bilan(&issues, a, b);
                demi[a][b] = 2 * v + e;
            }
        }
    }
    let elo = classer(&demi);
    let mut ordre: Vec<usize> = (0..n).collect();
    ordre.sort_by(|&a, &b| elo[b].partial_cmp(&elo[a]).unwrap().then(a.cmp(&b)));
    let forfaits = issues.iter().filter(|i| i.forfait.is_some()).count();
    let actions: usize = issues.iter().map(|i| i.replay.actions.len()).sum();

    let mut s = String::new();
    s += "# Tournoi des IA\n\n";
    s += &format!(
        "Commande : `cargo run --release -p ia -- tournoi --ia {} --parties {parties} --graine {graine} --taille {}`\n\n",
        profils.iter().map(|p| p.nom.as_str()).collect::<Vec<_>>().join(","),
        r.taille
    );
    s += &format!(
        "Règles version {VERSION_REGLES} ; plateau {t}×{t}, {} joueurs, limite {} tours. {parties} parties par paire, chaque carte jouée des deux côtés ; {} parties en tout, {forfaits} forfait(s).\n\n",
        r.nb_joueurs,
        r.limite_tours,
        issues.len(),
        t = r.taille
    );
    s += "## Classement\n\nElo ajusté sur toutes les parties (Bradley-Terry), ancré à 1000 pour la première IA de la liste.\n\n";
    s += "| Rang | IA | Elo | Points | Description |\n|---|---|---|---|---|\n";
    for (rang, &a) in ordre.iter().enumerate() {
        let gagnes: u32 = demi[a].iter().sum();
        let joues: u32 = (0..n).filter(|&b| b != a).map(|b| demi[a][b] + demi[b][a]).sum();
        s += &format!(
            "| {} | {} | {:.0} | {:.1} % | {} |\n",
            rang + 1,
            profils[a].nom,
            elo[a],
            100.0 * gagnes as f64 / joues.max(1) as f64,
            decrire(&profils[a])
        );
    }
    s += "\n## Face à face\n\nVictoires-égalités-défaites de l'IA de la ligne contre celle de la colonne.\n\n|  |";
    for &b in &ordre {
        s += &format!(" {} |", profils[b].nom);
    }
    s += "\n|---|";
    s += &"---|".repeat(n);
    s += "\n";
    for &a in &ordre {
        s += &format!("| {} |", profils[a].nom);
        for &b in &ordre {
            if a == b {
                s += " — |";
            } else {
                let (v, e, d) = bilan(&issues, a, b);
                s += &format!(" {v}-{e}-{d} |");
            }
        }
        s += "\n";
    }
    s += &format!(
        "\n## Vitesse (seule partie qui varie d'une machine à l'autre)\n\n{} parties, {} actions, en {:.2} s sur {nf} fils : {:.0} parties/s.\n",
        issues.len(),
        actions,
        duree,
        issues.len() as f64 / duree
    );
    if let Some(f) = o.texte("archive") {
        std::fs::write(f, archive::ecrire(&profils, &issues)).map_err(|e| format!("{f} : {e}"))?;
        s += &format!("\nToutes les parties sont rejouables : `cargo run --release -p ia -- rejouer {f}`.\n");
    }
    match o.texte("rapport") {
        Some(f) => std::fs::write(f, &s).map_err(|e| format!("{f} : {e}"))?,
        None => print!("{s}"),
    }
    Ok(())
}

fn rejouer(o: &Options) -> Result<bool, String> {
    let f = o.libres.get(1).ok_or("rejouer : chemin de l'archive attendu")?;
    let (profils, _, c) = archive::rejouer(&lire_fichier(f)?)?;
    println!("{} IA, {} parties rejouées, {} conformes.", profils.len(), c.parties, c.conformes);
    for e in c.erreurs.iter().take(20) {
        println!("  {e}");
    }
    Ok(c.erreurs.is_empty())
}

fn valider_cmd(o: &Options) -> Result<bool, String> {
    let nom = o.libres.get(1).ok_or("valider : nom de la candidate attendu")?;
    let (g, c) = galerie_et_candidates(o)?;
    if g.iter().any(|p| &p.nom == nom) {
        return Err(format!("{nom} est déjà dans la galerie"));
    }
    let cand = c.iter().find(|p| &p.nom == nom).ok_or(format!("{nom} n'est pas dans les candidates"))?;
    if !cand.a_jour() {
        return Err(format!("{nom} est faite pour les règles {}, pas {VERSION_REGLES}", cand.version_regles));
    }
    let parties = o.nombre("parties", 400)? as u32;
    let v = valider(cand, &g, parties, o.nombre("graine", 1)?, reglages(o)?, fils(o)?);
    println!("Candidate : {} ({})", v.candidate, decrire(cand));
    for e in &v.ecartees {
        println!("  écartée (autres règles, à requalifier) : {e}");
    }
    for c in &v.confrontations {
        let seuil = if c.champion { SEUIL_CHAMPION } else { SEUIL_ANCIENNES };
        println!(
            "  contre {}{} : {}-{}-{}, {:.1} % des points (exigé {:.1} %) : {}",
            c.adversaire,
            if c.champion { " (en service)" } else { "" },
            c.victoires,
            c.egalites,
            c.defaites,
            c.pour_mille as f64 / 10.0,
            seuil as f64 / 10.0,
            if c.reussie { "réussi" } else { "ÉCHEC" }
        );
    }
    println!("Verdict : {}", if v.admise { "ADMISE" } else { "REFUSÉE" });
    if v.admise && o.drapeau("inscrire") {
        let f = chemin(o, "galerie", "galerie.txt");
        let mut texte = lire_fichier(&f)?;
        if !texte.ends_with('\n') {
            texte.push('\n');
        }
        texte += &cand.vers_ligne();
        texte.push('\n');
        std::fs::write(&f, texte).map_err(|e| format!("{f} : {e}"))?;
        println!("Inscrite dans la galerie : {} est maintenant l'IA en service.", cand.nom);
    }
    Ok(v.admise)
}

fn galerie_cmd(o: &Options) -> Result<(), String> {
    let (g, c) = galerie_et_candidates(o)?;
    let service = g.iter().rev().find(|p| p.a_jour()).map(|p| p.nom.clone());
    println!("Galerie (règles en cours : {VERSION_REGLES}) :");
    for p in &g {
        let marque = if Some(&p.nom) == service.as_ref() { " ← en service" } else if !p.a_jour() { " (autres règles)" } else { "" };
        println!("  {} : {}{marque}", p.nom, decrire(p));
    }
    println!("Candidates :");
    for p in &c {
        println!("  {} : {}", p.nom, decrire(p));
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let o = Options::lire(&args);
    let r = match o.libres.first().map(String::as_str) {
        Some("tournoi") => tournoi(&o).map(|_| true),
        Some("rejouer") => rejouer(&o),
        Some("valider") => valider_cmd(&o),
        Some("galerie") => galerie_cmd(&o).map(|_| true),
        _ => Err("commandes : tournoi, rejouer, valider, galerie (voir l'en-tête de src/main.rs)".into()),
    };
    match r {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::from(2)
        }
    }
}
