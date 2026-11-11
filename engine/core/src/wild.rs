//! Faune et vie des biomes froids et secs : renards, lièvres blancs et corbeaux de la taïga, virevoltants de la steppe,
//! traces de pas dans la neige. Suite aléatoire séparée de la faune d'origine, qui garde sa trace du prototype.
use crate::actors::{b, Draw, Models};
use crate::blocks::*;
use crate::det::{cos, hypot2, sin};
use crate::fauna::{fits, ground};
use crate::mesher::VBuf;
use crate::noise::Rng;
use crate::player::Voxels;
use crate::view::{mul, roll_z, rot_axis, trs, M4};

const ORANGE: [f64; 3] = [0.8, 0.38, 0.12]; const WHITE: [f64; 3] = [0.94, 0.93, 0.9]; const BLACK: [f64; 3] = [0.08, 0.07, 0.07];
const CROW: f64 = 1.9; const FOX: f64 = 1.45; /* un renard roux adulte : environ 40 cm au garrot */ const MAX_PRINTS: usize = 160; const PRINT_LIFE: f64 = 150.0;
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let mut fox = VBuf::new(400); { let o = &mut fox;
        b(o, [-0.11, 0.0, -0.3, 0.11, 0.22, 0.28], ORANGE); b(o, [-0.09, -0.02, -0.18, 0.09, 0.04, 0.24], WHITE);
        // queue touffue, pointe blanche
        b(o, [-0.07, 0.02, -0.62, 0.07, 0.15, -0.3], ORANGE); b(o, [-0.06, 0.03, -0.74, 0.06, 0.14, -0.62], WHITE); }
    let mut fox_head = VBuf::new(400); { let o = &mut fox_head;
        b(o, [-0.1, -0.02, -0.04, 0.1, 0.15, 0.14], ORANGE); b(o, [-0.05, -0.02, 0.14, 0.05, 0.06, 0.28], WHITE); b(o, [-0.025, 0.02, 0.28, 0.025, 0.06, 0.31], BLACK);
        b(o, [-0.1, 0.15, -0.02, -0.04, 0.27, 0.04], ORANGE); b(o, [0.04, 0.15, -0.02, 0.1, 0.27, 0.04], ORANGE); b(o, [-0.09, 0.23, -0.015, -0.05, 0.28, 0.035], BLACK); b(o, [0.05, 0.23, -0.015, 0.09, 0.28, 0.035], BLACK);
        b(o, [-0.07, 0.08, 0.13, -0.04, 0.11, 0.145], BLACK); b(o, [0.04, 0.08, 0.13, 0.07, 0.11, 0.145], BLACK); }
    let mut fox_leg = VBuf::new(120); b(&mut fox_leg, [-0.03, -0.28, -0.03, 0.03, 0.0, 0.03], [0.22, 0.12, 0.07]);
    // Lièvres et lapins : brun, gris, blanc d'hiver ou noir ; ventre et queue plus clairs.
    let hare_mesh = |fur: [f64; 3], belly: [f64; 3], tip: [f64; 3]| { let mut o = VBuf::new(500); { let o = &mut o;
        b(o, [-0.1, 0.0, -0.16, 0.1, 0.2, 0.14], fur); b(o, [-0.08, 0.16, -0.14, 0.08, 0.22, 0.1], [fur[0] * 0.88, fur[1] * 0.88, fur[2] * 0.88]);
        b(o, [-0.07, 0.1, 0.1, 0.07, 0.24, 0.25], fur); b(o, [-0.02, 0.15, 0.25, 0.02, 0.18, 0.27], [0.75, 0.55, 0.55]);
        b(o, [-0.075, 0.18, 0.18, -0.06, 0.21, 0.21], BLACK); b(o, [0.06, 0.18, 0.18, 0.075, 0.21, 0.21], BLACK);
        for sx in [-1.0f64, 1.0] { let (a, c) = if sx < 0.0 { (-0.055, -0.02) } else { (0.02, 0.055) }; b(o, [a, 0.24, 0.12, c, 0.46, 0.16], fur); b(o, [a, 0.42, 0.12, c, 0.47, 0.16], tip); }
        b(o, [-0.04, 0.08, -0.2, 0.04, 0.15, -0.15], belly); b(o, [-0.09, 0.0, -0.08, 0.09, 0.05, 0.2], belly); } o };
    let hares = [hare_mesh([0.52, 0.37, 0.24], [0.86, 0.8, 0.7], BLACK), hare_mesh([0.56, 0.55, 0.53], [0.82, 0.81, 0.79], [0.25, 0.24, 0.23]), hare_mesh(WHITE, [0.9, 0.9, 0.9], BLACK), hare_mesh([0.13, 0.12, 0.12], [0.3, 0.28, 0.27], [0.08, 0.08, 0.08])];
    let mut crow = VBuf::new(300); { let o = &mut crow; let mut q = |a: [f64; 6], c: [f64; 3]| b(o, a.map(|v| v * CROW), c);
        q([-0.06, 0.0, -0.12, 0.06, 0.1, 0.1], BLACK); q([-0.05, 0.07, 0.07, 0.05, 0.16, 0.16], BLACK); q([-0.02, 0.1, 0.16, 0.02, 0.13, 0.23], [0.2, 0.2, 0.22]);
        q([-0.04, 0.04, -0.24, 0.04, 0.07, -0.12], [0.1, 0.1, 0.12]); q([-0.035, -0.06, 0.0, -0.02, 0.0, 0.015], BLACK); q([0.02, -0.06, 0.0, 0.035, 0.0, 0.015], BLACK); }
    let mut crow_wing = VBuf::new(60); b(&mut crow_wing, [0.0, 0.0, -0.08 * CROW, 0.2 * CROW, 0.015 * CROW, 0.09 * CROW], [0.1, 0.1, 0.12]);
    // Virevoltant : une boule de brindilles emmêlées, des arcs de petits cubes sur une sphère.
    let mut tumble = VBuf::new(4000); { let o = &mut tumble; let mut r = Rng::new(77);
        for _ in 0..14 {
            let (a, e) = (r.range(0.0, 6.283), r.range(-1.4, 1.4)); let (ax, ay, az) = (cos(a) * cos(e), sin(e), sin(a) * cos(e)); let (ux, uy, uz) = { let (x, y, z) = (-az, 0.0, ax); let l = hypot2(x, z).max(1e-6); (x / l, y, z / l) };
            let (vx, vy, vz) = (ay * uz - az * uy, az * ux - ax * uz, ax * uy - ay * ux); let (start, len) = (r.range(0.0, 6.283), r.range(1.6, 3.2)); let rad = r.range(0.45, 0.62); let k = r.range(0.85, 1.15);
            for i in 0..14 { let t = start + len * i as f64 / 14.0; let (cx, cy, cz) = ((ux * cos(t) + vx * sin(t)) * rad, (uy * cos(t) + vy * sin(t)) * rad, (uz * cos(t) + vz * sin(t)) * rad); let w = 0.028; b(o, [cx - w, cy - w, cz - w, cx + w, cy + w, cz + w], [0.8 * k, 0.68 * k, 0.48 * k]); }
        }
        for _ in 0..40 { let (x, y, z) = (r.range(-0.35, 0.35), r.range(-0.35, 0.35), r.range(-0.35, 0.35)); let w = 0.024; b(o, [x - w, y - w, z - w, x + w, y + w, z + w], [0.66, 0.56, 0.4]); }
    }
    let mut print = VBuf::new(60); b(&mut print, [-0.07, 0.0, -0.11, 0.07, 0.012, 0.11], [0.2, 0.24, 0.33]);
    // Loup : la silhouette du renard, plus haute et plus lourde, gris dessus et pâle dessous ; queue droite, oreilles courtes.
    let mut wolf = VBuf::new(400); { let o = &mut wolf; b(o, [-0.13, 0.0, -0.32, 0.13, 0.26, 0.3], GREY); b(o, [-0.11, -0.02, -0.2, 0.11, 0.05, 0.26], PALE); b(o, [-0.14, 0.12, 0.1, 0.14, 0.3, 0.3], [0.36, 0.35, 0.33]); b(o, [-0.06, 0.04, -0.6, 0.06, 0.16, -0.3], GREY); b(o, [-0.05, 0.05, -0.68, 0.05, 0.14, -0.6], [0.2, 0.2, 0.19]); }
    let mut wolf_head = VBuf::new(400); { let o = &mut wolf_head; b(o, [-0.11, -0.03, -0.04, 0.11, 0.16, 0.15], GREY); b(o, [-0.06, -0.03, 0.15, 0.06, 0.07, 0.3], PALE); b(o, [-0.03, 0.02, 0.3, 0.03, 0.07, 0.33], BLACK);
        b(o, [-0.1, 0.16, -0.02, -0.05, 0.24, 0.04], GREY); b(o, [0.05, 0.16, -0.02, 0.1, 0.24, 0.04], GREY); b(o, [-0.075, 0.08, 0.14, -0.04, 0.11, 0.155], [0.85, 0.7, 0.2]); b(o, [0.04, 0.08, 0.14, 0.075, 0.11, 0.155], [0.85, 0.7, 0.2]); }
    let mut wolf_leg = VBuf::new(120); b(&mut wolf_leg, [-0.035, -0.32, -0.035, 0.035, 0.0, 0.035], [0.32, 0.31, 0.3]);
    // Sanglier : un corps massif et bas, brun noir, une crête de soies, le groin et deux défenses.
    let mut boar = VBuf::new(400); { let o = &mut boar; b(o, [-0.18, 0.0, -0.36, 0.18, 0.34, 0.3], BRISTLE); b(o, [-0.05, 0.3, -0.3, 0.05, 0.4, 0.26], [0.15, 0.11, 0.08]); b(o, [-0.02, 0.1, -0.42, 0.02, 0.22, -0.36], [0.15, 0.11, 0.08]); }
    let mut boar_head = VBuf::new(400); { let o = &mut boar_head; b(o, [-0.14, -0.08, -0.04, 0.14, 0.18, 0.2], BRISTLE); b(o, [-0.07, -0.06, 0.2, 0.07, 0.06, 0.32], [0.32, 0.22, 0.17]); b(o, [-0.05, -0.04, 0.32, 0.05, 0.04, 0.34], [0.5, 0.36, 0.32]);
        b(o, [-0.1, -0.04, 0.22, -0.07, 0.06, 0.26], TUSK); b(o, [0.07, -0.04, 0.22, 0.1, 0.06, 0.26], TUSK); b(o, [-0.12, 0.16, -0.02, -0.06, 0.24, 0.04], BRISTLE); b(o, [0.06, 0.16, -0.02, 0.12, 0.24, 0.04], BRISTLE); }
    let mut boar_leg = VBuf::new(120); b(&mut boar_leg, [-0.045, -0.2, -0.045, 0.045, 0.0, 0.045], [0.16, 0.12, 0.09]);
    // Ours : massif, brun, garrot bossu, museau clair ; bouquetin : élancé, cornes en arc ; lézard : plat, queue longue, pattes écartées.
    let mut bear = VBuf::new(400); { let o = &mut bear; b(o, [-0.18, 0.0, -0.32, 0.18, 0.32, 0.3], BROWN); b(o, [-0.16, 0.24, 0.02, 0.16, 0.38, 0.26], [0.28, 0.18, 0.11]); b(o, [-0.05, 0.08, -0.38, 0.05, 0.16, -0.32], BROWN); }
    let mut bear_head = VBuf::new(400); { let o = &mut bear_head; b(o, [-0.13, -0.06, -0.04, 0.13, 0.15, 0.14], BROWN); b(o, [-0.07, -0.05, 0.14, 0.07, 0.05, 0.25], [0.5, 0.38, 0.26]); b(o, [-0.03, -0.01, 0.25, 0.03, 0.03, 0.27], BLACK);
        b(o, [-0.13, 0.13, -0.02, -0.07, 0.2, 0.04], BROWN); b(o, [0.07, 0.13, -0.02, 0.13, 0.2, 0.04], BROWN); b(o, [-0.08, 0.05, 0.13, -0.05, 0.08, 0.145], BLACK); b(o, [0.05, 0.05, 0.13, 0.08, 0.08, 0.145], BLACK); }
    let mut bear_leg = VBuf::new(120); b(&mut bear_leg, [-0.06, -0.26, -0.06, 0.06, 0.0, 0.06], [0.24, 0.15, 0.09]);
    let mut ibex = VBuf::new(400); { let o = &mut ibex; b(o, [-0.1, 0.0, -0.26, 0.1, 0.2, 0.24], IBEX_C); b(o, [-0.08, -0.02, -0.16, 0.08, 0.04, 0.2], [0.85, 0.8, 0.7]); b(o, [-0.03, 0.08, -0.31, 0.03, 0.16, -0.26], [0.85, 0.8, 0.7]); }
    let mut ibex_head = VBuf::new(400); { let o = &mut ibex_head; b(o, [-0.06, -0.02, -0.03, 0.06, 0.12, 0.1], IBEX_C); b(o, [-0.04, -0.03, 0.1, 0.04, 0.05, 0.2], IBEX_C); b(o, [-0.02, -0.08, 0.12, 0.02, -0.02, 0.16], [0.4, 0.34, 0.26]);
        for x in [-0.05, 0.03] { b(o, [x, 0.12, -0.02, x + 0.025, 0.24, 0.02], HORN); b(o, [x, 0.22, -0.1, x + 0.025, 0.27, -0.01], HORN); b(o, [x, 0.14, -0.16, x + 0.025, 0.24, -0.1], HORN); } }
    let mut ibex_leg = VBuf::new(120); b(&mut ibex_leg, [-0.025, -0.34, -0.025, 0.025, 0.0, 0.025], [0.42, 0.36, 0.28]);
    let mut lizard = VBuf::new(400); { let o = &mut lizard; b(o, [-0.045, 0.0, -0.1, 0.045, 0.04, 0.1], SCALE); b(o, [-0.03, 0.0, 0.1, 0.03, 0.035, 0.17], SCALE); b(o, [-0.015, 0.0, -0.28, 0.015, 0.02, -0.1], [0.42, 0.44, 0.24]);
        for (x, z) in [(-0.08, 0.06), (0.05, 0.06), (-0.08, -0.07), (0.05, -0.07)] { b(o, [x, 0.0, z, x + 0.03, 0.015, z + 0.02], [0.4, 0.42, 0.22]); } b(o, [-0.025, 0.025, 0.15, -0.012, 0.035, 0.16], BLACK); b(o, [0.012, 0.025, 0.15, 0.025, 0.035, 0.16], BLACK); }
    let [h0, h1, h2, h3] = hares; vec![("bear", bear), ("bearHead", bear_head), ("bearLeg", bear_leg), ("ibex", ibex), ("ibexHead", ibex_head), ("ibexLeg", ibex_leg), ("lizard", lizard), ("wolf", wolf), ("wolfHead", wolf_head), ("wolfLeg", wolf_leg), ("boar", boar), ("boarHead", boar_head), ("boarLeg", boar_leg), ("fox", fox), ("foxHead", fox_head), ("foxLeg", fox_leg), ("hare0", h0), ("hare1", h1), ("hare2", h2), ("hare3", h3), ("crow", crow), ("crowWing", crow_wing), ("tumble", tumble), ("print", print)]
}

#[derive(Clone, Copy, PartialEq, Debug)] pub enum Kind { Fox, Hare, Crow, Tumble, Wolf, Boar, Bear, Ibex, Lizard }
#[derive(Clone, Copy, PartialEq, Debug)] enum State { Idle, Walk, Flee, Fly, Gone, Chase }
#[derive(Clone, Debug)] pub struct Critter { pub kind: Kind, pub color: u8, pub p: [f64; 3], gy: f64, pub yaw: f64, state: State, t: f64, phase: f64, speed: f64, v: [f64; 3], spin: f64, axis: [f64; 3], step: f64, foot: bool, life: f64, pub hp: f64, pub hit: f64, cd: f64 }
/// Ce que cette faune fait entendre.
#[derive(Clone, Copy, Debug)] pub enum Ev { Caw([f64; 3]), Bark([f64; 3]), Scream([f64; 3]), Thump([f64; 3]), Rustle([f64; 3]), Bite([f64; 3], f64, &'static str), Howl([f64; 3]), Grunt([f64; 3]) }
/// Une empreinte posée sur la neige.
#[derive(Clone, Copy, Debug)] pub struct Print { pub p: [f64; 3], pub yaw: f64, pub age: f64, pub small: bool }
pub struct Wild { cold: f64, dry: f64, pub night: bool, rng: Rng, pub all: Vec<Critter>, pub prints: Vec<Print>, pub ev: Vec<Ev>, pub wind: f64, t: f64, last_print: Option<[f64; 3]>, left: bool }

/// Hauteur de la surface de neige à cet endroit, si l'on marche dans la neige.
/// Dessus du sol le plus haut sous `from` : le premier bloc plein en descendant (feuillage et bois compris). Rien sur l'eau.
/// Le sursaut d'une bête touchée : il monte puis retombe en douceur (0 au début et à la fin), au lieu de sauter d'un coup.
pub fn flinch(h: f64) -> f64 { if h <= 0.0 { 0.0 } else { ((1.0 - h / 0.3).clamp(0.0, 1.0) * std::f64::consts::PI).sin() * 0.3 } }
/// Le recul d'un coup : une poussée qui glisse sur le sol et s'amortit, arrêtée par les murs et les marches trop hautes.
fn knock(c: &mut Critter, dt: f64, w: &impl Voxels) {
    if c.v[0] == 0.0 && c.v[2] == 0.0 { return; }
    let (nx, nz) = (c.p[0] + c.v[0] * dt, c.p[2] + c.v[2] * dt);
    match ground(w, nx, nz, c.gy) { Some(y) if (y - c.gy).abs() <= 1.0 && fits(w, nx, y, nz, c.yaw, 1) => { c.p[0] = nx; c.p[2] = nz; c.gy = y; } _ => { c.v = [0.0; 3]; return; } }
    let k = (-8.0 * dt).exp(); c.v[0] *= k; c.v[2] *= k; if hypot2(c.v[0], c.v[2]) < 0.1 { c.v = [0.0; 3]; }
}
fn surface(w: &impl Voxels, x: f64, z: f64, from: f64) -> Option<f64> {
    let (fx, fz) = (x.floor() as i32, z.floor() as i32); let mut y = from.floor() as i32;
    while y > 1 && y > from as i32 - 60 { let b = w.get(fx, y, fz)?; if is_water(b) { return None; } if is_solid(b) { return Some((y + 1) as f64); } y -= 1; } None
}
pub fn snow_at(w: &impl Voxels, x: f64, y: f64, z: f64) -> Option<f64> {
    let (fx, fy, fz) = (x.floor() as i32, (y + 0.05).floor() as i32, z.floor() as i32);
    let n = snow_level(w.get(fx, fy, fz)?); if n > 0 { return Some(fy as f64 + n as f64 / 10.0); }
    if w.get(fx, fy - 1, fz)? == B::SNOW { return Some(fy as f64); } None
}
fn hp_of(k: Kind) -> f64 { match k { Kind::Fox => crate::survival::FOX_HP, Kind::Hare => crate::survival::HARE_HP, Kind::Wolf => crate::survival::WOLF_HP, Kind::Boar => crate::survival::BOAR_HP, Kind::Bear => crate::survival::BEAR_HP, Kind::Ibex => crate::survival::IBEX_HP, Kind::Lizard => crate::survival::LIZARD_HP, _ => 1.0 } }
const WOLF: f64 = 1.75; const BOAR: f64 = 1.25; const BEAR: f64 = 2.3; const IBEX: f64 = 1.5;
const BROWN: [f64; 3] = [0.32, 0.21, 0.13]; const IBEX_C: [f64; 3] = [0.6, 0.52, 0.4]; const HORN: [f64; 3] = [0.45, 0.4, 0.33]; const SCALE: [f64; 3] = [0.5, 0.52, 0.3];
const GREY: [f64; 3] = [0.42, 0.41, 0.39]; const PALE: [f64; 3] = [0.7, 0.68, 0.64]; const BRISTLE: [f64; 3] = [0.24, 0.17, 0.12]; const TUSK: [f64; 3] = [0.92, 0.88, 0.78];
impl Wild {
    /// Un coup porté à l'animal `i` : il détale, ou tombe. Renvoie son espèce s'il est mort (il disparaît alors).
    pub fn strike(&mut self, i: usize, dmg: f64, from: [f64; 3]) -> Option<Kind> {
        let c = self.all.get_mut(i)?; c.hp -= dmg; c.hit = 0.3; let k = c.kind; { let (dx, dz) = (c.p[0] - from[0], c.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); c.v = [dx / d * 3.0, 0.0, dz / d * 3.0]; }
        if c.hp <= 0.0 { self.all.remove(i); return Some(k); }
        // Frappés, le loup et le sanglier se retournent contre vous (sauf à bout de forces) ; les autres fuient.
        if matches!(k, Kind::Wolf | Kind::Boar | Kind::Bear) && c.hp > hp_of(k) * 0.35 { c.state = State::Chase; c.t = 12.0; if k == Kind::Boar { self.ev.push(Ev::Grunt(c.p)); } return None; }
        c.state = State::Flee; c.t = self.rng.range(4.0, 6.0); c.yaw = (c.p[0] - from[0]).atan2(c.p[2] - from[2]);
        if k == Kind::Hare { self.ev.push(Ev::Thump(c.p)); } else { self.ev.push(Ev::Bark(c.p)); } None
    }
    /// Une bête repoussée (parade, coup lourd) : elle recule d'un pas et ne mord plus de quelques instants.
    pub fn stagger(&mut self, i: usize, from: [f64; 3], secs: f64) { if let Some(c) = self.all.get_mut(i) { let (dx, dz) = (c.p[0] - from[0], c.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); c.v = [dx / d * 6.0, 0.0, dz / d * 6.0]; c.cd = c.cd.max(secs); c.hit = 0.3; } }
    /// La bête la plus proche d'un point (celle qui vient de mordre).
    pub fn nearest(&self, p: [f64; 3]) -> Option<usize> { self.all.iter().enumerate().map(|(i, c)| (i, (c.p[0] - p[0]).hypot(c.p[2] - p[2]))).filter(|x| x.1 < 3.0).min_by(|a, b| a.1.total_cmp(&b.1)).map(|x| x.0) }
    pub fn new(seed: i32) -> Self { Wild { cold: 0.0, dry: 0.0, night: false, rng: Rng::new((seed ^ 0x2f0c) as u32), all: Vec::new(), prints: Vec::new(), ev: Vec::new(), wind: 0.7, t: 0.0, last_print: None, left: false } }
    /// Pose un animal ou un virevoltant à un endroit précis (bancs d'essai, captures).
    pub fn put(&mut self, kind: Kind, p: [f64; 3], yaw: f64, flying: bool) { self.all.push(Critter { kind, color: if kind == Kind::Hare { 2 } else { 0 }, p, gy: p[1], yaw, state: if flying { State::Fly } else { State::Idle }, t: 30.0, phase: 0.6, speed: 0.0, v: [0.0; 3], spin: 0.7, axis: [0.6, 0.0, 0.8], step: 0.0, foot: false, life: 0.0, hp: hp_of(kind), hit: 0.0, cd: 0.0 }); }
    fn count(&self, k: Kind) -> usize { self.all.iter().filter(|c| c.kind == k).count() }
    fn spawn(&mut self, kind: Kind, w: &impl Voxels, p: [f64; 3]) {
        let r = &mut self.rng; let (a, d) = if kind == Kind::Tumble { (self.wind + std::f64::consts::PI + r.range(-0.9, 0.9), r.range(26.0, 44.0)) } else { (r.range(0.0, 6.283), r.range(18.0, 50.0)) };
        let (x, z) = (p[0] + sin(a) * d, p[2] + cos(a) * d); let Some(y) = surface(w, x, z, p[1] + 25.0) else { return }; if (y - p[1]).abs() > 30.0 { return; } let yaw = r.range(0.0, 6.283);
        if kind != Kind::Crow && kind != Kind::Tumble && !fits(w, x, y, z, yaw, 1) { return; }
        let y = if kind == Kind::Crow { y + r.range(10.0, 16.0) } else { y };
        let axis = [r.range(-1.0, 1.0), 0.0, r.range(-1.0, 1.0)]; let t = r.range(1.0, 5.0);
        let roll = r.next(); let color = if kind != Kind::Hare { 0 } else if self.cold > 0.5 { if roll < 0.75 { 2 } else { 1 } } else if self.dry > 0.5 { if roll < 0.6 { 0 } else { 1 } } else if roll < 0.5 { 0 } else if roll < 0.8 { 1 } else if roll < 0.9 { 3 } else { 2 };
        self.all.push(Critter { kind, color, p: [x, y, z], gy: y, yaw, state: if kind == Kind::Crow { State::Fly } else { State::Idle }, t, phase: 0.0, speed: 0.0, v: [0.0; 3], spin: 0.0, axis, step: 0.0, foot: false, life: 0.0, hp: hp_of(kind), hit: 0.0, cd: 0.0 });
    }
    /// `taiga` et `steppe` : force du biome autour du joueur, déjà lissée.
    pub fn update(&mut self, dt: f64, w: &impl Voxels, p: [f64; 3], day: f64, taiga: f64, steppe: f64, open: f64, player_ground: bool, wild: [f64; 3]) {
        let [deep, high, desert] = wild;
        self.t += dt; self.cold = taiga; self.dry = steppe; self.wind = 0.7 + 0.6 * sin(self.t * 0.011) + 0.25 * sin(self.t * 0.047);
        let want = |k: Kind| match k { Kind::Fox => (2.0 * taiga + 1.0 * open).round() as usize, Kind::Hare => (3.0 * taiga + 2.0 * open + 2.0 * steppe).round() as usize, Kind::Crow => if day > 0.2 { (4.0 * taiga).round() as usize } else { 0 }, Kind::Tumble => (4.0 * steppe).round() as usize, Kind::Ibex => (3.5 * high).round() as usize, Kind::Lizard => (3.0 * desert).round() as usize, _ => 0 };
        // Loups : en meute, dans la taïga, et la nuit hors des forêts ; sangliers : dans les prairies et les bois clairs.
        let night = day < 0.05; self.night = night;
        let want_pred = |k: Kind| match k { Kind::Wolf => (3.0 * taiga + if night { 2.0 * open } else { 0.0 }).round() as usize, Kind::Boar => (1.6 * open).round() as usize, Kind::Bear => (0.8 * taiga + 1.2 * deep).round() as usize, _ => 0 };
        for k in [Kind::Fox, Kind::Hare, Kind::Crow, Kind::Tumble, Kind::Ibex, Kind::Lizard] { if self.count(k) < want(k) && self.rng.next() < dt * 0.6 { self.spawn(k, w, p); } }
        for k in [Kind::Wolf, Kind::Boar, Kind::Bear] { if self.count(k) < want_pred(k) && self.rng.next() < dt * 0.25 { let n = self.all.len(); self.spawn(k, w, p); if k == Kind::Wolf && self.all.len() > n { let q = self.all[n].p; for _ in 0..2 { let o = [q[0] + self.rng.range(-2.0, 2.0), q[1], q[2] + self.rng.range(-2.0, 2.0)]; let yaw = self.rng.range(0.0, 6.28); self.put(Kind::Wolf, o, yaw, false); } self.ev.push(Ev::Howl(q)); } } }
        let mut all = std::mem::take(&mut self.all);
        for c in all.iter_mut() { c.hit = (c.hit - dt).max(0.0); match c.kind { Kind::Fox | Kind::Hare | Kind::Ibex | Kind::Lizard => self.walker(c, dt, w, p), Kind::Crow => self.crow(c, dt, w, p), Kind::Tumble => self.tumble(c, dt, w), Kind::Wolf | Kind::Boar | Kind::Bear => self.beast(c, dt, w, p) } }
        all.retain(|c| c.state != State::Gone && hypot2(c.p[0] - p[0], c.p[2] - p[2]) < 90.0 && w.get(c.p[0].floor() as i32, c.p[1].floor() as i32, c.p[2].floor() as i32).is_some()); self.all = all;
        // Les pas du joueur s'impriment dans la neige, en alternant pied gauche et pied droit.
        if player_ground { self.foot(w, p, None, false); } else { self.last_print = None; }
        for pr in &mut self.prints { pr.age += dt; } self.prints.retain(|pr| pr.age < PRINT_LIFE && hypot2(pr.p[0] - p[0], pr.p[2] - p[2]) < 80.0);
    }
    fn foot(&mut self, w: &impl Voxels, p: [f64; 3], yaw: Option<f64>, small: bool) {
        let Some(top) = snow_at(w, p[0], p[1], p[2]) else { self.last_print = None; return };
        if let Some(l) = self.last_print { if hypot2(p[0] - l[0], p[2] - l[2]) < 0.7 { return; } }
        let yaw = yaw.unwrap_or_else(|| self.last_print.map_or(0.0, |l| (p[0] - l[0]).atan2(p[2] - l[2]))); self.left = !self.left; let side = if self.left { 0.13 } else { -0.13 };
        self.prints.push(Print { p: [p[0] + cos(yaw) * side, top + 0.004, p[2] - sin(yaw) * side], yaw, age: 0.0, small }); self.last_print = Some(p);
        if self.prints.len() > MAX_PRINTS { self.prints.remove(0); }
    }
    fn walker(&mut self, c: &mut Critter, dt: f64, w: &impl Voxels, p: [f64; 3]) {
        knock(c, dt, w); let hare = c.kind == Kind::Hare; let dist = hypot2(c.p[0] - p[0], c.p[2] - p[2]); let r = &mut self.rng;
        // Distance de fuite, vitesse de fuite et de marche : le bouquetin file loin, le lézard détale au dernier moment.
        let (shy, run, amble) = match c.kind { Kind::Hare => (4.5, 8.5, 1.2), Kind::Ibex => (8.0, 7.5, 1.3), Kind::Lizard => (2.5, 5.5, 0.7), _ => (6.0, 7.0, 1.7) };
        if dist < shy && c.state != State::Flee {
            c.state = State::Flee; c.t = r.range(3.0, 5.0); c.yaw = (c.p[0] - p[0]).atan2(c.p[2] - p[2]); if hare { self.ev.push(Ev::Thump(c.p)); } else if r.next() < 0.6 { self.ev.push(Ev::Bark(c.p)); }
        }
        if c.kind == Kind::Fox && c.state == State::Idle && r.next() < dt * 0.02 { self.ev.push(if r.next() < 0.3 { Ev::Scream(c.p) } else { Ev::Bark(c.p) }); } // le renard appelle de loin en loin
        c.t -= dt;
        if c.t <= 0.0 { match c.state { State::Idle => { c.state = State::Walk; c.t = r.range(2.0, 6.0); c.yaw += r.range(-1.2, 1.2); } _ => { c.state = State::Idle; c.t = r.range(2.0, 7.0); } } }
        // Le lièvre file en zigzag ; le renard trotte, puis détale.
        if c.state == State::Flee && hare && r.next() < dt * 2.0 { c.yaw += r.range(-0.9, 0.9); }
        let sp = match c.state { State::Flee => run, State::Walk => amble, _ => 0.0 }; c.speed = sp;
        if sp == 0.0 { c.phase = 0.0; c.p[1] += (c.gy - c.p[1]) * (dt * 9.0).min(1.0); return; }
        c.phase += dt * (4.0 + sp * 2.2);
        if !steer(c, w, sp * dt) { c.yaw += if c.hit > 0.0 { 0.0 } else { r.range(1.6, 2.6) }; c.t = c.t.max(0.6); }
        c.p[1] += (c.gy - c.p[1]) * (dt * 9.0).min(1.0);
        c.step += dt * sp; if c.step > 0.55 { c.step = 0.0; c.foot = !c.foot; let (q, y) = (c.p, c.yaw); let saved = (self.last_print, self.left); self.last_print = None; self.foot(w, q, Some(y), true); (self.last_print, self.left) = saved; }
    }
    /// Loup et sanglier. Le loup rôde, flaire le joueur (plus loin la nuit), le poursuit et mord ; blessé, il renonce et fuit.
    /// Le sanglier broute et ignore le joueur, mais charge celui qui l'a frappé.
    fn beast(&mut self, c: &mut Critter, dt: f64, w: &impl Voxels, p: [f64; 3]) {
        knock(c, dt, w); let wolf = c.kind == Kind::Wolf; let bear = c.kind == Kind::Bear; let dist = hypot2(c.p[0] - p[0], c.p[2] - p[2]); let r = &mut self.rng; c.cd -= dt; c.t -= dt;
        let weak = c.hp < hp_of(c.kind) * 0.35;
        // L'ours défend son coin : il charge qui s'approche trop, de jour comme de nuit.
        if bear && !weak && c.state != State::Chase && c.state != State::Flee && dist < 6.0 { c.state = State::Chase; c.t = 10.0; self.ev.push(Ev::Grunt(c.p)); }
        if wolf && !weak && c.state != State::Chase && c.state != State::Flee && dist < if self.night { 16.0 } else { 8.0 } { c.state = State::Chase; c.t = 20.0; if r.next() < 0.5 { self.ev.push(Ev::Howl(c.p)); } }
        if weak && c.state == State::Chase { c.state = State::Flee; c.t = 12.0; }
        if c.state == State::Chase && (dist > if bear { 18.0 } else { 30.0 } || c.t <= 0.0) { c.state = State::Walk; c.t = r.range(2.0, 5.0); }
        if c.t <= 0.0 { match c.state { State::Idle => { c.state = State::Walk; c.t = r.range(2.0, 6.0); c.yaw += r.range(-1.2, 1.2); } State::Chase => {} _ => { c.state = State::Idle; c.t = r.range(2.0, 7.0); } } }
        if c.state == State::Chase {
            c.yaw = (p[0] - c.p[0]).atan2(p[2] - c.p[2]);
            if dist < if bear { 2.0 } else { 1.7 } && c.cd <= 0.0 { c.cd = if wolf { 1.3 } else if bear { 2.2 } else { 1.8 }; self.ev.push(Ev::Bite(c.p, if wolf { 6.0 } else if bear { 14.0 } else { 8.0 }, if wolf { "La morsure d'un loup" } else if bear { "Le coup de patte d'un ours" } else { "La charge d'un sanglier" }));
                if c.kind == Kind::Boar { c.state = State::Flee; c.t = 4.0; c.yaw += std::f64::consts::PI; } } // le sanglier charge une fois, puis s'éloigne
        }
        if c.state == State::Flee { if dist < 20.0 { c.yaw = (c.p[0] - p[0]).atan2(c.p[2] - p[2]); } if !wolf && c.t <= 0.0 { c.state = State::Idle; } }
        if bear && c.state == State::Idle && r.next() < dt * 0.03 { self.ev.push(Ev::Grunt(c.p)); }
        if !wolf && c.state == State::Idle && r.next() < dt * 0.05 { self.ev.push(Ev::Grunt(c.p)); }
        let sp = match c.state { State::Chase => if dist < if bear { 1.6 } else { 1.2 } { 0.0 } else if wolf { 6.6 } else if bear { 5.4 } else { 6.2 }, State::Flee => if bear { 4.5 } else { 6.0 }, State::Walk => if wolf { 1.9 } else if bear { 1.0 } else { 1.1 }, _ => 0.0 }; c.speed = sp;
        if sp == 0.0 { c.phase = 0.0; c.p[1] += (c.gy - c.p[1]) * (dt * 9.0).min(1.0); return; }
        c.phase += dt * (4.0 + sp * 2.0);
        if !steer(c, w, sp * dt) && c.state != State::Chase { c.yaw += r.range(1.0, 3.0); }
        c.p[1] += (c.gy - c.p[1]) * (dt * 9.0).min(1.0);
    }
    fn crow(&mut self, c: &mut Critter, dt: f64, w: &impl Voxels, p: [f64; 3]) {
        let r = &mut self.rng; c.phase += dt * if c.state == State::Fly { 11.0 } else { 0.0 }; c.t -= dt;
        if r.next() < dt * 0.12 { self.ev.push(Ev::Caw(c.p)); }
        if c.state == State::Idle {
            if hypot2(c.p[0] - p[0], c.p[2] - p[2]) < 5.0 || c.t <= 0.0 { c.state = State::Fly; c.t = r.range(8.0, 18.0); c.v[1] = 3.0; self.ev.push(Ev::Caw(c.p)); }
            return;
        }
        // Vol plané en larges cercles au-dessus des arbres, parfois une pause au sol.
        c.yaw += dt * 0.35 * if c.axis[0] > 0.0 { 1.0 } else { -1.0 }; let sp = 4.5; let surf = ground(w, c.p[0], c.p[2], c.p[1]).unwrap_or(c.gy);
        let land = c.t <= 0.0 && hypot2(c.p[0] - p[0], c.p[2] - p[2]) > 14.0; let target = if land { surf } else { surf + 11.0 };
        c.v = [sin(c.yaw) * sp, ((target - c.p[1]) * 1.2).clamp(-3.0, 3.0), cos(c.yaw) * sp];
        let n = [c.p[0] + c.v[0] * dt, c.p[1] + c.v[1] * dt, c.p[2] + c.v[2] * dt];
        if w.get(n[0].floor() as i32, n[1].floor() as i32, n[2].floor() as i32).is_some_and(|b| b > 0 && is_solid(b)) { c.p[1] += 3.0 * dt; c.yaw += 0.8 * dt; } else { c.p = n; }
        if land && (c.p[1] - surf).abs() < 0.3 { c.p[1] = surf; c.state = State::Idle; c.t = r.range(4.0, 12.0); }
    }
    /// Virevoltant : poussé par le vent, il roule et rebondit au sol, franchit une marche d'un bloc et repart dans l'autre sens contre un mur.
    /// Il ne monte jamais plus haut que la marche qu'il franchit : rien ne le soulève.
    fn tumble(&mut self, c: &mut Critter, dt: f64, w: &impl Voxels) {
        let r = &mut self.rng; c.life += dt; if c.life > 70.0 { c.state = State::Gone; return; }
        let gust = 3.2 + 1.4 * sin(self.t * 0.7 + c.axis[0] * 5.0); let (wx, wz) = (sin(self.wind) * gust, cos(self.wind) * gust);
        c.v[0] += (wx - c.v[0]) * (dt * 1.5).min(1.0); c.v[2] += (wz - c.v[2]) * (dt * 1.5).min(1.0); c.v[1] = (c.v[1] - 14.0 * dt).max(-20.0);
        let (nx, nz) = (c.p[0] + c.v[0] * dt, c.p[2] + c.v[2] * dt);
        match surface(w, nx, nz, c.p[1] + 1.0) {
            Some(g) if g <= c.p[1] + 1.02 => { c.p[0] = nx; c.p[2] = nz; if g > c.p[1] { c.p[1] = g; c.v[1] = c.v[1].max(2.0); } }
            _ => { c.v[0] *= -0.3; c.v[2] *= -0.3; } // mur, tronc ou eau : il rebondit en arrière
        }
        c.p[1] += c.v[1] * dt;
        let Some(floor) = surface(w, c.p[0], c.p[2], c.p[1] + 0.5) else { c.state = State::Gone; return };
        if c.p[1] <= floor { c.p[1] = floor; if c.v[1] < -2.5 { self.ev.push(Ev::Rustle(c.p)); } c.v[1] = if r.next() < dt * 8.0 { r.range(1.5, 3.5) } else { (-c.v[1] * 0.3).max(0.0) }; }
        let sp = hypot2(c.v[0], c.v[2]); c.spin += sp / 0.58 * dt; c.axis = if sp > 0.01 { [c.v[2] / sp, 0.0, -c.v[0] / sp] } else { c.axis };
    }
    pub fn draws(&self, md: &Models) -> Vec<Draw> {
        let mut out = Vec::with_capacity(self.all.len() * 7 + self.prints.len());
        let mut put = |n: &str, m: M4| out.push(Draw { m, r: md.part(n), view: false, local: false, hero: false, dynamic: false });
        for c in &self.all {
            match c.kind {
                Kind::Fox => {
                    let base = trs(c.p[0], c.p[1] + flinch(c.hit) * 0.4, c.p[2], c.yaw, -flinch(c.hit) * 0.8, FOX); let s = if c.speed > 0.0 { sin(c.phase) * 0.8 } else { 0.0 }; const HIP: f64 = 0.28;
                    put("fox", mul(&base, &trs(0.0, HIP, 0.0, 0.0, 0.0, 1.0))); put("foxHead", mul(&base, &trs(0.0, HIP + 0.12, 0.3, 0.0, if c.state == State::Idle { 0.25 } else { 0.0 }, 1.0)));
                    for (x, z, a) in [(-0.07, 0.2, s), (0.07, 0.2, -s), (-0.07, -0.22, -s), (0.07, -0.22, s)] { put("foxLeg", mul(&base, &trs(x, HIP, z, 0.0, a, 1.0))); }
                }
                Kind::Hare => { let hop = if c.speed > 0.0 { sin(c.phase).abs() * if c.state == State::Flee { 0.4 } else { 0.15 } } else { 0.0 }; put(["hare0", "hare1", "hare2", "hare3"][c.color as usize & 3], trs(c.p[0], c.p[1] + hop + flinch(c.hit) * 0.5, c.p[2], c.yaw, -hop * 0.8 - c.hit, 1.35)); }
                Kind::Crow => {
                    let base = trs(c.p[0], c.p[1] + 0.05, c.p[2], c.yaw, 0.0, 1.0); let f = if c.state == State::Fly { 0.25 + 0.55 * sin(c.phase) } else { -1.25 };
                    put("crow", base); put("crowWing", mul(&base, &mul(&trs(0.06 * CROW, 0.08 * CROW, 0.0, 0.0, 0.0, 1.0), &roll_z(f)))); put("crowWing", mul(&base, &mul(&trs(-0.06 * CROW, 0.08 * CROW, 0.0, 0.0, 0.0, 1.0), &roll_z(std::f64::consts::PI - f))));
                }
                Kind::Wolf | Kind::Boar => {
                    let wolf = c.kind == Kind::Wolf; let (sc, hip, n) = if wolf { (WOLF, 0.32, "wolf") } else { (BOAR, 0.2, "boar") };
                    let base = trs(c.p[0], c.p[1] + flinch(c.hit) * 0.3, c.p[2], c.yaw, -flinch(c.hit) * 0.6, sc); let s = if c.speed > 0.0 { sin(c.phase) * 0.8 } else { 0.0 };
                    put(n, mul(&base, &trs(0.0, hip, 0.0, 0.0, 0.0, 1.0))); let lunge = if c.cd > 1.0 { 0.25 } else { 0.0 };
                    put(if wolf { "wolfHead" } else { "boarHead" }, mul(&base, &trs(0.0, hip + if wolf { 0.14 } else { 0.08 }, if wolf { 0.32 } else { 0.32 } + lunge * 0.3, 0.0, if c.state == State::Idle { 0.3 } else { -lunge }, 1.0)));
                    for (x, z, a) in [(-0.08, 0.22, s), (0.08, 0.22, -s), (-0.08, -0.24, -s), (0.08, -0.24, s)] { put(if wolf { "wolfLeg" } else { "boarLeg" }, mul(&base, &trs(x * if wolf { 1.0 } else { 1.4 }, hip, z, 0.0, a, 1.0))); }
                }
                Kind::Bear => {
                    let base = trs(c.p[0], c.p[1] + flinch(c.hit) * 0.2, c.p[2], c.yaw, -flinch(c.hit) * 0.4, BEAR); let s = if c.speed > 0.0 { sin(c.phase) * 0.6 } else { 0.0 }; const HIP: f64 = 0.26;
                    // Dressé un instant avant de frapper.
                    let rear = if c.cd > 1.6 { 0.45 } else { 0.0 }; let body = mul(&base, &trs(0.0, HIP, 0.0, 0.0, -rear, 1.0));
                    put("bear", body); put("bearHead", mul(&body, &trs(0.0, 0.14, 0.32, 0.0, if c.state == State::Idle { 0.3 } else { 0.0 }, 1.0)));
                    for (x, z, a) in [(-0.11, 0.2, s), (0.11, 0.2, -s), (-0.11, -0.22, -s), (0.11, -0.22, s)] { put("bearLeg", mul(&body, &trs(x, 0.0, z, 0.0, a, 1.0))); }
                }
                Kind::Ibex => {
                    let base = trs(c.p[0], c.p[1] + flinch(c.hit) * 0.3, c.p[2], c.yaw, -flinch(c.hit) * 0.6, IBEX); let s = if c.speed > 0.0 { sin(c.phase) * 0.8 } else { 0.0 }; const HIP: f64 = 0.34;
                    put("ibex", mul(&base, &trs(0.0, HIP, 0.0, 0.0, 0.0, 1.0))); put("ibexHead", mul(&base, &trs(0.0, HIP + 0.16, 0.26, 0.0, if c.state == State::Idle { 0.5 } else { -0.1 }, 1.0)));
                    for (x, z, a) in [(-0.06, 0.18, s), (0.06, 0.18, -s), (-0.06, -0.2, -s), (0.06, -0.2, s)] { put("ibexLeg", mul(&base, &trs(x, HIP, z, 0.0, a, 1.0))); }
                }
                Kind::Lizard => { let wag = if c.speed > 0.0 { sin(c.phase * 1.6) * 0.25 } else { 0.0 }; put("lizard", trs(c.p[0], c.p[1] + 0.01 + flinch(c.hit) * 0.2, c.p[2], c.yaw + wag, 0.0, 1.6)); }
                Kind::Tumble => put("tumble", rot_axis([c.p[0], c.p[1] + 0.58, c.p[2]], c.axis, c.spin, 1.0)),
            }
        }
        for pr in &self.prints { let sc = if pr.small { 0.55 } else { 1.0 }; put("print", trs(pr.p[0], pr.p[1], pr.p[2], pr.yaw, 0.0, sc)); }
        out
    }
}

/// Avancer de `step` dans le sens voulu ; si un mur barre la route, longer l'obstacle par le cap libre le plus proche
/// (de 25° en 25°, d'un côté puis de l'autre), en montant ou descendant d'un bloc s'il le faut. Faux si tout est bouché.
fn steer(c: &mut Critter, w: &impl Voxels, step: f64) -> bool {
    for k in [0.0, 0.44, -0.44, 0.87, -0.87, 1.31, -1.31, 1.75, -1.75, 2.4, -2.4] {
        let yaw = c.yaw + k; let (nx, nz) = (c.p[0] + sin(yaw) * step, c.p[2] + cos(yaw) * step);
        if let Some(y) = ground(w, nx, nz, c.gy) { if (y - c.gy).abs() <= 1.0 && fits(w, nx, y, nz, yaw, 1) {
            c.p[0] = nx; c.p[2] = nz; c.gy = y; if k != 0.0 { c.yaw += k * 0.6; } return true; } }
    }
    false
}
