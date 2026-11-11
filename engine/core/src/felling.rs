//! Abattage : trouve l'arbre visé, puis fait tomber le tronc en un seul morceau pendant que le feuillage disparaît.
use crate::actors::{cube_at, Draw, Models, Range};
use crate::blocks::*;
use crate::det::hypot2;
use crate::mesher::VBuf;
use crate::noise::Rng;
use crate::player::Voxels;
use crate::view::{mul, rot_axis, trs, M4};
use std::collections::{HashSet, VecDeque};
pub const FELL_TIME: f64 = 10.0; pub const MAX_LOGS: usize = 900; const REACH: i32 = 11; const MAX_LEAVES: usize = 2500; const LEAF_DIST: u8 = 6; const FLUTTER: usize = 24;
type P = (i32, i32, i32);
pub struct Tree { pub logs: Vec<P>, pub leaves: Vec<P> }
/// Un arbre, c'est des troncs de même essence reliés entre eux et portant du feuillage : un mur de bûches n'en est pas un.
pub fn find_tree(w: &impl Voxels, x: i32, y: i32, z: i32) -> Option<Tree> {
    let b0 = w.get(x, y, z)?; if !is_log(b0) && !is_branch(b0) { return None; } let sp = species(b0);
    let (mut seen, mut q) = (HashSet::from([(x, y, z)]), VecDeque::from([(x, y, z)]));
    while let Some(p) = q.pop_front() {
        for dy in -1..=1 { for dz in -1..=1 { for dx in -1..=1 {
            let n = (p.0 + dx, p.1 + dy, p.2 + dz); if (n.0 - x).abs() > REACH || (n.2 - z).abs() > REACH || seen.contains(&n) || seen.len() >= MAX_LOGS { continue; }
            if !w.get(n.0, n.1, n.2).is_some_and(|b| (is_log(b) || is_branch(b)) && species(b) == sp) { continue; }
            // Un arbre voisin collé au nôtre n'est pas emporté : au-delà du tronc, un bois qui a encore un tronc sous lui, à hauteur de l'entaille, appartient à un autre arbre.
            let far = (n.0 - x).abs().max((n.2 - z).abs()) > 3; if far && w.get(n.0, y.min(n.1), n.2).is_some_and(|b| is_log(b) && species(b) == sp) && w.get(n.0, y.min(n.1) - 1, n.2).is_some_and(|b| is_log(b) || is_opaque(b)) { continue; }
            seen.insert(n); q.push_back(n);
        } } }
    }
    let logs: Vec<P> = { let mut v: Vec<P> = seen.into_iter().collect(); v.sort(); v };
    let (mut done, mut lq): (HashSet<P>, VecDeque<(P, u8)>) = (HashSet::new(), logs.iter().map(|p| (*p, 0)).collect());
    while let Some((p, d)) = lq.pop_front() {
        if d >= LEAF_DIST { continue; }
        for dy in -1..=1 { for dz in -1..=1 { for dx in -1..=1 {
            let n = (p.0 + dx, p.1 + dy, p.2 + dz); if done.contains(&n) { continue; }
            if done.len() < MAX_LEAVES && w.get(n.0, n.1, n.2).is_some_and(|b| is_leaf(b) && species(b) == sp) { done.insert(n); lq.push_back((n, d + 1)); }
        } } }
    }
    if done.len() < 6 && !(sp == "dead" && logs.len() >= 4) { return None; } // un arbre mort n'a plus de feuillage
    let mut leaves: Vec<P> = done.into_iter().collect(); leaves.sort(); Some(Tree { logs, leaves })
}
struct Flutter { p: [f64; 3], v: [f64; 3], spin: f64, life: f64 }
/// Le tronc en train de tomber : un seul maillage, tournant autour de sa base.
pub struct Fall { pub data: Vec<u8>, pub n: usize, pivot: [f64; 3], axis: [f64; 3], dir: [f64; 3], reach: f64, t: f64, angle: f64, speed: f64, rest: f64, flutter: Vec<Flutter>, leaf: &'static str, pub landed: bool }
const GRAVITY_TURN: f64 = 1.25; const REST: f64 = 0.9; const FADE: f64 = 0.7;
impl Fall {
    pub fn new(logs: &[(P, crate::blocks::Id)], leaves: &[P], leaf_id: crate::blocks::Id, cut: P, from: [f64; 3], seed: u32) -> Self {
        let pivot = [cut.0 as f64 + 0.5, cut.1 as f64, cut.2 as f64 + 0.5]; let mut o = VBuf::new(logs.len() * 24 + 24); let mut top = 1.0f64;
        for ((x, y, z), id) in logs { cube_at(&mut o, *id, [*x as f64 - pivot[0], *y as f64 - pivot[1], *z as f64 - pivot[2]]); top = top.max((*y - cut.1 + 1) as f64); }
        let (dx, dz) = (pivot[0] - from[0], pivot[2] - from[2]); let l = hypot2(dx, dz); let dir = if l > 0.01 { [dx / l, 0.0, dz / l] } else { [1.0, 0.0, 0.0] };
        let mut r = Rng::new(seed); let step = (leaves.len() / FLUTTER).max(1);
        let flutter = leaves.iter().step_by(step).take(FLUTTER).map(|p| Flutter { p: [p.0 as f64 + 0.5, p.1 as f64 + 0.5, p.2 as f64 + 0.5], v: [r.range(-1.0, 1.0), r.range(-0.5, 1.0), r.range(-1.0, 1.0)], spin: r.range(-3.0, 3.0), life: r.range(0.9, 1.6) }).collect();
        let n = o.n; let leaf = BLOCKS[leaf_id as usize].name;
        Fall { data: o.data, n, pivot, axis: [dir[2], 0.0, -dir[0]], dir, reach: top, t: 0.0, angle: 0.0, speed: 0.0, rest: 0.0, flutter, leaf, landed: false }
    }
    /// Rend vrai au moment où le tronc touche le sol, et l'objet disparaît quand `done` passe à vrai.
    pub fn step(&mut self, dt: f64, w: &impl Voxels) -> bool {
        self.t += dt; let mut hit = false;
        for f in &mut self.flutter { f.life -= dt; f.v[1] -= 5.0 * dt; for k in 0..3 { f.p[k] += f.v[k] * dt; } }
        if self.landed { self.rest += dt; return false; }
        self.speed += GRAVITY_TURN * dt * (0.15 + crate::det::sin(self.angle)).max(0.15) * 1.6; self.angle += self.speed * dt;
        let (s, c) = (crate::det::sin(self.angle), crate::det::cos(self.angle));
        // On suit trois points du tronc : le premier qui rencontre un bloc solide arrête la chute.
        let blocked = [0.55, 0.8, 1.0].iter().any(|k| { let d = self.reach * k; let p = [self.pivot[0] + self.dir[0] * d * s, self.pivot[1] + d * c, self.pivot[2] + self.dir[2] * d * s]; w.get(p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32).is_some_and(is_solid) });
        if self.angle >= 1.53 || (blocked && self.angle > 0.35) { self.landed = true; hit = true; }
        hit
    }
    pub fn done(&self) -> bool { self.rest >= REST + FADE }
    pub fn draws(&self, md: &Models) -> Vec<Draw> {
        let sc = 1.0 - ((self.rest - REST) / FADE).clamp(0.0, 1.0); let mut o = Vec::with_capacity(self.flutter.len() + 1);
        if sc > 0.0 { let mid = self.reach * 0.5; let (s, c) = (crate::det::sin(self.angle), crate::det::cos(self.angle)); let m = [self.dir[0] * s * mid, c * mid, self.dir[2] * s * mid]; // disparaît vers le milieu du tronc
            let p = [self.pivot[0] + m[0] * (1.0 - sc), self.pivot[1] + m[1] * (1.0 - sc), self.pivot[2] + m[2] * (1.0 - sc)];
            o.push(Draw { m: rot_axis(p, self.axis, self.angle, sc), r: Range { first: 0, count: (self.n / 4 * 6) as u32 }, view: false, local: true, hero: false, dynamic: true }); }
        if let Some(h) = md.held(self.leaf) { for f in &self.flutter { if f.life > 0.0 {
            let k = (f.life / 0.9).min(1.0) * 0.55; let m: M4 = mul(&trs(f.p[0], f.p[1], f.p[2], f.spin * self.t, f.spin * 0.6 * self.t, k), &trs(-0.5, -0.5, -0.5, 0.0, 0.0, 1.0));
            o.push(Draw { m, r: h.r, view: false, local: true, hero: false, dynamic: false });
        } } }
        o
    }
}
