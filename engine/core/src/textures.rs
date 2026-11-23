//! Matériaux procéduraux périodiques : albédo, normale et rugosité. Aucun fichier image.
use crate::blocks::{MATERIALS, VARIANTS};
use crate::det::{hypot2, sin};
use crate::noise::{clamp, lerp, smooth, Rng};

pub const TEX: f64 = 64.0;
type C = [f64; 3];
#[derive(Clone, Copy)] pub struct S { c: C, h: f64, r: f64 }

const fn hd(b: u8) -> u32 { if b >= b'a' { (b - b'a') as u32 + 10 } else { (b - b'0') as u32 } }
const fn hx(s: &str) -> C { let b = s.as_bytes(); [((hd(b[1]) * 16 + hd(b[2])) as f64) / 255.0, ((hd(b[3]) * 16 + hd(b[4])) as f64) / 255.0, ((hd(b[5]) * 16 + hd(b[6])) as f64) / 255.0] }
macro_rules! pl { ($($h:expr),*) => { [$(hx($h)),*] } }

struct Tile { n: f64, p: Vec<f32>, /** la variante (0 à VARIANTS - 1) : certaines matières en font des faciès différents */ v: usize }
impl Tile {
    fn new(seed: u32) -> Self { let mut r = Rng::new(seed); Tile { n: TEX, p: (0..1024).map(|_| r.next() as f32).collect(), v: 0 } }
    #[inline] fn pi(&self, i: i64) -> f64 { self.p[(i & 1023) as usize] as f64 }
    fn val(&self, x: f64, y: f64, f: i64) -> f64 {
        let ff = f as f64; let (fx, fy) = (x * ff / self.n, y * ff / self.n); let (xf, yf) = (fx.floor(), fy.floor()); let (a, b) = (fx - xf, fy - yf); let (xi, yi) = (xf as i64, yf as i64);
        let h = |i: i64, j: i64| self.pi(i.rem_euclid(f) * 37 + j.rem_euclid(f) * 101);
        let (u, v) = (a * a * (3.0 - 2.0 * a), b * b * (3.0 - 2.0 * b));
        lerp(lerp(h(xi, yi), h(xi + 1, yi), u), lerp(h(xi, yi + 1), h(xi + 1, yi + 1), u), v)
    }
    fn fbm(&self, x: f64, y: f64, f: i64, o: usize) -> f64 { let (mut s, mut a, mut n, mut f) = (0.0, 1.0, 0.0, f); for _ in 0..o { s += a * self.val(x, y, f); n += a; a *= 0.5; f *= 2; } s / n }
    /// Voronoï périodique : F1, F2 et identifiant de cellule.
    fn vor(&self, x: f64, y: f64, f: i64) -> (f64, f64, i64) {
        let c = self.n / f as f64; let (xi, yi) = ((x / c).floor() as i64, (y / c).floor() as i64);
        let (mut d1, mut d2, mut id) = (1e9, 1e9, 0);
        for j in -1..=1 { for i in -1..=1 {
            let (a, b) = (xi + i, yi + j); let k = (a.rem_euclid(f) * 31 + b.rem_euclid(f) * 67) & 1023;
            let px = (a as f64 + 0.15 + 0.7 * self.pi(k)) * c; let py = (b as f64 + 0.15 + 0.7 * self.pi(k + 211)) * c; let d = hypot2(px - x, py - y);
            if d < d1 { d2 = d1; d1 = d; id = k; } else if d < d2 { d2 = d; }
        } }
        (d1 / c, d2 / c, id)
    }
}
fn pal(a: &[C], t: f64) -> C { let t = clamp(t, 0.0, 0.9999) * (a.len() - 1) as f64; let i = t.floor() as usize; let f = t - i as f64; [lerp(a[i][0], a[i + 1][0], f), lerp(a[i][1], a[i + 1][1], f), lerp(a[i][2], a[i + 1][2], f)] }
fn mul(c: C, k: f64) -> C { [c[0] * k, c[1] * k, c[2] * k] }

const GRASS: [C; 4] = pl!("#556b2a", "#6f8834", "#8ba042", "#a9b658"); const DIRT: [C; 4] = pl!("#4d3019", "#6e4725", "#8a5f35", "#a57c4d");
const FLOW: [C; 4] = pl!("#9a5718", "#cf8a3c", "#ead2a8", "#fbf3e4");
const LIME_F: [C; 4] = pl!("#5f5341", "#97856a", "#c0ae8d", "#e2d6ba");
const LIME_P: [C; 4] = pl!("#a6916f", "#cdb995", "#e2d2b0", "#f1e6cc");
const STONE: [C; 4] = pl!("#5c564c", "#7f786b", "#a0998b", "#bfb8aa"); const LEAF: [C; 4] = pl!("#2f5518", "#4f7f25", "#86ac38", "#c6d862");
const MOSS: [C; 4] = pl!("#37561d", "#55792a", "#7a9c37", "#a6bd4f");
const OAK: [C; 4] = pl!("#4a2c14", "#7a4f29", "#a8743f", "#cf9d62"); const BEECH: [C; 4] = pl!("#726c60", "#938c7f", "#b0a99b", "#c9c3b5");
const BIRCH: [C; 3] = pl!("#d6d0bf", "#ece7da", "#fbf8f0"); const BIRCH_DARK: [C; 2] = pl!("#17140f", "#3e3428");
const OAK_W: [C; 4] = pl!("#5d3a1c", "#8e6133", "#bd8c52", "#dcb67e"); const BIRCH_W: [C; 4] = pl!("#a98a5d", "#cdb080", "#e6d3a8", "#f3e8c9"); const BEECH_W: [C; 4] = pl!("#8a6a48", "#b28f66", "#d0b08a", "#e4cdaa");
const SPRUCE: [C; 4] = pl!("#2f2219", "#4a3727", "#664c36", "#86694c"); const SPRUCE_W: [C; 4] = pl!("#7d5d3c", "#a17f57", "#c2a276", "#dcc39b");
const SNOW: [C; 3] = pl!("#c9d5e3", "#e3ebf3", "#f7fafd"); const ICE: [C; 3] = pl!("#7fa6c4", "#a6c6dd", "#d2e6f2");
const PLANKS: [C; 3] = pl!("#6c4524", "#94673a", "#b98a52"); const PATH: [C; 4] = pl!("#7c5b35", "#a07a4c", "#bc9663", "#d2b080");
const SOIL: [C; 4] = pl!("#2f2413", "#4a3a1f", "#5e5127", "#6f6a2f"); const SAND: [C; 4] = pl!("#a89268", "#c2ac7e", "#d2bf92", "#e0cfa6");

const DRY: [C; 4] = pl!("#8a7136", "#ad8f47", "#cbae63", "#e2cb8a"); const CRACKED: [C; 4] = pl!("#7a6248", "#98805f", "#b29b78", "#c8b493");
/// Herbe sèche de steppe : paille dorée, quelques touffes encore vertes, de la terre qui perce.
fn dry_top(t: &Tile, x: f64, y: f64) -> S {
    let (n, s, b) = (t.fbm(x, y, 4, 3), t.val(x, y, 32), t.val(x * 0.5 + 3.0, y, 24)); let green = smooth(0.68, 0.8, t.fbm(x + 17.0, y - 5.0, 3, 2)); let bare = smooth(0.66, 0.78, t.fbm(x - 9.0, y + 21.0, 5, 2));
    let mut c = pal(&DRY, 0.2 + 0.6 * n + 0.3 * (s - 0.5) + 0.22 * (b - 0.5)); c = [lerp(c[0], c[0] * 0.78, green), lerp(c[1], c[1] * 0.98, green), lerp(c[2], c[2] * 0.7, green)];
    if bare > 0.0 { let e = pal(&CRACKED, 0.25 + 0.5 * n); c = [lerp(c[0], e[0], bare), lerp(c[1], e[1], bare), lerp(c[2], e[2], bare)]; }
    S { c, h: 0.4 + 0.5 * s * (1.0 - bare), r: 0.8 }
}
fn dry_side(t: &Tile, x: f64, y: f64) -> S {
    let fringe = 9.0 + 7.0 * t.val(x, 0.0, 16) + 4.0 * t.val(x, 0.0, 32);
    if y < fringe { let mut g = dry_top(t, x, y); g.c = mul(g.c, 0.75 + 0.3 * (1.0 - y / fringe)); return g; }
    let mut d = dirt(t, x, y, &DIRT); let e = smooth(fringe + 4.0, fringe, y); d.c = mul(d.c, 1.0 - 0.3 * e); d
}
/// Terre sèche craquelée : des plaques pâles séparées de fentes sombres.
fn cracked(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x + 4.0 * t.fbm(x, y, 4, 2), y + 4.0 * t.fbm(x + 30.0, y, 4, 2), 7); let _ = d1; let crack = smooth(0.0, 0.06, d2 - d1); let (n, s) = (t.fbm(x, y, 6, 3), t.val(x, y, 32));
    let c = pal(&CRACKED, 0.3 + 0.3 * t.pi(id) + 0.4 * (n - 0.5) + 0.2 * (s - 0.5));
    S { c: mul(c, 0.5 + 0.5 * crack), h: 0.15 + 0.6 * crack + 0.15 * n, r: 0.9 }
}
const DEAD: [C; 4] = pl!("#4f4a44", "#726b62", "#958d81", "#b7afa2"); const DEAD_W: [C; 4] = pl!("#7e7466", "#9f9483", "#bdb29f", "#d4cab7"); const LICHEN: [C; 3] = pl!("#8b987a", "#a9b592", "#c7cfae");
/// Roche couverte de lichens : des plaques pâles vert-de-gris, quelques taches orangées.
fn lichen(t: &Tile, x: f64, y: f64) -> S {
    let st = stone(t, x, y, &STONE, 5); let (d1, _, id) = t.vor(x + 5.0, y + 11.0, 9); let m = t.fbm(x + 13.0, y - 7.0, 6, 3); let g = t.val(x, y, 32);
    let crust = m > 0.56 || (t.pi(id) > 0.72 && d1 < 0.32 + 0.1 * g); let orange = t.pi(id) > 0.93 && d1 < 0.22;
    if orange { return S { c: mul(pl!("#c98b3c")[0], 0.85 + 0.25 * g), h: 0.55, r: 0.85 }; }
    if crust { return S { c: pal(&LICHEN, 0.2 + 0.6 * g + 0.3 * (m - 0.56)), h: 0.5 + 0.3 * g, r: 0.9 }; } st
}
const ACACIA: [C; 4] = pl!("#3b322b", "#5a4b3f", "#7a6654", "#97836c"); const ACACIA_W: [C; 4] = pl!("#8a3f1f", "#ad5a2e", "#c87a45", "#dd9a63");
const BONE: [C; 4] = pl!("#a99d84", "#c6bba2", "#ddd3bd", "#efe8d8"); const NEEDLE: [C; 4] = pl!("#1b3624", "#2b5132", "#467446", "#76a274");
/// Os blanchis par le soleil : ivoire, pores fins, quelques fêlures.
fn bone(t: &Tile, x: f64, y: f64) -> S {
    let (n, s) = (t.fbm(x, y, 5, 3), t.val(x, y, 32)); let pore = if t.val(x * 1.3, y * 1.3, 48) > 0.8 { 0.18 } else { 0.0 }; let (d1, d2, _) = t.vor(x, y, 4); let _ = d1; let crack = smooth(0.03, 0.0, d2 - d1) * smooth(0.55, 0.7, t.fbm(x + 3.0, y, 3, 2));
    S { c: mul(pal(&BONE, 0.3 + 0.55 * n + 0.15 * (s - 0.5)), 1.0 - pore - 0.35 * crack), h: 0.5 + 0.3 * n - pore - 0.4 * crack, r: 0.6 }
}
/// Aiguilles d'épicéa : des rameaux croisés, chacun couvert d'aiguilles fines en arêtes de poisson, plus claires au bout.
fn needles(t: &Tile, x: f64, y: f64) -> S {
    const CELL: f64 = 16.0; let (cx, cy) = ((x / CELL).floor() as i64, (y / CELL).floor() as i64); let mut best: Option<(f64, f64, bool)> = None;
    for j in -1..=1 { for i in -1..=1 {
        let (gx, gy) = (cx + i, cy + j); let k = gx.rem_euclid(4) * 7 + gy.rem_euclid(4) * 13 + 3;
        let (px, py) = ((gx as f64 + 0.5 + 0.7 * (t.pi(k) - 0.5)) * CELL, (gy as f64 + 0.5 + 0.7 * (t.pi(k + 50) - 0.5)) * CELL); let a = t.pi(k + 100) * 6.283; let l = 10.0 + 7.0 * t.pi(k + 150);
        let (du, dv) = (x - px, y - py); let (u, v) = (du * a.cos() + dv * a.sin(), -du * a.sin() + dv * a.cos()); if u.abs() > l { continue; }
        let w = 6.5 * (1.0 - (u / l).powi(2)); if v.abs() > w { continue; }
        let twig = v.abs() < 0.6 && u < l * 0.85; let s2 = ((u + v.abs() * 1.1) / 2.1).rem_euclid(1.0); let needle = s2 < 0.45;
        if twig || needle { let pr = t.pi(k + 200); if best.is_none_or(|b| pr > b.0) { best = Some((pr, v.abs() / w.max(0.1), twig)); } }
    } }
    match best {
        Some((_, tip, true)) => { let _ = tip; S { c: pl!("#3d3024")[0], h: 0.55, r: 0.8 } }
        Some((_, tip, false)) => { let n = t.val(x, y, 16); S { c: pal(&NEEDLE, 0.15 + 0.6 * tip + 0.25 * (n - 0.5)), h: 0.45 + 0.5 * (1.0 - tip), r: 0.5 } }
        None => S { c: pl!("#0d1912")[0], h: 0.0, r: 0.9 },
    }
}
/// Cœur de bois mort mis à nu : des fibres longues dans le sens du fil, brun clair, fendues et piquées de pourriture plus sombre.
fn wood_inner(t: &Tile, x: f64, y: f64) -> S {
    let fib = t.fbm(x * 3.0, y * 0.35, 4, 3); let streak = t.val(x * 4.0, y * 0.2, 16); let rot = smooth(0.55, 0.72, t.fbm(x + 21.0, y - 8.0, 3, 3)); let split = smooth(0.06, 0.0, (t.val(x * 1.5, y * 0.12, 8) - 0.5).abs());
    let c = pal(&pl!("#5e4733", "#7d6248", "#9c7f60", "#b89c7c"), 0.25 + 0.5 * fib + 0.25 * (streak - 0.5)); let k = (1.0 - 0.45 * rot) * (1.0 - 0.5 * split);
    S { c: [c[0] * k, c[1] * k * 0.97, c[2] * k * 0.92], h: 0.4 + 0.3 * fib - 0.3 * split - 0.15 * rot, r: 0.92 }
}
/// Grès : des lits horizontaux de sable cimenté, ocre et beige, parfois obliques, au grain fin et poudreux.
fn sandstone(t: &Tile, x: f64, y: f64, base: [f64; 3], band: [f64; 3]) -> S {
    let warp = 3.0 * t.fbm(x, y, 3, 2); let cross = if t.fbm(x + 50.0, y, 2, 2) > 0.55 { x * 0.35 } else { 0.0 }; let bed = ((y + warp + cross) / 64.0 * 9.0).rem_euclid(1.0);
    let layer = t.pi(((y + warp + cross) / 64.0 * 9.0).floor() as i64 * 13 + 7); let grain = t.val(x, y, 64); let n = t.fbm(x, y, 5, 3);
    let k = 0.9 + 0.1 * layer + 0.06 * (grain - 0.5) + 0.06 * (n - 0.5); let line = smooth(0.0, 0.06, bed) * smooth(1.0, 0.94, bed);
    let c = [lerp(band[0], base[0], line) * k, lerp(band[1], base[1], line) * k, lerp(band[2], base[2], line) * k];
    S { c, h: 0.45 + 0.25 * line + 0.15 * (grain - 0.5), r: 0.92 }
}
fn sandstone_top(t: &Tile, x: f64, y: f64) -> S { let (n, g) = (t.fbm(x, y, 4, 3), t.val(x, y, 64)); let k = 0.92 + 0.1 * (n - 0.5) + 0.06 * (g - 0.5); S { c: [0.84 * k, 0.72 * k, 0.52 * k], h: 0.5 + 0.2 * (n - 0.5), r: 0.9 } }
/// Cactus : des côtes verticales vert sombre, les sillons plus foncés, des aréoles claires piquées d'épines.
fn cactus_side(t: &Tile, x: f64, _y: f64) -> S {
    let rib = (x / 64.0 * 8.0).rem_euclid(1.0); let ridge = (rib * std::f64::consts::PI).sin(); let n = t.fbm(x, _y, 6, 3);
    let areole = { let q = (_y / 64.0 * 10.0 + if (x / 8.0).floor() as i64 % 2 == 0 { 0.5 } else { 0.0 }).rem_euclid(1.0); (rib - 0.5).abs() < 0.08 && (q - 0.5).abs() < 0.06 };
    if areole { return S { c: [0.86, 0.84, 0.72], h: 1.0, r: 0.6 }; }
    let k = 0.55 + 0.45 * ridge + 0.08 * (n - 0.5); S { c: [0.2 * k, 0.42 * k, 0.2 * k], h: ridge, r: 0.55 }
}
fn cactus_top(t: &Tile, x: f64, y: f64) -> S { let (dx, dy) = (x - 31.5, y - 31.5); let a = dy.atan2(dx); let rib = (a * 8.0 / 6.283).rem_euclid(1.0); let ridge = (rib * std::f64::consts::PI).sin(); let r = hypot2(dx, dy) / 32.0; let k = (0.55 + 0.45 * ridge) * (1.0 - 0.3 * r) + 0.05 * (t.val(x, y, 32) - 0.5); S { c: [0.24 * k, 0.46 * k, 0.22 * k], h: ridge * (1.0 - r), r: 0.55 } }
/// Stipe de palmier : des écailles en losanges laissées par les palmes tombées, brun gris et fibreuses.
fn bark_palm(t: &Tile, x: f64, y: f64) -> S {
    let row = (y / 64.0 * 6.0).floor(); let off = if row as i64 % 2 == 0 { 0.0 } else { 0.5 }; let u = (x / 64.0 * 4.0 + off).rem_euclid(1.0); let v = (y / 64.0 * 6.0).rem_euclid(1.0);
    let scale = 1.0 - ((u - 0.5).abs() * 2.0 + (1.0 - v)) * 0.5; let fib = t.val(x * 3.0, y * 0.5, 32); let edge = smooth(0.0, 0.12, scale);
    let c = pal(&pl!("#4a3c2e", "#6b5845", "#8c7860", "#a8957a"), 0.25 + 0.5 * scale + 0.2 * (fib - 0.5)); S { c: mul(c, 0.6 + 0.4 * edge), h: scale, r: 0.9 }
}
fn end_palm(t: &Tile, x: f64, y: f64) -> S { let r = hypot2(x - 31.5, y - 31.5) / 32.0; let f = t.val(x * 2.0, y * 2.0, 64); let c = pal(&pl!("#7a6448", "#9a8160", "#b89e7a"), 0.3 + 0.4 * f - 0.3 * r); S { c, h: 0.5 + 0.3 * f, r: 0.85 } }
fn grass_top(t: &Tile, x: f64, y: f64) -> S {
    let (n, s, b) = (t.fbm(x, y, 4, 3), t.val(x, y, 32), t.val(x + 7.0, y * 0.45, 24));
    S { c: pal(&GRASS, 0.2 + 0.55 * n + 0.32 * (s - 0.5) + 0.25 * (b - 0.5)), h: 0.4 + 0.5 * s, r: 0.72 }
}
fn dirt(t: &Tile, x: f64, y: f64, a: &[C]) -> S {
    let (n, s) = (t.fbm(x, y, 5, 3), t.val(x, y, 32)); let (d1, d2, id) = t.vor(x, y, 12); let _ = d1; let rim = smooth(0.0, 0.1, d2 - d1); let pb = if t.pi(id) > 0.72 { rim } else { 0.0 };
    S { c: pal(a, 0.08 + 0.72 * n + 0.34 * (s - 0.5) - 0.22 * (1.0 - rim) * 0.5 + 0.34 * pb), h: 0.3 + 0.3 * n + 0.2 * s + 0.4 * pb, r: 0.86 }
}
fn grass_side(t: &Tile, x: f64, y: f64) -> S {
    let fringe = 13.0 + 9.0 * t.val(x, 0.0, 16) + 5.0 * t.val(x, 0.0, 32);
    if y < fringe { let mut g = grass_top(t, x, y); g.c = mul(g.c, 0.72 + 0.38 * (1.0 - y / fringe)); return g; }
    let mut d = dirt(t, x, y, &DIRT); let e = smooth(fringe + 5.0, fringe, y); d.c = mul(d.c, 1.0 - 0.35 * e); d
}
fn stone(t: &Tile, x: f64, y: f64, a: &[C], cells: i64) -> S {
    let (d1, d2, id) = t.vor(x, y, cells); let crack = smooth(0.0, 0.11, d2 - d1); let (n, s) = (t.fbm(x, y, 8, 3), t.val(x, y, 32));
    let tone = 0.34 + 0.32 * t.pi(id) + 0.55 * (n - 0.5) + 0.3 * (s - 0.5) + 0.12 * crack;
    S { c: pal(a, tone * (0.72 + 0.28 * crack)), h: 0.2 + 0.6 * crack + 0.2 * n, r: 0.8 }
}
/// Granit : une mosaïque serrée de cristaux de quelques millimètres — feldspath rose saumon et blanc laiteux, quartz gris vitreux,
/// mica noir en paillettes — sous une patine d'altération grise. Chaque cristal a sa teinte propre et des bords nets.
fn granite(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x + 0.6 * t.fbm(x, y, 16, 2), y + 0.6 * t.fbm(x + 5.0, y, 16, 2), 44); let m = t.pi(id); let v = t.pi(id + 501); let edge = smooth(0.0, 0.05, d2 - d1);
    let (c, h, r) = if m < 0.36 { ([0.78 + 0.08 * v, 0.6 + 0.07 * v, 0.53 + 0.06 * v], 0.62, 0.62) } // feldspath rose
        else if m < 0.6 { ([0.86 + 0.06 * v, 0.84 + 0.06 * v, 0.8 + 0.06 * v], 0.6, 0.6) } // feldspath blanc
        else if m < 0.86 { ([0.5 + 0.1 * v, 0.5 + 0.1 * v, 0.52 + 0.1 * v], 0.7, 0.3) } // quartz
        else { ([0.08 + 0.05 * v, 0.07 + 0.05 * v, 0.08 + 0.05 * v], 0.45, 0.45) }; // mica
    // Reflets internes et clivages fins dans chaque cristal, puis la patine grise qui ternit la surface exposée.
    let inner = 0.92 + 0.12 * t.val(x * 2.0 + v * 40.0, y * 2.0, 64); let patina = 0.25 + 0.2 * smooth(0.35, 0.75, t.fbm(x, y, 3, 3)); let dirt = 0.06 * (t.fbm(x + 30.0, y, 6, 3) - 0.5);
    let k = inner * (0.82 + 0.18 * edge); let gray = 0.58 + dirt;
    S { c: [lerp(c[0] * k, gray, patina), lerp(c[1] * k, gray * 0.99, patina), lerp(c[2] * k, gray * 0.98, patina)], h: h * (0.6 + 0.4 * edge), r }
}
/// Calcaire : une pierre crème, lisse, aux lits à peine marqués, tachée de gris et piquée de petites cavités.
fn limestone(t: &Tile, x: f64, y: f64) -> S { if t.v < 2 { limestone_fossil(t, x, y) } else { limestone_pale(t, x, y) } }
/// Calcaire coquillier : une pierre grise et beige, marbrée, criblée de pores ; çà et là un coquillage en relief
/// (une coquille Saint-Jacques à côtes, une ammonite enroulée), plus claire que la roche.
fn limestone_fossil(t: &Tile, x: f64, y: f64) -> S {
    let big = t.fbm(x, y, 2, 3); let mid = t.fbm(x + 41.0, y + 3.0, 6, 4); let grain = t.fbm(x - 7.0, y + 19.0, 24, 2); let fine = t.val(x, y, 64);
    let mottle = smooth(0.35, 0.75, t.fbm(x + 13.0, y - 31.0, 4, 3));
    // les pores : de petits trous serrés, et de plus grands, rares
    let pore = smooth(0.8, 0.88, t.val(x + 5.0, y - 3.0, 64)) * 0.8 + smooth(0.84, 0.9, t.fbm(x - 21.0, y + 11.0, 16, 2)) * 0.6;
    let mut tone = 0.6 + 0.14 * (big - 0.5) + 0.12 * (mid - 0.5) + 0.08 * (grain - 0.5) + 0.05 * (fine - 0.5) + 0.1 * mottle;
    let mut h = 0.5 + 0.18 * (mid - 0.5) + 0.08 * (grain - 0.5) - 0.45 * pore.min(1.0);
    tone -= 0.28 * pore.min(1.0);
    // les fossiles, à des places fixes selon la variante, toujours entiers dans la case
    let fossils: [(f64, f64, f64, f64, u8); 2] = if t.v == 0 { [(40.0, 42.0, 11.0, 0.3, 1), (19.0, 18.0, 6.0, -0.5, 0)] } else { [(22.0, 40.0, 7.5, 2.6, 0), (45.0, 17.0, 5.5, 0.0, 0)] };
    let mut shell = 0.0f64; let mut rim = 0.0f64;
    for (cx, cy, r, a, kind) in fossils {
        let (dx, dy) = (x - cx, y - cy); let d = hypot2(dx, dy); if d > r + 1.5 { continue; }
        let (ca, sa) = (crate::det::cos(a), crate::det::sin(a)); let (u, w) = (dx * ca - dy * sa, dx * sa + dy * ca);
        if kind == 1 { // l'ammonite : un enroulement en spirale, côtes rayonnantes, un ourlet
            let ang = w.atan2(u); let turn = (d / r * 3.2 - ang / std::f64::consts::TAU).rem_euclid(1.0); let ribs = 0.5 + 0.5 * (ang * 22.0).cos();
            let body = 1.0 - smooth(r - 1.2, r + 0.4, d); let groove = smooth(0.0, 0.12, turn) * (1.0 - smooth(0.82, 1.0, turn));
            shell = shell.max(body * (0.55 + 0.3 * groove + 0.15 * ribs)); rim = rim.max(body * (1.0 - groove) * 0.7);
        } else { // la coquille : un éventail de côtes, l'oreillette à la charnière
            let ang = u.atan2(-w); let fan = 1.0 - smooth(1.1, 1.3, ang.abs()); let body = fan * (1.0 - smooth(r - 0.8, r + 0.5, d)) * smooth(0.2, 1.4, d);
            let ribs = 0.5 + 0.5 * (ang * 9.0).cos(); let ear = (1.0 - smooth(0.0, 1.6, (w + 0.6).abs())) * (1.0 - smooth(1.2, 2.8, u.abs()));
            shell = shell.max(body * (0.6 + 0.4 * ribs)).max(ear * 0.7); rim = rim.max(body * (1.0 - ribs) * 0.5);
        }
    }
    tone = lerp(tone, 0.86 + 0.08 * (grain - 0.5), shell.min(1.0) * 0.9) - 0.3 * rim; h = lerp(h, 0.72 + 0.25 * shell, shell.min(1.0)) - 0.15 * rim;
    let c = pal(&LIME_F, tone); S { c, h: clamp(h, 0.0, 1.0), r: 0.88 }
}
/// Calcaire pâle : une pierre crème presque lisse, à peine marbrée, quelques creux irréguliers, et un fin lit de
/// sédimentation qui la traverse en travers.
fn limestone_pale(t: &Tile, x: f64, y: f64) -> S {
    let big = t.fbm(x, y, 2, 3); let mid = t.fbm(x + 41.0, y + 3.0, 6, 4); let grain = t.fbm(x - 7.0, y + 19.0, 32, 2);
    let hole = smooth(0.74, 0.84, t.fbm(x + 3.0, y - 17.0, 8, 3)) + 0.6 * smooth(0.82, 0.9, t.fbm(x - 5.0, y + 9.0, 16, 2)); let hole = hole.min(1.0); let speck = smooth(0.84, 0.92, t.val(x + 9.0, y + 2.0, 64));
    let lines = if t.v == 2 { [22.0, 47.0] } else { [12.0, 38.0] }; let mut bed = 0.0f64;
    for l in lines { let yy = l + 3.0 * (t.fbm(x, l, 4, 2) - 0.5) * 2.0; let gap = (y - yy).abs(); let broken = smooth(0.3, 0.55, t.fbm(x + l, 7.0, 8, 2)); bed = bed.max((1.0 - smooth(0.4, 1.3, gap)) * broken); }
    let mottle = smooth(0.4, 0.7, t.fbm(x + 17.0, y + 5.0, 4, 3)); let tone = 0.7 + 0.1 * (big - 0.5) + 0.12 * (mid - 0.5) - 0.08 * mottle + 0.05 * (grain - 0.5) - 0.3 * hole - 0.16 * bed - 0.12 * speck;
    S { c: pal(&LIME_P, tone), h: clamp(0.55 + 0.12 * (mid - 0.5) + 0.05 * (grain - 0.5) - 0.5 * hole - 0.3 * bed - 0.2 * speck, 0.0, 1.0), r: 0.82 }
}
/// Calcite des concrétions : des lits de croissance ondulés, miel, caramel, crème et blanc laiteux, que coupent des
/// coulées verticales ; chaque lit fait un petit bourrelet ; mouillée et luisante.
fn flowstone(t: &Tile, x: f64, y: f64) -> S {
    let wave = 2.6 * crate::det::sin(x * std::f64::consts::TAU / 64.0 + 4.0 * t.fbm(x, y, 2, 2)) + 3.0 * (t.fbm(x, y, 3, 3) - 0.5);
    let drip = t.fbm(x + 3.0 * (t.fbm(x, y, 4, 2) - 0.5), 0.0, 8, 3); let col = smooth(0.45, 0.75, t.fbm(x - 9.0, 0.0, 4, 2));
    let yy = y + wave + 2.2 * col * (drip - 0.5) * 2.0; let band = (yy * 6.0 / 64.0 + 0.35 * t.fbm(x + 30.0, y, 4, 2)).rem_euclid(1.0);
    let honey = smooth(0.0, 0.18, band) * (1.0 - smooth(0.32, 0.5, band)); let dark = smooth(0.2, 0.28, band) * (1.0 - smooth(0.3, 0.4, band));
    let milk = smooth(0.62, 0.8, band); let fine = t.val(x, y, 64); let n = t.fbm(x, y, 8, 3);
    let streak = smooth(0.55, 0.8, drip) * col; let tone = 0.64 + 0.28 * milk - 0.36 * honey - 0.08 * dark + 0.16 * streak - 0.06 * (1.0 - col) * (drip - 0.5) + 0.07 * (n - 0.5) + 0.05 * (fine - 0.5);
    let lip = (1.0 - smooth(0.0, 0.1, (band - 0.5).abs())) + 0.6 * (1.0 - smooth(0.0, 0.08, (band - 0.95).abs()));
    S { c: pal(&FLOW, tone), h: clamp(0.5 + 0.22 * lip + 0.12 * milk - 0.1 * honey + 0.1 * (drip - 0.5) * col, 0.0, 1.0), r: 0.28 + 0.12 * honey }
}
// ------------------------------------------------------------------ les grottes
// Chaque matière rend aussi sa part de lumière propre (0 à 1, par texel) : la lave, les fissures du magma, le cœur des
// cristaux, les filaments de la mousse lumineuse brillent dans le noir. Elle est rangée dans l'alpha de l'albédo.
/// Un amas de cristaux : des prismes hexagonaux pointus, en éventail, qui se chevauchent ; chaque prisme a trois faces
/// (ombre, plein jour, reflet), des arêtes vives, un cœur qui luit et des stries de croissance. Entre eux, une gangue sombre.
fn shards(t: &Tile, x: f64, y: f64, c: &[C; 4]) -> (S, f64) {
    let mut best: Option<(f64, f64, f64, f64, f64, i64)> = None; // (profondeur, u, côté, demi-largeur, demi-longueur, id)
    for i in 0..26i64 {
        let r = |j: i64| t.pi(i * 11 + j + 40 + t.v as i64 * 293); let (cx, cy) = (r(0) * 64.0, r(1) * 64.0); let ang = std::f64::consts::FRAC_PI_2 + (r(2) - 0.5) * 1.5;
        let hl = 9.0 + 15.0 * r(3); let wid = 3.0 + 4.5 * r(4); let depth = r(5); let (ca, sa) = (ang.cos(), ang.sin());
        for ox in [-64.0, 0.0, 64.0] { for oy in [-64.0, 0.0, 64.0] {
            let (dx, dy) = (x - cx - ox, y - cy - oy); let u = dx * ca + dy * sa; if u.abs() >= hl { continue; }
            let w = -dx * sa + dy * ca; let hw = wid * ((hl - u.abs()) / (wid * 1.3)).min(1.0); if w.abs() >= hw { continue; }
            if best.map_or(true, |b| depth > b.0) { best = Some((depth, u, w / hw, hw, hl, i)); }
        } }
    }
    let grain = t.val(x, y, 32);
    let Some((_, u, side, hw, hl, id)) = best else { // la gangue : roche sombre teintée, piquée d'éclats minuscules
        let g = t.fbm(x, y, 8, 3); let spark = smooth(0.93, 0.98, t.val(x + 9.0, y, 64));
        return (S { c: pal(c, 0.04 + 0.12 * g + 0.5 * spark), h: 0.1 + 0.1 * g, r: 0.8 }, spark * 0.4);
    };
    let k = t.pi(id + 900 + t.v as i64 * 53); let face = if side < -0.34 { 0.3 } else if side < 0.34 { 0.58 } else { 0.82 };
    let ridge = smooth(0.1, 0.0, (side.abs() - 0.34).abs()); let rim = smooth(0.82, 1.0, side.abs()) * smooth(1.2, 0.4, hw * (1.0 - side.abs()));
    let tip = smooth(hl * 0.4, hl, u.abs()); let bands = 0.5 + 0.5 * sin(u * 1.6 + 3.0 * t.val(x, y, 8));
    let core = (1.0 - side.abs()).powf(2.0) * (0.55 + 0.45 * bands);
    let tone = face + 0.12 * (k - 0.5) + 0.25 * ridge + 0.15 * tip + 0.06 * (grain - 0.5) - 0.45 * rim;
    (S { c: pal(c, tone.clamp(0.02, 0.98)), h: 0.55 + 0.25 * (1.0 - side.abs()) - 0.4 * rim, r: 0.08 }, ((0.08 + 0.3 * core + 0.2 * tip) * (1.0 - rim)).min(0.6))
}
fn cave_mat(name: &str, t: &Tile, x: f64, y: f64) -> Option<(S, f64)> {
    let crystal = |c: [C; 4], t: &Tile| shards(t, x, y, &c);
    Some(match name {
        "basalt" => { // des colonnes : un pavage de cellules aux joints creux, grain sombre et bleuté
            let (d1, d2, id) = t.vor(x, y, 4); let joint = 1.0 - smooth(0.03, 0.08, d2 - d1); let k = (id % 5) as f64 / 5.0; let g = t.fbm(x, y, 16, 3);
            (S { c: pal(&pl!("#1d1f24", "#2f3239", "#43464e", "#595c63"), 0.35 + 0.25 * k + 0.25 * (g - 0.5) - 0.3 * joint), h: 0.6 - 0.5 * joint + 0.1 * (g - 0.5), r: 0.78 }, 0.0) }
        "obsidian" => { // un verre noir, des volutes violettes, des éclats qui accrochent la lumière
            let swirl = t.fbm(x + 6.0 * t.fbm(x, y, 2, 2), y, 4, 3); let glint = smooth(0.9, 0.96, t.val(x, y, 32));
            (S { c: pal(&pl!("#07060b", "#14101e", "#2b1f42", "#5a4a7c"), 0.18 + 0.4 * smooth(0.45, 0.8, swirl) + 0.5 * glint), h: 0.5 + 0.1 * (swirl - 0.5), r: 0.06 }, 0.0) }
        "lava" => { // une pâte orange qui coule, des plaques sombres qui flottent dessus
            let flow = t.fbm(x + 5.0 * t.fbm(x, y, 2, 2), y + 3.0 * t.fbm(x + 9.0, y, 3, 2), 4, 4); let (d1, d2, _) = t.vor(x, y, 3); let crust = smooth(0.18, 0.05, d2 - d1).max(0.0) * smooth(0.5, 0.7, flow);
            let hot = 1.0 - 0.75 * crust; (S { c: pal(&pl!("#3a0d02", "#a8300a", "#f2701a", "#ffd25a"), 0.25 + 0.6 * hot * flow + 0.25 * hot), h: 0.5 - 0.3 * crust, r: 0.6 }, 0.55 + 0.45 * hot) }
        "magma" => { // une croûte noire cordée (des plis de lave refroidie) ; entre les cordes, des rigoles de lave qui serpentent et des flaques qui bouillonnent
            let wx = x + 7.0 * t.fbm(x, y, 2, 2); let wy = y + 5.0 * t.fbm(x + 13.0, y, 3, 2);
            let tau = std::f64::consts::TAU / 64.0; let rope = 0.5 + 0.5 * sin((wy + 4.0 * sin(wx * tau * 2.0)) * tau * 9.0); let flow = t.fbm(wx, wy, 3, 3);
            let rill = 1.0 - smooth(0.0, 0.035, (flow - 0.5).abs()); let pool = smooth(0.71, 0.76, t.fbm(x + 30.0 + 17.0 * t.v as f64, y, 3, 3)); let hot = rill.max(pool) * (0.75 + 0.25 * t.val(x, y, 16));
            let crust = pal(&pl!("#0d0908", "#1d1410", "#2f2119", "#47322a"), 0.25 + 0.45 * rope + 0.15 * (t.val(x, y, 32) - 0.5));
            let fire = pal(&pl!("#6a1404", "#d3420c", "#ff9a2a", "#fff0a0"), 0.35 + 0.6 * hot);
            let glow_rim = smooth(0.45, 0.0, (rill.max(pool) - 0.15).abs()) * 0.3; let c = if hot > 0.3 { fire } else { [crust[0] + glow_rim * 0.5, crust[1] + glow_rim * 0.12, crust[2]] };
            (S { c, h: if hot > 0.3 { 0.15 } else { 0.35 + 0.45 * rope }, r: if hot > 0.3 { 0.5 } else { 0.9 } }, if hot > 0.3 { 0.5 + 0.5 * hot } else { glow_rim }) }
        "crystal_violet" => crystal(pl!("#3a1a63", "#7a3fc2", "#b98af0", "#f0dcff"), t),
        "crystal_blue" => crystal(pl!("#0d2c5c", "#2a72c8", "#79c4f5", "#dff6ff"), t),
        "crystal_pink" => crystal(pl!("#5a1a40", "#c04d92", "#f29ccc", "#ffe6f4"), t),
        "crystal_white" => { let (s, e) = crystal(pl!("#4c5a6a", "#8698ad", "#bccbd9", "#e6eef5"), t); (s, e * 0.5) }
        "abyss_rock" => { // une roche presque noire, bleutée, parsemée de minuscules éclats froids
            let g = t.fbm(x, y, 6, 4); let fine = t.val(x, y, 64); let spark = smooth(0.95, 0.985, t.val(x + 17.0, y - 3.0, 64)); let vein = 1.0 - smooth(0.0, 0.05, (t.fbm(x, y, 3, 3) - 0.5).abs());
            (S { c: pal(&pl!("#0e1119", "#1f2433", "#323a50", "#6f8fc0"), 0.32 + 0.25 * (g - 0.5) + 0.06 * (fine - 0.5) + 0.12 * vein + 0.7 * spark), h: 0.5 + 0.2 * (g - 0.5) + 0.1 * vein, r: 0.7 }, spark * 0.7) }
        "glow_moss" => { // un coussin de mousse bleu-vert, touffu ; des perles luminescentes en grappes, chacune avec son halo
            let g = t.fbm(x, y, 8, 3); let tuft = t.val(x, y, 32); let fine = t.val(x + 5.0, y, 64); let clump = smooth(0.42, 0.62, t.fbm(x, y + 9.0, 4, 2));
            let (d1, _, id) = t.vor(x, y, 14); let on = (t.pi(id + 500) < 0.04 + 0.2 * clump) as i32 as f64; let rad = 0.16 + 0.16 * t.pi(id + 700);
            let pearl = smooth(rad, rad * 0.35, d1) * on; let halo = smooth(rad * 2.4, rad, d1) * on * 0.45;
            let moss = pal(&pl!("#06140f", "#0f2a22", "#17423a", "#2a6457"), 0.15 + 0.5 * g + 0.3 * tuft + 0.12 * (fine - 0.5));
            let c = if pearl > 0.2 { pal(&pl!("#1a8f86", "#45d6c6", "#a5fff1", "#f2fffd"), 0.3 + 0.7 * pearl) } else { [moss[0] + halo * 0.08, moss[1] + halo * 0.45, moss[2] + halo * 0.42] };
            (S { c, h: 0.35 + 0.35 * tuft + 0.2 * g + 0.25 * pearl, r: if pearl > 0.2 { 0.3 } else { 0.95 } }, (pearl * 0.6).max(halo * 0.3)) }
        "mycelium" => { // un tapis gris-violet, parcouru de fils blancs
            let g = t.fbm(x, y, 12, 3); let thread = 1.0 - smooth(0.0, 0.05, (t.fbm(x, y + 7.0, 8, 3) - 0.5).abs());
            (S { c: pal(&pl!("#3a2f3c", "#5a4a5e", "#7d6c82", "#d9d0dc"), 0.3 + 0.35 * g + 0.45 * thread), h: 0.5 + 0.25 * g + 0.15 * thread, r: 0.92 }, 0.0) }
        "mushroom_stem" => { // la chair pâle, fibreuse, en long
            let f = t.fbm(x, 0.0, 24, 3); let g = t.fbm(x, y, 6, 3); (S { c: pal(&pl!("#b8ab92", "#d6cbb4", "#e9e1cf", "#f6f1e6"), 0.35 + 0.35 * f + 0.2 * (g - 0.5)), h: 0.5 + 0.3 * f, r: 0.8 }, 0.0) }
        "cap_red" => { let (d1, _, id) = t.vor(x, y, 3); let spot = smooth(0.3, 0.2, d1) * ((id % 3 != 0) as i32 as f64); let g = t.fbm(x, y, 6, 3);
            (S { c: if spot > 0.5 { pal(&pl!("#d8cfc0", "#ece4d6", "#f8f2e8", "#ffffff"), 0.5 + 0.3 * g) } else { pal(&pl!("#5c0d0b", "#9d1a14", "#cf3324", "#ee6a4c"), 0.4 + 0.35 * (g - 0.5) + 0.2 * d1) }, h: 0.5 + 0.25 * spot + 0.1 * g, r: 0.55 }, 0.0) }
        "cap_violet" => { let g = t.fbm(x, y, 5, 4); let ring = 0.5 + 0.5 * sin(t.fbm(x, y, 2, 2) * 20.0); let (d1, _, id) = t.vor(x, y, 4); let spot = smooth(0.22, 0.14, d1) * ((id % 2) as f64);
            (S { c: if spot > 0.5 { pal(&pl!("#b48ad8", "#d4b4f0", "#ecdcff", "#ffffff"), 0.6) } else { pal(&pl!("#2a1240", "#4d2275", "#7a3cab", "#a66fd6"), 0.35 + 0.3 * (g - 0.5) + 0.1 * ring) }, h: 0.5 + 0.2 * g + 0.2 * spot, r: 0.5 }, spot * 0.6) }
        "cap_blue" => { let g = t.fbm(x, y, 5, 4); let (d1, _, id) = t.vor(x, y, 4); let spot = smooth(0.24, 0.12, d1) * ((id % 2) as f64);
            (S { c: if spot > 0.5 { pal(&pl!("#7fd8ff", "#b6ecff", "#e2f9ff", "#ffffff"), 0.6) } else { pal(&pl!("#0b1f45", "#163d7a", "#2a63b0", "#4f93dd"), 0.35 + 0.3 * (g - 0.5)) }, h: 0.5 + 0.2 * g + 0.2 * spot, r: 0.5 }, 0.15 + spot * 0.75) }
        "gills" => { // le dessous du chapeau : des lamelles rayonnantes, chaudes, qui luisent
            let l = 0.5 + 0.5 * sin((x + 2.5 * t.fbm(x, y, 2, 2)) * std::f64::consts::TAU * 12.0 / 64.0); let g = t.fbm(x, y, 8, 2);
            (S { c: pal(&pl!("#8a4a12", "#d98a2c", "#ffc35c", "#fff0b8"), 0.35 + 0.45 * l + 0.15 * (g - 0.5)), h: 0.4 + 0.4 * l, r: 0.6 }, 0.35 + 0.45 * l) }
        "ice_blue" => { // une glace dense et bleue, des bulles prises, des fêlures blanches
            let g = t.fbm(x, y, 4, 4); let bub = smooth(0.88, 0.95, t.val(x, y, 32)); let (d1, d2, _) = t.vor(x, y, 3); let crack = 1.0 - smooth(0.0, 0.04, d2 - d1);
            (S { c: pal(&pl!("#1b4f8c", "#3b86c8", "#8cc8ef", "#eaf8ff"), 0.4 + 0.25 * (g - 0.5) + 0.4 * bub + 0.35 * crack + 0.1 * d1), h: 0.55 + 0.1 * g - 0.2 * crack, r: 0.08 }, 0.0) }
        "rail_bed" => { let g = t.fbm(x, 0.0, 24, 2); (S { c: pal(&pl!("#3a2614", "#5c3d20", "#7a5532", "#97704a"), 0.4 + 0.4 * g), h: 0.5 + 0.3 * g, r: 0.85 }, 0.0) }
        _ => return None,
    })
}
/// Gravier : des galets arrondis serrés, de tailles et de couleurs variées, ombrés comme des petits dômes, avec du sable sombre entre eux.
fn gravel(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x + 3.0 * t.fbm(x, y, 6, 2), y + 3.0 * t.fbm(x + 9.0, y, 6, 2), 13); let k = t.pi(id); let k2 = t.pi(id + 300);
    let size = 0.36 + 0.12 * k2; let dome = (1.0 - (d1 / size).min(1.0)).max(0.0); let gap = smooth(0.0, 0.07, d2 - d1);
    let pebble = match (k * 5.0) as i32 { 0 => [0.52, 0.5, 0.47], 1 => [0.6, 0.55, 0.47], 2 => [0.42, 0.41, 0.4], 3 => [0.66, 0.64, 0.6], _ => [0.55, 0.47, 0.38] };
    let shade = 0.62 + 0.38 * dome.sqrt() + 0.06 * (t.val(x, y, 48) - 0.5); let fill = [0.26, 0.24, 0.21];
    let c = if gap < 0.35 || dome <= 0.0 { fill } else { [pebble[0] * shade, pebble[1] * shade, pebble[2] * shade] };
    S { c, h: if gap < 0.35 { 0.05 } else { 0.25 + 0.7 * dome.sqrt() }, r: if gap < 0.35 { 0.95 } else { 0.6 } }
}
fn mossy(t: &Tile, x: f64, y: f64) -> S {
    let st = stone(t, x, y, &STONE, 5); let drip = 15.0 + 22.0 * t.fbm(x, 0.0, 5, 3); let m = t.fbm(x, y, 7, 3); let k = y < drip || m > 0.61; let g = t.val(x, y, 32);
    let c = pal(&MOSS, 0.14 + 0.72 * m + 0.26 * (g - 0.5));
    S { c: if k { c } else { mul(st.c, if y < drip + 5.0 { 0.72 } else { 0.92 }) }, h: if k { 0.72 + 0.28 * g } else { st.h }, r: if k { 0.92 } else { st.r } }
}
fn bark(t: &Tile, x: f64, y: f64, colors: &[C], relief: f64) -> S {
    // Plaques de largeurs et de longueurs inégales, sillons qui serpentent : rien ne doit se lire comme une grille.
    const COLS: f64 = 9.0; let w = x + 6.0 * t.fbm(x, y, 3, 2) + 2.6 * t.fbm(x, y, 7, 2) - 4.3; let q = ((w + 128.0) % 64.0) * COLS / 64.0; let col = q.floor() % COLS; let u0 = q - q.floor();
    let tc = t.pi(col as i64 * 61 + 7); let segs = 3.0 + (tc * 4.0).floor(); let seg = ((y + tc * 64.0) % 64.0) / 64.0 * segs; let sid = seg.floor(); let sf = seg - sid; let ts = t.pi(col as i64 * 17 + sid as i64 * 131);
    let u = u0.powf(0.65 + 0.9 * ts); let ridge = sin(u * std::f64::consts::PI).powf(0.7); let gap = smooth(0.0, 0.07, sf) * smooth(1.0, 0.93, sf); let (n, fine) = (t.fbm(x, y, 8, 3), t.val(x, y * 0.5, 32));
    let c = pal(colors, 0.16 + 0.34 * tc + 0.26 * ts + 0.3 * (n - 0.5) + 0.22 * ridge + 0.16 * (fine - 0.5));
    let k = 1.0 - relief * (0.58 * (1.0 - ridge) + 0.3 * (1.0 - gap));
    S { c: mul(c, k), h: ridge * (0.5 + 0.5 * ts) * (0.4 + 0.6 * gap), r: 0.84 }
}
fn birch(t: &Tile, x: f64, y: f64) -> S {
    let row = (y / 9.0).floor(); let jit = (t.pi(row as i64 * 29) * 11.0).floor(); let cell = ((x + jit) / 11.0).floor() % 6.0; let id = t.pi(cell as i64 * 37 + row as i64 * 101);
    let ry = y / 9.0 - row; let rx = ((x + jit) / 11.0) % 1.0;
    if id > 0.6 && ry > 0.16 + 0.2 * id && ry < 0.86 && rx > 0.1 && rx < 0.95 { return S { c: pal(&BIRCH_DARK, t.val(x, y, 32)), h: 0.12, r: 0.72 }; }
    let (n, warm) = (t.fbm(x, y, 6, 3), t.val(x, y, 5)); let c = pal(&BIRCH, 0.22 + 0.7 * n);
    S { c: [c[0], c[1] * (0.985 - 0.03 * warm), c[2] * (0.97 - 0.09 * warm)], h: 0.5 + 0.3 * n, r: 0.5 }
}
fn wood_end(t: &Tile, x: f64, y: f64, wood: &[C], rim: f64, bf: &dyn Fn(&Tile, f64, f64) -> S) -> S {
    let (dx, dy) = (x - 31.5, y - 31.5); let r = hypot2(dx, dy); let bx = dx.abs().max(dy.abs()); let a = dy.atan2(dx);
    if bx > 27.5 + 1.6 * t.val(x, y, 8) { let b = bf(t, x, y); return S { c: mul(b.c, rim), h: b.h, r: 0.85 }; }
    let wob = 3.2 * t.fbm(x, y, 4, 2) - 1.6; let q = r * 0.62 + wob; let ring = 0.5 + 0.5 * sin(q * 2.2); let late = smooth(0.72, 0.96, ring);
    let crack = if sin(a * 3.0 + 1.2 + 2.0 * t.val(x, y, 3)).abs() < 0.035 && r > 5.0 && r < 24.0 { 1.0 } else { 0.0 }; let pith = smooth(4.5, 2.0, r);
    let c = pal(wood, 0.7 - 0.27 * late + 0.14 * (t.val(x, y, 32) - 0.5) - 0.3 * pith - 0.12 * smooth(26.0, 21.0, r));
    let k = (1.0 - 0.42 * crack) * (1.0 - 0.16 * smooth(21.0, 27.0, bx));
    S { c: mul(c, k), h: 0.45 + 0.3 * ring - 0.4 * crack, r: 0.7 }
}
fn leaf(t: &Tile, x: f64, y: f64, sh: C) -> S {
    let (d1, d2, id) = t.vor(x, y, 11); let rim = smooth(0.0, 0.14, d2 - d1); let vein = smooth(0.26, 0.05, d1); let (m, g) = (t.fbm(x, y, 5, 2), t.val(x, y, 32));
    let c = pal(&LEAF, 0.12 + 0.56 * t.pi(id) + 0.3 * (m - 0.2) + 0.12 * vein + 0.1 * (g - 0.5)); let k = 0.55 + 0.45 * rim;
    S { c: [c[0] * sh[0] * k, c[1] * sh[1] * k, c[2] * sh[2] * k], h: rim * (0.5 + 0.5 * t.pi(id)), r: 0.5 }
}
fn planks(t: &Tile, x: f64, y: f64, p: &[C]) -> S {
    let row = (y / 16.0).floor(); let f = (y % 16.0) / 16.0; let seam = smooth(0.0, 0.07, f) * smooth(1.0, 0.93, f); let g = t.val(x + row * 19.0, y * 2.2, 12); let tone = t.pi(row as i64 * 13);
    S { c: mul(pal(p, 0.15 + 0.45 * tone + 0.4 * g), 0.5 + 0.5 * seam), h: seam * (0.7 + 0.3 * g), r: 0.7 }
}
/// Corail massif : une surface claire et poreuse, semée de calices ronds en creux et de crêtes sinueuses ; la couleur vient de la teinte du bloc.
fn coral(t: &Tile, x: f64, y: f64) -> S {
    let (d1, _d2, id) = t.vor(x, y, 9); let cup = smooth(0.34, 0.12, d1); let rim = smooth(0.22, 0.34, d1) * smooth(0.48, 0.34, d1);
    let ridge = (0.5 + 0.5 * sin((x * 0.21 + 3.0 * t.fbm(x, y, 4, 3)) * std::f64::consts::PI)).powf(3.0); let n = t.fbm(x, y, 6, 3); let k = 0.78 + 0.16 * (n - 0.5) + 0.08 * t.pi(id) + 0.12 * rim - 0.32 * cup + 0.06 * ridge;
    S { c: [0.86 * k, 0.84 * k, 0.8 * k], h: 0.55 + 0.25 * rim - 0.5 * cup + 0.15 * ridge, r: 0.8 }
}
/// Bordages d'épave : planches grises et gonflées d'eau, nœuds noircis, traînées d'algues vertes, croûtes de bernacles.
fn planks_wreck(t: &Tile, x: f64, y: f64) -> S {
    let mut p = planks(t, x, y, &pl!("#2e2a22", "#463f32", "#5b5243")); let algae = smooth(0.52, 0.7, t.fbm(x * 0.6, y * 1.6, 4, 3)); let barn = t.val(x + 7.0, y - 3.0, 32) > 0.86;
    if barn { p.c = [0.72, 0.7, 0.64]; p.h = 0.9; } else { p.c = [lerp(p.c[0], 0.2, algae * 0.7), lerp(p.c[1], 0.36, algae * 0.7), lerp(p.c[2], 0.18, algae * 0.7)]; }
    p.r = 0.95; p
}
const PLANKS_BIRCH: [C; 3] = pl!("#a3865c", "#c7aa7a", "#e2c999"); const PLANKS_SPRUCE: [C; 3] = pl!("#4a301a", "#6a4628", "#8a603a");
const CUT_STONE: [C; 4] = pl!("#6e695f", "#8d877a", "#aaa395", "#c4bdae"); const BRICK: [C; 4] = pl!("#6e2c1c", "#8e3c26", "#a8502f", "#bf6a44");
/// Minerai : la pierre ordinaire, piquée de grains et de veines de la couleur du métal.
fn ore(t: &Tile, x: f64, y: f64, a: C, b: C) -> S {
    let base = stone(t, x, y, &STONE, 5); let (d1, d2, id) = t.vor(x + 0.4 * t.fbm(x, y, 12, 2), y, 14); let vein = smooth(0.42, 0.62, t.fbm(x * 0.7 + 11.0, y * 1.3, 6, 3));
    let k = t.pi(id); let grain = k > 0.72 && d1 < 0.3; let edge = smooth(0.0, 0.08, d2 - d1);
    if grain || (vein > 0.8 && k > 0.55) { let c = if t.pi(id + 77) > 0.5 { a } else { b }; let m = 0.85 + 0.3 * t.val(x, y, 64); return S { c: mul(c, m * (0.78 + 0.22 * edge)), h: 0.75 + 0.2 * edge, r: 0.35 }; }
    base
}
fn clay(t: &Tile, x: f64, y: f64) -> S { let (n, s, m) = (t.fbm(x, y, 3, 3), t.val(x, y, 48), t.fbm(x + 17.0, y, 8, 2)); let band = 0.05 * (y * 0.35 + 4.0 * n).sin(); let k = 0.86 + 0.2 * (n - 0.5) + 0.08 * (s - 0.5) + band; let warm = smooth(0.55, 0.75, m) * 0.12; S { c: [(0.55 + warm) * k, (0.56 + warm * 0.6) * k, (0.6 - warm * 0.4) * k], h: 0.45 + 0.2 * n, r: 0.75 } }
/// Appareil de blocs : rangées décalées d'un demi-bloc, joints de mortier en creux.
fn bricks(t: &Tile, x: f64, y: f64, h: f64, w: f64, p: &[C], mortar: C) -> S {
    let row = (y / h).floor(); let xo = x + if row as i64 % 2 == 0 { 0.0 } else { w * 0.5 }; let col = (xo / w).floor(); let (fx, fy) = (xo - col * w, y - row * h);
    let joint = fx.min(w - fx).min(fy.min(h - fy)); let m = smooth(0.6, 1.6, joint); let id = (row as i64) * 31 + col as i64;
    if m < 0.5 { let n = t.val(x, y, 32); return S { c: mul(mortar, 0.9 + 0.2 * n), h: 0.1, r: 0.95 }; }
    let n = t.fbm(x, y, 8, 3); let tone = 0.2 + 0.5 * t.pi(id) + 0.35 * (n - 0.5); let chip = smooth(0.6, 1.0, joint / 3.0);
    S { c: mul(pal(p, tone), 0.8 + 0.2 * chip), h: 0.5 + 0.4 * m * (0.6 + 0.4 * n), r: 0.82 }
}
fn thatch(t: &Tile, x: f64, y: f64) -> S {
    let strand = t.val(x * 3.0, y * 0.25, 64); let n = t.fbm(x, y, 6, 3); let row = (y / 10.0).floor(); let lap = smooth(0.0, 2.5, y - row * 10.0);
    let k = 0.62 + 0.4 * strand + 0.15 * (n - 0.5); S { c: [0.76 * k * (0.7 + 0.3 * lap), 0.62 * k * (0.7 + 0.3 * lap), 0.32 * k * (0.7 + 0.3 * lap)], h: 0.3 + 0.5 * strand * lap, r: 0.95 }
}
fn cobble(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x, y, 4); let gap = smooth(0.0, 0.16, d2 - d1); let n = t.fbm(x, y, 8, 3); let tone = 0.3 + 0.45 * t.pi(id) + 0.3 * (n - 0.5);
    S { c: mul(pal(&STONE, tone), 0.35 + 0.65 * gap), h: 0.15 + 0.7 * gap * (0.7 + 0.3 * n), r: 0.85 }
}

// ------------------------------------------------------------------ refonte des villages : matériaux de construction
const LIME: [C; 4] = pl!("#94866c", "#b2a385", "#c9bb9b", "#dcd0b3"); const GREYST: [C; 4] = pl!("#4a4845", "#66635e", "#84807a", "#a29d95");
const BOARD: [C; 4] = pl!("#3b2a1d", "#56402e", "#725840", "#8b7256"); const TRAIL: [C; 4] = pl!("#5a432e", "#73573a", "#8a6c4b", "#a0835e");
/// Pierre calcaire appareillée : des assises de hauteurs inégales, des pierres de longueurs variées, blondes,
/// arêtes adoucies, joints de mortier creusés ; quelques pierres plus brutes ou plus claires.
fn lime_masonry(t: &Tile, x: f64, y: f64) -> S {
    let rows = [0.0, 13.0, 31.0, 45.0, 64.0]; let r = (0..4).find(|i| y < rows[i + 1]).unwrap_or(3); let (y0, y1) = (rows[r], rows[r + 1]);
    let n = 2 + (t.pi(r as i64 * 5 + 1) * 3.0) as i64; let seg = 64.0 / n as f64; let off = t.pi(r as i64 * 7 + 3) * 64.0; let xr = (x + off).rem_euclid(64.0);
    let jit = |i: i64| (t.pi(r as i64 * 31 + i.rem_euclid(n)) - 0.5) * seg * 0.4;
    let mut k = (xr / seg).floor() as i64; if xr < k as f64 * seg + jit(k) { k -= 1; } if xr >= (k + 1) as f64 * seg + jit(k + 1) { k += 1; }
    let (xa, xb) = (k as f64 * seg + jit(k), (k + 1) as f64 * seg + jit(k + 1));
    let j = (xr - xa).min(xb - xr).min(y - y0).min(y1 - y); let m = smooth(0.6, 2.4, j); let id = r as i64 * 17 + k.rem_euclid(n);
    let n2 = t.fbm(x, y, 8, 3); let pit = smooth(0.7, 0.85, t.val(x * 2.0, y * 2.0, 64)); let rough = t.pi(id + 40) > 0.75;
    let tone = 0.25 + 0.5 * t.pi(id) + 0.25 * (n2 - 0.5) - 0.12 * pit - if rough { 0.08 } else { 0.0 };
    if m < 0.25 { return S { c: mul([0.66, 0.62, 0.55], 0.8 + 0.2 * n2), h: 0.12, r: 0.95 }; }
    S { c: mul(pal(&LIME, tone), 0.7 + 0.3 * m), h: 0.25 + 0.65 * m * (0.85 + 0.15 * n2) - 0.15 * pit, r: 0.88 }
}
/// Moellons gris équarris : des pierres anguleuses de tailles inégales, posées à plat, joints de mortier larges et creux.
fn rubble(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x * 0.8, y * 1.35, 4); let gap = smooth(0.03, 0.12, d2 - d1); let n = t.fbm(x, y, 8, 3);
    if gap < 0.4 { return S { c: mul([0.6, 0.58, 0.54], 0.8 + 0.25 * n), h: 0.1, r: 0.95 }; }
    let facet = smooth(0.12, 0.4, d2 - d1); let warm = t.pi(id + 13); let tone = 0.15 + 0.6 * t.pi(id) + 0.3 * (n - 0.5);
    let c = pal(&GREYST, tone); S { c: [c[0] * (0.96 + 0.1 * warm), c[1] * (0.98 + 0.03 * warm), c[2] * (1.02 - 0.06 * warm)], h: 0.3 + 0.4 * facet + 0.2 * n, r: 0.84 }
}
/// Bout d'une poutre équarrie : des cernes presque carrés, une fente de séchage, l'arête sombre.
fn beam_end(t: &Tile, x: f64, y: f64) -> S {
    let (dx, dy) = (x - 32.0 + 3.0 * t.fbm(x, y, 4, 2), y - 32.0); let d = dx.abs().max(dy.abs()) * 0.7 + dx.hypot(dy) * 0.3;
    let ring = (d * 0.85).sin() * 0.5 + 0.5; let crack = (1.0 - smooth(0.0, 1.2, (dx * 0.2 + dy).abs())) * smooth(4.0, 8.0, d) * (1.0 - smooth(20.0, 26.0, d));
    let edge = 1.0 - smooth(1.0, 5.0, x.min(63.99 - x).min(y.min(63.99 - y)));
    S { c: mul(pal(&TIMBER, 0.45 + 0.3 * ring), 1.0 - 0.4 * edge - 0.5 * crack), h: 0.5 + 0.2 * ring - 0.4 * crack - 0.3 * edge, r: 0.8 }
}
/// Bardage de grange : planches verticales vieillies, couvre-joints cloués sur les fentes, fil du bois, pointes rouillées.
fn boards(t: &Tile, x: f64, y: f64) -> S {
    let bw = 16.0; let k = (x / bw).floor(); let fx = x - k * bw; let id = k as i64;
    let batten = fx < 2.5 || fx > bw - 2.5; let grain = t.val(x * 0.3 + id as f64 * 7.0, y * 3.0, 64); let n = t.fbm(x, y, 4, 3);
    let weather = smooth(0.45, 0.8, t.fbm(x * 0.5 + 9.0, y * 0.3, 4, 3));
    let tone = 0.25 + 0.4 * t.pi(id) + 0.25 * (grain - 0.5) + 0.15 * weather; let gap = smooth(0.0, 1.0, fx.min(bw - fx));
    let nail = batten && ((y - 8.0).abs() < 1.2 || (y - 40.0).abs() < 1.2) && (fx - if fx < 8.0 { 1.2 } else { bw - 1.2 }).abs() < 1.2;
    if nail { return S { c: [0.28, 0.2, 0.16], h: 1.0, r: 0.5 }; }
    let c = pal(&BOARD, if batten { tone - 0.08 } else { tone }); let gr = 0.9 + 0.1 * weather;
    S { c: [c[0] * gr, c[1] * gr, c[2] * (gr + 0.03 * weather)], h: if batten { 0.85 + 0.1 * n } else { 0.35 + 0.25 * grain } * (0.4 + 0.6 * gap), r: 0.82 }
}
/// Tuiles vieillies : la terre cuite d'un toit ancien, mangée par la mousse et les lichens dans les recoins.
fn tiles_moss(t: &Tile, x: f64, y: f64) -> S {
    let mut s = roof_tiles(t, x, y); let moss = smooth(0.52, 0.66, t.fbm(x + 17.0, y + 5.0, 4, 4)); let lich = smooth(0.9, 0.94, t.val(x * 2.0, y * 2.0, 64)) * smooth(0.4, 0.55, t.fbm(x + 3.0, y + 41.0, 4, 3));
    let m = [0.3, 0.4, 0.16]; for i in 0..3 { s.c[i] = s.c[i] * (1.0 - 0.8 * moss) + m[i] * 0.8 * moss * (0.8 + 0.4 * t.val(x * 3.0, y * 3.0, 64)); s.c[i] = s.c[i] * (1.0 - 0.5 * lich) + 0.7 * 0.5 * lich; }
    s.h += 0.15 * moss; s
}
/// Chemin de terre battue : la terre tassée et plus claire au milieu, des cailloux affleurants, un peu de poussière.
fn path_top(t: &Tile, x: f64, y: f64) -> S {
    let n = t.fbm(x, y, 4, 4); let fine = t.val(x * 2.0, y * 2.0, 64); let (d1, d2, id) = t.vor(x, y, 8);
    let pebble = t.pi(id) > 0.8 && d1 < 0.3 * (0.6 + 0.6 * t.pi(id + 5)); let worn = smooth(0.4, 0.7, t.fbm(x * 0.5 + 3.0, y * 0.5, 2, 3));
    if pebble { let tone = 0.3 + 0.5 * t.pi(id + 9); return S { c: pal(&GREYST, tone), h: 0.75 - d1 * 1.5 + 0.0 * d2, r: 0.8 }; }
    S { c: mul(pal(&TRAIL, 0.3 + 0.35 * n + 0.15 * fine + 0.2 * worn), 1.0), h: 0.3 + 0.2 * n + 0.1 * fine - 0.1 * worn, r: 0.97 }
}
// ------------------------------------------------------------------ village médiéval
const PLASTER: [C; 4] = pl!("#b9ad94", "#d3c8b0", "#e3dac6", "#efe8d8"); const TIMBER: [C; 4] = pl!("#3a2414", "#55361f", "#73502f", "#906942");
const TILE: [C; 4] = pl!("#5a2a1e", "#743a29", "#8c4d38", "#a2634c"); const SLATE: [C; 4] = pl!("#2c3138", "#3b424b", "#4d555f", "#636b74");
/// Torchis chaulé : une surface claire, un peu bosselée, des taches d'humidité et quelques fissures fines.
fn plaster(t: &Tile, x: f64, y: f64) -> S {
    let n = t.fbm(x, y, 4, 4); let fine = t.val(x, y, 64); let stain = smooth(0.58, 0.78, t.fbm(x + 31.0, y * 0.6 + 7.0, 3, 3));
    let (d1, d2, _) = t.vor(x + 3.0 * t.fbm(x, y, 8, 2), y, 3); let crack = 1.0 - smooth(0.0, 0.025, d2 - d1) * 1.0; let crack = crack * smooth(0.62, 0.75, t.val(x, y, 6));
    let c = mul(pal(&PLASTER, 0.45 + 0.45 * n + 0.12 * (fine - 0.5)), 1.0 - 0.07 * stain - 0.16 * crack);
    S { c: [c[0], c[1] * (1.0 - 0.03 * stain), c[2] * (1.0 - 0.08 * stain)], h: 0.45 + 0.35 * n + 0.1 * fine - 0.3 * crack, r: 0.93 }
}
/// Chêne équarri : un fil droit et serré dans la longueur, des cernes plus sombres, un nœud ici ou là, les arêtes
/// chanfreinées qui foncent au bord : un bloc se lit comme une vraie poutre.
fn dark_timber(t: &Tile, x: f64, y: f64) -> S {
    let wob = 1.6 * t.fbm(x * 0.5, y * 0.05, 4, 2); let ring = ((x + wob) * 0.9).sin() * 0.5 + 0.5; let fine = t.val(x * 3.0, y * 0.12, 64);
    let (kx, ky) = (t.pi((y / 64.0).floor() as i64 + 3) * 40.0 + 12.0, 22.0 + 20.0 * t.pi(7)); let kd = ((x - kx).powi(2) * 1.8 + (y - ky).powi(2) * 0.35).sqrt(); let knot = 1.0 - smooth(1.5, 4.0, kd);
    let edge = x.min(63.99 - x); let chamfer = 1.0 - smooth(1.0, 5.0, edge);
    let tone = 0.42 + 0.22 * ring + 0.18 * (fine - 0.5) - 0.3 * knot;
    S { c: mul(pal(&TIMBER, tone), 1.0 - 0.35 * chamfer), h: 0.55 + 0.15 * ring + 0.1 * fine - 0.3 * chamfer - 0.2 * knot, r: 0.75 }
}
/// Tuiles plates de terre cuite, en rangs décalés : bas arrondi, ombre du rang du dessus, teintes de cuisson inégales, un peu de mousse.
fn roof_tiles(t: &Tile, x: f64, y: f64) -> S {
    let (h, w) = (16.0, 16.0); let row = (y / h).floor(); let xo = x + if row as i64 % 2 == 0 { 0.0 } else { w * 0.5 }; let col = (xo / w).floor();
    let (fx, fy) = ((xo - col * w) / w, (y - row * h) / h); let id = row as i64 * 17 + col as i64;
    let round = 1.0 - (2.0 * fx - 1.0).powi(2); let bottom = 0.78 + 0.2 * round; let gap = smooth(0.0, 0.06, fx.min(1.0 - fx));
    let n = t.fbm(x, y, 8, 3); let tone = 0.2 + 0.55 * t.pi(id) + 0.25 * (n - 0.5);
    let shade = 0.55 + 0.45 * smooth(0.0, 0.35, fy); let lip = if fy > bottom { 0.45 } else { 1.0 };
    let moss = smooth(0.7, 0.85, t.fbm(x + 13.0, y + 5.0, 4, 3)) * smooth(0.5, 0.0, fy);
    let mut c = mul(pal(&TILE, tone), shade * lip * (0.55 + 0.45 * gap));
    c = [lerp(c[0], 0.24, moss * 0.6), lerp(c[1], 0.28, moss * 0.6), lerp(c[2], 0.12, moss * 0.6)];
    S { c, h: (0.25 + 0.65 * fy) * gap * lip, r: 0.72 }
}
/// Ardoises en écailles fines, posées en rangs serrés, bords ébréchés, reflets bleutés.
fn slate(t: &Tile, x: f64, y: f64) -> S {
    let h = 12.8; let row = (y / h).floor(); let w = 11.0 + 5.0 * t.pi(row as i64 * 7 + 3); let xo = x + t.pi(row as i64 * 13) * w; let col = (xo / w).floor();
    let (fx, fy) = ((xo - col * w) / w, (y - row * h) / h); let id = row as i64 * 23 + col as i64; let chip = 0.06 * t.val(x, y, 32);
    let gap = smooth(0.0, 0.05 + chip, fx.min(1.0 - fx)); let n = t.fbm(x, y, 16, 2); let tone = 0.2 + 0.6 * t.pi(id) + 0.2 * (n - 0.5);
    let shade = 0.6 + 0.4 * smooth(0.0, 0.3, fy);
    S { c: mul(pal(&SLATE, tone), shade * (0.5 + 0.5 * gap)), h: (0.3 + 0.6 * fy) * gap, r: 0.55 }
}
// ------------------------------------------------------------------ richesse : pierres taillées, sols, toits, tissus, mobilier
const MARBLE: [C; 4] = pl!("#b9b5ae", "#d6d2ca", "#e8e5de", "#f6f4ef"); const ASHLAR: [C; 4] = pl!("#a99a7c", "#bfb194", "#d2c6aa", "#e2d8c0");
const FIELD: [C; 4] = pl!("#5a554d", "#79726a", "#968e83", "#b3ab9e"); const SETT: [C; 4] = pl!("#4c4a48", "#625f5b", "#7a7671", "#928d86");
const TERRA: [C; 4] = pl!("#8a4429", "#a2563a", "#b86b4b", "#c9825f"); const PARQ: [C; 4] = pl!("#6b4322", "#8a5a30", "#a87344", "#c18e5c");
const SHINGLE: [C; 4] = pl!("#4c3624", "#634732", "#7c5a40", "#94704f"); const WOOL: [C; 4] = pl!("#cfc8bb", "#ded8cc", "#ebe6dc", "#f6f2ea");
const IRON: [C; 4] = pl!("#2c2d30", "#3c3e42", "#4f5156", "#64676c"); const VERD: [C; 4] = pl!("#3f7466", "#4f8a78", "#64a08b", "#86b9a2");
const STRAW: [C; 4] = pl!("#9c7f3c", "#b8994c", "#d0b25e", "#e3c977");
/// Un cadre de poutres sombres au bord du bloc ; `w` largeur en pixels de la texture (64 de côté).
fn border(x: f64, y: f64, w: f64) -> f64 { 1.0 - smooth(w - 0.8, w + 0.8, x.min(63.99 - x).min(y).min(63.99 - y)) }
fn beam_px(t: &Tile, x: f64, y: f64) -> S { let s = dark_timber(t, x, y); S { c: s.c, h: s.h * 0.4 + 0.6, r: s.r } }
/// Marbre : fond blanc crème, veines grises sinueuses.
fn marble(t: &Tile, x: f64, y: f64) -> S {
    let w = t.fbm(x, y, 4, 4); let vein = (((x + y * 0.6) * 0.09 + 6.0 * w).sin()).abs(); let v = 1.0 - smooth(0.0, 0.08, vein);
    let v2 = 1.0 - smooth(0.0, 0.05, (((x * 0.7 - y) * 0.13 + 5.0 * t.fbm(x + 9.0, y, 4, 3)).sin()).abs());
    S { c: mul(pal(&MARBLE, 0.55 + 0.4 * t.fbm(x, y, 8, 2)), 1.0 - 0.35 * v - 0.15 * v2), h: 0.6, r: 0.25 }
}
/// Pierre de taille : deux assises de grands blocs, joints fins, layage du ciseau.
fn ashlar(t: &Tile, x: f64, y: f64) -> S {
    let (h, w) = (32.0, 48.0); let row = (y / h).floor(); let xo = x + if row as i64 % 2 == 0 { 0.0 } else { 24.0 }; let col = (xo / w).floor(); let (fx, fy) = (xo - col * w, y - row * h);
    let j = fx.min(w - fx).min(fy.min(h - fy)); let m = smooth(0.4, 1.2, j); let id = row as i64 * 7 + col as i64; let tool = 0.04 * ((x * 1.7 + y * 0.4).sin());
    let n = t.fbm(x, y, 8, 3); let tone = 0.3 + 0.4 * t.pi(id) + 0.25 * (n - 0.5) + tool;
    S { c: mul(pal(&ASHLAR, tone), 0.55 + 0.45 * m), h: 0.2 + 0.7 * m, r: 0.85 }
}
/// Moellons : pierres des champs arrondies, hourdées à la chaux.
fn fieldstone(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x * 1.0, y * 1.35, 5); let gap = smooth(0.02, 0.2, d2 - d1); let n = t.fbm(x, y, 8, 3);
    if gap < 0.35 { return S { c: mul([0.72, 0.68, 0.6], 0.85 + 0.2 * n), h: 0.15, r: 0.95 }; }
    let tone = 0.15 + 0.6 * t.pi(id) + 0.3 * (n - 0.5); let warm = t.pi(id + 77);
    let c = pal(&FIELD, tone); S { c: [c[0] * (0.95 + 0.12 * warm), c[1], c[2] * (1.02 - 0.1 * warm)], h: 0.3 + 0.6 * gap * (0.6 + 0.4 * n), r: 0.82 }
}
/// Pavés de granit, rectangulaires, en rangs décalés : la chaussée des villes.
fn setts(t: &Tile, x: f64, y: f64) -> S {
    let (h, w) = (12.8, 21.33); let row = (y / h).floor(); let xo = x + t.pi(row as i64 * 5) * w; let col = (xo / w).floor(); let (fx, fy) = (xo - col * w, y - row * h);
    let j = fx.min(w - fx).min(fy.min(h - fy)); let m = smooth(0.4, 2.2, j); let id = row as i64 * 13 + col as i64; let n = t.fbm(x, y, 16, 2);
    S { c: mul(pal(&SETT, 0.2 + 0.6 * t.pi(id) + 0.2 * (n - 0.5)), 0.3 + 0.7 * m), h: 0.1 + 0.85 * m * (0.8 + 0.2 * n), r: 0.7 }
}
/// Pierre polie : grande dalle lisse, filet gravé au bord.
fn polished(t: &Tile, x: f64, y: f64) -> S {
    let e = x.min(63.99 - x).min(y).min(63.99 - y); let groove = 1.0 - smooth(0.4, 1.4, (e - 3.0).abs()); let n = t.fbm(x, y, 4, 3);
    S { c: mul(pal(&STONE, 0.62 + 0.18 * n + 0.05 * t.val(x, y, 32)), 1.0 - 0.3 * groove - 0.15 * (1.0 - smooth(0.0, 1.2, e))), h: 0.7 - 0.4 * groove, r: 0.35 }
}
/// Damier de marbre noir et blanc, quatre carreaux par bloc.
fn checker(t: &Tile, x: f64, y: f64) -> S {
    let (cx, cy) = ((x / 32.0).floor() as i64, (y / 32.0).floor() as i64); let dark = (cx + cy) % 2 == 1; let m = marble(t, x, y);
    let j = (x % 32.0).min(32.0 - x % 32.0).min((y % 32.0).min(32.0 - y % 32.0)); let g = smooth(0.0, 0.8, j);
    let c = if dark { [0.16 + m.c[0] * 0.12, 0.16 + m.c[1] * 0.12, 0.17 + m.c[2] * 0.13] } else { m.c }; S { c: mul(c, 0.7 + 0.3 * g), h: 0.5 + 0.2 * g, r: 0.2 }
}
/// Tomettes de terre cuite, carrées, joints de chaux.
fn terracotta(t: &Tile, x: f64, y: f64) -> S {
    let (cx, cy) = ((x / 32.0).floor(), (y / 32.0).floor()); let j = (x - cx * 32.0).min(32.0 - (x - cx * 32.0)).min((y - cy * 32.0).min(32.0 - (y - cy * 32.0))); let m = smooth(0.6, 1.6, j);
    if m < 0.4 { return S { c: [0.66, 0.6, 0.5], h: 0.2, r: 0.95 }; }
    let id = cx as i64 * 3 + cy as i64 * 7; let n = t.fbm(x, y, 8, 3); let wear = smooth(0.5, 0.8, t.fbm(x + 5.0, y, 4, 2));
    S { c: mul(pal(&TERRA, 0.3 + 0.4 * t.pi(id) + 0.25 * (n - 0.5) + 0.15 * wear), 0.85 + 0.15 * m), h: 0.5 + 0.3 * m, r: 0.6 }
}
/// Parquet à bâtons rompus de chêne ciré.
fn parquet(t: &Tile, x: f64, y: f64) -> S {
    let (u, v) = (x + y, y - x); let (bw, bl) = (8.0, 32.0); let col = (u / bw).floor(); let flip = col as i64 % 2 == 0;
    let along = if flip { v } else { -v } + col * 8.0; let k = (along / bl).floor(); let fa = along - k * bl; let fu = u - col * bw;
    let j = fu.min(bw - fu).min(fa.min(bl - fa)); let m = smooth(0.2, 0.9, j); let id = col as i64 * 17 + k as i64;
    let grain = t.val(fa * 0.3 + col * 13.0, fu * 2.0, 32); let tone = 0.3 + 0.45 * t.pi(id) + 0.2 * (grain - 0.5);
    S { c: mul(pal(&PARQ, tone), 0.55 + 0.45 * m), h: 0.5 + 0.3 * m, r: 0.45 }
}
/// Bardeaux de châtaignier fendus, en rangs décalés, bouts usés.
fn shingles(t: &Tile, x: f64, y: f64) -> S {
    let h = 16.0; let row = (y / h).floor(); let w = 9.0 + 5.0 * t.pi(row as i64 * 3); let xo = x + t.pi(row as i64 * 11) * w; let col = (xo / w).floor();
    let (fx, fy) = ((xo - col * w) / w, (y - row * h) / h); let id = row as i64 * 19 + col as i64; let gap = smooth(0.0, 0.1, fx.min(1.0 - fx));
    let grain = t.val(x * 3.0, y * 0.3, 64); let shade = 0.55 + 0.45 * smooth(0.0, 0.4, fy); let tone = 0.2 + 0.6 * t.pi(id) + 0.2 * (grain - 0.5);
    S { c: mul(pal(&SHINGLE, tone), shade * (0.4 + 0.6 * gap)), h: (0.2 + 0.7 * fy) * gap, r: 0.85 }
}
/// Drap de laine foulée : trame et chaîne fines, un peu de bourre. Blanc cassé, teint par le bloc.
fn wool(t: &Tile, x: f64, y: f64) -> S {
    let weave = 0.5 + 0.25 * ((x * 1.6).sin() * (y * 1.6).cos()); let fuzz = t.fbm(x, y, 16, 3);
    S { c: mul(pal(&WOOL, 0.35 + 0.4 * fuzz + 0.2 * (weave - 0.5)), 1.0), h: 0.4 + 0.3 * weave + 0.2 * fuzz, r: 0.98 }
}
/// Tapis d'Orient : bordure, galons, médaillon losangé, motifs répétés.
fn rug(t: &Tile, x: f64, y: f64) -> S {
    let red = [0.5, 0.1, 0.09]; let blue = [0.12, 0.17, 0.36]; let gold = [0.78, 0.6, 0.28]; let cream = [0.85, 0.78, 0.62];
    let e = x.min(63.99 - x).min(y).min(63.99 - y); let (cx, cy) = ((x - 32.0).abs(), (y - 32.0).abs()); let d = cx + cy;
    let c = if e < 3.0 { blue } else if e < 4.5 { gold } else if e < 8.0 { if ((x + y) * 0.5).sin() > 0.3 { cream } else { blue } } else if e < 9.5 { gold }
        else if d < 9.0 { if d < 5.0 { gold } else { blue } } else if d < 11.0 { cream } else if ((x * 0.6).sin() * (y * 0.6).sin()) > 0.55 { gold } else { red };
    let fuzz = t.fbm(x, y, 16, 2); S { c: mul(c, 0.85 + 0.25 * fuzz), h: 0.5 + 0.2 * fuzz, r: 0.98 }
}
/// Pan de bois : torchis dans un cadre de poutres, avec ou sans écharpes.
fn frame(t: &Tile, x: f64, y: f64, kind: u8) -> S {
    let b = border(x, y, 5.0); let d1 = (x - y).abs() / std::f64::consts::SQRT_2; let d2 = (x + y - 63.99).abs() / std::f64::consts::SQRT_2;
    let brace = match kind { 1 => 1.0 - smooth(3.0, 4.2, d1.min(d2)), 2 => 1.0 - smooth(3.0, 4.2, d2), _ => 0.0 };
    if b.max(brace) > 0.5 { return beam_px(t, x * 0.5 + if brace > b { 13.0 } else { 0.0 }, y); }
    let p = plaster(t, x, y); S { c: p.c, h: p.h * 0.7, r: p.r }
}
/// Clayonnage : branches de noisetier tressées entre des piquets.
fn wattle(t: &Tile, x: f64, y: f64) -> S {
    let stake = (x % 16.0 - 8.0).abs(); let row = (y / 8.0).floor(); let over = ((x / 16.0).floor() as i64 + row as i64) % 2 == 0;
    let wav = ((x / 16.0 * std::f64::consts::PI).sin() * if over { 1.0 } else { -1.0 }) * 1.2; let fy = (y - row * 8.0 - 4.0 + wav).abs();
    let rod = 1.0 - smooth(2.6, 3.8, fy); let g = t.val(x * 2.0, y * 4.0, 64); let bark = [0.38 + 0.1 * g, 0.28 + 0.07 * g, 0.17 + 0.04 * g];
    if stake < 1.6 && !over { return S { c: mul(bark, 0.8), h: 0.9, r: 0.9 }; }
    S { c: mul(bark, 0.25 + 0.75 * rod * (0.7 + 0.3 * (1.0 - fy / 4.0))), h: 0.2 + 0.7 * rod, r: 0.9 }
}
/// Rayonnage de livres : deux étagères, dos colorés de hauteurs inégales.
fn books(t: &Tile, x: f64, y: f64) -> S {
    let shelf = (y % 32.0) < 4.0 || (y % 32.0) > 30.0; if shelf || x < 3.0 || x > 61.0 { let p = planks(t, x, y, &OAK_W); return S { c: mul(p.c, 0.75), h: 0.9, r: 0.8 }; }
    let row = (y / 32.0).floor() as i64; let mut acc = 3.0; let mut k = 0i64; let mut wdt = 4.0;
    while acc < 61.0 { wdt = 3.0 + (t.pi(row * 97 + k) * 4.0).floor(); if x < acc + wdt { break; } acc += wdt; k += 1; }
    let id = row * 97 + k; let top = 4.0 + 4.0 * t.pi(id + 5); let fy = y % 32.0; if fy < top { return S { c: [0.06, 0.05, 0.04], h: 0.1, r: 1.0 }; }
    let cols: [C; 7] = [[0.42, 0.1, 0.08], [0.12, 0.2, 0.36], [0.16, 0.3, 0.14], [0.45, 0.3, 0.14], [0.3, 0.12, 0.25], [0.55, 0.45, 0.28], [0.2, 0.16, 0.12]];
    let c = cols[(t.pi(id + 9) * 7.0) as usize % 7]; let edge = smooth(0.0, 0.8, (x - acc).min(acc + wdt - x)); let band = ((fy - top - 5.0).abs() < 1.0) as i32 as f64;
    S { c: mul(if band > 0.0 { [0.75, 0.6, 0.3] } else { c }, 0.6 + 0.4 * edge), h: 0.6 + 0.3 * edge, r: 0.8 }
}
/// Botte de paille pressée, deux liens de chanvre.
fn hay(t: &Tile, x: f64, y: f64) -> S {
    let strand = t.val(x * 0.3, y * 3.0, 64); let n = t.fbm(x, y, 8, 3); let tie = ((x - 16.0).abs() < 2.0 || (x - 48.0).abs() < 2.0) as i32 as f64;
    let c = if tie > 0.0 { [0.42, 0.32, 0.18] } else { pal(&STRAW, 0.3 + 0.5 * strand + 0.2 * (n - 0.5)) }; S { c, h: 0.4 + 0.5 * strand, r: 0.95 }
}
/// Caisse : cadre de planches et écharpe, clous.
fn crate_tex(t: &Tile, x: f64, y: f64) -> S {
    let b = border(x, y, 7.0); let brace = 1.0 - smooth(3.5, 4.5, (x - y).abs() / std::f64::consts::SQRT_2);
    let p = planks(t, x, y, &OAK_W); let nail = ((x - 3.5).abs() < 1.2 || (x - 60.5).abs() < 1.2) && ((y - 3.5).abs() < 1.2 || (y - 60.5).abs() < 1.2);
    if nail { return S { c: [0.15, 0.15, 0.16], h: 1.0, r: 0.4 }; }
    if b.max(brace) > 0.5 { return S { c: mul(p.c, 0.85), h: 0.9, r: 0.85 }; } S { c: mul(p.c, 0.62), h: 0.45, r: 0.9 }
}
/// Pierre sculptée : panneau à moulures et rosace.
fn chiseled(t: &Tile, x: f64, y: f64) -> S {
    let e = x.min(63.99 - x).min(y).min(63.99 - y); let r = ((x - 32.0).powi(2) + (y - 32.0).powi(2)).sqrt(); let a = (y - 32.0).atan2(x - 32.0);
    let petal = (r - 9.0 - 4.0 * (a * 6.0).cos()).abs(); let carve = (1.0 - smooth(0.6, 1.6, (e - 6.0).abs())).max(1.0 - smooth(0.6, 1.6, petal)).max(1.0 - smooth(0.5, 1.5, (r - 3.0).abs()));
    let s = ashlar(t, x * 0.5, y * 0.5); S { c: mul(pal(&ASHLAR, 0.45 + 0.3 * t.fbm(x, y, 8, 3)), 1.0 - 0.4 * carve - 0.2 * (1.0 - smooth(0.0, 1.5, e))), h: 0.7 - 0.5 * carve + 0.0 * s.h, r: 0.85 }
}
/// Vitrail : verres de couleur sertis de plomb, en losanges.
fn stained(t: &Tile, x: f64, y: f64) -> S {
    let (u, v) = ((x + y) / 16.0, (y - x) / 16.0); let (iu, iv) = (u.floor() as i64, v.floor() as i64); let (fu, fv) = (u - u.floor(), v - v.floor());
    let lead = 1.0 - smooth(0.05, 0.1, fu.min(1.0 - fu).min(fv.min(1.0 - fv))); let e = x.min(63.99 - x).min(y).min(63.99 - y);
    if lead > 0.5 || e < 2.5 { return S { c: [0.12, 0.12, 0.13], h: 0.9, r: 0.5 }; }
    let cols: [C; 5] = [[0.75, 0.12, 0.1], [0.12, 0.25, 0.7], [0.85, 0.65, 0.15], [0.15, 0.5, 0.25], [0.5, 0.18, 0.55]];
    let c = cols[(t.pi(iu * 7 + iv * 13 + 300) * 5.0) as usize % 5]; let n = t.fbm(x, y, 8, 2); S { c: mul(c, 0.8 + 0.4 * n), h: 0.4, r: 0.15 }
}
/// Tôle de fer forgé, plaques rivetées.
fn iron_plate(t: &Tile, x: f64, y: f64) -> S {
    let (fx, fy) = (x % 32.0, y % 32.0); let seam = 1.0 - smooth(0.4, 1.2, fx.min(32.0 - fx).min(fy.min(32.0 - fy)));
    let rivet = [(4.0, 4.0), (28.0, 4.0), (4.0, 28.0), (28.0, 28.0), (16.0, 4.0), (16.0, 28.0)].iter().any(|(a, b)| (fx - a).powi(2) + (fy - b).powi(2) < 3.2);
    let n = t.fbm(x, y, 8, 3); let rust = smooth(0.6, 0.85, t.fbm(x + 3.0, y + 8.0, 4, 3));
    let c = pal(&IRON, 0.35 + 0.35 * n); let c = [lerp(c[0], 0.4, rust * 0.5), lerp(c[1], 0.2, rust * 0.5), lerp(c[2], 0.1, rust * 0.5)];
    S { c: mul(c, if rivet { 1.5 } else { 1.0 - 0.4 * seam }), h: if rivet { 1.0 } else { 0.6 - 0.4 * seam }, r: if rivet { 0.3 } else { 0.55 } }
}
/// Cuivre vert-de-grisé : feuilles à joints debout.
fn verdigris(t: &Tile, x: f64, y: f64) -> S {
    let seam = 1.0 - smooth(0.6, 1.6, (x % 16.0).min(16.0 - x % 16.0)); let n = t.fbm(x, y, 8, 3); let streak = t.val(x * 2.0, y * 0.2, 32);
    let c = pal(&VERD, 0.3 + 0.45 * n + 0.2 * (streak - 0.5)); let copper = smooth(0.75, 0.9, t.fbm(x + 7.0, y, 4, 3));
    let c = [lerp(c[0], 0.6, copper * 0.6), lerp(c[1], 0.35, copper * 0.6), lerp(c[2], 0.2, copper * 0.6)]; S { c: mul(c, 1.0 + 0.25 * seam), h: 0.5 + 0.4 * seam, r: 0.45 }
}
fn farmland(t: &Tile, x: f64, y: f64) -> S {
    let furrow = 0.5 + 0.5 * (y / 64.0 * std::f64::consts::TAU * 4.0).cos(); let n = t.fbm(x, y, 5, 3); let d = dirt(t, x, y, &DIRT);
    S { c: mul(d.c, 0.62 + 0.3 * furrow + 0.08 * (n - 0.5)), h: 0.2 + 0.6 * furrow, r: 0.9 }
}
fn snow(t: &Tile, x: f64, y: f64) -> S {
    let (n, s, sp) = (t.fbm(x, y, 4, 3), t.val(x, y, 32), t.pi((x as i64) * 67 + (y as i64) * 131)); let glint = if sp > 0.985 { 0.12 } else { 0.0 };
    S { c: pal(&SNOW, 0.25 + 0.55 * n + 0.2 * (s - 0.5) + glint), h: 0.35 + 0.4 * n, r: 0.6 }
}
fn snow_side(t: &Tile, x: f64, y: f64) -> S {
    let fringe = 10.0 + 7.0 * t.val(x, 0.0, 16) + 4.0 * t.val(x, 0.0, 32);
    if y < fringe { return snow(t, x, y); } let mut d = stone(t, x, y, &STONE, 5); if y < fringe + 3.0 { d.c = mul(d.c, 0.75); } d
}
fn ice(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, _) = t.vor(x, y, 5); let crack = smooth(0.06, 0.0, d2 - d1); let (n, s) = (t.fbm(x, y, 3, 3), t.val(x, y, 16)); let bub = if t.val(x, y, 48) > 0.82 { 0.12 } else { 0.0 };
    S { c: mul(pal(&ICE, 0.25 + 0.5 * n + 0.2 * (s - 0.5) + bub + 0.35 * crack), 1.0), h: 0.6 - 0.5 * crack, r: 0.12 + 0.3 * crack }
}
/// Cerisier en fleur : des grappes de fleurs roses et blanches serrées, cinq pétales pâles autour d'un cœur plus vif, et çà
/// et là une jeune feuille vert tendre entre elles.
fn leaf_cherry(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x, y, 9); let rim = smooth(0.0, 0.16, d2 - d1); let heart = smooth(0.18, 0.04, d1); let (m, g) = (t.fbm(x, y, 5, 2), t.val(x, y, 32)); let pi = t.pi(id);
    if pi < 0.12 { let l = leaf(t, x, y, [1.05, 1.2, 0.8]); return S { c: l.c, h: l.h, r: 0.5 }; } // une jeune feuille
    let pink = [0.97 - 0.1 * m, 0.74 - 0.2 * pi + 0.1 * g, 0.84 - 0.1 * pi]; let white = [0.98, 0.93, 0.95]; let w = smooth(0.55, 0.95, pi) * 0.7;
    let c = [lerp(pink[0], white[0], w), lerp(pink[1], white[1], w), lerp(pink[2], white[2], w)]; let core = [0.82, 0.36, 0.5]; let k = 0.62 + 0.38 * rim;
    S { c: [lerp(c[0], core[0], heart * 0.6) * k, lerp(c[1], core[1], heart * 0.6) * k, lerp(c[2], core[2], heart * 0.6) * k], h: rim * (0.6 + 0.4 * pi), r: 0.45 }
}
/// Côté d'un étage d'épicéa enneigé : la neige déborde du dessus, les aiguilles sombres dessous.
fn leaf_snow(t: &Tile, x: f64, y: f64) -> S {
    let fringe = 9.0 + 9.0 * t.val(x, 0.0, 16) + 5.0 * t.val(x, 0.0, 32);
    if y < fringe { let s = snow(t, x, y); return S { c: s.c, h: 0.75, r: 0.6 }; } needles(t, x, y)
}
fn gen(name: &str, t: &Tile, x: f64, y: f64) -> S {
    if let Some((m, _)) = cave_mat(name, t, x, y) { return m; }
    let b_oak = |a: &Tile, b: f64, c: f64| bark(a, b, c, &OAK, 1.0); let b_beech = |a: &Tile, b: f64, c: f64| bark(a, b, c, &BEECH, 0.28);
    match name {
        "grass_top" => grass_top(t, x, y), "grass_side" => grass_side(t, x, y), "dirt" => dirt(t, x, y, &DIRT), "path" => dirt(t, x, y, &PATH), "forest_soil" => dirt(t, x, y, &SOIL),
        "stone" => stone(t, x, y, &STONE, 5), "mossy_stone" => mossy(t, x, y), "limestone" => limestone(t, x, y), "flowstone" => flowstone(t, x, y), "granite" => granite(t, x, y), "gravel" => gravel(t, x, y), "sand" => dirt(t, x, y, &SAND),
        "bark_oak" => b_oak(t, x, y), "bark_birch" => birch(t, x, y), "bark_beech" => b_beech(t, x, y),
        "leaf_oak" => leaf(t, x, y, [1.0, 1.0, 1.0]), "leaf_cherry" => leaf_cherry(t, x, y), "leaf_birch" => leaf(t, x, y, [1.2, 1.12, 0.9]), "leaf_beech" => leaf(t, x, y, [0.86, 1.0, 0.82]),
        "end_oak" => wood_end(t, x, y, &OAK_W, 0.78, &b_oak), "end_birch" => wood_end(t, x, y, &BIRCH_W, 0.95, &birch), "end_beech" => wood_end(t, x, y, &BEECH_W, 0.85, &b_beech),
        "planks" => planks(t, x, y, &PLANKS), "planks_birch" => planks(t, x, y, &PLANKS_BIRCH), "planks_spruce" => planks(t, x, y, &PLANKS_SPRUCE),
        "ore_copper" => ore(t, x, y, [0.32, 0.62, 0.48], [0.78, 0.46, 0.24]), "ore_tin" => ore(t, x, y, [0.8, 0.82, 0.84], [0.58, 0.6, 0.62]), "ore_iron" => ore(t, x, y, [0.7, 0.42, 0.28], [0.52, 0.28, 0.18]), "ore_coal" => ore(t, x, y, [0.07, 0.07, 0.08], [0.16, 0.16, 0.17]),
        "clay" => clay(t, x, y), "stone_bricks" => bricks(t, x, y, 16.0, 32.0, &CUT_STONE, [0.36, 0.34, 0.31]), "bricks" => bricks(t, x, y, 8.0, 16.0, &BRICK, [0.74, 0.7, 0.62]),
        "thatch" => thatch(t, x, y), "cobble" => cobble(t, x, y), "farmland_top" => farmland(t, x, y),
        "lime_masonry" => lime_masonry(t, x, y), "rubble" => rubble(t, x, y), "beam_end" => beam_end(t, x, y), "boards" => boards(t, x, y), "tiles_moss" => tiles_moss(t, x, y), "path_top" => path_top(t, x, y),
        "plaster" => plaster(t, x, y), "dark_timber" => dark_timber(t, x, y), "roof_tiles" => roof_tiles(t, x, y), "slate" => slate(t, x, y),
        "marble" => marble(t, x, y), "ashlar" => ashlar(t, x, y), "fieldstone" => fieldstone(t, x, y), "setts" => setts(t, x, y), "polished_stone" => polished(t, x, y),
        "checker" => checker(t, x, y), "terracotta" => terracotta(t, x, y), "parquet" => parquet(t, x, y), "shingles" => shingles(t, x, y), "wool" => wool(t, x, y),
        "rug" => rug(t, x, y), "frame_square" => frame(t, x, y, 0), "frame_cross" => frame(t, x, y, 1), "frame_diag" => frame(t, x, y, 2),
        "wattle" => wattle(t, x, y), "books" => books(t, x, y), "hay" => hay(t, x, y), "crate" => crate_tex(t, x, y), "chiseled" => chiseled(t, x, y),
        "stained_glass" => stained(t, x, y), "iron_plate" => iron_plate(t, x, y), "verdigris" => verdigris(t, x, y),
        "snow" => snow(t, x, y), "snow_side" => snow_side(t, x, y), "ice" => ice(t, x, y),
        "bark_spruce" => bark(t, x, y, &SPRUCE, 0.7), "end_spruce" => wood_end(t, x, y, &SPRUCE_W, 0.8, &|a: &Tile, b: f64, c: f64| bark(a, b, c, &SPRUCE, 0.7)),
        "leaf_spruce" => needles(t, x, y), "leaf_snow" => leaf_snow(t, x, y),
        "bark_dead" => bark(t, x, y, &DEAD, 0.95), "end_dead" => wood_end(t, x, y, &DEAD_W, 0.8, &|a: &Tile, b: f64, c: f64| bark(a, b, c, &DEAD, 0.95)), "lichen" => lichen(t, x, y),
        "bark_acacia" => bark(t, x, y, &ACACIA, 1.0), "end_acacia" => wood_end(t, x, y, &ACACIA_W, 0.75, &|a: &Tile, b: f64, c: f64| bark(a, b, c, &ACACIA, 1.0)), "leaf_acacia" => leaf(t, x, y, [0.62, 0.7, 0.4]), "bone" => bone(t, x, y), "wood_inner" => wood_inner(t, x, y),
        "sandstone" => sandstone(t, x, y, [0.82, 0.68, 0.48], [0.7, 0.55, 0.36]), "sandstone_top" => sandstone_top(t, x, y), "red_sandstone" => sandstone(t, x, y, [0.74, 0.42, 0.26], [0.6, 0.32, 0.2]),
        "cactus_side" => cactus_side(t, x, y), "cactus_top" => cactus_top(t, x, y), "bark_palm" => bark_palm(t, x, y), "end_palm" => end_palm(t, x, y), "leaf_palm" => leaf(t, x, y, [0.8, 0.95, 0.55]),
        "grass_dry_top" => dry_top(t, x, y), "grass_dry_side" => dry_side(t, x, y), "dry_earth" => cracked(t, x, y),
        "coral" => coral(t, x, y), "planks_wreck" => planks_wreck(t, x, y),
        "water_foam" => { let f = t.fbm(x, y, 8, 3); S { c: [f, f, f], h: f, r: 0.3 } }
        _ => panic!("matériau sans générateur : {name}"),
    }
}
pub struct Textures { pub albedo: Vec<u8>, pub nrm: Vec<u8>, pub layers: u32, pub size: u32 }
/// Deux piles RGBA : albédo, puis normale XY et rugosité. Le motif vit dans un repère de 64 unités ; à 128 pixels, une octave fine ajoute un vrai détail.
pub fn build_textures(size: usize) -> Textures {
    // Pleine résolution, net : le motif est lu en chaque texel (256 par bloc en jeu), sans grain aléatoire ajouté par texel —
    // c'était lui, amplifié dans la carte de normales, qui donnait l'aspect duveteux « en laine ». Le relief se lit sur la
    // hauteur légèrement lissée (noyau 3 × 3) : les joints, les arêtes et les fentes gardent leur modelé franc, le
    // bruit fin de la matière ne crée plus de micro-bosses.
    let layers = MATERIALS.len() * VARIANTS as usize; let n = size * size * 4 * layers;
    let (mut albedo, mut nrm) = (vec![0u8; n], vec![0u8; n]);
    let k = TEX / size as f64; let gain = 0.62 * size as f64 / TEX;
    let b8 = |v: f64| (clamp(v, 0.0, 1.0) * 255.0) as u8;
    let mut hh = vec![0f64; size * size]; let mut sm = vec![0f64; size * size];
    let wrap = |v: usize, d: i32| (v as i32 + d).rem_euclid(size as i32) as usize;
    for (m, name) in MATERIALS.iter().enumerate() {
        for v in 0..VARIANTS as usize {
            let mut t = Tile::new((1000 + m * 17 + v * 3) as u32); t.v = v; let layer = m * VARIANTS as usize + v; let base = layer * size * size * 4;
            let amp = grain(name);
            for y in 0..size { for x in 0..size { let (lx, ly) = ((x as f64 + 0.5) * k, (y as f64 + 0.5) * k); let mut c = gen(name, &t, lx, ly); let p = base + (x + y * size) * 4;
                // Un grain doux de la matière (cellules de deux texels, pas de bruit texel par texel) : de la vie de près, sans duvet.
                let d = t.val(lx * 2.0, ly * 2.0, 128) - 0.5 + 0.5 * (t.val(lx * 4.0 + 7.0, ly * 4.0, 256) - 0.5); let g = 1.0 + d * amp * 1.6; c.h += d * amp * 0.35;
                albedo[p] = b8(c.c[0] * g); albedo[p + 1] = b8(c.c[1] * g); albedo[p + 2] = b8(c.c[2] * g); albedo[p + 3] = 255 - (cave_mat(name, &t, lx, ly).map_or(0.0, |m| m.1).clamp(0.0, 1.0) * 255.0) as u8; nrm[p + 2] = (c.r * 255.0) as u8; nrm[p + 3] = 255; hh[x + y * size] = c.h; } }
            for y in 0..size { for x in 0..size { let mut acc = 0.0; for (dy, wy) in [(-1, 1.0), (0, 2.0), (1, 1.0)] { for (dx, wx) in [(-1, 1.0), (0, 2.0), (1, 1.0)] { acc += hh[wrap(x, dx) + wrap(y, dy) * size] * wx * wy; } } sm[x + y * size] = acc / 16.0; } }
            let gain = gain * relief(name);
            for y in 0..size { for x in 0..size { let p = base + (x + y * size) * 4;
                let dx = sm[wrap(x, 1) + y * size] - sm[wrap(x, -1) + y * size]; let dy = sm[x + wrap(y, 1) * size] - sm[x + wrap(y, -1) * size];
                nrm[p] = b8(0.5 - dx * gain); nrm[p + 1] = b8(0.5 - dy * gain); } }
        }
    }
    Textures { albedo, nrm, layers: layers as u32, size: size as u32 }
}
/// Le grain fin de chaque matière : marqué pour l'herbe, les feuilles, l'écorce, la roche brute, le sable ; discret pour la
/// maçonnerie, les tuiles, les planches et les enduits.
fn grain(n: &str) -> f64 {
    if n.starts_with("leaf") || n.starts_with("grass") || n.contains("moss") { 0.16 } else if n.starts_with("bark") || n.starts_with("end_") { 0.12 }
    else if matches!(n, "stone" | "granite" | "limestone" | "gravel" | "dirt" | "dry_earth" | "path" | "path_top" | "sand" | "sandstone" | "snow" | "lichen") { 0.11 }
    else if matches!(n, "planks" | "planks_birch" | "planks_spruce" | "dark_timber" | "boards" | "wood_inner") { 0.07 } else { 0.05 }
}
/// Relief propre à chaque matière : la maçonnerie, les toitures et le bois de charpente sont creusés plus franchement
/// (joints, rangs de tuiles, planches qui se chevauchent) ; le reste garde un relief doux.
fn relief(n: &str) -> f64 {
    match n { "stone_bricks" | "bricks" | "cobble" | "fieldstone" | "ashlar" | "setts" | "chiseled" | "lime_masonry" => 2.0, "rubble" | "roof_tiles" | "slate" | "shingles" | "thatch" | "tiles_moss" => 2.2, "boards" => 1.8, "beam_end" => 1.5, "path_top" => 1.2,
        "dark_timber" | "planks" | "planks_birch" | "planks_spruce" | "planks_wreck" | "parquet" | "wattle" | "crate" | "books" => 1.7, "plaster" | "terracotta" | "wood_inner" | "iron_plate" => 1.4, _ => 1.0 }
}
/// Nombre de pixels d'art par côté de bloc.
pub const ART: usize = 64;

/// Niveaux de détail réduits par moyenne, du plus grand au plus petit, le niveau 0 compris.
pub fn mip_chain(base: &[u8], size: usize, layers: usize) -> Vec<Vec<u8>> {
    let mut out = vec![base.to_vec()]; let mut s = size;
    while s > 1 {
        let n = s / 2; let src = out.last().unwrap(); let mut d = vec![0u8; n * n * 4 * layers];
        for l in 0..layers { for y in 0..n { for x in 0..n { for c in 0..4 {
            let a = |i: usize, j: usize| src[l * s * s * 4 + (i + j * s) * 4 + c] as u32;
            d[l * n * n * 4 + (x + y * n) * 4 + c] = ((a(x * 2, y * 2) + a(x * 2 + 1, y * 2) + a(x * 2, y * 2 + 1) + a(x * 2 + 1, y * 2 + 1) + 2) / 4) as u8;
        } } } }
        out.push(d); s = n;
    }
    out
}

/// Bruit 3D périodique, pour les nuages, la brume et l'eau.
pub fn build_noise_3d(n: usize) -> Vec<u8> {
    let total = n * n * n; let mut r = Rng::new(9001); let g: Vec<f32> = (0..total).map(|_| r.next() as f32).collect(); let nf = n as f64;
    let samp = |x: usize, y: usize, z: usize, f: usize| {
        let s = nf / f as f64; let (fx, fy, fz) = (x as f64 / s, y as f64 / s, z as f64 / s); let (xx, yy, zz) = (fx.floor(), fy.floor(), fz.floor());
        let (u, v, w) = (fx - xx, fy - yy, fz - zz); let sm = |t: f64| t * t * (3.0 - 2.0 * t); let (su, sv, sw) = (sm(u), sm(v), sm(w));
        let h = |i: usize, j: usize, k: usize| g[((i % f) * 3 + (j % f) * n * 5 + (k % f) * n * n * 7) % total] as f64; let (xi, yi, zi) = (xx as usize, yy as usize, zz as usize);
        lerp(lerp(lerp(h(xi, yi, zi), h(xi + 1, yi, zi), su), lerp(h(xi, yi + 1, zi), h(xi + 1, yi + 1, zi), su), sv), lerp(lerp(h(xi, yi, zi + 1), h(xi + 1, yi, zi + 1), su), lerp(h(xi, yi + 1, zi + 1), h(xi + 1, yi + 1, zi + 1), su), sv), sw)
    };
    let mut d = vec![0u8; total];
    for z in 0..n { for y in 0..n { for x in 0..n {
        let a = samp(x, y, z, 4) * 0.55 + samp(x, y, z, 8) * 0.28 + samp(x, y, z, 16) * 0.17;
        d[x + y * n + z * n * n] = (clamp((a - 0.2) / 0.6, 0.0, 1.0) * 255.0) as u8;
    } } }
    d
}
