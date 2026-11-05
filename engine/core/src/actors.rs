//! Personnage, outils et objets tenus : modèles construits par code, animation procédurale.
use crate::blocks::*;
use crate::det::{cos, round, sin};
use crate::mesher::{bx, kind, Mesher, VBuf, VSIZE};
use crate::noise::hash_float;
use crate::session::{item, ORDER};
use crate::view::{invert, mul, roll_z, trs, M4};
use std::collections::HashMap;

type C = [f64; 3];
const SKIN: C = [0.93, 0.7, 0.53]; const HAIR: C = [0.27, 0.15, 0.08]; const HAIR2: C = [0.36, 0.2, 0.1]; const SHIRT: C = [0.88, 0.82, 0.68]; const SHIRT2: C = [0.78, 0.72, 0.58]; const PANTS: C = [0.37, 0.3, 0.24]; const BOOT: C = [0.3, 0.17, 0.09]; const CUFF: C = [0.62, 0.58, 0.5];
const BAG: C = [0.46, 0.28, 0.14]; const BAG2: C = [0.58, 0.38, 0.2]; const ROLL: C = [0.8, 0.68, 0.46]; const STRAP: C = [0.32, 0.19, 0.1]; const GLOVE: C = [0.42, 0.24, 0.12];
pub(crate) fn b(o: &mut VBuf, a: [f64; 6], c: C) { bx(o, a[0], a[1], a[2], a[3], a[4], a[5], c[0], c[1], c[2], 0.0, 255.0, 0.0, kind::FLAT, true, 0.0, 255.0); }

#[derive(Clone, Copy, Default, Debug)] pub struct Range { pub first: u32, pub count: u32 }
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Hold { Tool, Block, Deco }
#[derive(Clone, Copy, Debug)] pub struct Held { pub r: Range, pub hold: Hold, pub local: bool }
/// Tous les modèles dans un seul tampon de sommets.
pub struct Models { pub data: Vec<u8>, pub n: usize, pub char_verts: usize, parts: HashMap<&'static str, Range>, items: HashMap<&'static str, Held> }
fn take(all: &mut VBuf, o: VBuf) -> Range { let st = all.n; all.data.extend_from_slice(&o.data); all.n += o.n; Range { first: (st / 4 * 6) as u32, count: (o.n / 4 * 6) as u32 } }
fn torso() -> VBuf {
    let mut o = VBuf::new(600);
    b(&mut o, [-0.25, 0.0, -0.15, 0.25, 0.62, 0.15], SHIRT); b(&mut o, [-0.26, 0.0, -0.16, 0.26, 0.1, 0.16], SHIRT2);
    b(&mut o, [-0.2, 0.05, -0.34, 0.2, 0.56, -0.15], BAG); b(&mut o, [-0.15, 0.1, -0.38, 0.15, 0.3, -0.34], BAG2);
    b(&mut o, [-0.04, 0.32, -0.39, 0.04, 0.4, -0.33], STRAP);
    b(&mut o, [-0.24, 0.1, -0.16, -0.16, 0.62, 0.16], STRAP); b(&mut o, [0.16, 0.1, -0.16, 0.24, 0.62, 0.16], STRAP);
    for i in 0..6 { let a = (i as f64 / 6.0) * 6.283; let (y, z) = (0.72 + cos(a) * 0.12, -0.24 + sin(a) * 0.12); b(&mut o, [-0.3, y - 0.07, z - 0.07, 0.3, y + 0.07, z + 0.07], if i % 2 != 0 { ROLL } else { [0.72, 0.6, 0.4] }); }
    o
}
fn head() -> VBuf {
    let mut o = VBuf::new(600);
    b(&mut o, [-0.24, 0.0, -0.24, 0.24, 0.46, 0.24], SKIN);
    b(&mut o, [-0.15, 0.16, 0.24, -0.05, 0.26, 0.25], [0.1, 0.07, 0.05]); b(&mut o, [0.05, 0.16, 0.24, 0.15, 0.26, 0.25], [0.1, 0.07, 0.05]);
    for x in -3i32..=3 { for z in -3i32..=3 { for y in 0..3i32 {
        if z > 1 && y < 2 && x.abs() < 3 { continue; }
        let h = hash_float(x, y, z, 5); let e = if y == 0 { 0.3 } else { 0.2 }; if y == 2 && h < 0.3 { continue; }
        let (xf, zf, yf, s) = (x as f64 * 0.085, z as f64 * 0.085, if y == 0 { 0.26 } else { 0.4 + (y - 1) as f64 * 0.1 }, 0.075 + 0.03 * h);
        if y == 0 && x.abs() < 3 && z.abs() < 3 { continue; }
        b(&mut o, [xf - s, yf - e * 0.5, zf - s, xf + s, yf + 0.1 + 0.12 * h, zf + s], if h > 0.5 { HAIR } else { HAIR2 });
    } } }
    o
}
fn leg() -> VBuf { let mut o = VBuf::new(100); b(&mut o, [-0.11, -0.46, -0.12, 0.11, 0.0, 0.12], PANTS); b(&mut o, [-0.13, -0.55, -0.14, 0.13, -0.46, 0.14], CUFF); b(&mut o, [-0.13, -0.78, -0.14, 0.13, -0.55, 0.2], BOOT); o }
fn arm() -> VBuf { let mut o = VBuf::new(100); b(&mut o, [-0.09, -0.26, -0.1, 0.09, 0.0, 0.1], SHIRT); b(&mut o, [-0.1, -0.32, -0.11, 0.1, -0.26, 0.11], SHIRT2); b(&mut o, [-0.085, -0.56, -0.09, 0.085, -0.32, 0.09], SKIN); b(&mut o, [-0.1, -0.66, -0.11, 0.1, -0.54, 0.11], GLOVE); o }
fn fist() -> VBuf { let mut o = VBuf::new(100); b(&mut o, [-0.085, -0.34, -0.085, 0.085, 0.0, 0.085], SHIRT); b(&mut o, [-0.09, -0.38, -0.09, 0.09, -0.33, 0.09], SHIRT2); b(&mut o, [-0.08, -0.66, -0.08, 0.08, -0.38, 0.08], SKIN); o }

// Outils : image 56 × 56 décrite par des formes, puis extrudée en petits voxels. Couleurs : indices de PAL.
const PAL: [C; 12] = [[0.3, 0.17, 0.08], [0.48, 0.29, 0.14], [0.66, 0.43, 0.22], [0.2, 0.11, 0.06], [0.36, 0.2, 0.1], [0.4, 0.39, 0.38], [0.62, 0.61, 0.6], [0.84, 0.83, 0.82], [1.0, 0.99, 0.97], [0.62, 0.42, 0.1], [0.92, 0.7, 0.2], [0.0; 3]];
const WOOD: [u8; 3] = [0, 1, 2]; const LEATHER: [u8; 2] = [3, 4]; const STEEL: [u8; 4] = [5, 6, 7, 8]; const GOLD: [u8; 2] = [9, 10];
const R2: f64 = std::f64::consts::FRAC_1_SQRT_2;
fn handle(s: f64, p: f64, a: f64, e: f64) -> Option<u8> {
    if s < a || s > e || p.abs() > 0.042 { return None; }
    if s < a + 0.26 { return Some(if (s * 40.0).floor() as i64 & 1 != 0 { LEATHER[0] } else { LEATHER[1] }); }
    Some(if p.abs() > 0.024 { WOOD[0] } else if p > 0.0 { WOOD[2] } else { WOOD[1] })
}
fn shape(name: &str, x: f64, y: f64) -> Option<u8> {
    match name {
        "pickaxe" => {
            let (dx, dy) = (x - 0.34, y - 0.34); let r = crate::det::hypot2(dx, dy); let t = (dy.atan2(dx) - std::f64::consts::FRAC_PI_4) / 1.02;
            if t.abs() < 1.0 { let w = 0.018 + 0.066 * (1.0 - t * t); let d = r - 0.62; if d.abs() < w { return Some(if d > w * 0.45 { if t.abs() > 0.6 { STEEL[3] } else { STEEL[2] } } else if d < -w * 0.45 { STEEL[0] } else { STEEL[1] }); } }
            let (s, p) = ((x + y) * R2, (y - x) * R2);
            if s > 0.86 && s < 1.08 && p.abs() < 0.062 { return Some(STEEL[0]); }
            handle(s, p, 0.08, 0.9)
        }
        "axe" => {
            let (s, p) = ((x + y) * R2, (x - y) * R2);
            if p > 0.0 && p < 0.36 { let half = 0.075 + 0.2 * (p / 0.36).powf(1.5); let c = 0.88; if (s - c).abs() < half { let e = 0.36 - p; return Some(if e < 0.035 { STEEL[3] } else if e < 0.075 { STEEL[2] } else if s - c > 0.02 { STEEL[1] } else { STEEL[0] }); } }
            if p <= 0.0 && p > -0.11 && (s - 0.88).abs() < 0.065 { return Some(if p < -0.08 { STEEL[2] } else { STEEL[1] }); }
            handle(s, p, 0.08, 1.03)
        }
        _ => {
            let (s, p) = ((x + y) * R2, (y - x) * R2);
            if s > 0.47 && s < 1.32 { let w = if s > 1.14 { 0.058 * (1.32 - s) / 0.18 } else { 0.058 }; if p.abs() < w { return Some(if p.abs() < 0.01 { STEEL[0] } else if p > 0.0 { STEEL[3] } else { STEEL[1] }); } }
            if (s - 0.44).abs() < 0.034 && p.abs() < 0.18 { return Some(if p.abs() > 0.14 { GOLD[1] } else { GOLD[0] }); }
            if s > 0.08 && s < 0.41 && p.abs() < 0.034 { return Some(if (s * 44.0).floor() as i64 & 1 != 0 { LEATHER[0] } else { LEATHER[1] }); }
            if crate::det::hypot2(s - 0.05, p) < 0.05 { return Some(GOLD[1]); }
            None
        }
    }
}
fn tool_mesh(name: &str) -> VBuf {
    let mut o = VBuf::new(8000); const N: usize = 56; let s = 0.86; let px = s / N as f64; let grip = 0.2 * R2 * s;
    for j in 0..N {
        let mut run: Option<(usize, u8)> = None;
        for i in 0..=N {
            let c = if i < N { shape(name, (i as f64 + 0.5) / N as f64, (j as f64 + 0.5) / N as f64) } else { None };
            if let Some((a, k)) = run { if c != Some(k) {
                let th = if k == STEEL[3] { 0.012 } else if STEEL.contains(&k) { 0.022 } else { 0.03 }; let col = PAL[k as usize];
                bx(&mut o, a as f64 * px - grip, j as f64 * px - grip, -th, i as f64 * px - grip, (j + 1) as f64 * px - grip, th, col[0], col[1], col[2], 0.0, 255.0, 0.0, kind::FLAT, true, 0.0, 255.0);
                run = None;
            } }
            if let (Some(k), None) = (c, run) { run = Some((i, k)); }
        }
    }
    o
}
fn cube_mesh(id: u8) -> VBuf { let mut o = VBuf::new(32); cube_at(&mut o, id, [0.0; 3]); o }
/// Un bloc complet, posé à `at` : le mode « tenu » lit la texture dans ces coordonnées, sans dépendre du monde.
pub fn cube_at(o: &mut VBuf, id: u8, at: [f64; 3]) {
    let bk = BLOCKS[id as usize];
    if id == B::WATER { bx(o, 0.0, 0.0, 0.0, 1.0, 0.9, 1.0, 0.3, 0.55, 0.75, 0.0, 255.0, 0.0, kind::FLAT, true, 0.0, 255.0); return; }
    let kd = if bk.shape == Shape::Leaf { kind::LEAF } else { kind::TEX }; let t = bk.tint;
    let mut q = |di: usize, pts: [[f64; 3]; 4], n: [f64; 3]| { o.rot = bk.grain[di]; for p in pts { o.v(p[0] + at[0], p[1] + at[1], p[2] + at[2], bk.dir_faces[di] as f64, n[0], n[1], n[2], 0.0, 255.0, 255.0, 0.0, kd, t[0], t[1], t[2]); } o.rot = 0; };
    q(1, [[0., 1., 1.], [1., 1., 1.], [1., 1., 0.], [0., 1., 0.]], [0., 1., 0.]); q(2, [[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]], [0., -1., 0.]);
    q(3, [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]], [0., 0., 1.]); q(3, [[1., 0., 0.], [0., 0., 0.], [0., 1., 0.], [1., 1., 0.]], [0., 0., -1.]);
    q(0, [[1., 0., 1.], [1., 0., 0.], [1., 1., 0.], [1., 1., 1.]], [1., 0., 0.]); q(0, [[0., 0., 0.], [0., 0., 1.], [0., 1., 1.], [0., 1., 0.]], [-1., 0., 0.]);
}
impl Models {
    pub fn build() -> Self {
        let mut all = VBuf::new(5000); let mut parts = HashMap::new();
        parts.insert("torso", take(&mut all, torso())); parts.insert("head", take(&mut all, head())); let l = take(&mut all, leg()); parts.insert("legL", l); parts.insert("legR", l);
        parts.insert("armL", take(&mut all, arm())); parts.insert("armR", take(&mut all, arm())); parts.insert("fist", take(&mut all, fist()));
        let char_verts = all.n; let mut items = HashMap::new(); let mv = Mesher::new();
        for name in ORDER {
            let it = item(name).unwrap(); let (o, hold) = match it.block {
                None => (tool_mesh(name), Hold::Tool),
                // Dans le monde, le polypore se dessine contre son arbre : tenu ou en icône, il n'a pas de support.
                Some(id) if id == B::SHELF_FUNGUS => { let mut o = VBuf::new(60); for (y, w, t) in [(0.12, 0.42, 1.0), (0.38, 0.33, 1.08), (0.62, 0.24, 0.94)] { b(&mut o, [0.5 - w, y, 0.5 - w, 0.5 + w, y + 0.09, 0.5 + w], [0.82 * t, 0.68 * t, 0.5 * t]); } (o, Hold::Deco) }
                Some(id) if BLOCKS[id as usize].shape == Shape::Deco => { let mut o = VBuf::new(800); mv.deco_model(&mut o, id, 0, 0, 0, 7, 11, 255.0, if id == B::LANTERN { 255.0 } else { 0.0 }, 0); (o, Hold::Deco) }
                Some(id) => (cube_mesh(id), Hold::Block),
            };
            let local = hold == Hold::Block && it.block != Some(B::WATER);
            items.insert(it.id, Held { r: take(&mut all, o), hold, local });
        }
        for (name, o) in crate::fauna::parts() { parts.insert(name, take(&mut all, o)); }
        let n = all.n; Models { data: all.data, n, char_verts, parts, items }
    }
    pub fn held(&self, id: &str) -> Option<Held> { self.items.get(id).copied() }
    pub fn bytes(&self) -> usize { self.n * VSIZE }
    pub fn part(&self, n: &str) -> Range { self.parts[n] }
    /// Cadrage d'une icône : vue de trois quarts pour les blocs, de face pour les outils, l'objet remplit sa case.
    pub fn icon_matrix(&self, id: &str) -> M4 {
        let h = self.items[id]; let (yaw, tilt): (f64, f64) = match h.hold { Hold::Tool => (0.0, 0.0), Hold::Block => (std::f64::consts::FRAC_PI_4, 0.6155), Hold::Deco => (0.6, 0.3) };
        let (sy, cy, st, ct) = (yaw.sin(), yaw.cos(), tilt.sin(), tilt.cos()); let r = [[cy, 0.0, -sy], [-sy * st, ct, -cy * st], [sy * ct, st, cy * ct]];
        let (a, b) = (h.r.first as usize / 6 * 4, (h.r.first + h.r.count) as usize / 6 * 4); let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
        for v in self.data[a * VSIZE..b * VSIZE].chunks_exact(VSIZE) {
            let p = [0, 2, 4].map(|k| i16::from_le_bytes([v[k], v[k + 1]]) as f64 / 128.0);
            for i in 0..3 { let q = r[i][0] * p[0] + r[i][1] * p[1] + r[i][2] * p[2]; lo[i] = lo[i].min(q); hi[i] = hi[i].max(q); }
        }
        let s = 1.84 / (hi[0] - lo[0]).max(hi[1] - lo[1]).max(0.01); let (cx, cy2) = ((lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5); let dz = 1.0 / (hi[2] - lo[2]).max(0.01);
        // Colonnes : x et y suivent la vue, la profondeur va de 0 (devant) à 1.
        [(s * r[0][0]) as f32, (s * r[1][0]) as f32, (-dz * r[2][0]) as f32, 0.0, (s * r[0][1]) as f32, (s * r[1][1]) as f32, (-dz * r[2][1]) as f32, 0.0, (s * r[0][2]) as f32, (s * r[1][2]) as f32, (-dz * r[2][2]) as f32, 0.0, (-s * cx) as f32, (-s * cy2) as f32, (dz * hi[2]) as f32, 1.0]
    }
}
/// Un morceau à dessiner : matrice, plage d'indices, et la façon de le traiter.
#[derive(Clone, Copy, Debug)] pub struct Draw { pub m: M4, pub r: Range, pub view: bool, pub local: bool, pub hero: bool, pub dynamic: bool }
fn hold_m(h: Hold) -> M4 { match h { Hold::Tool => mul(&trs(0.0, -0.58, 0.02, -std::f64::consts::FRAC_PI_2, 0.0, 1.0), &roll_z(-std::f64::consts::FRAC_PI_4)), Hold::Block => trs(-0.14, -0.76, -0.14, 0.0, 0.0, 0.28), Hold::Deco => trs(-0.28, -0.72, -0.1, 0.0, 0.0, 0.55) } }
#[derive(Default, Clone, Copy)] pub struct AnimIn { pub speed: f64, pub ground: bool, pub vy: f64, pub crouch: bool, pub fly: bool, pub mining: bool }
#[derive(Default)] pub struct Anim { phase: f64, walk: f64, run: f64, low: f64, air: f64, rise: f64, pub swing: f64, punch: f64, landed: f64 }
impl Anim {
    pub fn update(&mut self, dt: f64, s: AnimIn) {
        let ease = |c: f64, t: f64, k: f64| c + (t - c) * (dt * k).min(1.0); let was = self.air;
        self.walk = ease(self.walk, (s.speed / 4.5).min(1.0), 10.0); self.run = ease(self.run, if s.speed > 5.2 { 1.0 } else { 0.0 }, 8.0); self.low = ease(self.low, if s.crouch { 1.0 } else { 0.0 }, 13.0);
        self.air = ease(self.air, if s.ground || s.fly { 0.0 } else { 1.0 }, 12.0); self.rise = ease(self.rise, if s.vy > 0.0 { 1.0 } else { 0.0 }, 12.0);
        if was > 0.5 && self.air <= 0.5 { self.landed = 1.0; }
        self.landed = (self.landed - dt * 5.0).max(0.0);
        if s.speed > 0.2 { self.phase += dt * (3.4 + s.speed * 1.35); }
        if s.mining { self.swing = (self.swing + dt * 3.3) % 1.0; } else if self.swing > 0.0 { self.swing += dt * 3.3; if self.swing >= 1.0 { self.swing = 0.0; } }
        self.punch = (self.punch - dt * 4.6).max(0.0);
    }
    pub fn trigger(&mut self) { self.punch = 1.0; }
    pub fn third(&self, md: &Models, p: [f64; 3], yaw: f64, pitch: f64, held: Option<Held>) -> Vec<Draw> {
        let (w, s, ground, a) = (self.walk, sin(self.phase), 1.0 - self.air, self.air); let pi = std::f64::consts::PI;
        let leg = s * (0.72 + 0.42 * self.run) * w * (1.0 - 0.45 * self.low) * ground; let armv = s * (0.55 + 0.6 * self.run) * w * ground;
        let hip = 0.8 + cos(self.phase).abs() * 0.045 * w * ground - 0.24 * self.low - 0.1 * self.landed + 0.03 * a;
        let lean = 0.05 * w + 0.24 * self.run + 0.44 * self.low + 0.14 * self.landed - 0.08 * a * self.rise;
        let leg_l = -leg - 0.58 * self.low - (0.62 * self.rise + 0.25) * a; let leg_r = leg - 0.58 * self.low + (0.3 * self.rise - 0.1) * a;
        let arm_l = armv + 0.1 - 0.95 * a * (0.5 + 0.5 * self.rise) - 0.3 * self.low;
        let swing_a = if self.swing > 0.0 { -1.5 + cos(self.swing * 6.283) * 0.95 } else { 0.0 }; let punch_a = -sin(self.punch * pi) * 1.05;
        let arm_r = (if self.swing > 0.0 { swing_a } else { -0.5 - armv * 0.35 - 0.7 * a }) + punch_a;
        let base = trs(p[0], p[1], p[2], yaw, 0.0, 1.0); let t = mul(&base, &trs(0.0, hip, 0.0, 0.0, lean, 1.0)); let mut out = Vec::with_capacity(8);
        let part = |n: &str| md.parts[n]; let mut put = |n: &str, m: M4| out.push(Draw { m, r: part(n), view: false, local: false, hero: true, dynamic: false });
        put("torso", t); put("head", mul(&t, &trs(0.0, 0.64, 0.0, 0.0, -lean * 0.75 - pitch * 0.5, 1.0)));
        put("legL", mul(&base, &trs(0.13, hip, 0.0, 0.0, leg_l, 1.0))); put("legR", mul(&base, &trs(-0.13, hip, 0.0, 0.0, leg_r, 1.0)));
        let ar = mul(&t, &trs(-0.34, 0.58, 0.02, 0.0, arm_r, 1.0)); put("armL", mul(&t, &trs(0.34, 0.58, 0.0, 0.0, arm_l, 1.0))); put("armR", ar);
        if let Some(h) = held { out.push(Draw { m: mul(&ar, &hold_m(h.hold)), r: h.r, view: false, local: h.local, hero: true, dynamic: false }); }
        out
    }
    /// Première personne : poing si la main est vide, bloc seul s'il y en a un, bras et outil sinon.
    pub fn first(&self, md: &Models, view: &M4, held: Option<Held>) -> Vec<Draw> {
        let s = sin(self.phase); let c = cos(self.phase * 0.5); let w = self.walk * (1.0 - self.air); let pi = std::f64::consts::PI; let v = mul(&invert(view), &trs(0.0, 0.0, 0.0, pi, 0.0, 1.0));
        let strike = if self.swing > 0.0 { sin(self.swing * pi) } else { 0.0 }; let push = sin(self.punch * pi); let r = md.parts["fist"]; let mut out = Vec::new();
        let bx_ = c * 0.022 * w - 0.05 * self.run; let by = -s.abs() * 0.022 * w - 0.05 * self.low + 0.07 * self.air * (self.rise - 0.5);
        let hand = |m: M4| Draw { m, r, view: true, local: false, hero: true, dynamic: false };
        match held {
            None => { let hit = strike.max(push); out.push(hand(mul(&v, &trs(-0.27 + bx_ + 0.08 * hit, -0.33 + by + 0.05 * hit, 0.2 + 0.2 * hit, 0.34 - 0.16 * hit, -2.06 - 0.3 * hit, 0.46)))); }
            Some(h) if h.hold == Hold::Tool => {
                let arm = mul(&v, &trs(-0.235 + bx_, -0.25 + by - 0.06 * strike, 0.1 + 0.2 * push + 0.12 * strike, 0.22 + 0.3 * strike, -1.72 + 0.22 * self.run + 0.85 * strike - 0.3 * push, 0.46));
                out.push(hand(arm)); out.push(Draw { m: mul(&arm, &hold_m(Hold::Tool)), r: h.r, view: true, local: h.local, hero: true, dynamic: false });
            }
            Some(h) => {
                let sc = if h.hold == Hold::Block { 0.26 } else { 0.46 };
                let base = mul(&v, &trs(-0.29 + bx_ + 0.05 * strike, -0.2 + by - 0.1 * strike + 0.03 * push, 0.52 + 0.12 * push + 0.08 * strike, 0.62 + 0.25 * strike, 0.26 + 0.6 * strike - 0.3 * push, sc));
                out.push(Draw { m: mul(&base, &trs(-0.5, if h.hold == Hold::Block { -0.5 } else { -0.4 }, -0.5, 0.0, 0.0, 1.0)), r: h.r, view: true, local: h.local, hero: true, dynamic: false });
            }
        }
        out
    }
}
#[allow(dead_code)] fn _r(x: f64) -> f64 { round(x) }
