//! Les factions et la réputation. Chaque peuple est une faction : le Royaume (les villages médiévaux), les Oasis (le désert),
//! la Ligue des ports (la côte), les Clans du Nord (taïga et montagne). Ce qu'on fait pour un village compte pour lui, et
//! pour tout son peuple : la nouvelle se répand. La position auprès d'un village mêle les deux : sa réputation propre et la
//! moitié de celle de son peuple.
//!
//! Ce qui la fait monter : commercer, passer commande, offrir, abattre des brigands près d'un village, repousser un pillage,
//! disperser un camp de brigands (tous les peuples alentour en savent gré). Ce qui la fait baisser : piller les réserves
//! d'un village.
//!
//! Ce qu'elle change : hostile, on vous chasse sans un mot de plus ; méfiant, tout coûte moitié plus cher ; ami, un quart
//! de moins ; héros, moitié moins. Devenir l'ami d'un peuple vaut un titre et un présent ; en devenir le héros, un autre, et
//! les villages de ce peuple à des lieues à la ronde vous sont indiqués. Vérifié par `tests/factions.rs`.
use crate::bourg::{civ_of, Civ};
use crate::session::Session;

/// Les peuples.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Faction { Royaume, Oasis, Ports, Clans }
pub const ALL: [Faction; 4] = [Faction::Royaume, Faction::Oasis, Faction::Ports, Faction::Clans];
impl Faction {
    pub fn name(self) -> &'static str { match self { Faction::Royaume => "le Royaume", Faction::Oasis => "les Oasis", Faction::Ports => "la Ligue des ports", Faction::Clans => "les Clans du Nord" } }
    pub fn short(self) -> &'static str { match self { Faction::Royaume => "Royaume", Faction::Oasis => "Oasis", Faction::Ports => "Ports", Faction::Clans => "Clans" } }
    /// Le titre qu'on y gagne : ami, puis héros.
    pub fn title(self, hero: bool) -> &'static str {
        match (self, hero) { (Faction::Royaume, false) => "Ami du Royaume", (Faction::Royaume, true) => "Héros du Royaume", (Faction::Oasis, false) => "Hôte des Oasis", (Faction::Oasis, true) => "Héros des sables",
            (Faction::Ports, false) => "Ami des ports", (Faction::Ports, true) => "Héros de la Ligue", (Faction::Clans, false) => "Ami des Clans", (Faction::Clans, true) => "Héros des Clans" }
    }
    /// Le peuple d'un village, d'après sa sorte.
    pub fn of_kind(kind: &str) -> Faction { match civ_of(kind) { Civ::Medieval => Faction::Royaume, Civ::Desert => Faction::Oasis, Civ::Coastal => Faction::Ports, Civ::Nordic => Faction::Clans } }
    fn i(self) -> usize { self as usize }
}

/// Le degré de la position auprès d'un village.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier { Hostile, Mefiant, Neutre, Apprecie, Ami, Heros }
impl Tier {
    pub fn of(standing: i32) -> Tier { match standing { i32::MIN..=-20 => Tier::Hostile, -19..=-1 => Tier::Mefiant, 0..=14 => Tier::Neutre, 15..=39 => Tier::Apprecie, 40..=74 => Tier::Ami, _ => Tier::Heros } }
    pub fn label(self) -> &'static str { match self { Tier::Hostile => "hostile", Tier::Mefiant => "méfiant", Tier::Neutre => "neutre", Tier::Apprecie => "apprécié", Tier::Ami => "ami", Tier::Heros => "héros" } }
    /// Ce que coûtent les échanges et les services : méfiant, moitié plus ; ami, un quart de moins ; héros, moitié moins.
    pub fn price(self, n: u32) -> u32 { if n == 0 { return 0; } match self { Tier::Mefiant | Tier::Hostile => (n * 3).div_ceil(2), Tier::Ami => (n * 3 / 4).max(1), Tier::Heros => n.div_ceil(2), _ => n } }
}
/// Les paliers de la réputation d'un peuple : ami, héros ; et le seuil où il vous tient pour un ennemi.
pub const FRIEND: f64 = 30.0; pub const HERO: f64 = 70.0; pub const FOE: f64 = -30.0;
/// La part de ce qu'on fait pour un village qui compte pour tout son peuple.
pub const SPREAD: f64 = 0.4;

/// Ce qu'on sait des peuples : leur réputation (de -100 à 100) et les paliers déjà franchis (titres et présents reçus).
#[derive(Clone, Debug, Default)]
pub struct Standing { pub rep: [f64; 4], pub friend: [bool; 4], pub hero: [bool; 4], pub foe: [bool; 4] }

impl Session {
    /// La réputation d'un peuple, arrondie.
    pub fn faction_rep(&self, f: Faction) -> i32 { self.factions.rep[f.i()].round() as i32 }
    /// Le peuple d'un village qu'on connaît (ou qui est autour).
    pub fn faction_of(&self, v: (i32, i32)) -> Option<Faction> { self.town.village(v).map(|x| Faction::of_kind(x.kind)).or_else(|| self.town.atlas.get(&v).map(|x| Faction::of_kind(x.kind))) }
    /// La position auprès d'un village : sa réputation propre, plus la moitié de celle de son peuple.
    pub fn standing(&self, v: (i32, i32)) -> i32 { (self.rep(v) + self.faction_of(v).map_or(0, |f| self.faction_rep(f) / 2)).clamp(-100, 150) }
    pub fn tier(&self, v: (i32, i32)) -> Tier { Tier::of(self.standing(v)) }
    /// La réputation d'un peuple bouge ; franchir un palier vaut un titre, un présent, ou un avertissement.
    pub fn sway(&mut self, f: Faction, n: f64) {
        let i = f.i(); let was = self.factions.rep[i]; let now = (was + n).clamp(-100.0, 100.0); self.factions.rep[i] = now; self.dirty = true;
        if now >= FRIEND && !self.factions.friend[i] { self.factions.friend[i] = true;
            self.notes.push(format!("{} vous tient pour un ami : vous voilà « {} » (les prix y baissent)", cap(f.name()), f.title(false))); let left = self.add_item(crate::economie::COIN, 20); if left > 0 { self.notes.push("Sac plein".into()); }
            self.notes.push("Présent : 20 pièces".into()); self.earn(crate::niveaux::XP_PLACE, "amitié d'un peuple"); self.sound.pickup(); }
        if now >= HERO && !self.factions.hero[i] { self.factions.hero[i] = true; let p = self.player.p;
            let mut shown = 0; for v in self.gen.villages_near(p[0], p[2], 3000.0) { if Faction::of_kind(v.kind) == f { if !self.town.known.contains(&v.id) { shown += 1; } self.discover(&v); } }
            self.notes.push(format!("Vous êtes « {} » ! Ses villages vous sont tous indiqués ({} de plus sur la carte)", f.title(true), shown)); let left = self.add_item(crate::economie::COIN, 60); if left > 0 { self.notes.push("Sac plein".into()); }
            self.earn(crate::niveaux::XP_PLACE * 2.0, "héros d'un peuple"); self.sound.pickup(); }
        if now <= FOE && !self.factions.foe[i] { self.factions.foe[i] = true; self.notes.push(format!("{} vous tient pour un ennemi : on vous fermera des portes", cap(f.name()))); }
        if now > FOE + 10.0 { self.factions.foe[i] = false; }
    }
    /// Un brigand abattu : si c'est près d'un village, il vous en sait gré.
    pub(crate) fn brigand_down(&mut self, p: [f64; 3]) {
        let near = self.town.here.iter().filter(|v| (v.c[0] as f64 - p[0]).hypot(v.c[2] as f64 - p[2]) < v.r as f64 + 60.0).map(|v| v.id).next();
        if let Some(id) = near { self.gain_rep(id, 1); }
    }
    /// Un camp de brigands dispersé : tous les villages à mille pas en savent gré, et leurs peuples avec eux.
    pub(crate) fn camp_relief(&mut self, at: [f64; 2]) {
        let vs: Vec<_> = self.gen.villages_near(at[0], at[1], 1000.0); let mut seen = [false; 4];
        for v in &vs { let r = self.town.rep.entry(v.id).or_insert(0); *r = (*r + 3).clamp(-50, 100); let f = Faction::of_kind(v.kind); if !seen[f.i()] { seen[f.i()] = true; self.sway(f, 6.0); } }
        if !vs.is_empty() { self.notes.push(format!("La nouvelle se répand : {} village{} vous en sai{} gré", vs.len(), if vs.len() > 1 { "s" } else { "" }, if vs.len() > 1 { "vent" } else { "t" })); }
    }
    /// Ce qu'on a vu des peuples, pour le journal : nom, réputation, degré, titre.
    pub fn faction_lines(&self) -> Vec<(Faction, i32, Tier, Option<&'static str>)> {
        ALL.iter().map(|&f| { let r = self.faction_rep(f); let i = f.i(); let t = match r as f64 { x if x >= HERO => Tier::Heros, x if x >= FRIEND => Tier::Ami, x if x >= 10.0 => Tier::Apprecie, x if x > -10.0 => Tier::Neutre, x if x > FOE => Tier::Mefiant, _ => Tier::Hostile }; (f, r, t, if self.factions.hero[i] { Some(f.title(true)) } else if self.factions.friend[i] { Some(f.title(false)) } else { None }) }).collect()
    }
    pub(crate) fn factions_save(&self) -> String { let s = &self.factions; let b = |x: [bool; 4]| x.iter().fold(0, |a, &v| a * 2 + v as u32);
        format!("factions {:.1} {:.1} {:.1} {:.1} {} {} {}\n", s.rep[0], s.rep[1], s.rep[2], s.rep[3], b(s.friend), b(s.hero), b(s.foe)) }
    pub(crate) fn factions_load(&mut self, a: &[&str]) { let f = |i: usize| a.get(i).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0); let u = |i: usize| a.get(i).and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
        let bits = |m: u32| { let mut o = [false; 4]; for (k, x) in o.iter_mut().enumerate() { *x = m >> (3 - k) & 1 == 1; } o };
        self.factions = Standing { rep: [f(0), f(1), f(2), f(3)], friend: bits(u(4)), hero: bits(u(5)), foe: bits(u(6)) }; }
}
fn cap(s: &str) -> String { let mut c = s.chars(); c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str()) }
