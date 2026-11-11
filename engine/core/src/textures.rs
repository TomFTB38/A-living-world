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

struct Tile { n: f64, p: Vec<f32> }
impl Tile {
    fn new(seed: u32) -> Self { let mut r = Rng::new(seed); Tile { n: TEX, p: (0..1024).map(|_| r.next() as f32).collect() } }
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
fn limestone(t: &Tile, x: f64, y: f64) -> S {
    let big = t.fbm(x, y, 2, 3); let mid = t.fbm(x + 41.0, y + 3.0, 6, 4); let fine = t.val(x, y, 48); let bed = 0.5 + 0.5 * sin((y + 6.0 * t.fbm(x, y + 9.0, 3, 2)) * std::f64::consts::PI * 4.0 / 64.0);
    let stain = smooth(0.5, 0.75, t.fbm(x - 13.0, y + 40.0, 4, 3)); let pit = smooth(0.82, 0.9, t.val(x + 5.0, y - 3.0, 32));
    let v = 0.72 + 0.08 * (big - 0.5) + 0.07 * (mid - 0.5) + 0.03 * (fine - 0.5) + 0.025 * (bed - 0.5);
    let c = [v * (1.0 - 0.1 * stain) * (1.0 - 0.35 * pit), v * 0.96 * (1.0 - 0.1 * stain) * (1.0 - 0.35 * pit), v * 0.86 * (1.0 - 0.06 * stain) * (1.0 - 0.35 * pit)];
    S { c, h: 0.5 + 0.15 * (mid - 0.5) + 0.05 * bed - 0.4 * pit, r: 0.85 }
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
/// Côté d'un étage d'épicéa enneigé : la neige déborde du dessus, les aiguilles sombres dessous.
fn leaf_snow(t: &Tile, x: f64, y: f64) -> S {
    let fringe = 9.0 + 9.0 * t.val(x, 0.0, 16) + 5.0 * t.val(x, 0.0, 32);
    if y < fringe { let s = snow(t, x, y); return S { c: s.c, h: 0.75, r: 0.6 }; } needles(t, x, y)
}
fn gen(name: &str, t: &Tile, x: f64, y: f64) -> S {
    let b_oak = |a: &Tile, b: f64, c: f64| bark(a, b, c, &OAK, 1.0); let b_beech = |a: &Tile, b: f64, c: f64| bark(a, b, c, &BEECH, 0.28);
    match name {
        "grass_top" => grass_top(t, x, y), "grass_side" => grass_side(t, x, y), "dirt" => dirt(t, x, y, &DIRT), "path" => dirt(t, x, y, &PATH), "forest_soil" => dirt(t, x, y, &SOIL),
        "stone" => stone(t, x, y, &STONE, 5), "mossy_stone" => mossy(t, x, y), "limestone" => limestone(t, x, y), "granite" => granite(t, x, y), "gravel" => gravel(t, x, y), "sand" => dirt(t, x, y, &SAND),
        "bark_oak" => b_oak(t, x, y), "bark_birch" => birch(t, x, y), "bark_beech" => b_beech(t, x, y),
        "leaf_oak" => leaf(t, x, y, [1.0, 1.0, 1.0]), "leaf_birch" => leaf(t, x, y, [1.2, 1.12, 0.9]), "leaf_beech" => leaf(t, x, y, [0.86, 1.0, 0.82]),
        "end_oak" => wood_end(t, x, y, &OAK_W, 0.78, &b_oak), "end_birch" => wood_end(t, x, y, &BIRCH_W, 0.95, &birch), "end_beech" => wood_end(t, x, y, &BEECH_W, 0.85, &b_beech),
        "planks" => planks(t, x, y, &PLANKS), "planks_birch" => planks(t, x, y, &PLANKS_BIRCH), "planks_spruce" => planks(t, x, y, &PLANKS_SPRUCE),
        "ore_copper" => ore(t, x, y, [0.32, 0.62, 0.48], [0.78, 0.46, 0.24]), "ore_tin" => ore(t, x, y, [0.8, 0.82, 0.84], [0.58, 0.6, 0.62]), "ore_iron" => ore(t, x, y, [0.7, 0.42, 0.28], [0.52, 0.28, 0.18]), "ore_coal" => ore(t, x, y, [0.07, 0.07, 0.08], [0.16, 0.16, 0.17]),
        "clay" => clay(t, x, y), "stone_bricks" => bricks(t, x, y, 16.0, 32.0, &CUT_STONE, [0.36, 0.34, 0.31]), "bricks" => bricks(t, x, y, 8.0, 16.0, &BRICK, [0.74, 0.7, 0.62]),
        "thatch" => thatch(t, x, y), "cobble" => cobble(t, x, y), "farmland_top" => farmland(t, x, y),
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
        "water_foam" => { let f = t.fbm(x, y, 8, 3); S { c: [f, f, f], h: f, r: 0.3 } }
        _ => panic!("matériau sans générateur : {name}"),
    }
}
fn detail(n: &str) -> f64 { match n { "bark_oak" => 0.2, "bark_birch" => 0.08, "bark_beech" => 0.14, "end_oak" => 0.1, "end_birch" => 0.08, "end_beech" => 0.08, "leaf_oak" => 0.16, "leaf_birch" => 0.16, "leaf_beech" => 0.16, "stone" => 0.2, "mossy_stone" => 0.2, "limestone" => 0.06, "granite" => 0.06, "gravel" => 0.08, "dirt" => 0.22, "path" => 0.2, "forest_soil" => 0.24, "grass_top" => 0.22, "grass_side" => 0.2, "planks" => 0.14, "sand" => 0.14, "grass_dry_top" => 0.22, "grass_dry_side" => 0.2, "dry_earth" => 0.18, "bark_dead" => 0.2, "lichen" => 0.2, "bark_acacia" => 0.2, "leaf_acacia" => 0.16, "bone" => 0.12, "leaf_spruce" => 0.1, _ => 0.0 } }
pub struct Textures { pub albedo: Vec<u8>, pub nrm: Vec<u8>, pub layers: u32, pub size: u32 }
/// Deux piles RGBA : albédo, puis normale XY et rugosité. Le motif vit dans un repère de 64 unités ; à 128 pixels, une octave fine ajoute un vrai détail.
pub fn build_textures(size: usize) -> Textures {
    let layers = MATERIALS.len() * VARIANTS as usize; let n = size * size * 4 * layers;
    let (mut albedo, mut nrm) = (vec![0u8; n], vec![0u8; n]); let mut hh = vec![0f32; size * size];
    let k = TEX / size as f64; let gain = (1.6 * size as f64) / TEX;
    let wrap = |v: usize, d: i32| (v as i32 + d).rem_euclid(size as i32) as usize;
    let b8 = |v: f64| (clamp(v, 0.0, 1.0) * 255.0) as u8;
    for (m, name) in MATERIALS.iter().enumerate() {
        let amp = if size as f64 > TEX { detail(name) } else { 0.0 };
        for v in 0..VARIANTS as usize {
            let t = Tile::new((1000 + m * 17 + v * 3) as u32); let layer = m * VARIANTS as usize + v; let base = layer * size * size * 4;
            for y in 0..size { for x in 0..size {
                let (lx, ly) = (x as f64 * k, y as f64 * k); let s = gen(name, &t, lx, ly); let p = base + (x + y * size) * 4;
                let d = if amp != 0.0 { t.val(lx, ly, size as i64) - 0.5 } else { 0.0 }; let g = 1.0 + d * amp * 2.0;
                albedo[p] = b8(s.c[0] * g); albedo[p + 1] = b8(s.c[1] * g); albedo[p + 2] = b8(s.c[2] * g); albedo[p + 3] = 255;
                hh[x + y * size] = (s.h + d * amp) as f32; nrm[p + 2] = (s.r * 255.0) as u8; nrm[p + 3] = 255;
            } }
            for y in 0..size { for x in 0..size {
                let p = base + (x + y * size) * 4;
                let dx = hh[wrap(x, 1) + y * size] as f64 - hh[wrap(x, -1) + y * size] as f64; let dy = hh[x + wrap(y, 1) * size] as f64 - hh[x + wrap(y, -1) * size] as f64;
                nrm[p] = b8(0.5 - dx * gain); nrm[p + 1] = b8(0.5 - dy * gain);
            } }
        }
    }
    Textures { albedo, nrm, layers: layers as u32, size: size as u32 }
}

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
