//! Niveaux et compétences de l'explorateur.
//!
//! On gagne de l'expérience en explorant (chaque biome découvert, chaque lieu trouvé), en combattant (chaque créature vaincue,
//! bien davantage pour un boss ou un camp de brigands dispersé) et en aidant les villages (la réputation gagnée, un projet du
//! maire mené à bien, une attaque repoussée). Chaque niveau donne un point de compétence. L'arbre a trois branches de trois
//! compétences ; chacune a trois rangs, et chaque rang coûte plus cher que le précédent (1, puis 2, puis 3 points). Les
//! compétences du deuxième et du troisième étage d'une branche s'ouvrent quand on y a déjà investi. Remplir tout l'arbre
//! demande 54 points : il faut du temps, et des choix.
use crate::session::Session;
use std::collections::BTreeSet;

/// Une compétence : sa branche (0 Agilité, 1 Force, 2 Endurance), son étage (0 à 2), ce qu'apporte chaque rang.
#[derive(Clone, Copy, Debug)]
pub struct Skill { pub id: &'static str, pub name: &'static str, pub branch: usize, pub tier: usize, pub what: &'static str, pub icon: &'static str }
pub const BRANCHES: [&str; 3] = ["Agilité", "Force", "Endurance"];
pub const SKILLS: [Skill; 9] = [
    Skill { id: "foulee", name: "Foulée", branch: 0, tier: 0, what: "+6 % de vitesse à pied par rang", icon: "leather_boots" },
    Skill { id: "nage", name: "Nage", branch: 0, tier: 1, what: "+20 % de vitesse dans l'eau et +25 % de souffle par rang", icon: "bucket" },
    Skill { id: "escalade", name: "Escalade", branch: 0, tier: 2, what: "Grimper les parois (avancer en tenant Saut) ; plus vite à chaque rang", icon: "rope" },
    Skill { id: "frappe", name: "Frappe", branch: 1, tier: 0, what: "+6 % de dégâts par rang", icon: "sword" },
    Skill { id: "portefaix", name: "Portefaix", branch: 1, tier: 1, what: "+20 % de charge portée par rang", icon: "chest" },
    Skill { id: "puissance", name: "Coup puissant", branch: 1, tier: 2, what: "Coup lourd +15 % et chargé 10 % plus vite par rang", icon: "axe" },
    Skill { id: "souffle", name: "Souffle", branch: 2, tier: 0, what: "-8 % d'endurance dépensée par rang", icon: "bread" },
    Skill { id: "parade", name: "Parade", branch: 2, tier: 1, what: "+0,06 s pour parer au bon moment par rang", icon: "iron_shield" },
    Skill { id: "peau", name: "Peau dure", branch: 2, tier: 2, what: "-6 % de dégâts subis en combat par rang", icon: "leather" },
];
/// Les rangs d'une compétence.
pub const RANKS: u8 = 3;
/// Le prix du rang `r` (1 à 3) : 1, 2, puis 3 points.
pub fn cost(r: u8) -> u32 { r as u32 }
/// Les points qu'il faut avoir déjà mis dans la branche pour ouvrir un étage.
pub const UNLOCK: [u32; 3] = [0, 3, 8];
/// L'expérience pour passer du niveau `l` au suivant : chaque niveau en demande un peu plus.
pub fn need(l: u32) -> f64 { 60.0 + 12.0 * l as f64 }

/// Expérience gagnée : par niveau de la créature vaincue (un boss en vaut six fois plus), par biome et par lieu découverts,
/// par camp dispersé, par point de réputation, par projet de village mené à bien, par attaque repoussée.
pub const XP_KILL: f64 = 8.0; pub const XP_BOSS: f64 = 6.0; pub const XP_BIOME: f64 = 120.0; pub const XP_PLACE: f64 = 60.0;
pub const XP_CAMP: f64 = 250.0; pub const XP_REP: f64 = 5.0; pub const XP_PROJECT: f64 = 150.0; pub const XP_RAID: f64 = 120.0;

/// La progression du joueur : son expérience totale, ses rangs, les biomes et les lieux déjà découverts.
#[derive(Clone, Debug, Default)]
pub struct Prog { pub xp: f64, pub ranks: [u8; 9], pub biomes: u16, pub places: BTreeSet<(i32, i32)>, /** l'écran des compétences est ouvert */ pub open: bool }
impl Prog {
    /// Le niveau (dès 1), l'expérience déjà faite dans ce niveau, et ce qu'il demande.
    pub fn level(&self) -> (u32, f64, f64) { let (mut l, mut x) = (1, self.xp); while x >= need(l) { x -= need(l); l += 1; } (l, x, need(l)) }
    pub fn spent(&self) -> u32 { self.ranks.iter().map(|&r| (1..=r).map(cost).sum::<u32>()).sum() }
    pub fn branch_spent(&self, b: usize) -> u32 { SKILLS.iter().zip(self.ranks.iter()).filter(|(s, _)| s.branch == b).map(|(_, &r)| (1..=r).map(cost).sum::<u32>()).sum() }
    /// Les points à dépenser : un par niveau gagné.
    pub fn points(&self) -> u32 { (self.level().0 - 1).saturating_sub(self.spent()) }
    pub fn rank(&self, id: &str) -> f64 { SKILLS.iter().position(|s| s.id == id).map_or(0.0, |i| self.ranks[i] as f64) }
    /// Ce qui manque pour acheter le rang suivant de la compétence `i` (aucun : on peut).
    pub fn why_not(&self, i: usize) -> Option<String> {
        let s = SKILLS.get(i)?; let r = self.ranks[i];
        if r >= RANKS { return Some("Compétence au maximum".into()); }
        let need = UNLOCK[s.tier]; let have = self.branch_spent(s.branch);
        if have < need { return Some(format!("S'ouvre après {need} points en {}", BRANCHES[s.branch])); }
        if self.points() < cost(r + 1) { return Some(format!("Il faut {} point{}", cost(r + 1), if cost(r + 1) > 1 { "s" } else { "" })); }
        None
    }
    // Les effets, lus par le jeu.
    /// Vitesse à pied.
    pub fn stride(&self) -> f64 { 1.0 + 0.06 * self.rank("foulee") }
    /// Vitesse dans l'eau, et durée du souffle.
    pub fn swim(&self) -> f64 { 1.0 + 0.2 * self.rank("nage") }
    pub fn lungs(&self) -> f64 { 1.0 + 0.25 * self.rank("nage") }
    /// Vitesse d'escalade (0 : on ne grimpe pas).
    pub fn climb(&self) -> f64 { match self.rank("escalade") as u8 { 0 => 0.0, 1 => 1.5, 2 => 2.1, _ => 2.8 } }
    /// Ce que coûte une seconde d'escalade en endurance.
    pub fn climb_cost(&self) -> f64 { 11.0 - 2.0 * self.rank("escalade") }
    pub fn melee(&self) -> f64 { 1.0 + 0.06 * self.rank("frappe") }
    pub fn carry(&self) -> f64 { 1.0 + 0.2 * self.rank("portefaix") }
    pub fn heavy(&self) -> f64 { 1.0 + 0.15 * self.rank("puissance") }
    pub fn charge_time(&self) -> f64 { 1.0 - 0.1 * self.rank("puissance") }
    pub fn stamina(&self) -> f64 { 1.0 - 0.08 * self.rank("souffle") }
    pub fn parry(&self) -> f64 { 0.06 * self.rank("parade") }
    pub fn hide(&self) -> f64 { 1.0 - 0.06 * self.rank("peau") }
    pub(crate) fn save(&self) -> String {
        let r: Vec<String> = self.ranks.iter().map(|r| r.to_string()).collect(); let p: Vec<String> = self.places.iter().map(|(x, z)| format!("{x},{z}")).collect();
        format!("prog {:.1} {} {} {}\n", self.xp, r.join(","), self.biomes, if p.is_empty() { "-".into() } else { p.join(";") })
    }
    pub(crate) fn load(&mut self, rest: &str) {
        let mut a = rest.split_whitespace(); self.xp = a.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
        if let Some(r) = a.next() { for (i, v) in r.split(',').enumerate().take(9) { self.ranks[i] = v.parse::<u8>().unwrap_or(0).min(RANKS); } }
        self.biomes = a.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        if let Some(p) = a.next().filter(|p| *p != "-") { for c in p.split(';') { if let Some((x, z)) = c.split_once(',') { if let (Ok(x), Ok(z)) = (x.parse(), z.parse()) { self.places.insert((x, z)); } } } }
    }
}

/// Une carte de l'écran des compétences.
#[derive(Clone, Debug)]
pub struct Card { pub skill: Skill, pub rank: u8, pub next_cost: u32, pub can: bool, pub why: Option<String> }

impl Session {
    /// De l'expérience gagnée ; un niveau de plus donne un point de compétence.
    pub fn earn(&mut self, xp: f64, why: &str) {
        if self.creative || xp <= 0.0 { return; }
        let before = self.prog.level().0; self.prog.xp += xp; let after = self.prog.level().0; self.dirty = true;
        self.notes.push(format!("+{:.0} XP · {why}", xp));
        if after > before { self.sound.pickup(); self.notes.push(format!("Niveau {after} ! +{} point{} de compétence (K)", after - before, if after - before > 1 { "s" } else { "" })); }
    }
    /// Acheter le rang suivant d'une compétence.
    pub fn buy_skill(&mut self, i: usize) -> Result<String, String> {
        if let Some(w) = self.prog.why_not(i) { return Err(w); }
        self.prog.ranks[i] += 1; self.dirty = true; let s = SKILLS[i]; self.sound.pickup();
        Ok(format!("{} · rang {}/{}", s.name, self.prog.ranks[i], RANKS))
    }
    /// L'écran des compétences : chaque compétence, son rang, le prix du suivant, et si on peut le prendre.
    pub fn skill_cards(&self) -> Vec<Card> {
        SKILLS.iter().enumerate().map(|(i, s)| { let r = self.prog.ranks[i]; let why = self.prog.why_not(i); Card { skill: *s, rank: r, next_cost: if r < RANKS { cost(r + 1) } else { 0 }, can: why.is_none(), why } }).collect()
    }
    /// L'explorateur découvre : un biome qu'il n'avait jamais foulé (le premier, celui du départ, ne compte pas).
    pub(crate) fn explore_tick(&mut self) {
        let b = self.mix().main(); let bit = 1u16 << b; if self.prog.biomes & bit != 0 { return; }
        let first = self.prog.biomes == 0; self.prog.biomes |= bit; self.dirty = true;
        if !first { self.earn(XP_BIOME, &format!("nouveau biome : {}", crate::biome::NAMES[b])); }
    }
    /// Un lieu trouvé pour la première fois (tour, cimetière, camp…).
    pub(crate) fn found_place(&mut self, cell: (i32, i32), name: &str) { if self.prog.places.insert(cell) { self.earn(XP_PLACE, &format!("{name} découvert")); } }
    /// Ce que rapporte une créature vaincue.
    pub fn kill_xp(sp: usize, vet: bool) -> f64 { let s = &crate::hostile::bestiary().species[sp]; XP_KILL * (s.tier.max(1) as f64) * if s.boss { XP_BOSS } else { 1.0 } * if vet { 1.5 } else { 1.0 } }
}
