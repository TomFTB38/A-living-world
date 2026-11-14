//! Les commandes du chat (touche T ou « / » en jeu) et les œufs d'apparition.
//!
//! Commandes : /aide, /jour, /midi, /soir, /nuit, /heure <h>, /avance [h], /vitesse <x>, /figer, /survie, /creatif,
//! /vol. Les accents, la barre oblique et la casse sont facultatifs.
//! Œufs : un œuf par créature, hostile ou non ; en main, un clic droit sur un bloc fait naître la créature dessus
//! (dans l'eau pour les bêtes de mer). En survie, l'œuf est consommé.
use crate::session::Session;
use crate::wild::Kind;

/// Les œufs d'apparition : identifiant de l'objet, nom de la créature.
pub const EGGS: [(&str, &str); 25] = [
    ("egg_rodeur", "rôdeur décharné"), ("egg_brigand", "brigand errant"), ("egg_rodeuse", "bête rôdeuse"), ("egg_rampant", "rampant nocturne"),
    ("egg_deer", "cerf"), ("egg_fox", "renard"), ("egg_hare", "lièvre"), ("egg_crow", "corbeau"), ("egg_wolf", "loup"), ("egg_boar", "sanglier"), ("egg_bear", "ours"),
    ("egg_ibex", "bouquetin"), ("egg_lizard", "lézard"), ("egg_mackerel", "maquereau"), ("egg_reef", "poisson de récif"), ("egg_dolphin", "dauphin"), ("egg_crab", "crabe"),
    ("egg_gull", "goéland"), ("egg_shark", "requin"), ("egg_whale", "baleine"),
    ("egg_cow", "vache"), ("egg_sheep", "mouton"), ("egg_pig", "cochon"), ("egg_horse", "cheval"), ("egg_chicken", "poule"),
];
fn kind_of(mob: &str) -> Option<Kind> {
    Some(match mob { "fox" => Kind::Fox, "hare" => Kind::Hare, "crow" => Kind::Crow, "wolf" => Kind::Wolf, "boar" => Kind::Boar, "bear" => Kind::Bear, "ibex" => Kind::Ibex, "lizard" => Kind::Lizard,
        "mackerel" => Kind::Fish, "reef" => Kind::Reef, "dolphin" => Kind::Dolphin, "crab" => Kind::Crab, "gull" => Kind::Gull, "shark" => Kind::Shark, "whale" => Kind::Whale, "cow" => Kind::Cow, "sheep" => Kind::Sheep, "pig" => Kind::Pig, "horse" => Kind::Horse, "chicken" => Kind::Chicken, _ => return None })
}
/// Retire accents et majuscules.
fn plain(s: &str) -> String { s.to_lowercase().chars().map(|c| match c { 'é' | 'è' | 'ê' | 'ë' => 'e', 'à' | 'â' => 'a', 'î' | 'ï' => 'i', 'ô' => 'o', 'ù' | 'û' => 'u', 'ç' => 'c', c => c }).collect() }

impl Session {
    /// Exécute une ligne du chat ; rend la réponse à afficher.
    pub fn command(&mut self, line: &str) -> String {
        let l = plain(line.trim()); let l = l.trim_start_matches('/'); let mut w = l.split_whitespace(); let cmd = w.next().unwrap_or(""); let arg = w.next();
        let num = |a: Option<&str>| a.and_then(|x| x.replace(',', ".").trim_end_matches('h').parse::<f64>().ok());
        let at = |s: &mut Session, h: f64, what: &str| { let d = (h - s.hour).rem_euclid(24.0); s.pass_hours(d); format!("Il est {}h : {what}", h as i32) };
        match cmd {
            "" => String::new(),
            "coin1" => self.set_corner(0), "coin2" => self.set_corner(1),
            "exporter" | "export" => self.export(arg),
            "structures" => { let v = crate::structures::names(); if v.is_empty() { "Aucune structure exportée pour l'instant : /coin1, /coin2 puis /exporter nom".into() } else { format!("Structures : {}", v.join(", ")) } }
            "aide" | "help" | "?" => "Commandes : /jour /midi /soir /nuit /heure <h> /avance [h] /vitesse <x> /figer /survie /creatif /vol ; construction : /coin1 /coin2 /exporter [nom] /structures".into(),
            "jour" | "matin" => at(self, 7.0, "le jour se lève"), "midi" => at(self, 12.0, "plein jour"), "soir" => at(self, 19.0, "le soir tombe"), "nuit" => at(self, 22.0, "la nuit"),
            "heure" | "temps" => match num(arg) { Some(h) => at(self, h.rem_euclid(24.0), "c'est fait"), None => format!("Il est {}h{:02}. Exemple : /heure 18", self.hour as i32, ((self.hour.fract()) * 60.0) as i32) },
            "avance" | "passer" => { let h = num(arg).unwrap_or(1.0).clamp(0.0, 240.0); self.pass_hours(h); format!("{h} heure(s) passent : il est {}h{:02}", self.hour as i32, (self.hour.fract() * 60.0) as i32) }
            "vitesse" => match num(arg) { Some(x) if x > 0.0 => { self.time_speed = x.min(200.0); self.time_frozen = false; format!("Le temps passe {} fois plus vite (une journée en {:.0} min)", self.time_speed, 25.0 / self.time_speed) } _ => format!("Vitesse du temps : {} (exemple : /vitesse 20, /vitesse 1 pour revenir)", self.time_speed) },
            "figer" | "pause" => { self.time_frozen = !self.time_frozen; if self.time_frozen { "Le temps est figé".into() } else { "Le temps reprend".into() } }
            "survie" => { self.set_creative(false); "Mode survie".into() }
            "creatif" | "creative" => { self.set_creative(true); "Mode créatif : casse instantanée, blocs illimités, œufs d'apparition, /vol pour voler".into() }
            "vol" | "voler" | "fly" => { if !self.creative { return "Le vol demande le mode créatif (/creatif)".into(); } self.player.fly = !self.player.fly; if self.player.fly { "Vol activé".into() } else { "Vol désactivé".into() } }
            _ => format!("Commande inconnue : « {} ». Tapez /aide", line.trim()),
        }
    }
    /// Fait naître la créature d'un œuf en ce point ; rend son nom, ou rien si l'œuf n'en est pas un.
    pub fn hatch(&mut self, egg: &str, p: [f64; 3]) -> Option<&'static str> {
        let (_, name) = EGGS.iter().find(|e| e.0 == egg)?; let mob = &egg[4..]; let yaw = self.player.yaw + std::f64::consts::PI;
        if let Some(sp) = crate::hostile::species(mob) { self.horde.put(sp, p, yaw, false); return Some(name); }
        if mob == "deer" { self.fauna.put(p, yaw); return Some(name); }
        let k = kind_of(mob)?;
        let wet = crate::player::Voxels::get(&self.world, p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32).is_some_and(crate::blocks::is_water);
        match k { Kind::Fish | Kind::Reef | Kind::Dolphin | Kind::Shark | Kind::Whale if wet => self.wild.put_swim(k, p, yaw, 0, 0),
            Kind::Crow | Kind::Gull => self.wild.put(k, [p[0], p[1] + 2.0, p[2]], yaw, true), _ => self.wild.put(k, p, yaw, false) }
        Some(name)
    }
}
