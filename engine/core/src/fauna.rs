//! Faune simple : habitat, comportements, réactions au joueur. Mêmes règles et même suite aléatoire que le prototype.
use crate::actors::{b, Draw, Models};
use crate::blocks::*;
use crate::det::{cos, hypot2, sin};
use crate::mesher::VBuf;
use crate::noise::Rng;
use crate::player::Voxels;
use crate::view::{mul, roll_z, trs, M4};
pub const BIRD: f64 = 1.7; // à l'échelle réelle, un oiseau disparaît dans l'herbe et le feuillage
const FUR: [f64; 3] = [0.68, 0.45, 0.26]; const FUR2: [f64; 3] = [0.56, 0.36, 0.2]; const CREAM: [f64; 3] = [0.92, 0.84, 0.7]; const DARK: [f64; 3] = [0.15, 0.1, 0.07]; const ANT: [f64; 3] = [0.74, 0.62, 0.45];
const MAX_DEER: usize = 5; const MAX_BIRDS: usize = 9;
/// Maillages, dans l'ordre du prototype.
pub fn parts() -> [(&'static str, VBuf); 5] {
    let mut body = VBuf::new(300); { let o = &mut body;
        b(o, [-0.2, 0.0, -0.55, 0.2, 0.4, 0.55], FUR); b(o, [-0.16, -0.05, -0.45, 0.16, 0.02, 0.45], CREAM);
        for (x, z) in [(-0.11, -0.3), (0.09, -0.12), (-0.06, 0.12), (0.12, 0.32), (0.02, -0.42), (-0.13, 0.36)] { b(o, [x - 0.035, 0.4, z - 0.035, x + 0.035, 0.42, z + 0.035], CREAM); }
        b(o, [-0.07, 0.24, -0.62, 0.07, 0.4, -0.55], CREAM); }
    let mut head = VBuf::new(300); { let o = &mut head;
        b(o, [-0.09, 0.0, -0.08, 0.09, 0.42, 0.1], FUR2); b(o, [-0.11, 0.38, -0.08, 0.11, 0.58, 0.16], FUR); b(o, [-0.07, 0.4, 0.16, 0.07, 0.51, 0.3], FUR2);
        b(o, [-0.04, 0.44, 0.3, 0.04, 0.5, 0.33], DARK);
        b(o, [-0.13, 0.48, 0.05, -0.1, 0.51, 0.08], DARK); b(o, [0.1, 0.48, 0.05, 0.13, 0.51, 0.08], DARK);
        b(o, [-0.25, 0.54, -0.04, -0.11, 0.6, 0.03], FUR2); b(o, [0.11, 0.54, -0.04, 0.25, 0.6, 0.03], FUR2);
        for sx in [-1.0f64, 1.0] {
            let m = |a: f64, c: f64| (f64::min(sx * a, sx * c), f64::max(sx * a, sx * c));
            let q = m(0.06, 0.1); b(o, [q.0, 0.58, -0.03, q.1, 0.92, 0.01], ANT); let q = m(0.1, 0.24); b(o, [q.0, 0.8, -0.03, q.1, 0.84, 0.01], ANT);
            let q = m(0.2, 0.24); b(o, [q.0, 0.84, -0.03, q.1, 1.04, 0.01], ANT); let q = m(0.08, 0.12); b(o, [q.0, 0.68, 0.01, q.1, 0.72, 0.15], ANT);
        } }
    let mut leg = VBuf::new(300); b(&mut leg, [-0.06, -0.48, -0.07, 0.06, 0.0, 0.07], FUR2); b(&mut leg, [-0.05, -0.62, -0.06, 0.05, -0.48, 0.06], CREAM); b(&mut leg, [-0.055, -0.7, -0.065, 0.055, -0.62, 0.065], DARK);
    let mut bird = VBuf::new(300); { let o = &mut bird; const BLUE: [f64; 3] = [0.22, 0.5, 0.86]; let mut q = |a: [f64; 6], c: [f64; 3]| b(o, a.map(|v| v * BIRD), c);
        q([-0.055, 0.0, -0.1, 0.055, 0.1, 0.09], BLUE); q([-0.056, 0.0, -0.03, 0.056, 0.05, 0.1], [0.95, 0.9, 0.82]);
        q([-0.05, 0.08, 0.06, 0.05, 0.17, 0.15], BLUE); q([-0.015, 0.11, 0.15, 0.015, 0.13, 0.19], [0.92, 0.65, 0.15]);
        q([-0.03, 0.04, -0.22, 0.03, 0.07, -0.1], [0.14, 0.3, 0.6]);
        q([-0.03, 0.14, 0.14, -0.02, 0.15, 0.15], DARK); q([0.02, 0.14, 0.14, 0.03, 0.15, 0.15], DARK);
        q([-0.04, -0.06, 0.0, -0.025, 0.0, 0.015], DARK); q([0.025, -0.06, 0.0, 0.04, 0.0, 0.015], DARK); }
    let mut wing = VBuf::new(300); b(&mut wing, [0.0, 0.0, -0.07 * BIRD, 0.17 * BIRD, 0.015 * BIRD, 0.08 * BIRD], [0.15, 0.36, 0.72]);
    [("deerBody", body), ("deerHead", head), ("deerLeg", leg), ("bird", bird), ("wing", wing)]
}
#[derive(Clone, Copy, PartialEq, Debug)] pub enum DeerState { Graze, Walk, Flee }
#[derive(Clone, Debug)] pub struct Deer { pub p: [f64; 3], pub gy: f64, pub yaw: f64, pub state: DeerState, pub t: f64, pub phase: f64, pub head: f64, pub hoof: f64, pub speed: f64, blocked: u32 }
#[derive(Clone, Copy, PartialEq, Debug)] pub enum BirdState { Fly, Perch, Gone }
#[derive(Clone, Debug)] pub struct Bird { pub p: [f64; 3], pub yaw: f64, pub state: BirdState, pub t: f64, pub v: [f64; 3], pub flap: f64, pub voice: usize, pub sing: f64, pub hop: f64, tg: [f64; 3], land: bool, scan: f64, surf: Option<f64>, stuck: f64 }
/// Ce que la faune fait entendre.
#[derive(Clone, Copy, Debug)] pub enum Ev { Bird([f64; 3], usize), Wings([f64; 3]), Hoof([f64; 3], bool), Alarm([f64; 3]), Call([f64; 3]) }
pub struct Fauna { rng: Rng, pub deer: Vec<Deer>, pub birds: Vec<Bird>, t: f64, day: f64, pub ev: Vec<Ev> }
pub(crate) fn ground(w: &impl Voxels, x: f64, z: f64, y0: f64) -> Option<f64> {
    let (fx, fz, base) = (x.floor() as i32, z.floor() as i32, y0.floor() as i32); let mut y = base + 2;
    while y > base - 22 && y > 1 {
        let b = w.get(fx, y, fz)?; if is_water(b) { return None; }
        if is_solid(b) && BLOCKS[b as usize].shape != Shape::Leaf { return if BLOCKS[b as usize].cat == "wood" { None } else { Some((y + 1) as f64) }; }
        y -= 1;
    }
    None
}
/// Le corps tient-il là ? Pattes, flancs, tête et croupe doivent rester hors des blocs solides.
pub(crate) fn fits(w: &impl Voxels, x: f64, y: f64, z: f64, yaw: f64, h: i32) -> bool {
    let (s, c, yy) = (sin(yaw), cos(yaw), (y + 0.05).floor() as i32);
    for (f, l) in [(0.75, 0.0), (0.5, 0.25), (0.5, -0.25), (0.0, 0.25), (0.0, -0.25), (-0.6, 0.25), (-0.6, -0.25)] {
        let (bx, bz) = ((x + s * f + c * l).floor() as i32, (z + c * f - s * l).floor() as i32);
        for k in 0..h { match w.get(bx, yy + k, bz) { None => return false, Some(b) => if is_solid(b) || is_water(b) { return false; } } }
    }
    true
}
fn turn(d: &mut Deer, w: &impl Voxels, yaw: f64) -> bool { if fits(w, d.p[0], d.gy, d.p[2], yaw, 2) { d.yaw = yaw; true } else { false } }
/// Dessus d'une colonne, feuillage compris ; le booléen dit si un oiseau y resterait visible.
fn top_at(w: &impl Voxels, x: f64, z: f64, y0: f64) -> Option<(f64, bool)> {
    let (fx, fz, base) = (x.floor() as i32, z.floor() as i32, y0.floor() as i32); let (mut deco, mut y) = (false, base + 26);
    while y > base - 26 && y > 1 {
        let b = w.get(fx, y, fz)?;
        if is_water(b) { return Some(((y + 1) as f64, false)); }
        if is_solid(b) { return Some(((y + 1) as f64, !deco)); }
        if b > 0 { deco = true; }
        y -= 1;
    }
    None
}
impl Fauna {
    pub fn new(seed: i32) -> Self { Fauna { rng: Rng::new((seed ^ 0x51ed) as u32), deer: Vec::new(), birds: Vec::new(), t: 0.0, day: 1.0, ev: Vec::new() } }
    fn spawn_spot(&mut self, w: &impl Voxels, p: [f64; 3], a0: f64, a1: f64) -> Option<[f64; 3]> {
        let (a, d) = (self.rng.range(0.0, 6.283), self.rng.range(a0, a1)); let (x, z) = (p[0] + cos(a) * d, p[2] + sin(a) * d);
        ground(w, x, z, p[1]).map(|y| [x, y, z])
    }
    pub fn update(&mut self, dt: f64, w: &impl Voxels, p: [f64; 3], day: f64) {
        self.t += dt; self.day = day;
        if self.deer.len() < MAX_DEER && self.rng.next() < dt * 2.0 {
            let s = self.spawn_spot(w, p, 16.0, 60.0); let yaw = self.rng.range(0.0, 6.28);
            if let Some(s) = s { if fits(w, s[0], s[1], s[2], yaw, 2) { let t = self.rng.range(2.0, 7.0); let phase = self.rng.range(0.0, 6.0); self.deer.push(Deer { p: s, gy: s[1], yaw, state: DeerState::Graze, t, phase, head: 1.0, hoof: 0.0, speed: 0.0, blocked: 0 }); } }
        }
        // Les oiseaux arrivent en vol, en petits groupes, puis se posent à découvert.
        if day > 0.25 && self.birds.len() < MAX_BIRDS - 2 && self.rng.next() < dt * 1.2 { self.flock(w, p); }
        let mut deer = std::mem::take(&mut self.deer); for d in deer.iter_mut() { self.step_deer(d, dt, w, p); }
        deer.retain(|d| hypot2(d.p[0] - p[0], d.p[2] - p[2]) < 92.0 && w.get(d.p[0].floor() as i32, d.p[1].floor() as i32, d.p[2].floor() as i32).is_some()); self.deer = deer;
        let mut birds = std::mem::take(&mut self.birds); for b in birds.iter_mut() { self.step_bird(b, dt, w, p); }
        birds.retain(|b| b.state != BirdState::Gone && hypot2(b.p[0] - p[0], b.p[2] - p[2]) < 82.0); self.birds = birds;
    }
    fn step_deer(&mut self, d: &mut Deer, dt: f64, w: &impl Voxels, p: [f64; 3]) {
        let r = &mut self.rng; let dist = hypot2(d.p[0] - p[0], d.p[2] - p[2]);
        if dist < 8.0 && d.state != DeerState::Flee {
            d.state = DeerState::Flee; d.t = r.range(3.0, 5.0); self.ev.push(Ev::Alarm(d.p)); let a = (d.p[0] - p[0]).atan2(d.p[2] - p[2]);
            for o in [0.0, 0.5, -0.5, 1.0, -1.0, 1.6, -1.6] { if turn(d, w, a + o) { break; } }
        }
        d.t -= dt;
        if d.t <= 0.0 { if d.state == DeerState::Graze { d.state = DeerState::Walk; d.t = r.range(3.0, 8.0); let y = d.yaw + r.range(-1.3, 1.3); turn(d, w, y); } else { d.state = DeerState::Graze; d.t = r.range(3.0, 9.0); } }
        let sp = match d.state { DeerState::Flee => 6.0, DeerState::Walk => 1.3, DeerState::Graze => 0.0 };
        if d.state == DeerState::Graze && r.next() < dt * 0.05 { self.ev.push(Ev::Call(d.p)); }
        if sp != 0.0 { d.hoof -= dt * (2.2 + sp * 1.1); if d.hoof <= 0.0 { d.hoof = 1.0; self.ev.push(Ev::Hoof(d.p, sp > 3.0)); } }
        d.speed = sp; d.head += ((if d.state == DeerState::Graze { 1.0 } else { 0.0 }) - d.head) * (dt * 3.0).min(1.0);
        if sp != 0.0 {
            d.phase += dt * (3.0 + sp * 2.0);
            let (sy, cy) = (sin(d.yaw), cos(d.yaw)); let (nx, nz, base) = (d.p[0] + sy * sp * dt, d.p[2] + cy * sp * dt, d.gy);
            // Le corps se pose sur le sol le plus haut sous ses pattes : il monte et descend un bloc sans entrer dedans.
            let (gc, gf, gr) = (ground(w, nx, nz, base), ground(w, nx + sy * 0.8, nz + cy * 0.8, base), ground(w, nx - sy * 0.65, nz - cy * 0.65, base));
            let y = match (gc, gf, gr) { (Some(a), Some(b), Some(c)) => Some(a.max(b).max(c)), _ => None };
            match y {
                Some(y) if (y - base).abs() <= 1.0 && fits(w, nx, y, nz, d.yaw, 2) => { d.p[0] = nx; d.p[2] = nz; d.gy = y; d.blocked = 0; }
                _ => { d.blocked += 1; for _ in 0..6 { let y = d.yaw + r.range(0.6, 3.2); if turn(d, w, y) { break; } } if d.blocked > 40 { d.state = DeerState::Graze; d.t = r.range(1.0, 3.0); d.blocked = 0; } }
            }
            d.p[1] += (d.gy - d.p[1]) * (dt * 9.0).min(1.0);
        }
    }
    fn flock(&mut self, w: &impl Voxels, p: [f64; 3]) {
        let r = &mut self.rng; let (a, d) = (r.range(0.0, 6.283), r.range(18.0, 36.0)); let (cx, cz) = (p[0] + cos(a) * d, p[2] + sin(a) * d);
        let Some(top) = top_at(w, cx, cz, p[1]) else { return };
        let n = 1 + (r.next() * 3.0).floor() as usize;
        for _ in 0..n {
            let r = &mut self.rng; let pp = [cx + r.range(-2.0, 2.0), top.0 + r.range(7.0, 13.0), cz + r.range(-2.0, 2.0)]; let yaw = r.range(0.0, 6.28); let flap = r.range(0.0, 6.0); let voice = (r.next() * 3.0).floor() as usize; let sing = r.range(1.0, 5.0);
            let mut b = Bird { p: pp, yaw, state: BirdState::Fly, t: 0.0, v: [0.0; 3], flap, voice, sing, hop: 0.0, tg: pp, land: false, scan: 0.0, surf: None, stuck: 0.0 };
            if w.get(pp[0].floor() as i32, (pp[1] + 0.1).floor() as i32, pp[2].floor() as i32) != Some(0) { continue; }
            self.goal(&mut b, w, p); self.birds.push(b);
        }
    }
    /// Cherche un perchoir dégagé, autour du joueur un essai sur deux ; à défaut, l'oiseau poursuit son vol.
    fn goal(&mut self, b: &mut Bird, w: &impl Voxels, p: [f64; 3]) {
        let r = &mut self.rng;
        for i in 0..12 {
            let c = if i % 2 != 0 { b.p } else { p }; let (a, d) = (r.range(0.0, 6.283), r.range(8.0, 24.0)); let (x, z) = ((c[0] + cos(a) * d).floor() + 0.5, (c[2] + sin(a) * d).floor() + 0.5);
            if let Some((y, clear)) = top_at(w, x, z, b.p[1]) { if clear && hypot2(x - p[0], z - p[2]) > 7.0 && (y < p[1] + 9.0 || i > 9) { b.tg = [x, y, z]; b.land = true; return; } }
        }
        let a = r.range(0.0, 6.283); let top = top_at(w, b.p[0], b.p[2], b.p[1]);
        b.tg = [b.p[0] + cos(a) * 24.0, top.map_or(b.p[1], |t| t.0) + 8.0, b.p[2] + sin(a) * 24.0]; b.land = false;
    }
    fn step_bird(&mut self, b: &mut Bird, dt: f64, w: &impl Voxels, p: [f64; 3]) {
        let dist = hypot2(b.p[0] - p[0], b.p[2] - p[2]);
        if b.state == BirdState::Perch {
            let r = &mut self.rng; b.t -= dt; b.sing -= dt; b.hop = (b.hop - dt * 5.0).max(0.0);
            if b.sing <= 0.0 { b.sing = r.range(3.0, 9.0); if self.day > 0.25 { self.ev.push(Ev::Bird(b.p, b.voice)); } b.hop = 0.6; }
            if r.next() < dt * 0.5 { b.yaw += r.range(-1.5, 1.5); b.hop = 1.0; }
            if dist < 6.0 || (b.t <= 0.0 && self.day > 0.25) { b.state = BirdState::Fly; self.ev.push(Ev::Wings(b.p)); self.goal(b, w, p); b.v[1] = 3.5; }
            return;
        }
        b.flap += dt * 30.0; b.scan -= dt;
        if b.scan <= 0.0 { b.scan = 0.25; let top = top_at(w, b.p[0], b.p[2], b.p[1]); b.surf = Some(match top { Some(t) => t.0, None => b.surf.unwrap_or(b.p[1]) }); }
        let (dx, dz) = (b.tg[0] - b.p[0], b.tg[2] - b.p[2]); let h = hypot2(dx, dz); let sp = (1.6 + h * 1.4).min(6.5); let cruise = (b.surf.unwrap() + 3.5).max(b.tg[1] + if h > 7.0 { 3.5 } else { 0.0 });
        let k = (dt * 3.2).min(1.0); let want = if h < 2.5 { b.tg[1] + 0.1 - b.p[1] } else { cruise - b.p[1] }; let hh = if h != 0.0 { h } else { 1.0 };
        b.v[0] += (dx / hh * sp - b.v[0]) * k; b.v[2] += (dz / hh * sp - b.v[2]) * k; b.v[1] += ((want * 1.8).min(4.5).max(-4.0) - b.v[1]) * k;
        let (nx, ny, nz) = (b.p[0] + b.v[0] * dt, b.p[1] + b.v[1] * dt, b.p[2] + b.v[2] * dt); let solid = |c: Option<u8>| c.is_some_and(|c| c > 0 && is_solid(c));
        // Un obstacle devant : l'oiseau monte au lieu de traverser.
        if solid(w.get(nx.floor() as i32, (ny + 0.1).floor() as i32, nz.floor() as i32)) {
            if solid(w.get(b.p[0].floor() as i32, (b.p[1] + 0.1 + 5.0 * dt).floor() as i32, b.p[2].floor() as i32)) { b.v[0] *= -1.0; b.v[2] *= -1.0; b.surf = Some(b.p[1]); } else { b.v[1] = 5.0; b.p[1] += 5.0 * dt; }
            b.stuck += dt; if b.stuck > 2.5 { b.state = BirdState::Gone; } return;
        }
        b.stuck = 0.0; b.p = [nx, ny, nz]; if h > 0.3 { b.yaw = b.v[0].atan2(b.v[2]); }
        if h < 0.5 && (b.tg[1] - b.p[1]).abs() < 0.6 {
            if b.land && w.get(b.tg[0].floor() as i32, b.tg[1].floor() as i32, b.tg[2].floor() as i32) == Some(0) { b.p = b.tg; b.state = BirdState::Perch; b.t = self.rng.range(5.0, 16.0); b.v = [0.0; 3]; return; }
            self.goal(b, w, p);
        }
    }
    /// Ce que le rendu doit dessiner : une matrice par morceau.
    pub fn draws(&self, md: &Models) -> Vec<Draw> {
        let mut out = Vec::with_capacity(self.deer.len() * 6 + self.birds.len() * 3);
        let mut put = |n: &str, m: M4| out.push(Draw { m, r: md.part(n), view: false, local: false, hero: false, dynamic: false });
        for d in &self.deer {
            let base = trs(d.p[0], d.p[1], d.p[2], d.yaw, 0.0, 1.0); let s = if d.speed != 0.0 { sin(d.phase) * 0.75 } else { 0.0 }; const HIP: f64 = 0.72;
            put("deerBody", mul(&base, &trs(0.0, HIP, 0.0, 0.0, 0.0, 1.0))); put("deerHead", mul(&base, &trs(0.0, HIP + 0.28, 0.45, 0.0, 0.42 + 1.05 * d.head, 1.0)));
            for (x, z, a) in [(-0.13, 0.44, s), (0.13, 0.44, -s), (-0.13, -0.44, -s), (0.13, -0.44, s)] { put("deerLeg", mul(&base, &trs(x, HIP, z, 0.0, a, 1.0))); }
        }
        for b in &self.birds {
            let base = trs(b.p[0], b.p[1] + 0.1 + b.hop * 0.14, b.p[2], b.yaw, 0.0, 1.0); let f = if b.state == BirdState::Fly { sin(b.flap) } else { -1.25 };
            put("bird", base); put("wing", mul(&base, &mul(&trs(0.055 * BIRD, 0.08 * BIRD, 0.0, 0.0, 0.0, 1.0), &roll_z(f)))); put("wing", mul(&base, &mul(&trs(-0.055 * BIRD, 0.08 * BIRD, 0.0, 0.0, 0.0, 1.0), &roll_z(std::f64::consts::PI - f))));
        }
        out
    }
}
