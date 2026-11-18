//! Les mécanismes des donjons.
//!
//! Un **fil** relie des entrées (leviers, dalles à pression) à des effets : des grilles qui s'ouvrent (et se referment, sauf si
//! le fil verrouille), des pics qui sortent, des lance-fléchettes qui tirent. Le fil s'enclenche quand une entrée est active
//! (ou toutes, pour une énigme). Une dalle s'enfonce sous le joueur, une créature, ou un bloc qu'on y a poussé. À part les
//! fils : les pics sortis blessent qui marche dessus, et le sol friable s'effondre un instant après qu'on y a mis le pied.
//! Le levier se manœuvre au clic droit ; le bloc à pousser glisse d'une case, droit devant soi, au clic droit.
use crate::blocks::{by_name, Id, BLOCKS};
use crate::session::Session;
use crate::player::Voxels;
use std::collections::BTreeMap;

/// Un fil : ses entrées, ce qu'il commande, son état.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wire {
    /// leviers ou dalles à pression
    pub inputs: Vec<[i32; 3]>,
    /// toutes les entrées doivent être actives (sinon une seule suffit)
    pub all: bool,
    /// les cases d'une grille et le bloc qui les ferme
    pub doors: Vec<([i32; 3], Id)>,
    /// les lance-fléchettes qui tirent quand le fil s'enclenche
    pub darts: Vec<[i32; 3]>,
    /// les pics qui sortent quand le fil s'enclenche (et rentrent quand il se relâche)
    pub spikes: Vec<[i32; 3]>,
    /// une fois enclenché, il le reste (la grille ne se referme plus)
    pub latch: bool,
    pub on: bool,
}
/// Tous les fils du monde, et l'état des pièges.
#[derive(Clone, Debug, Default)]
pub struct Mech { pub wires: Vec<Wire>, crumble: BTreeMap<[i32; 3], f64>, hurt_t: f64 }

/// Les dégâts des pics (par atteinte), d'une fléchette, et le temps que tient le sol friable.
pub const SPIKE_DMG: f64 = 5.0; pub const DART_DMG: f64 = 7.0; pub const CRUMBLE_T: f64 = 0.6;
fn id(n: &str) -> Id { by_name(n).unwrap_or(0) }
fn name(b: Id) -> &'static str { BLOCKS[b as usize].name }
/// Le nom sans son sens (« lever_on@2 » → « lever_on ») et le sens.
fn split(n: &str) -> (&str, &str) { match n.split_once('@') { Some((a, f)) => (a, f), None => (n, "") } }
fn with_face(base: &str, f: &str) -> Id { id(&if f.is_empty() { base.to_string() } else { format!("{base}@{f}") }) }

impl Session {
    /// Un levier est-il abaissé, une dalle enfoncée ?
    fn input_on(&self, p: [i32; 3]) -> bool { self.world.get(p[0], p[1], p[2]).is_some_and(|b| { let n = split(name(b)).0; n == "lever_on" || n == "pressure_plate_on" }) }
    /// Quelque chose pèse-t-il sur la dalle `p` : le joueur, une créature, un bloc à pousser ?
    fn weighs(&self, p: [i32; 3]) -> bool {
        let on = |q: [f64; 3]| q[0].floor() as i32 == p[0] && q[2].floor() as i32 == p[2] && ((q[1] - (p[1] + 1) as f64).abs() < 0.3);
        (self.dead.is_none() && !self.player.fly && on(self.player.p)) || self.horde.foes.iter().any(|f| f.dead == 0.0 && on(f.p)) || self.wild.all.iter().any(|c| on(c.p))
            || self.world.get(p[0], p[1] + 1, p[2]) == Some(id("push_block"))
    }
    /// Les mécanismes, à chaque pas de temps.
    pub(crate) fn mech_tick(&mut self, dt: f64) {
        // les dalles des fils s'enfoncent ou remontent
        let plates: Vec<[i32; 3]> = self.mech.wires.iter().flat_map(|w| w.inputs.iter().copied()).filter(|p| self.world.get(p[0], p[1], p[2]).is_some_and(|b| split(name(b)).0.starts_with("pressure_plate"))).collect();
        for p in plates { let down = self.weighs(p); if down != self.input_on(p) { self.world.edit(p[0], p[1], p[2], id(if down { "pressure_plate_on" } else { "pressure_plate" })); self.sound.place("stone"); } }
        // les fils s'enclenchent ou se relâchent
        for k in 0..self.mech.wires.len() {
            let w = &self.mech.wires[k]; if w.inputs.is_empty() { continue; }
            let act = if w.all { w.inputs.iter().all(|p| self.input_on(*p)) } else { w.inputs.iter().any(|p| self.input_on(*p)) };
            if act == w.on || (w.latch && w.on) { continue; }
            let w = self.mech.wires[k].clone(); self.mech.wires[k].on = act; self.dirty = true;
            for (p, closed) in &w.doors { let now = self.world.get(p[0], p[1], p[2]); if act && now == Some(*closed) { self.world.edit(p[0], p[1], p[2], 0); } else if !act && now == Some(0) { self.world.edit(p[0], p[1], p[2], *closed); } }
            if !w.doors.is_empty() { self.sound.place("iron_bars"); if act { self.notes.push("Une grille se lève".into()); } }
            for p in &w.spikes { self.world.edit(p[0], p[1], p[2], id(if act { "spikes" } else { "spikes_down" })); }
            if act { for p in &w.darts { self.dart(*p); } }
        }
        // les pics sortis blessent ; le sol friable cède
        if self.dead.is_some() || self.creative || self.player.fly { return; }
        let f = [self.player.p[0].floor() as i32, (self.player.p[1] + 0.05).floor() as i32, self.player.p[2].floor() as i32];
        self.mech.hurt_t -= dt; if self.world.get(f[0], f[1], f[2]) == Some(id("spikes")) && self.mech.hurt_t <= 0.0 { self.mech.hurt_t = 0.7; self.harm(SPIKE_DMG, "Des pics dans un donjon", false); }
        let below = [f[0], f[1] - 1, f[2]];
        if self.player.ground && self.world.get(below[0], below[1], below[2]) == Some(id("crumbling_floor")) && !self.mech.crumble.contains_key(&below) { self.mech.crumble.insert(below, CRUMBLE_T); self.sound.place("gravel"); }
        let mut fall = Vec::new(); for (p, t) in self.mech.crumble.iter_mut() { *t -= dt; if *t <= 0.0 { fall.push(*p); } }
        for p in fall { self.mech.crumble.remove(&p); if self.world.get(p[0], p[1], p[2]) == Some(id("crumbling_floor")) { self.world.edit(p[0], p[1], p[2], 0); self.sound.place("gravel"); self.dirty = true; } }
    }
    /// Un lance-fléchettes tire, par sa face libre la plus proche du joueur.
    fn dart(&mut self, p: [i32; 3]) {
        let me = self.player.p; let mut best: Option<([f64; 3], f64)> = None;
        for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if self.world.get(p[0] + dx, p[1], p[2] + dz) != Some(0) { continue; }
            let d = (me[0] - (p[0] as f64 + 0.5)) * dx as f64 + (me[2] - (p[2] as f64 + 0.5)) * dz as f64; if best.is_none_or(|b| d > b.1) { best = Some(([dx as f64, 0.0, dz as f64], d)); } }
        let Some((dir, _)) = best else { return };
        let from = [p[0] as f64 + 0.5 + dir[0] * 0.6, p[1] as f64 + 0.5, p[2] as f64 + 0.5 + dir[2] * 0.6];
        self.horde.shots.push((from, [dir[0] * 24.0, 0.6, dir[2] * 24.0], 0.0, DART_DMG, "Une fléchette piégée")); self.sound.swing(None, 0.6);
    }
    /// Clic droit sur un levier : il bascule. Rend vrai si c'en était un.
    pub fn lever_at(&mut self, p: [i32; 3]) -> bool {
        let Some(b) = self.world.get(p[0], p[1], p[2]) else { return false }; let (n, f) = split(name(b));
        let to = match n { "lever" => "lever_on", "lever_on" => "lever", _ => return false };
        self.world.edit(p[0], p[1], p[2], with_face(to, f)); self.sound.place("lever"); self.dirty = true; true
    }
    /// Clic droit sur un bloc à pousser : il glisse d'une case, droit devant soi (et tombe s'il n'a plus de sol). Rend vrai si
    /// c'en était un.
    pub fn push_at(&mut self, p: [i32; 3]) -> bool {
        if self.world.get(p[0], p[1], p[2]) != Some(id("push_block")) { return false; }
        let (dx, dz) = { let (ex, ez) = (p[0] as f64 + 0.5 - self.player.p[0], p[2] as f64 + 0.5 - self.player.p[2]); if ex.abs() >= ez.abs() { (ex.signum() as i32, 0) } else { (0, ez.signum() as i32) } };
        let mut q = [p[0] + dx, p[1], p[2] + dz]; if self.world.get(q[0], q[1], q[2]) != Some(0) { self.notes.push("Le bloc ne bouge pas : quelque chose le bloque".into()); return true; }
        for _ in 0..8 { if self.world.get(q[0], q[1] - 1, q[2]) == Some(0) { q[1] -= 1; } else { break; } }
        self.world.edit(p[0], p[1], p[2], 0); self.world.edit(q[0], q[1], q[2], id("push_block")); self.sound.place("stone"); self.dirty = true; true
    }
    pub(crate) fn mech_save(&self) -> String {
        let pts = |v: &[[i32; 3]]| if v.is_empty() { "-".to_string() } else { v.iter().map(|p| format!("{},{},{}", p[0], p[1], p[2])).collect::<Vec<_>>().join(";") };
        self.mech.wires.iter().map(|w| { let doors = if w.doors.is_empty() { "-".to_string() } else { w.doors.iter().map(|(p, b)| format!("{},{},{},{}", p[0], p[1], p[2], name(*b))).collect::<Vec<_>>().join(";") };
            format!("wire {} {} {} {} {} {} {}\n", w.all as u8, w.latch as u8, w.on as u8, pts(&w.inputs), doors, pts(&w.darts), pts(&w.spikes)) }).collect()
    }
    pub(crate) fn mech_load(&mut self, rest: &str) {
        let a: Vec<&str> = rest.split_whitespace().collect(); if a.len() < 7 { return; }
        let pts = |s: &str| -> Vec<[i32; 3]> { if s == "-" { return Vec::new(); } s.split(';').filter_map(|c| { let v: Vec<i32> = c.split(',').filter_map(|x| x.parse().ok()).collect(); (v.len() == 3).then(|| [v[0], v[1], v[2]]) }).collect() };
        let doors = if a[4] == "-" { Vec::new() } else { a[4].split(';').filter_map(|c| { let v: Vec<&str> = c.splitn(4, ',').collect(); if v.len() < 4 { return None; } Some(([v[0].parse().ok()?, v[1].parse().ok()?, v[2].parse().ok()?], by_name(v[3])?)) }).collect() };
        self.mech.wires.push(Wire { all: a[0] == "1", latch: a[1] == "1", on: a[2] == "1", inputs: pts(a[3]), doors, darts: pts(a[5]), spikes: pts(a[6]) });
    }
}
