//! Dessins des objets qui ne sont pas des blocs : une image 56 × 56 décrite par des formes, extrudée ensuite en petits
//! voxels (actors.rs). Chaque famille d'objets a sa silhouette, chaque matière sa palette : un outil de cuivre a la forme
//! de celui de bronze, avec une autre couleur.
use crate::data::def;
type C = [f64; 3];
const R2: f64 = std::f64::consts::FRAC_1_SQRT_2;
const WOOD: [C; 3] = [[0.3, 0.17, 0.08], [0.48, 0.29, 0.14], [0.66, 0.43, 0.22]];
const BIND: [C; 2] = [[0.7, 0.64, 0.38], [0.52, 0.5, 0.27]];
/// Palette de la tête d'un outil : sombre, moyen, clair, fil.
fn metal(m: &str) -> [C; 4] {
    match m {
        "silex" => [[0.18, 0.2, 0.24], [0.3, 0.32, 0.37], [0.44, 0.46, 0.5], [0.66, 0.68, 0.72]],
        "cuivre" => [[0.46, 0.22, 0.11], [0.7, 0.37, 0.19], [0.87, 0.54, 0.31], [0.98, 0.78, 0.55]],
        "bronze" => [[0.42, 0.3, 0.11], [0.64, 0.48, 0.2], [0.81, 0.65, 0.33], [0.95, 0.86, 0.55]],
        "fer" => [[0.4, 0.39, 0.38], [0.62, 0.61, 0.6], [0.84, 0.83, 0.82], [1.0, 0.99, 0.97]],
        // Acier : bleuté et plus sombre que le fer ; pièce de maître : acier damassé, reflets chauds.
        "acier" => [[0.24, 0.27, 0.32], [0.42, 0.46, 0.53], [0.64, 0.69, 0.76], [0.9, 0.94, 1.0]],
        "maitre" => [[0.2, 0.2, 0.24], [0.4, 0.38, 0.44], [0.66, 0.6, 0.56], [0.98, 0.9, 0.7]],
        _ => [[0.33, 0.32, 0.31], [0.46, 0.45, 0.43], [0.6, 0.59, 0.56], [0.72, 0.7, 0.66]],
    }
}
const TH: f64 = 0.03; const THIN: f64 = 0.02; const FAT: f64 = 0.05;
fn wood(p: f64) -> (C, f64) { (if p > 0.012 { WOOD[2] } else if p < -0.012 { WOOD[0] } else { WOOD[1] }, TH) }
fn shaft(s: f64, p: f64, a: f64, e: f64, w: f64) -> Option<(C, f64)> { if s > a && s < e && p.abs() < w { Some(wood(p)) } else { None } }
fn shade(h: &[C; 4], t: f64) -> (C, f64) { (h[(t.clamp(0.0, 0.999) * 3.0) as usize], THIN) }
fn disc(x: f64, y: f64, cx: f64, cy: f64, rx: f64, ry: f64) -> f64 { let (dx, dy) = ((x - cx) / rx, (y - cy) / ry); dx * dx + dy * dy }
/// La silhouette d'un outil selon son type, avec la palette de sa matière.
fn tool(kind: &str, h: &[C; 4], x: f64, y: f64) -> Option<(C, f64)> {
    let (s, p) = ((x + y) * R2, (y - x) * R2);
    match kind {
        "hache" => { let (s2, p2) = (s, (x - y) * R2); if p2 > -0.02 && p2 < 0.33 { let half = 0.07 + 0.17 * (p2 / 0.33).max(0.0).powf(1.4); if (s2 - 0.88).abs() < half { let e = 0.33 - p2; return Some(if e < 0.035 { (h[3], 0.012) } else { shade(h, 0.2 + 0.6 * ((s2 - 0.88) / half * 0.5 + 0.5)) }); } } shaft(s, p, 0.08, 1.02, 0.036) }
        "pioche" => { let (dx, dy) = (x - 0.34, y - 0.34); let r = crate::det::hypot2(dx, dy); let t = (dy.atan2(dx) - std::f64::consts::FRAC_PI_4) / 1.0;
            if t.abs() < 1.0 { let w = 0.02 + 0.065 * (1.0 - t * t); let d = r - 0.62; if d.abs() < w { return Some(if t.abs() > 0.78 { (h[3], 0.012) } else { shade(h, 0.5 + d / w * 0.45) }); } } shaft(s, p, 0.08, 0.9, 0.036) }
        "pelle" => { if s > 0.74 && s < 1.16 { let w = 0.11 * (1.0 - ((s - 0.74) / 0.42).powi(4) * 0.5); if p.abs() < w { return Some(if s > 1.1 { (h[3], 0.012) } else { shade(h, 0.3 + 0.5 * (p / w * 0.5 + 0.5)) }); } } shaft(s, p, 0.06, 0.8, 0.032) }
        "houe" => { if (s - 0.98).abs() < 0.05 && p > -0.04 && p < 0.26 { return Some(if p > 0.21 { (h[3], 0.012) } else { shade(h, 0.4 + (s - 0.98) * 4.0) }); } shaft(s, p, 0.06, 1.03, 0.032) }
        "marteau" => { if (s - 0.92).abs() < 0.1 && p.abs() < 0.17 { return Some(shade(h, 0.25 + 0.6 * (p / 0.17 * 0.5 + 0.5))); } shaft(s, p, 0.06, 0.86, 0.034) }
        "faucille" => { let d = disc(x, y, 0.6, 0.42, 0.3, 0.3).sqrt(); let a = (y - 0.42).atan2(x - 0.6); if (d - 1.0).abs() < 0.13 && a > -2.6 && a < 0.9 { return Some(if d > 1.07 { (h[3], 0.012) } else { shade(h, 0.3 + d * 0.3) }); } shaft(s, p, 0.06, 0.48, 0.035) }
        "couteau" => { if s > 0.52 && s < 1.12 { let w = 0.07 * ((1.12 - s) / 0.6).powf(0.7) + 0.008; if p > -w * 0.5 && p < w { return Some(if p > w * 0.55 { (h[3], 0.012) } else { shade(h, 0.4 + 0.3 * (s - 0.52)) }); } }
            if s > 0.12 && s <= 0.52 && p.abs() < 0.038 { return Some(if ((s * 50.0).floor() as i64) & 1 != 0 { (BIND[0], TH) } else { wood(p) }); } None }
        "epee" => { if s > 0.47 && s < 1.32 { let w = if s > 1.14 { 0.058 * (1.32 - s) / 0.18 } else { 0.058 }; if p.abs() < w { return Some(if p.abs() < 0.01 { (h[0], THIN) } else if p > 0.0 { (h[3], 0.012) } else { (h[1], THIN) }); } }
            if (s - 0.44).abs() < 0.034 && p.abs() < 0.18 { return Some((h[2], TH)); } if s > 0.08 && s < 0.41 && p.abs() < 0.034 { return Some((if (s * 44.0).floor() as i64 & 1 != 0 { [0.2, 0.11, 0.06] } else { [0.36, 0.2, 0.1] }, TH)); }
            if crate::det::hypot2(s - 0.05, p) < 0.05 { return Some((h[2], TH)); } None }
        "lance" => { if s > 1.08 && s < 1.36 { let w = 0.07 * (1.36 - s) / 0.28; if p.abs() < w { return Some(if p > w * 0.4 { (h[3], 0.012) } else { shade(h, 0.4) }); } }
            if s > 1.02 && s <= 1.1 && p.abs() < 0.04 { return Some((BIND[((s * 80.0) as i64 & 1) as usize], TH)); } shaft(s, p, 0.0, 1.06, 0.026) }
        _ => None,
    }
}
/// Un vêtement selon l'emplacement, avec deux couleurs et un motif.
fn garment(slot: &str, a: C, b: C, x: f64, y: f64) -> Option<(C, f64)> {
    let stripe = |v: f64| if ((v * 24.0).floor() as i64) & 1 != 0 { a } else { b };
    let inside = match slot {
        "tete" => disc(x, y, 0.5, 0.56, 0.28, 0.22) < 1.0 && y < 0.6 || ((y - 0.6).abs() < 0.05 && (x - 0.5).abs() < 0.38),
        "torse" => { let body = (x - 0.5).abs() < 0.2 && y > 0.25 && y < 0.85; let arms = y > 0.25 && y < 0.5 && (x - 0.5).abs() < 0.38; let neck = disc(x, y, 0.5, 0.25, 0.09, 0.06) < 1.0; (body || arms) && !neck }
        "jambes" => { y > 0.18 && y < 0.86 && (x - 0.5).abs() < 0.2 && !(y > 0.42 && (x - 0.5).abs() < 0.035) }
        "pieds" => { (x > 0.2 && x < 0.42 && y > 0.25 && y < 0.78) || (x > 0.2 && x < 0.66 && y > 0.66 && y < 0.8) }
        // Bouclier : un écu, large en haut, en pointe en bas, avec un umbo au centre et un cerclage.
        "main" => { let half = if y < 0.55 { 0.32 } else { 0.32 * (1.0 - (y - 0.55) / 0.34).max(0.0) }; (x - 0.5).abs() < half && y > 0.16 && y < 0.89 }
        "dos" => { let bag = (x - 0.5).abs() < 0.22 && y > 0.26 && y < 0.84; let strap = (x - 0.5).abs() < 0.12 && y > 0.14 && y <= 0.26 && (x - 0.5).abs() > 0.06; bag || strap }
        _ => false,
    };
    if !inside { return None; } let edge = (slot == "dos" && (y - 0.46).abs() < 0.03) || (slot == "torse" && (y - 0.8).abs() < 0.03) || (slot == "main" && ((x - 0.5).hypot(y - 0.46) < 0.07 || y < 0.2)); Some((if edge { [a[0] * 0.6, a[1] * 0.6, a[2] * 0.6] } else { stripe(x + y * 0.3) }, FAT))
}
pub fn look(name: &str, x: f64, y: f64) -> Option<(C, f64)> {
    let d = def(name)?;
    if let Some(t) = d.tool { return tool(t.kind, &metal(t.mat.name), x, y); }
    if let Some(w) = d.wear {
        let slot = match w.slot { crate::data::Slot::Head => "tete", crate::data::Slot::Torso => "torse", crate::data::Slot::Legs => "jambes", crate::data::Slot::Feet => "pieds", crate::data::Slot::Back => "dos", crate::data::Slot::Hand => "main" };
        let (a, b): (C, C) = if name.starts_with("wooden") { ([0.5, 0.34, 0.18], [0.62, 0.44, 0.24]) } else if name.starts_with("fur") { ([0.62, 0.48, 0.32], [0.78, 0.66, 0.5]) } else if name.starts_with("leather") || name == "backpack" { ([0.44, 0.27, 0.13], [0.56, 0.36, 0.19]) } else if name.starts_with("steel") { ([0.32, 0.36, 0.42], [0.52, 0.57, 0.64]) } else if name.starts_with("iron") { ([0.5, 0.5, 0.52], [0.7, 0.7, 0.72]) } else if name == "straw_hat" { ([0.78, 0.66, 0.36], [0.66, 0.54, 0.28]) } else { ([0.58, 0.56, 0.32], [0.46, 0.46, 0.24]) };
        return garment(slot, a, b, x, y);
    }
    let (s, p) = ((x + y) * R2, (y - x) * R2);
    match name {
        "copper_ingot" | "tin_ingot" | "bronze_ingot" | "iron_ingot" | "steel_ingot" => { let h = metal(match name { "copper_ingot" => "cuivre", "bronze_ingot" => "bronze", "iron_ingot" => "fer", "steel_ingot" => "acier", _ => "fer" }); let h = if name == "tin_ingot" { [[0.5, 0.52, 0.55], [0.68, 0.7, 0.73], [0.82, 0.84, 0.86], [0.95, 0.96, 0.97]] } else { h };
            let top = y > 0.38 && y < 0.5 && x > 0.24 && x < 0.76; let side = y >= 0.5 && y < 0.66 && x > 0.18 && x < 0.82; if top { return Some((h[3], FAT)); } if side { return Some((h[1 + ((x * 6.0) as usize & 1)], FAT)); } None }
        // Arc : une branche courbée et sa corde ; flèche : hampe en diagonale, pointe de silex, empennage ; canne : longue tige, fil et bouchon.
        "bow" => { let (dx, dy) = (x - 0.18, y - 0.82); let r = (dx * dx + dy * dy).sqrt(); if (r - 0.62).abs() < 0.035 && dx > 0.0 && dy < 0.0 { return Some((if (r - 0.62) > 0.0 { [0.62, 0.42, 0.22] } else { [0.44, 0.28, 0.14] }, FAT)); } let (s, p) = ((x + y) * R2, (y - x) * R2); if p.abs() < 0.008 && s > 0.3 && s < 1.12 && (x - y).abs() > 0.0 { return Some(([0.9, 0.88, 0.8], THIN)); } return None; }
        "arrow" => { let (s, p) = ((x + y) * R2, (y - x) * R2); if s > 1.02 && s < 1.24 && p.abs() < 0.05 * (1.24 - s) / 0.22 + 0.005 { return Some(([0.32, 0.34, 0.38], TH)); } if s > 0.24 && s <= 1.02 && p.abs() < 0.016 { return Some(([0.6, 0.45, 0.25], TH)); } if s > 0.14 && s < 0.38 && p.abs() < 0.07 && p.abs() > 0.012 { return Some(([0.8, 0.74, 0.46], THIN)); } return None; }
        "fishing_rod" => { let (s, p) = ((x + y) * R2, (y - x) * R2); if s > 0.12 && s < 1.25 && p.abs() < 0.02 - 0.008 * s { return Some((if s < 0.45 { [0.4, 0.26, 0.13] } else { [0.62, 0.46, 0.24] }, TH)); } if (x - 0.86).abs() < 0.004 && y > 0.18 && y < 0.62 { return Some(([0.85, 0.85, 0.85], THIN)); } let (bx, by) = (x - 0.86, y - 0.66); if bx * bx + by * by < 0.0016 { return Some((if by < 0.0 { [0.9, 0.12, 0.08] } else { [0.95, 0.94, 0.9] }, FAT)); } return None; }
        "trout" | "perch" | "char" | "cooked_fish" => {
            // Un poisson de profil : corps fuselé, queue en éventail, œil ; la robe dépend de l'espèce, le poisson grillé est doré.
            let (body, back, belly): (C, C, C) = match name { "trout" => ([0.6, 0.6, 0.5], [0.36, 0.4, 0.3], [0.86, 0.72, 0.68]), "perch" => ([0.6, 0.62, 0.3], [0.26, 0.34, 0.18], [0.86, 0.84, 0.6]), "char" => ([0.5, 0.52, 0.58], [0.28, 0.3, 0.38], [0.92, 0.5, 0.36]), _ => ([0.78, 0.55, 0.26], [0.56, 0.34, 0.14], [0.9, 0.74, 0.44]) };
            let (dx, dy) = ((x - 0.46) / 0.32, (y - 0.5) / 0.15); if dx * dx + dy * dy < 1.0 { if (x - 0.68).hypot(y - 0.46) < 0.025 { return Some(([0.05, 0.05, 0.05], FAT)); } let stripe = name == "perch" && ((x * 14.0).floor() as i64) & 1 == 0 && dy < 0.2; return Some((if stripe { back } else if dy < -0.35 { back } else if dy > 0.45 { belly } else { body }, FAT)); }
            if x < 0.2 && x > 0.04 && (y - 0.5).abs() < (0.2 - x) * 1.1 + 0.02 && (y - 0.5).abs() > (0.2 - x) * 0.2 { return Some((back, TH)); } return None;
        }
        "carrot" => { let (s, p) = ((x + y) * R2, (y - x) * R2); if s > 0.35 && s < 1.15 && p.abs() < 0.11 * (1.15 - s) / 0.8 + 0.01 { return Some((if ((s * 22.0).floor() as i64) & 1 == 0 { [0.92, 0.48, 0.12] } else { [0.84, 0.4, 0.1] }, FAT)); } if s > 0.12 && s <= 0.36 && p.abs() < 0.09 && ((p * 40.0).floor() as i64) & 1 == 0 { return Some(([0.3, 0.6, 0.2], TH)); } return None; }
        "potato" | "baked_potato" => { let (dx, dy) = ((x - 0.5) / 0.3, (y - 0.52) / 0.22); let d = dx * dx + dy * dy; if d < 1.0 { let base = if name == "potato" { [0.7, 0.56, 0.36] } else { [0.62, 0.4, 0.2] }; let eye = ((x * 9.0).fract() < 0.1 && (y * 7.0).fract() < 0.12) as i32 as f64; return Some(([base[0] * (1.0 - 0.3 * eye), base[1] * (1.0 - 0.3 * eye), base[2] * (1.0 - 0.3 * eye)], FAT)); } return None; }
        // Verre : un carreau bleuté, plus clair sur un bord, avec un reflet en biais.
        "glass" => { let (dx, dy) = ((x - 0.5).abs(), (y - 0.5).abs()); if dx > 0.3 || dy > 0.36 { return None; } let edge = dx > 0.26 || dy > 0.32; let glint = ((x - y) - 0.05).abs() < 0.035 && dx < 0.2; return Some((if edge { [0.5, 0.66, 0.74] } else if glint { [0.95, 0.98, 1.0] } else { [0.66, 0.82, 0.9] }, FAT)); }
        "coal" | "charcoal" | "flint" => { let (a, b, c): (C, C, C) = match name { "coal" => ([0.07, 0.07, 0.08], [0.16, 0.16, 0.18], [0.3, 0.3, 0.33]), "charcoal" => ([0.1, 0.08, 0.07], [0.2, 0.17, 0.15], [0.32, 0.27, 0.22]), _ => ([0.16, 0.18, 0.22], [0.28, 0.3, 0.35], [0.5, 0.52, 0.58]) };
            let r = disc(x, y, 0.5, 0.52, 0.3 + 0.03 * (y * 13.0).sin(), 0.24 + 0.03 * (x * 11.0).cos()); if r < 1.0 { return Some((if x + y < 0.85 && r > 0.4 { c } else if r < 0.5 { b } else { a }, FAT)); } None }
        "rope" => { let r = disc(x, y, 0.5, 0.5, 0.32, 0.32).sqrt(); let k = (r * 4.0).fract(); if r < 1.0 && r > 0.15 && k < 0.6 { return Some((BIND[((r * 4.0) as usize) & 1], FAT)); } None }
        "raw_hide" | "fur" => { let (dx, dy) = ((x - 0.5).abs(), (y - 0.5).abs()); if dx > 0.34 || dy > 0.28 || (dx > 0.26 && dy > 0.2) || (dx > 0.3 && dy < 0.06) || (dy > 0.24 && dx < 0.07) { return None; }
            let (a, b): (C, C) = if name == "fur" { ([0.6, 0.45, 0.3], [0.76, 0.62, 0.45]) } else { ([0.62, 0.42, 0.36], [0.76, 0.56, 0.48]) }; Some((if ((x * 9.0 + y * 3.0).sin()) > 0.3 { b } else { a }, FAT)) }
        "bone_needle" => { if s > 0.12 && s < 1.2 && p.abs() < 0.014 + 0.012 * (1.0 - s) { return Some(([0.92, 0.88, 0.78], THIN)); } if crate::det::hypot2(s - 0.18, p) < 0.035 && crate::det::hypot2(s - 0.18, p) > 0.015 { return Some(([0.86, 0.82, 0.7], THIN)); } None }
        "raw_brick" | "brick" => { let c: C = if name == "brick" { [0.66, 0.3, 0.2] } else { [0.58, 0.58, 0.6] }; if x > 0.2 && x < 0.8 && y > 0.36 && y < 0.64 { return Some((if y < 0.42 { [c[0] * 1.2, c[1] * 1.2, c[2] * 1.2] } else { c }, FAT)); } None }
        "bowl" | "mushroom_stew" | "stew" => { let bowl = y > 0.48 && disc(x, y, 0.5, 0.48, 0.34, 0.3) < 1.0; let food = y > 0.42 && y <= 0.5 && (x - 0.5).abs() < 0.3;
            if bowl { return Some((if y < 0.54 { WOOD[2] } else { WOOD[1] }, FAT)); } if food && name != "bowl" { return Some((if name == "stew" { [0.5, 0.26, 0.14] } else { [0.62, 0.5, 0.36] }, FAT)); } None }
        "waterskin" | "waterskin_raw" | "waterskin_clean" => { if disc(x, y, 0.5, 0.58, 0.26, 0.26) < 1.0 { return Some((if name == "waterskin" { [0.5, 0.33, 0.18] } else { [0.42, 0.26, 0.13] }, FAT)); } if (x - 0.5).abs() < 0.06 && y > 0.18 && y < 0.34 { return Some((WOOD[0], TH)); }
            if name == "waterskin_clean" && disc(x, y, 0.45, 0.52, 0.06, 0.06) < 1.0 { return Some(([0.5, 0.75, 0.95], FAT)); } None }
        "wheat" => { for k in 0..5 { let c = 0.38 + k as f64 * 0.06; if (x - c - (y - 0.5) * 0.1 * (k as f64 - 2.0)).abs() < 0.014 && y > 0.3 && y < 0.88 { return Some(([0.76, 0.66, 0.32], THIN)); } if disc(x, y, c - 0.02 * (k as f64 - 2.0), 0.24, 0.03, 0.09) < 1.0 { return Some(([0.88, 0.74, 0.36], FAT)); } } None }
        "jerky" => { for k in 0..3 { let c = 0.36 + k as f64 * 0.14; if (x - c).abs() < 0.05 && y > 0.22 + 0.04 * k as f64 && y < 0.8 { return Some((if (y * 14.0).sin() > 0.4 { [0.4, 0.15, 0.08] } else { [0.52, 0.22, 0.12] }, FAT)); } } None }
        "dried_berries" => { for (cx, cy) in [(0.4, 0.45), (0.56, 0.42), (0.48, 0.58), (0.62, 0.58), (0.36, 0.62)] { if disc(x, y, cx, cy, 0.07, 0.065) < 1.0 { return Some(([0.3, 0.12, 0.2], FAT)); } } None }
        "pine_nuts" => { for (cx, cy) in [(0.4, 0.44), (0.56, 0.4), (0.48, 0.56), (0.62, 0.56), (0.36, 0.6), (0.52, 0.68)] { if disc(x, y, cx, cy, 0.05, 0.035) < 1.0 { return Some(([0.86, 0.76, 0.56], FAT)); } } None }
        "bread" => { let r = disc(x, y, 0.5, 0.52, 0.32, 0.18); if r < 1.0 { return Some((if r > 0.6 && y < 0.52 { [0.56, 0.32, 0.14] } else if ((x * 10.0).floor() as i64 & 1) != 0 && y < 0.46 { [0.82, 0.62, 0.36] } else { [0.72, 0.48, 0.24] }, FAT)); } None }
        "bandage" => { let r = disc(x, y, 0.5, 0.5, 0.24, 0.24); if r < 1.0 { return Some((if r < 0.2 { [0.75, 0.73, 0.66] } else if ((r * 5.0) as i64 & 1) != 0 { [0.95, 0.93, 0.88] } else { [0.85, 0.83, 0.76] }, FAT)); } None }
        _ => { // de quoi ne jamais rester sans image : un petit paquet
            if (x - 0.5).abs() < 0.22 && (y - 0.5).abs() < 0.22 { return Some(([0.6, 0.55, 0.45], FAT)); } None }
    }
}
