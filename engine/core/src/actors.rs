//! Personnage, outils et objets tenus : modèles construits par code, animation procédurale.
use crate::blocks::*;
use crate::det::{cos, round, sin};
use crate::mesher::{bx, kind, Mesher, VBuf, VSIZE};
use crate::noise::hash_float;
use crate::session::{item, order};
use crate::view::{invert, mul, roll_z, trs, M4};
use std::collections::HashMap;

type C = [f64; 3];
const SKIN: C = [0.86, 0.66, 0.53]; const SKIN2: C = [0.76, 0.55, 0.43]; const LIP: C = [0.6, 0.34, 0.3]; const WHITE: C = [1.0, 0.98, 0.94]; const IRIS: C = [0.3, 0.24, 0.14]; const PUPIL: C = [0.06, 0.05, 0.05];
const HAIR: C = [0.2, 0.12, 0.07]; const HAIR2: C = [0.3, 0.18, 0.1]; const TUNIC: C = [0.36, 0.44, 0.3]; const TUNIC2: C = [0.28, 0.35, 0.25]; const LINEN: C = [0.8, 0.76, 0.64]; const LEATHER2: C = [0.42, 0.26, 0.14]; const BELT: C = [0.26, 0.16, 0.09]; const BUCKLE: C = [0.74, 0.6, 0.3];
const PANTS: C = [0.33, 0.29, 0.24]; const PANTS2: C = [0.27, 0.23, 0.19]; const BOOT: C = [0.24, 0.15, 0.09]; const SOLE: C = [0.12, 0.09, 0.07];
const BAG: C = [0.42, 0.27, 0.15]; const BAG2: C = [0.52, 0.35, 0.2]; const ROLL: C = [0.68, 0.6, 0.44]; const STRAP: C = [0.26, 0.16, 0.09];
const SHIRT: C = TUNIC; const SHIRT2: C = TUNIC2;
pub(crate) fn b(o: &mut VBuf, a: [f64; 6], c: C) { bx(o, a[0], a[1], a[2], a[3], a[4], a[5], c[0], c[1], c[2], 0.0, 255.0, 0.0, kind::FLAT, true, 0.0, 255.0); }
#[derive(Clone, Copy, Default, Debug)] pub struct Range { pub first: u32, pub count: u32 }
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Hold { Tool, Block, Deco }
#[derive(Clone, Copy, Debug)] pub struct Held { pub r: Range, pub hold: Hold, pub local: bool }
/// Tous les modèles dans un seul tampon de sommets.
pub struct Models { pub data: Vec<u8>, pub n: usize, pub char_verts: usize, parts: HashMap<&'static str, Range>, items: HashMap<&'static str, Held> }
fn take(all: &mut VBuf, o: VBuf) -> Range { let st = all.n; all.data.extend_from_slice(&o.data); all.n += o.n; Range { first: (st / 4 * 6) as u32, count: (o.n / 4 * 6) as u32 } }
fn torso() -> VBuf {
    let mut o = VBuf::new(900); let o2 = &mut o;
    b(o2, [-0.19, -0.04, -0.115, 0.19, 0.26, 0.115], TUNIC2); b(o2, [-0.225, 0.24, -0.13, 0.225, 0.6, 0.13], TUNIC); // taille, puis épaules plus larges
    b(o2, [-0.2, -0.16, -0.125, 0.2, 0.0, 0.125], TUNIC2); b(o2, [-0.09, 0.42, 0.13, 0.09, 0.59, 0.137], LINEN); b(o2, [-0.014, 0.28, 0.13, 0.014, 0.43, 0.137], TUNIC2); // pans de la tunique, col ouvert
    b(o2, [-0.205, 0.0, -0.125, 0.205, 0.06, 0.125], BELT); b(o2, [-0.045, -0.005, 0.125, 0.045, 0.065, 0.134], BUCKLE); b(o2, [0.1, -0.11, 0.1, 0.19, 0.03, 0.15], LEATHER2); // ceinture, boucle, bourse
    b(o2, [-0.085, 0.58, -0.085, 0.085, 0.68, 0.085], SKIN2); // cou
    b(o2, [-0.17, 0.08, -0.32, 0.17, 0.52, -0.13], BAG); b(o2, [-0.13, 0.12, -0.355, 0.13, 0.29, -0.32], BAG2); b(o2, [-0.035, 0.33, -0.36, 0.035, 0.4, -0.31], BUCKLE); // sac
    b(o2, [-0.17, 0.1, -0.135, -0.11, 0.6, 0.135], STRAP); b(o2, [0.11, 0.1, -0.135, 0.17, 0.6, 0.135], STRAP); // bretelles
    for i in 0..6 { let a = (i as f64 / 6.0) * 6.283; let (y, z) = (0.6 + cos(a) * 0.085, -0.22 + sin(a) * 0.085); b(o2, [-0.25, y - 0.05, z - 0.05, 0.25, y + 0.05, z + 0.05], if i % 2 != 0 { ROLL } else { [0.6, 0.53, 0.38] }); }
    o
}
fn head() -> VBuf {
    let mut o = VBuf::new(1500); let o2 = &mut o;
    b(o2, [-0.175, 0.03, -0.185, 0.175, 0.38, 0.185], SKIN); b(o2, [-0.14, -0.01, -0.13, 0.14, 0.05, 0.16], SKIN2); // crâne, mâchoire plus étroite
    for sx in [-1.0f64, 1.0] {
        let box_ = |o: &mut VBuf, a: f64, y0: f64, z0: f64, c: f64, y1: f64, z1: f64, col: C| b(o, [f64::min(sx * a, sx * c), y0, z0, f64::max(sx * a, sx * c), y1, z1], col);
        // Les sommets sont arrondis au 1/128 : chaque couche de l'œil tient exactement sur un cran, sinon elles se confondent.
        let g = |n: f64| n / 128.0;
        box_(o2, g(5.0), g(25.0), g(24.0), g(16.0), g(32.0), g(25.0), WHITE); box_(o2, g(8.0), g(25.0), g(24.0), g(14.0), g(32.0), g(26.0), IRIS); box_(o2, g(10.0), g(27.0), g(24.0), g(13.0), g(31.0), g(27.0), PUPIL); // œil
        box_(o2, 0.032, 0.262, 0.185, 0.135, 0.288, 0.2, HAIR); box_(o2, 0.04, 0.175, 0.185, 0.125, 0.19, 0.19, SKIN2); // sourcil, paupière basse
        box_(o2, 0.175, 0.14, -0.03, 0.195, 0.245, 0.05, SKIN2); // oreille
    }
    b(o2, [-0.028, 0.125, 0.185, 0.028, 0.235, 0.232], SKIN); b(o2, [-0.034, 0.125, 0.205, 0.034, 0.155, 0.238], SKIN2); // nez
    b(o2, [-0.068, 0.075, 0.185, 0.068, 0.092, 0.195], LIP); b(o2, [-0.045, 0.092, 0.185, 0.045, 0.1, 0.192], SKIN2); // bouche
    // Cheveux : une calotte, puis des mèches de longueurs inégales qui cassent la forme du bloc.
    b(o2, [-0.19, 0.33, -0.2, 0.19, 0.43, 0.2], HAIR); b(o2, [-0.19, 0.1, -0.2, 0.19, 0.36, -0.1], HAIR); b(o2, [-0.19, 0.23, -0.12, -0.165, 0.36, 0.13], HAIR); b(o2, [0.165, 0.23, -0.12, 0.19, 0.36, 0.13], HAIR);
    for x in -3i32..=3 { for z in -3i32..=3 {
        let h = hash_float(x, 3, z, 5); let (xf, zf) = (x as f64 * 0.058, z as f64 * 0.058); let s = 0.03 + 0.012 * h; b(o2, [xf - s, 0.415, zf - s, xf + s, 0.44 + 0.055 * h, zf + s], if h > 0.5 { HAIR2 } else { HAIR });
    } }
    for x in -3i32..=3 { let h = hash_float(x, 8, 2, 5); let xf = x as f64 * 0.056; b(o2, [xf - 0.03, 0.3 - 0.05 * h, 0.178, xf + 0.03, 0.37, 0.215], if h > 0.5 { HAIR2 } else { HAIR }); } // frange
    for x in -3i32..=3 { let h = hash_float(x, 2, 9, 5); let xf = x as f64 * 0.056; b(o2, [xf - 0.03, 0.05 - 0.05 * h, -0.215, xf + 0.03, 0.2, -0.185], if h > 0.5 { HAIR2 } else { HAIR }); } // nuque
    o
}
const THIGH: f64 = 0.38; const UPPER: f64 = 0.29;
fn thigh() -> VBuf { let mut o = VBuf::new(60); b(&mut o, [-0.1, -THIGH - 0.03, -0.105, 0.1, 0.02, 0.105], PANTS); b(&mut o, [-0.103, -0.26, -0.03, 0.103, -0.13, 0.108], PANTS2); o }
fn shin() -> VBuf { let mut o = VBuf::new(100); b(&mut o, [-0.09, -0.18, -0.09, 0.09, 0.02, 0.095], PANTS); b(&mut o, [-0.105, -0.22, -0.105, 0.105, -0.13, 0.11], BOOT); b(&mut o, [-0.098, -0.37, -0.1, 0.098, -0.2, 0.105], BOOT); b(&mut o, [-0.098, -0.4, -0.11, 0.098, -0.32, 0.2], BOOT); b(&mut o, [-0.102, -0.42, -0.115, 0.102, -0.395, 0.205], SOLE); o }
fn upper() -> VBuf { let mut o = VBuf::new(60); b(&mut o, [-0.082, -UPPER - 0.03, -0.085, 0.082, 0.03, 0.085], TUNIC); b(&mut o, [-0.09, -0.03, -0.095, 0.09, 0.05, 0.095], STRAP); o }
fn fore() -> VBuf { let mut o = VBuf::new(60); b(&mut o, [-0.075, -0.09, -0.078, 0.075, 0.03, 0.078], TUNIC2); b(&mut o, [-0.062, -0.21, -0.065, 0.062, -0.09, 0.065], SKIN); b(&mut o, [-0.072, -0.235, -0.074, 0.072, -0.185, 0.074], LEATHER2); b(&mut o, [-0.066, -0.295, -0.07, 0.066, -0.23, 0.07], SKIN); o }
fn fist() -> VBuf { let mut o = VBuf::new(100); b(&mut o, [-0.085, -0.34, -0.085, 0.085, 0.0, 0.085], SHIRT); b(&mut o, [-0.09, -0.38, -0.09, 0.09, -0.33, 0.09], SHIRT2); b(&mut o, [-0.08, -0.66, -0.08, 0.08, -0.38, 0.08], SKIN); o }

// Outils : image 56 × 56 décrite par des formes, puis extrudée en petits voxels. Couleurs : indices de PAL.
const PAL: [C; 31] = [[0.3, 0.17, 0.08], [0.48, 0.29, 0.14], [0.66, 0.43, 0.22], [0.2, 0.11, 0.06], [0.36, 0.2, 0.1], [0.4, 0.39, 0.38], [0.62, 0.61, 0.6], [0.84, 0.83, 0.82], [1.0, 0.99, 0.97], [0.62, 0.42, 0.1], [0.92, 0.7, 0.2], [0.0; 3],
    [0.14, 0.17, 0.4], [0.32, 0.4, 0.7], [0.24, 0.44, 0.18], [0.74, 0.14, 0.08], [0.96, 0.4, 0.18], [0.66, 0.16, 0.16], [0.9, 0.62, 0.56], [0.9, 0.86, 0.76], [0.36, 0.17, 0.08], [0.56, 0.3, 0.13], [0.6, 0.4, 0.22], [0.42, 0.26, 0.13],
    [0.33, 0.32, 0.31], [0.46, 0.45, 0.43], [0.6, 0.59, 0.56], [0.72, 0.7, 0.66], [0.7, 0.64, 0.38], [0.52, 0.5, 0.27], [0.76, 0.64, 0.42]];
// Survie : baies, gibier, cuir. Des volumes plus épais que les lames.
const BERRY: [u8; 2] = [12, 13]; const LEAF: u8 = 14; const ROWAN: [u8; 2] = [15, 16]; const MEAT: [u8; 2] = [17, 18]; const BONE: u8 = 19; const COOKED: [u8; 2] = [20, 21]; const HIDE: [u8; 2] = [22, 23]; const FLINT: [u8; 4] = [24, 25, 26, 27]; const FIBER: [u8; 2] = [28, 29]; const SEED: u8 = 30;
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
        "stick" => { let (s, p) = ((x + y) * R2, (y - x) * R2); if s > 0.14 && s < 1.26 && p.abs() < 0.036 { return Some(if p > 0.012 { WOOD[2] } else if p < -0.012 { WOOD[0] } else { WOOD[1] }); } None }
        "berries" | "rowan_berries" => {
            let c = if name == "berries" { BERRY } else { ROWAN };
            for (cx, cy) in [(0.52, 0.36), (0.4, 0.5), (0.6, 0.5), (0.5, 0.63), (0.36, 0.67), (0.65, 0.67)] { let (dx, dy) = (x - cx, y - cy); if dx * dx + dy * dy < 0.0081 { return Some(if dx + dy < -0.05 { c[1] } else { c[0] }); } }
            let (dx, dy) = (x - 0.66, y - 0.22); if (dx + dy) * (dx + dy) / 0.03 + (dx - dy) * (dx - dy) / 0.006 < 1.0 { return Some(LEAF); }
            if (x - 0.52).abs() < 0.015 && y > 0.18 && y < 0.3 { return Some(WOOD[0]); } None
        }
        "raw_meat" | "cooked_meat" => {
            let c = if name == "raw_meat" { MEAT } else { COOKED }; let (s, p) = ((x + y) * R2, (y - x) * R2);
            for (cs, cp) in [(0.28, -0.07), (0.28, 0.07)] { if crate::det::hypot2(s - cs, p - cp) < 0.075 { return Some(BONE); } }
            if s > 0.28 && s < 0.6 && p.abs() < 0.04 { return Some(BONE); }
            let (ds, dp) = (s - 0.86, p); if ds * ds / 0.08 + dp * dp / 0.042 < 1.0 { return Some(if ds * ds / 0.08 + dp * dp / 0.042 > 0.72 && dp < 0.0 { c[1] } else { c[0] }); } None
        }
        // Outils de pierre : un manche de bois, une pierre taillée aux arêtes irrégulières, ligaturée de fibre.
        "stone_axe" => {
            let (s, p) = ((x + y) * R2, (x - y) * R2); let jag = 0.018 * ((s * 31.0).sin() + (p * 23.0).cos());
            if p > -0.03 && p < 0.3 { let half = 0.07 + 0.11 * (p / 0.3).max(0.0) + jag; if (s - 0.88).abs() < half { let e = 0.3 - p; return Some(if e < 0.04 { FLINT[3] } else if (s - 0.88) > half * 0.4 { FLINT[2] } else if (s - 0.88) < -half * 0.5 { FLINT[0] } else { FLINT[1] }); } }
            if p <= -0.03 && p > -0.08 && (s - 0.88).abs() < 0.1 { return Some(if ((s * 60.0).floor() as i64) & 1 != 0 { FIBER[0] } else { FIBER[1] }); }
            if s > 0.08 && s < 1.0 && p.abs() < 0.036 { return Some(if p > 0.012 { WOOD[2] } else if p < -0.012 { WOOD[0] } else { WOOD[1] }); } None
        }
        "stone_pickaxe" => {
            let (dx, dy) = (x - 0.34, y - 0.34); let r = crate::det::hypot2(dx, dy); let t = (dy.atan2(dx) - std::f64::consts::FRAC_PI_4) / 0.95; let jag = 0.014 * ((t * 17.0).sin() + (r * 40.0).cos());
            if t.abs() < 1.0 { let w = 0.02 + 0.07 * (1.0 - t * t) + jag; let d = r - 0.62; if d.abs() < w { return Some(if t.abs() > 0.75 { FLINT[3] } else if d > w * 0.35 { FLINT[2] } else if d < -w * 0.4 { FLINT[0] } else { FLINT[1] }); } }
            let (s, p) = ((x + y) * R2, (y - x) * R2);
            if s > 0.82 && s < 0.96 && p.abs() < 0.06 { return Some(if ((p * 70.0).floor() as i64) & 1 != 0 { FIBER[0] } else { FIBER[1] }); }
            if s > 0.08 && s < 0.9 && p.abs() < 0.036 { return Some(if p > 0.012 { WOOD[2] } else if p < -0.012 { WOOD[0] } else { WOOD[1] }); } None
        }
        "stone_knife" => {
            let (s, p) = ((x + y) * R2, (y - x) * R2);
            if s > 0.52 && s < 1.12 { let w = 0.07 * ((1.12 - s) / 0.6).powf(0.7) + 0.008; if p > -w * 0.5 && p < w { return Some(if p > w * 0.55 { FLINT[3] } else if ((s * 26.0).floor() as i64) & 1 != 0 { FLINT[1] } else { FLINT[2] }); } }
            if s > 0.12 && s <= 0.52 && p.abs() < 0.038 { return Some(if s > 0.4 || ((s * 50.0).floor() as i64) & 1 != 0 { FIBER[0] } else { WOOD[1] }); } None
        }
        "pebble" => { let (dx, dy) = ((x - 0.5) / 0.3, (y - 0.52) / 0.22); let d = dx * dx + dy * dy; if d < 1.0 { return Some(if dx + dy < -0.6 { FLINT[3] } else if dx + dy > 0.7 { FLINT[0] } else if d < 0.5 { FLINT[2] } else { FLINT[1] }); } None }
        "fiber" => {
            // Une botte de longues fibres ondulées, liée au milieu.
            if (y - 0.5).abs() < 0.035 && (x - 0.5).abs() < 0.13 { return Some(WOOD[0]); }
            for k in 0..6 { let c = 0.36 + k as f64 * 0.055 + 0.05 * ((y * 9.0 + k as f64).sin()) * ((y - 0.5).abs() * 2.0); if (x - c).abs() < 0.016 && y > 0.12 + 0.03 * (k % 2) as f64 && y < 0.88 - 0.04 * (k % 3) as f64 { return Some(FIBER[k % 2]); } }
            None
        }
        "seeds" => { for (cx, cy) in [(0.38, 0.42), (0.55, 0.36), (0.62, 0.55), (0.44, 0.6), (0.52, 0.5), (0.34, 0.58)] { let (dx, dy) = ((x - cx) / 0.06, (y - cy) / 0.04); if dx * dx + dy * dy < 1.0 { return Some(if dx < -0.3 { FIBER[0] } else { SEED }); } } None }
        "leather" => {
            let (dx, dy) = ((x - 0.5).abs(), (y - 0.5).abs()); if dx > 0.34 || dy > 0.28 { return None; }
            if dx > 0.26 && dy > 0.2 { return None; } if (dx > 0.3 && dy < 0.06) || (dy > 0.24 && dx < 0.07) { return None; } // coins et pattes de la peau
            Some(if ((x + y * 0.3) * 9.0).floor() as i64 & 1 != 0 { HIDE[0] } else { HIDE[1] })
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
/// Couleur et épaisseur d'un point de l'image d'un objet : les anciens dessins par indice de palette, les nouveaux par familles.
fn look(name: &str, x: f64, y: f64) -> Option<(C, f64)> {
    if matches!(name, "pickaxe" | "axe" | "sword" | "stick" | "berries" | "rowan_berries" | "raw_meat" | "cooked_meat" | "leather" | "stone_axe" | "stone_pickaxe" | "stone_knife" | "pebble" | "fiber" | "seeds") {
        return shape(name, x, y).map(|k| (PAL[k as usize], if k == STEEL[3] { 0.012 } else if STEEL.contains(&k) { 0.022 } else if k >= BERRY[0] { 0.05 } else { 0.03 }));
    }
    crate::icons::look(name, x, y)
}
fn tool_mesh(name: &str) -> VBuf {
    let mut o = VBuf::new(8000); const N: usize = 56; let s = 0.86; let px = s / N as f64; let grip = 0.2 * R2 * s;
    for j in 0..N {
        let mut run: Option<(usize, (C, f64))> = None;
        for i in 0..=N {
            let c = if i < N { look(name, (i as f64 + 0.5) / N as f64, (j as f64 + 0.5) / N as f64) } else { None };
            if let Some((a, (col, th))) = run { if c != Some((col, th)) {
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
        for (n, m) in [("torso", torso()), ("head", head()), ("thigh", thigh()), ("shin", shin()), ("upper", upper()), ("fore", fore()), ("fist", fist())] { let r = take(&mut all, m); parts.insert(n, r); }
        let char_verts = all.n; let mut items = HashMap::new(); let mv = Mesher::new();
        for name in order() {
            let it = item(name).unwrap(); let (o, hold) = match it.block {
                None => (tool_mesh(name), Hold::Tool),
                // Dans le monde, le polypore se dessine contre son arbre : tenu ou en icône, il n'a pas de support.
                Some(id) if id == B::SHELF_FUNGUS => { let mut o = VBuf::new(60); for (y, w, t) in [(0.12, 0.42, 1.0), (0.38, 0.33, 1.08), (0.62, 0.24, 0.94)] { b(&mut o, [0.5 - w, y, 0.5 - w, 0.5 + w, y + 0.09, 0.5 + w], [0.82 * t, 0.68 * t, 0.5 * t]); } (o, Hold::Deco) }
                Some(id) if BLOCKS[id as usize].shape == Shape::Deco => { let mut o = VBuf::new(800); mv.deco_model(&mut o, id, 0, 0, 0, 7, 11, 255.0, if emit(id) != 0 { 255.0 } else { 0.0 }, 0); (o, Hold::Deco) }
                Some(id) => (cube_mesh(id), Hold::Block),
            };
            let local = hold == Hold::Block && it.block != Some(B::WATER);
            items.insert(it.id, Held { r: take(&mut all, o), hold, local });
        }
        for (name, o) in crate::fauna::parts() { parts.insert(name, take(&mut all, o)); }
        for (name, o) in crate::wild::parts() { parts.insert(name, take(&mut all, o)); }
        for (name, o) in crate::survival::parts() { parts.insert(name, take(&mut all, o)); }
        for (name, o) in crate::village::parts() { parts.insert(name, take(&mut all, o)); }
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
    /// Troisième personne. Hanches et épaules : négatif vers l'avant. Genoux : positif plie vers l'arrière ; coudes : négatif plie vers l'avant.
    pub fn third(&self, md: &Models, p: [f64; 3], yaw: f64, pitch: f64, held: Option<Held>, roll: Option<(f64, [f64; 2])>) -> Vec<Draw> {
        let (w, s, c, ground, a, pi) = (self.walk, sin(self.phase), cos(self.phase), 1.0 - self.air, self.air, std::f64::consts::PI);
        let (run, low, rise, land) = (self.run, self.low, self.rise, self.landed);
        let amp = (0.6 + 0.48 * run) * w * ground * (1.0 - 0.4 * low); let bend = 0.5 + 1.2 * run;
        // Le genou plie surtout quand la jambe revient vers l'avant, et se déplie pour poser le pied.
        let leg = |sg: f64| (-sg * s * amp, w * ground * (bend * (sg * c).max(0.0).powf(1.4) + 0.12 + 0.12 * (sg * s).max(0.0)));
        let (mut hip_l, mut knee_l) = leg(1.0); let (mut hip_r, mut knee_r) = leg(-1.0);
        for (h, k) in [(&mut hip_l, &mut knee_l), (&mut hip_r, &mut knee_r)] { *h += -0.95 * low - 0.34 * land; *k += 1.7 * low + 0.68 * land; }
        hip_l += a * (-1.1 * rise - 0.4 * (1.0 - rise)); knee_l += a * (1.4 * rise + 0.7 * (1.0 - rise)); hip_r += a * (0.32 * rise - 0.28 * (1.0 - rise)); knee_r += a * (0.4 * rise + 0.95 * (1.0 - rise)); // saut : genou de tête relevé, jambes rassemblées en retombant
        let armv = s * (0.5 + 0.55 * run) * w * ground; let elbow_rest = 0.18 + 0.3 * w + 1.05 * run;
        let mut arm_l = armv + 0.08 - 1.0 * a * (0.5 + 0.5 * rise) - 0.3 * low; let mut elbow_l = -(elbow_rest * ground + 0.8 * a + 0.5 * low);
        let swing_a = if self.swing > 0.0 { -1.5 + cos(self.swing * 6.283) * 0.95 } else { 0.0 }; let punch_a = -sin(self.punch * pi) * 1.05;
        let mut arm_r = (if self.swing > 0.0 { swing_a } else { -0.2 - armv * 0.35 - 0.55 * a }) + punch_a; let mut elbow_r = if self.swing > 0.0 { -0.3 - 0.45 * sin(self.swing * pi) } else { -0.62 - 0.45 * run };
        let mut hip_y = 0.8 + c.abs() * 0.05 * w * ground - 0.27 * low - 0.15 * land + 0.03 * a; let mut lean = 0.05 * w + 0.26 * run + 0.46 * low + 0.16 * land - 0.08 * a * rise; let mut base = trs(p[0], p[1], p[2], yaw, 0.0, 1.0);
        if let Some((t, d)) = roll {
            // Roulade : le corps se ramasse, puis tourne d'un tour complet autour de son centre, dans le sens de la fuite.
            let tuck = (t * 6.0).min(1.0).min((1.0 - t) * 6.0); let e = t * t * (3.0 - 2.0 * t); let mix = |x: f64, y: f64| x + (y - x) * tuck;
            hip_l = mix(hip_l, -1.85); hip_r = mix(hip_r, -1.7); knee_l = mix(knee_l, 2.25); knee_r = mix(knee_r, 2.1); arm_l = mix(arm_l, -1.5); arm_r = mix(arm_r, -1.35); elbow_l = mix(elbow_l, -1.7); elbow_r = mix(elbow_r, -1.5);
            hip_y = mix(hip_y, 0.52); lean = mix(lean, 0.95 * d[1].abs().max(0.3)); let k = 0.5;
            base = mul(&base, &mul(&trs(0.0, k, 0.0, 0.0, 0.0, 1.0), &mul(&crate::view::rot_axis([0.0; 3], [d[1], 0.0, -d[0]], e * 6.2832, 1.0), &trs(0.0, -k, 0.0, 0.0, 0.0, 1.0))));
        }
        let t = mul(&base, &trs(0.0, hip_y, 0.0, 0.0, lean, 1.0)); let mut out = Vec::with_capacity(12); let up_y = 0.56;
        let part = |n: &str| md.parts[n]; let mut put = |n: &str, m: M4| out.push(Draw { m, r: part(n), view: false, local: false, hero: true, dynamic: false });
        put("torso", t); put("head", mul(&t, &trs(0.0, 0.66, 0.0, 0.0, -lean * 0.75 - pitch * 0.5, 1.0)));
        for (x, h, k) in [(0.105, hip_l, knee_l), (-0.105, hip_r, knee_r)] { let m = mul(&base, &trs(x, hip_y, 0.0, 0.0, h, 1.0)); put("thigh", m); put("shin", mul(&m, &trs(0.0, -THIGH, 0.0, 0.0, k, 1.0))); }
        let ul = mul(&t, &trs(0.3, up_y, 0.0, 0.0, arm_l, 1.0)); put("upper", ul); put("fore", mul(&ul, &trs(0.0, -UPPER, 0.0, 0.0, elbow_l, 1.0)));
        let ur = mul(&t, &trs(-0.3, up_y, 0.02, 0.0, arm_r, 1.0)); put("upper", ur); let fr = mul(&ur, &trs(0.0, -UPPER, 0.0, 0.0, elbow_r, 1.0)); put("fore", fr);
        if let Some(h) = held { out.push(Draw { m: mul(&mul(&fr, &trs(0.0, UPPER, 0.0, 0.0, 0.0, 1.0)), &hold_m(h.hold)), r: h.r, view: false, local: h.local, hero: true, dynamic: false }); }
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
