//! Indicateurs de survie : cœur, estomac, goutte, poumons, thermomètre, œil. Pas de jauges rectangulaires :
//! chaque silhouette est remplie exactement à la valeur du besoin, ligne de pixels par ligne de pixels, avec un contour
//! sombre, un liquide qui ondule, un battement ou une respiration. Un gestionnaire commun décide de ce qui se montre.
use crate::hud::Ui;
use alw_core::session::Session;

/// Les six indicateurs, dans l'ordre de l'écran.
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Need { Health, Hunger, Thirst, Stamina, Temperature, Fatigue }
pub const ALL: [Need; 6] = [Need::Health, Need::Hunger, Need::Thirst, Need::Stamina, Need::Temperature, Need::Fatigue];
/// Mode d'affichage : seulement en cas d'action ou de danger, plus tôt quand un besoin baisse, ou en permanence.
pub const MODES: [&str; 3] = ["Immersif", "Dynamique", "Permanent"];
pub struct Opts { pub mode: usize, pub reduce: bool, pub alt: bool }
type C = [f32; 4];

/// La forme : vrai si le point (u, v) dans [-1, 1]² (v vers le bas) est dans la silhouette.
fn inside(n: Need, u: f32, v: f32) -> bool {
    let e = |u: f32, v: f32, cx: f32, cy: f32, rx: f32, ry: f32| { let (a, b) = ((u - cx) / rx, (v - cy) / ry); a * a + b * b <= 1.0 };
    match n {
        Need::Health => { let (x, y) = (u * 1.18, -v * 1.18 + 0.18); let a = x * x + y * y - 1.0; a * a * a - x * x * y * y * y <= 0.0 }
        // Estomac : une poche en haricot, l'œsophage qui descend à gauche, le pylore qui part à droite.
        Need::Hunger => { let body = e(u, v, 0.08, 0.2, 0.72, 0.62) && !e(u, v, -0.42, -0.18, 0.34, 0.3); let tube = u > -0.5 && u < -0.24 && v > -0.95 && v < 0.0; let exit = e(u, v, 0.72, 0.62, 0.22, 0.16); body || tube || exit }
        Need::Thirst => { let r = 0.64; if e(u, v, 0.0, 0.3, r, r) { return true; } v > -0.96 && v < 0.3 && u.abs() <= (v + 0.96) / 1.26 * r }
        // Poumons : deux lobes, la trachée et les bronches.
        Need::Stamina => { let lobe = |s: f32| { let (x, y) = (u * s - 0.46, v - 0.18); e(x, y, 0.0, 0.0, 0.4, 0.74) && x > -0.36 }; let trachea = u.abs() < 0.08 && v > -0.98 && v < -0.18; let bronchi = v > -0.24 && v < -0.1 && u.abs() < 0.3; lobe(1.0) || lobe(-1.0) || trachea || bronchi }
        Need::Temperature => { e(u, v, 0.0, 0.64, 0.34, 0.32) || (u.abs() < 0.17 && v > -0.92 && v < 0.5) || e(u, v, 0.0, -0.92, 0.17, 0.08) }
        Need::Fatigue => { u.abs() < 0.98 && v.abs() <= 0.56 * (1.0 - u * u).max(0.0).powf(0.75) }
    }
}
/// Couleurs : contenu plein, contenu vide, contour.
fn colors(n: Need, s: &Session, fill: f32) -> (C, C, C) {
    let rim = [0.05, 0.04, 0.04, 0.95];
    match n {
        Need::Health => ([0.74, 0.08, 0.1, 1.0], [0.2, 0.06, 0.06, 0.55], rim),
        Need::Hunger => ([0.82, 0.55, 0.26, 1.0], [0.2, 0.14, 0.08, 0.55], rim),
        Need::Thirst => ([0.27, 0.58, 0.92, 1.0], [0.07, 0.12, 0.2, 0.55], rim),
        Need::Stamina => if s.winded { ([0.62, 0.58, 0.5, 1.0], [0.16, 0.14, 0.12, 0.55], rim) } else { ([0.88, 0.5, 0.5, 1.0], [0.22, 0.1, 0.1, 0.55], rim) },
        Need::Temperature => { let x = s.exposure as f32; let c = if x < -3.0 { [0.4, 0.66, 0.98, 1.0] } else if x > 3.0 { if x > 8.0 { [0.92, 0.2, 0.12, 1.0] } else { [0.95, 0.56, 0.2, 1.0] } } else { [0.82, 0.78, 0.74, 1.0] }; let _ = fill; (c, [0.12, 0.12, 0.14, 0.55], rim) }
        Need::Fatigue => ([0.92, 0.9, 0.86, 1.0], [0.48, 0.38, 0.34, 1.0], rim),
    }
}
/// Valeur normalisée du besoin, de 0 (vide, danger) à 1 (plein, bien).
pub fn value(n: Need, s: &Session) -> f32 {
    let g = s.gauge;
    match n { Need::Health => g[0], Need::Hunger => g[1], Need::Thirst => g[2], Need::Stamina => g[3], Need::Temperature => g[4], Need::Fatigue => g[5] }.clamp(0.0, 1.0) as f32
}
/// L'état est-il critique (l'indicateur reste alors affiché) ?
pub fn critical(n: Need, s: &Session) -> bool {
    match n { Need::Health => s.vit[0] < 30.0, Need::Hunger => s.vit[1] < 20.0, Need::Thirst => s.vit[2] < 20.0, Need::Stamina => s.winded || s.stamina < 15.0, Need::Temperature => s.exposure.abs() > 8.0 || (s.body - 37.0).abs() > 1.5, Need::Fatigue => s.fatigue > 80.0 }
}
/// En mode dynamique, l'indicateur apparaît plus tôt.
fn low(n: Need, s: &Session) -> bool {
    match n { Need::Health => s.vit[0] < 70.0, Need::Hunger => s.vit[1] < 45.0, Need::Thirst => s.vit[2] < 45.0, Need::Stamina => s.stamina < 60.0, Need::Temperature => s.exposure.abs() > 3.0, Need::Fatigue => s.fatigue > 55.0 }
}
/// Opacité de l'indicateur : il apparaît quand son besoin change, s'efface après quelques secondes, reste en cas de danger.
pub fn visibility(n: Need, s: &Session, o: &Opts) -> f32 {
    if o.mode == 2 || o.alt || critical(n, s) { return 1.0; }
    if o.mode == 1 && low(n, s) { return 1.0; }
    let since = (s.elapsed - s.last_gain[n as usize]) as f32; (1.0 - (since - 4.0) / 1.2).clamp(0.0, 1.0)
}
/// Dessine un indicateur centré en (cx, cy), de demi-taille r. `t` : le temps, pour les animations (indépendantes de la cadence).
pub fn draw(u: &mut Ui, n: Need, s: &Session, cx: f32, cy: f32, r: f32, alpha: f32, o: &Opts) {
    let t = s.elapsed as f32; let val = value(n, s); let (full, empty, rim) = colors(n, s, val); let anim = !o.reduce;
    // Battement du cœur, plus rapide quand la vie baisse ; respiration des poumons après l'effort.
    let k = match n {
        Need::Health if anim => { let rate = 1.1 + 1.6 * (1.0 - val); let ph = (t * rate).fract(); 1.0 + 0.07 * (-(ph * 9.0)).exp() * (ph * 30.0).sin().abs() }
        Need::Stamina if anim && (s.elapsed - s.last_gain[3]) < 3.0 => 1.0 + 0.045 * (t * 4.2).sin(),
        _ => 1.0,
    };
    let alert = if critical(n, s) && anim { 0.75 + 0.25 * (t * 5.0).sin().abs() } else { 1.0 };
    let px = (2.0 * r).round().max(8.0) as i32; let step = 2.0 * r / px as f32; let inv = 1.0 / (r * k);
    let heat = n == Need::Temperature && s.exposure > 8.0 && anim; let frost = n == Need::Temperature && s.exposure < -8.0;
    let fill_v = 1.0 - 2.0 * val; // la ligne de remplissage, de bas en haut
    for row in 0..px {
        let y = cy - r + (row as f32 + 0.5) * step; let v = (y - cy) * inv;
        let mut run: Option<(i32, C)> = None;
        for col in 0..=px {
            let c = if col < px {
                let x = cx - r + (col as f32 + 0.5) * step; let mut uu = (x - cx) * inv; if heat { uu += 0.06 * (v * 9.0 + t * 6.0).sin(); }
                if !inside(n, uu, v) { None } else {
                    let d = 1.6 * step * inv; let edge = !inside(n, uu + d, v) || !inside(n, uu - d, v) || !inside(n, uu, v + d) || !inside(n, uu, v - d);
                    Some(if edge { rim } else { match n {
                        // Œil : la paupière descend avec la fatigue ; l'iris et la pupille restent au centre.
                        Need::Fatigue => { let lid = -0.56 + 1.12 * (1.0 - val); if v < lid { empty } else { let ri = (uu * uu + v * v).sqrt(); if ri < 0.14 { [0.05, 0.04, 0.04, 1.0] } else if ri < 0.33 { [0.32, 0.46, 0.36, 1.0] } else { full } } }
                        // Thermomètre : la colonne monte avec la chaleur ressentie ; le bulbe est toujours plein.
                        Need::Temperature => { let lvl = 0.5 - 1.3 * val; if v > 0.36 || v > lvl { full } else { empty } }
                        _ => { let wave = if anim && (n == Need::Thirst || n == Need::Hunger) { 0.045 * (uu * 6.0 + t * 2.6).sin() } else { 0.0 }; let lv = fill_v + wave; if v >= lv { let shade = if v > lv + 0.5 { 0.86 } else { 1.0 }; [full[0] * shade, full[1] * shade, full[2] * shade, full[3]] } else { empty } }
                    } })
                }
            } else { None };
            let same = match (run, c) { (Some((_, a)), Some(b)) => a == b, _ => false };
            if !same { if let Some((a, col_c)) = run { let x0 = cx - r + a as f32 * step; let x1 = cx - r + col as f32 * step; let cc = [col_c[0] * alert, col_c[1] * alert, col_c[2] * alert, col_c[3] * alpha]; u.rect(x0, y - step * 0.5, x1 - x0, step, cc); } run = c.map(|c| (col, c)); }
        }
    }
    // Givre : quelques cristaux blancs autour du thermomètre quand le froid devient dangereux.
    if frost { for i in 0..6 { let a = i as f32 * 1.047 + t * 0.2 * anim as i32 as f32; let (fx, fy) = (cx + a.cos() * r * 0.95, cy + a.sin() * r * 0.95); let w = r * 0.12; u.rect(fx - w * 0.5, fy - w * 0.5, w, w, [0.9, 0.96, 1.0, 0.8 * alpha]); } }
}
/// Le gestionnaire commun : la rangée compacte en bas à gauche, seulement ce qui doit se voir, dans un ordre fixe.
pub fn row(u: &mut Ui, s: &Session, h: f32, k: f32, o: &Opts) {
    let r = 17.0 * k; let gap = 9.0 * k; let mut x = 18.0 * k + r; let y = h - 30.0 * k - r;
    for n in ALL {
        let a = visibility(n, s, o); if a <= 0.01 { continue; }
        draw(u, n, s, x, y, r, a, o); x += 2.0 * r + gap;
    }
}
