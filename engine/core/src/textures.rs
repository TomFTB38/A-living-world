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
const FELD: [C; 4] = pl!("#7a5a52", "#9a7468", "#b98f7f", "#d3ab98"); const QUARTZ: [C; 3] = pl!("#a9a5a0", "#c9c6c0", "#e6e3dc"); const MICA: [C; 3] = pl!("#17151a", "#2d2a30", "#45414a");
const LIME: [C; 4] = pl!("#857a63", "#ad9f84", "#cdc0a4", "#e6dcc4"); const MOSS: [C; 4] = pl!("#37561d", "#55792a", "#7a9c37", "#a6bd4f");
const OAK: [C; 4] = pl!("#4a2c14", "#7a4f29", "#a8743f", "#cf9d62"); const BEECH: [C; 4] = pl!("#726c60", "#938c7f", "#b0a99b", "#c9c3b5");
const BIRCH: [C; 3] = pl!("#d6d0bf", "#ece7da", "#fbf8f0"); const BIRCH_DARK: [C; 2] = pl!("#17140f", "#3e3428");
const OAK_W: [C; 4] = pl!("#5d3a1c", "#8e6133", "#bd8c52", "#dcb67e"); const BIRCH_W: [C; 4] = pl!("#a98a5d", "#cdb080", "#e6d3a8", "#f3e8c9"); const BEECH_W: [C; 4] = pl!("#8a6a48", "#b28f66", "#d0b08a", "#e4cdaa");
const PLANKS: [C; 3] = pl!("#6c4524", "#94673a", "#b98a52"); const PATH: [C; 4] = pl!("#7c5b35", "#a07a4c", "#bc9663", "#d2b080");
const SOIL: [C; 4] = pl!("#2f2413", "#4a3a1f", "#5e5127", "#6f6a2f"); const SAND: [C; 4] = pl!("#a89268", "#c2ac7e", "#d2bf92", "#e0cfa6"); const GRAVEL: [C; 4] = pl!("#6c6761", "#8e8679", "#a89f91", "#c0b7a9");

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
fn granite(t: &Tile, x: f64, y: f64) -> S {
    let (d1, d2, id) = t.vor(x, y, 17); let (e1, _, id2) = t.vor(x + 9.0, y + 5.0, 27); let (r, m, n, fine) = (t.pi(id), t.pi(id2), t.fbm(x, y, 6, 3), t.val(x, y, 32));
    let (dark, clear) = (m > 0.8 && e1 < 0.42, r > 0.68);
    let c = if dark { pal(&MICA, n) } else if clear { pal(&QUARTZ, 0.2 + 0.7 * n) } else { pal(&FELD, 0.15 + 0.6 * r + 0.35 * (n - 0.5) + 0.2 * (fine - 0.5)) };
    let edge = smooth(0.0, 0.08, d2 - d1);
    S { c: mul(c, 0.86 + 0.14 * edge), h: 0.35 + 0.35 * edge + if clear { 0.2 } else { 0.0 } + 0.1 * n, r: if clear { 0.42 } else { 0.62 } }
}
fn limestone(t: &Tile, x: f64, y: f64) -> S {
    let wx = x + 9.0 * t.fbm(x, y, 3, 3) - 4.5; let wy = y + 9.0 * t.fbm(x + 23.0, y + 11.0, 3, 3) - 4.5; let (d1, d2, id) = t.vor(wx, wy, 4);
    let broken = smooth(0.42, 0.58, t.fbm(x + 41.0, y + 3.0, 4, 3)); let crack = 1.0 - broken * (1.0 - smooth(0.0, 0.04, d2 - d1));
    let (p1, _, pid) = t.vor(x + 3.0, y + 8.0, 10); let (q1, _, qid) = t.vor(x + 17.0, y + 2.0, 22);
    let big = if t.pi(pid) > 0.7 { smooth(0.08, 0.34, p1) } else { 1.0 }; let small = if t.pi(qid) > 0.8 { smooth(0.1, 0.42, q1) } else { 1.0 }; let pit = big.min(small);
    let n = t.fbm(x, y, 7, 4); let stain = smooth(0.42, 0.72, t.fbm(x + 9.0, y + 40.0, 3, 3)); let bed = 0.5 + 0.5 * sin(wy * std::f64::consts::PI * 6.0 / 64.0 + 5.0 * (n - 0.5));
    let c = pal(&LIME, 0.42 + 0.2 * t.pi(id) + 0.85 * (n - 0.5) + 0.12 * (bed - 0.5)); let k = (0.55 + 0.45 * crack) * (0.72 + 0.28 * pit);
    S { c: [c[0] * k, c[1] * k * (1.0 - 0.05 * stain), c[2] * k * (1.0 - 0.14 * stain)], h: 0.1 + 0.42 * crack * pit + 0.48 * n * pit, r: 0.9 }
}
fn mossy(t: &Tile, x: f64, y: f64) -> S {
    let st = stone(t, x, y, &STONE, 5); let drip = 15.0 + 22.0 * t.fbm(x, 0.0, 5, 3); let m = t.fbm(x, y, 7, 3); let k = y < drip || m > 0.61; let g = t.val(x, y, 32);
    let c = pal(&MOSS, 0.14 + 0.72 * m + 0.26 * (g - 0.5));
    S { c: if k { c } else { mul(st.c, if y < drip + 5.0 { 0.72 } else { 0.92 }) }, h: if k { 0.72 + 0.28 * g } else { st.h }, r: if k { 0.92 } else { st.r } }
}
fn bark(t: &Tile, x: f64, y: f64, colors: &[C], relief: f64) -> S {
    const COLS: f64 = 12.0; let w = x + 2.6 * t.fbm(x, y, 4, 2) - 1.3; let q = ((w + 64.0) % 64.0) * COLS / 64.0; let col = q.floor() % COLS; let u = q - q.floor();
    let tc = t.pi(col as i64 * 61 + 7); let seg = ((y + tc * 64.0) % 64.0) / 64.0 * 5.0; let sid = seg.floor(); let sf = seg - sid; let ts = t.pi(col as i64 * 17 + sid as i64 * 131);
    let ridge = sin(u * std::f64::consts::PI).powf(0.7); let gap = smooth(0.0, 0.07, sf) * smooth(1.0, 0.93, sf); let (n, fine) = (t.fbm(x, y, 8, 3), t.val(x, y * 0.5, 32));
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
fn planks(t: &Tile, x: f64, y: f64) -> S {
    let row = (y / 16.0).floor(); let f = (y % 16.0) / 16.0; let seam = smooth(0.0, 0.07, f) * smooth(1.0, 0.93, f); let g = t.val(x + row * 19.0, y * 2.2, 12); let tone = t.pi(row as i64 * 13);
    S { c: mul(pal(&PLANKS, 0.15 + 0.45 * tone + 0.4 * g), 0.5 + 0.5 * seam), h: seam * (0.7 + 0.3 * g), r: 0.7 }
}
fn gen(name: &str, t: &Tile, x: f64, y: f64) -> S {
    let b_oak = |a: &Tile, b: f64, c: f64| bark(a, b, c, &OAK, 1.0); let b_beech = |a: &Tile, b: f64, c: f64| bark(a, b, c, &BEECH, 0.28);
    match name {
        "grass_top" => grass_top(t, x, y), "grass_side" => grass_side(t, x, y), "dirt" => dirt(t, x, y, &DIRT), "path" => dirt(t, x, y, &PATH), "forest_soil" => dirt(t, x, y, &SOIL),
        "stone" => stone(t, x, y, &STONE, 5), "mossy_stone" => mossy(t, x, y), "limestone" => limestone(t, x, y), "granite" => granite(t, x, y), "gravel" => stone(t, x, y, &GRAVEL, 11), "sand" => dirt(t, x, y, &SAND),
        "bark_oak" => b_oak(t, x, y), "bark_birch" => birch(t, x, y), "bark_beech" => b_beech(t, x, y),
        "leaf_oak" => leaf(t, x, y, [1.0, 1.0, 1.0]), "leaf_birch" => leaf(t, x, y, [1.2, 1.12, 0.9]), "leaf_beech" => leaf(t, x, y, [0.86, 1.0, 0.82]),
        "end_oak" => wood_end(t, x, y, &OAK_W, 0.78, &b_oak), "end_birch" => wood_end(t, x, y, &BIRCH_W, 0.95, &birch), "end_beech" => wood_end(t, x, y, &BEECH_W, 0.85, &b_beech),
        "planks" => planks(t, x, y),
        "water_foam" => { let f = t.fbm(x, y, 8, 3); S { c: [f, f, f], h: f, r: 0.3 } }
        _ => panic!("matériau sans générateur : {name}"),
    }
}
fn detail(n: &str) -> f64 { match n { "bark_oak" => 0.2, "bark_birch" => 0.08, "bark_beech" => 0.14, "end_oak" => 0.1, "end_birch" => 0.08, "end_beech" => 0.08, "leaf_oak" => 0.16, "leaf_birch" => 0.16, "leaf_beech" => 0.16, "stone" => 0.2, "mossy_stone" => 0.2, "limestone" => 0.2, "granite" => 0.2, "gravel" => 0.22, "dirt" => 0.22, "path" => 0.2, "forest_soil" => 0.24, "grass_top" => 0.22, "grass_side" => 0.2, "planks" => 0.14, "sand" => 0.14, _ => 0.0 } }
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
