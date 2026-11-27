//! Les peuples entre eux : alliances, guerres et routes commerciales.
//!
//! Les quatre peuples (`factions`) ont des relations, de -100 à 100 : alliés au-dessus de 40, en guerre quand elles tombent
//! sous -50 (et jusqu'à ce qu'une trêve les ramène au-dessus de -30), en paix entre les deux, en froid sous -20. Chaque monde
//! part de liens plausibles (le Royaume et la Ligue des ports commercent depuis longtemps ; le Royaume et les Clans du Nord
//! se méfient l'un de l'autre), un peu mêlés par la graine. Chaque jour, elles reviennent lentement vers ce fond ; les routes
//! commerciales ouvertes entre leurs villages les rapprochent ; et parfois survient un incident de frontière, une querelle de
//! péage, un mariage, un traité, une foire commune. Déclarations de guerre, trêves et alliances se racontent.
//!
//! Les routes commerciales relient chaque village connu à ses deux voisins les plus proches (à moins de dix-huit cents pas) ;
//! la guerre les coupe. Elles se voient sur la carte : dorées, ouvertes ; rouges, coupées.
//!
//! Ce que cela change pour le joueur : ce qu'on fait pour un peuple réjouit un peu ses alliés et fâche ses ennemis en guerre ;
//! dans les villages d'un peuple en guerre, tout coûte un quart de plus ; les caravanes ne passent pas les routes coupées.
//! Vérifié par `tests/politique.rs`.
use crate::factions::{Faction, ALL};
use crate::noise::Rng;
use crate::session::Session;

/// Où en sont deux peuples.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bond { Guerre, Froid, Paix, Allies }
impl Bond {
    pub fn label(self) -> &'static str { match self { Bond::Guerre => "en guerre", Bond::Froid => "en froid", Bond::Paix => "en paix", Bond::Allies => "alliés" } }
}
pub const ALLY: f64 = 40.0; pub const WAR: f64 = -50.0; pub const TRUCE: f64 = -30.0; pub const COLD: f64 = -20.0;
/// La part de ce qu'on fait pour un peuple que ressentent ses alliés (en bien) et ses ennemis en guerre (en mal).
pub const ALLY_SHARE: f64 = 0.25; pub const WAR_SHARE: f64 = 0.35;
/// Une route commerciale : de quel village à quel autre, et si la guerre la coupe.
#[derive(Clone, Debug, PartialEq)]
pub struct Route { pub a: (i32, i32), pub b: (i32, i32), pub pa: [f64; 2], pub pb: [f64; 2], pub open: bool, pub fa: Faction, pub fb: Faction }

/// Les relations des peuples.
#[derive(Clone, Debug)]
pub struct Politics { pub rel: [[f64; 4]; 4], pub war: [[bool; 4]; 4], base: [[f64; 4]; 4], pub day: i64, rng: Rng, /** les nouvelles des derniers jours */ pub news: Vec<(i64, String)> }
impl Politics {
    /// Les liens de départ : un fond plausible, mêlé par la graine.
    pub fn new(seed: i32) -> Politics {
        let mut r = Rng::new((seed as u32) ^ 0x9e11_7a5d); let mut base = [[0.0; 4]; 4];
        for (a, b, v) in [(0, 2, 45.0), (0, 3, -38.0), (0, 1, 5.0), (1, 2, 25.0), (1, 3, 0.0), (2, 3, -10.0)] { let v = v + r.range(-12.0, 12.0); base[a][b] = v; base[b][a] = v; }
        let mut p = Politics { rel: base, war: [[false; 4]; 4], base, day: -1, rng: r, news: Vec::new() }; p.settle(); p
    }
    pub fn bond(&self, a: Faction, b: Faction) -> Bond { let (i, j) = (a as usize, b as usize); if i == j { return Bond::Allies; } if self.war[i][j] { Bond::Guerre } else if self.rel[i][j] >= ALLY { Bond::Allies } else if self.rel[i][j] <= COLD { Bond::Froid } else { Bond::Paix } }
    pub fn at_war(&self, f: Faction) -> bool { ALL.iter().any(|&g| self.war[f as usize][g as usize]) }
    /// Les guerres commencent sous -50 et durent jusqu'à ce que la relation remonte au-dessus de -30.
    fn settle(&mut self) -> Vec<String> {
        let mut said = Vec::new();
        for i in 0..4 { for j in i + 1..4 { let (a, b) = (ALL[i].name(), ALL[j].name()); let r = self.rel[i][j];
            if !self.war[i][j] && r <= WAR { self.war[i][j] = true; self.war[j][i] = true; said.push(format!("La guerre ! {} et {} prennent les armes", cap(a), b)); }
            else if self.war[i][j] && r > TRUCE { self.war[i][j] = false; self.war[j][i] = false; said.push(format!("Trêve : {} et {} déposent les armes", cap(a), b)); } } }
        said
    }
    fn nudge(&mut self, i: usize, j: usize, n: f64) { let v = (self.rel[i][j] + n).clamp(-100.0, 100.0); self.rel[i][j] = v; self.rel[j][i] = v; }
    /// Un jour qui passe : le retour au fond, ce que font les routes ouvertes (`trade[i][j]` : combien entre i et j), et parfois
    /// un événement. Rend ce qui mérite d'être raconté.
    pub fn day_passes(&mut self, trade: [[u32; 4]; 4]) -> Vec<String> {
        let mut said = Vec::new();
        for i in 0..4 { for j in i + 1..4 { let pull = (self.base[i][j] - self.rel[i][j]) * 0.03; let t = (trade[i][j] as f64 * 0.4).min(2.0) * if self.war[i][j] { 0.0 } else { 1.0 }; self.nudge(i, j, pull + t); } }
        if self.rng.next() < 0.3 {
            let i = (self.rng.next() * 4.0) as usize % 4; let j = (i + 1 + (self.rng.next() * 3.0) as usize % 3) % 4; let (a, b) = (cap(ALL[i].name()), ALL[j].name());
            let was = self.bond(ALL[i], ALL[j]); let good = self.rng.next() < 0.5;
            let (n, what) = if good { let k = self.rng.range(6.0, 14.0); (k, [format!("{a} et {b} tiennent une foire commune"), format!("Un mariage unit {a} et {b}"), format!("{a} et {b} signent un traité de commerce")][(self.rng.next() * 3.0) as usize % 3].clone()) }
                else { let k = -self.rng.range(8.0, 20.0); (k, [format!("Incident de frontière entre {a} et {b}"), format!("{a} accuse {b} de détrousser ses marchands"), format!("Querelle de péage entre {a} et {b}")][(self.rng.next() * 3.0) as usize % 3].clone()) };
            self.nudge(i, j, n); said.push(what); let now = self.bond(ALL[i], ALL[j]);
            if now == Bond::Allies && was != Bond::Allies { said.push(format!("Alliance : {a} et {b} se jurent assistance")); }
            if was == Bond::Allies && now != Bond::Allies { said.push(format!("{a} et {b} rompent leur alliance")); }
        }
        said.extend(self.settle()); said
    }
    pub(crate) fn save(&self) -> String { let mut s = format!("politique {}", self.day); for i in 0..4 { for j in i + 1..4 { s += &format!(" {:.1}:{}", self.rel[i][j], self.war[i][j] as u8); } } s + "\n" }
    pub(crate) fn load(&mut self, a: &[&str]) {
        self.day = a.first().and_then(|v| v.parse().ok()).unwrap_or(-1); let mut k = 1;
        for i in 0..4 { for j in i + 1..4 { if let Some((r, w)) = a.get(k).and_then(|v| v.split_once(':')) { let r: f64 = r.parse().unwrap_or(self.rel[i][j]); let w = w == "1"; self.rel[i][j] = r; self.rel[j][i] = r; self.war[i][j] = w; self.war[j][i] = w; } k += 1; } }
    }
}
fn cap(s: &str) -> String { let mut c = s.chars(); c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str()) }

impl Session {
    /// Les routes commerciales entre les villages connus : chacun vers ses deux voisins les plus proches, à moins de 1800 pas.
    pub fn trade_routes(&self) -> Vec<Route> {
        let vs: Vec<_> = self.town.atlas.values().filter(|v| self.town.known.contains(&v.id) || self.town.here.iter().any(|h| h.id == v.id)).collect();
        let mut out: Vec<Route> = Vec::new();
        for v in &vs { let mut near: Vec<_> = vs.iter().filter(|w| w.id != v.id).map(|w| (((w.c[0] - v.c[0]) as f64).hypot((w.c[2] - v.c[2]) as f64), *w)).filter(|(d, _)| *d < 1800.0).collect();
            near.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (_, w) in near.into_iter().take(2) { if out.iter().any(|r| (r.a == v.id && r.b == w.id) || (r.a == w.id && r.b == v.id)) { continue; }
                let (fa, fb) = (Faction::of_kind(v.kind), Faction::of_kind(w.kind)); let open = self.politics.bond(fa, fb) != Bond::Guerre;
                out.push(Route { a: v.id, b: w.id, pa: [v.c[0] as f64 + 0.5, v.c[2] as f64 + 0.5], pb: [w.c[0] as f64 + 0.5, w.c[2] as f64 + 0.5], open, fa, fb }); } }
        out
    }
    /// Chaque jour : les relations bougent ; ce qui arrive se raconte (dans les notes, et dans les nouvelles du journal).
    pub(crate) fn politics_tick(&mut self) {
        let day = (self.hours / 24.0).floor() as i64; if self.politics.day < 0 { self.politics.day = day; return; } if day <= self.politics.day { return; }
        let n = (day - self.politics.day).min(30); self.politics.day = day;
        let mut trade = [[0u32; 4]; 4]; for r in self.trade_routes() { if r.fa != r.fb && r.open { let (i, j) = (r.fa as usize, r.fb as usize); trade[i][j] += 1; trade[j][i] += 1; } }
        for k in 0..n { for line in self.politics.day_passes(trade) { if k == n - 1 || line.starts_with("La guerre") || line.starts_with("Trêve") || line.starts_with("Alliance") { self.notes.push(line.clone()); } self.politics.news.push((day, line)); } }
        let keep = self.politics.news.len().saturating_sub(8); self.politics.news.drain(..keep); self.dirty = true;
        self.fate_day(n); // et la fortune des villages (`destin`)
    }
    /// Les liens qui ne sont pas une paix ordinaire, pour le journal.
    pub fn politics_lines(&self) -> Vec<(Faction, Faction, Bond)> {
        let mut o = Vec::new(); for i in 0..4 { for j in i + 1..4 { let b = self.politics.bond(ALL[i], ALL[j]); if b != Bond::Paix { o.push((ALL[i], ALL[j], b)); } } } o
    }
}
