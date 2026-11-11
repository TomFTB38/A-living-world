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
/// Le personnage un peu plus étoffé : 15 % plus large et plus épais, même taille.
const BULK: M4 = [1.15, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.15, 0.0, 0.0, 0.0, 0.0, 1.0];
/// Habits portés, par matière : fourrure, cuir, fer, acier, paille, bois, étoffe. Des coques un peu plus larges que le corps.
pub const FAMILIES: [&str; 7] = ["fur", "leather", "iron", "steel", "straw", "wooden", "cloth"];
pub fn family(id: &str) -> &'static str { FAMILIES.iter().find(|f| id.contains(*f)).copied().unwrap_or(if id == "backpack" { "leather" } else { "cloth" }) }
/// Le bouclier dans son repère : `u` en largeur, `v` en hauteur, `w` en épaisseur (la face peinte vers -w, la poignée vers +w).
/// Des planches de teintes un peu différentes, joints sombres, cerclage riveté, umbo bombé, sangle et poignée ; un écu à la pointe arrondie.
/// `map` place chaque boîte dans le repère du modèle (de dos en première personne, au bras en troisième).
fn shield_mesh(q: &mut VBuf, a: C, d: C, metal: bool, map: &dyn Fn([f64; 6]) -> [f64; 6]) {
    let k = 1.22; let mut bx_ = |bb: [f64; 6], c: C| { let t = map([bb[0] * k, bb[1] * k, bb[2], bb[3] * k, bb[4] * k, bb[5]]); b(q, t, c); };
    let rows: [(f64, f64, f64); 3] = [(-0.16, 0.26, 0.2), (-0.3, -0.16, 0.14), (-0.38, -0.3, 0.06)];
    bx_([-0.196, -0.156, -0.008, 0.196, 0.256, 0.008], d); bx_([-0.136, -0.296, -0.008, 0.136, -0.16, 0.008], d); bx_([-0.056, -0.376, -0.008, 0.056, -0.3, 0.008], d);
    for i in 0..5 { let (x0, x1) = (-0.2 + i as f64 * 0.08 + 0.0035, -0.2 + (i + 1) as f64 * 0.08 - 0.0035); let t = [1.0, 0.88, 1.07, 0.93, 1.03][i]; let c = if metal { [a[0] * (0.96 + 0.04 * t), a[1] * (0.96 + 0.04 * t), a[2] * (0.96 + 0.04 * t)] } else { [a[0] * t, a[1] * t, a[2] * t] };
        for (y0, y1, w) in rows { let (l, r) = (x0.max(-w), x1.min(w)); if r > l { bx_([l, y0, -0.015, r, y1, 0.015], c); } } }
    let rim = [d[0] * 0.8, d[1] * 0.8, d[2] * 0.8]; let rv = [d[0] * 1.5, d[1] * 1.5, d[2] * 1.5];
    bx_([-0.21, 0.245, -0.022, 0.21, 0.275, 0.022], rim); bx_([-0.21, -0.16, -0.022, -0.18, 0.26, 0.022], rim); bx_([0.18, -0.16, -0.022, 0.21, 0.26, 0.022], rim);
    bx_([-0.15, -0.3, -0.022, -0.12, -0.16, 0.022], rim); bx_([0.12, -0.3, -0.022, 0.15, -0.16, 0.022], rim); bx_([-0.07, -0.39, -0.022, 0.07, -0.36, 0.022], rim);
    for (x, y) in [(-0.195, 0.2), (-0.195, 0.05), (-0.195, -0.1), (0.195, 0.2), (0.195, 0.05), (0.195, -0.1), (-0.1, 0.26), (0.0, 0.26), (0.1, 0.26), (-0.135, -0.24), (0.135, -0.24)] { bx_([x - 0.009, y - 0.009, -0.027, x + 0.009, y + 0.009, 0.027], rv); }
    // Face avant : l'umbo de métal bombé ; au dos, la sangle et la poignée.
    bx_([-0.07, -0.02, -0.05, 0.07, 0.12, -0.015], rv); bx_([-0.045, 0.005, -0.065, 0.045, 0.095, -0.05], [rv[0] * 1.1, rv[1] * 1.1, rv[2] * 1.1]);
    bx_([-0.18, 0.13, 0.015, 0.18, 0.16, 0.03], [0.32, 0.2, 0.11]);
    bx_([-0.022, -0.05, 0.015, 0.022, 0.15, 0.045], [0.3, 0.19, 0.11]); bx_([-0.026, -0.03, 0.015, 0.026, -0.01, 0.05], [0.2, 0.13, 0.08]); bx_([-0.026, 0.11, 0.015, 0.026, 0.13, 0.05], [0.2, 0.13, 0.08]);
}
fn outfit_parts() -> Vec<(&'static str, VBuf)> {
    const SLOT: [&str; 8] = ["head", "torso", "arm", "thigh", "shin", "back", "shield", "fpshield"];
    let mut o = Vec::new();
    for f in FAMILIES {
        let (a, d): (C, C) = match f { "fur" => ([0.6, 0.47, 0.32], [0.78, 0.68, 0.52]), "leather" => ([0.44, 0.27, 0.14], [0.3, 0.18, 0.09]), "iron" => ([0.6, 0.6, 0.62], [0.4, 0.4, 0.42]), "steel" => ([0.5, 0.55, 0.62], [0.32, 0.35, 0.4]), "straw" => ([0.82, 0.7, 0.4], [0.68, 0.56, 0.3]), "wooden" => ([0.55, 0.37, 0.19], [0.32, 0.22, 0.12]), _ => ([0.5, 0.52, 0.38], [0.36, 0.38, 0.27]) };
        let metal = f == "iron" || f == "steel"; let hi = [(a[0] * 1.18).min(1.0), (a[1] * 1.18).min(1.0), (a[2] * 1.18).min(1.0)]; let dark = [d[0] * 0.7, d[1] * 0.7, d[2] * 0.7];
        for slot in SLOT {
            let mut m = VBuf::new(300); let q = &mut m;
            match slot {
                // Tous les couvre-chefs enveloppent la chevelure (qui monte à 0,5 et déborde de 0,215 devant et derrière).
                "head" => if f == "straw" { b(q, [-0.2, 0.3, -0.21, 0.2, 0.52, 0.22], a); b(q, [-0.34, 0.3, -0.35, 0.34, 0.335, 0.36], d); b(q, [-0.205, 0.335, -0.215, 0.205, 0.375, 0.225], [0.36, 0.18, 0.12]); }
                    else if metal { b(q, [-0.21, 0.2, -0.225, 0.21, 0.47, 0.232], a); b(q, [-0.17, 0.47, -0.185, 0.17, 0.53, 0.195], hi); b(q, [-0.215, 0.18, -0.23, 0.215, 0.24, 0.237], d);
                        b(q, [-0.022, 0.05, 0.232, 0.022, 0.24, 0.25], d); for sx in [-1.0, 1.0] { let (x0, x1) = if sx < 0.0 { (-0.222, -0.195) } else { (0.195, 0.222) }; b(q, [x0, 0.03, -0.06, x1, 0.2, 0.17], a); } b(q, [-0.205, 0.06, -0.245, 0.205, 0.22, -0.215], d);
                        b(q, [-0.012, 0.47, -0.19, 0.012, 0.545, 0.2], d); }
                    else if f == "fur" { b(q, [-0.22, 0.27, -0.235, 0.22, 0.53, 0.245], a); b(q, [-0.228, 0.23, -0.243, 0.228, 0.33, 0.253], d); b(q, [-0.24, 0.02, -0.12, -0.2, 0.28, 0.12], d); b(q, [0.2, 0.02, -0.12, 0.24, 0.28, 0.12], d); }
                    else { b(q, [-0.205, 0.25, -0.222, 0.205, 0.47, 0.23], a); b(q, [-0.165, 0.47, -0.18, 0.165, 0.515, 0.19], a); b(q, [-0.21, 0.23, -0.226, 0.21, 0.285, 0.234], d); for x in [-0.08, 0.0, 0.08] { b(q, [x - 0.008, 0.29, 0.23, x + 0.008, 0.46, 0.236], d); } },
                "torso" => {
                    b(q, [-0.238, -0.02, -0.148, 0.238, 0.6, 0.148], a); b(q, [-0.225, -0.2, -0.142, 0.225, -0.0, 0.142], d); b(q, [-0.242, -0.02, -0.152, 0.242, 0.04, 0.152], dark);
                    if metal { b(q, [-0.19, 0.18, 0.148, 0.19, 0.56, 0.168], hi); b(q, [-0.012, 0.18, 0.168, 0.012, 0.56, 0.174], d); for sx in [-1.0, 1.0] { let (x0, x1) = if sx < 0.0 { (-0.36, -0.19) } else { (0.19, 0.36) }; b(q, [x0, 0.48, -0.165, x1, 0.62, 0.165], a); b(q, [x0, 0.44, -0.168, x1, 0.49, 0.168], d); }
                        for y in [-0.16, -0.1, -0.04] { b(q, [-0.228, y, -0.146, 0.228, y + 0.035, 0.146], a); } for (x, y) in [(-0.15, 0.53), (0.15, 0.53), (-0.15, 0.22), (0.15, 0.22)] { b(q, [x - 0.012, y - 0.012, 0.168, x + 0.012, y + 0.012, 0.176], dark); } }
                    else if f == "fur" { b(q, [-0.26, 0.5, -0.17, 0.26, 0.66, 0.17], d); b(q, [-0.24, -0.22, -0.155, 0.24, -0.14, 0.155], d); b(q, [-0.06, 0.04, 0.148, 0.06, 0.5, 0.156], d); }
                    else if f == "leather" { b(q, [-0.018, 0.08, 0.148, 0.018, 0.56, 0.155], dark); for y in [0.14, 0.24, 0.34, 0.44] { b(q, [-0.045, y, 0.148, 0.045, y + 0.015, 0.156], [0.62, 0.52, 0.36]); } b(q, [-0.2, 0.52, -0.155, 0.2, 0.61, 0.155], d); }
                    else { b(q, [-0.1, 0.42, 0.148, 0.1, 0.6, 0.154], d); }
                }
                // Manches ou spalières sur le haut du bras, au-dessus de la bretelle.
                "arm" => { b(q, [-0.096, -UPPER - 0.02, -0.1, 0.096, 0.062, 0.1], a); if metal { b(q, [-0.104, -UPPER - 0.04, -0.106, 0.104, -UPPER + 0.03, 0.106], d); b(q, [-0.1, -0.08, -0.104, 0.1, -0.05, 0.104], d); } else if f == "fur" { b(q, [-0.1, -UPPER - 0.035, -0.104, 0.1, -UPPER + 0.02, 0.104], d); } else { b(q, [-0.099, -UPPER - 0.03, -0.103, 0.099, -UPPER + 0.0, 0.103], d); } }
                "thigh" => { b(q, [-0.112, -THIGH - 0.03, -0.117, 0.112, 0.035, 0.117], a); if metal { b(q, [-0.106, -0.32, 0.117, 0.106, -0.04, 0.132], hi); b(q, [-0.118, -THIGH - 0.06, -0.06, 0.118, -THIGH + 0.04, 0.135], d); } else { b(q, [-0.116, -0.02, -0.12, 0.116, 0.035, 0.12], d); } }
                "shin" => { b(q, [-0.112, -0.43, -0.12, 0.112, -0.12, 0.124], a); b(q, [-0.106, -0.43, 0.1, 0.106, -0.31, 0.218], a); b(q, [-0.11, -0.445, -0.125, 0.11, -0.415, 0.223], dark); b(q, [-0.117, -0.17, -0.126, 0.117, -0.11, 0.13], d);
                    if metal { b(q, [-0.09, -0.38, 0.124, 0.09, -0.16, 0.138], hi); } }
                "back" => { b(q, [-0.19, 0.06, -0.38, 0.19, 0.56, -0.13], a); b(q, [-0.15, 0.1, -0.41, 0.15, 0.32, -0.37], d); }
                "fpshield" => shield_mesh(q, a, d, metal, &|bb| bb),
                // Au bras, la face peinte vers l'extérieur (+x), la poignée contre l'avant-bras.
                _ => shield_mesh(q, a, d, metal, &|bb| [0.13 - bb[5], bb[1] - 0.06, bb[0], 0.13 - bb[2], bb[4] - 0.06, bb[3]]),
            }
            o.push((Box::leak(format!("wear_{slot}_{f}").into_boxed_str()) as &'static str, m));
        }
    }
    o
}
fn thigh() -> VBuf { let mut o = VBuf::new(60); b(&mut o, [-0.1, -THIGH - 0.03, -0.105, 0.1, 0.02, 0.105], PANTS); b(&mut o, [-0.103, -0.26, -0.03, 0.103, -0.13, 0.108], PANTS2); o }
fn shin() -> VBuf { let mut o = VBuf::new(100); b(&mut o, [-0.09, -0.18, -0.09, 0.09, 0.02, 0.095], PANTS); b(&mut o, [-0.105, -0.22, -0.105, 0.105, -0.13, 0.11], BOOT); b(&mut o, [-0.098, -0.37, -0.1, 0.098, -0.2, 0.105], BOOT); b(&mut o, [-0.098, -0.4, -0.11, 0.098, -0.32, 0.2], BOOT); b(&mut o, [-0.102, -0.42, -0.115, 0.102, -0.395, 0.205], SOLE); o }
fn upper() -> VBuf { let mut o = VBuf::new(60); b(&mut o, [-0.082, -UPPER - 0.03, -0.085, 0.082, 0.03, 0.085], TUNIC); b(&mut o, [-0.09, -0.03, -0.095, 0.09, 0.05, 0.095], STRAP); o }
fn fore() -> VBuf { let mut o = VBuf::new(60); b(&mut o, [-0.075, -0.09, -0.078, 0.075, 0.03, 0.078], TUNIC2); b(&mut o, [-0.062, -0.21, -0.065, 0.062, -0.09, 0.065], SKIN); b(&mut o, [-0.072, -0.235, -0.074, 0.072, -0.185, 0.074], LEATHER2); b(&mut o, [-0.066, -0.295, -0.07, 0.066, -0.23, 0.07], SKIN);
    // Les doigts, un peu repliés vers l'avant, et le pouce.
    for k in 0..4 { let x = -0.058 + k as f64 * 0.03; b(&mut o, [x, -0.355, -0.03, x + 0.026, -0.29, 0.06], if k % 2 == 0 { SKIN } else { SKIN2 }); b(&mut o, [x + 0.002, -0.37, 0.02, x + 0.024, -0.345, 0.07], SKIN2); }
    b(&mut o, [-0.02, -0.3, 0.065, 0.024, -0.24, 0.095], SKIN2); b(&mut o, [-0.016, -0.33, 0.07, 0.02, -0.29, 0.1], SKIN); o }
fn fist() -> VBuf {
    // Le bras vu en première personne : manche à revers, brassard de cuir sanglé, poignet, main serrée (phalanges, pouce).
    let mut o = VBuf::new(400); let q = &mut o;
    b(q, [-0.072, -0.3, -0.072, 0.072, 0.0, 0.072], SHIRT); b(q, [-0.074, -0.12, -0.074, 0.074, -0.1, 0.074], SHIRT2); b(q, [-0.078, -0.33, -0.078, 0.078, -0.29, 0.078], SHIRT2);
    b(q, [-0.064, -0.46, -0.066, 0.064, -0.33, 0.066], LEATHER2); for y in [-0.36, -0.43] { b(q, [-0.067, y - 0.012, -0.069, 0.067, y + 0.012, 0.069], BELT); } b(q, [0.062, -0.442, -0.016, 0.071, -0.418, 0.016], BUCKLE);
    b(q, [-0.05, -0.5, -0.053, 0.05, -0.46, 0.053], SKIN2);
    b(q, [-0.058, -0.6, -0.056, 0.058, -0.5, 0.056], SKIN);
    for k in 0..4 { let x = -0.052 + k as f64 * 0.027; let t = if k % 2 == 0 { SKIN } else { SKIN2 };
        b(q, [x, -0.655, -0.04, x + 0.024, -0.585, 0.064], t); b(q, [x + 0.002, -0.6, 0.056, x + 0.022, -0.58, 0.068], SKIN2); }
    b(q, [0.054, -0.6, 0.0, 0.08, -0.52, 0.05], SKIN2); b(q, [0.046, -0.62, 0.03, 0.074, -0.58, 0.07], SKIN);
    o
}

// Outils : image 84 × 84 décrite par des formes, puis extrudée en petits voxels. Couleurs : indices de PAL.
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
    // Les outils, l'arc, la flèche et la canne sont dessinés en diagonale depuis le bas ; tout le reste l'est comme une image, du haut vers le bas.
    let upright = matches!(name, "bow" | "arrow" | "fishing_rod") || crate::data::def(name).is_some_and(|d| d.tool.is_some());
    crate::icons::look(name, x, if upright { y } else { 1.0 - y })
}
fn tool_mesh(name: &str) -> VBuf {
    // L'image est relue sur une grille tournée de 45° : les outils sont dessinés en diagonale, leurs lames et manches
    // tombent ainsi le long des rangées et gardent des bords droits, sans marches d'escalier. Puis le tout est retourné.
    let mut o = VBuf::new(8000); o.scale = 2048.0; const N: usize = 84; const E: f64 = 0.0032; /* un léger recouvrement : les arrondis ne laissent pas de fentes */ let s = 0.86; let px = s / N as f64; let grip = 0.2 * R2 * s;
    let (ns, np) = ((N as f64 * 1.4143).ceil() as usize, (N as f64 * 1.4143).ceil() as usize);
    let at = |i: usize, j: usize| { let (u, v) = ((i as f64 + 0.5) / N as f64, (j as f64 + 0.5) / N as f64 - 0.7072); look(name, (u - v) * R2, (u + v) * R2) };
    for j in 0..np {
        let mut run: Option<(usize, (C, f64))> = None;
        for i in 0..=ns {
            let c = if i < ns { at(i, j) } else { None };
            if let Some((a, (col, th))) = run { if c != Some((col, th)) {
                bx(&mut o, a as f64 * px - E, j as f64 * px - 0.7072 * s - E, -th, i as f64 * px + E, (j + 1) as f64 * px - 0.7072 * s + E, th, col[0], col[1], col[2], 0.0, 255.0, 0.0, kind::FLAT, true, 0.0, 255.0);
                run = None;
            } }
            if let (Some(k), None) = (c, run) { run = Some((i, k)); }
        }
    }
    for v in o.data.chunks_exact_mut(VSIZE) {
        let g = |k: usize| i16::from_le_bytes([v[k], v[k + 1]]) as f64 / 2048.0; let (u, w, z) = (g(0), g(2), g(4));
        let (x, y) = ((u - w) * R2 - grip, (u + w) * R2 - grip);
        for (k, val) in [(0, x), (2, y), (4, z)] { v[k..k + 2].copy_from_slice(&((val * 128.0).round() as i16).to_le_bytes()); }
        let (nx, ny) = (v[8] as i8 as f64, v[9] as i8 as f64); v[8] = ((nx - ny) * R2).round() as i8 as u8; v[9] = ((nx + ny) * R2).round() as i8 as u8;
    }
    o.scale = 128.0; o
}
fn cube_mesh(id: crate::blocks::Id) -> VBuf { let mut o = VBuf::new(32); cube_at(&mut o, id, [0.0; 3]); o }
/// Un bloc complet, posé à `at` : le mode « tenu » lit la texture dans ces coordonnées, sans dépendre du monde.
pub fn cube_at(o: &mut VBuf, id: crate::blocks::Id, at: [f64; 3]) {
    let bk = BLOCKS[id as usize];
    if id == B::WATER { bx(o, 0.0, 0.0, 0.0, 1.0, 0.9, 1.0, 0.3, 0.55, 0.75, 0.0, 255.0, 0.0, kind::FLAT, true, 0.0, 255.0); return; }
    let kd = if bk.shape == Shape::Leaf { kind::LEAF } else { kind::TEX }; let t = bk.tint;
    let mut q = |di: usize, pts: [[f64; 3]; 4], n: [f64; 3]| { o.rot = bk.grain[di]; for p in pts { o.v(p[0] + at[0], p[1] + at[1], p[2] + at[2], bk.dir_faces[di] as f64, n[0], n[1], n[2], 0.0, 255.0, 255.0, 0.0, kd, t[0], t[1], t[2]); } o.rot = 0; };
    q(1, [[0., 1., 1.], [1., 1., 1.], [1., 1., 0.], [0., 1., 0.]], [0., 1., 0.]); q(2, [[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]], [0., -1., 0.]);
    q(3, [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]], [0., 0., 1.]); q(3, [[1., 0., 0.], [0., 0., 0.], [0., 1., 0.], [1., 1., 0.]], [0., 0., -1.]);
    q(0, [[1., 0., 1.], [1., 0., 0.], [1., 1., 0.], [1., 1., 1.]], [1., 0., 0.]); q(0, [[0., 0., 0.], [0., 0., 1.], [0., 1., 1.], [0., 1., 0.]], [-1., 0., 0.]);
}
/// Une icône d'équipement tirée de ce que l'on porte vraiment : le casque, la cuirasse et ses manches, deux jambières, deux bottes,
/// le sac, le bouclier de face. Ramenée dans le cube unité, posée au sol, comme un objet décoratif.
fn wear_icon(parts: &[(&'static str, VBuf)], slot: crate::data::Slot, f: &str) -> Option<VBuf> {
    use crate::data::Slot;
    let get = |n: &str| parts.iter().find(|(k, _)| *k == format!("wear_{n}_{f}")).map(|(_, v)| v);
    let pieces: Vec<(&VBuf, [f64; 3])> = match slot {
        Slot::Head => vec![(get("head")?, [0.0; 3])],
        Slot::Torso => vec![(get("torso")?, [0.0; 3]), (get("arm")?, [0.31, 0.56, 0.0]), (get("arm")?, [-0.31, 0.56, 0.0])],
        Slot::Legs => vec![(get("thigh")?, [0.14, 0.0, 0.0]), (get("thigh")?, [-0.14, 0.0, 0.0])],
        Slot::Feet => vec![(get("shin")?, [0.15, 0.0, 0.0]), (get("shin")?, [-0.15, 0.0, 0.0])],
        Slot::Back => vec![(get("back")?, [0.0; 3])],
        Slot::Hand => vec![(get("fpshield")?, [0.0; 3])],
    };
    let rd = |v: &[u8], k: usize| i16::from_le_bytes([v[k], v[k + 1]]) as f64 / 128.0;
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for (m, d) in &pieces { for v in m.data.chunks_exact(VSIZE) { for k in 0..3 { let x = rd(v, k * 2) + d[k]; lo[k] = lo[k].min(x); hi[k] = hi[k].max(x); } } }
    let sc = 0.86 / (hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2]).max(0.01); let c = [(lo[0] + hi[0]) * 0.5, lo[1], (lo[2] + hi[2]) * 0.5];
    let mut o = VBuf::new(0);
    for (m, d) in &pieces { let mut data = m.data.clone();
        // Le bouclier tourne d'un demi-tour : on voit sa face peinte et son umbo.
        let turn = slot == Slot::Hand;
        for v in data.chunks_exact_mut(VSIZE) { for k in 0..3 { let mut x = (rd(v, k * 2) + d[k] - c[k]) * sc; if turn && k != 1 { x = -x; } x += if k == 1 { 0.02 } else { 0.5 }; let q = ((x * 128.0).round() as i16).to_le_bytes(); v[k * 2] = q[0]; v[k * 2 + 1] = q[1]; }
            if turn { for k in [8usize, 10] { v[k] = (v[k] as i8).wrapping_neg() as u8; } } }
        o.data.extend_from_slice(&data); o.n += m.n; }
    Some(o)
}
impl Models {
    pub fn build() -> Self {
        let mut all = VBuf::new(5000); let mut parts = HashMap::new();
        for (n, m) in [("torso", torso()), ("head", head()), ("thigh", thigh()), ("shin", shin()), ("upper", upper()), ("fore", fore()), ("fist", fist())] { let r = take(&mut all, m); parts.insert(n, r); }
        let outfits = outfit_parts(); let mut icons: HashMap<&'static str, VBuf> = HashMap::new();
        for name in order() { if let Some(w) = crate::data::def(name).and_then(|d| d.wear) { if let Some(o) = wear_icon(&outfits, w.slot, family(name)) { icons.insert(name, o); } } }
        for (n, m) in outfits { let r = take(&mut all, m); parts.insert(n, r); }
        let char_verts = all.n; let mut items = HashMap::new(); let mv = Mesher::new();
        for name in order() {
            let it = item(name).unwrap(); let (o, hold) = match it.block {
                None if icons.contains_key(name) => (icons.remove(name).unwrap(), Hold::Deco),
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
#[derive(Clone, Copy, Debug)] pub struct Draw { pub m: M4, pub r: Range, pub view: bool, pub local: bool, pub hero: bool, pub dynamic: bool, /** coup reçu, de 0 à 1 : la bête rougit */ pub hurt: f32 }
pub(crate) fn hold_m(h: Hold) -> M4 { match h { Hold::Tool => mul(&trs(0.0, -0.58, 0.02, -std::f64::consts::FRAC_PI_2, 0.0, 1.0), &roll_z(-std::f64::consts::FRAC_PI_4)), Hold::Block => trs(-0.14, -0.76, -0.14, 0.0, 0.0, 0.28), Hold::Deco => trs(-0.28, -0.72, -0.1, 0.0, 0.0, 0.55) } }
#[derive(Default, Clone, Copy)] pub struct AnimIn { pub speed: f64, pub ground: bool, pub vy: f64, pub crouch: bool, pub fly: bool, pub mining: bool, pub guard: bool }
#[derive(Default)] pub struct Anim { /** arc bandé, de 0 à 1 */ pub bow: f64, phase: f64, walk: f64, run: f64, low: f64, air: f64, rise: f64, pub swing: f64, punch: f64, landed: f64, /** garde levée, de 0 à 1 */ pub guard: f64 }
impl Anim {
    pub fn update(&mut self, dt: f64, s: AnimIn) {
        let ease = |c: f64, t: f64, k: f64| c + (t - c) * (dt * k).min(1.0); let was = self.air;
        self.walk = ease(self.walk, (s.speed / 4.5).min(1.0), 10.0); self.run = ease(self.run, if s.speed > 5.2 { 1.0 } else { 0.0 }, 8.0); self.low = ease(self.low, if s.crouch { 1.0 } else { 0.0 }, 13.0);
        self.air = ease(self.air, if s.ground || s.fly { 0.0 } else { 1.0 }, 12.0); self.rise = ease(self.rise, if s.vy > 0.0 { 1.0 } else { 0.0 }, 12.0);
        if was > 0.5 && self.air <= 0.5 { self.landed = 1.0; } self.guard = ease(self.guard, if s.guard { 1.0 } else { 0.0 }, 16.0);
        self.landed = (self.landed - dt * 5.0).max(0.0);
        if s.speed > 0.2 { self.phase += dt * (3.4 + s.speed * 1.35); }
        if s.mining { self.swing = (self.swing + dt * 3.3) % 1.0; } else if self.swing > 0.0 { self.swing += dt * 3.3; if self.swing >= 1.0 { self.swing = 0.0; } }
        self.punch = (self.punch - dt * 4.6).max(0.0);
    }
    pub fn trigger(&mut self) { self.punch = 1.0; }
    /// Troisième personne. Hanches et épaules : négatif vers l'avant. Genoux : positif plie vers l'arrière ; coudes : négatif plie vers l'avant.
    /// `outfit` : la matière de ce qui est porté à la tête, au torse, aux jambes, aux pieds, au dos et au bras (voir `family`).
    pub fn third(&self, md: &Models, p: [f64; 3], yaw: f64, pitch: f64, held: Option<Held>, roll: Option<(f64, [f64; 2])>, outfit: [Option<&'static str>; 6]) -> Vec<Draw> {
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
        let g = self.guard; arm_l += (-1.25 - arm_l) * g; elbow_l += (-1.35 - elbow_l) * g; // garde : l'avant-bras gauche et le bouclier devant la poitrine
        let mut hip_y = 0.8 + c.abs() * 0.05 * w * ground - 0.27 * low - 0.15 * land + 0.03 * a; let mut lean = 0.05 * w + 0.26 * run + 0.46 * low + 0.16 * land - 0.08 * a * rise; let mut base = mul(&trs(p[0], p[1], p[2], yaw, 0.0, 1.0), &BULK);
        if let Some((t, d)) = roll {
            // Roulade : le corps se ramasse, puis tourne d'un tour complet autour de son centre, dans le sens de la fuite.
            let tuck = (t * 6.0).min(1.0).min((1.0 - t) * 6.0); let e = t * t * (3.0 - 2.0 * t); let mix = |x: f64, y: f64| x + (y - x) * tuck;
            hip_l = mix(hip_l, -1.85); hip_r = mix(hip_r, -1.7); knee_l = mix(knee_l, 2.25); knee_r = mix(knee_r, 2.1); arm_l = mix(arm_l, -1.5); arm_r = mix(arm_r, -1.35); elbow_l = mix(elbow_l, -1.7); elbow_r = mix(elbow_r, -1.5);
            hip_y = mix(hip_y, 0.52); lean = mix(lean, 0.95 * d[1].abs().max(0.3)); let k = 0.5;
            base = mul(&base, &mul(&trs(0.0, k, 0.0, 0.0, 0.0, 1.0), &mul(&crate::view::rot_axis([0.0; 3], [d[1], 0.0, -d[0]], e * 6.2832, 1.0), &trs(0.0, -k, 0.0, 0.0, 0.0, 1.0))));
        }
        let t = mul(&base, &trs(0.0, hip_y, 0.0, 0.0, lean, 1.0)); let mut out = Vec::with_capacity(12); let up_y = 0.56;
        let part = |n: &str| md.parts[n]; let mut put = |n: &str, m: M4| out.push(Draw { m, r: part(n), view: false, local: false, hero: true, dynamic: false, hurt: 0.0 });
        let wear = |slot: usize, n: &str| outfit[slot].map(|f| format!("wear_{n}_{f}"));
        let hd = mul(&t, &trs(0.0, 0.66, 0.0, 0.0, -lean * 0.75 - pitch * 0.5, 1.0)); put("torso", t); put("head", hd);
        if let Some(n) = wear(0, "head") { put(&n, hd); } if let Some(n) = wear(1, "torso") { put(&n, t); } if let Some(n) = wear(4, "back") { put(&n, t); }
        for (x, h, k) in [(0.105, hip_l, knee_l), (-0.105, hip_r, knee_r)] { let m = mul(&base, &trs(x, hip_y, 0.0, 0.0, h, 1.0)); put("thigh", m); let sh = mul(&m, &trs(0.0, -THIGH, 0.0, 0.0, k, 1.0)); put("shin", sh);
            if let Some(n) = wear(2, "thigh") { put(&n, m); } if let Some(n) = wear(3, "shin") { put(&n, sh); } }
        let ul = mul(&t, &trs(0.3, up_y, 0.0, 0.0, arm_l, 1.0)); put("upper", ul); if let Some(n) = wear(1, "arm") { put(&n, ul); } let fl = mul(&ul, &trs(0.0, -UPPER, 0.0, 0.0, elbow_l, 1.0)); put("fore", fl); if let Some(n) = wear(5, "shield") { put(&n, fl); }
        let ur = mul(&t, &trs(-0.3, up_y, 0.02, 0.0, arm_r, 1.0)); put("upper", ur); if let Some(n) = wear(1, "arm") { put(&n, ur); } let fr = mul(&ur, &trs(0.0, -UPPER, 0.0, 0.0, elbow_r, 1.0)); put("fore", fr);
        if let Some(h) = held { out.push(Draw { m: mul(&mul(&fr, &trs(0.0, UPPER, 0.0, 0.0, 0.0, 1.0)), &hold_m(h.hold)), r: h.r, view: false, local: h.local, hero: true, dynamic: false, hurt: 0.0 }); }
        out
    }
    /// Première personne : poing si la main est vide, bloc seul s'il y en a un, bras et outil sinon.
    /// `shield` : la matière du bouclier porté au bras gauche (voir `family`), visible en bas à gauche et levé en garde.
    pub fn first(&self, md: &Models, view: &M4, held: Option<Held>, shield: Option<&'static str>) -> Vec<Draw> {
        let s = sin(self.phase); let c = cos(self.phase * 0.5); let w = self.walk * (1.0 - self.air); let pi = std::f64::consts::PI; let v = mul(&invert(view), &trs(0.0, 0.0, 0.0, pi, 0.0, 1.0));
        let strike = if self.swing > 0.0 { sin(self.swing * pi) } else { 0.0 }; let push = sin(self.punch * pi); let r = md.parts["fist"]; let mut out = Vec::new();
        let bx_ = c * 0.022 * w - 0.05 * self.run; let by = -s.abs() * 0.022 * w - 0.05 * self.low + 0.07 * self.air * (self.rise - 0.5);
        let hand = |m: M4| Draw { m, r, view: true, local: false, hero: true, dynamic: false, hurt: 0.0 };
        match held {
            None => { let hit = strike.max(push); out.push(hand(mul(&v, &trs(-0.27 + bx_ + 0.08 * hit, -0.33 + by + 0.05 * hit, 0.2 + 0.2 * hit, 0.34 - 0.16 * hit, -2.06 - 0.3 * hit, 0.46)))); }
            // L'arc bandé : il se redresse au milieu de la vue, la flèche encochée recule avec la corde, la main tremble à fond.
            Some(h) if h.hold == Hold::Tool && self.bow > 0.0 => {
                let b = self.bow; let shake = if b >= 1.0 { 0.004 * sin(self.phase * 23.0 + 1.0) } else { 0.0 };
                let arm = mul(&v, &trs(-0.25 + 0.13 * b + bx_ * 0.3 + shake, -0.27 + 0.06 * b + by * 0.3, 0.1 + 0.08 * b, 0.22 - 0.2 * b, -1.72 + 0.22 * b, 0.46));
                out.push(hand(arm)); out.push(Draw { m: mul(&arm, &mul(&hold_m(Hold::Tool), &mul(&roll_z(std::f64::consts::FRAC_PI_4 * b), &trs(0.0, 0.0, 0.0, 0.0, 0.0, 0.8)))), r: h.r, view: true, local: h.local, hero: true, dynamic: false, hurt: 0.0 });
                out.push(Draw { m: mul(&v, &trs(-0.07 + shake, -0.16 + 0.02 * b, 0.62 - 0.22 * b, 0.0, 0.0, 1.0)), r: md.parts["arrow"], view: true, local: false, hero: true, dynamic: false, hurt: 0.0 });
            }
            Some(h) if h.hold == Hold::Tool => {
                let arm = mul(&v, &trs(-0.25 + bx_, -0.27 + by - 0.06 * strike, 0.1 + 0.2 * push + 0.12 * strike, 0.22 + 0.3 * strike, -1.72 + 0.22 * self.run + 0.85 * strike - 0.3 * push, 0.46));
                out.push(hand(arm)); out.push(Draw { m: mul(&arm, &mul(&hold_m(Hold::Tool), &trs(0.0, 0.0, 0.0, 0.0, 0.0, 0.8))), r: h.r, view: true, local: h.local, hero: true, dynamic: false, hurt: 0.0 });
            }
            Some(h) => {
                let sc = if h.hold == Hold::Block { 0.26 } else { 0.46 };
                let base = mul(&v, &trs(-0.29 + bx_ + 0.05 * strike, -0.2 + by - 0.1 * strike + 0.03 * push, 0.52 + 0.12 * push + 0.08 * strike, 0.62 + 0.25 * strike, 0.26 + 0.6 * strike - 0.3 * push, sc));
                out.push(Draw { m: mul(&base, &trs(-0.5, if h.hold == Hold::Block { -0.5 } else { -0.4 }, -0.5, 0.0, 0.0, 1.0)), r: h.r, view: true, local: h.local, hero: true, dynamic: false, hurt: 0.0 });
            }
        }
        if let Some(f) = shield {
            // Au repos, le bord du bouclier en bas à gauche ; en garde, il remonte au milieu et couvre une part de la vue.
            let g = self.guard; let lerp = |a: f64, b: f64| a + (b - a) * g;
            // Au repos, sur la gauche, à moitié dans le champ ; en garde, il passe devant soi, un peu à gauche, et couvre le buste.
            let m = mul(&v, &trs(lerp(0.46, 0.15) + bx_ * 0.6, lerp(-0.47, -0.2) + by, lerp(0.5, 0.6), lerp(-0.62, -0.1), lerp(0.38, 0.04), lerp(0.95, 1.0)));
            out.push(Draw { m, r: md.parts[format!("wear_fpshield_{f}").as_str()], view: true, local: false, hero: true, dynamic: false, hurt: 0.0 });
        }
        out
    }
}
#[allow(dead_code)] fn _r(x: f64) -> f64 { round(x) }
